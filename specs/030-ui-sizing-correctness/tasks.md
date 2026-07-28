---

description: "Task list for slice 030, application interface sizing correctness"
---

# Tasks: Application Interface Sizing Correctness

**Input**: Design documents from `/specs/030-ui-sizing-correctness/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Test tasks are REQUIRED for this feature. Constitution principle III
mandates test-first, and FR-018 through FR-021 make the automated checks the
feature's primary deliverable. Every rendered-geometry assertion is written red
before the fix that satisfies it.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Exact file paths are included in every task

## Path Conventions

Single Rust crate: `src/` and `tests/` at the repository root.

## Two non-negotiable ordering rules

1. **Diagnose before fixing.** T004 confirms or replaces the ratchet root-cause
   hypothesis with real instrumentation. No fix task may start before it
   completes (plan.md decision D8). Three prior slices shipped speculative fixes.
2. **Red before green.** Every test task in a story phase precedes the
   implementation task it constrains, and must be observed failing against the
   current code before that implementation begins.

---

## Phase 1: Setup

**Purpose**: Make the test harness available.

- [x] T001 Add the test harness with
      `cargo add egui_kittest --dev --no-default-features`, so `Cargo.lock` is
      refreshed at the same time. Adding the dependency by hand would leave the
      lockfile stale and every later `cargo test --all --locked` would fail on
      the `--locked` flag alone (analyze finding D1). Then add a comment in
      `Cargo.toml` recording that zero features are enabled by default, so no
      GPU, windowing, or image stack is pulled into the test build.
- [x] T002 Run `cargo test --all --locked` in the foreground to confirm the
      refreshed lockfile is accepted, the new dev-dependency resolves, and the
      existing suites are green before any change. Confirm `Cargo.lock` shows
      `egui_kittest` and that both it and `Cargo.toml` are staged together.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: The testability seam and the diagnosis. Everything else depends on
these. No user story work may begin until Phase 2 is complete.

- [x] T003 Extract the body of `impl eframe::App for EsoWeaveApp::ui` in
      `src/app/ui.rs` into an inherent `fn frame_ui(&mut self, ui: &mut egui::Ui)`
      on `EsoWeaveApp`, leaving the trait method as a one-line delegation. The
      `_frame: &mut eframe::Frame` argument is already unused, so this is a pure
      move with no behavior change (contract C7). Verify by running
      `cargo test --all --locked` and confirming no test changes behavior.
- [x] T004 Instrument a real resize and record the findings in
      `specs/030-ui-sizing-correctness/research.md` under a new "R1 confirmed"
      subsection. Add temporary `tracing` output in `src/app/ui.rs` for: the
      measured extent, the computed enforced minimum, and every frame on which
      `ViewportCommand::MinInnerSize` is sent. Run `cargo run`, drag the window
      smaller on each axis, and capture the log. Confirm or replace the
      `WM_GETMINMAXINFO` gesture-latch explanation for the ratchet, and confirm
      that the measured width equals the window width. Remove the temporary
      instrumentation before the phase ends. **Blocks every fix task.**
- [x] T005 Create `tests/app_ui_sizing.rs` with a shared helper that builds an
      `EsoWeaveApp` in a known state and drives it through
      `egui_kittest::Harness::new_ui`, exposing a way to set the simulated window
      size via `input_mut().screen_rect` and to step frames. Include one trivial
      assertion (the harness renders a frame without panicking) so the file
      compiles and runs before any real assertion is added.

**Checkpoint**: The frame body is drivable from a test, and the ratchet mechanism
is understood rather than assumed.

---

## Phase 3: User Story 1 - The window shrinks freely in one drag (P1)

**Goal**: The enforced minimum is intrinsic, so a single continuous drag reaches
the content minimum on each axis.

**Independent test**: Persist an oversized window, restart, and shrink each axis
in one gesture (quickstart MV-6).

### Tests first (must fail against current code)

- [x] T006 [US1] In `tests/app_ui_sizing.rs`, assert contract C1: rendering the
      same application state at two different simulated window sizes produces an
      identical intrinsic extent on both axes. This fails today because the
      measured width equals the window width.
- [x] T007 [P] [US1] In `tests/app_ui_sizing.rs`, assert contract C2: the value
      pushed as the enforced minimum equals the intrinsic extent with the log
      closed, and the intrinsic extent plus `LOG_WIDTH_BONUS` and
      `open_log_reserve(row)` with the log open.
- [x] T008 [US1] In `tests/app_ui_sizing.rs`, assert contract C3, the ratchet
      assertion: across a monotonically shrinking sequence of simulated window
      sizes rendered as consecutive frames, the enforced minimum never rises and
      never exceeds the intrinsic extent at any step. This is the assertion no
      arithmetic-only test can express (research.md R5), and it is the first half
      of FR-019.
- [x] T009 [P] [US1] In `tests/app_ui_sizing.rs`, assert that the enforced
      minimum decreases when a control row disappears (FR-004) and that it is
      recomputed on a scale change (FR-005).
- [x] T009a [P] [US1] In `tests/app_ui_sizing.rs`, assert FR-002 and FR-006: on
      the frames before the measurement is stable the enforced minimum is the boot
      minimum (480 by 420), and once stable the rendered content fits within the
      enforced minimum on both axes with the log closed and with it open
      (analyze finding G3).
- [x] T009b [US1] In `tests/app_window_sizing.rs`, assert FR-008 against a new
      pure helper: capping an enforced minimum at a work area returns the minimum
      unchanged when it fits, and the work area when it does not, per axis
      independently (analyze finding G2).
- [x] T009c [US1] Assert FR-009: when the intrinsic content grows beyond the
      current window the requested window size grows to fit it, and when the
      content shrinks the window size request is unchanged and only the enforced
      minimum drops (analyze finding G1). **Deviation**: asserted against the pure
      `window_growth_request` helper in `tests/app_window_sizing.rs` rather than as
      a rendered-frame test. The harness has no real window, so the viewport
      command it would assert has no observable effect there; the decision rule is
      what carries the behavior and it is fully covered. The rendered wiring is
      exercised by the desk validation (MV-12).

### Implementation

- [x] T010 [US1] Add an intrinsic-extent accumulator to `src/app/mod.rs` beside
      the existing pure helpers, per `data-model.md` (`IntrinsicExtent`): it
      unions content-sized block rectangles, and accepts a height-only
      contribution for blocks that fill the available width (FR-007).
- [x] T011 [US1] In `src/app/ui.rs`, replace the
      `measured = ui.min_rect().size()` capture with the accumulator: feed it the
      rectangle of each content-sized block (status grid, weapon bar row, skills
      grid, button rows, uninstall confirmation row) as it is laid out, and feed
      the menu bar and separators as height-only contributions.
- [x] T012 [US1] In `src/app/ui.rs`, send `ViewportCommand::MinInnerSize` only
      when the intrinsic extent value changes, never in response to a window
      geometry change (FR-005, contract C2). Keep `content_min_size` and
      `measurement_stable` unchanged (plan.md decision D2).
- [x] T013 [US1] Add a pure `cap_to_work_area(minimum, work_area)` helper to
      `src/app/mod.rs` beside the existing sizing helpers (per-axis, returns the
      smaller value), and call it in `src/app/ui.rs` before sending the enforced
      minimum (FR-008). The helper is what makes FR-008 assertable headlessly,
      since the work area itself comes from the platform. Record in a comment why
      a scrollable central area was rejected (research.md R7).
- [x] T013a [US1] In `src/app/ui.rs`, implement FR-009: when the intrinsic extent
      exceeds the current window size, request a window size that fits it; when
      the intrinsic extent shrinks, leave the window size alone and let only the
      enforced minimum drop (analyze finding G1).
- [x] T014 [US1] Apply whatever additional fix T004's instrumentation identified,
      if the intrinsic measurement alone did not remove the ratchet. **Explicitly
      skipped, no additional fix needed.** T004 confirmed the measured extent was
      the window size less a constant on both axes, and replacing it with the
      intrinsic extent made the ratchet assertion pass on the first run. The
      platform gesture-latch mechanism was never needed to explain the defect and
      is not relied upon; recorded in `research.md` under "R1 confirmed".

**Checkpoint**: T006 through T009c green. US1 is independently shippable.

---

## Phase 4: User Story 2 - The log pane never covers a control (P1)

**Goal**: The log pane's top edge is never above the central content's bottom
edge, on any frame.

**Independent test**: Drag the splitter upward hard at several window sizes
(quickstart MV-7, MV-8).

### Tests first (must fail against current code)

- [x] T015 [US2] In `tests/app_ui_sizing.rs`, assert contract C4 under a splitter
      drag: simulate `hover_at` then `drag_at` past the boundary and assert that
      on every frame of the gesture the log pane's rendered top edge is at or
      below the central content's rendered bottom edge. This is the second half of
      FR-019.
- [x] T016 [P] [US2] In `tests/app_ui_sizing.rs`, assert contract C4 under a
      window resize with the log open, and under a resize immediately followed by
      a drag with no settled frame between them (spec CHK011).
- [x] T017 [P] [US2] In `tests/app_ui_sizing.rs`, assert that a pane height
      committed past the boundary is clamped before it is stored in application
      state and before it is raised as a persistence intent (FR-011), and that a
      restored out-of-range height is clamped before the first frame (FR-012).

### Implementation

- [x] T018 [US2] In `src/app/ui.rs`, re-clamp the height egui commits after the
      drag before storing it in `self.log_height` and before raising
      `UiIntent::SetLogHeight`, and force the clamped value on the next frame
      (plan.md decision D4).
- [x] T019 [US2] In `src/app/ui.rs`, clamp the restored pane height against the
      boundary before the first frame is rendered (FR-012).
- [x] T020 [US2] Confirm `clamp_log_height`, `log_min_height`,
      `open_log_reserve`, and `split_log_height` in `src/app/mod.rs` are
      unchanged, and that the existing `tests/app_window_sizing.rs` and
      `tests/app_log_view.rs` suites still pass unmodified.

**Checkpoint**: T015 through T017 green. US2 is independently shippable.

---

## Phase 5: User Story 3 - The settings modal grows with the window (P2)

**Goal**: The modal's rendered rectangle equals its computed extent on both axes.

**Independent test**: Open Settings at a small, a medium, and a very large window
(quickstart MV-9, MV-10).

### Tests first (must fail against current code)

- [x] T021 [US3] In `tests/app_ui_sizing.rs`, assert contract C5: the rendered
      modal rectangle matches `modal_extent` within one point on both axes, at
      window heights 420, 720, 1200, 1440, and 2160, using the worked values in
      `contracts/sizing-contracts.md`.
- [x] T022 [P] [US3] In `tests/app_ui_sizing.rs`, assert contract C6: at the
      modal's maximum size, the visible body height is at least half the settings
      body's total laid-out height at the same inner width.

### Implementation

- [x] T023 [US3] In `src/app/ui.rs` `settings_modal`, constrain the modal's area
      to an explicit centered rectangle of `modal_w` by `modal_h` so the rendered
      rectangle matches the computed extent, instead of inheriting the residual
      space a centered area leaves (plan.md decision D5). Leave `modal_extent` in
      `src/app/mod.rs` unchanged.
- [x] T024 [US3] Measure the settings body's laid-out height at the modal's inner
      width and evaluate FR-017. If half the body is not visible at the current
      maximum, raise the configured maximum height until it is, and record the
      new value and the measurement as a dated decision in `CHANGELOG.md`
      (plan.md decision D9). If the current maximum already satisfies it, record
      the measurement and change nothing.

**Checkpoint**: T021 and T022 green. US3 is independently shippable.

---

## Phase 6: User Story 4 - Sizing regressions cannot ship green (P1)

**Goal**: A passing suite actually means the sizing works.

**Independent test**: Reintroduce each defect and confirm the suite turns red.

- [x] T025 [US4] Verify the anti-regression property by hand, per the quickstart
      section "Proving the new checks actually bite": temporarily revert the
      measurement to `ui.min_rect().size()` and confirm the C1, C2, and C3
      assertions fail; restore and confirm green. Repeat for the log boundary
      clamp and for the modal height enforcement. Record the three results in
      `specs/030-ui-sizing-correctness/quickstart.md`.
- [x] T026 [US4] If any of the three reverts leaves the suite green, strengthen
      that assertion until it fails, then restore the fix. A check that does not
      fail on its own defect does not satisfy FR-021.
- [x] T027 [P] [US4] Confirm the new suite runs with no display and no GPU by
      checking that `egui_kittest` resolved with no default features in
      `Cargo.lock`, and that no `wgpu`, `winit`, or `x11` crate was added to the
      dev-dependency graph.

**Checkpoint**: The suite is proven to bite on all three defects.

---

## Phase 7: Polish and Cross-Cutting Concerns

- [x] T028 [P] Rewrite the module header of `src/app/ui.rs`: the claim that the
      layer is "excluded from the unit-tested surface and validated manually" is
      no longer true. State what is now covered by rendered-frame tests and what
      remains manual.
- [x] T029 [P] Correct the master specification
      `docs/ESO-Weave-Specification.md`: section 11.1 states a fixed
      minimum of 480 by 420, which is the boot floor and not the enforced
      minimum, so add the sizing model (boot floor, then intrinsic content
      minimum); add the log pane's never-overlap boundary to 11.2; add the
      modal's growth behavior to 11.3 (FR-022).
- [x] T030 Update `CHANGELOG.md` `[Unreleased]`: a `Fixed` entry for each of
      issues #12, #13, and #14, plus a dated Decisions entry for the
      `egui_kittest` dev-dependency (following the `ureq` precedent from slice
      018) and, if T024 changed it, for the new modal maximum.
- [ ] T031 **OWED, operator only.** Run the full manual desk validation in
      `specs/030-ui-sizing-correctness/quickstart.md`: MV-6 through MV-12 for
      this slice, and the absorbed MV-1 through MV-5 owed from slice 029. Record
      the outcome of each. This is the only step the agent cannot perform: it
      requires a real window-manager drag on a real display, which is also the
      only place the platform gesture-latch behavior (research.md R1) is
      observable. Every automated gate is green, but the reported reproductions
      are not closed until this runs.
- [x] T032 Run CI parity in the foreground and watch to completion:
      `cargo fmt --all -- --check`, then
      `cargo clippy --all-targets --all-features -- -D warnings`, then
      `cargo test --all --locked`.

---

## Dependencies

```text
Phase 1 (T001-T002)
    v
Phase 2 (T003 seam, T004 diagnosis, T005 harness)   <- blocks everything
    v
    +--> Phase 3 US1 (T006-T014, incl. T009a-c, T013a) ---+
    +--> Phase 4 US2 (T015-T020)  ---+--> Phase 6 US4 (T025-T027)
    +--> Phase 5 US3 (T021-T024)  ---+          v
                                          Phase 7 Polish (T028-T032)
```

- T004 blocks T010 through T014 absolutely (diagnose before fixing).
- T003 and T005 block every test task in Phases 3 through 6.
- US2 depends on US1 in practice, because the log boundary is computed from the
  intrinsic extent US1 establishes. US2's tests can be written before US1 lands,
  but they will not pass until it does.
- US3 is fully independent of US1 and US2 and could ship alone.
- US4 requires all three fixes to exist before its property can be verified.

## Parallel execution examples

Within Phase 3: T007 and T009 are `[P]` and touch independent assertions in the
same new file; write them in either order once T006's helper exists.

Across phases: Phase 5 (US3, the modal) shares no code path with Phases 3 and 4
and can be implemented concurrently by a separate worker.

Within Phase 7: T028 and T029 are `[P]` (different files, no shared state).

## Implementation strategy

**MVP scope**: Phase 1 plus Phase 2 plus Phase 3 (US1). That delivers the
intrinsic measurement, which is the root fix, and the harness that proves it.

**Incremental delivery**: US1 then US2 (US2's correctness depends on US1's
extent), with US3 landing at any point. US4 and Phase 7 close the slice.

**Do not skip**: T004 (diagnose before fixing) and T025/T026 (prove the checks
bite). Those two are the entire difference between this slice and the three point
patches that preceded it.
