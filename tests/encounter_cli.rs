use std::process::Command;

use eso_weave::catalog::compiler::{build_catalog, BuildRequest};
use eso_weave::catalog::Channel;

const COMPLETE: &str = include_str!("fixtures/encounter/valid-complete.lua");

fn command() -> Command {
    let command = Command::new(env!("CARGO_BIN_EXE_catalog-compiler"));
    hidden(command)
}

#[cfg(windows)]
fn hidden(mut command: Command) -> Command {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

#[cfg(not(windows))]
fn hidden(command: Command) -> Command {
    command
}

#[test]
fn encounter_lifecycle_commands_emit_json_and_fail_noninteractively() {
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let backup = sandbox.path().join("backup.sqlite");
    std::fs::write(&input, COMPLETE).unwrap();

    let imported = command()
        .args([
            "encounter-import",
            "--input",
            input.to_str().unwrap(),
            "--store",
            store.to_str().unwrap(),
            "--channel",
            "live",
        ])
        .output()
        .unwrap();
    assert!(imported.status.success());
    let receipt: serde_json::Value = serde_json::from_slice(&imported.stdout).unwrap();
    assert_eq!(receipt["outcome"], "imported");
    assert!(receipt.get("events").is_none());

    let listed = command()
        .args(["encounter-list", "--store", store.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(listed.status.success());
    let list: serde_json::Value = serde_json::from_slice(&listed.stdout).unwrap();
    assert_eq!(list.as_array().unwrap().len(), 1);

    let deleted_one = command()
        .args([
            "encounter-delete",
            "--store",
            store.to_str().unwrap(),
            "--session",
            "session-1788912000-1000",
            "--encounter",
            "encounter-1788912000-1",
        ])
        .output()
        .unwrap();
    assert!(deleted_one.status.success());
    let receipt: serde_json::Value = serde_json::from_slice(&deleted_one.stdout).unwrap();
    assert_eq!(receipt["deleted_records"], 1);

    let reimported = command()
        .args([
            "encounter-import",
            "--input",
            input.to_str().unwrap(),
            "--store",
            store.to_str().unwrap(),
            "--channel",
            "live",
        ])
        .output()
        .unwrap();
    assert!(reimported.status.success());

    let backed_up = command()
        .args([
            "encounter-backup",
            "--store",
            store.to_str().unwrap(),
            "--output",
            backup.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(backed_up.status.success());
    assert!(backup.is_file());

    let deleted = command()
        .args([
            "encounter-delete",
            "--store",
            store.to_str().unwrap(),
            "--all",
        ])
        .output()
        .unwrap();
    assert!(deleted.status.success());
    let receipt: serde_json::Value = serde_json::from_slice(&deleted.stdout).unwrap();
    assert_eq!(receipt["deleted_records"], 1);

    let invalid = command()
        .args([
            "encounter-delete",
            "--store",
            store.to_str().unwrap(),
            "--all",
            "--session",
            "session-1-1",
            "--encounter",
            "encounter-1-1",
        ])
        .output()
        .unwrap();
    assert!(!invalid.status.success());
}

#[test]
fn encounter_projection_command_publishes_and_prints_the_same_identity() {
    let sandbox = tempfile::tempdir().unwrap();
    let capture_json =
        include_str!("../specs/077-encounter-metrics/fixtures/encounter-metrics-capture.json");
    let value: serde_json::Value = serde_json::from_str(capture_json).unwrap();
    let input = sandbox.path().join("capture.lua");
    let store = sandbox.path().join("encounters.sqlite");
    let catalog = sandbox.path().join("catalog.sqlite");
    let output = sandbox.path().join("projection.json");
    std::fs::write(
        &input,
        format!("EsoWeaveEncounterSaved = {}", json_to_lua(&value)),
    )
    .unwrap();
    build_catalog(&BuildRequest::new(
        "specs/070-catalog-compiler/fixtures/minimal-live.json",
        &catalog,
        Channel::Live,
    ))
    .unwrap();
    assert!(command()
        .args([
            "encounter-import",
            "--input",
            input.to_str().unwrap(),
            "--store",
            store.to_str().unwrap(),
            "--channel",
            "live",
        ])
        .output()
        .unwrap()
        .status
        .success());

    let projected = command()
        .args([
            "encounter-project",
            "--store",
            store.to_str().unwrap(),
            "--catalog",
            catalog.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--session",
            "session-1788998400-500000",
            "--encounter",
            "encounter-1788998400-1",
        ])
        .output()
        .unwrap();
    assert!(projected.status.success());
    let stdout: serde_json::Value = serde_json::from_slice(&projected.stdout).unwrap();
    let published: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
    assert_eq!(stdout, published);
    assert_eq!(stdout["observed_dps"]["value"], 300.0);
}

fn json_to_lua(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "nil".into(),
        serde_json::Value::Bool(value) => value.to_string(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::String(value) => serde_json::to_string(value).unwrap(),
        serde_json::Value::Array(values) => format!(
            "{{{}}}",
            values
                .iter()
                .enumerate()
                .map(|(index, value)| format!("[{}]={}", index + 1, json_to_lua(value)))
                .collect::<Vec<_>>()
                .join(",")
        ),
        serde_json::Value::Object(values) => format!(
            "{{{}}}",
            values
                .iter()
                .map(|(key, value)| format!(
                    "[{}]={}",
                    serde_json::to_string(key).unwrap(),
                    json_to_lua(value)
                ))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}
