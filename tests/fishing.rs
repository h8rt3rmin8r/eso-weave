//! State-machine and safety tests for the Fishing Controller.
//!
//! The safety-critical behavior (SignalLost disables fishing rather than
//! blind-firing, and pending interacts are cancelled on leaving their state) is
//! exercised here against a stub detector, a mock sink, and an injected clock,
//! per constitution Principle II.

use eso_weave::config::NoticeKind;
use eso_weave::fishing::{
    map_event, BiteDetector, DetectorEvent, FishingConfig, FishingController, FishingState,
    MockFishingSink, RealFishingSink, StopReason, StubDetector,
};
use eso_weave::input::mock::MockBackend;
use eso_weave::input::{BindingTable, InputEngine, Key, Transition};
use eso_weave::pixelbus::{
    LifeState, MockSampler, PixelBusReader, ReaderConfig, Rgb, TravelState, WorldState,
};

fn controller() -> FishingController {
    let mut controller = FishingController::new(FishingConfig::default());
    let mut sink = MockFishingSink::new();
    controller.set_game_environment(true, true, 0, &mut sink);
    controller.set_life_state(LifeState::Alive);
    controller.set_world_state(WorldState::Active);
    controller.set_travel_state(TravelState::Inactive);
    controller
}

#[test]
fn non_alive_cancels_pending_fishing_without_replay_and_keeps_request() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 10, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 20, &mut sink);
    sink.clear();

    c.set_life_state(LifeState::Dead);
    assert!(c.enabled());
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::PlayerUnavailable));
    c.tick(20 + u64::from(cfg.reel_delay_ms), &mut sink);
    assert!(sink.ops.is_empty());

    c.set_life_state(LifeState::Alive);
    assert!(sink.ops.is_empty(), "alive must not replay the reel");
    c.on_event(DetectorEvent::FishingStarted, 10_000, &mut sink);
    assert_eq!(c.state(), FishingState::Waiting);
    assert!(
        sink.ops.is_empty(),
        "a manual fresh cast is observed, not synthesized"
    );
}

#[test]
fn pending_travel_cancels_fishing_without_replay_and_names_the_reason() {
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 10, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 20, &mut sink);
    sink.clear();

    c.set_travel_state(TravelState::Pending);
    assert!(c.enabled());
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::TravelPending));
    c.tick(10_000, &mut sink);
    c.set_travel_state(TravelState::Inactive);
    assert!(
        sink.ops.is_empty(),
        "travel recovery must not replay the reel"
    );
}

#[test]
fn shared_life_gate_blocks_enable_before_controller_routing_catches_up() {
    let cfg = FishingConfig::default();
    let (input, _rx) = InputEngine::new(BindingTable::default(), 4);
    let mut c = FishingController::with_life_gate(cfg, input.life_gate());
    let mut sink = MockFishingSink::new();
    c.set_game_environment(true, true, 0, &mut sink);
    c.set_life_state(LifeState::Alive);

    input.set_life_gated(true);
    c.set_enabled(true, 10, &mut sink);

    assert!(c.enabled());
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::PlayerUnavailable));
    assert!(sink.ops.is_empty());
}

#[test]
fn shared_travel_gate_blocks_enable_before_controller_routing_catches_up() {
    let (input, _rx) = InputEngine::new(BindingTable::default(), 4);
    input.set_world_gated(false);
    input.set_travel_gated(false);
    let mut c = FishingController::with_safety_gates(
        FishingConfig::default(),
        input.life_gate(),
        input.world_travel_gate(),
    );
    let mut sink = MockFishingSink::new();
    c.set_game_environment(true, true, 0, &mut sink);
    c.set_life_state(LifeState::Alive);
    c.set_world_state(WorldState::Active);
    c.set_travel_state(TravelState::Inactive);

    input.set_travel_gated(true);
    c.set_enabled(true, 10, &mut sink);

    assert!(c.enabled());
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::TravelPending));
    assert!(sink.ops.is_empty());
}

fn press_release(key: Key) -> Vec<(Key, Transition)> {
    vec![(key, Transition::Down), (key, Transition::Up)]
}

// US1: full cast-reel-recast cycle.

#[test]
fn cast_reel_recast_cycle() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();

    c.set_enabled(true, 0, &mut sink);
    assert_eq!(c.state(), FishingState::Armed);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
    sink.clear();

    c.on_event(DetectorEvent::FishingStarted, 10, &mut sink);
    assert_eq!(c.state(), FishingState::Waiting);
    assert!(sink.ops.is_empty());

    c.on_event(DetectorEvent::BiteDetected, 20, &mut sink);
    assert_eq!(c.state(), FishingState::Reeling);
    assert!(sink.ops.is_empty());

    // Not yet due.
    c.tick(20 + u64::from(cfg.reel_delay_ms) - 1, &mut sink);
    assert!(sink.ops.is_empty());
    // Reel fires.
    let reel_at = 20 + u64::from(cfg.reel_delay_ms);
    c.tick(reel_at, &mut sink);
    assert_eq!(c.state(), FishingState::Recast);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
    sink.clear();

    // Recast fires.
    let recast_at = reel_at + u64::from(cfg.recast_delay_ms);
    c.tick(recast_at, &mut sink);
    assert_eq!(c.state(), FishingState::Recast);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
    sink.clear();

    // A new FishingStarted continues into Waiting.
    c.on_event(DetectorEvent::FishingStarted, recast_at + 5, &mut sink);
    assert_eq!(c.state(), FishingState::Waiting);
    assert!(sink.ops.is_empty());
}

#[test]
fn recast_timeout_returns_to_armed_and_recasts() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();

    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 5, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 10, &mut sink);
    let reel_at = 10 + u64::from(cfg.reel_delay_ms);
    c.tick(reel_at, &mut sink);
    let recast_at = reel_at + u64::from(cfg.recast_delay_ms);
    c.tick(recast_at, &mut sink);
    sink.clear();

    // No FishingStarted within arm_timeout of the recast: re-cast to Armed.
    c.tick(recast_at + u64::from(cfg.arm_timeout_ms), &mut sink);
    assert_eq!(c.state(), FishingState::Armed);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
}

#[test]
fn bite_while_armed_is_handled_defensively() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    sink.clear();

    // A bite before FishingStarted still reels.
    c.on_event(DetectorEvent::BiteDetected, 30, &mut sink);
    assert_eq!(c.state(), FishingState::Reeling);
    c.tick(30 + u64::from(cfg.reel_delay_ms), &mut sink);
    assert_eq!(c.state(), FishingState::Recast);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
}

// US2: signal loss disables fishing safely (safety-critical).

#[test]
fn signal_lost_from_every_active_state_disables_without_emitting() {
    let cfg = FishingConfig::default();

    // Armed.
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    sink.clear();
    c.on_event(DetectorEvent::SignalLost, 1, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());

    // Waiting.
    let mut c = controller();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 1, &mut sink);
    sink.clear();
    c.on_event(DetectorEvent::SignalLost, 2, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());

    // Reeling, with a pending reel deadline that must not fire.
    let mut c = controller();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 1, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 2, &mut sink);
    sink.clear();
    c.on_event(DetectorEvent::SignalLost, 3, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    c.tick(2 + u64::from(cfg.reel_delay_ms) + 1, &mut sink);
    assert!(sink.ops.is_empty(), "no scheduled reel after signal loss");

    // Recast, with a pending recast deadline that must not fire.
    let mut c = controller();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 1, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 2, &mut sink);
    c.tick(2 + u64::from(cfg.reel_delay_ms), &mut sink);
    assert_eq!(c.state(), FishingState::Recast);
    sink.clear();
    c.on_event(DetectorEvent::SignalLost, 100, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    c.tick(100 + u64::from(cfg.recast_delay_ms) + 1, &mut sink);
    assert!(sink.ops.is_empty(), "no scheduled recast after signal loss");
}

#[test]
fn nothing_is_emitted_while_disabled() {
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    // Ticks and events while Disabled do nothing.
    c.tick(10_000, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 10_000, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 10_000, &mut sink);
    c.on_event(DetectorEvent::Heartbeat, 10_000, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());
}

// US3: arm and disarm control.

#[test]
fn arm_timeout_disarms() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    sink.clear();
    // No FishingStarted within arm_timeout.
    c.tick(u64::from(cfg.arm_timeout_ms), &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());
}

#[test]
fn disable_from_any_state_clears_pending_and_emits_nothing() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 1, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 2, &mut sink);
    sink.clear();

    c.set_enabled(false, 3, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());
    // The cancelled reel does not fire.
    c.tick(2 + u64::from(cfg.reel_delay_ms) + 1, &mut sink);
    assert!(sink.ops.is_empty());
}

#[test]
fn fishing_stopped_from_waiting_recasts() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 1, &mut sink);
    sink.clear();

    c.on_event(DetectorEvent::FishingStopped, 2, &mut sink);
    assert_eq!(c.state(), FishingState::Armed);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
}

#[test]
fn toggles_are_idempotent() {
    let mut c = controller();
    let mut sink = MockFishingSink::new();

    c.set_enabled(true, 0, &mut sink);
    let after_first = sink.ops.len();
    c.set_enabled(true, 1, &mut sink); // redundant on
    assert_eq!(
        sink.ops.len(),
        after_first,
        "redundant enable emits no second cast"
    );

    // Redundant off while Disabled emits nothing.
    let mut c2 = controller();
    let mut sink2 = MockFishingSink::new();
    c2.set_enabled(false, 0, &mut sink2);
    assert_eq!(c2.state(), FishingState::Disabled);
    assert!(sink2.ops.is_empty());
}

// US4: configuration and stop reasons.

#[test]
fn default_timeouts_are_tuned() {
    let cfg = FishingConfig::default();
    assert_eq!(cfg.arm_timeout_ms, 8000);
    assert_eq!(cfg.reel_delay_ms, 100);
    assert_eq!(cfg.recast_delay_ms, 3000);
}

#[test]
fn stop_reason_records_why_fishing_ended() {
    let cfg = FishingConfig::default();
    let mut sink = MockFishingSink::new();

    // A fresh controller has no stop reason.
    let mut c = controller();
    assert_eq!(c.stop_reason(), None);

    // A user stop is recorded, and a fresh cast clears the reason.
    c.set_enabled(true, 0, &mut sink);
    assert_eq!(c.stop_reason(), None, "a cast clears any prior reason");
    c.set_enabled(false, 1, &mut sink);
    assert_eq!(c.stop_reason(), Some(StopReason::UserStop));

    // An arm timeout with no cast confirmation records NoCastDetected.
    let mut c = controller();
    c.set_enabled(true, 0, &mut sink);
    c.tick(u64::from(cfg.arm_timeout_ms), &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::NoCastDetected));
    // Starting again clears the prior reason.
    c.set_enabled(true, 10_000, &mut sink);
    assert_eq!(c.stop_reason(), None);

    // Signal loss records SignalLost.
    let mut c = controller();
    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::SignalLost, 1, &mut sink);
    assert_eq!(c.stop_reason(), Some(StopReason::SignalLost));
}

#[test]
fn config_round_trips_and_defaults() {
    let mut notices = Vec::new();
    assert_eq!(
        FishingConfig::load(&serde_json::Value::Null, &mut notices),
        FishingConfig::default()
    );
    assert!(notices.is_empty());

    let custom = FishingConfig {
        arm_timeout_ms: 4000,
        reel_delay_ms: 150,
        recast_delay_ms: 2500,
        interact_key: Key::R,
    };
    let value = custom.store();
    let mut notices = Vec::new();
    assert_eq!(FishingConfig::load(&value, &mut notices), custom);
    assert!(notices.is_empty());
}

#[test]
fn invalid_config_values_fall_back_with_notice() {
    let mut notices = Vec::new();
    let value = serde_json::json!({
        "arm_timeout_ms": 10_000_000,
        "interact_key": "nope",
    });
    let cfg = FishingConfig::load(&value, &mut notices);
    let defaults = FishingConfig::default();
    assert_eq!(cfg.arm_timeout_ms, defaults.arm_timeout_ms);
    assert_eq!(cfg.interact_key, defaults.interact_key);
    assert_eq!(notices.len(), 2);
    assert!(notices.iter().all(|n| n.kind == NoticeKind::InvalidValue));
}

// Detector abstraction and adapter.

#[test]
fn map_event_drops_latency_and_maps_the_rest() {
    use eso_weave::pixelbus::PixelBusEvent;
    assert_eq!(map_event(PixelBusEvent::Latency(40)), None);
    assert_eq!(
        map_event(PixelBusEvent::Heartbeat),
        Some(DetectorEvent::Heartbeat)
    );
    assert_eq!(
        map_event(PixelBusEvent::SignalLost),
        Some(DetectorEvent::SignalLost)
    );
    assert_eq!(
        map_event(PixelBusEvent::FishingStarted),
        Some(DetectorEvent::FishingStarted)
    );
    assert_eq!(
        map_event(PixelBusEvent::BiteDetected),
        Some(DetectorEvent::BiteDetected)
    );
    assert_eq!(
        map_event(PixelBusEvent::FishingStopped),
        Some(DetectorEvent::FishingStopped)
    );
}

#[test]
fn stub_detector_drives_the_controller() {
    let mut detector = StubDetector::new();
    detector.push_batch(vec![DetectorEvent::FishingStarted]);
    detector.push_batch(vec![DetectorEvent::BiteDetected]);

    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    sink.clear();

    for event in detector.poll(1) {
        c.on_event(event, 1, &mut sink);
    }
    assert_eq!(c.state(), FishingState::Waiting);
    for event in detector.poll(2) {
        c.on_event(event, 2, &mut sink);
    }
    assert_eq!(c.state(), FishingState::Reeling);
}

#[test]
fn pixel_bus_detector_maps_reader_events_and_drops_latency() {
    use eso_weave::fishing::PixelBusDetector;

    let mut sampler = MockSampler::new();
    // Status point magenta -> heartbeat; latency point valid -> Latency (dropped).
    sampler.set(8, 8, Rgb::new(0xFF, 0x00, 0xFF));
    sampler.set(40, 8, Rgb::new(10, 0xA5, 245)); // marker 0xA5, r+b = 255
    let mut detector = PixelBusDetector::new(PixelBusReader::new(ReaderConfig::default()), sampler);

    // Only the heartbeat surfaces; the valid latency sample is dropped.
    let events = detector.poll(0);
    assert_eq!(events, vec![DetectorEvent::Heartbeat]);
}

// Real sink drives the input backend.

#[test]
fn real_sink_drives_the_input_backend() {
    let backend = MockBackend::new();
    let recorded = backend.synthesized.clone();
    let mut sink = RealFishingSink::new(backend);

    let mut c = controller();
    c.set_enabled(true, 0, &mut sink);

    let ops = recorded.lock().unwrap().clone();
    assert_eq!(ops, press_release(FishingConfig::default().interact_key));
}

// Slice 032: the menu gate on the fishing synthesis path.
//
// This path does not go through the interception decision at all: the controller
// presses the interact key on its own timers in response to beacon events. A gate
// placed only on interception would leave it free to send a reel into a chat
// message the operator is composing, and waiting for a bite is exactly when
// someone opens chat.

#[test]
fn a_gated_controller_defers_the_reel_instead_of_sending_it() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();

    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 10, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 20, &mut sink);
    sink.clear();

    // A surface opens before the reel is due.
    c.set_gated(true);
    let reel_at = 20 + u64::from(cfg.reel_delay_ms);
    c.tick(reel_at, &mut sink);
    assert!(
        sink.ops.is_empty(),
        "a gated controller must not send the reel interact"
    );
    assert_eq!(
        c.state(),
        FishingState::Reeling,
        "state must not advance past an interact the game never received"
    );

    // Still gated a while later: still nothing, and still consistent.
    c.tick(reel_at + 1_000, &mut sink);
    assert!(sink.ops.is_empty());
    assert_eq!(c.state(), FishingState::Reeling);

    // The surface closes and the deferred reel goes through on its own.
    c.set_gated(false);
    c.tick(reel_at + 2_000, &mut sink);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
    assert_eq!(c.state(), FishingState::Recast);
}

#[test]
fn a_gated_controller_defers_the_recast_too() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();

    c.set_enabled(true, 0, &mut sink);
    c.on_event(DetectorEvent::FishingStarted, 10, &mut sink);
    c.on_event(DetectorEvent::BiteDetected, 20, &mut sink);
    let reel_at = 20 + u64::from(cfg.reel_delay_ms);
    c.tick(reel_at, &mut sink);
    sink.clear();

    c.set_gated(true);
    let recast_at = reel_at + u64::from(cfg.recast_delay_ms);
    c.tick(recast_at, &mut sink);
    assert!(sink.ops.is_empty(), "recast must be deferred while gated");

    c.set_gated(false);
    c.tick(recast_at + 500, &mut sink);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
}

#[test]
fn an_ungated_controller_is_unchanged() {
    // The default is ungated, so every other test in this file already asserts
    // the pre-feature behavior. This states it explicitly.
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    assert_eq!(
        sink.ops,
        press_release(FishingConfig::default().interact_key)
    );
}

#[test]
fn the_arm_timeout_still_fires_while_gated() {
    // Only the two autonomous interacts are deferred. A timeout that merely ends
    // the session must not be, or a gated session could hang forever waiting for
    // a cast confirmation that will never come.
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();

    c.set_enabled(true, 0, &mut sink);
    sink.clear();
    c.set_gated(true);
    c.tick(u64::from(cfg.arm_timeout_ms), &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());
}

#[test]
fn game_exit_disables_an_active_session_without_emitting() {
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    sink.clear();
    c.set_game_environment(false, false, 1, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::GameInactive));
    assert!(c.enabled(), "runtime exit must preserve requested fishing");
    assert!(sink.ops.is_empty());
}

#[test]
fn inactive_game_refuses_the_initial_cast() {
    let mut c = FishingController::new(FishingConfig::default());
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::GameInactive));
    assert!(c.enabled(), "the request is paused rather than destroyed");
    assert!(sink.ops.is_empty());
}

#[test]
fn game_return_waits_for_fresh_safety_and_manual_cast_evidence() {
    let cfg = FishingConfig::default();
    let mut c = FishingController::new(cfg);
    let mut sink = MockFishingSink::new();

    c.set_enabled(true, 0, &mut sink);
    assert!(sink.ops.is_empty());

    c.set_game_environment(true, true, 1000, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());

    c.set_life_state(LifeState::Alive);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(
        sink.ops.is_empty(),
        "Alive alone must not replay the old cast"
    );
    c.set_world_state(WorldState::Active);
    c.set_travel_state(TravelState::Inactive);
    assert!(
        sink.ops.is_empty(),
        "safe telemetry must not replay the old cast"
    );

    c.on_event(DetectorEvent::FishingStarted, 1100, &mut sink);
    assert_eq!(c.state(), FishingState::Waiting);
    assert!(
        sink.ops.is_empty(),
        "the fresh cast was manual and is only observed"
    );
}

#[test]
fn focus_loss_pauses_and_refocus_rearms_requested_fishing() {
    let cfg = FishingConfig::default();
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    sink.clear();

    c.set_game_environment(true, false, 10, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert_eq!(c.stop_reason(), Some(StopReason::Unfocused));
    assert!(c.enabled());
    assert!(sink.ops.is_empty());

    c.set_game_environment(true, true, 20, &mut sink);
    assert_eq!(c.state(), FishingState::Armed);
    assert_eq!(sink.ops, press_release(cfg.interact_key));
}

#[test]
fn signal_loss_while_focus_paused_applies_the_existing_reset_policy() {
    let mut c = controller();
    let mut sink = MockFishingSink::new();
    c.set_enabled(true, 0, &mut sink);
    c.set_game_environment(true, false, 10, &mut sink);

    c.on_event(DetectorEvent::SignalLost, 20, &mut sink);
    assert!(!c.enabled());
    assert_eq!(c.stop_reason(), Some(StopReason::SignalLost));

    sink.clear();
    c.set_game_environment(true, true, 30, &mut sink);
    assert_eq!(c.state(), FishingState::Disabled);
    assert!(sink.ops.is_empty());
}
