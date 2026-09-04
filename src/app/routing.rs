//! Reader-event routing (pure): maps a pixel bus reader event to subsystem calls.

use crate::app::UiIntent;
use crate::fishing::{map_event, FishingController, FishingSink};
use crate::game::{GameState, SurfaceObservation};
use crate::input::{Action, InputEngine};
use crate::pixelbus::PixelBusEvent;
use crate::potion::AutoPotionController;
use crate::weave::WeaveEngine;

/// Maps an application-level toggle action (delivered by a hotkey) to the same
/// [`UiIntent`] the corresponding GUI button raises, so a hotkey and its button
/// reach one shared state, one persistence mark, and one display path.
///
/// - `ToggleSuspend` maps to [`UiIntent::ToggleSuspend`] (the intent reads and
///   flips the live suspend state itself).
/// - `ToggleFishing` maps to [`UiIntent::SetFishing`] with the negation of the
///   current on/off state, matching the Fishing button.
/// - `ToggleAutoPotion` maps to [`UiIntent::SetAutoPotion`] the same way.
/// - Any non-toggle action maps to `None`; weave actions never travel this path.
pub fn app_toggle_intent(
    action: Action,
    fishing_on: bool,
    auto_potion_on: bool,
) -> Option<UiIntent> {
    match action {
        Action::ToggleSuspend => Some(UiIntent::ToggleSuspend),
        Action::ToggleFishing => Some(UiIntent::SetFishing(!fishing_on)),
        Action::ToggleAutoPotion => Some(UiIntent::SetAutoPotion(!auto_potion_on)),
        _ => None,
    }
}

/// Routes one reader event to the weave engine and the fishing controller.
///
/// - `Latency(ms)` sets the weave engine's current latency (nothing to fishing).
/// - `WeaponBar(signal)` sets the weave engine's active bar and weapon classes.
/// - `Combat(signal)` stores the decoded combat state (nothing acts on it).
/// - `MenuGate(surface)` sets the menu gate on the input engine, the fishing
///   controller, and the auto-potion controller, so none starts new work while a
///   game UI surface is up.
/// - `Resources(set)` stores the decoded resource levels for the next auto-potion tick.
/// - `Movement(signal)` stores the decoded movement state (nothing acts on it).
/// - `Cooldowns(set)` stores the decoded slot cooldowns (nothing acts on them).
/// - `Quickslot(state)` stores the decoded quickslot state for the next auto-potion tick.
/// - `SignalLost` clears the weave latency, disables fishing, and marks the
///   auto-potion beacon unavailable without clearing its requested setting.
/// - `FishingStarted`, `BiteDetected`, `FishingStopped` reach the controller.
/// - `Heartbeat` marks the auto-potion beacon available and reaches fishing.
///
/// The fishing forwarding reuses [`map_event`], so the reader-to-detector mapping
/// has one source of truth; latency is set before the map, so a `Latency` event
/// (which maps to `None`) never reaches fishing.
pub fn route_reader_event(
    event: PixelBusEvent,
    weave: &mut WeaveEngine,
    fishing: &mut FishingController,
    potion: &mut AutoPotionController,
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
        PixelBusEvent::Movement(signal) => {
            weave.set_movement(signal);
            return;
        }
        PixelBusEvent::Cooldowns(set) => {
            weave.set_cooldowns(set);
            return;
        }
        PixelBusEvent::Quickslot(state) => {
            weave.set_quickslot(state);
            return;
        }
        PixelBusEvent::MenuGate(surface) => {
            let gates = surface.is_some_and(crate::pixelbus::MenuSurface::gates);
            input.set_menu_gated(gates);
            fishing.set_gated(gates);
            // The auto-potion controller is gated directly, for the same reason
            // the fishing controller is: it synthesizes on its own timers and
            // never passes through interception, so gating interception alone
            // would leave it firing into a chat message being composed.
            potion.set_gated(gates);
            weave.set_menu(surface.unwrap_or(crate::pixelbus::MenuSurface::None));
            return;
        }
        PixelBusEvent::SignalLost => {
            weave.set_latency(None);
            potion.on_signal_lost();
        }
        PixelBusEvent::Heartbeat => potion.on_heartbeat(),
        _ => {}
    }
    if let Some(detector_event) = map_event(event) {
        fishing.on_event(detector_event, now_ms, sink);
    }
}

/// Routes the observation axes that contribute to the truthful Game Context.
pub fn route_game_observation(event: PixelBusEvent, game: &GameState) {
    match event {
        PixelBusEvent::Heartbeat => game.observe_heartbeat(),
        PixelBusEvent::SignalLost => game.signal_lost(),
        PixelBusEvent::MenuGate(surface) => game.observe_surface(match surface {
            Some(surface) => SurfaceObservation::Observed(surface),
            None => SurfaceObservation::Unavailable,
        }),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::app_toggle_intent;
    use crate::app::UiIntent;
    use crate::input::Action;

    #[test]
    fn suspend_toggle_maps_to_toggle_suspend_regardless_of_the_others() {
        for fishing_on in [false, true] {
            for potion_on in [false, true] {
                match app_toggle_intent(Action::ToggleSuspend, fishing_on, potion_on) {
                    Some(UiIntent::ToggleSuspend) => {}
                    other => panic!(
                        "expected ToggleSuspend (fishing_on={fishing_on}, potion_on={potion_on}, some={})",
                        other.is_some()
                    ),
                }
            }
        }
    }

    #[test]
    fn fishing_toggle_negates_the_current_state() {
        match app_toggle_intent(Action::ToggleFishing, false, false) {
            Some(UiIntent::SetFishing(true)) => {}
            _ => panic!("fishing off must map to SetFishing(true)"),
        }
        match app_toggle_intent(Action::ToggleFishing, true, false) {
            Some(UiIntent::SetFishing(false)) => {}
            _ => panic!("fishing on must map to SetFishing(false)"),
        }
    }

    #[test]
    fn auto_potion_toggle_negates_the_current_state() {
        match app_toggle_intent(Action::ToggleAutoPotion, false, false) {
            Some(UiIntent::SetAutoPotion(true)) => {}
            _ => panic!("auto-potion off must map to SetAutoPotion(true)"),
        }
        match app_toggle_intent(Action::ToggleAutoPotion, false, true) {
            Some(UiIntent::SetAutoPotion(false)) => {}
            _ => panic!("auto-potion on must map to SetAutoPotion(false)"),
        }
    }

    #[test]
    fn each_toggle_reads_only_its_own_state() {
        // Fishing's intent must not depend on the auto-potion state, and vice
        // versa. Two booleans in one signature is exactly where they get crossed.
        match app_toggle_intent(Action::ToggleFishing, false, true) {
            Some(UiIntent::SetFishing(true)) => {}
            _ => panic!("fishing must read the fishing state, not the potion state"),
        }
        match app_toggle_intent(Action::ToggleAutoPotion, true, false) {
            Some(UiIntent::SetAutoPotion(true)) => {}
            _ => panic!("auto-potion must read the potion state, not the fishing state"),
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
                app_toggle_intent(action, false, false).is_none(),
                "{action:?} must not map to an app-toggle intent"
            );
        }
    }
}
