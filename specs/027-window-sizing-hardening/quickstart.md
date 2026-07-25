# Quickstart: Validating UI Window-Sizing and Layout Hardening

This is the desk validation guide for slice 027. All scenarios run without the
game. Automated coverage lives in `tests/app_window_sizing.rs` and
`tests/app_session_state.rs`; the manual scenarios below cover the viewport and
visual behavior that unit tests cannot.

## Prerequisites

- Toolchain 1.96.0 (`rust-toolchain.toml`), Windows or Linux desktop.
- No PixelBeacon deploy and no running game are required.

## Automated checks (run first)

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all green. The new/updated pure-helper tests
(`content_min_size`, `log_min_height`, updated `clamp_log_height`) and the
`SaveScheduler` notify-flag tests pass. See the contract
`contracts/ui-window-sizing.md` for the exact clauses each test covers.

## Manual visual scenarios

Launch the app:

```bash
cargo run
```

### Scenario 1 - Minimum size fits content (US1 / FR-001, FR-002, FR-003)

1. With the live log viewer off, drag the bottom edge up until the window stops.
   - Expected: the bottom Skills grid row is fully visible; nothing is clipped.
2. Drag a side edge in until the window stops.
   - Expected: the Pixel Beacon row (Install / Update / Uninstall) is fully
     visible; no horizontal clipping.
3. Repeat in both light and dark themes (View/Settings theme).
   - Expected: identical, no clipping in either theme (SC-001, SC-002).

### Scenario 2 - Live log viewer behaves (US2 / FR-004..FR-008)

1. From the default window size with the viewer off, enable the live log viewer
   (View menu).
   - Expected: the window grows in height; no existing control is covered
     (SC-003).
2. Observe the pane at its minimum height.
   - Expected: at least six lines of log text are visible (SC-003).
3. Compare the window's minimum width to the viewer-off state (drag width in).
   - Expected: the minimum width is wider while the viewer is open (FR-006).
4. Drag the resize bar above the pane upward as far as it goes.
   - Expected: the pane top stops before the Skills area and never covers a
     Skills control (SC-004).
5. Disable the live log viewer.
   - Expected: the window shrinks back by the amount it grew (height-neutral
     toggle; FR-008).

### Scenario 3 - Save confirmation only on real changes (US3 / FR-009, FR-010, FR-013)

1. Move the window; resize the window; with the viewer open, drag the log-pane
   divider.
   - Expected: no "Settings saved" confirmation appears for any of these
     (SC-005).
2. Toggle Status, Fishing, or a Skill Enabled control, or edit a form field.
   - Expected: the "Settings saved" confirmation appears exactly once (SC-005).
3. Quit and relaunch.
   - Expected: the window position and log-pane height are restored as last left
     (persistence unchanged; FR-013).

### Scenario 4 - Shorter controls, legible text (US4 / FR-011, FR-012)

1. Inspect the buttons, toggle switches, and dropdown menus.
   - Expected: each is shorter than before by a single consistent amount of up to
     ~20 percent (SC-006).
2. Read every control label in both light and dark themes.
   - Expected: all text fully legible, none clipped or overflowed (SC-006). The
     final reduction figure is recorded in `plan.md` (D1) and the CHANGELOG.

## Optional screenshot capture

For a visual record on Windows, capture the focused window with the PowerShell
`CopyFromScreen` GUI-capture approach (the app is a normal desktop window). This
is optional evidence for the pre-push halt, not a gate.

## Done gate

All four scenarios pass in both themes, and the automated checks are green, with
no clipped control at any allowed window size and exactly-once toast behavior for
real settings changes.
