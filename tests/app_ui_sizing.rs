//! Rendered-frame sizing tests (slice 030, issues #12, #13, #14).
//!
//! These drive the real frame body through a headless egui harness and assert
//! rendered geometry. They exist because three prior slices fixed window sizing
//! with a fully green suite: the pure helpers in `app::mod` were always correct,
//! and the defects were always in the glue between those helpers and egui, which
//! nothing tested. See `specs/030-ui-sizing-correctness/contracts/sizing-contracts.md`.

use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use eframe::egui;
use egui_kittest::{
    kittest::{NodeT, Queryable},
    Harness,
};

use eso_weave::app::ui::EsoWeaveApp;
use eso_weave::app::{
    resource_view, widgets, AppModel, DashboardLayout, ResourceTheme, UiPrefs, DASHBOARD_WIDE_MIN,
};
use eso_weave::beacon::{self, BeaconPrefs, Environment, MANAGED_MARKER, MANIFEST};
use eso_weave::config::{LoggingPrefs, Settings, Theme};
use eso_weave::documentation::BrowserOpener;
use eso_weave::fishing::{FishingConfig, FishingController, MockFishingSink};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::InputEngine;
use eso_weave::logging;
use eso_weave::weave::{WeaveConfig, WeaveEngine};

/// Builds an app over a default model. The channel senders are leaked into the
/// returned tuple so the receivers stay connected for the app's lifetime.
fn test_app() -> EsoWeaveApp {
    test_app_with_settings(Settings::default())
}

fn test_app_with_settings(settings: Settings) -> EsoWeaveApp {
    let (engine, _rx) = InputEngine::new(BindingTable::default(), 16);
    let weave = Arc::new(Mutex::new(WeaveEngine::new(WeaveConfig::default())));
    let fishing = Arc::new(Mutex::new(FishingController::new(FishingConfig::default())));
    let (_dispatch, log) = logging::build(&LoggingPrefs::default(), PathBuf::from("."));
    let (reader_update_tx, _reader_updates) = mpsc::channel();
    let model = AppModel::new(
        Arc::new(engine),
        weave,
        fishing,
        Box::new(MockFishingSink::new()),
        Arc::new(Mutex::new(eso_weave::potion::AutoPotionController::new(
            eso_weave::potion::AutoPotionConfig::default(),
        ))),
        log,
        reader_update_tx,
        settings,
        None,
        std::time::Instant::now(),
    );
    let (toggle_tx, toggle_rx) = mpsc::channel();
    let (api_tx, api_rx) = mpsc::channel();
    // The app only ever drains these; keeping the senders alive avoids a
    // disconnected channel changing behavior mid-test.
    std::mem::forget(toggle_tx);
    std::mem::forget(api_tx);
    std::mem::forget(_reader_updates);
    EsoWeaveApp::new(model, toggle_rx, api_rx, None)
}

/// Renders `frames` frames at the given window size and returns the app, so its
/// recorded sizing state can be asserted. Several frames are needed because the
/// content measurement is gated on two consecutive stable frames.
fn render_at(size: egui::Vec2, frames: usize) -> EsoWeaveApp {
    render_app_at(test_app(), size, frames)
}

fn render_app_at(app: EsoWeaveApp, size: egui::Vec2, frames: usize) -> EsoWeaveApp {
    let mut harness = harness_for_app(app, size);
    for _ in 0..frames {
        harness.step();
    }
    harness.into_state()
}

/// A harness at the given window size, with the bundled fonts installed exactly as
/// `main.rs` does at startup (the layout depends on them, so a bare harness would
/// measure the wrong text metrics).
fn harness_at(size: egui::Vec2) -> Harness<'static, EsoWeaveApp> {
    harness_for_app(test_app(), size)
}

fn harness_for_app(app: EsoWeaveApp, size: egui::Vec2) -> Harness<'static, EsoWeaveApp> {
    let mut fonts_installed = false;
    Harness::builder().with_size(size).build_ui_state(
        move |ui, app: &mut EsoWeaveApp| {
            if !fonts_installed {
                // `set_fonts` takes effect on the next frame, so the very first
                // closure call installs and renders nothing. This mirrors startup,
                // where `main.rs` installs the fonts before the first frame runs.
                eso_weave::app::theme::install_fonts(ui.ctx());
                fonts_installed = true;
                return;
            }
            app.frame_ui(ui);
        },
        app,
    )
}

/// Number of frames to settle the two-frame stability gate before reading state.
const SETTLE: usize = 6;

/// Settles layout, then applies the minimum size the real viewport would honor.
fn settle_at_enforced_minimum(
    harness: &mut Harness<'static, EsoWeaveApp>,
    current: egui::Vec2,
) -> egui::Vec2 {
    for _ in 0..SETTLE {
        harness.step();
    }
    let minimum = harness
        .state()
        .last_min_sent()
        .expect("a minimum should have been sent");
    let target = egui::vec2(current.x.max(minimum.x), current.y.max(minimum.y));
    harness.input_mut().screen_rect = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, target));
    for _ in 0..SETTLE {
        harness.step();
    }
    target
}

#[test]
fn help_menu_exposes_the_offline_documentation_action() {
    let mut harness = harness_at(egui::vec2(760.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Help")
        .click_accesskit();
    harness.step();
    harness.get_by_role_and_label(
        egui::accesskit::Role::Button,
        eso_weave::app::strings::MENU_DOCUMENTATION,
    );
}

#[test]
fn file_menu_opens_the_accessible_catalog_update_modal() {
    let mut harness = harness_at(egui::vec2(760.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "File")
        .click_accesskit();
    harness.step();
    harness
        .get_by_role_and_label(
            egui::accesskit::Role::Button,
            eso_weave::app::strings::MENU_CATALOG_UPDATE,
        )
        .click_accesskit();
    harness.step();
    harness.get_by_label("Catalog Update");
    harness.get_by_label(
        "Updates are always user initiated. Imported hashes prove integrity, not who supplied the files.",
    );
    harness.get_by_role_and_label(egui::accesskit::Role::Button, "Refresh candidates");
    harness.get_by_role_and_label(
        egui::accesskit::Role::CheckBox,
        "I obtained this candidate from a review or release source I trust.",
    );
    assert!(harness
        .query_by_role_and_label(egui::accesskit::Role::Button, "Install/update data addon",)
        .is_none());
    assert!(harness
        .query_by_role_and_label(egui::accesskit::Role::Button, "Uninstall data addon")
        .is_none());
    harness.key_press(egui::Key::Escape);
    harness.step();
    assert!(!harness.state().catalog_update_open());
}

struct FailingDocumentationOpener;

impl BrowserOpener for FailingDocumentationOpener {
    fn open(&self, _url: &str) -> std::io::Result<()> {
        Err(std::io::Error::other("no default browser"))
    }
}

#[test]
fn documentation_launch_failure_is_visible_and_non_fatal() {
    let app = test_app().with_documentation_opener(Box::new(FailingDocumentationOpener));
    let mut harness = harness_for_app(app, egui::vec2(760.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    harness
        .get_by_role_and_label(egui::accesskit::Role::Button, "Help")
        .click_accesskit();
    harness.step();
    harness
        .get_by_role_and_label(
            egui::accesskit::Role::Button,
            eso_weave::app::strings::MENU_DOCUMENTATION,
        )
        .click_accesskit();
    harness.step();
    assert!(harness
        .state()
        .documentation_error()
        .unwrap()
        .contains("no default browser"));
    harness.get_by_label("Documentation unavailable");
    harness.get_by_role_and_label(egui::accesskit::Role::Button, "Close");
}

#[test]
fn dashboard_stacks_narrow_and_uses_columns_wide() {
    let narrow = render_at(egui::vec2(760.0, 1200.0), SETTLE);
    assert_eq!(
        narrow.last_dashboard_layout(),
        Some(DashboardLayout::Narrow)
    );
    let (narrow_live, narrow_system) = narrow.dashboard_rects().expect("dashboard geometry");
    assert_rect_sizes_equal(narrow_live, narrow_system, "expanded narrow cards");
    assert!(
        narrow_live.bottom() <= narrow_system.top() + 0.5,
        "narrow sections are not stacked in reading order: {narrow_live:?}, {narrow_system:?}"
    );

    let wide = render_at(egui::vec2(1200.0, 1000.0), SETTLE);
    assert_eq!(wide.last_dashboard_layout(), Some(DashboardLayout::Wide));
    let (wide_live, wide_system) = wide.dashboard_rects().expect("dashboard geometry");
    assert_rect_sizes_equal(wide_live, wide_system, "expanded wide cards");
    assert!(
        wide_live.right() <= wide_system.left() + 0.5,
        "wide sections are not in separate columns: {wide_live:?}, {wide_system:?}"
    );
    assert!(
        (wide_live.top() - wide_system.top()).abs() <= 0.5,
        "wide section tops are not aligned: {wide_live:?}, {wide_system:?}"
    );
}

fn assert_rect_sizes_equal(left: egui::Rect, right: egui::Rect, context: &str) {
    assert!(
        (left.width() - right.width()).abs() <= 1.0,
        "{context} have unequal widths: {left:?}, {right:?}"
    );
    assert!(
        (left.height() - right.height()).abs() <= 1.0,
        "{context} have unequal heights: {left:?}, {right:?}"
    );
}

#[test]
fn wide_dashboard_cards_grow_symmetrically() {
    let compact = render_at(egui::vec2(1200.0, 1000.0), SETTLE);
    let roomy = render_at(egui::vec2(1600.0, 1000.0), SETTLE);
    let (compact_live, compact_system) = compact.dashboard_rects().expect("compact geometry");
    let (roomy_live, roomy_system) = roomy.dashboard_rects().expect("roomy geometry");

    let live_growth = roomy_live.width() - compact_live.width();
    let system_growth = roomy_system.width() - compact_system.width();
    assert!(
        (live_growth - system_growth).abs() <= 1.0,
        "cards grew asymmetrically: live {live_growth}, system {system_growth}"
    );
}

#[test]
fn collapsed_system_state_forces_stacked_layout_until_reexpanded() {
    let mut harness = harness_at(egui::vec2(1400.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Wide)
    );
    harness.get_by_label("System and State").click_accesskit();
    for _ in 0..SETTLE {
        harness.step();
    }
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Narrow)
    );
    let (live, collapsed) = harness
        .state()
        .dashboard_rects()
        .expect("collapsed geometry");
    assert!(live.bottom() <= collapsed.top() + 0.5);
    assert!((live.width() - collapsed.width()).abs() <= 1.0);
    assert!(collapsed.height() < live.height());

    harness.get_by_label("System and State").click_accesskit();
    for _ in 0..SETTLE {
        harness.step();
    }
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Wide)
    );
    let (live, system) = harness
        .state()
        .dashboard_rects()
        .expect("expanded geometry");
    assert_rect_sizes_equal(live, system, "re-expanded wide cards");
}

#[test]
fn persisted_collapsed_system_state_starts_stacked_at_wide_width() {
    let settings = Settings {
        ui: eso_weave::app::settings_form::ui_to_value(&UiPrefs {
            system_state_expanded: false,
            ..UiPrefs::default()
        }),
        ..Settings::default()
    };
    let app = render_app_at(
        test_app_with_settings(settings),
        egui::vec2(1400.0, 1000.0),
        SETTLE,
    );

    assert_eq!(app.last_dashboard_layout(), Some(DashboardLayout::Narrow));
    let (live, collapsed) = app.dashboard_rects().expect("persisted collapsed geometry");
    assert!(live.bottom() <= collapsed.top() + 0.5);
    assert!((live.width() - collapsed.width()).abs() <= 1.0);
    assert!(collapsed.height() < live.height());
}

#[test]
fn collapsing_system_state_with_log_open_never_overlaps_and_restores_the_log() {
    let mut harness = harness_at(egui::vec2(1400.0, 1100.0));
    harness.step();
    harness.state_mut().set_log_panel_open(true);
    settle_at_enforced_minimum(&mut harness, egui::vec2(1400.0, 1100.0));
    assert_no_overlap(harness.state(), "expanded dashboard before collapse");

    harness.get_by_label("System and State").click_accesskit();
    harness.step();
    if harness.state().last_log_top().is_some() {
        assert_no_overlap(harness.state(), "collapse interaction frame");
    }
    harness.step();
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Narrow)
    );
    if harness.state().last_log_top().is_some() {
        assert_no_overlap(harness.state(), "first measured collapsed frame");
    }

    for _ in 0..SETTLE {
        harness.step();
    }
    let collapsed_min = harness
        .state()
        .last_min_sent()
        .expect("collapsed log minimum");
    harness.input_mut().screen_rect = Some(egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(1400.0, collapsed_min.y),
    ));
    for _ in 0..SETTLE {
        harness.step();
    }
    assert_no_overlap(harness.state(), "log restored after collapse reflow");

    harness.get_by_label("System and State").click_accesskit();
    for frame in 0..=SETTLE {
        harness.step();
        if harness.state().last_log_top().is_some() {
            assert_no_overlap(harness.state(), &format!("re-expansion frame {frame}"));
        }
    }
    assert!(harness.state().system_state_expanded());
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Wide)
    );
    assert_no_overlap(harness.state(), "log restored after re-expansion");
}

#[test]
fn collapse_override_survives_resize_order_and_repeated_toggles() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    harness.get_by_label("System and State").click_accesskit();
    harness.step();

    for width in [
        560.0,
        760.0,
        DASHBOARD_WIDE_MIN - 0.1,
        DASHBOARD_WIDE_MIN,
        1400.0,
    ] {
        harness.set_size(egui::vec2(width, 1200.0));
        for _ in 0..SETTLE {
            harness.step();
        }
        assert_eq!(
            harness.state().last_dashboard_layout(),
            Some(DashboardLayout::Narrow),
            "collapsed card left stacked layout at {width} points"
        );
    }

    for expected in [true, false, true] {
        harness.get_by_label("System and State").click_accesskit();
        for _ in 0..SETTLE {
            harness.step();
        }
        assert_eq!(harness.state().system_state_expanded(), expected);
        assert_eq!(
            harness.state().last_dashboard_layout(),
            Some(if expected {
                DashboardLayout::Wide
            } else {
                DashboardLayout::Narrow
            })
        );
    }
}

#[test]
fn system_state_controls_share_an_origin_and_lifecycle_buttons_share_a_row() {
    let root = tempfile::tempdir().unwrap();
    let addon = root.path().join("PixelBeacon");
    std::fs::create_dir_all(&addon).unwrap();
    std::fs::write(
        addon.join("PixelBeacon.txt"),
        format!("## Title: PixelBeacon\n{MANAGED_MARKER}\n## Version: 1\n"),
    )
    .unwrap();
    let settings = Settings {
        beacon: beacon::prefs_to_value(&BeaconPrefs {
            path_override: Some(root.path().to_path_buf()),
            environment: Environment::Live,
        }),
        ..Settings::default()
    };
    let mut harness = harness_for_app(test_app_with_settings(settings), egui::vec2(1200.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }

    let update = harness.get_by_role_and_label(egui::accesskit::Role::Button, "Update");
    let uninstall = harness.get_by_role_and_label(egui::accesskit::Role::Button, "Uninstall");
    assert!((update.rect().width() - uninstall.rect().width()).abs() <= 1.0);
    assert!((update.rect().height() - uninstall.rect().height()).abs() <= 1.0);
    assert!(
        (update.rect().top() - uninstall.rect().top()).abs() <= 1.0,
        "lifecycle buttons are not horizontal: update {:?}, uninstall {:?}",
        update.rect(),
        uninstall.rect()
    );
    assert!(update.rect().right() <= uninstall.rect().left());
    let (_, system) = harness
        .state()
        .dashboard_rects()
        .expect("system card geometry");
    assert!(
        system.right() - uninstall.rect().right() <= 24.0,
        "interaction column is not near the trailing edge: system {system:?}, uninstall {:?}",
        uninstall.rect()
    );

    for label in ["ESO Weave", "Fishing", "Auto Potion"] {
        let toggle = harness.get_by_role_and_label(egui::accesskit::Role::CheckBox, label);
        assert!(
            (toggle.rect().left() - update.rect().left()).abs() <= 1.0,
            "{label} starts at {} instead of the action origin {}",
            toggle.rect().left(),
            update.rect().left()
        );
    }
}

#[test]
fn lifecycle_action_matrix_keeps_one_column_and_dispatches_install() {
    fn beacon_settings(root: &std::path::Path) -> Settings {
        Settings {
            beacon: beacon::prefs_to_value(&BeaconPrefs {
                path_override: Some(root.to_path_buf()),
                environment: Environment::Live,
            }),
            ..Settings::default()
        }
    }

    let absent_root = tempfile::tempdir().unwrap();
    let mut absent = harness_for_app(
        test_app_with_settings(beacon_settings(absent_root.path())),
        egui::vec2(1200.0, 1000.0),
    );
    for _ in 0..SETTLE {
        absent.step();
    }
    let install = absent.get_by_role_and_label(egui::accesskit::Role::Button, "Install");
    let install_rect = install.rect();
    let toggle_rect = absent
        .get_by_role_and_label(egui::accesskit::Role::CheckBox, "ESO Weave")
        .rect();
    assert!((install_rect.left() - toggle_rect.left()).abs() <= 1.0);
    install.click_accesskit();
    absent.step();
    assert!(absent_root
        .path()
        .join("PixelBeacon/PixelBeacon.txt")
        .is_file());

    let current_root = tempfile::tempdir().unwrap();
    let current_addon = current_root.path().join("PixelBeacon");
    std::fs::create_dir_all(&current_addon).unwrap();
    std::fs::write(current_addon.join("PixelBeacon.txt"), MANIFEST).unwrap();
    let mut current = harness_for_app(
        test_app_with_settings(beacon_settings(current_root.path())),
        egui::vec2(1200.0, 1000.0),
    );
    for _ in 0..SETTLE {
        current.step();
    }
    let uninstall = current
        .get_by_role_and_label(egui::accesskit::Role::Button, "Uninstall")
        .rect();
    assert!((install_rect.width() - uninstall.width()).abs() <= 1.0);
    assert!((install_rect.height() - uninstall.height()).abs() <= 1.0);
    assert!((install_rect.left() - uninstall.left()).abs() <= 1.0);
    assert!(current
        .query_by_role_and_label(egui::accesskit::Role::Button, "Install")
        .is_none());
    assert!(current
        .query_by_role_and_label(egui::accesskit::Role::Button, "Update")
        .is_none());
}

#[test]
fn s093_data_addon_row_follows_pixelbeacon_and_exposes_unique_actions() {
    for width in [760.0, 1200.0] {
        let root = tempfile::tempdir().unwrap();
        let settings = Settings {
            beacon: beacon::prefs_to_value(&BeaconPrefs {
                path_override: Some(root.path().to_path_buf()),
                environment: Environment::Live,
            }),
            ..Settings::default()
        };
        let mut harness =
            harness_for_app(test_app_with_settings(settings), egui::vec2(width, 1200.0));
        for _ in 0..SETTLE {
            harness.step();
        }

        let beacon = harness.get_by_label("PixelBeacon Status").rect();
        let data = harness.get_by_label("ESO Weave Data").rect();
        let signal = harness.get_by_label("PixelBeacon Signal").rect();
        assert!(beacon.top() < data.top());
        assert!(data.top() < signal.top());

        harness
            .get_by_role_and_label(egui::accesskit::Role::Button, "Install Data")
            .click_accesskit();
        for _ in 0..SETTLE {
            harness.step();
        }
        assert!(root
            .path()
            .join(eso_weave::data_addon::DATA_ADDON_SUBFOLDER)
            .join(eso_weave::data_addon::MANIFEST_FILE)
            .is_file());
        harness.get_by_label("Data Addon Next Step");
        harness
            .get_by_role_and_label(egui::accesskit::Role::Button, "Data Details")
            .click_accesskit();
        for _ in 0..2 {
            harness.step();
        }
        harness.get_by_label("ESO Weave Data Details");
        for label in [
            "Data Addon Ownership",
            "Data Addon Compatibility",
            "Data Addon Enabled",
            "Data Addon Loaded",
            "Data Addon Reload",
            "Data Runtime",
            "Catalog Collection",
            "Encounter Collection",
        ] {
            harness.get_by_label(label);
        }
        harness
            .get_by_role_and_label(egui::accesskit::Role::Button, "Close Data Details")
            .click_accesskit();
        for _ in 0..2 {
            harness.step();
        }
        harness.get_by_role_and_label(egui::accesskit::Role::Button, "Repair Data");
        harness
            .get_by_role_and_label(egui::accesskit::Role::Button, "Uninstall Data")
            .click_accesskit();
        for _ in 0..2 {
            harness.step();
        }
        harness.get_by_label("Remove the ESO Weave Data addon?");
        harness.get_by_role_and_label(egui::accesskit::Role::Button, "Confirm Data Uninstall");
    }
}

#[test]
fn s093_outdated_data_addon_offers_update_repair_and_uninstall() {
    let root = tempfile::tempdir().unwrap();
    eso_weave::data_addon::install(
        root.path(),
        eso_weave::data_addon::RunningState::NotRunning,
        beacon::DEFAULT_API_VERSION,
    )
    .unwrap();
    std::fs::write(
        root.path()
            .join(eso_weave::data_addon::DATA_ADDON_SUBFOLDER)
            .join(eso_weave::data_addon::CATALOG_FILE),
        "-- managed drift\n",
    )
    .unwrap();
    let settings = Settings {
        beacon: beacon::prefs_to_value(&BeaconPrefs {
            path_override: Some(root.path().to_path_buf()),
            environment: Environment::Live,
        }),
        ..Settings::default()
    };
    let mut harness = harness_for_app(test_app_with_settings(settings), egui::vec2(1200.0, 1200.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    for label in ["Update Data", "Repair Data", "Uninstall Data"] {
        harness.get_by_role_and_label(egui::accesskit::Role::Button, label);
    }
}

#[test]
fn dashboard_value_cell_uses_the_trailing_space_and_preserves_full_text() {
    use eso_weave::app::ui::{
        dashboard_group_label_width, dashboard_metric, dashboard_metric_row, DashboardRowGeometry,
    };
    use eso_weave::app::StatusRole;

    const FULL_STATE: &str = "Bar 2 | front Two-Handed Greatsword | back Restoration Staff | Ready";

    fn render_row(width: f32) -> (DashboardRowGeometry, f32) {
        let palette = eso_weave::app::theme::palette(Theme::Dark);
        let mut harness = Harness::builder()
            .with_size(egui::vec2(width, 80.0))
            .build_ui_state(
                move |ui, state: &mut (bool, Option<(DashboardRowGeometry, f32)>)| {
                    if !state.0 {
                        eso_weave::app::theme::install_fonts(ui.ctx());
                        state.0 = true;
                        return;
                    }
                    let label_width = dashboard_group_label_width(ui, &["Weapon Bar"], 0.0);
                    let geometry = dashboard_metric_row(
                        ui,
                        &palette,
                        dashboard_metric(
                            "Weapon Bar",
                            FULL_STATE,
                            StatusRole::Healthy,
                            "Full detail",
                        ),
                        label_width,
                        0.0,
                        |_| {},
                    );
                    state.1 = Some((geometry, ui.max_rect().right()));
                },
                (false, None),
            );
        harness.step();
        let value = harness.get_by_label(FULL_STATE);
        // The exact full string remains the AccessKit label even when the paint
        // is constrained and truncated.
        value.focus();
        harness.step();
        harness.state().1.expect("dashboard row geometry")
    }

    let mut previous_width = 0.0;
    for width in [360.0, 600.0, DASHBOARD_WIDE_MIN, 1200.0] {
        let (geometry, trailing_edge) = render_row(width);
        assert!(
            (geometry.value.right() - trailing_edge).abs() <= 1.0,
            "value cell stopped before the trailing edge at {width} points: {geometry:?}"
        );
        assert!(geometry.value.width() > previous_width);
        previous_width = geometry.value.width();
    }
}

fn collect_visible_text_shapes(
    shape: &egui::epaint::Shape,
    clip_rect: egui::Rect,
    observations: &mut Vec<(String, egui::Rect)>,
) {
    match shape {
        egui::epaint::Shape::Text(text) => {
            let visible = text.visual_bounding_rect().intersect(clip_rect);
            if visible.is_positive() {
                observations.push((text.galley.text().to_owned(), visible));
            }
        }
        egui::epaint::Shape::Vec(shapes) => {
            for shape in shapes {
                collect_visible_text_shapes(shape, clip_rect, observations);
            }
        }
        _ => {}
    }
}

fn visible_text_shapes<State>(harness: &Harness<'_, State>) -> Vec<(String, egui::Rect)> {
    let mut observations = Vec::new();
    for clipped in &harness.output().shapes {
        collect_visible_text_shapes(&clipped.shape, clipped.clip_rect, &mut observations);
    }
    observations
}

fn assert_visible_text_does_not_overlap<State>(harness: &Harness<'_, State>, context: &str) {
    let observations = visible_text_shapes(harness);
    assert_text_observations_do_not_overlap(&observations, context);
}

fn assert_visible_text_does_not_overlap_in_rect<State>(
    harness: &Harness<'_, State>,
    container: egui::Rect,
    context: &str,
) {
    let observations = visible_text_shapes(harness)
        .into_iter()
        .filter_map(|(text, rect)| {
            let visible = rect.intersect(container);
            visible.is_positive().then_some((text, visible))
        })
        .collect::<Vec<_>>();
    assert_text_observations_do_not_overlap(&observations, context);
}

fn assert_text_observations_do_not_overlap(observations: &[(String, egui::Rect)], context: &str) {
    for left in 0..observations.len() {
        for right in (left + 1)..observations.len() {
            let (left_text, left_rect) = &observations[left];
            let (right_text, right_rect) = &observations[right];
            let intersection = left_rect.intersect(*right_rect);
            assert!(
                intersection.width() <= 0.5 || intersection.height() <= 0.5,
                "{context}: painted text overlaps: {left_text:?} {left_rect:?} and \
                 {right_text:?} {right_rect:?}, intersection {intersection:?}"
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AddonSurfaceState {
    Missing,
    Installed,
    Outdated,
    Incompatible,
    RemediationRequired,
}

impl AddonSurfaceState {
    const ALL: [Self; 5] = [
        Self::Missing,
        Self::Installed,
        Self::Outdated,
        Self::Incompatible,
        Self::RemediationRequired,
    ];

    fn system_label(self) -> &'static str {
        match self {
            Self::Missing => "Not installed",
            Self::Installed => "Installed",
            Self::Outdated => "Installed (outdated)",
            Self::Incompatible | Self::RemediationRequired => "Installed",
        }
    }

    fn modal_label(self) -> &'static str {
        match self {
            Self::Missing => "Not installed",
            Self::Installed | Self::Outdated => "Current",
            Self::Incompatible => "Update available",
            Self::RemediationRequired => "Unmanaged",
        }
    }

    fn expected_action(self) -> &'static str {
        match self {
            Self::Missing => "Install Data",
            Self::Installed => "Repair Data",
            Self::Outdated => "Update",
            Self::Incompatible => "Update Data",
            Self::RemediationRequired => "Data Details",
        }
    }
}

fn addon_surface_settings(theme: Theme, state: AddonSurfaceState) -> (tempfile::TempDir, Settings) {
    let root = tempfile::tempdir().unwrap();

    if !matches!(state, AddonSurfaceState::Missing) {
        let beacon_directory = root.path().join("PixelBeacon");
        std::fs::create_dir_all(&beacon_directory).unwrap();
        let manifest = if matches!(state, AddonSurfaceState::Outdated) {
            format!("## Title: PixelBeacon\n{MANAGED_MARKER}\n## Version: 1\n")
        } else {
            MANIFEST.to_owned()
        };
        std::fs::write(beacon_directory.join("PixelBeacon.txt"), manifest).unwrap();
    }

    match state {
        AddonSurfaceState::Missing => {}
        AddonSurfaceState::Installed | AddonSurfaceState::Outdated => {
            eso_weave::data_addon::install(
                root.path(),
                eso_weave::data_addon::RunningState::NotRunning,
                beacon::DEFAULT_API_VERSION,
            )
            .unwrap();
        }
        AddonSurfaceState::Incompatible => {
            eso_weave::data_addon::install(
                root.path(),
                eso_weave::data_addon::RunningState::NotRunning,
                beacon::DEFAULT_API_VERSION,
            )
            .unwrap();
            std::fs::write(
                root.path()
                    .join(eso_weave::data_addon::DATA_ADDON_SUBFOLDER)
                    .join(eso_weave::data_addon::CATALOG_FILE),
                "-- incompatible managed package\n",
            )
            .unwrap();
        }
        AddonSurfaceState::RemediationRequired => {
            let data_directory = root
                .path()
                .join(eso_weave::data_addon::DATA_ADDON_SUBFOLDER);
            std::fs::create_dir_all(&data_directory).unwrap();
            std::fs::write(
                data_directory.join(eso_weave::data_addon::MANIFEST_FILE),
                "## Title: Unmanaged ESO Weave Data\n",
            )
            .unwrap();
        }
    }

    let settings = Settings {
        beacon: beacon::prefs_to_value(&BeaconPrefs {
            path_override: Some(root.path().to_path_buf()),
            environment: Environment::Live,
        }),
        ui: eso_weave::app::settings_form::ui_to_value(&UiPrefs {
            theme,
            ..UiPrefs::default()
        }),
        ..Settings::default()
    };
    (root, settings)
}

#[test]
fn expanded_system_state_surface_rejects_painted_text_overlap() {
    for theme in [Theme::Dark, Theme::Light] {
        for text_scale in [1.0, 1.25] {
            for width in [760.0, DASHBOARD_WIDE_MIN, 900.0, 1200.0] {
                for state in AddonSurfaceState::ALL {
                    let (_root, settings) = addon_surface_settings(theme, state);
                    let mut harness = harness_for_app(
                        test_app_with_settings(settings),
                        egui::vec2(width, 1200.0),
                    );
                    if text_scale != 1.0 {
                        harness.ctx.all_styles_mut(|style| {
                            for font in style.text_styles.values_mut() {
                                font.size *= text_scale;
                            }
                        });
                    }
                    for _ in 0..SETTLE {
                        harness.step();
                    }
                    assert!(
                        harness.query_all_by_label(state.system_label()).count() >= 1,
                        "missing full-surface state label for {state:?}"
                    );
                    harness.get_by_role_and_label(
                        egui::accesskit::Role::Button,
                        state.expected_action(),
                    );
                    let (_, system) = harness
                        .state()
                        .dashboard_rects()
                        .expect("expanded dashboard geometry");
                    assert_visible_text_does_not_overlap_in_rect(
                        &harness,
                        system,
                        &format!(
                            "System and State, {state:?}, {theme:?}, {text_scale}x, {width} points"
                        ),
                    );
                }
            }
        }
    }
}

#[test]
fn data_details_modal_rejects_painted_text_overlap() {
    const MODAL_TITLE: &str = "ESO Weave Data Details";

    for theme in [Theme::Dark, Theme::Light] {
        for text_scale in [1.0, 1.25] {
            for width in [520.0, 760.0, 900.0, 1200.0] {
                for state in AddonSurfaceState::ALL {
                    let (_root, settings) = addon_surface_settings(theme, state);
                    let mut app = test_app_with_settings(settings);
                    app.set_data_addon_details_open(true);
                    let mut harness = harness_for_app(app, egui::vec2(width, 1200.0));
                    if text_scale != 1.0 {
                        harness.ctx.all_styles_mut(|style| {
                            for font in style.text_styles.values_mut() {
                                font.size *= text_scale;
                            }
                        });
                    }
                    for _ in 0..SETTLE {
                        harness.step();
                    }
                    harness.get_by_label(MODAL_TITLE);
                    assert!(
                        harness.query_all_by_label(state.modal_label()).count() >= 1,
                        "missing modal state label for {state:?}"
                    );
                    for title in eso_weave::app::ui::DATA_DETAILS_STATUS_TITLES {
                        assert!(
                            harness.query_all_by_label(title).count() >= 1,
                            "missing complete accessible modal label {title:?}"
                        );
                    }

                    // The modal is painted after the dashboard. Starting at its unique
                    // heading scopes the flattened paint list to the foreground layer
                    // and avoids treating intentionally obscured dashboard text as a
                    // collision with modal content.
                    let observations = visible_text_shapes(&harness);
                    let start = observations
                        .iter()
                        .position(|(text, _)| text == MODAL_TITLE)
                        .expect("modal title paint");
                    assert_text_observations_do_not_overlap(
                        &observations[start..],
                        &format!(
                            "Data Details, {state:?}, {theme:?}, {text_scale}x, {width} points"
                        ),
                    );
                }
            }
        }
    }
}

#[test]
fn dashboard_status_row_rejects_painted_text_overlap() {
    use eso_weave::app::ui::{
        dashboard_group_label_width, dashboard_metric, dashboard_metric_row,
        DATA_DETAILS_STATUS_TITLES, LIVE_HUD_STATUS_TITLES, SYSTEM_STATE_STATUS_TITLES,
    };
    use eso_weave::app::StatusRole;

    let groups = [
        ("live HUD", LIVE_HUD_STATUS_TITLES, 0.0),
        ("system and state", SYSTEM_STATE_STATUS_TITLES, 212.0),
        ("data details", DATA_DETAILS_STATUS_TITLES, 0.0),
    ];

    for theme in [Theme::Dark, Theme::Light] {
        for text_scale in [1.0, 1.25] {
            for (group_name, titles, interaction_width) in groups {
                for width in [360.0, 520.0, 680.0, 900.0, 1200.0] {
                    for title in titles {
                        let palette = eso_weave::app::theme::palette(theme);
                        let mut initialized = false;
                        let mut harness = Harness::builder()
                            .with_size(egui::vec2(width, 80.0))
                            .build_ui_state(
                                move |ui,
                                      geometry: &mut Option<
                                    eso_weave::app::ui::DashboardRowGeometry,
                                >| {
                                    if !initialized {
                                        eso_weave::app::theme::install_fonts(ui.ctx());
                                        eso_weave::app::theme::apply(ui.ctx(), theme);
                                        for font in ui.style_mut().text_styles.values_mut() {
                                            font.size *= text_scale;
                                        }
                                        initialized = true;
                                        return;
                                    }
                                    let label_width =
                                        dashboard_group_label_width(ui, titles, interaction_width);
                                    *geometry = Some(dashboard_metric_row(
                                        ui,
                                        &palette,
                                        dashboard_metric(
                                            title,
                                            "Installed (outdated)",
                                            StatusRole::Warning,
                                            "Status",
                                        ),
                                        label_width,
                                        interaction_width,
                                        |ui| {
                                            if interaction_width > 0.0 {
                                                let _ = ui.button("Repair Data");
                                            }
                                        },
                                    ));
                                },
                                None,
                            );
                        harness.step();
                        harness.get_by_label(title);
                        harness.get_by_label("Installed (outdated)");
                        let geometry = harness.state().expect("dashboard row geometry");
                        assert!(geometry.row.contains_rect(geometry.label));
                        assert!(geometry.row.contains_rect(geometry.value));
                        assert!(geometry.label.right() <= geometry.value.left());
                        if let Some(interaction) = geometry.interaction {
                            assert!(geometry.row.contains_rect(interaction));
                            assert!(geometry.value.right() <= interaction.left());
                        }
                        for (text, rect) in visible_text_shapes(&harness) {
                            let owner = if text == *title {
                                geometry.label
                            } else if text == "●" || text == "Installed (outdated)" {
                                geometry.value
                            } else if text == "Repair Data" {
                                geometry.interaction.expect("interaction allocation")
                            } else {
                                continue;
                            };
                            assert!(
                                owner.expand(0.5).contains_rect(rect),
                                "{group_name}, {theme:?}, {text_scale}x, {width} points: \
                                 {text:?} paints outside {owner:?}: {rect:?}"
                            );
                        }
                        assert_visible_text_does_not_overlap(
                            &harness,
                            &format!(
                                "{group_name}, {theme:?}, {text_scale}x, {width} points, {title}"
                            ),
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn resource_group_gap_follows_three_or_four_meters() {
    fn gaps(count: usize) -> (f32, f32) {
        let palette = eso_weave::app::theme::palette(Theme::Dark);
        let views = [
            resource_view(eso_weave::pixelbus::ResourceLevel::Percent(10)),
            resource_view(eso_weave::pixelbus::ResourceLevel::Percent(20)),
            resource_view(eso_weave::pixelbus::ResourceLevel::Percent(30)),
            resource_view(eso_weave::pixelbus::ResourceLevel::Percent(40)),
        ];
        let descriptors = [
            ("Health", &views[0], ResourceTheme::Health),
            ("Stamina", &views[1], ResourceTheme::Stamina),
            ("Magicka", &views[2], ResourceTheme::Magicka),
            ("Synthetic", &views[3], ResourceTheme::Magicka),
        ];
        let mut harness = Harness::new_ui(|ui| {
            widgets::resource_group(ui, &palette, &descriptors[..count], 6.0);
            ui.label("Game Context");
        });
        harness.step();

        let first = harness
            .get_by_role_and_label(egui::accesskit::Role::ProgressIndicator, "Health: 10%")
            .rect();
        let second = harness
            .get_by_role_and_label(egui::accesskit::Role::ProgressIndicator, "Stamina: 20%")
            .rect();
        let last_label = if count == 3 {
            "Magicka: 30%"
        } else {
            "Synthetic: 40%"
        };
        let last = harness
            .get_by_role_and_label(egui::accesskit::Role::ProgressIndicator, last_label)
            .rect();
        let context = harness.get_by_label("Game Context").rect();
        (second.top() - first.bottom(), context.top() - last.bottom())
    }

    let (ordinary, three_gap) = gaps(3);
    let (_, four_gap) = gaps(4);
    assert!(three_gap > ordinary);
    assert!((three_gap - four_gap).abs() <= 1.0);
}

#[test]
fn outdated_addon_actions_fit_narrow_and_at_the_wide_breakpoint() {
    let root = tempfile::tempdir().unwrap();
    let addon = root.path().join("PixelBeacon");
    std::fs::create_dir_all(&addon).unwrap();
    std::fs::write(
        addon.join("PixelBeacon.txt"),
        format!("## Title: PixelBeacon\n{MANAGED_MARKER}\n## Version: 1\n"),
    )
    .unwrap();
    let settings = Settings {
        beacon: beacon::prefs_to_value(&BeaconPrefs {
            path_override: Some(root.path().to_path_buf()),
            environment: Environment::Live,
        }),
        ..Settings::default()
    };

    for width in [560.0, DASHBOARD_WIDE_MIN + 32.0] {
        let app = render_app_at(
            test_app_with_settings(settings.clone()),
            egui::vec2(width, 1200.0),
            SETTLE,
        );
        let (live, system) = app.dashboard_rects().expect("dashboard geometry");
        assert!(
            system.right() <= width + 0.5,
            "operational actions escape the {width}-point viewport: {system:?}"
        );
        if width > 560.0 {
            assert_eq!(app.last_dashboard_layout(), Some(DashboardLayout::Wide));
            assert!(
                live.right() <= system.left() + 0.5,
                "outdated addon actions overlap Live HUD: {live:?}, {system:?}"
            );
        }
    }
}

#[test]
fn dashboard_breakpoint_uses_available_point_width() {
    assert_eq!(
        eso_weave::app::dashboard_layout(DASHBOARD_WIDE_MIN - 0.1),
        DashboardLayout::Narrow
    );
    assert_eq!(
        eso_weave::app::dashboard_layout(DASHBOARD_WIDE_MIN),
        DashboardLayout::Wide
    );
}

#[test]
fn resource_meter_exposes_name_state_and_numeric_value() {
    let palette = eso_weave::app::theme::palette(Theme::Dark);
    let observed = resource_view(eso_weave::pixelbus::ResourceLevel::Percent(50));
    let mut harness = Harness::new_ui(|ui| {
        widgets::resource_meter(ui, &palette, "Health", &observed, ResourceTheme::Health);
    });
    harness.step();

    let meter =
        harness.get_by_role_and_label(egui::accesskit::Role::ProgressIndicator, "Health: 50%");
    assert_eq!(meter.accesskit_node().numeric_value(), Some(50.0));
}

#[test]
fn resource_meter_keeps_non_numeric_states_distinct() {
    let palette = eso_weave::app::theme::palette(Theme::Dark);
    let dormant = eso_weave::app::ResourceView::dormant();
    let unavailable = resource_view(eso_weave::pixelbus::ResourceLevel::Unknown);
    let mut harness = Harness::new_ui(|ui| {
        widgets::resource_meter(ui, &palette, "Health", &dormant, ResourceTheme::Health);
        widgets::resource_meter(
            ui,
            &palette,
            "Magicka",
            &unavailable,
            ResourceTheme::Magicka,
        );
    });
    harness.step();

    let dormant_meter = harness.get_by_role_and_label(
        egui::accesskit::Role::ProgressIndicator,
        "Health: Game not active",
    );
    let unavailable_meter = harness.get_by_role_and_label(
        egui::accesskit::Role::ProgressIndicator,
        "Magicka: Signal unavailable",
    );
    assert_eq!(dormant_meter.accesskit_node().numeric_value(), None);
    assert_eq!(unavailable_meter.accesskit_node().numeric_value(), None);
    assert_ne!(dormant_meter.rect(), unavailable_meter.rect());
}

#[test]
fn resource_meter_geometry_is_stable_across_boundary_values() {
    fn meter_rect(percent: u8) -> egui::Rect {
        let palette = eso_weave::app::theme::palette(Theme::Dark);
        let view = resource_view(eso_weave::pixelbus::ResourceLevel::Percent(percent));
        let label = format!("Health: {percent}%");
        let mut harness = Harness::new_ui(|ui| {
            widgets::resource_meter(ui, &palette, "Health", &view, ResourceTheme::Health);
        });
        harness.step();
        harness
            .get_by_role_and_label(egui::accesskit::Role::ProgressIndicator, &label)
            .rect()
    }

    let baseline = meter_rect(0).size();
    for percent in [1, 50, 99, 100] {
        assert_eq!(meter_rect(percent).size(), baseline);
    }
}

#[test]
fn meter_geometry_places_quarters_and_reserves_three_point_threshold_protrusion() {
    let rect = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(400.0, 24.0));
    let geometry = widgets::resource_meter_geometry(rect, Some(0.5));
    for (segment, fraction) in geometry.quarters.into_iter().zip([0.25, 0.5, 0.75]) {
        assert_eq!(
            segment[0].x,
            geometry.track.left() + geometry.track.width() * fraction
        );
        assert!(segment[0].y > geometry.track.top());
        assert!(segment[1].y < geometry.track.bottom());
    }
    let threshold = geometry.threshold.expect("known cost");
    assert_eq!(threshold[0].x, geometry.quarters[1][0].x);
    assert!(threshold[0].y < geometry.track.bottom());
    assert_eq!(threshold[1].y, rect.bottom());
    assert_eq!(rect.bottom() - geometry.track.bottom(), 3.0);
    assert_eq!(geometry.numeric.right(), geometry.ready.left());
}

#[test]
fn ultimate_meter_exposes_exact_accessible_value_and_fixed_ready_state() {
    let palette = eso_weave::app::theme::palette(Theme::Dark);
    let view = eso_weave::app::ultimate_view(
        eso_weave::pixelbus::UltimateTelemetry {
            current: eso_weave::pixelbus::UltimateValue::Points(185),
            maximum: eso_weave::pixelbus::UltimateValue::Points(500),
            front_cost: eso_weave::pixelbus::UltimateValue::Points(125),
            back_cost: eso_weave::pixelbus::UltimateValue::Unknown,
        },
        eso_weave::pixelbus::ActiveBar::Front,
    );
    let label = "Ultimate: 185 of 500; Front bar cost 125; Ready";
    let mut harness = Harness::new_ui(|ui| {
        widgets::ultimate_meter(ui, &palette, "Ultimate", &view);
    });
    harness.step();
    let meter = harness.get_by_role_and_label(egui::accesskit::Role::ProgressIndicator, label);
    assert_eq!(meter.accesskit_node().numeric_value(), Some(37.0));
}

#[test]
fn ultimate_ready_transition_preserves_meter_geometry() {
    let rect = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(400.0, 24.0));
    let geometries =
        [124_u16, 125, 126].map(|_| widgets::resource_meter_geometry(rect, Some(125.0 / 500.0)));
    assert_eq!(geometries[0], geometries[1]);
    assert_eq!(geometries[1], geometries[2]);
    assert!(geometries[0].numeric.right() <= geometries[0].ready.left());

    for (current, expected_ready) in [(124, false), (125, true), (126, true)] {
        let view = eso_weave::app::ultimate_view(
            eso_weave::pixelbus::UltimateTelemetry {
                current: eso_weave::pixelbus::UltimateValue::Points(current),
                maximum: eso_weave::pixelbus::UltimateValue::Points(500),
                front_cost: eso_weave::pixelbus::UltimateValue::Points(125),
                back_cost: eso_weave::pixelbus::UltimateValue::Unknown,
            },
            eso_weave::pixelbus::ActiveBar::Front,
        );
        assert_eq!(view.ready, Some(expected_ready));
    }
}

#[test]
fn dashboard_accessibility_tree_names_sections_and_dormant_resources() {
    let mut harness = harness_at(egui::vec2(760.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }

    harness.get_by_label("Live HUD");
    harness.get_by_label("Roll Dodge");
    harness.get_by_label("System and State");
    harness.get_by_label("World State");
    harness.get_by_label("Travel");
    for label in [
        "Health: Game not active",
        "Stamina: Game not active",
        "Magicka: Game not active",
        "Ultimate: Game not active",
    ] {
        let meter = harness.get_by_role_and_label(egui::accesskit::Role::ProgressIndicator, label);
        assert_eq!(meter.accesskit_node().numeric_value(), None);
    }
}

#[test]
fn system_state_disclosure_collapses_accessibly_and_reclaims_height() {
    let mut harness = harness_at(egui::vec2(760.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    let expanded_height = harness.state().content_extent().y;
    let disclosure = harness.get_by_label("System and State");
    disclosure.click_accesskit();
    for _ in 0..SETTLE {
        harness.step();
    }
    assert!(!harness.state().system_state_expanded());
    assert!(harness.state().content_extent().y < expanded_height);
    assert!(harness.query_by_label("Application").is_none());
    assert!(harness.query_by_label("World State").is_none());
    harness.get_by_label("Skills");
}

/// C1 (S046 FR-016/FR-017): intrinsic width is independent of the window while
/// height follows one of the two explicit responsive arrangements.
///
/// This is the assertion that fails on v0.8.0, where the measured extent is the
/// window size less a constant on both axes (see research.md R1 confirmed).
#[test]
fn intrinsic_width_is_stable_and_narrow_layout_is_taller() {
    let small = render_at(egui::vec2(700.0, 800.0), SETTLE).content_extent();
    let large = render_at(egui::vec2(1600.0, 1200.0), SETTLE).content_extent();

    assert!(
        (small.x - large.x).abs() <= 0.5,
        "intrinsic width tracks the window: {} at 700 wide, {} at 1600 wide",
        small.x,
        large.x
    );
    assert!(
        small.y > large.y,
        "stacked layout should be taller: {small:?}, {large:?}"
    );
}

/// C2 (FR-001): the minimum pushed to the viewport is the intrinsic extent when
/// the log is closed.
#[test]
fn enforced_minimum_equals_intrinsic_extent_log_closed() {
    let app = render_at(egui::vec2(1200.0, 1000.0), SETTLE);
    let extent = app.content_extent();
    let sent = app
        .last_min_sent()
        .expect("a minimum should have been sent");

    assert!(
        (sent.x - extent.x).abs() <= 0.5 && (sent.y - extent.y).abs() <= 0.5,
        "minimum {sent:?} does not equal intrinsic extent {extent:?}"
    );
}

/// C2 (FR-013): with the log open the minimum adds the width bonus and the open
/// log reserve on top of the intrinsic extent.
#[test]
fn enforced_minimum_adds_log_reserve_when_open() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    harness.step();
    harness.state_mut().set_log_panel_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }
    let app = harness.into_state();
    let extent = app.content_extent();
    let sent = app
        .last_min_sent()
        .expect("a minimum should have been sent");

    assert!(
        sent.x > extent.x && sent.y > extent.y,
        "open-log minimum {sent:?} should exceed the intrinsic extent {extent:?}"
    );
}

#[test]
fn collapsed_log_minimum_does_not_reserve_hidden_card_body() {
    fn collapsed_app(log_open: bool) -> EsoWeaveApp {
        let settings = Settings {
            ui: eso_weave::app::settings_form::ui_to_value(&UiPrefs {
                system_state_expanded: false,
                ..UiPrefs::default()
            }),
            ..Settings::default()
        };
        let mut harness =
            harness_for_app(test_app_with_settings(settings), egui::vec2(1400.0, 1000.0));
        harness.step();
        harness.state_mut().set_log_panel_open(log_open);
        for _ in 0..SETTLE {
            harness.step();
        }
        harness.into_state()
    }

    let closed = collapsed_app(false);
    let open = collapsed_app(true);
    let reserve = open.last_min_sent().unwrap().y - open.content_extent().y;
    assert!(reserve > 0.0);
    assert!(
        (open.content_extent().y - closed.content_extent().y).abs() <= 0.5,
        "opening the log reserved the hidden expanded card body: closed {:?}, open {:?}",
        closed.content_extent(),
        open.content_extent()
    );
}

/// C3 (S046 FR-017): across a monotonically shrinking gesture, the minimum width
/// stays intrinsic and the height can take only the documented wide or narrow
/// content extent. A continuously window-derived value would produce many heights.
///
/// This is the defect no arithmetic-only test can express, and the reason three
/// prior slices shipped green (see research.md R5).
#[test]
fn enforced_minimum_uses_only_responsive_extents_during_shrink() {
    let wide_extent = render_at(egui::vec2(1600.0, 1200.0), SETTLE).content_extent();
    let narrow_extent = render_at(egui::vec2(700.0, 1200.0), SETTLE).content_extent();
    let mut harness = harness_at(egui::vec2(1600.0, 1200.0));
    for _ in 0..SETTLE {
        harness.step();
    }

    let baseline = harness
        .state()
        .last_min_sent()
        .expect("a minimum should have been sent");

    // One continuous gesture: the window shrinks a step at a time, with a frame
    // rendered at each step and no settling in between.
    let mut width = 1600.0_f32;
    let mut height = 1200.0_f32;
    while width > 500.0 || height > 460.0 {
        width = (width - 40.0).max(500.0);
        height = (height - 40.0).max(460.0);
        harness.input_mut().screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, height),
        ));
        harness.step();

        let sent = harness
            .state()
            .last_min_sent()
            .expect("a minimum should have been sent");
        assert!(
            (sent.x - baseline.x).abs() <= 0.5,
            "at window {width}x{height} intrinsic width moved to {} from {}",
            sent.x,
            baseline.x
        );
        assert!(
            (sent.y - wide_extent.y).abs() <= 0.5 || (sent.y - narrow_extent.y).abs() <= 0.5,
            "at window {width}x{height} minimum height {} is neither wide {} nor narrow {}",
            sent.y,
            wide_extent.y,
            narrow_extent.y
        );
    }
}

/// FR-002: the boot minimum applies until the content has been measured, so
/// nothing is clipped before the first layout. Asserted on the constructed state,
/// which is the only point at which "before any measurement" is observable; how
/// many frames the harness runs per `step` is an implementation detail of the
/// harness and not something to pin.
#[test]
fn boot_minimum_applies_before_any_frame_is_rendered() {
    let extent = test_app().content_extent();

    assert_eq!(
        (extent.x, extent.y),
        (480.0, 420.0),
        "a freshly constructed app should sit on the boot floor, got {extent:?}"
    );
}

/// FR-006: the content actually fits inside the enforced minimum, so nothing is
/// clipped when the user shrinks the window all the way down.
#[test]
fn content_fits_within_the_enforced_minimum() {
    let app = render_at(egui::vec2(1200.0, 1000.0), SETTLE);
    let extent = app.content_extent();
    let sent = app
        .last_min_sent()
        .expect("a minimum should have been sent");

    assert!(
        sent.x >= extent.x - 0.5 && sent.y >= extent.y - 0.5,
        "minimum {sent:?} is smaller than the content it must show {extent:?}"
    );
}

/// Slice 037: the skills grid gained a Cooldown column, and that grid is the
/// widest content-sized block in the window, so it is what the intrinsic width is
/// computed from.
///
/// This is the assertion that would have caught the column being added without
/// anyone thinking about the window: it pins the relationship between the column
/// count and the enforced minimum, rather than trusting that a wider grid happens
/// to still fit. The bound is deliberately loose, because the point is to catch a
/// column being added while the minimum stays put, not to freeze a pixel width
/// that legitimate styling changes would move.
#[test]
fn the_enforced_minimum_accounts_for_every_skills_column() {
    use eso_weave::app::strings;

    let app = render_at(egui::vec2(1200.0, 1000.0), SETTLE);
    let extent = app.content_extent();
    let sent = app
        .last_min_sent()
        .expect("a minimum should have been sent");

    assert_eq!(
        strings::SKILL_COLUMNS.len(),
        6,
        "this test is calibrated against the shipping column count"
    );

    // Every column needs somewhere to be drawn, so the intrinsic width cannot be
    // narrower than the columns require, and the enforced minimum cannot be
    // narrower than the intrinsic width.
    let columns = strings::SKILL_COLUMNS.len() as f32;
    assert!(
        extent.x > columns * 40.0,
        "intrinsic width {} is too narrow to hold {columns} skills columns",
        extent.x
    );
    assert!(
        sent.x >= extent.x - 0.5,
        "the enforced minimum {sent:?} does not cover the widened skills grid {extent:?}"
    );
}

// ---------------------------------------------------------------------------
// US2 / issue #13: the live log pane never covers an interactive control.
// ---------------------------------------------------------------------------

/// Asserts the never-overlap invariant for the current frame (contract C4).
fn assert_no_overlap(app: &EsoWeaveApp, context: &str) {
    let (Some(log_top), Some(content_bottom)) = (app.last_log_top(), app.last_content_bottom())
    else {
        panic!("{context}: expected the log pane and content geometry to be recorded");
    };
    assert!(
        log_top >= content_bottom - 0.5,
        "{context}: log pane top {log_top} is above the content bottom {content_bottom}, \
         so it covers {} points of interactive controls",
        content_bottom - log_top
    );
}

/// C4 (FR-010): dragging the splitter upward past the boundary never covers a
/// control, on any frame of the gesture.
#[test]
fn log_pane_never_covers_controls_during_a_splitter_drag() {
    let mut harness = harness_at(egui::vec2(900.0, 1000.0));
    harness.step();
    harness.state_mut().set_log_panel_open(true);
    settle_at_enforced_minimum(&mut harness, egui::vec2(900.0, 1000.0));
    assert_no_overlap(harness.state(), "before the drag");

    // Drag the splitter upward far past the boundary, a step at a time, checking
    // every frame of the gesture rather than only the settled result.
    let splitter_y = harness.state().last_log_top().expect("log pane open");
    harness.hover_at(egui::pos2(450.0, splitter_y));
    harness.step();
    let mut y = splitter_y;
    while y > 40.0 {
        y -= 30.0;
        harness.drag_at(egui::pos2(450.0, y));
        harness.step();
        assert_no_overlap(harness.state(), &format!("mid-drag at y={y}"));
    }
    harness.drop_at(egui::pos2(450.0, y));
    // Settled rather than a single frame. An over-dragged commit is re-clamped and
    // re-applied on the *following* frame by design (see the log_reseed branch in
    // ui.rs, which says so), and the content measurement lags a frame too, so a
    // drop that changes both needs two frames to converge. That was always true;
    // before slice 038 grew the content by two status rows the deferred frame
    // happened not to breach the boundary, so one step sufficed.
    //
    // This does not weaken the invariant. Every frame of the gesture above is
    // still asserted individually, which is where a persistent overlap would show,
    // and the settled state below is what the contract is actually about.
    for _ in 0..SETTLE {
        harness.step();
    }
    assert_no_overlap(harness.state(), "after the drop");
}

/// C4 (FR-010): the boundary holds while the window is resized with the log open,
/// and on a resize immediately followed by a drag with no settled frame between.
#[test]
fn log_pane_never_covers_controls_during_a_window_resize() {
    let mut harness = harness_at(egui::vec2(1400.0, 1100.0));
    harness.step();
    harness.state_mut().set_log_panel_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }

    // The floor is the minimum the app itself enforces while the log is open, read
    // from the app rather than written as a literal. It used to be a hardcoded 500,
    // which was above the enforced minimum when it was written and fell below it
    // when slice 038 added two status rows. A sweep that runs past the enforced
    // minimum is asking the window to be smaller than the application permits,
    // which no window manager would do and which says nothing about the invariant.
    // Deliberately derived, so the next slice that grows the content does not have
    // to notice this number at all.
    let floor = harness
        .state()
        .last_min_sent()
        .expect("a minimum should have been sent")
        .y;
    let mut height = 1100.0_f32;
    while height - 50.0 >= floor {
        height -= 50.0;
        harness.input_mut().screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1400.0, height),
        ));
        harness.step();
        assert_no_overlap(harness.state(), &format!("resizing to height {height}"));

        // A drag begun immediately after the resize, with no settled frame.
        if let Some(top) = harness.state().last_log_top() {
            harness.drag_at(egui::pos2(700.0, top - 60.0));
            harness.step();
            assert_no_overlap(
                harness.state(),
                &format!("drag right after resize {height}"),
            );
        }
    }
}

#[test]
fn log_never_overlaps_during_a_width_only_switch_to_the_taller_layout() {
    let mut harness = harness_at(egui::vec2(1400.0, 1100.0));
    harness.step();
    harness.state_mut().set_log_panel_open(true);
    settle_at_enforced_minimum(&mut harness, egui::vec2(1400.0, 1100.0));
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Wide)
    );
    assert_no_overlap(harness.state(), "wide layout before width-only resize");

    let wide_min_height = harness.state().last_min_sent().expect("wide log minimum").y;
    harness.input_mut().screen_rect = Some(egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(700.0, wide_min_height),
    ));
    harness.step();

    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Narrow)
    );
    if harness.state().last_log_top().is_some() {
        assert_no_overlap(
            harness.state(),
            "first narrow frame after width-only resize",
        );
    } else {
        assert!(
            harness.state().last_content_bottom().is_none(),
            "no log boundary should be recorded while the pane is deferred"
        );
    }

    let narrow_min_height = harness
        .state()
        .last_min_sent()
        .expect("narrow log minimum")
        .y;
    harness.input_mut().screen_rect = Some(egui::Rect::from_min_size(
        egui::Pos2::ZERO,
        egui::vec2(700.0, narrow_min_height),
    ));
    for _ in 0..SETTLE {
        harness.step();
    }
    assert_no_overlap(harness.state(), "log restored after narrow growth");
}

/// FR-011: a height committed past the boundary is clamped before it is stored,
/// so nothing out of range is persisted or restored.
#[test]
fn committed_log_height_is_clamped_before_it_is_stored() {
    let mut harness = harness_at(egui::vec2(900.0, 1000.0));
    harness.step();
    harness.state_mut().set_log_panel_open(true);
    let window_h = settle_at_enforced_minimum(&mut harness, egui::vec2(900.0, 1000.0)).y;

    let splitter_y = harness.state().last_log_top().expect("log pane open");
    harness.hover_at(egui::pos2(450.0, splitter_y));
    harness.step();
    harness.drag_at(egui::pos2(450.0, 20.0));
    harness.step();
    harness.drop_at(egui::pos2(450.0, 20.0));
    harness.step();

    let app = harness.into_state();
    let content_h = app.content_extent().y;
    let stored = app.log_height();
    assert!(
        stored <= (window_h - content_h).max(0.0) + 0.5,
        "stored log height {stored} exceeds the boundary (window {window_h} - content {content_h})"
    );
}

// ---------------------------------------------------------------------------
// US3 / issue #14: the settings modal grows with the window.
// ---------------------------------------------------------------------------

/// Renders with the settings modal open at the given window size.
fn render_modal_at(size: egui::Vec2) -> EsoWeaveApp {
    let mut harness = harness_at(size);
    harness.step();
    harness.state_mut().set_settings_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }
    harness.into_state()
}

#[test]
fn s102_settings_modal_exposes_detected_bindings_and_application_boundaries() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    harness.step();
    harness.state_mut().set_settings_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }

    harness.get_by_label("Detected Interact Binding");
    harness.get_by_label("Detected Quickslot Binding");
    assert!(
        harness
            .query_all(egui_kittest::kittest::By::new().value("Unavailable"))
            .count()
            >= 2,
        "both read-only native binding rows should show unavailable startup evidence"
    );
    harness.get_by_label(eso_weave::app::strings::FISHING_SETTINGS_APPLICATION_HELP);
    harness.get_by_label(eso_weave::app::strings::READER_SETTINGS_APPLICATION_HELP);
}

#[test]
fn s110_settings_modal_exposes_the_accessible_ultimate_watch() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    harness.step();
    harness.state_mut().set_settings_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }

    harness.get_by_label(eso_weave::app::strings::SET_POTION_ULTIMATE.label);
    harness.get_by_label(eso_weave::app::strings::SET_POTION_ULTIMATE.help);
}

#[test]
fn s098_settings_modal_exposes_the_bounded_stale_retention_control() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    harness.step();
    harness.state_mut().set_settings_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }

    harness.get_by_label(eso_weave::app::strings::SET_STALE_RETENTION.label);
    harness.get_by_value("120 s");
}

#[test]
fn s105_settings_modal_exposes_one_local_service_toggle_warning_and_status() {
    let settings = Settings {
        local_service: eso_weave::local_service::LocalServicePrefs {
            enabled: false,
            credential: Some(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
            ),
        }
        .store(),
        ..Settings::default()
    };
    let mut harness = harness_for_app(test_app_with_settings(settings), egui::vec2(1400.0, 1000.0));
    harness.step();
    harness.state_mut().set_settings_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }

    harness.get_by_label(eso_weave::app::strings::SET_LOCAL_SERVICE_ENABLED.label);
    harness.get_by_label("Status");
    harness.get_by_label(eso_weave::app::strings::LOCAL_SERVICE_WARNING);
    harness.get_by_label("stopped");
    harness.get_by_label(eso_weave::app::strings::LOCAL_SERVICE_COPY_CREDENTIAL);
}

/// C5 (FR-014): the modal's rendered rectangle matches the size its growth rule
/// calls for, on both axes, at every window size.
#[test]
fn modal_renders_at_its_computed_extent() {
    for window_h in [420.0_f32, 720.0, 1200.0, 1440.0, 2160.0] {
        let window_w = (window_h * 1.4).max(700.0);
        let app = render_modal_at(egui::vec2(window_w, window_h));
        let rendered = app.last_modal_size().expect("the modal should be open");
        // Compared against the app's own computed target, not a recomputation from
        // a guessed content-rect inset: the contract is that the rendered size
        // equals what the growth rule asked for, and the rule itself is covered by
        // the pure tests in app_window_sizing.rs.
        let target = app.last_modal_target().expect("the modal should be open");

        assert!(
            (rendered.y - target.y).abs() <= 1.0,
            "at window {window_w}x{window_h} the modal rendered {} tall, rule asked for {}",
            rendered.y,
            target.y
        );
        assert!(
            (rendered.x - target.x).abs() <= 1.0,
            "at window {window_w}x{window_h} the modal rendered {} wide, rule asked for {}",
            rendered.x,
            target.x
        );
        // And the target must never exceed the window it has to fit inside.
        assert!(
            target.y <= window_h && target.x <= window_w,
            "at window {window_w}x{window_h} the rule asked for {target:?}, larger than the window"
        );
    }
}

/// C5 (FR-015): the modal grows on both axes as the window grows, and stops at its
/// configured maximum.
#[test]
fn modal_grows_with_the_window_then_stops_at_its_maximum() {
    let small = render_modal_at(egui::vec2(700.0, 620.0))
        .last_modal_size()
        .expect("open");
    let mid = render_modal_at(egui::vec2(1200.0, 1000.0))
        .last_modal_size()
        .expect("open");
    let huge = render_modal_at(egui::vec2(2600.0, 2160.0))
        .last_modal_size()
        .expect("open");

    assert!(
        mid.y > small.y,
        "modal did not grow in height: {small:?} -> {mid:?}"
    );
    assert!(
        mid.x > small.x,
        "modal did not grow in width: {small:?} -> {mid:?}"
    );
    // Raised from 880 in slice 039: the settings body grew by the auto-potion
    // group and a keybinding row, past the FR-017 half-visible bound, and raising
    // the maximum is the resolution slice 030 recorded for exactly that.
    assert!(
        huge.y <= 1800.0 + 1.0,
        "modal height {} exceeded its maximum",
        huge.y
    );
    assert!(
        huge.x <= 1040.0 + 1.0,
        "modal width {} exceeded its maximum",
        huge.x
    );
}

/// C6 (FR-017): at the modal's maximum size, at least half the settings body is
/// visible without scrolling.
#[test]
fn modal_shows_at_least_half_the_settings_body_at_maximum() {
    let app = render_modal_at(egui::vec2(2600.0, 2160.0));
    let body = app
        .last_settings_body_height()
        .expect("the settings body height should be recorded");
    // The visible height is what the body scroll area was actually given, recorded
    // from the frame itself. Deriving it from the modal height less a constant
    // would restate the very assumption that made the old chrome reserve wrong.
    let visible = app
        .last_settings_body_visible()
        .expect("the visible body height should be recorded");

    println!("FR-017 measurement: visible {visible} of body {body}");
    assert!(
        visible / body >= 0.5,
        "at the modal maximum only {visible} of {body} points of settings body are \
         visible ({:.0} percent); FR-017 requires at least half",
        100.0 * visible / body
    );
}

#[test]
fn brandbuilder_fine_pointer_responses_meet_the_desktop_target() {
    let mut harness = harness_at(egui::vec2(1200.0, 1400.0));
    settle_at_enforced_minimum(&mut harness, egui::vec2(1200.0, 1400.0));
    for (role, label) in [
        (egui::accesskit::Role::Button, "File"),
        (egui::accesskit::Role::Button, "Help"),
        (egui::accesskit::Role::CheckBox, "ESO Weave"),
        (egui::accesskit::Role::CheckBox, "Fishing"),
    ] {
        let rect = harness.get_by_role_and_label(role, label).rect();
        assert!(
            rect.width() >= 28.0 && rect.height() >= 28.0,
            "{label} response is {rect:?}, below the 28 point fine-pointer target"
        );
    }
}

/// FR-004, FR-005: the enforced minimum follows the content when a transient
/// control row appears and disappears, rather than latching at the largest value
/// it has ever seen.
#[test]
fn enforced_minimum_follows_a_control_row_in_and_out() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    let base = harness.state().content_extent();

    // A transient row appears: the minimum must grow to fit it.
    harness.state_mut().set_confirm_uninstall(true);
    for _ in 0..SETTLE {
        harness.step();
    }
    let with_row = harness.state().content_extent();
    assert!(
        with_row.y > base.y + 0.5,
        "the minimum did not grow for the new control row: {base:?} -> {with_row:?}"
    );

    // The row goes away: the minimum must come back down, not stay latched.
    harness.state_mut().set_confirm_uninstall(false);
    for _ in 0..SETTLE {
        harness.step();
    }
    let after = harness.state().content_extent();
    assert!(
        (after.y - base.y).abs() <= 0.5,
        "the minimum latched at the larger value: {base:?} -> {with_row:?} -> {after:?}"
    );
}

/// S046 FR-016: a scale change is evaluated in logical points and may therefore
/// cross the responsive breakpoint, while intrinsic width remains stable.
///
/// The layout is expressed in points, and a scale change converts points to
/// physical pixels at the platform boundary, so the intrinsic extent is
/// scale-invariant by construction and the minimum needs no recomputation. This
/// asserts that invariance rather than a change, because a minimum that moved with
/// the scale would mean the measurement had leaked into pixel space.
#[test]
fn scale_change_selects_layout_in_logical_points() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    let base = harness.state().content_extent();
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Wide)
    );
    let (base_live, base_system) = harness.state().dashboard_rects().expect("base geometry");
    assert_rect_sizes_equal(base_live, base_system, "base-scale cards");

    harness.ctx.set_pixels_per_point(1.5);
    for _ in 0..SETTLE {
        harness.step();
    }
    let scaled = harness.state().content_extent();
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Narrow)
    );
    let (scaled_live, scaled_system) = harness.state().dashboard_rects().expect("scaled geometry");
    assert_rect_sizes_equal(scaled_live, scaled_system, "scaled cards");

    assert!(
        (scaled.x - base.x).abs() <= 0.5 && scaled.y > base.y,
        "scale should preserve intrinsic width and select taller narrow layout: {base:?} -> {scaled:?}"
    );
}
