//! Pixel Bus Reader: samples the PixelBeacon blocks from the game window surface
//! and decodes them into typed events.
//!
//! The decoders and the [`PixelBusReader`] state machine are pure and fully
//! tested with crafted samples and an injected clock. Surface sampling sits
//! behind the [`SurfaceSampler`] seam with a mock plus thin OS backends.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::X11Sampler;
#[cfg(windows)]
pub use windows::GdiSampler;

use std::collections::HashMap;

use serde::Deserialize;

use crate::config::{Notice, NoticeKind};

/// A red-green-blue color triple sampled from a beacon point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
}

impl Rgb {
    /// Creates a color from its channels.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// The decoded fishing signal from the fishing block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FishingSignal {
    /// No fishing block present.
    None,
    /// A cast is active and waiting.
    Waiting,
    /// A bite is detected.
    Bite,
}

/// A typed event decoded from the pixel bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelBusEvent {
    /// The status block is present.
    Heartbeat,
    /// The status block has been absent past the timeout.
    SignalLost,
    /// A cast became active and waiting.
    FishingStarted,
    /// A bite was detected.
    BiteDetected,
    /// Fishing stopped (block absent).
    FishingStopped,
    /// A decoded server latency in milliseconds.
    Latency(u16),
}

/// The surface sampling seam: reads one client-area pixel.
pub trait SurfaceSampler {
    /// The color at a client-area point, or `None` when the surface cannot be
    /// sampled.
    fn sample(&self, x: u32, y: u32) -> Option<Rgb>;
}

/// A test sampler that returns crafted colors for specific points.
#[derive(Debug, Default)]
pub struct MockSampler {
    points: HashMap<(u32, u32), Rgb>,
}

impl MockSampler {
    /// Creates an empty mock sampler (every point returns `None`).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the color returned for a point.
    pub fn set(&mut self, x: u32, y: u32, color: Rgb) {
        self.points.insert((x, y), color);
    }

    /// Clears the color for a point (it will return `None`).
    pub fn clear(&mut self, x: u32, y: u32) {
        self.points.remove(&(x, y));
    }
}

impl SurfaceSampler for MockSampler {
    fn sample(&self, x: u32, y: u32) -> Option<Rgb> {
        self.points.get(&(x, y)).copied()
    }
}

/// Reader configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReaderConfig {
    /// Per-channel color match tolerance.
    pub tolerance: u8,
    /// Absence past this after the last heartbeat raises signal loss.
    pub heartbeat_timeout_ms: u64,
    /// Sample point for the status block (B0).
    pub status_point: (u32, u32),
    /// Sample point for the fishing block (B1).
    pub fishing_point: (u32, u32),
    /// Sample point for the latency block (B2).
    pub latency_point: (u32, u32),
    /// Sampling interval while fishing is enabled.
    pub interval_fishing_ms: u64,
    /// Sampling interval otherwise.
    pub interval_idle_ms: u64,
}

impl Default for ReaderConfig {
    fn default() -> Self {
        Self {
            tolerance: 2,
            heartbeat_timeout_ms: 2000,
            status_point: (8, 8),
            fishing_point: (24, 8),
            latency_point: (40, 8),
            interval_fishing_ms: 100,
            interval_idle_ms: 1000,
        }
    }
}

/// The minimum accepted sampling interval, in milliseconds.
const MIN_INTERVAL_MS: u64 = 1;
/// The maximum accepted sampling interval, in milliseconds.
const MAX_INTERVAL_MS: u64 = 60_000;

#[derive(Deserialize, Default)]
struct RawPixelBus {
    #[serde(default)]
    tolerance: Option<u8>,
    #[serde(default)]
    interval_fishing_ms: Option<u64>,
    #[serde(default)]
    interval_idle_ms: Option<u64>,
}

/// Loads the user-editable reader configuration (tolerance and sampling
/// intervals) from the opaque `pixelbus` settings value onto a default
/// [`ReaderConfig`]. The fixed beacon geometry and heartbeat timeout are not
/// user settings and keep their defaults. Null or absent yields defaults; an
/// out-of-range interval falls back with a notice.
pub fn load_reader_config(value: &serde_json::Value, notices: &mut Vec<Notice>) -> ReaderConfig {
    let defaults = ReaderConfig::default();
    if value.is_null() {
        return defaults;
    }
    let raw: RawPixelBus = serde_json::from_value(value.clone()).unwrap_or_default();
    ReaderConfig {
        tolerance: raw.tolerance.unwrap_or(defaults.tolerance),
        interval_fishing_ms: checked_interval(
            raw.interval_fishing_ms,
            defaults.interval_fishing_ms,
            "interval_fishing_ms",
            notices,
        ),
        interval_idle_ms: checked_interval(
            raw.interval_idle_ms,
            defaults.interval_idle_ms,
            "interval_idle_ms",
            notices,
        ),
        ..defaults
    }
}

/// Serializes the user-editable reader configuration to the opaque `pixelbus`
/// settings value.
pub fn store_reader_config(config: &ReaderConfig) -> serde_json::Value {
    serde_json::json!({
        "tolerance": config.tolerance,
        "interval_fishing_ms": config.interval_fishing_ms,
        "interval_idle_ms": config.interval_idle_ms,
    })
}

fn checked_interval(
    value: Option<u64>,
    default: u64,
    name: &str,
    notices: &mut Vec<Notice>,
) -> u64 {
    match value {
        None => default,
        Some(ms) if (MIN_INTERVAL_MS..=MAX_INTERVAL_MS).contains(&ms) => ms,
        Some(_) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!("pixelbus {name} is out of range; using default {default}"),
            });
            default
        }
    }
}

fn within(a: u8, b: u8, tolerance: u8) -> bool {
    a.abs_diff(b) <= tolerance
}

/// Whether a sample matches the status block magenta within tolerance.
pub fn status_present(sample: Rgb, tolerance: u8) -> bool {
    within(sample.r, 0xFF, tolerance)
        && within(sample.g, 0x00, tolerance)
        && within(sample.b, 0xFF, tolerance)
}

/// Decodes the fishing signal from a sample.
pub fn fishing_signal(sample: Rgb, tolerance: u8) -> FishingSignal {
    if within(sample.r, 0x00, tolerance)
        && within(sample.g, 0x80, tolerance)
        && within(sample.b, 0xFF, tolerance)
    {
        FishingSignal::Waiting
    } else if within(sample.r, 0x00, tolerance)
        && within(sample.g, 0xFF, tolerance)
        && within(sample.b, 0x00, tolerance)
    {
        FishingSignal::Bite
    } else {
        FishingSignal::None
    }
}

/// Decodes latency from the latency block, or `None` when the marker or checksum
/// fails validation. The value is the red channel times four.
pub fn decode_latency(sample: Rgb, tolerance: u8) -> Option<u16> {
    let checksum = u16::from(sample.r) + u16::from(sample.b);
    if within(sample.g, 0xA5, tolerance) && checksum.abs_diff(255) <= u16::from(tolerance) {
        Some(u16::from(sample.r) * 4)
    } else {
        None
    }
}

/// The pixel bus reader state machine.
pub struct PixelBusReader {
    config: ReaderConfig,
    last_heartbeat_ms: Option<u64>,
    signal_lost: bool,
    fishing: FishingSignal,
}

impl PixelBusReader {
    /// Creates a reader with the given configuration.
    pub fn new(config: ReaderConfig) -> Self {
        Self {
            config,
            last_heartbeat_ms: None,
            signal_lost: false,
            fishing: FishingSignal::None,
        }
    }

    /// Whether the signal is currently lost.
    pub fn signal_lost(&self) -> bool {
        self.signal_lost
    }

    /// Observes the three samples at `now_ms` and returns the resulting events.
    pub fn observe(
        &mut self,
        b0: Option<Rgb>,
        b1: Option<Rgb>,
        b2: Option<Rgb>,
        now_ms: u64,
    ) -> Vec<PixelBusEvent> {
        let mut events = Vec::new();
        let tolerance = self.config.tolerance;
        let heartbeat = b0.is_some_and(|c| status_present(c, tolerance));

        if heartbeat {
            self.last_heartbeat_ms = Some(now_ms);
            self.signal_lost = false;
            events.push(PixelBusEvent::Heartbeat);

            let signal = b1.map_or(FishingSignal::None, |c| fishing_signal(c, tolerance));
            if signal != self.fishing {
                match signal {
                    FishingSignal::Waiting => events.push(PixelBusEvent::FishingStarted),
                    FishingSignal::Bite => events.push(PixelBusEvent::BiteDetected),
                    FishingSignal::None => events.push(PixelBusEvent::FishingStopped),
                }
                self.fishing = signal;
            }

            if let Some(latency) = b2.and_then(|c| decode_latency(c, tolerance)) {
                events.push(PixelBusEvent::Latency(latency));
            }
        } else if let Some(last) = self.last_heartbeat_ms {
            if !self.signal_lost && now_ms.saturating_sub(last) > self.config.heartbeat_timeout_ms {
                self.signal_lost = true;
                self.fishing = FishingSignal::None;
                events.push(PixelBusEvent::SignalLost);
            }
        }

        events
    }

    /// Samples the three configured points and observes them (the runtime path).
    pub fn sample_and_observe(
        &mut self,
        sampler: &dyn SurfaceSampler,
        now_ms: u64,
    ) -> Vec<PixelBusEvent> {
        let (sx, sy) = self.config.status_point;
        let (fx, fy) = self.config.fishing_point;
        let (lx, ly) = self.config.latency_point;
        let b0 = sampler.sample(sx, sy);
        let b1 = sampler.sample(fx, fy);
        let b2 = sampler.sample(lx, ly);
        self.observe(b0, b1, b2, now_ms)
    }
}
