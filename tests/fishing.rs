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
use eso_weave::input::{Key, Transition};
use eso_weave::pixelbus::{MockSampler, PixelBusReader, ReaderConfig, Rgb};

fn controller() -> FishingController {
    FishingController::new(FishingConfig::default())
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
