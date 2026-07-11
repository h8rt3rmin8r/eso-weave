//! Logging subsystem: structured events with a runtime-selectable level, an
//! always-on in-memory ring buffer, and an optional monthly file sink.
//!
//! Level filtering is applied by the sink itself against a shared threshold, so
//! the level can change at runtime without rebuilding the subscriber. The ring
//! buffer is always populated (independent of the file sink), and the file sink
//! writes `YYYY-MM.log` files. Input contents are only ever emitted at debug or
//! below, and a suppression control can drop input events entirely.

use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};

use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use tracing::field::{Field, Visit};
use tracing::{Dispatch, Event, Level, Subscriber};
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};
use tracing_subscriber::Registry;

use crate::config::{LevelName, LoggingPrefs};

/// Default in-memory ring buffer capacity.
pub const RING_CAPACITY: usize = 1000;

/// The tracing target used for input-related events. Events on this target are
/// dropped when input suppression is enabled.
pub const INPUT_TARGET: &str = "eso_weave::input";

/// A single captured log record.
#[derive(Clone, Debug)]
pub struct LogEvent {
    /// The event time in coordinated universal time.
    pub timestamp: OffsetDateTime,
    /// The event level.
    pub level: Level,
    /// The event source target.
    pub target: String,
    /// The rendered message.
    pub message: String,
}

impl LogEvent {
    /// Formats the event as a single log line (UTC RFC-3339 timestamp, level,
    /// target, message).
    pub fn to_line(&self) -> String {
        let ts = self.timestamp.format(&Rfc3339).unwrap_or_default();
        let level = self.level;
        let target = &self.target;
        let message = &self.message;
        format!("{ts}  {level:<5}  {target}  {message}")
    }

    fn month(&self) -> String {
        self.timestamp
            .format(&Rfc3339)
            .ok()
            .and_then(|s| s.get(..7).map(str::to_string))
            .unwrap_or_else(|| "0000-00".to_string())
    }
}

struct FileSink {
    enabled: bool,
    dir: PathBuf,
    current_month: Option<String>,
    file: Option<File>,
}

impl FileSink {
    fn write_line(&mut self, event: &LogEvent) -> std::io::Result<()> {
        let month = event.month();
        if self.current_month.as_deref() != Some(month.as_str()) {
            std::fs::create_dir_all(&self.dir)?;
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.dir.join(format!("{month}.log")))?;
            self.file = Some(file);
            self.current_month = Some(month);
        }
        if let Some(file) = self.file.as_mut() {
            writeln!(file, "{}", event.to_line())?;
        }
        Ok(())
    }
}

struct Shared {
    threshold: AtomicU8,
    input_suppressed: AtomicBool,
    ring: Mutex<VecDeque<LogEvent>>,
    file: Mutex<FileSink>,
}

/// A handle to the running logging facility for runtime control and inspection.
#[derive(Clone)]
pub struct LogHandle {
    shared: Arc<Shared>,
}

impl LogHandle {
    /// Sets the active level, effective immediately for subsequent events.
    pub fn set_level(&self, level: LevelName) {
        self.shared
            .threshold
            .store(level_threshold(level), Ordering::Relaxed);
    }

    /// Enables or disables the monthly file sink.
    pub fn set_file_enabled(&self, enabled: bool) {
        self.shared.file.lock().unwrap().enabled = enabled;
    }

    /// Enables or disables suppression of input-target events.
    pub fn set_input_suppressed(&self, suppressed: bool) {
        self.shared
            .input_suppressed
            .store(suppressed, Ordering::Relaxed);
    }

    /// Returns up to `limit` of the most recent events, oldest first.
    pub fn recent(&self, limit: usize) -> Vec<LogEvent> {
        let ring = self.shared.ring.lock().unwrap();
        let start = ring.len().saturating_sub(limit);
        ring.iter().skip(start).cloned().collect()
    }

    /// Returns the live logging preferences so a caller can persist them.
    pub fn current_prefs(&self) -> LoggingPrefs {
        LoggingPrefs {
            level: threshold_level(self.shared.threshold.load(Ordering::Relaxed)),
            file_enabled: self.shared.file.lock().unwrap().enabled,
        }
    }
}

struct SinkLayer {
    shared: Arc<Shared>,
}

impl<S: Subscriber> Layer<S> for SinkLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();

        let threshold = self.shared.threshold.load(Ordering::Relaxed);
        if threshold == 0 || level_threshold_from(*metadata.level()) > threshold {
            return;
        }

        let target = metadata.target();
        if target == INPUT_TARGET && self.shared.input_suppressed.load(Ordering::Relaxed) {
            return;
        }

        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);

        let record = LogEvent {
            timestamp: OffsetDateTime::now_utc(),
            level: *metadata.level(),
            target: target.to_string(),
            message: visitor.message,
        };

        {
            let mut ring = self.shared.ring.lock().unwrap();
            if ring.len() == RING_CAPACITY {
                ring.pop_front();
            }
            ring.push_back(record.clone());
        }

        let mut file = self.shared.file.lock().unwrap();
        if file.enabled {
            let _ = file.write_line(&record);
        }
    }
}

#[derive(Default)]
struct MessageVisitor {
    message: String,
}

impl Visit for MessageVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{value:?}");
        }
    }
}

/// Builds the logging subscriber and its handle without installing it globally.
///
/// This is the test seam: install the returned [`Dispatch`] in a scope with
/// `tracing::dispatcher::with_default`, then inspect via the handle. [`init`]
/// calls this and sets the global default.
pub fn build(prefs: &LoggingPrefs, log_dir: PathBuf) -> (Dispatch, LogHandle) {
    let shared = Arc::new(Shared {
        threshold: AtomicU8::new(level_threshold(prefs.level)),
        input_suppressed: AtomicBool::new(false),
        ring: Mutex::new(VecDeque::with_capacity(RING_CAPACITY)),
        file: Mutex::new(FileSink {
            enabled: prefs.file_enabled,
            dir: log_dir,
            current_month: None,
            file: None,
        }),
    });
    let subscriber = Registry::default().with(SinkLayer {
        shared: Arc::clone(&shared),
    });
    (Dispatch::new(subscriber), LogHandle { shared })
}

/// Installs the logging facility as the global default and returns its handle.
///
/// Setting the global default more than once per process is a no-op after the
/// first call.
pub fn init(prefs: &LoggingPrefs, log_dir: PathBuf) -> LogHandle {
    let (dispatch, handle) = build(prefs, log_dir);
    let _ = tracing::dispatcher::set_global_default(dispatch);
    handle
}

/// Records input-related content. Emitted at debug level only (never above), so
/// it is invisible at ordinary verbosity, and dropped entirely when input
/// suppression is enabled.
pub fn log_input(content: &str) {
    tracing::debug!(target: INPUT_TARGET, "{content}");
}

/// Maps a stored level to a verbosity threshold (0 = off, higher = more verbose).
fn level_threshold(level: LevelName) -> u8 {
    match level {
        LevelName::Off => 0,
        LevelName::Error => 1,
        LevelName::Warn => 2,
        LevelName::Info => 3,
        LevelName::Debug => 4,
        LevelName::Trace => 5,
    }
}

/// Maps a verbosity threshold back to a stored level.
fn threshold_level(threshold: u8) -> LevelName {
    match threshold {
        0 => LevelName::Off,
        1 => LevelName::Error,
        2 => LevelName::Warn,
        3 => LevelName::Info,
        4 => LevelName::Debug,
        _ => LevelName::Trace,
    }
}

/// Maps a tracing level to the same verbosity scale used by the threshold.
fn level_threshold_from(level: Level) -> u8 {
    match level {
        Level::ERROR => 1,
        Level::WARN => 2,
        Level::INFO => 3,
        Level::DEBUG => 4,
        Level::TRACE => 5,
    }
}
