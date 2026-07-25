# Implementation Plan: Window Sizing Model Rebuild

**Branch**: `main` (trunk-based) | **Date**: 2026-07-25 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/029-window-sizing-rebuild/spec.md`

## Summary

Rebuild the window minimum-size and live-log-pane sizing model that v0.7.0 (slice
027) left broken (issue #8). The permanent running-max "content min" seeded at the
boot floor is replaced with a stable-measured model (measured wins once stable, can
shrink); the log pane's available range is computed against the true content height
(no phantom band); window growth is split proportionally between the central pane
and the log pane; and open/close is height-neutral by the pane's actual height. All
sizing math becomes pure, unit-testable helpers in `src/app/mod.rs`; the egui frame
closure in `src/app/ui.rs` is thin glue.

## Technical Context

**Language/Version**: Rust (edition per `rust-toolchain.toml`).

**Primary Dependencies**: `egui`/`eframe` 0.35 (immediate-mode GUI; the bottom log
panel and the viewport min-size command). No new dependencies.

**Storage**: The persisted `log_panel_height` lives in the `ui` settings section
(`UiPrefs`), unchanged in shape.

**Testing**: `cargo test --all --locked`; the sizing helpers are pure and covered
by `tests/app_window_sizing.rs`. Desk (no game) manual validation via quickstart.

**Target Platform**: Windows 10/11 x64 and Linux x64 (same egui window model).

**Project Type**: Single Rust crate, desktop GUI app.

**Performance Goals**: No per-frame allocation added; the min-size viewport command
stays deduped so it is sent only when the target changes.

**Constraints**: No control is ever clipped; the log pane never covers the
controls; text hygiene (UTF-8, LF, no em/en dashes).

**Scale/Scope**: Two stacked panes (central + optional bottom log) in one window.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: Full spec-kit sequence; artifacts under
  `specs/029-window-sizing-rebuild/`. PASS.
- **II. Safety-Critical Surfaces Are Sacrosanct**: No safety-critical surface is
  touched (no input engine, no beacon uninstall/discovery, no pixel-bus, no
  fishing). PASS (not applicable).
- **III. Test-First With Explicit Seams**: All new sizing logic is pure functions
  in `src/app/mod.rs` tested in `tests/app_window_sizing.rs` before the egui glue.
  PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), and
  `cargo test --all --locked` run in the foreground before commit. PASS.
- **V. Bounded Scope: Outside The Game**: UI-only; nothing crosses the game
  boundary. PASS.

No violations; Complexity Tracking not required.

## Project Structure

### Documentation (this feature)

```text
specs/029-window-sizing-rebuild/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (sizing state + helper signatures)
├── quickstart.md        # Phase 1 output (desk reproduction of issue #8)
├── contracts/
│   └── sizing.md        # Phase 1 output: the pure sizing-helper contract
├── checklists/
│   ├── requirements.md  # Spec quality checklist
│   └── sizing.md        # Requirements-quality checklist
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
src/app/
├── mod.rs   # pure helpers: change content_min_size(measured, boot, stable);
│            #   add measurement_stable, split_log_height, open_log_reserve;
│            #   keep log_min_height, clamp_log_height
└── ui.rs    # frame glue: stable-measured content extent (prev_measured,
             #   content_extent), proportional-split log panel, height-neutral
             #   open/close; new state prev_window_h
src/main.rs  # MIN_SIZE / BOOT_MIN_SIZE stay the pre-measurement floor only
tests/app_window_sizing.rs  # update content_min_size tests; add stability,
                            #   split, open-reserve, resizability tests
```

**Structure Decision**: Single-crate layout unchanged; edits localized to the two
`src/app` files, `src/main.rs`, and the one test file.

## Key design decisions

1. **Stable-measured minimum replaces the permanent running max.** `content_min_size`
   gains a `stable: bool` and returns the boot floor per dimension until stable,
   then the measured extent (no longer maxed with the floor). Stability
   (`measurement_stable`) is two consecutive measurements equal within ~0.5pt. New
   `EsoWeaveApp` state `prev_measured: Option<Vec2>` and `content_extent: Vec2`
   replace the "never shrinks" `content_min`.
2. **Log max against true content height.** The log pane's max is
   `window_h - content_extent.y` (via `clamp_log_height`), which is now the real
   content height, so no phantom band and the pane is resizable at normal sizes.
3. **Proportional split (DECIDED).** `split_log_height(prev_window_h, window_h,
   log_h, content_h, log_min)` moves the log by `Fl = log_h / prev_window_h` of the
   height delta, clamps to `[log_min, max(window_h - content_h, log_min)]`; the
   central pane is the remainder. New state `prev_window_h: Option<f32>`. egui
   mechanism: on a frame where the window height changed, drive the panel to the
   split-computed height; otherwise use the full range and read back the user's
   drag. The split math is pure and tested regardless of the egui call chosen.
4. **Resizable-at-minimum + compressible (item 5).** Enforced minimum open-window
   height = `content_h + open_log_reserve(row_h)` where `open_log_reserve =
   log_min_height(row_h) + row_h`. At that minimum the log ranges
   `[log_min, log_min + row_h]` (max > min), and the window can still shrink with
   the log compressing toward six lines.
5. **Height-neutral open/close (item 4).** Open grows by the shown log height
   (persisted, clamped); close shrinks by the pane's actual realized height.
   Persisted `log_panel_height` is the single source of truth.

See [research.md](research.md) for rationale/alternatives and
[contracts/sizing.md](contracts/sizing.md) for the exact helper signatures and
invariants.

## Complexity Tracking

No constitution violations; no entries.
