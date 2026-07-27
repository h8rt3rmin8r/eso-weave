//! Reader-event routing (pure): maps a pixel bus reader event to subsystem calls.

use crate::app::UiIntent;
use crate::fishing::{map_event, FishingController, FishingSink};
use crate::input::{Action, InputEngine};
use crate::pixelbus::PixelBusEvent;
use crate::weave::WeaveEngine;

/// Maps an application-level toggle action (delivered by a hotkey) to the same
/// [`UiIntent`] the corresponding GUI button raises, so a hotkey and its button
/// reach one shared state, one persistence mark, and one display path.
///
/// - `ToggleSuspend` maps to [`UiIntent::ToggleSuspend`] (the intent reads and
///   flips the live suspend state itself).
/// - `ToggleFishing` maps to [`UiIntent::SetFishing`] with the negation of the
///   current on/off state, matching the Fishing button.
/// - Any non-toggle action maps to `None`; weave actions never travel this path.
pub fn app_toggle_intent(action: Action, fishing_on: bool) -> Option<UiIntent> {
    match action {
        Action::ToggleSuspend => Some(UiIntent::ToggleSuspend),
        Action::ToggleFishing => Some(UiIntent::SetFishing(!fishing_on)),
        _ => None,
    }
}

/// Routes one reader event to the weave engine and the fishing controller.
///
/// - `Latency(ms)` sets the weave engine's current latency (nothing to fishing).
/// - `WeaponBar(signal)` sets the weave engine's active bar and weapon classes.
/// - `Combat(signal)` stores the decoded combat state (nothing acts on it).
/// - `MenuGate(surface)` sets the menu gate on the input engine and the fishing
///   controller, so neither starts new work while a game UI surface is up.
/// - `Resources(set)` stores the decoded resource levels (nothing acts on them).
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
    input: &InputEngine,
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
        PixelBusEvent::Combat(signal) => {
            weave.set_combat(signal);
            return;
        }
        PixelBusEvent::Resources(set) => {
            weave.set_resources(set);
            return;
        }
        PixelBusEvent::MenuGate(surface) => {
            let gates = surface.gates();
            input.set_menu_gated(gates);
            fishing.set_gated(gates);
            weave.set_menu(surface);
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

#[cfg(test)]
mod tests {
    use super::app_toggle_intent;
    use crate::app::UiIntent;
    use crate::input::Action;

    #[test]
    fn suspend_toggle_maps_to_toggle_suspend_regardless_of_fishing() {
        for fishing_on in [false, true] {
            match app_toggle_intent(Action::ToggleSuspend, fishing_on) {
                Some(UiIntent::ToggleSuspend) => {}
                other => panic!("expected ToggleSuspend, got a different intent (fishing_on={fishing_on}, some={})", other.is_some()),
            }
        }
    }

    #[test]
    fn fishing_toggle_negates_the_current_state() {
        match app_toggle_intent(Action::ToggleFishing, false) {
            Some(UiIntent::SetFishing(true)) => {}
            _ => panic!("fishing off must map to SetFishing(true)"),
        }
        match app_toggle_intent(Action::ToggleFishing, true) {
            Some(UiIntent::SetFishing(false)) => {}
            _ => panic!("fishing on must map to SetFishing(false)"),
        }
    }

    #[test]
    fn non_toggle_actions_map_to_none() {
        for action in [
            Action::Skill1,
            Action::Skill2,
            Action::Skill3,
            Action::Skill4,
            Action::Skill5,
            Action::Ultimate,
            Action::Synergy,
        ] {
            assert!(
                app_toggle_intent(action, false).is_none(),
                "{action:?} must not map to an app-toggle intent"
            );
        }
    }
}
