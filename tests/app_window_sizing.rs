//! Pure-helper tests for window sizing, log-pane geometry, and control heights
//! (feature 027). These exercise the sizing math without a live window, per the
//! contract in specs/027-window-sizing-hardening/contracts/ui-window-sizing.md.

use eso_weave::app::{
    clamp_log_height, content_min_size, log_min_height, reduced_interact_height, LOG_FRAME_MARGIN,
    LOG_MIN_LINES,
};

// US1 / issue #4: content_min_size (contract C1).

#[test]
fn content_min_size_floors_per_dimension() {
    let floor = (480.0, 420.0);
    // Below the floor in both dimensions returns the floor.
    assert_eq!(content_min_size((300.0, 200.0), floor), floor);
    // Above the floor in both dimensions returns the measured size.
    assert_eq!(content_min_size((600.0, 700.0), floor), (600.0, 700.0));
    // Dimensions are independent: wide but short takes width from measured and
    // height from the floor.
    assert_eq!(content_min_size((600.0, 200.0), floor), (600.0, 420.0));
}

#[test]
fn content_min_size_running_max_never_shrinks() {
    let floor = (480.0, 420.0);
    let a = content_min_size((600.0, 700.0), floor);
    // A later transient under-measure, folded through the caller's running max,
    // must never shrink the floor.
    let b = content_min_size((500.0, 500.0), floor);
    let running = (a.0.max(b.0), a.1.max(b.1));
    assert_eq!(running, (600.0, 700.0));
}

// US2 / issue #5: log_min_height (contract C2) and clamp_log_height (contract C3).

#[test]
fn log_min_height_shows_six_lines_plus_margins() {
    let row = 14.0;
    assert_eq!(
        log_min_height(row),
        LOG_MIN_LINES * row + 2.0 * LOG_FRAME_MARGIN
    );
    assert!(log_min_height(row) >= 6.0 * row);
    // Strictly increasing in the row height (a larger font yields a taller floor).
    assert!(log_min_height(20.0) > log_min_height(14.0));
}

#[test]
fn clamp_log_height_bounds() {
    let window = 800.0;
    let row = 14.0;
    let content_min = 420.0;
    let min = log_min_height(row);
    let max = window - content_min;

    // Below the six-line minimum is raised to it.
    assert_eq!(clamp_log_height(0.0, window, row, content_min), min);
    // Above the max is lowered so the pane top cannot cross the Skills area.
    assert!((clamp_log_height(10_000.0, window, row, content_min) - max).abs() < 0.01);
    // A value inside the range passes through unchanged.
    assert_eq!(clamp_log_height(200.0, window, row, content_min), 200.0);
    // Degenerate: a window too short for both content and a six-line log collapses
    // the pane to its readable minimum (min wins) rather than covering controls.
    assert_eq!(clamp_log_height(50.0, 100.0, row, content_min), min);
}

// US4 / issue #7: reduced_interact_height (contract C6).

#[test]
fn reduced_interact_height_reduces_but_never_clips() {
    // About a 20 percent reduction when the font leaves room.
    assert!((reduced_interact_height(22.0, 14.0) - 17.6).abs() < 0.001);
    // Never shorter than the text line height (a large font raises the floor).
    let tall_font = 30.0;
    assert!(reduced_interact_height(22.0, tall_font) >= tall_font);
    // Monotonic in the base height.
    assert!(reduced_interact_height(30.0, 14.0) > reduced_interact_height(22.0, 14.0));
}
