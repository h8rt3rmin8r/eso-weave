//! Integration tests for the Logging subsystem (spec user story US2).

use eso_weave::config::{LevelName, LoggingPrefs};
use eso_weave::logging;

fn prefs(level: LevelName, file_enabled: bool) -> LoggingPrefs {
    LoggingPrefs {
        level,
        file_enabled,
    }
}

#[test]
fn runtime_level_change_takes_effect() {
    let dir = tempfile::tempdir().unwrap();
    let (dispatch, handle) =
        logging::build(&prefs(LevelName::Info, false), dir.path().to_path_buf());

    tracing::dispatcher::with_default(&dispatch, || {
        tracing::debug!(target: "t", "dbg-hidden");
        handle.set_level(LevelName::Debug);
        tracing::debug!(target: "t", "dbg-shown");
    });

    let messages: Vec<String> = handle.recent(100).into_iter().map(|e| e.message).collect();
    assert!(!messages.iter().any(|m| m.contains("dbg-hidden")));
    assert!(messages.iter().any(|m| m.contains("dbg-shown")));
}

#[test]
fn ring_buffer_is_independent_and_evicts_oldest() {
    let dir = tempfile::tempdir().unwrap();
    let (dispatch, handle) =
        logging::build(&prefs(LevelName::Trace, false), dir.path().to_path_buf());

    tracing::dispatcher::with_default(&dispatch, || {
        for i in 0..(logging::RING_CAPACITY + 100) {
            tracing::info!(target: "t", "evt-{i}");
        }
    });

    let recent = handle.recent(logging::RING_CAPACITY * 2);
    assert_eq!(recent.len(), logging::RING_CAPACITY);
    assert!(recent.iter().any(|e| e.message == "evt-1099"));
    assert!(!recent.iter().any(|e| e.message == "evt-0"));
}

#[test]
fn file_sink_writes_month_named_line() {
    let dir = tempfile::tempdir().unwrap();
    let (dispatch, _handle) =
        logging::build(&prefs(LevelName::Info, true), dir.path().to_path_buf());

    tracing::dispatcher::with_default(&dispatch, || {
        tracing::info!(target: "eso_weave::test", "hello-file");
    });

    let log_path = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.extension().map(|x| x == "log").unwrap_or(false))
        .expect("a log file should exist");

    let name = log_path.file_name().unwrap().to_string_lossy().into_owned();
    assert_eq!(name.len(), "2026-07.log".len());
    assert!(name.ends_with(".log"));

    let content = std::fs::read_to_string(&log_path).unwrap();
    assert!(content.contains("hello-file"));
    assert!(content.contains("INFO"));
    assert!(content.contains('T') && content.contains('Z'));
    assert!(!content.contains('\r'));
}

#[test]
fn input_content_is_not_logged_above_debug() {
    const SENTINEL: &str = "SECRET-INPUT-CONTENT";

    // At info, input (emitted at debug) is filtered out.
    let dir = tempfile::tempdir().unwrap();
    let (dispatch, handle) =
        logging::build(&prefs(LevelName::Info, false), dir.path().to_path_buf());
    tracing::dispatcher::with_default(&dispatch, || logging::log_input(SENTINEL));
    assert!(!handle
        .recent(100)
        .into_iter()
        .any(|e| e.message.contains(SENTINEL)));

    // At debug and not suppressed, it is captured.
    let dir2 = tempfile::tempdir().unwrap();
    let (dispatch2, handle2) =
        logging::build(&prefs(LevelName::Debug, false), dir2.path().to_path_buf());
    tracing::dispatcher::with_default(&dispatch2, || logging::log_input(SENTINEL));
    assert!(handle2
        .recent(100)
        .into_iter()
        .any(|e| e.message.contains(SENTINEL)));

    // Suppressed, it is dropped even at debug.
    let dir3 = tempfile::tempdir().unwrap();
    let (dispatch3, handle3) =
        logging::build(&prefs(LevelName::Debug, false), dir3.path().to_path_buf());
    handle3.set_input_suppressed(true);
    tracing::dispatcher::with_default(&dispatch3, || logging::log_input(SENTINEL));
    assert!(!handle3
        .recent(100)
        .into_iter()
        .any(|e| e.message.contains(SENTINEL)));
}

#[test]
fn current_prefs_reflect_runtime_changes() {
    let dir = tempfile::tempdir().unwrap();
    let (_dispatch, handle) =
        logging::build(&prefs(LevelName::Info, false), dir.path().to_path_buf());

    handle.set_level(LevelName::Warn);
    handle.set_file_enabled(true);

    let prefs = handle.current_prefs();
    assert_eq!(prefs.level, LevelName::Warn);
    assert!(prefs.file_enabled);
}
