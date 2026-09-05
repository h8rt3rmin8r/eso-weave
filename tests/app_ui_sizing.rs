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
    resource_view, widgets, AppModel, DashboardLayout, ResourceTheme, DASHBOARD_WIDE_MIN,
};
use eso_weave::beacon::{self, BeaconPrefs, Environment, MANAGED_MARKER};
use eso_weave::config::{LoggingPrefs, Settings, Theme};
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
    let model = AppModel::new(
        Arc::new(engine),
        weave,
        fishing,
        Box::new(MockFishingSink::new()),
        Arc::new(Mutex::new(eso_weave::potion::AutoPotionController::new(
            eso_weave::potion::AutoPotionConfig::default(),
        ))),
        log,
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

#[test]
fn dashboard_stacks_narrow_and_uses_columns_wide() {
    let narrow = render_at(egui::vec2(760.0, 1200.0), SETTLE);
    assert_eq!(
        narrow.last_dashboard_layout(),
        Some(DashboardLayout::Narrow)
    );
    let (narrow_live, narrow_system) = narrow.dashboard_rects().expect("dashboard geometry");
    assert!(
        narrow_live.bottom() <= narrow_system.top() + 0.5,
        "narrow sections are not stacked in reading order: {narrow_live:?}, {narrow_system:?}"
    );

    let wide = render_at(egui::vec2(1200.0, 1000.0), SETTLE);
    assert_eq!(wide.last_dashboard_layout(), Some(DashboardLayout::Wide));
    let (wide_live, wide_system) = wide.dashboard_rects().expect("dashboard geometry");
    assert!(
        wide_live.right() <= wide_system.left() + 0.5,
        "wide sections are not in separate columns: {wide_live:?}, {wide_system:?}"
    );
    assert!(
        (wide_live.top() - wide_system.top()).abs() <= 0.5,
        "wide section tops are not aligned: {wide_live:?}, {wide_system:?}"
    );
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
fn dashboard_accessibility_tree_names_sections_and_dormant_resources() {
    let mut harness = harness_at(egui::vec2(760.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }

    harness.get_by_label("Live HUD");
    harness.get_by_label("Roll dodge");
    harness.get_by_label("System and State");
    harness.get_by_label("World state");
    harness.get_by_label("Travel");
    for label in [
        "Health: Game not active",
        "Stamina: Game not active",
        "Magicka: Game not active",
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
    assert!(harness.query_by_label("World state").is_none());
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
    for _ in 0..SETTLE {
        harness.step();
    }
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
    for _ in 0..SETTLE {
        harness.step();
    }
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
    let window_h = 1000.0;
    let mut harness = harness_at(egui::vec2(900.0, window_h));
    harness.step();
    harness.state_mut().set_log_panel_open(true);
    for _ in 0..SETTLE {
        harness.step();
    }

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
        huge.y <= 1120.0 + 1.0,
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

    harness.ctx.set_pixels_per_point(1.5);
    for _ in 0..SETTLE {
        harness.step();
    }
    let scaled = harness.state().content_extent();
    assert_eq!(
        harness.state().last_dashboard_layout(),
        Some(DashboardLayout::Narrow)
    );

    assert!(
        (scaled.x - base.x).abs() <= 0.5 && scaled.y > base.y,
        "scale should preserve intrinsic width and select taller narrow layout: {base:?} -> {scaled:?}"
    );
}
// Temporary diagnostic, appended to tests/app_ui_sizing.rs, removed after use.
