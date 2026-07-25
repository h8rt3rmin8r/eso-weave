# Implementation Plan: Application Interface Sizing Correctness

**Branch**: `main` (trunk-based) | **Date**: 2026-07-25 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/030-ui-sizing-correctness/spec.md`

## Summary

Close GitHub issues #12, #13, and #14 by making the enforced window minimum
intrinsic (a function of the laid-out controls only, never of the current window
size), hard-enforcing the live log pane's boundary and the settings modal's
extent, and making the `src/app/ui.rs` sizing glue verifiable for the first time
with headless rendered-frame tests.

The technical approach has one central move. Today the enforced minimum is
`ui.min_rect()` of the central panel, which includes full-width chrome and
therefore tracks the window; that single fact explains the ratcheted shrink
directly and feeds the wrong content height into the log pane boundary. Replacing
it with a measurement that unions only content-sized blocks makes the extent
window-independent, which in turn makes it stable by construction, which removes
the lagging stability gate that produces the per-gesture ratchet. The modal defect
is independent and is fixed by giving the modal an explicit height to match the
width it already sets.

The second half of the work is the reason this is a slice and not a patch. Three
prior fixes shipped with a fully green suite because the tested surface (pure
arithmetic) was never the broken surface (the glue to egui). `egui_kittest`
0.35 with `Harness::new_ui` drives a real egui layout with no window, no GPU, and
no `eframe::Frame`, so the glue becomes testable at last.

## Technical Context

**Language/Version**: Rust (edition and toolchain per `rust-toolchain.toml`).

**Primary Dependencies**: `egui`/`eframe` 0.35. New dev-dependency
`egui_kittest` 0.35 with `default-features = false` (zero features are enabled by
default; none of `wgpu`, `x11`, `snapshot`, or `eframe` is required).

**Storage**: The persisted `log_panel_height` in the `ui` settings section and the
persisted window geometry in session state, both unchanged in shape.

**Testing**: `cargo test --all --locked`. Existing pure-helper suites
(`tests/app_window_sizing.rs`, `tests/app_view_model.rs`, `tests/app_log_view.rs`)
are retained unchanged; a new `tests/app_ui_sizing.rs` asserts rendered geometry.

**Target Platform**: Windows 10/11 x64 and Linux x64.

**Project Type**: Single Rust crate, desktop GUI application.

**Performance Goals**: No additional per-frame allocation. The min-size viewport
command is sent only when the intrinsic extent changes, which is strictly fewer
sends than today.

**Constraints**: No control is ever clipped; the log pane never covers an
interactive control on any frame; all sizing behavior is verifiable at the desk
with no game and no display.

**Scale/Scope**: Two stacked panes (central plus optional bottom log) and one
modal, in one window.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Traces to master specification section 11
  through build plan `docs/plans/plan-009.md` slice 030; the full spec-kit
  sequence is being run.
- **II. Safety-Critical Surfaces Are Sacrosanct**: PASS, not engaged. This feature
  touches no input-engine, beacon-manager, or fishing surface. No safety-critical
  test is modified, weakened, or skipped.
- **III. Test-First With Explicit Seams**: PASS, and materially advanced. The
  feature's central deliverable is a new seam (`frame_ui`, taking only
  `&mut egui::Ui`) that makes a previously untestable layer testable. Every
  rendered-geometry assertion is written red before the fix that satisfies it.
- **IV. CI Parity Before Every Commit**: PASS. `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cargo test --all --locked` run in the foreground before the commit. Note that
  adding a dev-dependency changes `Cargo.lock`, and `--locked` refuses to update
  it, so the lockfile must be refreshed by the task that adds the dependency and
  committed with it.
- **V. Bounded Scope: Outside The Game**: PASS. Desk-only UI work; no game
  process, memory, network, or addon surface is involved.
- **Text hygiene**: PASS. All new files are UTF-8 without BOM, LF, and free of
  em-dashes and en-dashes.
- **Pinned artifacts**: Not engaged. `Cargo.toml` is not a pinned artifact, but
  the new dev-dependency is recorded as a dated decision in `CHANGELOG.md`
  following the `ureq` precedent from slice 018.

Post-design re-check: PASS, unchanged. The design adds one dev-dependency and one
inherent method; it introduces no new module, no new crate, and no workspace.

## Project Structure

### Documentation (this feature)

```text
specs/030-ui-sizing-correctness/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── sizing-contracts.md
├── checklists/
│   ├── requirements.md  # Spec quality checklist
│   └── sizing.md        # Sizing requirements quality checklist
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
src/
├── app/
│   ├── mod.rs           # Pure sizing helpers; gains the intrinsic-extent type
│   └── ui.rs            # The glue under repair; gains the frame_ui seam
└── main.rs              # Boot minimum, unchanged in value

tests/
├── app_window_sizing.rs # Existing pure-helper suite, retained
├── app_view_model.rs    # Existing modal_extent tests, retained
├── app_log_view.rs      # Existing clamp tests, retained
└── app_ui_sizing.rs     # New: rendered-geometry assertions
```

**Structure Decision**: Single-crate layout unchanged. Edits are localized to
`src/app/ui.rs` and `src/app/mod.rs`, plus one new test file. No new module is
introduced; the intrinsic-extent accumulator is a small type in `src/app/mod.rs`
alongside the existing pure helpers it feeds.

## Key design decisions

**D1. Measure content-sized blocks, not the panel.** The enforced extent becomes
the union of the rectangles of the blocks that size to their content (the status
grid, the weapon bar row, the skills grid, the button rows, and the conditional
uninstall confirmation row), accumulated as each block is laid out. Full-width
chrome contributes height only (FR-007). Rejected: keeping `ui.min_rect()` and
subtracting a chrome allowance, because the subtraction would be a guess that
drifts as the layout changes, and it leaves the extent window-derived.

**D2. The stability gate stays, and becomes a no-op for resizes.** An intrinsic
extent does not change while the window is dragged, so `measurement_stable`
naturally reports stable throughout a gesture and `content_min_size` returns the
measured extent. The gate is retained unchanged because it still does useful work
on the first frames and across theme and scale changes. Rejected: deleting the
gate, which would let a transient first-frame layout latch the minimum, the exact
defect issue #8 reported.

**D3. Send the minimum only on change of the intrinsic extent.** `MinInnerSize` is
pushed when the extent value changes, never in response to window geometry
(FR-005). This is what makes a mid-gesture relaxation impossible, which is the
mechanism the ratchet depends on under the leading hypothesis.

**D4. Re-clamp the committed log height.** The height egui reports after the drag
is clamped before it is stored and before it is persisted, and the clamped value
is forced on the next frame (FR-011). Combined with D1 feeding a correct content
height, this makes the boundary hold under drag, resize, and both.

**D5. Constrain the modal's area to an explicit rectangle.** The modal receives an
explicit height to match the width it already sets, so its rendered rectangle
matches `modal_extent` on both axes (FR-014). `modal_extent` itself is correct,
already tested, and unchanged.

**D6. `frame_ui` is the testability seam.** The body of `eframe::App::ui` moves to
an inherent `fn frame_ui(&mut self, ui: &mut egui::Ui)`; the trait method becomes a
one-line delegation. The `eframe::Frame` argument is already unused, so nothing is
lost and `Harness::new_ui` can drive the real frame body. Rejected: `Harness`'s
eframe integration, which would pull the `eframe` feature and a windowing stack
into the test build for no gain.

**D7. Simulate a resize gesture as successive frames at shrinking screen sizes.**
A gesture is modelled by stepping the harness with a progressively smaller
`screen_rect`. The ratchet assertion is that the enforced minimum never rises
above the intrinsic extent at any step of the sequence, which is the property the
per-gesture ratchet violates and which no arithmetic-only test can express.

**D8. Instrument before fixing.** The `WM_GETMINMAXINFO` gesture-latch explanation
for the ratchet is inferred, not observed. The first implementation task records
the measured extent, the computed minimum, and the frames on which a minimum is
sent during a real resize, and confirms or replaces the explanation. If the
intrinsic measurement alone does not remove the ratchet, that instrumentation
identifies the remainder rather than leaving a fourth speculative fix to ship.

**D9. FR-017 is measured, not assumed.** Whether the modal's configured maximum is
large enough to show half the settings body is a measurement taken during
implementation. The maximum is raised only if the measurement shows the outcome is
otherwise unreachable, and any new value is recorded as a decision in
`CHANGELOG.md`. This is the only constant from the previous slice this feature may
change.

## Complexity Tracking

No constitution violations to justify. The one addition to the dependency graph
(`egui_kittest`, dev-only, default features off) is the direct mechanism for
Principle III on a layer that has never satisfied it, and is recorded as a dated
decision.
