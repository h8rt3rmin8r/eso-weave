//! The bite-detector abstraction and its version 1 pixel-bus implementation.
//!
//! The controller depends on the [`BiteDetector`] trait, not a concrete detector,
//! so future detectors can be added without touching the controller. The pure
//! [`map_event`] mapping and the already-tested reader carry the logic;
//! [`PixelBusDetector`] is a thin adapter.

use crate::pixelbus::{PixelBusEvent, PixelBusReader, SurfaceSampler};

use super::DetectorEvent;

/// Maps a Pixel Bus Reader event to a detector event, dropping latency.
pub fn map_event(event: PixelBusEvent) -> Option<DetectorEvent> {
    match event {
        PixelBusEvent::Heartbeat => Some(DetectorEvent::Heartbeat),
        PixelBusEvent::SignalLost => Some(DetectorEvent::SignalLost),
        PixelBusEvent::FishingStarted => Some(DetectorEvent::FishingStarted),
        PixelBusEvent::BiteDetected => Some(DetectorEvent::BiteDetected),
        PixelBusEvent::FishingStopped => Some(DetectorEvent::FishingStopped),
        PixelBusEvent::Latency(_) => None,
    }
}

/// A source of bite-detection events.
pub trait BiteDetector {
    /// Advances the detector to `now_ms` and returns any events produced.
    fn poll(&mut self, now_ms: u64) -> Vec<DetectorEvent>;
}

/// A test detector returning scripted event batches, one per poll.
#[derive(Debug, Default)]
pub struct StubDetector {
    batches: std::collections::VecDeque<Vec<DetectorEvent>>,
}

impl StubDetector {
    /// Creates an empty stub detector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues a batch of events to return on a subsequent poll.
    pub fn push_batch(&mut self, events: Vec<DetectorEvent>) {
        self.batches.push_back(events);
    }
}

impl BiteDetector for StubDetector {
    fn poll(&mut self, _now_ms: u64) -> Vec<DetectorEvent> {
        self.batches.pop_front().unwrap_or_default()
    }
}

/// The version 1 detector: adapts a [`PixelBusReader`] over a [`SurfaceSampler`].
pub struct PixelBusDetector<S> {
    reader: PixelBusReader,
    sampler: S,
}

impl<S: SurfaceSampler> PixelBusDetector<S> {
    /// Creates a detector over the given reader and sampler.
    pub fn new(reader: PixelBusReader, sampler: S) -> Self {
        Self { reader, sampler }
    }
}

impl<S: SurfaceSampler> BiteDetector for PixelBusDetector<S> {
    fn poll(&mut self, now_ms: u64) -> Vec<DetectorEvent> {
        self.reader
            .sample_and_observe(&self.sampler as &dyn SurfaceSampler, now_ms)
            .into_iter()
            .filter_map(map_event)
            .collect()
    }
}
