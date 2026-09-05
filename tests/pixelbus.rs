//! Decoder and state-machine tests for the Pixel Bus Reader.

use eso_weave::config::NoticeKind;
use eso_weave::pixelbus::{
    block_center, capture_dims, decode_combat, decode_cooldown, decode_latency,
    decode_layout_header, decode_life_state, decode_menu, decode_movement, decode_quickslot,
    decode_resource, decode_resources, decode_roll_dodge, decode_travel_state, decode_weapon_bar,
    decode_world_state, fishing_signal, grid_extent, grid_position, grid_rows,
    layout_header_colors, load_reader_config, poll_interval, sanitize_block_px, status_present,
    store_reader_config, strip_pixel, ActiveBar, BlockSamples, BusLayout, CombatSignal,
    CooldownSet, FishingSignal, LayoutFailure, LayoutHeaderSamples, LayoutMode, LayoutState,
    LifeState, MenuSurface, MockSampler, MovementSignal, PixelBusEvent, PixelBusReader,
    QuickslotClassification, QuickslotNonPotionKind, QuickslotPotionAvailability, QuickslotState,
    QuickslotUnavailableReason, ReaderConfig, ResourceLevel, ResourceSet, Rgb, RollDodgeState,
    Size, SlotCooldown, TravelState, WeaponBarSignal, WeaponClass, WorldState, BLOCK_CENTER_GREENS,
    COLUMNS, DEFAULT_BLOCK_PX, LAYOUT_HEADER_BLOCKS, LAYOUT_PROTOCOL_VERSION, LAYOUT_VERSION_CODE,
    LAYOUT_VERSION_ONE_BLOCKS, LAYOUT_VERSION_ONE_CODE, LAYOUT_VERSION_THREE_BLOCKS,
    LAYOUT_VERSION_THREE_CODE, LAYOUT_VERSION_TWO_BLOCKS, LAYOUT_VERSION_TWO_CODE, MAX_BLOCK_PX,
    MAX_LAYOUT_TOLERANCE, MIN_BLOCK_PX, NUM_BLOCKS,
};

#[test]
fn negotiated_header_round_trips_boundaries_and_checksums() {
    for columns in [3u32, 24, 32, 120, 255, 256, 3840, 65_535] {
        let colors = layout_header_colors(columns).expect("valid column count");
        assert_eq!(colors[0], Rgb::new(0x45, 0x53, LAYOUT_VERSION_CODE));
        let state = decode_layout_header(
            LayoutHeaderSamples::from(colors),
            ReaderConfig::default().tolerance,
            DEFAULT_BLOCK_PX,
            None,
        );
        assert_eq!(
            state,
            LayoutState::Ready(BusLayout::negotiated(columns).unwrap())
        );
    }
}

#[test]
fn negotiated_version_one_geometry_remains_readable_without_b22_support() {
    let columns = 120;
    let mut colors = layout_header_colors(columns).unwrap();
    colors[0].b = LAYOUT_VERSION_ONE_CODE;
    let expected = BusLayout {
        mode: LayoutMode::Negotiated { version: 1 },
        columns,
        payload_offset: LAYOUT_HEADER_BLOCKS,
    };

    assert_eq!(
        decode_layout_header(
            LayoutHeaderSamples::from(colors),
            ReaderConfig::default().tolerance,
            DEFAULT_BLOCK_PX,
            None,
        ),
        LayoutState::Ready(expected)
    );
    assert_eq!(expected.payload_blocks(), LAYOUT_VERSION_ONE_BLOCKS);
    assert_eq!(
        expected.total_cells(),
        LAYOUT_HEADER_BLOCKS + LAYOUT_VERSION_ONE_BLOCKS
    );
    assert!(!expected.supports_world_state());
    assert_eq!(
        BusLayout::negotiated(columns).unwrap().mode,
        LayoutMode::Negotiated {
            version: LAYOUT_PROTOCOL_VERSION,
        }
    );
    assert!(BusLayout::negotiated(columns)
        .unwrap()
        .supports_world_state());
}

#[test]
fn negotiated_version_two_geometry_remains_readable_without_b23_support() {
    let columns = 120;
    let mut colors = layout_header_colors(columns).unwrap();
    colors[0].b = LAYOUT_VERSION_TWO_CODE;
    let state = decode_layout_header(
        LayoutHeaderSamples::from(colors),
        ReaderConfig::default().tolerance,
        DEFAULT_BLOCK_PX,
        None,
    );
    let LayoutState::Ready(layout) = state else {
        panic!("version 2 layout should remain readable");
    };
    assert_eq!(layout.payload_blocks(), LAYOUT_VERSION_TWO_BLOCKS);
    assert!(layout.supports_world_state());
    assert!(!layout.supports_roll_dodge());
    assert!(BusLayout::negotiated(columns)
        .unwrap()
        .supports_roll_dodge());
}

#[test]
fn recognized_header_corruption_never_falls_back_to_legacy() {
    let tolerance = ReaderConfig::default().tolerance;
    let valid = layout_header_colors(120).unwrap();
    for (index, damaged) in [
        Rgb::new(valid[1].r, 0x00, valid[1].b),
        Rgb::new(valid[1].r, valid[1].g, 0x00),
        Rgb::new(valid[2].r, 0x00, valid[2].b),
        Rgb::new(valid[2].r, valid[2].g, 0x00),
    ]
    .into_iter()
    .enumerate()
    {
        let mut colors = valid;
        colors[1 + index / 2] = damaged;
        assert!(matches!(
            decode_layout_header(
                LayoutHeaderSamples::from(colors),
                tolerance,
                DEFAULT_BLOCK_PX,
                None
            ),
            LayoutState::Unavailable(_)
        ));
    }

    let mut unsupported = valid;
    unsupported[0].b = LAYOUT_VERSION_CODE + 0x20;
    assert_eq!(
        decode_layout_header(
            LayoutHeaderSamples::from(unsupported),
            tolerance,
            DEFAULT_BLOCK_PX,
            None
        ),
        LayoutState::Unavailable(LayoutFailure::UnsupportedVersion {
            observed: LAYOUT_VERSION_CODE + 0x20,
        })
    );

    let mut drifted = valid;
    drifted[0].b = LAYOUT_VERSION_CODE + tolerance;
    assert_eq!(
        decode_layout_header(
            LayoutHeaderSamples::from(drifted),
            tolerance,
            DEFAULT_BLOCK_PX,
            None
        ),
        LayoutState::Ready(BusLayout::negotiated(120).unwrap())
    );

    let mut future = valid;
    future[0].b = LAYOUT_VERSION_CODE + 0x20;
    assert_eq!(
        decode_layout_header(
            LayoutHeaderSamples::from(future),
            u8::MAX,
            DEFAULT_BLOCK_PX,
            None
        ),
        LayoutState::Unavailable(LayoutFailure::UnsupportedVersion {
            observed: LAYOUT_VERSION_CODE + 0x20,
        })
    );
    const {
        assert!(MAX_LAYOUT_TOLERANCE < 0x10);
    }
}

#[test]
fn legacy_requires_a_real_heartbeat_and_missing_is_unavailable() {
    let tolerance = ReaderConfig::default().tolerance;
    let legacy = decode_layout_header(
        LayoutHeaderSamples {
            h0: Some(Rgb::new(0xFF, 0x00, 0xFF)),
            h1: None,
            h2: None,
        },
        tolerance,
        DEFAULT_BLOCK_PX,
        None,
    );
    assert_eq!(legacy, LayoutState::Ready(BusLayout::legacy()));
    assert_eq!(BusLayout::legacy().mode, LayoutMode::Legacy);

    assert_eq!(
        decode_layout_header(
            LayoutHeaderSamples::default(),
            tolerance,
            DEFAULT_BLOCK_PX,
            None
        ),
        LayoutState::Unavailable(LayoutFailure::Missing)
    );
}

#[test]
fn negotiated_geometry_uses_capacity_and_wraps_at_the_exact_boundary() {
    let block_px = 16;
    let one_row = BusLayout::negotiated(LAYOUT_HEADER_BLOCKS + NUM_BLOCKS).unwrap();
    assert_eq!(one_row.rows(), 1);
    assert_eq!(
        one_row.extent(block_px),
        Size::new((LAYOUT_HEADER_BLOCKS + NUM_BLOCKS) * block_px, block_px)
    );
    for index in 0..NUM_BLOCKS {
        let (x, y) = one_row.payload_point(block_px, index);
        assert_eq!(y, block_px / 2);
        assert!(x < one_row.extent(block_px).width);
    }

    let wrapped = BusLayout::negotiated(LAYOUT_HEADER_BLOCKS + NUM_BLOCKS - 1).unwrap();
    assert_eq!(wrapped.rows(), 2);
    assert_eq!(
        wrapped.payload_point(block_px, NUM_BLOCKS - 1),
        (block_px / 2, block_px + block_px / 2)
    );
}

#[test]
fn negotiated_payload_points_are_unique_and_inside_every_tested_extent() {
    for columns in [3, 4, 8, 20, 23, 24, 120, 65_535] {
        let layout = BusLayout::negotiated(columns).unwrap();
        for block_px in [MIN_BLOCK_PX, DEFAULT_BLOCK_PX, MAX_BLOCK_PX] {
            let extent = layout.extent(block_px);
            let mut points = std::collections::HashSet::new();
            for index in 0..NUM_BLOCKS {
                let point = layout.payload_point(block_px, index);
                assert!(
                    points.insert(point),
                    "duplicate point for column count {columns}"
                );
                assert!(point.0 < extent.width && point.1 < extent.height);
            }
        }
    }
}

#[test]
fn every_supported_size_keeps_current_payload_on_one_row_at_minimum_width() {
    for block_px in [MIN_BLOCK_PX, 4, 8, DEFAULT_BLOCK_PX, 24, MAX_BLOCK_PX] {
        let columns = NARROWEST_CLIENT_WIDTH / block_px;
        let layout = BusLayout::negotiated(columns).unwrap();
        assert!(columns >= LAYOUT_HEADER_BLOCKS + NUM_BLOCKS);
        assert_eq!(layout.rows(), 1);
    }
}

#[test]
fn published_columns_cannot_exceed_the_measured_surface() {
    let colors = layout_header_colors(65).unwrap();
    assert_eq!(
        decode_layout_header(
            LayoutHeaderSamples::from(colors),
            2,
            16,
            Some(Size::new(1024, 768))
        ),
        LayoutState::Unavailable(LayoutFailure::ExceedsSurface {
            columns: 65,
            capacity: 64,
        })
    );
}

#[test]
fn invalid_block_size_and_short_surface_are_rejected() {
    let colors = layout_header_colors(3).unwrap();
    assert_eq!(
        decode_layout_header(LayoutHeaderSamples::from(colors), 2, 0, None),
        LayoutState::Unavailable(LayoutFailure::InvalidBlockSize)
    );
    assert_eq!(
        decode_layout_header(
            LayoutHeaderSamples::from(colors),
            2,
            16,
            Some(Size::new(48, 127))
        ),
        LayoutState::Unavailable(LayoutFailure::ExtentExceedsSurface {
            extent: Size::new(48, 160),
            surface: Size::new(48, 127),
        })
    );
}

#[test]
fn reader_prepares_twice_on_acquisition_then_once_when_steady() {
    let config = ReaderConfig::default();
    let mut reader = PixelBusReader::new(config);
    let mut sampler = MockSampler::new();
    sampler.set_display(Some(eso_weave::pixelbus::MeasuredDisplay {
        surface: Size::new(1920, 1080),
        surface_origin: eso_weave::pixelbus::Point::new(0, 0),
        display_origin: None,
        display_size: None,
        dpi: None,
    }));
    for (index, color) in layout_header_colors(120).unwrap().into_iter().enumerate() {
        let point = BusLayout::negotiated(120)
            .unwrap()
            .cell_point(config.block_px, index as u32);
        sampler.set(point.0, point.1, color);
    }
    let status = BusLayout::negotiated(120)
        .unwrap()
        .payload_point(config.block_px, 0);
    sampler.set(status.0, status.1, Rgb::new(0xFF, 0x00, 0xFF));

    let first = reader.sample_and_observe(&sampler, 0);
    assert!(first.contains(&PixelBusEvent::Layout(LayoutState::Ready(
        BusLayout::negotiated(120).unwrap()
    ))));
    assert_eq!(sampler.prepared_extents().len(), 2);

    sampler.clear_prepared_extents();
    reader.sample_and_observe(&sampler, 100);
    assert_eq!(sampler.prepared_extents().len(), 1);
}

#[test]
fn reader_recaptures_only_when_changed_geometry_escapes_the_prepared_frame() {
    let config = ReaderConfig::default();
    let mut reader = PixelBusReader::new(config);
    let mut sampler = MockSampler::new();
    sampler.set_display(Some(eso_weave::pixelbus::MeasuredDisplay {
        surface: Size::new(1920, 1080),
        surface_origin: eso_weave::pixelbus::Point::new(0, 0),
        display_origin: None,
        display_size: None,
        dpi: None,
    }));
    let header_points = [
        (config.block_px / 2, config.block_px / 2),
        (config.block_px + config.block_px / 2, config.block_px / 2),
        (
            config.block_px * 2 + config.block_px / 2,
            config.block_px / 2,
        ),
    ];
    let set_header = |sampler: &mut MockSampler, columns| {
        for (point, color) in header_points
            .into_iter()
            .zip(layout_header_colors(columns).unwrap())
        {
            sampler.set(point.0, point.1, color);
        }
    };

    set_header(&mut sampler, 120);
    reader.sample_and_observe(&sampler, 0);
    sampler.clear_prepared_extents();

    // Twenty columns makes the occupied grid taller than the cached 120-column
    // frame, so the reader captures the new complete extent once more.
    set_header(&mut sampler, 20);
    let events = reader.sample_and_observe(&sampler, 100);
    assert!(events.contains(&PixelBusEvent::Layout(LayoutState::Ready(
        BusLayout::negotiated(20).unwrap()
    ))));
    assert_eq!(sampler.prepared_extents().len(), 2);

    sampler.clear_prepared_extents();
    reader.sample_and_observe(&sampler, 200);
    assert_eq!(sampler.prepared_extents().len(), 1);
}

#[test]
fn invalid_recognized_header_suppresses_payload_sampling() {
    let config = ReaderConfig::default();
    let mut reader = PixelBusReader::new(config);
    let mut sampler = MockSampler::new();
    let layout = BusLayout::negotiated(120).unwrap();
    let colors = layout_header_colors(120).unwrap();
    for (index, color) in colors.into_iter().enumerate() {
        let point = layout.cell_point(config.block_px, index as u32);
        sampler.set(point.0, point.1, color);
    }
    let status = layout.payload_point(config.block_px, 0);
    sampler.set(status.0, status.1, Rgb::new(0xFF, 0x00, 0xFF));
    assert!(reader
        .sample_and_observe(&sampler, 0)
        .contains(&PixelBusEvent::Heartbeat));

    // Keep valid magic but destroy H1. A still-present payload heartbeat cannot
    // be observed through a corrupt geometry authority.
    let h1 = layout.cell_point(config.block_px, 1);
    sampler.set(h1.0, h1.1, Rgb::new(0x00, 0x00, 0x00));
    let events = reader.sample_and_observe(&sampler, 100);
    assert!(
        events.contains(&PixelBusEvent::Layout(LayoutState::Unavailable(
            LayoutFailure::CorruptHighByte
        )))
    );
    assert!(events.contains(&PixelBusEvent::SignalLost));
    assert!(!events.contains(&PixelBusEvent::Heartbeat));
}

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

#[test]
fn poll_interval_caps_interception_inside_the_roll_watchdog_window() {
    use eso_weave::pixelbus::SAFETY_POLL_MAX_MS;

    let cfg = ReaderConfig {
        interval_fishing_ms: 60_000,
        interval_idle_ms: 60_000,
        ..ReaderConfig::default()
    };

    assert_eq!(
        poll_interval(false, true, &cfg),
        SAFETY_POLL_MAX_MS,
        "every supported interception cadence must sample Active repeatedly before the watchdog clears it"
    );
    assert_eq!(poll_interval(true, false, &cfg), 60_000);
    assert_eq!(poll_interval(false, false, &cfg), 60_000);
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

fn life(red: u8) -> Rgb {
    Rgb::new(red, 0x89, 255 - red)
}

fn world(red: u8) -> Rgb {
    Rgb::new(red, 0xCC, 255 - red)
}

fn roll_dodge(red: u8) -> Rgb {
    Rgb::new(red, 0xF9, 255 - red)
}

fn travel(red: u8) -> Rgb {
    Rgb::new(red, 0x13, 255 - red)
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
    // (block_px, [B0..B19 centers], (capture_w, capture_h)) per
    // specs/028-pixelbus-block-size/contracts/geometry.md, extended by the B4, B5,
    // B6-to-B8, B9, B10-to-B15, and B16-to-B19 contracts in slices 031, 032, 033,
    // 036, 037, and 038.
    //
    // The last four entries of each case are on row 1, so their y is
    // block_px + block_px / 2 rather than block_px / 2, and their x restarts at
    // the left edge. The capture height is two block rows for the first time.
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
                (19, 1),
                (21, 1),
                (23, 1),
                (25, 1),
                (27, 1),
                (29, 1),
                (31, 1),
                // Row 1.
                (1, 3),
                (3, 3),
                (5, 3),
                (7, 3),
                (9, 3),
                (11, 3),
                (13, 3),
            ],
            (32u32, 4u32),
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
                (38, 2),
                (42, 2),
                (46, 2),
                (50, 2),
                (54, 2),
                (58, 2),
                (62, 2),
                // Row 1.
                (2, 6),
                (6, 6),
                (10, 6),
                (14, 6),
                (18, 6),
                (22, 6),
                (26, 6),
            ],
            (64, 8),
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
                (76, 4),
                (84, 4),
                (92, 4),
                (100, 4),
                (108, 4),
                (116, 4),
                (124, 4),
                // Row 1.
                (4, 12),
                (12, 12),
                (20, 12),
                (28, 12),
                (36, 12),
                (44, 12),
                (52, 12),
            ],
            (128, 16),
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
                (152, 8),
                (168, 8),
                (184, 8),
                (200, 8),
                (216, 8),
                (232, 8),
                (248, 8),
                // Row 1.
                (8, 24),
                (24, 24),
                (40, 24),
                (56, 24),
                (72, 24),
                (88, 24),
                (104, 24),
            ],
            (256, 32),
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
                (304, 16),
                (336, 16),
                (368, 16),
                (400, 16),
                (432, 16),
                (464, 16),
                (496, 16),
                // Row 1.
                (16, 48),
                (48, 48),
                (80, 48),
                (112, 48),
                (144, 48),
                (176, 48),
                (208, 48),
            ],
            (512, 64),
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
    assert_eq!(NUM_BLOCKS, 25);
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
fn reader_config_default_legacy_geometry_matches_release() {
    let cfg = ReaderConfig::default();
    assert_eq!(cfg.block_px, 16);
    assert_eq!(cfg.status_point(), (8, 8));
    assert_eq!(cfg.fishing_point(), (24, 8));
    assert_eq!(cfg.latency_point(), (40, 8));
    assert_eq!(cfg.weapon_point(), (56, 8));
}

#[test]
fn reader_config_legacy_points_track_block_px() {
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
fn sample_and_observe_reads_legacy_points_after_heartbeat_detection() {
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
        assert_eq!(decode_menu(menu(code as u8), t), Some(*want), "code {code}");
    }
}

#[test]
fn every_surface_except_gameplay_gates() {
    assert!(!MenuSurface::None.gates());
    for code in 1..=10u8 {
        let surface = decode_menu(menu(code), ReaderConfig::default().tolerance);
        assert!(
            surface.is_some_and(MenuSurface::gates),
            "code {code} must gate"
        );
    }
    // The unenumerated surface gates too. This is what keeps the gate correct for
    // a scene the addon could not name.
    assert!(MenuSurface::Other.gates());
}

#[test]
fn decode_menu_rejects_invalid_samples() {
    let t = ReaderConfig::default().tolerance;
    // Wrong marker (the combat block's).
    assert_eq!(decode_menu(Rgb::new(48, 0x2D, 207), t), None);
    // Valid marker, broken checksum.
    assert_eq!(decode_menu(Rgb::new(48, 0xD2, 0), t), None);
    // Valid marker and checksum, red sitting between two codes.
    assert_eq!(decode_menu(Rgb::new(36, 0xD2, 219), t), None);
    // Valid marker and checksum, red beyond the highest code.
    assert_eq!(decode_menu(Rgb::new(252, 0xD2, 3), t), None);
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
                    None,
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
            !decode_menu(other, t).is_some_and(MenuSurface::gates),
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
    assert!(first.contains(&PixelBusEvent::MenuGate(Some(MenuSurface::Map))));

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
    assert!(gone.contains(&PixelBusEvent::MenuGate(None)));

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
    assert!(lost.contains(&PixelBusEvent::MenuGate(None)));
}

#[test]
fn reset_republishes_an_unchanged_surface_after_game_restart() {
    let mut r = reader();
    let samples = BlockSamples {
        menu: Some(menu(3)),
        ..alive()
    };
    assert!(r
        .observe(samples, 0)
        .contains(&PixelBusEvent::MenuGate(Some(MenuSurface::Inventory))));
    r.reset();
    assert!(r
        .observe(samples, 1)
        .contains(&PixelBusEvent::MenuGate(Some(MenuSurface::Inventory))));
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

/// The block count at the moment the grid wrap shipped (slice 035).
///
/// The bound below was originally written as `COLUMNS >= NUM_BLOCKS`, justified
/// as "or the wrap would move an existing block and forfeit the no-change
/// property". That justification was always about the blocks that existed *when
/// the wrap shipped*, and it was expressed in terms of NUM_BLOCKS only because at
/// the time the two were the same number. They stopped being the same number at
/// slice 038, when the count crossed the column boundary on purpose.
///
/// Naming the wrap-era count keeps the actual invariant (no block that predates
/// the wrap may move) while letting the count grow, which is the entire point of
/// having wrapped.
const BLOCKS_AT_WRAP: u32 = 9;

#[test]
fn grid_position_wraps_column_then_row() {
    // Column first, then row, which is the whole contract in one line.
    assert_eq!(grid_position(0, 4), (0, 0));
    assert_eq!(grid_position(3, 4), (3, 0));
    assert_eq!(grid_position(4, 4), (0, 1));
    assert_eq!(grid_position(9, 4), (1, 2));
    // The shipped count, which since slice 038 spans two rows. Row 0 holds the
    // first COLUMNS blocks at their own index; row 1 holds the rest, restarting
    // the column at zero.
    for index in 0..COLUMNS {
        assert_eq!(grid_position(index, COLUMNS), (index, 0));
    }
    for index in COLUMNS..NUM_BLOCKS {
        assert_eq!(grid_position(index, COLUMNS), (index - COLUMNS, 1));
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
            COLUMNS >= BLOCKS_AT_WRAP,
            "the column count must be at least the block count at the wrap, or the \
             wrap would move a block that predates it and forfeit the no-change \
             property"
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
fn every_legacy_row_zero_block_sits_exactly_where_the_strip_put_it() {
    // The no-change obligation, now stated on the blocks it can apply to. Row 0
    // is every block the strip ever held, and each is still at the strip's
    // arithmetic. The blocks past it never had a strip position to keep, so the
    // second loop asserts the wrapped arithmetic instead of nothing.
    for block_px in [MIN_BLOCK_PX, 4, 8, DEFAULT_BLOCK_PX, 30, MAX_BLOCK_PX] {
        for index in 0..COLUMNS {
            let strip = (block_px * index + block_px / 2, block_px / 2);
            assert_eq!(
                block_center(block_px, index),
                strip,
                "block_px {block_px} index {index} moved"
            );
        }
        for index in COLUMNS..NUM_BLOCKS {
            let wrapped = (
                block_px * (index - COLUMNS) + block_px / 2,
                block_px + block_px / 2,
            );
            assert_eq!(
                block_center(block_px, index),
                wrapped,
                "block_px {block_px} index {index} is not on row 1 where it belongs"
            );
        }
    }
}

#[test]
fn the_legacy_captured_region_is_one_full_row_wide_and_two_rows_tall() {
    // Slice 038 crossed the boundary, so the region is no longer the strip's.
    // Spelled out as arithmetic rather than as a call to capture_dims's own
    // helper, so the test and the code cannot drift together.
    for block_px in [MIN_BLOCK_PX, 4, 8, DEFAULT_BLOCK_PX, 30, MAX_BLOCK_PX] {
        let two_rows = (block_px * COLUMNS, block_px * 2);
        assert_eq!(
            capture_dims(block_px),
            two_rows,
            "capture region wrong at block_px {block_px}"
        );
    }
}

#[test]
fn the_legacy_heartbeat_block_is_the_grid_origin_at_any_column_count() {
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
fn the_legacy_column_count_is_pinned() {
    // Changing this would break compatibility with pre-version-14 addons.
    assert_eq!(COLUMNS, 16);
}

#[test]
fn legacy_block_center_wraps_past_the_first_row() {
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

// Slice 036: the movement block (B9). Mounted only; the sprint axis is reserved
// in the encoding and never emitted, because the game exposes no sprint
// observable (see specs/036-movement-state-block/spec.md).

// The grid's shape, enforced at compile time rather than by a test that has to be
// run to be believed.
//
// This replaces the single-row premise `NUM_BLOCKS <= COLUMNS` that slices 035 to
// 037 carried. That assertion was deliberately left at its limit by slice 037 so
// that the seventeenth block would fail here, at the edit that adds it, and it
// did exactly that: slice 038 took the count to twenty and the build stopped.
//
// It is replaced rather than relaxed. A looser bound would be a guard that no
// longer states anything true about what ships, and deleting it would discard the
// only automatic warning that the grid's shape has changed. What follows is just
// as specific about two rows as its predecessor was about one, and calls
// grid_rows rather than open-coding the arithmetic, so the assertion and the
// function cannot drift into agreeing on something wrong.
//
// The slice that adds the twenty-first block will be told by the third one.
const _: () = assert!(
    grid_rows(NUM_BLOCKS, COLUMNS) == 2,
    "the capture region is exactly two rows tall at the shipped block count"
);
const _: () = assert!(
    NUM_BLOCKS > COLUMNS,
    "the grid has crossed onto a second row, so the first row is full"
);
const _: () = assert!(
    NUM_BLOCKS < COLUMNS * 2,
    "the last row is partially filled; a third row needs these assertions rewritten"
);

/// The movement block color for a code, mirroring the addon encoder: the code in
/// red, the marker in green, and the complement checksum in blue.
fn movement(red: u8) -> Rgb {
    Rgb::new(red, 0x43, 255 - red)
}

const MOVEMENT_ON_FOOT: Rgb = Rgb {
    r: 0x20,
    g: 0x43,
    b: 0xDF,
};
const MOVEMENT_MOUNTED: Rgb = Rgb {
    r: 0x60,
    g: 0x43,
    b: 0x9F,
};

#[test]
fn decode_movement_decodes_both_live_states() {
    let t = ReaderConfig::default().tolerance;
    assert_eq!(decode_movement(MOVEMENT_ON_FOOT, t), MovementSignal::OnFoot);
    assert_eq!(
        decode_movement(MOVEMENT_MOUNTED, t),
        MovementSignal::Mounted
    );
}

#[test]
fn decode_movement_survives_capture_drift_within_tolerance() {
    let t = ReaderConfig::default().tolerance;
    assert_eq!(
        decode_movement(Rgb::new(0x20 + 2, 0x43 + 2, 0xDF - 2), t),
        MovementSignal::OnFoot
    );
    assert_eq!(
        decode_movement(Rgb::new(0x60 - 2, 0x43 - 2, 0x9F + 2), t),
        MovementSignal::Mounted
    );
}

#[test]
fn decode_movement_rejects_every_other_blocks_marker() {
    // The tenth block is the one most likely to be sampled against a grid drawn
    // by an older addon, so every incumbent marker is rejected by name.
    let t = ReaderConfig::default().tolerance;
    for (name, green) in BLOCK_CENTER_GREENS {
        if green.abs_diff(0x43) <= t {
            continue;
        }
        assert_eq!(
            decode_movement(Rgb::new(0x60, green, 0x9F), t),
            MovementSignal::Unknown,
            "{name} decoded as a movement state"
        );
    }
}

#[test]
fn decode_movement_rejects_a_failed_checksum() {
    let t = ReaderConfig::default().tolerance;
    assert_eq!(
        decode_movement(Rgb::new(0x60, 0x43, 0x00), t),
        MovementSignal::Unknown
    );
}

#[test]
fn decode_movement_rejects_the_reserved_sprint_codes() {
    // US3: the sprint axis is reserved, not implemented. Its two codes read as
    // unavailable so no half-built state is reachable by an operator, and so a
    // future addon that emits them against an older companion degrades safely.
    let t = ReaderConfig::default().tolerance;
    assert_eq!(decode_movement(movement(0xA0), t), MovementSignal::Unknown);
    assert_eq!(decode_movement(movement(0xE0), t), MovementSignal::Unknown);
}

#[test]
fn decode_movement_rejects_an_unrecognized_state_code() {
    let t = ReaderConfig::default().tolerance;
    assert_eq!(decode_movement(movement(0x80), t), MovementSignal::Unknown);
    assert_eq!(decode_movement(movement(0x00), t), MovementSignal::Unknown);
}

#[test]
fn no_arbitrary_color_decodes_as_a_movement_state() {
    // US2: an addon older than version 10 draws no tenth block, leaving whatever
    // the game renders at that point. Nothing there may be read as a state.
    let t = ReaderConfig::default().tolerance;
    let mut checked = 0u32;
    for r in (0u32..=255).step_by(5) {
        for g in (0u32..=255).step_by(5) {
            for b in (0u32..=255).step_by(5) {
                let sample = Rgb::new(r as u8, g as u8, b as u8);
                let is_real = sample.g.abs_diff(0x43) <= t
                    && (i32::from(sample.r) + i32::from(sample.b) - 255).unsigned_abs()
                        <= u32::from(t);
                if is_real {
                    continue;
                }
                checked += 1;
                assert_eq!(
                    decode_movement(sample, t),
                    MovementSignal::Unknown,
                    "color {sample:?} decoded as a movement state"
                );
            }
        }
    }
    assert!(checked > 100_000, "sweep covered only {checked} colors");
}

#[test]
fn movement_point_is_the_tenth_block_center() {
    // Derived, never restated: the point follows both the configured block size
    // and the shared column count.
    for block_px in [MIN_BLOCK_PX, DEFAULT_BLOCK_PX, 24, MAX_BLOCK_PX] {
        let config = ReaderConfig {
            block_px,
            ..Default::default()
        };
        assert_eq!(config.movement_point(), block_center(block_px, 9));
    }
    // Ten blocks in a sixteen-column grid: still row 0, column 9.
    assert_eq!(grid_position(9, COLUMNS), (9, 0));
}

#[test]
fn movement_change_emits_exactly_one_event() {
    let mut reader = PixelBusReader::new(ReaderConfig::default());
    let mut samples = alive();
    samples.movement = Some(MOVEMENT_ON_FOOT);
    let events = reader.observe(samples, 0);
    assert!(events.contains(&PixelBusEvent::Movement(MovementSignal::OnFoot)));

    // The same state again announces nothing.
    let events = reader.observe(samples, 100);
    assert!(!events
        .iter()
        .any(|e| matches!(e, PixelBusEvent::Movement(_))));

    // A real transition announces once.
    samples.movement = Some(MOVEMENT_MOUNTED);
    let events = reader.observe(samples, 200);
    let movements: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, PixelBusEvent::Movement(_)))
        .collect();
    assert_eq!(movements.len(), 1);
    assert_eq!(
        movements[0],
        &PixelBusEvent::Movement(MovementSignal::Mounted)
    );
}

#[test]
fn movement_clears_to_unknown_when_the_block_stops_decoding() {
    // Follows the combat block rather than the weapon block: a stale "mounted"
    // surviving an addon downgrade is exactly the false reading US2 forbids.
    let mut reader = PixelBusReader::new(ReaderConfig::default());
    let mut samples = alive();
    samples.movement = Some(MOVEMENT_MOUNTED);
    reader.observe(samples, 0);

    samples.movement = Some(Rgb::new(0x11, 0x22, 0x33));
    let events = reader.observe(samples, 100);
    assert!(events.contains(&PixelBusEvent::Movement(MovementSignal::Unknown)));

    // And it does not announce again once already unknown.
    let events = reader.observe(samples, 200);
    assert!(!events
        .iter()
        .any(|e| matches!(e, PixelBusEvent::Movement(_))));
}

#[test]
fn movement_clears_to_unknown_on_signal_loss() {
    let config = ReaderConfig::default();
    let timeout = config.heartbeat_timeout_ms;
    let mut reader = PixelBusReader::new(config);
    let mut samples = alive();
    samples.movement = Some(MOVEMENT_MOUNTED);
    reader.observe(samples, 0);

    // The status block goes away and stays away past the timeout.
    let gone = BlockSamples::default();
    let events = reader.observe(gone, timeout + 500);
    assert!(events.contains(&PixelBusEvent::SignalLost));
    assert!(events.contains(&PixelBusEvent::Movement(MovementSignal::Unknown)));
}

#[test]
fn life_state_decodes_all_authoritative_values_and_rejects_invalid_evidence() {
    let tolerance = ReaderConfig::default().tolerance;
    assert_eq!(decode_life_state(life(0x20), tolerance), LifeState::Alive);
    assert_eq!(decode_life_state(life(0x80), tolerance), LifeState::Dead);
    assert_eq!(
        decode_life_state(life(0xE0), tolerance),
        LifeState::Reincarnating
    );
    assert_eq!(
        decode_life_state(Rgb::new(0x20, 0x00, 0xDF), tolerance),
        LifeState::Unknown
    );
    assert_eq!(
        decode_life_state(Rgb::new(0x20, 0x89, 0x00), tolerance),
        LifeState::Unknown
    );
    assert_eq!(decode_life_state(life(0x50), tolerance), LifeState::Unknown);
}

#[test]
fn life_state_uses_b21_and_clears_on_invalid_or_lost_signal() {
    let config = ReaderConfig::default();
    assert_eq!(config.life_point(), block_center(config.block_px, 21));

    let mut reader = PixelBusReader::new(config);
    let mut samples = alive();
    samples.life = Some(life(0x20));
    assert!(reader
        .observe(samples, 0)
        .contains(&PixelBusEvent::Life(LifeState::Alive)));

    samples.life = Some(Rgb::new(0x20, 0x89, 0x00));
    assert!(reader
        .observe(samples, 100)
        .contains(&PixelBusEvent::Life(LifeState::Unknown)));

    samples.life = Some(life(0x80));
    reader.observe(samples, 200);
    let lost = reader.observe(BlockSamples::default(), config.heartbeat_timeout_ms + 500);
    assert!(lost.contains(&PixelBusEvent::SignalLost));
    assert!(lost.contains(&PixelBusEvent::Life(LifeState::Unknown)));
}

#[test]
fn life_state_transitions_precede_same_sample_fishing_edges() {
    let mut reader = reader();
    reader.observe(
        BlockSamples {
            fishing: Some(WAITING),
            life: Some(life(0x20)),
            ..alive()
        },
        0,
    );

    let death = reader.observe(
        BlockSamples {
            life: Some(life(0x80)),
            ..alive()
        },
        100,
    );
    let life_index = death
        .iter()
        .position(|event| *event == PixelBusEvent::Life(LifeState::Dead))
        .unwrap();
    let fishing_index = death
        .iter()
        .position(|event| *event == PixelBusEvent::FishingStopped)
        .unwrap();
    assert!(
        life_index < fishing_index,
        "death must close the gate first"
    );

    let recovery = reader.observe(
        BlockSamples {
            fishing: Some(WAITING),
            life: Some(life(0x20)),
            ..alive()
        },
        200,
    );
    let life_index = recovery
        .iter()
        .position(|event| *event == PixelBusEvent::Life(LifeState::Alive))
        .unwrap();
    let fishing_index = recovery
        .iter()
        .position(|event| *event == PixelBusEvent::FishingStarted)
        .unwrap();
    assert!(
        life_index < fishing_index,
        "recovery must open the gate before the fresh cast edge"
    );
}

#[test]
fn world_state_decodes_all_wire_values_and_rejects_invalid_evidence() {
    let tolerance = ReaderConfig::default().tolerance;
    assert_eq!(
        decode_world_state(world(0x20), tolerance),
        WorldState::Unknown
    );
    assert_eq!(
        decode_world_state(world(0x80), tolerance),
        WorldState::Transitioning
    );
    assert_eq!(
        decode_world_state(world(0xE0), tolerance),
        WorldState::Active
    );
    assert_eq!(
        decode_world_state(Rgb::new(0xE0, 0x00, 0x1F), tolerance),
        WorldState::Unknown
    );
    assert_eq!(
        decode_world_state(Rgb::new(0xE0, 0xCC, 0x00), tolerance),
        WorldState::Unknown
    );
    assert_eq!(
        decode_world_state(world(0x50), tolerance),
        WorldState::Unknown
    );
}

#[test]
fn ambiguous_world_state_payloads_fail_closed_at_any_tolerance() {
    let codes = [
        (0x20u8, WorldState::Unknown),
        (0x80, WorldState::Transitioning),
        (0xE0, WorldState::Active),
    ];
    for tolerance in 0..=u8::MAX {
        for red in 0..=u8::MAX {
            let matching: Vec<_> = codes
                .iter()
                .filter(|(code, _)| red.abs_diff(*code) <= tolerance)
                .collect();
            let expected = if matching.len() == 1 {
                matching[0].1
            } else {
                WorldState::Unknown
            };
            assert_eq!(
                decode_world_state(Rgb::new(red, 0xCC, 255 - red), tolerance),
                expected,
                "red {red:#04X} with tolerance {tolerance} matched {} codes",
                matching.len()
            );
        }
    }
}

#[test]
fn world_state_uses_b22_change_detection_and_signal_loss() {
    let config = ReaderConfig::default();
    assert_eq!(config.world_point(), block_center(config.block_px, 22));

    let mut reader = PixelBusReader::new(config);
    let mut samples = alive();
    samples.world = Some(world(0xE0));
    assert!(reader
        .observe(samples, 0)
        .contains(&PixelBusEvent::World(WorldState::Active)));

    assert!(!reader
        .observe(samples, 100)
        .iter()
        .any(|event| matches!(event, PixelBusEvent::World(_))));

    samples.world = Some(Rgb::new(0xE0, 0xCC, 0x00));
    assert!(reader
        .observe(samples, 200)
        .contains(&PixelBusEvent::World(WorldState::Unknown)));

    samples.world = Some(world(0x80));
    reader.observe(samples, 300);
    let lost = reader.observe(BlockSamples::default(), config.heartbeat_timeout_ms + 500);
    assert!(lost.contains(&PixelBusEvent::SignalLost));
    assert!(lost.contains(&PixelBusEvent::World(WorldState::Unknown)));
}

#[test]
fn negotiated_version_one_never_samples_a_screen_pixel_as_b22() {
    let config = ReaderConfig::default();
    let mut reader = PixelBusReader::new(config);
    let mut sampler = MockSampler::new();
    let current = BusLayout::negotiated(120).unwrap();
    let mut header = layout_header_colors(120).unwrap();
    header[0].b = LAYOUT_VERSION_ONE_CODE;
    for (index, color) in header.into_iter().enumerate() {
        let point = current.cell_point(config.block_px, index as u32);
        sampler.set(point.0, point.1, color);
    }
    let status = current.payload_point(config.block_px, 0);
    sampler.set(status.0, status.1, Rgb::new(0xFF, 0x00, 0xFF));
    let ordinary_screen_pixel = current.payload_point(config.block_px, 22);
    sampler.set(
        ordinary_screen_pixel.0,
        ordinary_screen_pixel.1,
        world(0xE0),
    );

    let events = reader.sample_and_observe(&sampler, 0);
    assert!(events.contains(&PixelBusEvent::Heartbeat));
    assert!(!events
        .iter()
        .any(|event| matches!(event, PixelBusEvent::World(WorldState::Active))));
    assert_eq!(
        reader.layout(),
        LayoutState::Ready(BusLayout {
            mode: LayoutMode::Negotiated { version: 1 },
            columns: 120,
            payload_offset: LAYOUT_HEADER_BLOCKS,
        })
    );
}

#[test]
fn world_state_precedes_other_same_sample_transitions() {
    let mut reader = reader();
    reader.observe(
        BlockSamples {
            fishing: Some(WAITING),
            life: Some(life(0x20)),
            world: Some(world(0xE0)),
            ..alive()
        },
        0,
    );

    let events = reader.observe(
        BlockSamples {
            life: Some(life(0x80)),
            world: Some(world(0x80)),
            ..alive()
        },
        100,
    );
    let world_index = events
        .iter()
        .position(|event| *event == PixelBusEvent::World(WorldState::Transitioning))
        .unwrap();
    let life_index = events
        .iter()
        .position(|event| *event == PixelBusEvent::Life(LifeState::Dead))
        .unwrap();
    let fishing_index = events
        .iter()
        .position(|event| *event == PixelBusEvent::FishingStopped)
        .unwrap();
    assert!(world_index < life_index);
    assert!(world_index < fishing_index);
}

#[test]
fn roll_dodge_decodes_all_wire_values_and_rejects_invalid_evidence() {
    let tolerance = ReaderConfig::default().tolerance;
    assert_eq!(
        decode_roll_dodge(roll_dodge(0x20), tolerance),
        RollDodgeState::Unknown
    );
    assert_eq!(
        decode_roll_dodge(roll_dodge(0x80), tolerance),
        RollDodgeState::Inactive
    );
    assert_eq!(
        decode_roll_dodge(roll_dodge(0xE0), tolerance),
        RollDodgeState::Active
    );
    assert_eq!(
        decode_roll_dodge(Rgb::new(0xE0, 0x00, 0x1F), tolerance),
        RollDodgeState::Unknown
    );
    assert_eq!(
        decode_roll_dodge(Rgb::new(0xE0, 0xF9, 0x00), tolerance),
        RollDodgeState::Unknown
    );
    assert_eq!(
        decode_roll_dodge(roll_dodge(0x50), tolerance),
        RollDodgeState::Unknown
    );
}

#[test]
fn roll_dodge_uses_b23_change_detection_and_signal_loss() {
    let config = ReaderConfig::default();
    assert_eq!(config.roll_dodge_point(), block_center(config.block_px, 23));
    let mut reader = PixelBusReader::new(config);
    let mut samples = alive();
    samples.roll_dodge = Some(roll_dodge(0x80));
    assert!(reader
        .observe(samples, 0)
        .contains(&PixelBusEvent::RollDodge(RollDodgeState::Inactive)));
    samples.roll_dodge = Some(roll_dodge(0xE0));
    assert!(reader
        .observe(samples, 100)
        .contains(&PixelBusEvent::RollDodge(RollDodgeState::Active)));
    assert!(!reader
        .observe(samples, 200)
        .iter()
        .any(|event| matches!(event, PixelBusEvent::RollDodge(_))));
    let lost = reader.observe(BlockSamples::default(), config.heartbeat_timeout_ms + 500);
    assert!(lost.contains(&PixelBusEvent::SignalLost));
    assert!(lost.contains(&PixelBusEvent::RollDodge(RollDodgeState::Unknown)));
}

#[test]
fn negotiated_version_two_never_samples_a_screen_pixel_as_b23() {
    let config = ReaderConfig::default();
    let mut reader = PixelBusReader::new(config);
    let mut sampler = MockSampler::new();
    let current = BusLayout::negotiated(120).unwrap();
    let mut header = layout_header_colors(120).unwrap();
    header[0].b = LAYOUT_VERSION_TWO_CODE;
    for (index, color) in header.into_iter().enumerate() {
        let point = current.cell_point(config.block_px, index as u32);
        sampler.set(point.0, point.1, color);
    }
    let status = current.payload_point(config.block_px, 0);
    sampler.set(status.0, status.1, MAGENTA);
    let ordinary_screen_pixel = current.payload_point(config.block_px, 23);
    sampler.set(
        ordinary_screen_pixel.0,
        ordinary_screen_pixel.1,
        roll_dodge(0xE0),
    );
    let events = reader.sample_and_observe(&sampler, 0);
    assert!(events.contains(&PixelBusEvent::Heartbeat));
    assert!(!events
        .iter()
        .any(|event| matches!(event, PixelBusEvent::RollDodge(_))));
}

#[test]
fn travel_decodes_all_wire_values_and_rejects_invalid_evidence() {
    let tolerance = ReaderConfig::default().tolerance;
    assert_eq!(
        decode_travel_state(travel(0x20), tolerance),
        TravelState::Unknown
    );
    assert_eq!(
        decode_travel_state(travel(0x80), tolerance),
        TravelState::Inactive
    );
    assert_eq!(
        decode_travel_state(travel(0xE0), tolerance),
        TravelState::Pending
    );
    assert_eq!(
        decode_travel_state(Rgb::new(0xE0, 0x00, 0x1F), tolerance),
        TravelState::Unknown
    );
    assert_eq!(
        decode_travel_state(Rgb::new(0xE0, 0x13, 0x00), tolerance),
        TravelState::Unknown
    );
}

#[test]
fn travel_uses_b24_change_detection_and_signal_loss() {
    let config = ReaderConfig::default();
    assert_eq!(config.travel_point(), block_center(config.block_px, 24));
    let mut reader = PixelBusReader::new(config);
    let mut samples = alive();
    samples.travel = Some(travel(0x80));
    assert!(reader
        .observe(samples, 0)
        .contains(&PixelBusEvent::Travel(TravelState::Inactive)));
    samples.travel = Some(travel(0xE0));
    assert!(reader
        .observe(samples, 100)
        .contains(&PixelBusEvent::Travel(TravelState::Pending)));
    assert!(!reader
        .observe(samples, 200)
        .iter()
        .any(|event| matches!(event, PixelBusEvent::Travel(_))));
    let lost = reader.observe(BlockSamples::default(), config.heartbeat_timeout_ms + 500);
    assert!(lost.contains(&PixelBusEvent::Travel(TravelState::Unknown)));
}

#[test]
fn negotiated_version_three_keeps_24_blocks_and_never_samples_b24() {
    let config = ReaderConfig::default();
    let mut reader = PixelBusReader::new(config);
    let mut sampler = MockSampler::new();
    let current = BusLayout::negotiated(120).unwrap();
    let mut header = layout_header_colors(120).unwrap();
    header[0].b = LAYOUT_VERSION_THREE_CODE;
    for (index, color) in header.into_iter().enumerate() {
        let point = current.cell_point(config.block_px, index as u32);
        sampler.set(point.0, point.1, color);
    }
    let status = current.payload_point(config.block_px, 0);
    sampler.set(status.0, status.1, MAGENTA);
    let screen_pixel = current.payload_point(config.block_px, 24);
    sampler.set(screen_pixel.0, screen_pixel.1, travel(0xE0));
    let events = reader.sample_and_observe(&sampler, 0);
    assert_eq!(
        BusLayout {
            mode: LayoutMode::Negotiated { version: 3 },
            columns: 120,
            payload_offset: LAYOUT_HEADER_BLOCKS,
        }
        .payload_blocks(),
        LAYOUT_VERSION_THREE_BLOCKS
    );
    assert!(events.contains(&PixelBusEvent::Heartbeat));
    assert!(!events
        .iter()
        .any(|event| matches!(event, PixelBusEvent::Travel(_))));
}

#[test]
fn the_legacy_capture_region_is_two_rows_after_the_count_crossed() {
    // The parametric half is unchanged and still true: the region is one row for
    // any count up to COLUMNS, and the first block past it starts a second row.
    // That was the general statement before any count reached it.
    let block_px = DEFAULT_BLOCK_PX;
    for count in 1..=COLUMNS {
        assert_eq!(
            grid_extent(block_px, count, COLUMNS),
            Size::new(block_px * count, block_px),
            "{count} blocks should still be a single row"
        );
    }
    assert_eq!(
        grid_extent(block_px, COLUMNS + 1, COLUMNS),
        Size::new(block_px * COLUMNS, block_px * 2),
        "the first block past the column count must start a second row"
    );

    // And the concrete instance for the constants this slice ships. Slice 038 is
    // the first shipping count to cross, so this is the first time the general
    // statement above and the shipped grid are describing the same thing. That the
    // count exceeds the column count is asserted at compile time further up rather
    // than here, where every operand is a constant.
    assert_eq!(grid_rows(NUM_BLOCKS, COLUMNS), 2);
    assert_eq!(capture_dims(block_px), (block_px * COLUMNS, block_px * 2));

    // The shape in full: a full first row, eight blocks on the second.
    assert_eq!(NUM_BLOCKS - COLUMNS, 9, "row 1 should hold nine blocks");
}

// Slice 037: the six skill-cooldown blocks (B10 to B15).

/// The six marks in block order, mirroring the addon encoder.
const COOLDOWN_MARKS: [(&str, u8); 6] = [
    ("skill 1", 0x0B),
    ("skill 2", 0x21),
    ("skill 3", 0x4E),
    ("skill 4", 0x92),
    ("skill 5", 0xC6),
    ("ultimate", 0xE8),
];

/// A cooldown block color: the step count in red, the mark in green, and the
/// complement checksum in blue.
fn cooldown(steps: u8, mark: u8) -> Rgb {
    Rgb::new(steps, mark, 255 - steps)
}

#[test]
fn decode_cooldown_reads_ready_durations_and_unavailable() {
    let t = ReaderConfig::default().tolerance;
    for (name, mark) in COOLDOWN_MARKS {
        assert_eq!(
            decode_cooldown(cooldown(0, mark), mark, t),
            SlotCooldown::Ready,
            "{name} at zero steps should be ready"
        );
        assert_eq!(
            decode_cooldown(cooldown(1, mark), mark, t),
            SlotCooldown::RemainingMs(50),
            "{name} at one step"
        );
        assert_eq!(
            decode_cooldown(cooldown(24, mark), mark, t),
            SlotCooldown::RemainingMs(1200),
            "{name} at twenty four steps"
        );
        assert_eq!(
            decode_cooldown(cooldown(254, mark), mark, t),
            SlotCooldown::RemainingMs(12700),
            "{name} at the maximum step count"
        );
        assert_eq!(
            decode_cooldown(cooldown(255, mark), mark, t),
            SlotCooldown::Unknown,
            "{name} at the unavailable sentinel"
        );
    }
}

#[test]
fn decode_cooldown_rejects_every_other_blocks_marker() {
    // Six adjacent blocks carrying the same kind of value are exactly where an
    // off-by-one geometry error would decode a neighbour's cooldown as this
    // slot's. Each block must reject every other mark on the grid, including the
    // five sibling cooldown marks.
    let t = ReaderConfig::default().tolerance;
    for (name, mark) in COOLDOWN_MARKS {
        for (other_name, other) in BLOCK_CENTER_GREENS {
            if other.abs_diff(mark) <= t {
                continue;
            }
            assert_eq!(
                decode_cooldown(cooldown(24, other), mark, t),
                SlotCooldown::Unknown,
                "{other_name} decoded as the {name} cooldown"
            );
        }
    }
}

#[test]
fn decode_cooldown_rejects_a_failed_checksum() {
    let t = ReaderConfig::default().tolerance;
    for (name, mark) in COOLDOWN_MARKS {
        assert_eq!(
            decode_cooldown(Rgb::new(24, mark, 0), mark, t),
            SlotCooldown::Unknown,
            "{name} with a broken checksum"
        );
    }
}

#[test]
fn no_arbitrary_color_decodes_as_a_cooldown() {
    // US2: an addon older than version 11 draws none of these blocks, leaving
    // whatever the game renders at those points.
    let t = ReaderConfig::default().tolerance;
    let mark = COOLDOWN_MARKS[0].1;
    let mut checked = 0u32;
    for r in (0u32..=255).step_by(5) {
        for g in (0u32..=255).step_by(5) {
            for b in (0u32..=255).step_by(5) {
                let sample = Rgb::new(r as u8, g as u8, b as u8);
                let is_real = sample.g.abs_diff(mark) <= t
                    && (i32::from(sample.r) + i32::from(sample.b) - 255).unsigned_abs()
                        <= u32::from(t);
                if is_real {
                    continue;
                }
                checked += 1;
                assert_eq!(
                    decode_cooldown(sample, mark, t),
                    SlotCooldown::Unknown,
                    "color {sample:?} decoded as a cooldown"
                );
            }
        }
    }
    assert!(checked > 100_000, "sweep covered only {checked} colors");
}

#[test]
fn legacy_cooldown_points_are_the_eleventh_through_sixteenth_block_centers() {
    for block_px in [MIN_BLOCK_PX, DEFAULT_BLOCK_PX, 24, MAX_BLOCK_PX] {
        let config = ReaderConfig {
            block_px,
            ..Default::default()
        };
        let points = [
            config.cooldown_skill_1_point(),
            config.cooldown_skill_2_point(),
            config.cooldown_skill_3_point(),
            config.cooldown_skill_4_point(),
            config.cooldown_skill_5_point(),
            config.cooldown_ultimate_point(),
        ];
        for (offset, point) in points.iter().enumerate() {
            let index = 10 + offset as u32;
            assert_eq!(*point, block_center(block_px, index));
            // All six stay on row 0: the grid fills the row exactly.
            assert_eq!(grid_position(index, COLUMNS), (index, 0));
        }
    }
}

#[test]
fn cooldown_change_emits_one_aggregate_event() {
    let mut reader = PixelBusReader::new(ReaderConfig::default());
    let mut samples = alive();
    samples.cooldown_skill_1 = Some(cooldown(0, COOLDOWN_MARKS[0].1));
    let events = reader.observe(samples, 0);
    let count = events
        .iter()
        .filter(|e| matches!(e, PixelBusEvent::Cooldowns(_)))
        .count();
    assert_eq!(count, 1);

    // The same samples again announce nothing.
    let events = reader.observe(samples, 100);
    assert!(!events
        .iter()
        .any(|e| matches!(e, PixelBusEvent::Cooldowns(_))));

    // Three slots moving at once is still one event, which is the whole reason
    // the six values travel as a set.
    samples.cooldown_skill_1 = Some(cooldown(20, COOLDOWN_MARKS[0].1));
    samples.cooldown_skill_2 = Some(cooldown(20, COOLDOWN_MARKS[1].1));
    samples.cooldown_skill_3 = Some(cooldown(20, COOLDOWN_MARKS[2].1));
    let events = reader.observe(samples, 200);
    let cooldown_events: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, PixelBusEvent::Cooldowns(_)))
        .collect();
    assert_eq!(cooldown_events.len(), 1);
}

#[test]
fn cooldowns_clear_when_a_block_stops_decoding() {
    let mut reader = PixelBusReader::new(ReaderConfig::default());
    let mut samples = alive();
    samples.cooldown_skill_1 = Some(cooldown(20, COOLDOWN_MARKS[0].1));
    reader.observe(samples, 0);

    samples.cooldown_skill_1 = Some(Rgb::new(0x11, 0x22, 0x33));
    let events = reader.observe(samples, 100);
    match events
        .iter()
        .find(|e| matches!(e, PixelBusEvent::Cooldowns(_)))
    {
        Some(PixelBusEvent::Cooldowns(set)) => {
            assert_eq!(set.skill_1, SlotCooldown::Unknown)
        }
        _ => panic!("a non-decoding block must clear that slot"),
    }
}

#[test]
fn cooldowns_clear_on_signal_loss() {
    let config = ReaderConfig::default();
    let timeout = config.heartbeat_timeout_ms;
    let mut reader = PixelBusReader::new(config);
    let mut samples = alive();
    samples.cooldown_skill_1 = Some(cooldown(20, COOLDOWN_MARKS[0].1));
    reader.observe(samples, 0);

    let events = reader.observe(BlockSamples::default(), timeout + 500);
    assert!(events.contains(&PixelBusEvent::SignalLost));
    assert!(events.contains(&PixelBusEvent::Cooldowns(CooldownSet::new_unknown())));
}

// Slice 042: the five quickslot blocks (B16 to B20), the first blocks on row 1.

/// The four marks, mirroring the addon encoder. Kept as literals rather than
/// imported, so a change to a companion constant has to be made here too and the
/// two cannot drift together into agreeing on something wrong.
const QUICKSLOT_MARK: u8 = 0x38;
const QUICKSLOT_STATE_MARK: u8 = 0x76;
const QUICKSLOT_ID_MARKS: [(&str, u8); 3] =
    [("id high", 0xB0), ("id middle", 0xDD), ("id low", 0xF3)];

/// The quickslot cooldown block color: the step count in red, the mark in green,
/// and the complement checksum in blue. Identical in shape to the skill cooldown
/// blocks, which is the point: the encoding is shared, not parallel.
fn quickslot(steps: u8) -> Rgb {
    Rgb::new(steps, QUICKSLOT_MARK, 255 - steps)
}

/// One identity byte block color.
fn quickslot_id(byte: u8, mark: u8) -> Rgb {
    Rgb::new(byte, mark, 255 - byte)
}

fn quickslot_state(code: u8) -> Rgb {
    Rgb::new(code, QUICKSLOT_STATE_MARK, 255 - code)
}

/// The three identity blocks for an identity, most significant byte first.
fn quickslot_id_blocks(id: u32) -> (Option<Rgb>, Option<Rgb>, Option<Rgb>) {
    (
        Some(quickslot_id((id >> 16) as u8, QUICKSLOT_ID_MARKS[0].1)),
        Some(quickslot_id((id >> 8) as u8, QUICKSLOT_ID_MARKS[1].1)),
        Some(quickslot_id(id as u8, QUICKSLOT_ID_MARKS[2].1)),
    )
}

/// A fully decoding quickslot at `steps` holding item `id`.
fn decoded_quickslot(steps: u8, id: u32) -> QuickslotState {
    let (hi, mid, lo) = quickslot_id_blocks(id);
    decode_quickslot(
        Some(quickslot(steps)),
        hi,
        mid,
        lo,
        Some(quickslot_state(0xD0)),
        ReaderConfig::default().tolerance,
    )
}

#[test]
fn decode_quickslot_maps_every_explicit_discriminant() {
    let t = ReaderConfig::default().tolerance;
    let cases = [
        (
            0x10,
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::UnsupportedApi),
        ),
        (
            0x20,
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::InvalidSelection),
        ),
        (
            0x30,
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::InconsistentFacts),
        ),
        (0x40, QuickslotClassification::Empty),
        (
            0x50,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Item),
        ),
        (
            0x60,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Collectible),
        ),
        (
            0x70,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::QuestItem),
        ),
        (
            0x80,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Emote),
        ),
        (
            0x90,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::QuickChat),
        ),
        (
            0xA0,
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Other),
        ),
        (
            0xB0,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Depleted),
        ),
        (
            0xC0,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Blocked),
        ),
        (
            0xD0,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
        ),
    ];
    let (hi, mid, lo) = quickslot_id_blocks(0x12_3456);
    for (code, expected) in cases {
        let state = decode_quickslot(
            Some(quickslot(0)),
            hi,
            mid,
            lo,
            Some(quickslot_state(code)),
            t,
        );
        assert_eq!(state.classification, expected, "state code {code:#04X}");
        assert_eq!(state.item_id.is_some(), state.is_potion());
        assert_eq!(state.is_usable_potion(), code == 0xD0);
        assert_eq!(state.authorizes_auto_potion(), code == 0xD0);
    }
}

#[test]
fn explicit_classification_is_independent_from_cooldown_and_identity() {
    let t = ReaderConfig::default().tolerance;
    for steps in [0, 1, 254, 255] {
        let state = decode_quickslot(
            Some(quickslot(steps)),
            None,
            None,
            None,
            Some(quickslot_state(0xD0)),
            t,
        );
        assert!(state.is_potion(), "cooldown payload {steps} changed class");
        assert!(state.is_usable_potion());
        assert_eq!(state.item_id, None);
    }
}

#[test]
fn missing_and_corrupt_state_blocks_fail_closed() {
    let t = ReaderConfig::default().tolerance;
    let legacy = decode_quickslot(Some(quickslot(0)), None, None, None, None, t);
    assert_eq!(
        legacy.classification,
        QuickslotClassification::Unavailable(QuickslotUnavailableReason::LegacyProtocol)
    );
    let corrupt = decode_quickslot(
        Some(quickslot(0)),
        None,
        None,
        None,
        Some(Rgb::new(0xD0, QUICKSLOT_STATE_MARK, 0)),
        t,
    );
    assert_eq!(
        corrupt.classification,
        QuickslotClassification::Unavailable(QuickslotUnavailableReason::CorruptProtocol)
    );
    assert!(!legacy.is_potion() && !corrupt.is_potion());
}

#[test]
fn quickslot_discriminant_survives_capture_drift_within_tolerance() {
    let t = ReaderConfig::default().tolerance;
    for delta in 0..=t {
        let sample = Rgb::new(0xD0 + delta, QUICKSLOT_STATE_MARK + delta, 0x2F - delta);
        let state = decode_quickslot(Some(quickslot(0)), None, None, None, Some(sample), t);
        assert_eq!(
            state.classification,
            QuickslotClassification::Potion(QuickslotPotionAvailability::Usable)
        );
    }
}

#[test]
fn ambiguous_quickslot_discriminants_fail_closed_at_any_tolerance() {
    let codes = [
        0x10u8, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0,
    ];
    for tolerance in 0..=u8::MAX {
        for red in 0..=u8::MAX {
            let matching = codes
                .iter()
                .filter(|code| red.abs_diff(**code) <= tolerance)
                .count();
            if matching == 1 {
                continue;
            }
            let state = decode_quickslot(
                Some(quickslot(0)),
                None,
                None,
                None,
                Some(Rgb::new(red, QUICKSLOT_STATE_MARK, 255 - red)),
                tolerance,
            );
            assert_eq!(
                state.classification,
                QuickslotClassification::Unavailable(QuickslotUnavailableReason::CorruptProtocol),
                "red {red:#04X} with tolerance {tolerance} matched {matching} codes"
            );
        }
    }
}

#[test]
fn decode_quickslot_reads_ready_durations_and_unavailable() {
    // The same five cases as the skill cooldown blocks, because it is the same
    // encoding read through the same decoder.
    for (steps, expected) in [
        (0u8, SlotCooldown::Ready),
        (1, SlotCooldown::RemainingMs(50)),
        (24, SlotCooldown::RemainingMs(1200)),
        (254, SlotCooldown::RemainingMs(12700)),
        (255, SlotCooldown::Unknown),
    ] {
        assert_eq!(
            decoded_quickslot(steps, 0x00_1234).cooldown,
            expected,
            "at {steps} steps"
        );
    }
}

#[test]
fn decode_quickslot_assembles_the_identity_most_significant_byte_first() {
    // The byte order is the whole risk in a three-block value: reversed, it
    // decodes cleanly to a different number and nothing complains.
    for id in [0x00_0000u32, 0x00_0001, 0x01_0000, 0x12_3456, 0xFF_FFFF] {
        assert_eq!(
            decoded_quickslot(10, id).item_id,
            Some(id),
            "identity {id:#08X} did not round trip"
        );
    }

    // Explicitly: the high byte is the first block, not the last.
    let state = decoded_quickslot(10, 0xAB_CDEF);
    assert_eq!(state.item_id, Some(0xAB_CDEF));
    assert_ne!(
        state.item_id,
        Some(0xEF_CDAB),
        "the identity was assembled least significant byte first"
    );
}

#[test]
fn decode_quickslot_reports_no_identity_without_a_potion() {
    // FR-008: the cooldown saying unknown settles it, whatever the identity
    // blocks carry. This is what stops a consumer acting on an identity that
    // describes a slot with nothing usable in it.
    let t = ReaderConfig::default().tolerance;
    let (hi, mid, lo) = quickslot_id_blocks(0x12_3456);
    let state = decode_quickslot(
        Some(quickslot(255)),
        hi,
        mid,
        lo,
        Some(quickslot_state(0x40)),
        t,
    );
    assert_eq!(state.cooldown, SlotCooldown::Unknown);
    assert_eq!(state.item_id, None);
    assert!(!state.is_potion());
}

#[test]
fn a_quickslot_identity_never_exists_without_a_potion() {
    // The invariant from the data model, over every combination of the four
    // blocks decoding or not: item_id being present implies is_potion, always.
    let t = ReaderConfig::default().tolerance;
    let junk = Rgb::new(0x11, 0x22, 0x33);
    let (good_hi, good_mid, good_lo) = quickslot_id_blocks(0x12_3456);
    let mut checked = 0;
    for status in [None, Some(junk), Some(quickslot(255)), Some(quickslot(12))] {
        for hi in [None, Some(junk), good_hi] {
            for mid in [None, Some(junk), good_mid] {
                for lo in [None, Some(junk), good_lo] {
                    let state =
                        decode_quickslot(status, hi, mid, lo, Some(quickslot_state(0xD0)), t);
                    if state.item_id.is_some() {
                        assert!(
                            state.is_potion(),
                            "an identity was reported with no potion: {state:?}"
                        );
                    }
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 4 * 3 * 3 * 3);
}

#[test]
fn decode_quickslot_rejects_every_other_blocks_marker() {
    // Four adjacent blocks, three of which carry an unconstrained byte, are
    // exactly where an off-by-one geometry error would decode a neighbour's
    // payload as this one. Each must reject every other mark on the grid.
    let t = ReaderConfig::default().tolerance;
    for (other_name, other) in BLOCK_CENTER_GREENS {
        if other.abs_diff(QUICKSLOT_MARK) > t {
            let wrong = Rgb::new(24, other, 255 - 24);
            assert_eq!(
                decode_quickslot(
                    Some(wrong),
                    None,
                    None,
                    None,
                    Some(quickslot_state(0xD0)),
                    t
                )
                .cooldown,
                SlotCooldown::Unknown,
                "{other_name} decoded as the quickslot cooldown"
            );
        }
    }

    // And each identity block rejects every other mark, which is what an
    // off-by-one across these four adjacent blocks would actually produce.
    for (position, (name, mark)) in QUICKSLOT_ID_MARKS.iter().enumerate() {
        for (other_name, other) in BLOCK_CENTER_GREENS {
            if other.abs_diff(*mark) <= t {
                continue;
            }
            let (mut hi, mut mid, mut lo) = quickslot_id_blocks(0x12_3456);
            let wrong = Some(quickslot_id(0x42, other));
            match position {
                0 => hi = wrong,
                1 => mid = wrong,
                _ => lo = wrong,
            }
            let state = decode_quickslot(
                Some(quickslot(10)),
                hi,
                mid,
                lo,
                Some(quickslot_state(0xD0)),
                t,
            );
            assert_eq!(
                state.item_id, None,
                "{other_name} decoded as the quickslot {name} byte"
            );
            assert_eq!(
                state.cooldown,
                SlotCooldown::RemainingMs(500),
                "a bad identity block must not disturb the cooldown"
            );
        }
    }
}

#[test]
fn a_partial_identity_is_never_assembled() {
    // FR-008 and SC-008. One unreadable byte means no identity, not an identity
    // built from the two that read. Classification and cooldown remain intact
    // because all three facts degrade independently.
    let t = ReaderConfig::default().tolerance;
    for (position, (_, mark)) in QUICKSLOT_ID_MARKS.iter().enumerate() {
        let (mut hi, mut mid, mut lo) = quickslot_id_blocks(0x12_3456);
        // A broken complement, which is the failure a stray pixel produces.
        let bad = Some(Rgb::new(0x42, *mark, 0x00));
        match position {
            0 => hi = bad,
            1 => mid = bad,
            _ => lo = bad,
        }
        let state = decode_quickslot(
            Some(quickslot(10)),
            hi,
            mid,
            lo,
            Some(quickslot_state(0xD0)),
            t,
        );
        assert_eq!(state.item_id, None, "byte {position} broken");
        assert_eq!(state.cooldown, SlotCooldown::RemainingMs(500));
        assert!(state.is_potion());
    }

    // An absent block is the same as an unreadable one.
    let (_, mid, lo) = quickslot_id_blocks(0x12_3456);
    let state = decode_quickslot(
        Some(quickslot(10)),
        None,
        mid,
        lo,
        Some(quickslot_state(0xD0)),
        t,
    );
    assert_eq!(state.item_id, None);
    assert_eq!(state.cooldown, SlotCooldown::RemainingMs(500));
}

#[test]
fn decode_quickslot_rejects_a_failed_checksum() {
    let t = ReaderConfig::default().tolerance;
    assert_eq!(
        decode_quickslot(
            Some(Rgb::new(24, QUICKSLOT_MARK, 0)),
            None,
            None,
            None,
            Some(quickslot_state(0xD0)),
            t
        )
        .cooldown,
        SlotCooldown::Unknown
    );
}

#[test]
fn no_arbitrary_color_decodes_as_a_quickslot_cooldown() {
    // The US2 case at full strength: with the addon too old to draw these
    // blocks, whatever the game happens to be painting a row below the beacon is
    // sampled. None of it may become a cooldown.
    let t = ReaderConfig::default().tolerance;
    let mut checked = 0u32;
    for r in (0..=255u16).step_by(3) {
        for g in (0..=255u16).step_by(3) {
            for b in (0..=255u16).step_by(3) {
                let sample = Rgb::new(r as u8, g as u8, b as u8);
                let decoded = decode_quickslot(
                    Some(sample),
                    None,
                    None,
                    None,
                    Some(quickslot_state(0xD0)),
                    t,
                )
                .cooldown;
                if decoded != SlotCooldown::Unknown {
                    // The only colors that may decode are the encoding itself.
                    assert!(
                        sample.g.abs_diff(QUICKSLOT_MARK) <= t
                            && (u16::from(sample.r) + u16::from(sample.b)).abs_diff(255)
                                <= u16::from(t),
                        "{sample:?} decoded as {decoded:?} without being the encoding"
                    );
                }
                checked += 1;
            }
        }
    }
    assert!(checked > 600_000, "sweep covered only {checked} colors");
}

#[test]
fn legacy_quickslot_points_are_the_first_five_blocks_of_row_one() {
    // FR-021: derived from the shared rule with no special case for the second
    // row. These are the first sample points in the project whose y is not
    // block_px / 2.
    for block_px in [MIN_BLOCK_PX, DEFAULT_BLOCK_PX, 24, MAX_BLOCK_PX] {
        let config = ReaderConfig {
            block_px,
            ..Default::default()
        };
        let points = [
            config.quickslot_status_point(),
            config.quickslot_id_hi_point(),
            config.quickslot_id_mid_point(),
            config.quickslot_id_lo_point(),
            config.quickslot_state_point(),
        ];
        for (offset, point) in points.iter().enumerate() {
            let offset = offset as u32;
            let index = COLUMNS + offset;
            assert_eq!(*point, block_center(block_px, index));
            assert_eq!(grid_position(index, COLUMNS), (offset, 1));
            assert_eq!(
                *point,
                (block_px * offset + block_px / 2, block_px + block_px / 2),
                "block {index} at block_px {block_px} is not on row 1"
            );
        }
    }
}

#[test]
fn quickslot_change_emits_exactly_one_event_carrying_the_whole_state() {
    let mut reader = PixelBusReader::new(ReaderConfig::default());
    let mut samples = alive();
    let (hi, mid, lo) = quickslot_id_blocks(0x12_3456);
    samples.quickslot_status = Some(quickslot(0));
    samples.quickslot_id_hi = hi;
    samples.quickslot_id_mid = mid;
    samples.quickslot_id_lo = lo;
    samples.quickslot_state = Some(quickslot_state(0xD0));

    let events = reader.observe(samples, 0);
    let quickslots: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, PixelBusEvent::Quickslot(_)))
        .collect();
    assert_eq!(quickslots.len(), 1, "one event, not four");
    assert_eq!(
        quickslots[0],
        &PixelBusEvent::Quickslot(QuickslotState {
            classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable,),
            cooldown: SlotCooldown::Ready,
            item_id: Some(0x12_3456),
        })
    );

    // Twenty unchanged samples announce nothing, proving the 1 Hz backstop does
    // not turn a stable selection into repeated UI or automation events.
    for tick in 1..=20 {
        let events = reader.observe(samples, tick * 100);
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, PixelBusEvent::Quickslot(_))),
            "unchanged tick {tick} emitted a quickslot event"
        );
    }

    // A swap moves the identity and the cooldown together, as one event.
    let (hi2, mid2, lo2) = quickslot_id_blocks(0x00_00AA);
    samples.quickslot_status = Some(quickslot(40));
    samples.quickslot_id_hi = hi2;
    samples.quickslot_id_mid = mid2;
    samples.quickslot_id_lo = lo2;
    samples.quickslot_state = Some(quickslot_state(0xD0));
    let events = reader.observe(samples, 2_100);
    let quickslots: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, PixelBusEvent::Quickslot(_)))
        .collect();
    assert_eq!(quickslots.len(), 1, "a swap is one change, not two");
    assert_eq!(
        quickslots[0],
        &PixelBusEvent::Quickslot(QuickslotState {
            classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable,),
            cooldown: SlotCooldown::RemainingMs(2000),
            item_id: Some(0x00_00AA),
        })
    );
}

#[test]
fn an_addon_without_the_quickslot_blocks_reports_unknown_and_announces_nothing() {
    // FR-020: the blocks are optional, and their absence is the shipping state
    // for every operator who has not updated the addon yet.
    let mut reader = PixelBusReader::new(ReaderConfig::default());
    let events = reader.observe(alive(), 0);
    assert!(
        !events
            .iter()
            .any(|e| matches!(e, PixelBusEvent::Quickslot(_))),
        "absent blocks must not announce a change from the initial unknown"
    );
}

#[test]
fn quickslot_clears_to_corrupt_when_the_discriminant_stops_decoding() {
    // A stale positive classification must not survive damage to its authority.
    let mut reader = PixelBusReader::new(ReaderConfig::default());
    let mut samples = alive();
    let (hi, mid, lo) = quickslot_id_blocks(0x12_3456);
    samples.quickslot_status = Some(quickslot(0));
    samples.quickslot_id_hi = hi;
    samples.quickslot_id_mid = mid;
    samples.quickslot_id_lo = lo;
    samples.quickslot_state = Some(quickslot_state(0xD0));
    reader.observe(samples, 0);

    samples.quickslot_state = Some(Rgb::new(0x11, 0x22, 0x33));
    let events = reader.observe(samples, 100);
    assert!(events.contains(&PixelBusEvent::Quickslot(QuickslotState {
        classification: QuickslotClassification::Unavailable(
            QuickslotUnavailableReason::CorruptProtocol,
        ),
        cooldown: SlotCooldown::Ready,
        item_id: None,
    })));

    // And it does not announce again once already corrupt.
    let events = reader.observe(samples, 200);
    assert!(!events
        .iter()
        .any(|e| matches!(e, PixelBusEvent::Quickslot(_))));
}

#[test]
fn quickslot_clears_to_unknown_on_signal_loss() {
    let config = ReaderConfig::default();
    let timeout = config.heartbeat_timeout_ms;
    let mut reader = PixelBusReader::new(config);
    let mut samples = alive();
    let (hi, mid, lo) = quickslot_id_blocks(0x12_3456);
    samples.quickslot_status = Some(quickslot(20));
    samples.quickslot_id_hi = hi;
    samples.quickslot_id_mid = mid;
    samples.quickslot_id_lo = lo;
    samples.quickslot_state = Some(quickslot_state(0xD0));
    reader.observe(samples, 0);

    let events = reader.observe(BlockSamples::default(), timeout + 500);
    assert!(events.contains(&PixelBusEvent::SignalLost));
    assert!(events.contains(&PixelBusEvent::Quickslot(QuickslotState::new_unknown())));
}
