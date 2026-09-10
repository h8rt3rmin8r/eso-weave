use std::process::Command;

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
