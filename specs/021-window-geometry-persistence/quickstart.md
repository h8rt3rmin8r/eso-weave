# Quickstart: Window Geometry Persistence (Manual Validation)

The GUI capture and restore path cannot be exercised headlessly, so it is
validated manually against the running app. The pure sanitize helper and the
migration are covered by `cargo test`.

## Automated checks

```
cargo test --all --locked
```

Confirm the new tests pass:

- `sanitize_geometry` size clamping, off-screen position drop, no-bounds trust,
  and maximized passthrough.
- Session migration: a `state.json` without a `window` key loads as `None`; a
  round trip preserves geometry; `CURRENT_STATE_VERSION` is 3.

## Manual validation (Windows, primary monitor)

1. Delete or note `%APPDATA%\eso-weave\state.json`. Launch the app: it opens at
   the default size (600x720). (First-launch scenario, FR-007.)
2. Move the window to a new position and resize it. Close the app.
3. Relaunch: the window reopens at the same position and size. (US1, FR-001/002.)
4. Resize the window, then immediately close (within a fraction of a second).
   Relaunch: the latest size is restored, not the previous one. (US1 scenario 3,
   FR-008.)
5. Maximize the window, close, relaunch: it reopens maximized. Then restore it to
   a normal size, move it, close, relaunch: it reopens at that normal size, not
   maximized. (US2, FR-006 maximized handling.)
6. Snap the window to half the screen (drag to an edge), close, relaunch: it
   reopens at that snapped size and position (treated as a normal geometry).

## Manual validation (multi-monitor)

7. Move the window onto a secondary monitor, close, relaunch with the same layout:
   it reopens on the secondary monitor. (US3, FR-004.)
8. With the window on the secondary monitor, close the app, then disconnect (or
   disable) that monitor, and relaunch: the window opens fully visible on a
   remaining monitor rather than off-screen. (Edge case, FR-005, SC-003.)

## Manual validation (state hygiene)

9. Open `%APPDATA%\eso-weave\state.json` and confirm it contains a `window` object
   with `x`, `y`, `width`, `height`, and (when applicable) `maximized`, is UTF-8
   without a BOM, uses LF line endings, and ends with a trailing newline.
10. Hand-edit `state.json` to a pre-feature shape (remove the `window` key, set
    `schema_version` to 2). Relaunch: the app loads without error and opens at the
    default geometry. (FR-007.)

## Manual validation (Linux)

11. On X11, repeat steps 2-3: size and position restore. On Wayland, size and
    maximized restore; absolute position is not restored (a known platform
    limitation, documented in research.md), and no error is surfaced.

## Regression

12. Confirm the suspend and fishing intents still restore across a relaunch (set
    fishing on or suspend, close, relaunch, observe the state restored). (FR-009.)
