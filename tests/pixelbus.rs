//! Decoder and state-machine tests for the Pixel Bus Reader.

use eso_weave::config::NoticeKind;
use eso_weave::pixelbus::{
    block_center, capture_dims, decode_combat, decode_latency, decode_menu, decode_resource,
    decode_resources, decode_weapon_bar, fishing_signal, grid_extent, grid_position, grid_rows,
    load_reader_config, poll_interval, sanitize_block_px, status_present, store_reader_config,
    strip_pixel, ActiveBar, BlockSamples, CombatSignal, FishingSignal, MenuSurface, MockSampler,
    PixelBusEvent, PixelBusReader, ReaderConfig, ResourceLevel, ResourceSet, Rgb, Size,
    WeaponBarSignal, WeaponClass, BLOCK_CENTER_GREENS, COLUMNS, DEFAULT_BLOCK_PX, MAX_BLOCK_PX,
    MIN_BLOCK_PX, NUM_BLOCKS,
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
    assert_eq!(poll_interval(true, false, &cfg), cfg.interval_fishing_ms);
    assert_eq!(poll_interval(false, false, &cfg), cfg.interval_idle_ms);

    // A custom config is honored, not the defaults.
    let cfg = ReaderConfig {
        interval_fishing_ms: 75,
        interval_idle_ms: 1500,
        ..ReaderConfig::default()
    };
    assert_eq!(poll_interval(true, false, &cfg), 75);
    assert_eq!(poll_interval(false, false, &cfg), 1500);
}

#[test]
fn poll_interval_is_fast_whenever_the_gate_matters() {
    // Slice 032: the menu gate is useless if it engages a second late, so the
    // fast cadence also applies whenever the application can intercept.
    let cfg = ReaderConfig {
        interval_fishing_ms: 75,
        interval_idle_ms: 1500,
        ..ReaderConfig::default()
    };
    assert_eq!(poll_interval(true, true, &cfg), 75);
    assert_eq!(
        poll_interval(false, true, &cfg),
        75,
        "gate needs the fast cadence"
    );
    assert_eq!(poll_interval(true, false, &cfg), 75);

    // And the idle setting is still reachable, so it is not dead configuration.
    // This is the assertion that would fail if someone "simplified" the condition
    // to always-fast, which is the mirror image of the bug slice 016 fixed.
    assert_eq!(
        poll_interval(false, false, &cfg),
        1500,
        "a suspended application with no fishing session has no gate to keep current"
    );
}

fn reader() -> PixelBusReader {
    PixelBusReader::new(ReaderConfig::default())
}

/// A sample set carrying only the status block, so the reader sees a live beacon
/// and every other block reads as absent. Written as a base for struct-update
/// syntax (`BlockSamples { fishing: Some(..), ..alive() }`), which is what lets a
/// later block be added without touching these call sites.
fn alive() -> BlockSamples {
    BlockSamples {
        status: Some(MAGENTA),
        ..Default::default()
    }
}

/// The combat block color for a state, mirroring the addon's encoder: the state
/// code in red, the marker in green, and the complement checksum in blue.
fn combat(red: u8) -> Rgb {
    Rgb::new(red, 0x2D, 255 - red)
}

const COMBAT_IN: Rgb = Rgb {
    r: 0xE0,
    g: 0x2D,
    b: 0x1F,
};
const COMBAT_OUT: Rgb = Rgb {
    r: 0x20,
    g: 0x2D,
    b: 0xDF,
};

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

    assert!(r.observe(alive(), 0).contains(&PixelBusEvent::Heartbeat));
    assert!(!r.signal_lost());

    // Absent but within the timeout: no event.
    assert!(r.observe(BlockSamples::default(), 1000).is_empty());
    assert!(!r.signal_lost());

    // Absent past the 2000 ms timeout: exactly one SignalLost.
    assert_eq!(
        r.observe(BlockSamples::default(), 2500),
        vec![PixelBusEvent::SignalLost]
    );
    assert!(r.signal_lost());

    // Still absent: no further events.
    assert!(r.observe(BlockSamples::default(), 5000).is_empty());

    // Status returns: heartbeat and lost state cleared.
    assert!(r.observe(alive(), 6000).contains(&PixelBusEvent::Heartbeat));
    assert!(!r.signal_lost());
}

#[test]
fn fishing_and_latency_not_decoded_without_heartbeat() {
    let mut r = reader();
    let events = r.observe(
        BlockSamples {
            fishing: Some(WAITING),
            latency: Some(Rgb::new(100, 0xA5, 155)),
            ..Default::default()
        },
        0,
    );
    assert!(events.is_empty());
}

#[test]
fn weapon_bar_not_decoded_without_heartbeat() {
    let mut r = reader();
    let events = r.observe(
        BlockSamples {
            weapon: Some(weapon(1, 2, 1)),
            ..Default::default()
        },
        0,
    );
    assert!(events.is_empty());
}

// US2: fishing transitions and latency.

#[test]
fn fishing_transitions_emit_events() {
    let mut r = reader();

    assert!(r
        .observe(
            BlockSamples {
                fishing: Some(WAITING),
                ..alive()
            },
            0
        )
        .contains(&PixelBusEvent::FishingStarted));
    assert!(r
        .observe(
            BlockSamples {
                fishing: Some(BITE),
                ..alive()
            },
            100
        )
        .contains(&PixelBusEvent::BiteDetected));
    // Recast: bite back to waiting is a new cast.
    assert!(r
        .observe(
            BlockSamples {
                fishing: Some(WAITING),
                ..alive()
            },
            200
        )
        .contains(&PixelBusEvent::FishingStarted));
    assert!(r
        .observe(alive(), 300)
        .contains(&PixelBusEvent::FishingStopped));
}

#[test]
fn latency_event_emitted_with_heartbeat() {
    let mut r = reader();
    let events = r.observe(
        BlockSamples {
            latency: Some(Rgb::new(100, 0xA5, 155)),
            ..alive()
        },
        0,
    );
    assert!(events.contains(&PixelBusEvent::Latency(400)));
}

#[test]
fn weapon_bar_event_only_on_change() {
    let mut r = reader();

    // First decode with a heartbeat emits the event.
    let first = r.observe(
        BlockSamples {
            weapon: Some(weapon(1, 2, 1)),
            ..alive()
        },
        0,
    );
    assert!(first.contains(&PixelBusEvent::WeaponBar(WeaponBarSignal {
        bar: ActiveBar::Front,
        front: WeaponClass::DualWield,
        back: WeaponClass::TwoHanded,
    })));

    // The same signal does not re-emit (per-attack redraws must not churn).
    let repeat = r.observe(
        BlockSamples {
            weapon: Some(weapon(1, 2, 1)),
            ..alive()
        },
        100,
    );
    assert!(!repeat
        .iter()
        .any(|e| matches!(e, PixelBusEvent::WeaponBar(_))));

    // A real change (bar swap to back) emits again.
    let changed = r.observe(
        BlockSamples {
            weapon: Some(weapon(1, 2, 2)),
            ..alive()
        },
        200,
    );
    assert!(changed
        .iter()
        .any(|e| matches!(e, PixelBusEvent::WeaponBar(_))));
}

// Slice 028: block-size single source of truth.

#[test]
fn block_center_and_capture_dims_match_contract_table() {
    // (block_px, [B0..B8 centers], (capture_w, capture_h)) per
    // specs/028-pixelbus-block-size/contracts/geometry.md, extended by the B4, B5,
    // and B6-to-B8 contracts in slices 031, 032, and 033.
    let cases = [
        (
            2u32,
            [
                (1u32, 1u32),
                (3, 1),
                (5, 1),
                (7, 1),
                (9, 1),
                (11, 1),
                (13, 1),
                (15, 1),
                (17, 1),
            ],
            (18u32, 2u32),
        ),
        (
            4,
            [
                (2, 2),
                (6, 2),
                (10, 2),
                (14, 2),
                (18, 2),
                (22, 2),
                (26, 2),
                (30, 2),
                (34, 2),
            ],
            (36, 4),
        ),
        (
            8,
            [
                (4, 4),
                (12, 4),
                (20, 4),
                (28, 4),
                (36, 4),
                (44, 4),
                (52, 4),
                (60, 4),
                (68, 4),
            ],
            (72, 8),
        ),
        (
            16,
            [
                (8, 8),
                (24, 8),
                (40, 8),
                (56, 8),
                (72, 8),
                (88, 8),
                (104, 8),
                (120, 8),
                (136, 8),
            ],
            (144, 16),
        ),
        (
            32,
            [
                (16, 16),
                (48, 16),
                (80, 16),
                (112, 16),
                (144, 16),
                (176, 16),
                (208, 16),
                (240, 16),
                (272, 16),
            ],
            (288, 32),
        ),
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
    assert_eq!(NUM_BLOCKS, 9);
    assert_eq!(DEFAULT_BLOCK_PX, 16);
}

// Slice 031: the combat block (B4).

#[test]
fn decode_combat_decodes_both_states() {
    let t = ReaderConfig::default().tolerance;
    assert_eq!(decode_combat(COMBAT_IN, t), CombatSignal::InCombat);
    assert_eq!(decode_combat(COMBAT_OUT, t), CombatSignal::OutOfCombat);
}

#[test]
fn decode_combat_survives_capture_drift_within_tolerance() {
    // Every channel off by the full default tolerance in both directions still
    // decodes, because the checksum is validated within tolerance too.
    let t = ReaderConfig::default().tolerance;
    assert_eq!(
        decode_combat(Rgb::new(0xE0 + 2, 0x2D + 2, 0x1F - 2), t),
        CombatSignal::InCombat
    );
    assert_eq!(
        decode_combat(Rgb::new(0x20 - 2, 0x2D - 2, 0xDF + 2), t),
        CombatSignal::OutOfCombat
    );
}

#[test]
fn decode_combat_rejects_a_wrong_marker() {
    let t = ReaderConfig::default().tolerance;
    // Right payload and checksum, but the weapon block's marker.
    assert_eq!(
        decode_combat(Rgb::new(0xE0, 0x5A, 0x1F), t),
        CombatSignal::Unknown
    );
}

#[test]
fn decode_combat_rejects_a_failed_checksum() {
    let t = ReaderConfig::default().tolerance;
    // Right marker and a valid state code, but blue is not the complement.
    assert_eq!(
        decode_combat(Rgb::new(0xE0, 0x2D, 0x00), t),
        CombatSignal::Unknown
    );
}

#[test]
fn decode_combat_rejects_an_unrecognized_state_code() {
    let t = ReaderConfig::default().tolerance;
    // Right marker, valid checksum, red is neither state code.
    assert_eq!(decode_combat(combat(0x80), t), CombatSignal::Unknown);
}

#[test]
fn no_arbitrary_color_decodes_as_a_combat_state() {
    // US2: an addon that draws no combat block leaves whatever the game is
    // rendering at that point. Nothing there may be read as a combat state.
    let t = ReaderConfig::default().tolerance;
    let mut checked = 0u32;
    for r in (0u32..=255).step_by(5) {
        for g in (0u32..=255).step_by(5) {
            for b in (0u32..=255).step_by(5) {
                let sample = Rgb::new(r as u8, g as u8, b as u8);
                // Skip the genuine encodings and their tolerance neighbourhood.
                let is_real = sample.g.abs_diff(0x2D) <= t
                    && (i32::from(sample.r) + i32::from(sample.b) - 255).unsigned_abs()
                        <= u32::from(t);
                if is_real {
                    continue;
                }
                checked += 1;
                assert_eq!(
                    decode_combat(sample, t),
                    CombatSignal::Unknown,
                    "color {sample:?} decoded as a combat state"
                );
            }
        }
    }
    assert!(checked > 100_000, "sweep covered only {checked} colors");

    // The four colors the other blocks actually render must never decode either.
    for other in [
        MAGENTA,
        WAITING,
        BITE,
        Rgb::new(100, 0xA5, 155),
        weapon(1, 2, 1),
    ] {
        assert_eq!(
            decode_combat(other, t),
            CombatSignal::Unknown,
            "another block's color {other:?} decoded as a combat state"
        );
    }
}

#[test]
fn combat_event_only_on_change() {
    let mut r = reader();

    let first = r.observe(
        BlockSamples {
            combat: Some(COMBAT_OUT),
            ..alive()
        },
        0,
    );
    assert!(first.contains(&PixelBusEvent::Combat(CombatSignal::OutOfCombat)));

    // A steady state must not churn.
    let repeat = r.observe(
        BlockSamples {
            combat: Some(COMBAT_OUT),
            ..alive()
        },
        100,
    );
    assert!(!repeat.iter().any(|e| matches!(e, PixelBusEvent::Combat(_))));

    let changed = r.observe(
        BlockSamples {
            combat: Some(COMBAT_IN),
            ..alive()
        },
        200,
    );
    assert!(changed.contains(&PixelBusEvent::Combat(CombatSignal::InCombat)));
}

#[test]
fn combat_clears_when_the_block_stops_decoding() {
    // The deliberate divergence from the weapon block: with the beacon still
    // alive, a combat block that stops decoding clears to Unknown rather than
    // holding, so a stale "in combat" cannot survive an addon downgrade.
    let mut r = reader();
    r.observe(
        BlockSamples {
            combat: Some(COMBAT_IN),
            weapon: Some(weapon(1, 2, 1)),
            ..alive()
        },
        0,
    );

    let gone = r.observe(alive(), 100);
    assert!(gone.contains(&PixelBusEvent::Combat(CombatSignal::Unknown)));

    // The weapon block, sampled identically, still holds its last value. If this
    // ever fails, the two blocks have silently converged and the divergence
    // recorded in specs/031-combat-state-block/spec.md needs revisiting.
    assert!(!gone
        .iter()
        .any(|e| matches!(e, PixelBusEvent::WeaponBar(_))));
}

#[test]
fn combat_clears_on_signal_loss() {
    let mut r = reader();
    r.observe(
        BlockSamples {
            combat: Some(COMBAT_IN),
            ..alive()
        },
        0,
    );

    let lost = r.observe(BlockSamples::default(), 2500);
    assert!(lost.contains(&PixelBusEvent::SignalLost));
    assert!(lost.contains(&PixelBusEvent::Combat(CombatSignal::Unknown)));
}

#[test]
fn combat_not_decoded_without_heartbeat() {
    let mut r = reader();
    let events = r.observe(
        BlockSamples {
            combat: Some(COMBAT_IN),
            ..Default::default()
        },
        0,
    );
    assert!(events.is_empty());
}

#[test]
fn block_center_greens_are_pairwise_separated() {
    // FR-006, enforced rather than remembered: a later slice adding a colliding
    // marker fails here and is told which pair collides.
    let t = ReaderConfig::default().tolerance;
    for (i, (name_a, a)) in BLOCK_CENTER_GREENS.iter().enumerate() {
        for (name_b, b) in BLOCK_CENTER_GREENS.iter().skip(i + 1) {
            assert!(
                a.abs_diff(*b) > t,
                "{name_a} ({a:#04X}) and {name_b} ({b:#04X}) are within the default tolerance {t}"
            );
        }
    }
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
    let (cx, cy) = cfg.combat_point();
    let mut sampler = MockSampler::new();
    sampler.set(sx, sy, MAGENTA);
    // At block_px = 8 the combat center is (36, 4). Seeding it proves the runtime
    // path samples the fifth derived point too, not just the original four.
    assert_eq!((cx, cy), (36, 4));
    sampler.set(cx, cy, COMBAT_IN);
    let mut r = PixelBusReader::new(cfg);
    let events = r.sample_and_observe(&sampler, 0);
    assert!(events.contains(&PixelBusEvent::Heartbeat));
    assert!(events.contains(&PixelBusEvent::Combat(CombatSignal::InCombat)));
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

// Slice 032: the menu block (B5) and its gate.

/// The menu block color for a surface code, mirroring the addon's encoder.
fn menu(code: u8) -> Rgb {
    let red = code * 24;
    Rgb::new(red, 0xD2, 255 - red)
}

#[test]
fn decode_menu_reads_every_surface_code() {
    let t = ReaderConfig::default().tolerance;
    let expected = [
        MenuSurface::None,
        MenuSurface::SystemMenu,
        MenuSurface::Map,
        MenuSurface::Inventory,
        MenuSurface::Mail,
        MenuSurface::Character,
        MenuSurface::GuildStore,
        MenuSurface::CrownStore,
        MenuSurface::Journal,
        MenuSurface::ChatEntry,
        MenuSurface::Other,
    ];
    for (code, want) in expected.iter().enumerate() {
        assert_eq!(decode_menu(menu(code as u8), t), *want, "code {code}");
    }
}

#[test]
fn every_surface_except_gameplay_gates() {
    assert!(!MenuSurface::None.gates());
    for code in 1..=10u8 {
        let surface = decode_menu(menu(code), ReaderConfig::default().tolerance);
        assert!(surface.gates(), "code {code} must gate");
    }
    // The unenumerated surface gates too. This is what keeps the gate correct for
    // a scene the addon could not name.
    assert!(MenuSurface::Other.gates());
}

#[test]
fn decode_menu_rejects_invalid_samples() {
    let t = ReaderConfig::default().tolerance;
    // Wrong marker (the combat block's).
    assert_eq!(decode_menu(Rgb::new(48, 0x2D, 207), t), MenuSurface::None);
    // Valid marker, broken checksum.
    assert_eq!(decode_menu(Rgb::new(48, 0xD2, 0), t), MenuSurface::None);
    // Valid marker and checksum, red sitting between two codes.
    assert_eq!(decode_menu(Rgb::new(36, 0xD2, 219), t), MenuSurface::None);
    // Valid marker and checksum, red beyond the highest code.
    assert_eq!(decode_menu(Rgb::new(252, 0xD2, 3), t), MenuSurface::None);
}

#[test]
fn no_arbitrary_color_decodes_as_a_gating_surface() {
    // The failure this guards: an addon too old to draw B5 leaves whatever the
    // game renders there, and a false gate would silently stop interception.
    let t = ReaderConfig::default().tolerance;
    for r in (0u32..=255).step_by(3) {
        for g in (0u32..=255).step_by(3) {
            for b in (0u32..=255).step_by(3) {
                let sample = Rgb::new(r as u8, g as u8, b as u8);
                let is_real = sample.g.abs_diff(0xD2) <= t
                    && (i32::from(sample.r) + i32::from(sample.b) - 255).unsigned_abs()
                        <= u32::from(t);
                if is_real {
                    continue;
                }
                assert_eq!(
                    decode_menu(sample, t),
                    MenuSurface::None,
                    "color {sample:?} decoded as a surface"
                );
            }
        }
    }
    // The other blocks' rendered colors must never gate either.
    for other in [
        MAGENTA,
        WAITING,
        BITE,
        Rgb::new(100, 0xA5, 155),
        weapon(1, 2, 1),
        COMBAT_IN,
    ] {
        assert!(
            !decode_menu(other, t).gates(),
            "another block's color {other:?} gated input"
        );
    }
}

#[test]
fn menu_event_only_on_change_and_clears_on_loss() {
    let mut r = reader();

    let first = r.observe(
        BlockSamples {
            menu: Some(menu(2)),
            ..alive()
        },
        0,
    );
    assert!(first.contains(&PixelBusEvent::MenuGate(MenuSurface::Map)));

    let repeat = r.observe(
        BlockSamples {
            menu: Some(menu(2)),
            ..alive()
        },
        100,
    );
    assert!(!repeat
        .iter()
        .any(|e| matches!(e, PixelBusEvent::MenuGate(_))));

    // A block that stops decoding opens the gate, never closes it.
    let gone = r.observe(alive(), 200);
    assert!(gone.contains(&PixelBusEvent::MenuGate(MenuSurface::None)));

    // And so does losing the signal entirely.
    r.observe(
        BlockSamples {
            menu: Some(menu(1)),
            ..alive()
        },
        300,
    );
    let lost = r.observe(BlockSamples::default(), 3000);
    assert!(lost.contains(&PixelBusEvent::SignalLost));
    assert!(lost.contains(&PixelBusEvent::MenuGate(MenuSurface::None)));
}

// Slice 033: the resource blocks (B6 to B8).
//
// These are the first NUMERIC signal on the strip. Correctness here is not "each
// value is unmistakable for every other" but bounded error plus monotonicity,
// which is exactly why the payload is the percentage itself rather than an index
// into a colour table.

const HEALTH_MARKER: u8 = 0x16;
const STAMINA_MARKER: u8 = 0x6D;
const MAGICKA_MARKER: u8 = 0xBB;

/// A resource block colour, mirroring the addon encoder.
fn resource(marker: u8, payload: u8) -> Rgb {
    Rgb::new(payload, marker, 255u8.wrapping_sub(payload))
}

#[test]
fn every_publishable_percentage_decodes_to_itself() {
    let t = ReaderConfig::default().tolerance;
    for percent in 0..=100u8 {
        assert_eq!(
            decode_resource(resource(HEALTH_MARKER, percent), HEALTH_MARKER, t),
            ResourceLevel::Percent(percent),
            "health {percent}"
        );
    }
}

#[test]
fn resource_error_is_bounded_and_never_a_different_percentage() {
    // THE property that justifies this encoding over the colour table issue #2
    // specified. Exhaustive over the full publishable range crossed with every
    // in-tolerance perturbation of all three channels: decoding yields either a
    // value within tolerance, or unavailable. Never a different percentage.
    //
    // Under a lookup table this test could not be written, because a one-step
    // channel error there lands on whichever entry is nearest in colour space and
    // bears no relation to nearness in percentage.
    let t = ReaderConfig::default().tolerance;
    let drift = i32::from(t);
    let mut checked = 0u32;

    for percent in 0..=100u8 {
        let base = resource(HEALTH_MARKER, percent);
        for dr in -drift..=drift {
            for db in -drift..=drift {
                for dg in -drift..=drift {
                    let r = i32::from(base.r) + dr;
                    let g = i32::from(base.g) + dg;
                    let b = i32::from(base.b) + db;
                    if !(0..=255).contains(&r) || !(0..=255).contains(&g) || !(0..=255).contains(&b)
                    {
                        continue;
                    }
                    let sample = Rgb::new(r as u8, g as u8, b as u8);
                    checked += 1;
                    match decode_resource(sample, HEALTH_MARKER, t) {
                        ResourceLevel::Unknown => {}
                        ResourceLevel::Percent(got) => assert!(
                            got.abs_diff(percent) <= t,
                            "published {percent} decoded {got} from {sample:?}, outside tolerance"
                        ),
                    }
                }
            }
        }
    }
    assert!(checked > 2_000, "sweep covered only {checked} samples");
}

#[test]
fn resource_decoding_is_monotonic() {
    let t = ReaderConfig::default().tolerance;
    let mut previous = 0u8;
    for percent in 0..=100u8 {
        let ResourceLevel::Percent(got) =
            decode_resource(resource(HEALTH_MARKER, percent), HEALTH_MARKER, t)
        else {
            panic!("percent {percent} must decode");
        };
        assert!(got >= previous, "decoding went backwards at {percent}");
        previous = got;
    }
    assert_eq!(previous, 100);
}

#[test]
fn a_full_resource_survives_upward_drift() {
    // The analyze gate caught this: rejecting any payload above 100 would make a
    // full pool, the ordinary out-of-combat state, the least stable reading on the
    // strip. It clamps instead.
    let t = ReaderConfig::default().tolerance;
    for over in 0..=t {
        let sample = Rgb::new(100 + over, HEALTH_MARKER, 255 - 100);
        assert_eq!(
            decode_resource(sample, HEALTH_MARKER, t),
            ResourceLevel::Percent(100),
            "a full pool drifted up by {over} must still read 100"
        );
    }
}

#[test]
fn resource_rejects_invalid_samples() {
    let t = ReaderConfig::default().tolerance;
    // Another block marker.
    assert_eq!(
        decode_resource(resource(STAMINA_MARKER, 50), HEALTH_MARKER, t),
        ResourceLevel::Unknown
    );
    // Broken checksum.
    assert_eq!(
        decode_resource(Rgb::new(50, HEALTH_MARKER, 0), HEALTH_MARKER, t),
        ResourceLevel::Unknown
    );
    // The explicit unavailable payload, and any other out-of-range value.
    assert_eq!(
        decode_resource(resource(HEALTH_MARKER, 0xFF), HEALTH_MARKER, t),
        ResourceLevel::Unknown
    );
    assert_eq!(
        decode_resource(resource(HEALTH_MARKER, 200), HEALTH_MARKER, t),
        ResourceLevel::Unknown
    );
}

#[test]
fn no_arbitrary_colour_decodes_as_a_resource() {
    let t = ReaderConfig::default().tolerance;
    for marker in [HEALTH_MARKER, STAMINA_MARKER, MAGICKA_MARKER] {
        for r in (0u32..=255).step_by(3) {
            for g in (0u32..=255).step_by(3) {
                for b in (0u32..=255).step_by(3) {
                    let sample = Rgb::new(r as u8, g as u8, b as u8);
                    let plausible = sample.g.abs_diff(marker) <= t
                        && (i32::from(sample.r) + i32::from(sample.b) - 255).unsigned_abs()
                            <= u32::from(t)
                        && sample.r <= 100 + t;
                    if plausible {
                        continue;
                    }
                    assert_eq!(
                        decode_resource(sample, marker, t),
                        ResourceLevel::Unknown,
                        "colour {sample:?} decoded as a resource"
                    );
                }
            }
        }
    }
    // No other block rendered colour decodes as a resource either.
    for other in [
        MAGENTA,
        WAITING,
        BITE,
        Rgb::new(100, 0xA5, 155),
        weapon(1, 2, 1),
        COMBAT_IN,
    ] {
        for marker in [HEALTH_MARKER, STAMINA_MARKER, MAGICKA_MARKER] {
            assert_eq!(decode_resource(other, marker, t), ResourceLevel::Unknown);
        }
    }
}

#[test]
fn the_three_resources_decode_independently() {
    let t = ReaderConfig::default().tolerance;
    let set = decode_resources(
        Some(resource(HEALTH_MARKER, 73)),
        None,                                  // stamina block absent
        Some(Rgb::new(40, MAGICKA_MARKER, 0)), // magicka checksum broken
        t,
    );
    assert_eq!(set.health, ResourceLevel::Percent(73));
    assert_eq!(set.stamina, ResourceLevel::Unknown);
    assert_eq!(set.magicka, ResourceLevel::Unknown);
}

#[test]
fn resource_event_only_on_change_and_clears_on_loss() {
    let mut r = reader();

    let first = r.observe(
        BlockSamples {
            health: Some(resource(HEALTH_MARKER, 80)),
            stamina: Some(resource(STAMINA_MARKER, 60)),
            magicka: Some(resource(MAGICKA_MARKER, 40)),
            ..alive()
        },
        0,
    );
    assert!(first
        .iter()
        .any(|e| matches!(e, PixelBusEvent::Resources(_))));

    let repeat = r.observe(
        BlockSamples {
            health: Some(resource(HEALTH_MARKER, 80)),
            stamina: Some(resource(STAMINA_MARKER, 60)),
            magicka: Some(resource(MAGICKA_MARKER, 40)),
            ..alive()
        },
        100,
    );
    assert!(
        !repeat
            .iter()
            .any(|e| matches!(e, PixelBusEvent::Resources(_))),
        "a steady set must not churn"
    );

    // Blocks stop decoding: clears rather than holding.
    let gone = r.observe(alive(), 200);
    assert!(gone.contains(&PixelBusEvent::Resources(ResourceSet::new_unknown())));

    // And signal loss clears too.
    r.observe(
        BlockSamples {
            health: Some(resource(HEALTH_MARKER, 55)),
            ..alive()
        },
        300,
    );
    let lost = r.observe(BlockSamples::default(), 3000);
    assert!(lost.contains(&PixelBusEvent::SignalLost));
    assert!(lost.contains(&PixelBusEvent::Resources(ResourceSet::new_unknown())));
}

// Slice 035: the grid wrap. The beacon stopped being a strip and became a grid,
// and these tests carry two separate obligations. The first set is about the
// wrap working at block counts nobody has reached yet. The second, and the more
// important one today, is that at the current block count the wrap changes
// absolutely nothing: same positions, same captured region, same everything.

/// The narrowest client width the game is assumed to support, from the feature
/// specification's Assumptions section. Named rather than written inline so the
/// bound below is traceable to the thing that justifies it.
const NARROWEST_CLIENT_WIDTH: u32 = 1024;

#[test]
fn grid_position_wraps_column_then_row() {
    // Column first, then row, which is the whole contract in one line.
    assert_eq!(grid_position(0, 4), (0, 0));
    assert_eq!(grid_position(3, 4), (3, 0));
    assert_eq!(grid_position(4, 4), (0, 1));
    assert_eq!(grid_position(9, 4), (1, 2));
    // The shipped count, where every current block is still in row 0.
    for index in 0..NUM_BLOCKS {
        assert_eq!(grid_position(index, COLUMNS), (index, 0));
    }
    // A single column degenerates to one block per row.
    for index in 0..5 {
        assert_eq!(grid_position(index, 1), (0, index));
    }
}

#[test]
fn grid_rows_is_the_ceiling_with_no_phantom_row() {
    assert_eq!(grid_rows(0, 16), 0);
    assert_eq!(grid_rows(1, 16), 1);
    assert_eq!(grid_rows(16, 16), 1, "an exact multiple must not add a row");
    assert_eq!(grid_rows(17, 16), 2);
    assert_eq!(grid_rows(32, 16), 2);
    assert_eq!(grid_rows(33, 16), 3);
    assert_eq!(grid_rows(9, COLUMNS), 1);
}

#[test]
fn grid_extent_width_is_the_lesser_of_the_count_and_the_columns() {
    // A grid using a fraction of one row must not claim a full row's width.
    assert_eq!(grid_extent(16, 9, 16), Size::new(144, 16));
    // A full row.
    assert_eq!(grid_extent(16, 16, 16), Size::new(256, 16));
    // Past a row: full width, and one row taller for the partial row.
    assert_eq!(grid_extent(16, 17, 16), Size::new(256, 32));
    assert_eq!(grid_extent(16, 32, 16), Size::new(256, 32));
    // Block size scales both axes.
    assert_eq!(grid_extent(2, 17, 16), Size::new(32, 4));
    assert_eq!(grid_extent(32, 17, 16), Size::new(512, 64));
    // Nothing is nothing.
    assert_eq!(grid_extent(16, 0, 16), Size::new(0, 0));
}

#[test]
fn grid_positions_are_distinct_and_contained() {
    // The two properties that matter, over a spread of counts and column counts
    // rather than only the shipped pair.
    for columns in [1u32, 2, 3, 4, 7, 16, 32] {
        for count in [1u32, 5, 9, 16, 17, 31, 32, 33, 100] {
            let extent = grid_extent(1, count, columns);
            let mut seen = std::collections::HashSet::new();
            for index in 0..count {
                let (col, row) = grid_position(index, columns);
                assert!(
                    seen.insert((col, row)),
                    "columns {columns} count {count}: index {index} collides at ({col}, {row})"
                );
                assert!(
                    col < extent.width && row < extent.height,
                    "columns {columns} count {count}: index {index} at ({col}, {row}) \
                     falls outside the extent {extent:?}"
                );
            }
        }
    }
}

#[test]
fn a_partial_final_row_leaves_unreachable_cells_and_that_is_correct() {
    // The converse of containment does NOT hold, deliberately. At 17 blocks in
    // 16 columns the extent holds 32 cells and only 17 are reachable; the other
    // 15 sit beside the last block, undrawn and unread. Requiring surjectivity
    // would forbid every block count that is not a multiple of the column count,
    // so this asserts the gap exists rather than leaving a later reader to
    // "fix" the arithmetic into producing it.
    let count = 17;
    let columns = 16;
    let extent = grid_extent(1, count, columns);
    let cells = extent.width * extent.height;
    assert_eq!(cells, 32);
    assert!(
        cells > count,
        "a partial final row must leave cells no index maps to"
    );
}

#[test]
fn the_extent_grows_downward_and_never_sideways() {
    // The property the whole feature exists for: width is bounded forever, and
    // height is the only dimension that grows with the block count.
    let block_px = 16;
    let row_width = block_px * COLUMNS;
    let mut previous_height = 0;
    for count in [1u32, 16, 17, 100, 256, 1000, 5000] {
        let extent = grid_extent(block_px, count, COLUMNS);
        assert!(
            extent.width <= row_width,
            "count {count}: width {} exceeded one row",
            extent.width
        );
        assert!(
            extent.height >= previous_height,
            "count {count}: height went backwards"
        );
        previous_height = extent.height;
    }
    // And it really does grow: a thousand blocks is far taller than one row.
    assert!(grid_extent(block_px, 1000, COLUMNS).height > block_px * 60);
}

#[test]
fn the_column_count_satisfies_both_bounds_that_governed_its_choice() {
    // Pinned so a future change to the block count, the maximum block size, or
    // the column count fails here rather than silently violating the reasoning
    // recorded in the feature research.
    // Compile-time rather than runtime, because every operand is a constant and
    // a violated bound should fail the build rather than wait for the suite. The
    // messages are what a future reader gets when they raise the block count past
    // the column count or widen the largest supported block.
    const {
        assert!(
            COLUMNS >= NUM_BLOCKS,
            "the column count must be at least the block count, or the wrap would \
             move an existing block and forfeit the no-change property"
        );
    }
    const {
        assert!(
            COLUMNS * MAX_BLOCK_PX <= NARROWEST_CLIENT_WIDTH,
            "one row at the largest block size must fit the narrowest supported client"
        );
    }
}

// The no-change obligation. These assertions spell out the pre-wrap arithmetic
// rather than referencing the current implementation, so the test and the code
// cannot drift together into agreeing on something wrong.

#[test]
fn every_current_block_sits_exactly_where_the_strip_put_it() {
    for block_px in [MIN_BLOCK_PX, 4, 8, DEFAULT_BLOCK_PX, 30, MAX_BLOCK_PX] {
        for index in 0..NUM_BLOCKS {
            let strip = (block_px * index + block_px / 2, block_px / 2);
            assert_eq!(
                block_center(block_px, index),
                strip,
                "block_px {block_px} index {index} moved"
            );
        }
    }
}

#[test]
fn the_captured_region_is_exactly_what_the_strip_captured() {
    for block_px in [MIN_BLOCK_PX, 4, 8, DEFAULT_BLOCK_PX, 30, MAX_BLOCK_PX] {
        let strip = (block_px * NUM_BLOCKS, block_px);
        assert_eq!(
            capture_dims(block_px),
            strip,
            "capture region changed at block_px {block_px}"
        );
    }
}

#[test]
fn the_heartbeat_block_is_the_grid_origin_at_any_column_count() {
    // Signal-loss detection anchors on B0, so its position must not depend on
    // the layout.
    for columns in [1u32, 2, 9, 16, 32, 64] {
        assert_eq!(grid_position(0, columns), (0, 0));
    }
    for block_px in [MIN_BLOCK_PX, DEFAULT_BLOCK_PX, MAX_BLOCK_PX] {
        assert_eq!(block_center(block_px, 0), (block_px / 2, block_px / 2));
    }
}

#[test]
fn sampled_centres_stay_whole_pixels_on_both_axes() {
    // Block sizes are even by sanitization, so the half-block offset is exact.
    // The new axis has to inherit that, not only the old one.
    for block_px in [MIN_BLOCK_PX, 4, 8, DEFAULT_BLOCK_PX, 30, MAX_BLOCK_PX] {
        assert_eq!(block_px % 2, 0, "supported block sizes are even");
        for index in 0..40u32 {
            let (x, y) = block_center(block_px, index);
            let (col, row) = grid_position(index, COLUMNS);
            assert_eq!(x, block_px * col + block_px / 2);
            assert_eq!(y, block_px * row + block_px / 2);
        }
    }
}

#[test]
fn the_shipped_column_count_is_pinned() {
    // Changing this is a breaking change to the contract shared with the addon,
    // so it fails here first and names itself.
    assert_eq!(COLUMNS, 16);
}

#[test]
fn block_center_wraps_past_the_first_row() {
    // The public entry point, at indices no block has reached yet. Row 1 starts
    // back at x = block_px / 2 and drops one block height.
    let px = 16;
    assert_eq!(block_center(px, 15), (15 * px + px / 2, px / 2));
    assert_eq!(block_center(px, 16), (px / 2, px + px / 2));
    assert_eq!(block_center(px, 17), (px + px / 2, px + px / 2));
    assert_eq!(block_center(px, 32), (px / 2, 2 * px + px / 2));
    assert_eq!(block_center(px, 47), (15 * px + px / 2, 2 * px + px / 2));
    // And the x coordinate never runs past one row, whatever the index.
    for index in 0..200u32 {
        let (x, _) = block_center(px, index);
        assert!(x < px * COLUMNS, "index {index} escaped the row width");
    }
}
