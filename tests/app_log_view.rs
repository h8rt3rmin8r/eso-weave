//! Log-view tests for the GUI: level colors, filtering, and autoscroll.

use time::OffsetDateTime;
use tracing::Level;

use eso_weave::app::log_view::{autoscroll, build_log_view, level_color, LogColor};
use eso_weave::config::LevelName;
use eso_weave::logging::LogEvent;

fn event(level: Level, message: &str) -> LogEvent {
    LogEvent {
        timestamp: OffsetDateTime::UNIX_EPOCH,
        level,
        target: "eso_weave::test".to_string(),
        message: message.to_string(),
    }
}

#[test]
fn level_colors_are_distinct_per_level() {
    let error = level_color(Level::ERROR);
    assert_eq!(
        error,
        LogColor {
            r: 0xE0,
            g: 0x30,
            b: 0x30
        }
    );
    assert_ne!(level_color(Level::WARN), error);
    assert_ne!(level_color(Level::INFO), level_color(Level::DEBUG));
    assert_ne!(level_color(Level::DEBUG), level_color(Level::TRACE));
}

#[test]
fn build_log_view_filters_at_or_above_min_level() {
    let events = vec![
        event(Level::ERROR, "boom"),
        event(Level::WARN, "careful"),
        event(Level::INFO, "hello"),
        event(Level::DEBUG, "detail"),
        event(Level::TRACE, "noise"),
    ];

    // Warn filter keeps ERROR and WARN only, in order.
    let rows = build_log_view(&events, LevelName::Warn);
    assert_eq!(rows.len(), 2);
    assert!(rows[0].text.contains("boom"));
    assert!(rows[1].text.contains("careful"));
    assert_eq!(rows[0].color, level_color(Level::ERROR));

    // Trace filter keeps all five.
    assert_eq!(build_log_view(&events, LevelName::Trace).len(), 5);

    // Off filter keeps none.
    assert_eq!(build_log_view(&events, LevelName::Off).len(), 0);

    // Info filter keeps ERROR, WARN, INFO.
    assert_eq!(build_log_view(&events, LevelName::Info).len(), 3);
}

#[test]
fn autoscroll_follows_at_bottom() {
    assert!(autoscroll(true));
    assert!(!autoscroll(false));
}
