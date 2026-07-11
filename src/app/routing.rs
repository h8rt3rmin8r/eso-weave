//! Reader-event routing (pure): maps a pixel bus reader event to subsystem calls.

use crate::fishing::{map_event, FishingController, FishingSink};
use crate::pixelbus::PixelBusEvent;
use crate::weave::WeaveEngine;

/// Routes one reader event to the weave engine and the fishing controller.
///
/// - `Latency(ms)` sets the weave engine's current latency (nothing to fishing).
/// - `WeaponBar(signal)` sets the weave engine's active bar and weapon classes.
/// - `SignalLost` clears the weave latency and disables fishing.
/// - `FishingStarted`, `BiteDetected`, `FishingStopped` reach the controller.
/// - `Heartbeat` is forwarded to the controller (a no-op there).
///
/// The fishing forwarding reuses [`map_event`], so the reader-to-detector mapping
/// has one source of truth; latency is set before the map, so a `Latency` event
/// (which maps to `None`) never reaches fishing.
pub fn route_reader_event(
    event: PixelBusEvent,
    weave: &mut WeaveEngine,
    fishing: &mut FishingController,
    now_ms: u64,
    sink: &mut dyn FishingSink,
) {
    match event {
        PixelBusEvent::Latency(ms) => {
            weave.set_latency(Some(ms));
            return;
        }
        PixelBusEvent::WeaponBar(signal) => {
            weave.set_weapon_bar(signal);
            return;
        }
        PixelBusEvent::SignalLost => {
            weave.set_latency(None);
        }
        _ => {}
    }
    if let Some(detector_event) = map_event(event) {
        fishing.on_event(detector_event, now_ms, sink);
    }
}
