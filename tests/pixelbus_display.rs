//! Out-of-band display detection: geometry primitives, the stored-settings
//! parser, descriptor construction, reconciliation, and the detector's change
//! detection and read gating.
//!
//! Every test here runs with no game, no window, and no display hardware. The
//! read-gating tests are the ones worth reading closely: they count how many
//! times the settings-read closure is invoked across a scripted sequence of
//! measurements, which is what turns "the settings file is not read on every
//! sampling cycle" from an intention into a tested guarantee.

use eso_weave::pixelbus::display::{
    parse_user_settings, reconcile, DisplayDescriptor, DisplayDetector, DisplaySource,
    MeasuredDisplay, Point, Reconciliation, Size, StoredPair, StoredVideoSettings,
};

/// A settings file shaped like a real one: the keys of interest interleaved with
/// unrelated entries, values always quoted including the numerics, and one key
/// carrying the version suffix the game adds when a setting's meaning changes.
const REALISTIC: &str = "\
SET GraphicsDriver.6 \"D3D11\"
SET FullscreenWidth \"3440\"
SET FullscreenHeight \"1440\"
SET WindowedWidth \"5160\"
SET WindowedHeight \"2160\"
SET FULLSCREEN \"2\"
SET PreferExclusiveFullscreen \"0\"
SET PreferMaximizedWindow \"0\"
SET ACTIVE_DISPLAY \"0\"
SET OverscanWidthAdjustment \"0\"
SET OverscanHeightAdjustment \"0\"
SET CustomUIScale \"1.00000000\"
SET UseCustomUIScale.2 \"0\"
SET GamepadCustomUIScale \"1.00000000\"
SET UseGamepadCustomUIScale \"0\"
SET MinFrameTime.2 \"0.00000000\"
";

// ---------------------------------------------------------------------------
// Geometry primitives
// ---------------------------------------------------------------------------

#[test]
fn size_reports_empty_when_either_dimension_is_zero() {
    assert!(!Size::new(1920, 1080).is_empty());
    assert!(Size::new(0, 1080).is_empty());
    assert!(Size::new(1920, 0).is_empty());
    assert!(Size::new(0, 0).is_empty());
}

#[test]
fn geometry_primitives_compare_exactly() {
    // Change detection is an exact comparison, which is only possible because
    // every value in the model is integral.
    assert_eq!(Size::new(800, 600), Size::new(800, 600));
    assert_ne!(Size::new(800, 600), Size::new(800, 601));
    assert_eq!(Point::new(-1920, 0), Point::new(-1920, 0));
    assert_ne!(Point::new(-1920, 0), Point::new(0, 0));
}

// ---------------------------------------------------------------------------
// The parser
// ---------------------------------------------------------------------------

#[test]
fn parses_every_key_of_interest_from_a_realistic_file() {
    let s = parse_user_settings(REALISTIC);
    assert_eq!(s.fullscreen, Some(Size::new(3440, 1440)));
    assert_eq!(s.windowed, Some(Size::new(5160, 2160)));
    assert_eq!(s.window_mode_raw, Some(2));
    assert_eq!(s.prefer_exclusive_fullscreen, Some(0));
    assert_eq!(s.prefer_maximized_window, Some(0));
    assert_eq!(s.active_display, Some(0));
    assert_eq!(s.overscan, Some(Point::new(0, 0)));
    assert_eq!(s.custom_ui_scale, Some(1.0));
    assert_eq!(s.use_custom_ui_scale, Some(0));
    assert_eq!(s.gamepad_custom_ui_scale, Some(1.0));
    assert_eq!(s.use_gamepad_custom_ui_scale, Some(0));
}

#[test]
fn a_version_suffix_still_matches_its_base_key() {
    // The game bumps this suffix when a setting's meaning changes, so matching a
    // fixed literal would silently stop matching after a patch.
    for suffix in ["", ".2", ".3", ".17"] {
        let text = format!("SET UseCustomUIScale{suffix} \"1\"\n");
        assert_eq!(
            parse_user_settings(&text).use_custom_ui_scale,
            Some(1),
            "suffix {suffix:?} should match the base key"
        );
    }
}

#[test]
fn base_keys_are_compared_whole_and_never_by_prefix() {
    // FULLSCREEN and FullscreenWidth share a prefix and are different settings.
    let s = parse_user_settings("SET FULLSCREEN \"1\"\n");
    assert_eq!(s.window_mode_raw, Some(1));
    assert_eq!(s.fullscreen, None);

    let s = parse_user_settings("SET FullscreenWidth \"1920\"\nSET FullscreenHeight \"1080\"\n");
    assert_eq!(s.window_mode_raw, None);
    assert_eq!(s.fullscreen, Some(Size::new(1920, 1080)));
}

#[test]
fn key_matching_is_case_insensitive() {
    let s = parse_user_settings("set fullscreenwidth \"640\"\nSET FULLSCREENHEIGHT \"480\"\n");
    assert_eq!(s.fullscreen, Some(Size::new(640, 480)));
}

#[test]
fn empty_and_unrelated_input_yields_an_empty_reading() {
    for text in [
        "",
        "\n\n\n",
        "this is not a settings file at all",
        "{\"json\": true}",
        "\u{0}\u{1}\u{2}",
    ] {
        let s = parse_user_settings(text);
        assert_eq!(s, StoredVideoSettings::default(), "input {text:?}");
    }
}

#[test]
fn malformed_lines_are_skipped_without_discarding_the_rest() {
    let text = "\
SET
SET FullscreenWidth
SET FullscreenWidth \"1920
NOTSET FullscreenHeight \"1080\"
SET FullscreenHeight \"1080\"
";
    let s = parse_user_settings(text);
    // The truncated width line still yields a value once the unmatched quote is
    // handled permissively; what matters is that the file did not abort and the
    // later, well-formed height line was read.
    assert_eq!(s.fullscreen, Some(Size::new(1920, 1080)));
}

#[test]
fn an_unparsable_value_leaves_only_its_own_field_absent() {
    let text = "\
SET FULLSCREEN \"not a number\"
SET ACTIVE_DISPLAY \"1\"
SET CustomUIScale \"not a float\"
SET PreferMaximizedWindow \"1\"
";
    let s = parse_user_settings(text);
    assert_eq!(s.window_mode_raw, None);
    assert_eq!(s.custom_ui_scale, None);
    assert_eq!(s.active_display, Some(1));
    assert_eq!(s.prefer_maximized_window, Some(1));
}

#[test]
fn a_half_present_resolution_pair_is_not_a_pair() {
    assert_eq!(
        parse_user_settings("SET FullscreenWidth \"1920\"\n").fullscreen,
        None
    );
    assert_eq!(
        parse_user_settings("SET FullscreenHeight \"1080\"\n").fullscreen,
        None
    );
    assert_eq!(
        parse_user_settings("SET WindowedWidth \"1280\"\n").windowed,
        None
    );
}

#[test]
fn a_duplicate_key_resolves_to_the_last_assignment() {
    let text = "\
SET ACTIVE_DISPLAY \"0\"
SET ACTIVE_DISPLAY \"1\"
SET ACTIVE_DISPLAY \"2\"
";
    assert_eq!(parse_user_settings(text).active_display, Some(2));
}

#[test]
fn unknown_keys_are_ignored() {
    let text = "SET SomeFutureKey \"7\"\nSET ACTIVE_DISPLAY \"3\"\n";
    let s = parse_user_settings(text);
    assert_eq!(s.active_display, Some(3));
}

#[test]
fn values_without_quotes_are_still_read() {
    // The live file always quotes, including numerics. Accepting an unquoted
    // value costs nothing and guards against a format the game has not used yet.
    assert_eq!(
        parse_user_settings("SET ACTIVE_DISPLAY 4\n").active_display,
        Some(4)
    );
}

#[test]
fn an_unrecognized_window_mode_value_is_carried_raw() {
    // There is no verified integer-to-mode mapping, so every value including
    // unseen ones is carried as read and none of them names a mode. The type
    // system enforces the second half: there is no named mode to produce.
    for raw in [-1, 0, 1, 2, 3, 99] {
        let text = format!("SET FULLSCREEN \"{raw}\"\n");
        assert_eq!(parse_user_settings(&text).window_mode_raw, Some(raw));
    }
}

#[test]
fn parsing_never_panics_on_adversarial_input() {
    let cases = [
        "SET \"\" \"\"".to_string(),
        "SET FULLSCREEN \"\"".to_string(),
        format!("SET FullscreenWidth \"{}\"", "9".repeat(400)),
        format!("SET {} \"1\"", ".".repeat(50)),
        "SET FullscreenWidth.\"1920\"".to_string(),
        "SET FullscreenWidth. \"1920\"".to_string(),
        "\r\nSET ACTIVE_DISPLAY \"1\"\r\n".to_string(),
    ];
    for text in cases {
        let _ = parse_user_settings(&text);
    }
    // A carriage return must not become part of the value.
    assert_eq!(
        parse_user_settings("SET ACTIVE_DISPLAY \"1\"\r\n").active_display,
        Some(1)
    );
}

// ---------------------------------------------------------------------------
// Descriptor construction
// ---------------------------------------------------------------------------

fn measured(w: u32, h: u32) -> MeasuredDisplay {
    MeasuredDisplay {
        surface: Size::new(w, h),
        surface_origin: Point::new(0, 0),
        display_origin: Some(Point::new(0, 0)),
        display_size: Some(Size::new(3840, 2160)),
        dpi: Some(144),
    }
}

#[test]
fn a_zero_surface_never_becomes_a_descriptor() {
    for (w, h) in [(0, 1080), (1920, 0), (0, 0)] {
        let m = MeasuredDisplay {
            surface: Size::new(w, h),
            ..measured(1, 1)
        };
        assert_eq!(DisplayDescriptor::from_measured(m), None, "{w}x{h}");
    }
}

#[test]
fn a_measured_descriptor_carries_every_supplied_field() {
    let d = DisplayDescriptor::from_measured(measured(2560, 1440)).expect("descriptor");
    assert_eq!(d.surface, Size::new(2560, 1440));
    assert_eq!(d.surface_origin, Some(Point::new(0, 0)));
    assert_eq!(d.display_origin, Some(Point::new(0, 0)));
    assert_eq!(d.display_size, Some(Size::new(3840, 2160)));
    assert_eq!(d.dpi, Some(144));
    assert_eq!(d.source, DisplaySource::Measured);
}

#[test]
fn absent_probe_fields_stay_absent_and_are_never_defaulted() {
    // A fabricated scale of 1.0 is indistinguishable from a genuinely unscaled
    // display, which is the confident wrong answer this rule exists to prevent.
    let m = MeasuredDisplay {
        surface: Size::new(1024, 768),
        surface_origin: Point::new(10, 20),
        display_origin: None,
        display_size: None,
        dpi: None,
    };
    let d = DisplayDescriptor::from_measured(m).expect("descriptor");
    assert_eq!(d.surface_origin, Some(Point::new(10, 20)));
    assert_eq!(d.display_origin, None);
    assert_eq!(d.display_size, None);
    assert_eq!(d.dpi, None);
    assert_eq!(d.scale(), None);
}

#[test]
fn scale_is_computed_from_dpi_and_never_stored() {
    let at = |dpi: u32| {
        DisplayDescriptor::from_measured(MeasuredDisplay {
            dpi: Some(dpi),
            ..measured(800, 600)
        })
        .expect("descriptor")
        .scale()
    };
    assert_eq!(at(96), Some(1.0));
    assert_eq!(at(144), Some(1.5));
    assert_eq!(at(192), Some(2.0));
}

#[test]
fn a_configured_descriptor_is_produced_only_when_both_stored_pairs_agree() {
    let pair = Some(Size::new(1920, 1080));
    let agree = StoredVideoSettings {
        fullscreen: pair,
        windowed: pair,
        ..Default::default()
    };
    let d = DisplayDescriptor::from_stored(&agree).expect("configured descriptor");
    assert_eq!(d.surface, Size::new(1920, 1080));
    assert_eq!(d.source, DisplaySource::Configured);

    let differ = StoredVideoSettings {
        fullscreen: Some(Size::new(3440, 1440)),
        windowed: Some(Size::new(5160, 2160)),
        ..Default::default()
    };
    assert_eq!(DisplayDescriptor::from_stored(&differ), None);

    let only_one = StoredVideoSettings {
        fullscreen: pair,
        ..Default::default()
    };
    assert_eq!(DisplayDescriptor::from_stored(&only_one), None);

    assert_eq!(
        DisplayDescriptor::from_stored(&StoredVideoSettings::default()),
        None
    );

    let zero = Some(Size::new(0, 0));
    let both_zero = StoredVideoSettings {
        fullscreen: zero,
        windowed: zero,
        ..Default::default()
    };
    assert_eq!(DisplayDescriptor::from_stored(&both_zero), None);
}

#[test]
fn a_configured_descriptor_carries_no_display_geometry() {
    // The settings file records a display index, and an index is not geometry.
    let pair = Some(Size::new(1920, 1080));
    let stored = StoredVideoSettings {
        fullscreen: pair,
        windowed: pair,
        active_display: Some(1),
        ..Default::default()
    };
    let d = DisplayDescriptor::from_stored(&stored).expect("configured descriptor");
    assert_eq!(d.surface_origin, None);
    assert_eq!(d.display_origin, None);
    assert_eq!(d.display_size, None);
    assert_eq!(d.dpi, None);
    assert_eq!(d.scale(), None);
}

// ---------------------------------------------------------------------------
// Reconciliation
// ---------------------------------------------------------------------------

#[test]
fn reconciliation_reports_no_stored_settings() {
    assert_eq!(
        reconcile(Size::new(1920, 1080), None),
        Reconciliation::NoStored
    );
}

#[test]
fn reconciliation_reports_settings_without_pairs() {
    let stored = StoredVideoSettings {
        active_display: Some(0),
        ..Default::default()
    };
    assert_eq!(
        reconcile(Size::new(1920, 1080), Some(&stored)),
        Reconciliation::NoPairs
    );
}

#[test]
fn reconciliation_records_the_pair_and_the_raw_mode_when_exactly_one_matches() {
    // This is the observation that accumulates evidence about the unmapped mode
    // value from ordinary use, without anyone performing a procedure.
    let stored = StoredVideoSettings {
        fullscreen: Some(Size::new(3440, 1440)),
        windowed: Some(Size::new(5160, 2160)),
        window_mode_raw: Some(2),
        ..Default::default()
    };
    assert_eq!(
        reconcile(Size::new(3440, 1440), Some(&stored)),
        Reconciliation::Agreed {
            pair: StoredPair::Fullscreen,
            mode_raw: Some(2),
        }
    );
    assert_eq!(
        reconcile(Size::new(5160, 2160), Some(&stored)),
        Reconciliation::Agreed {
            pair: StoredPair::Windowed,
            mode_raw: Some(2),
        }
    );
}

#[test]
fn reconciliation_is_ambiguous_when_both_pairs_match() {
    let pair = Some(Size::new(1920, 1080));
    let stored = StoredVideoSettings {
        fullscreen: pair,
        windowed: pair,
        window_mode_raw: Some(1),
        ..Default::default()
    };
    assert_eq!(
        reconcile(Size::new(1920, 1080), Some(&stored)),
        Reconciliation::Ambiguous
    );
}

#[test]
fn reconciliation_reports_disagreement_without_changing_anything() {
    let stored = StoredVideoSettings {
        fullscreen: Some(Size::new(3440, 1440)),
        windowed: Some(Size::new(5160, 2160)),
        ..Default::default()
    };
    assert_eq!(
        reconcile(Size::new(1280, 720), Some(&stored)),
        Reconciliation::Disagreed {
            measured: Size::new(1280, 720),
        }
    );
}

#[test]
fn a_single_present_pair_still_reconciles() {
    let stored = StoredVideoSettings {
        fullscreen: Some(Size::new(3440, 1440)),
        window_mode_raw: Some(2),
        ..Default::default()
    };
    assert_eq!(
        reconcile(Size::new(3440, 1440), Some(&stored)),
        Reconciliation::Agreed {
            pair: StoredPair::Fullscreen,
            mode_raw: Some(2),
        }
    );
    assert_eq!(
        reconcile(Size::new(800, 600), Some(&stored)),
        Reconciliation::Disagreed {
            measured: Size::new(800, 600),
        }
    );
}

// ---------------------------------------------------------------------------
// The detector: change detection, read gating, loss and recovery
// ---------------------------------------------------------------------------

/// Counts how many times the settings-read closure was invoked, which is how the
/// "read only when the measurement changed" guarantee is proven rather than
/// asserted.
#[derive(Default)]
struct ReadCounter {
    reads: std::cell::Cell<usize>,
    settings: Option<StoredVideoSettings>,
}

impl ReadCounter {
    fn with(settings: StoredVideoSettings) -> Self {
        Self {
            reads: std::cell::Cell::new(0),
            settings: Some(settings),
        }
    }

    fn read(&self) -> Option<StoredVideoSettings> {
        self.reads.set(self.reads.get() + 1);
        self.settings.clone()
    }

    fn count(&self) -> usize {
        self.reads.get()
    }
}

#[test]
fn a_first_measurement_produces_one_update() {
    let mut d = DisplayDetector::new();
    let c = ReadCounter::default();
    let update = d.update(Some(measured(1920, 1080)), || c.read());
    let update = update.expect("first measurement is a change");
    assert_eq!(
        update.descriptor.expect("descriptor").surface,
        Size::new(1920, 1080)
    );
    assert_eq!(d.current().expect("current").surface, Size::new(1920, 1080));
}

#[test]
fn an_identical_measurement_produces_no_update_and_no_read() {
    let mut d = DisplayDetector::new();
    let c = ReadCounter::default();
    assert!(d.update(Some(measured(1920, 1080)), || c.read()).is_some());
    let after_first = c.count();
    for _ in 0..50 {
        assert!(
            d.update(Some(measured(1920, 1080)), || c.read()).is_none(),
            "an unchanged measurement is not a change"
        );
    }
    assert_eq!(
        c.count(),
        after_first,
        "a stationary window must read no files at all"
    );
}

#[test]
fn every_field_of_the_measurement_participates_in_change_detection() {
    let base = measured(1920, 1080);
    let variants = [
        MeasuredDisplay {
            surface: Size::new(1920, 1081),
            ..base
        },
        MeasuredDisplay {
            surface_origin: Point::new(1, 0),
            ..base
        },
        MeasuredDisplay {
            display_origin: Some(Point::new(-1920, 0)),
            ..base
        },
        MeasuredDisplay {
            display_size: Some(Size::new(1920, 1080)),
            ..base
        },
        MeasuredDisplay {
            dpi: Some(96),
            ..base
        },
    ];
    for variant in variants {
        let mut d = DisplayDetector::new();
        let c = ReadCounter::default();
        assert!(d.update(Some(base), || c.read()).is_some());
        assert!(
            d.update(Some(variant), || c.read()).is_some(),
            "a changed field must be a change: {variant:?}"
        );
    }
}

#[test]
fn a_changed_measurement_reads_the_settings_exactly_once() {
    let mut d = DisplayDetector::new();
    let c = ReadCounter::default();
    d.update(Some(measured(1920, 1080)), || c.read());
    assert_eq!(c.count(), 1);
    d.update(Some(measured(2560, 1440)), || c.read());
    assert_eq!(c.count(), 2);
    d.update(Some(measured(2560, 1440)), || c.read());
    assert_eq!(c.count(), 2, "the repeat must not read");
}

#[test]
fn a_lost_measurement_clears_the_descriptor_and_reads_nothing() {
    let mut d = DisplayDetector::new();
    let c = ReadCounter::default();
    d.update(Some(measured(1920, 1080)), || c.read());
    let reads_before = c.count();

    let update = d
        .update(None, || c.read())
        .expect("losing the window is a change");
    assert_eq!(update.descriptor, None);
    assert_eq!(d.current(), None);
    assert_eq!(
        c.count(),
        reads_before,
        "losing the window is not a reason to consult a file"
    );

    assert!(
        d.update(None, || c.read()).is_none(),
        "staying lost is not a further change"
    );
}

#[test]
fn a_recovered_measurement_re_resolves_without_a_restart() {
    let mut d = DisplayDetector::new();
    let c = ReadCounter::default();
    d.update(Some(measured(1920, 1080)), || c.read());
    d.update(None, || c.read());
    let update = d
        .update(Some(measured(2560, 1440)), || c.read())
        .expect("recovery is a change");
    assert_eq!(
        update.descriptor.expect("descriptor").surface,
        Size::new(2560, 1440)
    );
}

#[test]
fn an_undrawable_window_never_yields_a_zero_descriptor() {
    let mut d = DisplayDetector::new();
    let c = ReadCounter::default();
    d.update(Some(measured(1920, 1080)), || c.read());
    // A probe that returned a zero surface would have been rejected before this
    // point, but the detector must also refuse it rather than publishing it.
    let update = d
        .update(
            Some(MeasuredDisplay {
                surface: Size::new(0, 0),
                ..measured(1, 1)
            }),
            || c.read(),
        )
        .expect("becoming undrawable is a change");
    assert_eq!(update.descriptor, None);
    assert_eq!(d.current(), None);
}

#[test]
fn with_no_measurement_ever_the_settings_are_read_exactly_once() {
    // The pre-launch case must be reachable, and must not re-read the file on
    // every sampling cycle while the game stays closed.
    let pair = Some(Size::new(1920, 1080));
    let c = ReadCounter::with(StoredVideoSettings {
        fullscreen: pair,
        windowed: pair,
        ..Default::default()
    });
    let mut d = DisplayDetector::new();

    let update = d
        .update(None, || c.read())
        .expect("a configured descriptor is a change");
    let descriptor = update.descriptor.expect("descriptor");
    assert_eq!(descriptor.source, DisplaySource::Configured);
    assert_eq!(descriptor.surface, Size::new(1920, 1080));
    assert_eq!(c.count(), 1);

    for _ in 0..20 {
        assert!(d.update(None, || c.read()).is_none());
    }
    assert_eq!(
        c.count(),
        1,
        "the pre-launch read happens once, not per cycle"
    );
}

#[test]
fn with_no_measurement_and_ambiguous_settings_no_descriptor_is_produced() {
    let c = ReadCounter::with(StoredVideoSettings {
        fullscreen: Some(Size::new(3440, 1440)),
        windowed: Some(Size::new(5160, 2160)),
        ..Default::default()
    });
    let mut d = DisplayDetector::new();
    assert!(
        d.update(None, || c.read()).is_none(),
        "which pair is live cannot be known without guessing the mode"
    );
    assert_eq!(d.current(), None);
    assert_eq!(c.count(), 1);
    for _ in 0..20 {
        assert!(d.update(None, || c.read()).is_none());
    }
    assert_eq!(
        c.count(),
        1,
        "the failed attempt is not retried every cycle"
    );
}

#[test]
fn a_stored_reading_never_alters_a_measured_descriptor() {
    let m = measured(3440, 1440);
    let expected = DisplayDescriptor::from_measured(m).expect("descriptor");

    let agreeing = StoredVideoSettings {
        fullscreen: Some(Size::new(3440, 1440)),
        windowed: Some(Size::new(5160, 2160)),
        window_mode_raw: Some(2),
        ..Default::default()
    };
    let disagreeing = StoredVideoSettings {
        fullscreen: Some(Size::new(1280, 720)),
        windowed: Some(Size::new(640, 480)),
        ..Default::default()
    };
    let partial = StoredVideoSettings {
        active_display: Some(1),
        ..Default::default()
    };

    for stored in [Some(agreeing), Some(disagreeing), Some(partial), None] {
        let mut d = DisplayDetector::new();
        let update = d
            .update(Some(m), || stored.clone())
            .expect("first measurement is a change");
        assert_eq!(
            update.descriptor,
            Some(expected),
            "the measurement is authoritative regardless of what the file says"
        );
        assert_eq!(d.current(), Some(&expected));
    }
}

#[test]
fn the_reconciliation_outcome_accompanies_a_measured_change() {
    let stored = StoredVideoSettings {
        fullscreen: Some(Size::new(3440, 1440)),
        windowed: Some(Size::new(5160, 2160)),
        window_mode_raw: Some(2),
        ..Default::default()
    };
    let mut d = DisplayDetector::new();
    let update = d
        .update(Some(measured(3440, 1440)), || Some(stored.clone()))
        .expect("change");
    assert_eq!(
        update.reconciliation,
        Some(Reconciliation::Agreed {
            pair: StoredPair::Fullscreen,
            mode_raw: Some(2),
        })
    );
}

#[test]
fn losing_the_window_carries_no_reconciliation() {
    let mut d = DisplayDetector::new();
    d.update(Some(measured(1920, 1080)), || None);
    let update = d.update(None, || None).expect("change");
    assert_eq!(update.reconciliation, None);
}

// ---------------------------------------------------------------------------
// The seam, and the promise that detection writes nothing
// ---------------------------------------------------------------------------

#[test]
fn the_seam_defaults_to_no_measurement_and_the_mock_opts_in() {
    use eso_weave::pixelbus::{MockSampler, Rgb, SurfaceSampler};

    /// A sampler that does not override `display`, standing in for any backend
    /// that cannot answer the question.
    struct Silent;
    impl SurfaceSampler for Silent {
        fn sample(&self, _x: u32, _y: u32) -> Option<Rgb> {
            None
        }
    }
    assert_eq!(Silent.display(), None);

    let mut mock = MockSampler::new();
    assert_eq!(mock.display(), None);
    mock.set_display(Some(measured(1920, 1080)));
    assert_eq!(mock.display(), Some(measured(1920, 1080)));
    mock.set_display(None);
    assert_eq!(mock.display(), None);
}

#[test]
fn detection_writes_nothing_to_disk() {
    // The constitution treats this tree as safety-critical. Detection reads one
    // file there and must leave everything, present or absent, exactly as it
    // found it.
    let entries = |dir: &std::path::Path| {
        let mut names: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        names.sort();
        names
    };

    // A directory holding a settings file.
    let populated = tempfile::tempdir().expect("temp dir");
    let addons = populated.path().join("AddOns");
    std::fs::create_dir(&addons).expect("addons dir");
    let settings_path = eso_weave::beacon::user_settings_path(&addons).expect("path");
    std::fs::write(&settings_path, REALISTIC).expect("write settings");
    let before_entries = entries(populated.path());
    let before_bytes = std::fs::read(&settings_path).expect("read back");

    // A directory holding nothing at all, which is the ordinary case for an
    // install the application has never seen.
    let empty = tempfile::tempdir().expect("temp dir");
    let empty_addons = empty.path().join("AddOns");
    let empty_settings = eso_weave::beacon::user_settings_path(&empty_addons).expect("path");

    for path in [settings_path.clone(), empty_settings.clone()] {
        let mut detector = DisplayDetector::new();
        for _ in 0..5 {
            detector.update(Some(measured(1920, 1080)), || {
                std::fs::read_to_string(&path)
                    .ok()
                    .map(|t| parse_user_settings(&t))
            });
            detector.update(Some(measured(2560, 1440)), || {
                std::fs::read_to_string(&path)
                    .ok()
                    .map(|t| parse_user_settings(&t))
            });
            detector.update(None, || {
                std::fs::read_to_string(&path)
                    .ok()
                    .map(|t| parse_user_settings(&t))
            });
        }
    }

    assert_eq!(entries(populated.path()), before_entries);
    assert_eq!(
        std::fs::read(&settings_path).expect("read back"),
        before_bytes
    );
    assert!(!empty_settings.exists(), "an absent file must stay absent");
    assert!(
        !empty_addons.exists(),
        "an absent directory must stay absent"
    );
    assert!(entries(empty.path()).is_empty(), "nothing may appear");
}
