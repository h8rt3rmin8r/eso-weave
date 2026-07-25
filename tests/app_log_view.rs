//! Log-view tests for the GUI: level colors, filtering, and autoscroll.

use time::OffsetDateTime;
use tracing::Level;

use eso_weave::app::clamp_log_height;
use eso_weave::app::log_view::{autoscroll, build_log_view, level_color, LogColor};
use eso_weave::config::LevelName;
use eso_weave::logging::LogEvent;

#[test]
fn log_height_clamps_to_window() {
    use eso_weave::app::log_min_height;
    let window = 800.0;
    let row = 14.0;
    let content_min = 420.0;
    let min = log_min_height(row);
    let max = window - content_min;
    // Grows are capped so the interactive (Skills) area stays visible.
    assert!(clamp_log_height(10_000.0, window, row, content_min) <= max + 0.01);
    // Shrinks are floored to the six-line minimum so the panel stays readable.
    assert_eq!(clamp_log_height(0.0, window, row, content_min), min);
    // A reasonable height passes through unchanged.
    assert_eq!(clamp_log_height(200.0, window, row, content_min), 200.0);
    // A tiny window still yields the readable minimum.
    assert_eq!(clamp_log_height(10.0, 100.0, row, content_min), min);
}

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
    // ERROR is the brand palette red (retuned in S012 to be legible on both
    // themes). The distinctness checks below are what the view relies on.
    let error = level_color(Level::ERROR);
    assert_eq!(
        error,
        LogColor {
            r: 0xF8,
            g: 0x71,
            b: 0x71
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
