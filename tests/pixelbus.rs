//! Decoder and state-machine tests for the Pixel Bus Reader.

use eso_weave::pixelbus::{
    decode_latency, decode_weapon_bar, fishing_signal, status_present, ActiveBar, FishingSignal,
    PixelBusEvent, PixelBusReader, ReaderConfig, Rgb, WeaponBarSignal, WeaponClass,
};

fn reader() -> PixelBusReader {
    PixelBusReader::new(ReaderConfig::default())
}

const MAGENTA: Rgb = Rgb {
    r: 0xFF,
    g: 0x00,
    b: 0xFF,
};
const WAITING: Rgb = Rgb {
    r: 0x00,
    g: 0x80,
    b: 0xFF,
};
const BITE: Rgb = Rgb {
    r: 0x00,
    g: 0xFF,
    b: 0x00,
};

/// Builds a weapon-bar sample: green marker, red packs front and back class
/// nibbles, blue is the active-bar code.
fn weapon(front: u8, back: u8, bar: u8) -> Rgb {
    Rgb::new((front << 4) | back, 0x5A, bar)
}

// US3: decoders.

#[test]
fn status_present_respects_tolerance() {
    assert!(status_present(MAGENTA, 2));
    assert!(status_present(Rgb::new(253, 2, 253), 2)); // within tolerance
    assert!(!status_present(Rgb::new(252, 0, 255), 2)); // red off by 3
}

#[test]
fn fishing_signal_maps_colors() {
    assert_eq!(fishing_signal(WAITING, 2), FishingSignal::Waiting);
    assert_eq!(fishing_signal(BITE, 2), FishingSignal::Bite);
    assert_eq!(fishing_signal(Rgb::new(10, 10, 10), 2), FishingSignal::None);
}

#[test]
fn decode_latency_validates_marker_and_checksum() {
    // latency 400: red 100, green 0xA5, blue 155.
    assert_eq!(decode_latency(Rgb::new(100, 0xA5, 155), 2), Some(400));
    // clamped maximum 1020: red 255, blue 0.
    assert_eq!(decode_latency(Rgb::new(255, 0xA5, 0), 2), Some(1020));
    // wrong marker.
    assert_eq!(decode_latency(Rgb::new(100, 0x00, 155), 2), None);
    // wrong checksum (red + blue != 255).
    assert_eq!(decode_latency(Rgb::new(100, 0xA5, 100), 2), None);
}

#[test]
fn tolerance_boundary() {
    assert!(status_present(Rgb::new(253, 2, 253), 2)); // shifted by tolerance
    assert!(!status_present(Rgb::new(252, 3, 252), 2)); // shifted by tolerance + 1
}

// Weapon-bar decoding (slice 014).

#[test]
fn decode_weapon_bar_reads_bar_and_classes() {
    // front dual wield (1), back two handed (2), front bar active (1).
    let signal = decode_weapon_bar(weapon(1, 2, 1), 2).unwrap();
    assert_eq!(
        signal,
        WeaponBarSignal {
            bar: ActiveBar::Front,
            front: WeaponClass::DualWield,
            back: WeaponClass::TwoHanded,
        }
    );

    // back bar active (2), back bow (4), front restoration staff (6).
    let signal = decode_weapon_bar(weapon(6, 4, 2), 2).unwrap();
    assert_eq!(signal.bar, ActiveBar::Back);
    assert_eq!(signal.front, WeaponClass::RestorationStaff);
    assert_eq!(signal.back, WeaponClass::Bow);

    // out-of-range codes decode to Unknown.
    let signal = decode_weapon_bar(weapon(9, 0, 7), 2).unwrap();
    assert_eq!(signal.bar, ActiveBar::Unknown);
    assert_eq!(signal.front, WeaponClass::Unknown);
    assert_eq!(signal.back, WeaponClass::Unknown);
}

#[test]
fn decode_weapon_bar_requires_marker_within_tolerance() {
    // Wrong marker: not a weapon block.
    assert_eq!(decode_weapon_bar(Rgb::new(0x12, 0x00, 1), 2), None);
    // Marker shifted by tolerance still decodes; by tolerance + 1 does not.
    assert!(decode_weapon_bar(Rgb::new(0x12, 0x5C, 1), 2).is_some());
    assert!(decode_weapon_bar(Rgb::new(0x12, 0x5D, 1), 2).is_none());
    // The weapon marker never aliases the latency marker.
    assert!(decode_weapon_bar(Rgb::new(0x12, 0xA5, 1), 2).is_none());
}

// US1: heartbeat and signal loss.

#[test]
fn heartbeat_then_signal_loss_then_recovery() {
    let mut r = reader();

    assert!(r
        .observe(Some(MAGENTA), None, None, None, 0)
        .contains(&PixelBusEvent::Heartbeat));
    assert!(!r.signal_lost());

    // Absent but within the timeout: no event.
    assert!(r.observe(None, None, None, None, 1000).is_empty());
    assert!(!r.signal_lost());

    // Absent past the 2000 ms timeout: exactly one SignalLost.
    assert_eq!(
        r.observe(None, None, None, None, 2500),
        vec![PixelBusEvent::SignalLost]
    );
    assert!(r.signal_lost());

    // Still absent: no further events.
    assert!(r.observe(None, None, None, None, 5000).is_empty());

    // Status returns: heartbeat and lost state cleared.
    assert!(r
        .observe(Some(MAGENTA), None, None, None, 6000)
        .contains(&PixelBusEvent::Heartbeat));
    assert!(!r.signal_lost());
}

#[test]
fn fishing_and_latency_not_decoded_without_heartbeat() {
    let mut r = reader();
    let events = r.observe(None, Some(WAITING), Some(Rgb::new(100, 0xA5, 155)), None, 0);
    assert!(events.is_empty());
}

#[test]
fn weapon_bar_not_decoded_without_heartbeat() {
    let mut r = reader();
    let events = r.observe(None, None, None, Some(weapon(1, 2, 1)), 0);
    assert!(events.is_empty());
}

// US2: fishing transitions and latency.

#[test]
fn fishing_transitions_emit_events() {
    let mut r = reader();

    assert!(r
        .observe(Some(MAGENTA), Some(WAITING), None, None, 0)
        .contains(&PixelBusEvent::FishingStarted));
    assert!(r
        .observe(Some(MAGENTA), Some(BITE), None, None, 100)
        .contains(&PixelBusEvent::BiteDetected));
    // Recast: bite back to waiting is a new cast.
    assert!(r
        .observe(Some(MAGENTA), Some(WAITING), None, None, 200)
        .contains(&PixelBusEvent::FishingStarted));
    assert!(r
        .observe(Some(MAGENTA), None, None, None, 300)
        .contains(&PixelBusEvent::FishingStopped));
}

#[test]
fn latency_event_emitted_with_heartbeat() {
    let mut r = reader();
    let events = r.observe(Some(MAGENTA), None, Some(Rgb::new(100, 0xA5, 155)), None, 0);
    assert!(events.contains(&PixelBusEvent::Latency(400)));
}

#[test]
fn weapon_bar_event_only_on_change() {
    let mut r = reader();

    // First decode with a heartbeat emits the event.
    let first = r.observe(Some(MAGENTA), None, None, Some(weapon(1, 2, 1)), 0);
    assert!(first.contains(&PixelBusEvent::WeaponBar(WeaponBarSignal {
        bar: ActiveBar::Front,
        front: WeaponClass::DualWield,
        back: WeaponClass::TwoHanded,
    })));

    // The same signal does not re-emit (per-attack redraws must not churn).
    let repeat = r.observe(Some(MAGENTA), None, None, Some(weapon(1, 2, 1)), 100);
    assert!(!repeat
        .iter()
        .any(|e| matches!(e, PixelBusEvent::WeaponBar(_))));

    // A real change (bar swap to back) emits again.
    let changed = r.observe(Some(MAGENTA), None, None, Some(weapon(1, 2, 2)), 200);
    assert!(changed
        .iter()
        .any(|e| matches!(e, PixelBusEvent::WeaponBar(_))));
}
