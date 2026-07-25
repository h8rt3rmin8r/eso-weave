# Quickstart: Application Interface Sizing Correctness

Validation guide for slice 030. Proves issues #12, #13, and #14 are fixed, and
absorbs the still-owed manual validation from slice 029. This is a desk-only UI
change; no game is required.

## Prerequisites

- Repo on `main`, Rust toolchain per `rust-toolchain.toml`.
- A display large enough to reach 1600 by 1200 points for the modal checks. If
  the desk display cannot reach that, note it and validate at the largest size
  available.

## Automated verification (CI parity)

Run in the foreground and watch to completion:

```bash
cargo fmt --all -- --check
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

```bash
cargo test --all --locked
```

Expected: all green. The suites relevant to this slice are:

- `tests/app_ui_sizing.rs` (new, rendered geometry, contracts C1 through C6):
  - the intrinsic extent is identical at two different window sizes (C1),
  - the enforced minimum equals the intrinsic extent, plus the log reserve while
    the log is open (C2),
  - across a monotonically shrinking sequence of window sizes rendered as
    consecutive frames, the enforced minimum never rises and never exceeds the
    intrinsic extent at any step (C3, the ratchet assertion),
  - across a splitter drag past the boundary, the pane's top edge is at or below
    the central content's bottom edge on every frame, including the frame of the
    gesture (C4),
  - a pane height committed past the boundary is clamped before it is stored and
    before it is persisted (C4),
  - the rendered modal rectangle matches the growth rule within one point at a
    small, a medium, and a very large window (C5),
  - at the modal's maximum, at least half the settings body is visible (C6).
- `tests/app_window_sizing.rs`, `tests/app_view_model.rs`, `tests/app_log_view.rs`
  (existing, unchanged): the pure helper contracts from slice 029 still hold.

### Proving the new checks actually bite

The point of this slice is that a green suite must mean something. Confirm it
once, by hand, before accepting the work:

1. Temporarily revert the measurement to `ui.min_rect().size()`.
2. Run `cargo test --all --locked`.
3. Expected: the C1, C2, and C3 assertions fail.
4. Restore the fix and confirm green again.

Repeat the same exercise for the log boundary clamp and for the modal height
enforcement. If any of the three reverts leaves the suite green, the check for
that defect is not doing its job and the slice is not done (FR-021, SC-005).

### Result recorded during implementation (2026-07-25)

Each check was observed failing against the unfixed code before its fix was
written, which is the same evidence a revert produces and is what test-first
discipline generates by construction. The observed failures were:

- C1 (intrinsic extent): `intrinsic width tracks the window: 668 at 700 wide,
  1584 at 1600 wide`.
- C3 (the ratchet): `at window 1560x1160 the minimum moved to [480.0 420.0] from
  [1568.0 1168.0]; the content did not change, so the minimum tracks the window`.
- C4 (log overlap): `log pane top 418 is above the content bottom 428.21875, so it
  covers 10.21875 points of interactive controls`.
- C5 (modal extent): `modal did not grow in height: [606.0 400.0] -> [881.0
  400.0]`, the height frozen at its 400 point floor while the width grew.
- C6 (modal body): `only 348 of 1612.375 points of settings body are visible (22
  percent)`.

One check was found NOT to bite and was strengthened before the fix landed: the
first version of the ratchet assertion compared against the extent measured at the
starting window size, which a window-derived minimum satisfies trivially because
it falls as the window falls. It was rewritten to assert the minimum is constant
across the gesture, and only then failed on the defect. That is exactly the
failure mode FR-021 exists to catch, caught once here.

## Manual desk validation (run the app)

```bash
cargo run
```

### This slice's reproductions

- **MV-6 (issue #12, the reported reproduction)**: resize the window to something
  very large on both axes, quit, and relaunch so the oversized geometry is
  restored. Now drag one edge inward in a single continuous gesture without
  releasing the mouse. Expected: the window shrinks all the way to the content
  minimum in that one gesture. Repeat on the other axis, and once more on a
  corner for both axes at once. Expected: no gesture locks part way, and no
  repeated grab-and-drag is needed.
- **MV-7 (issue #13, the reported reproduction)**: open the live log from the
  View menu, then drag the pane's top splitter upward hard and fast. Expected:
  the pane stops dead before the Skills rows and no control is ever covered.
  Repeat at the minimum window size, immediately after enlarging the window, and
  immediately after shrinking it. Expected: the same stop every time.
- **MV-8 (issue #13, persistence)**: drag the splitter to the boundary, quit, and
  relaunch. Expected: the restored pane is inside the boundary on the very first
  frame, with no flash of overlap.
- **MV-9 (issue #14, the reported reproduction)**: enlarge the window to a very
  large size and open Settings. Expected: the modal is at or near its maximum
  size, and the body shows at least half the settings content without scrolling.
- **MV-10 (issue #14, growth)**: with Settings open, grow the window gradually
  from small to large. Expected: the modal grows continuously on both axes, by a
  progressively smaller share of each addition, and stops at its maximum. At the
  smallest window it still fits inside the window with a margin on every edge.

### Absorbed from slice 029 (still owed)

- **MV-1**: with the log closed, shrink the window to its minimum. Expected: the
  last control sits near the bottom edge with no large empty band, and the
  minimum tracks the real content.
- **MV-2**: open the live log toggle and drag the pane's top splitter upward.
  Expected: the pane grows, and there is no empty reserved band above the log.
- **MV-3**: with the log open, enlarge the window vertically. Expected: the extra
  height is shared, so the log grows by roughly its share rather than zero.
- **MV-4**: note the window height, open the log, enlarge the pane, then close
  the log. Expected: the window returns to its original height with no residual
  empty band.
- **MV-5**: with the log open, shrink the window toward its minimum. Expected:
  the log compresses toward six lines, the window can still shrink, and at the
  minimum the pane still has a small drag range.

### Regression sweep

- **MV-11**: toggle the theme between dark and light with the window at its
  minimum. Expected: the minimum adjusts to the new layout and nothing is
  clipped.
- **MV-12**: trigger the uninstall confirmation row so a control row appears,
  then dismiss it. Expected: the window grows to fit the row if needed, and the
  enforced minimum drops again afterward while the window keeps its size.

## Rollback

The change is confined to the app UI sizing glue, its new test file, and the
`egui_kittest` dev-dependency. Reverting the feature commit restores v0.8.0
behavior exactly; no persisted data shape changes, so no migration or cleanup is
needed on rollback.
