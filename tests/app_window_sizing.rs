//! Pure-helper tests for window sizing, log-pane geometry, and control heights.
//! These exercise the sizing math without a live window. The stable-measured
//! minimum, the proportional log split, and the open-window drag reserve are the
//! slice 029 rebuild (issue #8, specs/029-window-sizing-rebuild/contracts/sizing.md);
//! the control-height and log-minimum helpers date to feature 027.

use eso_weave::app::{
    clamp_log_height, content_min_size, log_min_height, measurement_stable, open_log_reserve,
    reduced_interact_height, split_log_height, LOG_FRAME_MARGIN, LOG_MIN_LINES,
};

// US1 / issue #8: content_min_size (boot floor until stable, then measured wins).

#[test]
fn content_min_size_uses_boot_floor_until_stable() {
    let floor = (480.0, 420.0);
    // Before a stable measurement, the boot floor applies regardless of the
    // measured value, so nothing is clipped during the initial frames.
    assert_eq!(content_min_size((300.0, 340.0), floor, false), floor);
    assert_eq!(content_min_size((600.0, 700.0), floor, false), floor);
}

#[test]
fn content_min_size_measured_wins_once_stable() {
    let floor = (480.0, 420.0);
    // Once stable, the measured extent sets the minimum per dimension, even when
    // smaller than the boot floor (the ~20 percent dead band is gone).
    assert_eq!(
        content_min_size((300.0, 340.0), floor, true),
        (300.0, 340.0)
    );
    assert_eq!(
        content_min_size((600.0, 700.0), floor, true),
        (600.0, 700.0)
    );
    assert_eq!(
        content_min_size((600.0, 340.0), floor, true),
        (600.0, 340.0)
    );
}

#[test]
fn content_min_size_can_shrink_no_permanent_latch() {
    let floor = (480.0, 420.0);
    // A larger stable measurement does not latch: a later smaller stable
    // measurement yields the smaller minimum (the running max is gone).
    let large = content_min_size((600.0, 700.0), floor, true);
    let small = content_min_size((500.0, 360.0), floor, true);
    assert_eq!(large, (600.0, 700.0));
    assert_eq!(small, (500.0, 360.0));
}

#[test]
fn measurement_stable_needs_two_consecutive_close_frames() {
    // The first measurement (no previous) is never stable.
    assert!(!measurement_stable(None, (400.0, 340.0), 0.5));
    // Two frames equal within epsilon are stable.
    assert!(measurement_stable(
        Some((400.0, 340.0)),
        (400.2, 339.8),
        0.5
    ));
    // A frame that differs beyond epsilon in either dimension is not stable.
    assert!(!measurement_stable(
        Some((400.0, 340.0)),
        (401.0, 340.0),
        0.5
    ));
    assert!(!measurement_stable(
        Some((400.0, 340.0)),
        (400.0, 342.0),
        0.5
    ));
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

// US2 / issue #8: open_log_reserve and resizability at the minimum open height.

#[test]
fn open_log_reserve_adds_one_row_of_drag_room() {
    let row = 14.0;
    assert_eq!(open_log_reserve(row), log_min_height(row) + row);
    // Strictly greater than the six-line minimum by exactly one row.
    assert!(open_log_reserve(row) > log_min_height(row));
    assert!((open_log_reserve(row) - log_min_height(row) - row).abs() < 0.001);
}

#[test]
fn log_pane_resizable_at_minimum_open_height() {
    let row = 14.0;
    let content_h = 340.0;
    let min = log_min_height(row);
    // At the enforced minimum open-window height the pane has one row of range, so
    // its max strictly exceeds its min (never frozen).
    let window = content_h + open_log_reserve(row);
    let max = (window - content_h).max(min);
    assert!(max > min);
    assert!((max - (min + row)).abs() < 0.001);
    // For any window at least content + six-line minimum, the range stays valid so
    // the window can shrink with the log compressing toward six lines.
    let tight = content_h + min;
    assert!((tight - content_h).max(min) >= min);
}

// US3 / issue #8: split_log_height (proportional window-growth split).

#[test]
fn split_log_height_shares_delta_by_live_fraction() {
    let content_h = 300.0;
    let log_min = log_min_height(14.0);
    // Log occupies 200 of a 1000 window (20 percent). Growing the window by 200
    // gives the log ~20 percent of the added height (~40), central takes the rest.
    let new_log = split_log_height(1000.0, 1200.0, 200.0, content_h, log_min);
    assert!((new_log - 240.0).abs() < 0.001);
    // Shrinking symmetrically removes ~20 percent from the log.
    let shrunk = split_log_height(1000.0, 800.0, 200.0, content_h, log_min);
    assert!((shrunk - 160.0).abs() < 0.001);
}

#[test]
fn split_log_height_clamps_to_min_and_available() {
    let content_h = 300.0;
    let log_min = log_min_height(14.0);
    // A large shrink cannot push the log below its six-line minimum.
    let clamped_low = split_log_height(1000.0, 380.0, 200.0, content_h, log_min);
    assert!(clamped_low >= log_min);
    // The log can never exceed the space above the content (window - content).
    let clamped_high = split_log_height(1000.0, 5000.0, 900.0, content_h, log_min);
    assert!(clamped_high <= 5000.0 - content_h);
    // A non-positive previous window height falls back to clamping the current.
    let fallback = split_log_height(0.0, 800.0, 200.0, content_h, log_min);
    assert_eq!(fallback, 200.0_f32.clamp(log_min, 800.0 - content_h));
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
