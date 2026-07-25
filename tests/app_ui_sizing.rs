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
use egui_kittest::Harness;

use eso_weave::app::ui::EsoWeaveApp;
use eso_weave::app::AppModel;
use eso_weave::config::{LoggingPrefs, Settings};
use eso_weave::fishing::{FishingConfig, FishingController, MockFishingSink};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::InputEngine;
use eso_weave::logging;
use eso_weave::weave::{WeaveConfig, WeaveEngine};

/// Builds an app over a default model. The channel senders are leaked into the
/// returned tuple so the receivers stay connected for the app's lifetime.
fn test_app() -> EsoWeaveApp {
    let (engine, _rx) = InputEngine::new(BindingTable::default(), 16);
    let weave = Arc::new(Mutex::new(WeaveEngine::new(WeaveConfig::default())));
    let fishing = Arc::new(Mutex::new(FishingController::new(FishingConfig::default())));
    let (_dispatch, log) = logging::build(&LoggingPrefs::default(), PathBuf::from("."));
    let model = AppModel::new(
        Arc::new(engine),
        weave,
        fishing,
        Box::new(MockFishingSink::new()),
        log,
        Settings::default(),
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
    let mut harness = harness_at(size);
    for _ in 0..frames {
        harness.step();
    }
    harness.into_state()
}

/// A harness at the given window size, with the bundled fonts installed exactly as
/// `main.rs` does at startup (the layout depends on them, so a bare harness would
/// measure the wrong text metrics).
fn harness_at(size: egui::Vec2) -> Harness<'static, EsoWeaveApp> {
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
        test_app(),
    )
}

/// Number of frames to settle the two-frame stability gate before reading state.
const SETTLE: usize = 6;

/// C1 (FR-001, FR-007): the intrinsic extent is a property of the content, so the
/// same application state measures the same at any window size.
///
/// This is the assertion that fails on v0.8.0, where the measured extent is the
/// window size less a constant on both axes (see research.md R1 confirmed).
#[test]
fn intrinsic_extent_is_independent_of_window_size() {
    let small = render_at(egui::vec2(700.0, 800.0), SETTLE).content_extent();
    let large = render_at(egui::vec2(1600.0, 1200.0), SETTLE).content_extent();

    assert!(
        (small.x - large.x).abs() <= 0.5,
        "intrinsic width tracks the window: {} at 700 wide, {} at 1600 wide",
        small.x,
        large.x
    );
    assert!(
        (small.y - large.y).abs() <= 0.5,
        "intrinsic height tracks the window: {} at 800 tall, {} at 1200 tall",
        small.y,
        large.y
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

/// C3 (FR-003, FR-019): the ratchet assertion. Across a monotonically shrinking
/// sequence of window sizes rendered as consecutive frames, which is what a single
/// continuous drag produces, the enforced minimum must never rise and must never
/// exceed the intrinsic extent.
///
/// This is the defect no arithmetic-only test can express, and the reason three
/// prior slices shipped green (see research.md R5).
#[test]
fn enforced_minimum_never_ratchets_during_a_shrink_gesture() {
    let mut harness = harness_at(egui::vec2(1600.0, 1200.0));
    for _ in 0..SETTLE {
        harness.step();
    }

    // The content does not change during the gesture, so the enforced minimum must
    // not change either. Asserting only "never rises" would pass on the defect,
    // because a window-derived minimum falls as the window falls.
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
            (sent.x - baseline.x).abs() <= 0.5 && (sent.y - baseline.y).abs() <= 0.5,
            "at window {width}x{height} the minimum moved to {sent:?} from {baseline:?}; \
             the content did not change, so the minimum tracks the window (the ratchet)"
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
    let mut harness = harness_at(egui::vec2(900.0, 800.0));
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
    harness.step();
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

    let mut height = 1100.0_f32;
    while height > 500.0 {
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

/// FR-011: a height committed past the boundary is clamped before it is stored,
/// so nothing out of range is persisted or restored.
#[test]
fn committed_log_height_is_clamped_before_it_is_stored() {
    let mut harness = harness_at(egui::vec2(900.0, 800.0));
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
        stored <= 800.0 - content_h + 0.5,
        "stored log height {stored} exceeds the boundary (window 800 - content {content_h})"
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
    assert!(
        huge.y <= 880.0 + 1.0,
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

/// FR-005: a display scale change does not disturb the enforced minimum.
///
/// The layout is expressed in points, and a scale change converts points to
/// physical pixels at the platform boundary, so the intrinsic extent is
/// scale-invariant by construction and the minimum needs no recomputation. This
/// asserts that invariance rather than a change, because a minimum that moved with
/// the scale would mean the measurement had leaked into pixel space.
#[test]
fn enforced_minimum_is_unchanged_by_a_scale_change() {
    let mut harness = harness_at(egui::vec2(1200.0, 1000.0));
    for _ in 0..SETTLE {
        harness.step();
    }
    let base = harness.state().content_extent();

    harness.ctx.set_pixels_per_point(1.5);
    for _ in 0..SETTLE {
        harness.step();
    }
    let scaled = harness.state().content_extent();

    assert!(
        (scaled.x - base.x).abs() <= 0.5 && (scaled.y - base.y).abs() <= 0.5,
        "the minimum moved with the display scale, so the measurement is in pixel \
         space rather than points: {base:?} -> {scaled:?}"
    );
}
