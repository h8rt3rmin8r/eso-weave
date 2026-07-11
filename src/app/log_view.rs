//! Live log view derivation (pure): snapshot filtering and per-level colors.

use tracing::Level;

use crate::config::LevelName;
use crate::logging::LogEvent;

/// A theme-independent RGB color for a log row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogColor {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
}

impl LogColor {
    const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// A rendered log row: its text and its level color.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRow {
    /// The formatted log line.
    pub text: String,
    /// The color for the row, by level.
    pub color: LogColor,
}

/// The color for a level: ERROR red, WARN amber, INFO neutral, DEBUG dim, TRACE
/// dimmer.
pub fn level_color(level: Level) -> LogColor {
    match level {
        Level::ERROR => LogColor::new(0xE0, 0x30, 0x30),
        Level::WARN => LogColor::new(0xE0, 0xA0, 0x30),
        Level::INFO => LogColor::new(0xC8, 0xC8, 0xC8),
        Level::DEBUG => LogColor::new(0x90, 0x90, 0x90),
        Level::TRACE => LogColor::new(0x60, 0x60, 0x60),
    }
}

/// Builds the visible log rows: events at or above `min_level`, in order, each
/// colored by its level.
pub fn build_log_view(events: &[LogEvent], min_level: LevelName) -> Vec<LogRow> {
    let threshold = name_threshold(min_level);
    events
        .iter()
        .filter(|event| threshold > 0 && level_threshold(event.level) <= threshold)
        .map(|event| LogRow {
            text: event.to_line(),
            color: level_color(event.level),
        })
        .collect()
}

/// The panel autoscrolls only while the user is at the bottom.
pub fn autoscroll(at_bottom: bool) -> bool {
    at_bottom
}

/// The verbosity threshold for a stored level name (0 = off, higher = more
/// verbose), matching the logging scale.
fn name_threshold(level: LevelName) -> u8 {
    match level {
        LevelName::Off => 0,
        LevelName::Error => 1,
        LevelName::Warn => 2,
        LevelName::Info => 3,
        LevelName::Debug => 4,
        LevelName::Trace => 5,
    }
}

/// The verbosity threshold for an event level, on the same scale.
fn level_threshold(level: Level) -> u8 {
    match level {
        Level::ERROR => 1,
        Level::WARN => 2,
        Level::INFO => 3,
        Level::DEBUG => 4,
        Level::TRACE => 5,
    }
}
