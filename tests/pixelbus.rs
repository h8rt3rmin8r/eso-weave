//! Decoder and state-machine tests for the Pixel Bus Reader.

use eso_weave::config::NoticeKind;
use eso_weave::pixelbus::{
    block_center, capture_dims, decode_latency, decode_weapon_bar, fishing_signal,
    load_reader_config, poll_interval, sanitize_block_px, status_present, store_reader_config,
    strip_pixel, ActiveBar, FishingSignal, MockSampler, PixelBusEvent, PixelBusReader,
    ReaderConfig, Rgb, WeaponBarSignal, WeaponClass, DEFAULT_BLOCK_PX, MAX_BLOCK_PX, MIN_BLOCK_PX,
    NUM_BLOCKS,
};

// Pixel extraction from a captured BGRA strip (the Windows screen-composited
// capture path). The bytes are blue, green, red, alpha per pixel.

#[test]
fn strip_pixel_decodes_bgra_channel_order() {
    // A 2x2 strip: (0,0) magenta FF00FF, (1,0) green 00FF00, (0,1) blue 0000FF,
    // (1,1) white FFFFFF. Stored BGRA.
    let buf: [u8; 16] = [
        0xFF, 0x00, 0xFF, 0x00, // (0,0) B=FF G=00 R=FF -> magenta
        0x00, 0xFF, 0x00, 0x00, // (1,0) B=00 G=FF R=00 -> green
        0xFF, 0x00, 0x00, 0x00, // (0,1) B=FF G=00 R=00 -> blue
        0xFF, 0xFF, 0xFF, 0x00, // (1,1) B=FF G=FF R=FF -> white
    ];
    assert_eq!(
        strip_pixel(&buf, 2, 2, 0, 0),
        Some(Rgb::new(0xFF, 0x00, 0xFF))
    );
    assert_eq!(
        strip_pixel(&buf, 2, 2, 1, 0),
        Some(Rgb::new(0x00, 0xFF, 0x00))
    );
    assert_eq!(
        strip_pixel(&buf, 2, 2, 0, 1),
        Some(Rgb::new(0x00, 0x00, 0xFF))
    );
    assert_eq!(
        strip_pixel(&buf, 2, 2, 1, 1),
        Some(Rgb::new(0xFF, 0xFF, 0xFF))
    );
}

#[test]
fn strip_pixel_rejects_out_of_range() {
    let buf = [0u8; 16];
    assert_eq!(strip_pixel(&buf, 2, 2, 2, 0), None);
    assert_eq!(strip_pixel(&buf, 2, 2, 0, 2), None);
}

#[test]
fn strip_pixel_rejects_truncated_buffer() {
    // A buffer shorter than the addressed pixel returns None rather than panicking.
    let buf = [0u8; 3];
    assert_eq!(strip_pixel(&buf, 2, 2, 1, 1), None);
}

#[test]
fn poll_interval_tracks_fishing_state() {
    let cfg = ReaderConfig::default();
    assert_eq!(poll_interval(true, &cfg), cfg.interval_fishing_ms);
    assert_eq!(poll_interval(false, &cfg), cfg.interval_idle_ms);

    // A custom config is honored, not the defaults.
    let cfg = ReaderConfig {
        interval_fishing_ms: 75,
        interval_idle_ms: 1500,
        ..ReaderConfig::default()
    };
    assert_eq!(poll_interval(true, &cfg), 75);
    assert_eq!(poll_interval(false, &cfg), 1500);
}

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

// Slice 028: block-size single source of truth.

#[test]
fn block_center_and_capture_dims_match_contract_table() {
    // (block_px, [B0, B1, B2, B3 centers], (capture_w, capture_h)) per
    // specs/028-pixelbus-block-size/contracts/geometry.md.
    let cases = [
        (2u32, [(1u32, 1u32), (3, 1), (5, 1), (7, 1)], (8u32, 2u32)),
        (4, [(2, 2), (6, 2), (10, 2), (14, 2)], (16, 4)),
        (8, [(4, 4), (12, 4), (20, 4), (28, 4)], (32, 8)),
        (16, [(8, 8), (24, 8), (40, 8), (56, 8)], (64, 16)),
        (32, [(16, 16), (48, 16), (80, 16), (112, 16)], (128, 32)),
    ];
    for (block_px, centers, cap) in cases {
        for (index, expected) in centers.iter().enumerate() {
            assert_eq!(
                block_center(block_px, index as u32),
                *expected,
                "block_px {block_px} index {index}"
            );
        }
        assert_eq!(
            capture_dims(block_px),
            cap,
            "capture dims block_px {block_px}"
        );
    }
    assert_eq!(NUM_BLOCKS, 4);
    assert_eq!(DEFAULT_BLOCK_PX, 16);
}

#[test]
fn sanitize_block_px_corrects_and_notices() {
    // A valid even value in range is unchanged and records no notice.
    let mut notices = Vec::new();
    assert_eq!(sanitize_block_px(16, &mut notices), 16);
    assert_eq!(sanitize_block_px(MIN_BLOCK_PX, &mut notices), MIN_BLOCK_PX);
    assert_eq!(sanitize_block_px(MAX_BLOCK_PX, &mut notices), MAX_BLOCK_PX);
    assert!(notices.is_empty(), "in-range even values record no notice");

    // Odd rounds down to the next even, with a notice.
    let mut notices = Vec::new();
    assert_eq!(sanitize_block_px(15, &mut notices), 14);
    assert_eq!(notices.len(), 1);
    assert_eq!(notices[0].kind, NoticeKind::InvalidValue);

    // Below range clamps to MIN; above range clamps to MAX (odd or even).
    assert_eq!(sanitize_block_px(1, &mut Vec::new()), MIN_BLOCK_PX);
    assert_eq!(sanitize_block_px(0, &mut Vec::new()), MIN_BLOCK_PX);
    assert_eq!(sanitize_block_px(33, &mut Vec::new()), MAX_BLOCK_PX);
    assert_eq!(sanitize_block_px(1000, &mut Vec::new()), MAX_BLOCK_PX);
}

#[test]
fn reader_config_default_geometry_matches_release() {
    let cfg = ReaderConfig::default();
    assert_eq!(cfg.block_px, 16);
    assert_eq!(cfg.status_point(), (8, 8));
    assert_eq!(cfg.fishing_point(), (24, 8));
    assert_eq!(cfg.latency_point(), (40, 8));
    assert_eq!(cfg.weapon_point(), (56, 8));
}

#[test]
fn reader_config_points_track_block_px() {
    for block_px in [2u32, 4, 8, 16, 32] {
        let cfg = ReaderConfig {
            block_px,
            ..ReaderConfig::default()
        };
        assert_eq!(cfg.status_point(), block_center(block_px, 0));
        assert_eq!(cfg.fishing_point(), block_center(block_px, 1));
        assert_eq!(cfg.latency_point(), block_center(block_px, 2));
        assert_eq!(cfg.weapon_point(), block_center(block_px, 3));
    }
}

#[test]
fn sample_and_observe_reads_derived_points() {
    // At block_px = 8 the status center is (4, 4); a heartbeat seeded there is
    // read, proving the runtime path samples the derived points.
    let cfg = ReaderConfig {
        block_px: 8,
        ..ReaderConfig::default()
    };
    let (sx, sy) = cfg.status_point();
    let mut sampler = MockSampler::new();
    sampler.set(sx, sy, MAGENTA);
    let mut r = PixelBusReader::new(cfg);
    let events = r.sample_and_observe(&sampler, 0);
    assert!(events.contains(&PixelBusEvent::Heartbeat));
}

#[test]
fn block_px_round_trips_through_settings() {
    let cfg = ReaderConfig {
        block_px: 8,
        ..ReaderConfig::default()
    };
    let value = store_reader_config(&cfg);
    let mut notices = Vec::new();
    let loaded = load_reader_config(&value, &mut notices);
    assert_eq!(loaded.block_px, 8);
    assert!(notices.is_empty());
}

#[test]
fn older_config_without_block_px_defaults_to_sixteen() {
    let value = serde_json::json!({
        "tolerance": 2,
        "interval_fishing_ms": 100,
        "interval_idle_ms": 1000,
    });
    let mut notices = Vec::new();
    let loaded = load_reader_config(&value, &mut notices);
    assert_eq!(loaded.block_px, DEFAULT_BLOCK_PX);
    assert!(notices.is_empty());
}

#[test]
fn load_reader_config_sanitizes_invalid_block_px() {
    // An odd persisted value is corrected with a notice, never a crash.
    let value = serde_json::json!({ "block_px": 15 });
    let mut notices = Vec::new();
    let loaded = load_reader_config(&value, &mut notices);
    assert_eq!(loaded.block_px, 14);
    assert!(notices.iter().any(|n| n.kind == NoticeKind::InvalidValue));

    // A wrong-typed value falls back to the default with no crash.
    let value = serde_json::json!({ "block_px": "huge" });
    let mut notices = Vec::new();
    let loaded = load_reader_config(&value, &mut notices);
    assert_eq!(loaded.block_px, DEFAULT_BLOCK_PX);
}
