//! Integration tests for the Config Store (spec user story US1).

use std::fs;

use eso_weave::config::{self, LevelName, LoggingPrefs, NoticeKind, Settings};

fn tmp() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

#[test]
fn round_trip_preserves_settings() {
    let dir = tmp();
    let settings = Settings {
        schema_version: config::CURRENT_SCHEMA_VERSION,
        logging: LoggingPrefs {
            level: LevelName::Debug,
            file_enabled: true,
        },
        ..Settings::default()
    };
    config::save(dir.path(), &settings).unwrap();

    let outcome = config::load(dir.path());
    assert_eq!(outcome.settings, settings);
    assert!(outcome.notices.is_empty());
}

#[test]
fn file_encoding_has_no_bom_lf_and_trailing_newline() {
    let dir = tmp();
    config::save(dir.path(), &Settings::default()).unwrap();

    let bytes = fs::read(dir.path().join("config.json")).unwrap();
    assert_ne!(bytes.get(..3), Some(&[0xEF, 0xBB, 0xBF][..]));
    assert!(!bytes.contains(&b'\r'));
    assert_eq!(bytes.last(), Some(&b'\n'));
}

#[test]
fn missing_file_yields_defaults_without_notice() {
    let dir = tmp();
    let outcome = config::load(dir.path());
    assert_eq!(outcome.settings, Settings::default());
    assert!(outcome.notices.is_empty());
}

#[test]
fn corrupt_file_is_preserved_and_defaults_returned() {
    let dir = tmp();
    fs::write(dir.path().join("config.json"), b"{ not valid json").unwrap();

    let outcome = config::load(dir.path());
    assert_eq!(outcome.settings, Settings::default());
    assert!(outcome
        .notices
        .iter()
        .any(|n| n.kind == NoticeKind::CorruptConfig));
    assert!(dir.path().join("config.json.invalid").exists());
    assert!(!dir.path().join("config.json").exists());
}

#[test]
fn corrupt_preserved_name_collision_gets_discriminator() {
    let dir = tmp();
    fs::write(dir.path().join("config.json.invalid"), b"older").unwrap();
    fs::write(dir.path().join("config.json"), b"still not json").unwrap();

    let outcome = config::load(dir.path());
    assert!(outcome
        .notices
        .iter()
        .any(|n| n.kind == NoticeKind::CorruptConfig));
    assert!(dir.path().join("config.json.invalid.2").exists());
}

#[test]
fn older_schema_migrates_with_notice() {
    let dir = tmp();
    fs::write(
        dir.path().join("config.json"),
        br#"{"schema_version":0,"logging":{"level":"warn","file_enabled":false}}"#,
    )
    .unwrap();

    let outcome = config::load(dir.path());
    assert_eq!(
        outcome.settings.schema_version,
        config::CURRENT_SCHEMA_VERSION
    );
    assert_eq!(outcome.settings.logging.level, LevelName::Warn);
    assert!(outcome
        .notices
        .iter()
        .any(|n| n.kind == NoticeKind::Migrated));
}

#[test]
fn unknown_keys_warn_but_are_not_fatal() {
    let dir = tmp();
    fs::write(
        dir.path().join("config.json"),
        br#"{"schema_version":1,"logging":{"level":"info","file_enabled":false},"mystery":42}"#,
    )
    .unwrap();

    let outcome = config::load(dir.path());
    assert!(outcome
        .notices
        .iter()
        .any(|n| n.kind == NoticeKind::UnknownKeys));
    assert_eq!(outcome.settings.logging.level, LevelName::Info);
}

#[test]
fn invalid_level_falls_back_with_notice() {
    let dir = tmp();
    fs::write(
        dir.path().join("config.json"),
        br#"{"schema_version":1,"logging":{"level":"bogus","file_enabled":true}}"#,
    )
    .unwrap();

    let outcome = config::load(dir.path());
    assert_eq!(outcome.settings.logging.level, LevelName::Info);
    assert!(outcome
        .notices
        .iter()
        .any(|n| n.kind == NoticeKind::InvalidValue));
    assert!(outcome.settings.logging.file_enabled);
}

#[test]
fn unreadable_path_yields_unwritable_notice() {
    let dir = tmp();
    // Make config.json a directory so reading it as a file fails (not NotFound).
    fs::create_dir(dir.path().join("config.json")).unwrap();

    let outcome = config::load(dir.path());
    assert_eq!(outcome.settings, Settings::default());
    assert!(outcome
        .notices
        .iter()
        .any(|n| n.kind == NoticeKind::Unwritable));
}
