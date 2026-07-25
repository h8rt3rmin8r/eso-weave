---

description: "Task list for UI Window-Sizing and Layout Hardening (slice 027)"
---

# Tasks: UI Window-Sizing and Layout Hardening

**Input**: Design documents from `specs/027-window-sizing-hardening/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md,
contracts/ui-window-sizing.md, quickstart.md

**Tests**: REQUIRED. Constitution principle III (Test-First With Explicit Seams)
mandates a failing unit test before each pure-helper implementation. The sizing
math and the scheduler flag are extracted as pure helpers precisely so they are
unit-testable without a live window.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete
  tasks)
- **[Story]**: US1 (min-size), US2 (log viewer), US3 (toast), US4 (control
  heights)

## Story-to-issue map

- US1 = GitHub #4 (minimum window size fits content) - Priority P1
- US2 = GitHub #5 (live log viewer behaves) - Priority P2
- US3 = GitHub #6 (save toast only on meaningful changes) - Priority P3
- US4 = GitHub #7 (reduce control heights) - Priority P4

**Implementation ordering note**: US4 (control heights) is sequenced first among
the stories per plan decision D1/D2, because the shared control-height reduction
sets the final control metrics that the dynamic content-extent floor (US1) and
the log-pane clamp (US2) measure. Each story remains independently testable; the
ordering avoids re-tuning the measured floor. US3 (toast) is independent of the
sizing work and can proceed in parallel with any of the others.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the test surface and confirm a green baseline before any
change.

- [x] T001 Create the new integration test file `tests/app_window_sizing.rs` with
  its module header and imports (`use eso_weave::app::...`), no tests yet, so the
  suite compiles.
- [x] T002 Run the CI-parity baseline (`cargo fmt --all -- --check`, `cargo
  clippy --all-targets --all-features -- -D warnings`, `cargo test --all
  --locked`) in the foreground and confirm all green before editing source.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: None beyond Setup. This slice has no cross-story foundational code;
each pure helper is introduced test-first inside the story that owns it. Proceed
to the user-story phases.

---

## Phase 3: User Story US4 - Controls take less vertical space (Priority P4, sequenced first)

**Goal**: Reduce button/toggle/dropdown height by up to ~20 percent through the
shared style, with text never clipped, in both themes.

**Independent test**: Compare control heights before/after; confirm a single
consistent reduction and fully legible text in light and dark themes
(quickstart Scenario 4).

- [x] T003 [P] [US4] In `tests/app_window_sizing.rs`, write failing unit tests for
  a pure `reduced_interact_height(base: f32, font_line_height: f32) -> f32`
  helper: (a) returns about 0.8 * base when that stays above the legibility floor;
  (b) never returns less than `font_line_height` plus the minimum text padding
  (no-clip bound); (c) is monotonic in `base`. Covers contract C6 and FR-012.
- [x] T004 [US4] Implement `reduced_interact_height` in `src/app/mod.rs` (next to
  `clamp_log_height` / `modal_extent`) so T003 passes; document the ~20 percent
  target and the legibility floor in a doc comment.
- [x] T005 [US4] In `src/app/theme.rs` (`theme::apply`), set
  `style.spacing.interact_size.y` from `reduced_interact_height` using the theme's
  body text line height, and align `button_padding` / combo min height to the
  reduced size. Do not add per-call overrides. Verify `toggle_switch`
  (`src/app/widgets.rs`) inherits the reduced height with no code change there.
- [x] T006 [US4] Record the final chosen reduction figure (percentage and
  resulting height) in `plan.md` decision D1.

**Checkpoint**: Controls are shorter and text is legible; content extent measured
by US1 will now reflect the reduced heights.

---

## Phase 4: User Story US1 - Every control stays visible at the smallest window (Priority P1)

**Goal**: The window can never be sized smaller than the measured content extent
in either dimension, and a restored geometry is floored at that extent.

**Independent test**: Shrink to minimum in both dimensions with the log viewer
off; no control clipped, Pixel Beacon row intact (quickstart Scenario 1).

- [x] T007 [P] [US1] In `tests/app_window_sizing.rs`, write failing unit tests for
  a pure `content_min_size(measured: Vec2, boot_floor: Vec2) -> Vec2`: measured
  below floor returns floor; measured above floor returns measured; per-dimension
  independence; running-max monotonicity. Covers contract C1 and FR-001/FR-002.
- [x] T008 [US1] Implement `content_min_size` in `src/app/mod.rs` so T007 passes.
- [x] T009 [US1] In `src/app/ui.rs`, capture the central panel's laid-out extent
  each frame via `ui.min_rect().size()` inside the `CentralPanel` closure and keep
  a running max on the app struct (add a `content_min: egui::Vec2` field to the
  app state, seeded from the boot floor).
- [x] T010 [US1] In `src/app/ui.rs`, feed the running max through
  `content_min_size` and send
  `egui::ViewportCommand::MinInnerSize(content_min_size(...))` only when the value
  changes from the last sent value (guard against per-frame spam).
- [x] T011 [US1] In `src/main.rs`, keep the compile-time `MIN_SIZE` as the boot
  floor only, and ensure `sanitize_geometry` / `RestoreBounds` floors a restored
  geometry at no less than the boot floor (the runtime `MinInnerSize` then raises
  it to the measured content extent once laid out). Add a comment that the
  authoritative floor is now measured, not the constant.

**Checkpoint**: Minimum window fits all controls in both dimensions and tracks
content.

---

## Phase 5: User Story US2 - The live log viewer never covers the controls (Priority P2)

**Goal**: Enabling the viewer grows the window; the pane shows >= 6 lines, is
wider while open, and its resize is hard-clamped against the Skills area;
disabling is height-neutral.

**Independent test**: Enable the viewer from default size; confirm grow, six-line
minimum, wider min width, hard top-clamp, and height-neutral disable (quickstart
Scenario 2).

- [x] T012 [P] [US2] In `tests/app_window_sizing.rs`, write failing unit tests for
  a pure `log_min_height(row_height: f32) -> f32`: result >= 6 * row_height plus
  frame margins; strictly increasing in `row_height`. Covers contract C2 and
  FR-005.
- [x] T013 [P] [US2] In `tests/app_window_sizing.rs`, write failing unit tests for
  the updated `clamp_log_height` bounds: below-min raised to the six-line minimum;
  above-max lowered to `window_height - content_min_height`; degenerate `min >
  max` returns min. Covers contract C3 and FR-005/FR-007.
- [x] T014 [US2] Implement `log_min_height` in `src/app/mod.rs` so T012 passes.
- [x] T015 [US2] Update `clamp_log_height` in `src/app/mod.rs` to use
  `log_min_height` as the lower bound and `window_height - content_min_height` as
  the upper bound (accept the content-min height as a parameter), so T013 passes;
  update its callers.
- [x] T016 [US2] In `src/app/ui.rs` bottom-panel block, set the panel `min_size`
  to `log_min_height(row_height)` and `max_size` to `window_h -
  content_min.height` (replacing `window_h*0.1`/`window_h*0.75`), computing
  `row_height` from the log text style.
- [x] T017 [US2] In `src/app/mod.rs` `ToggleLogPanel` handling, on enable send
  `egui::ViewportCommand::InnerSize` with height increased by
  `log_min_height`, and on disable decrease by the same amount (never below
  `content_min.height`); route these through the app so the command reaches the
  viewport. Keep the added delta in one place for symmetry.
- [x] T018 [US2] While the log viewer is open, raise the enforced minimum width to
  `content_min.width + LOG_WIDTH_BONUS` (define the constant, target ~80-120
  points) and restore the base minimum width on close, via `MinInnerSize` in
  `src/app/ui.rs`. Record the final `LOG_WIDTH_BONUS` in `plan.md` decision D6.

**Checkpoint**: Log viewer opens into its own grown space, is readable and wider,
cannot cover the Skills area, and closes height-neutral.

---

## Phase 6: User Story US3 - The save confirmation only appears for real changes (Priority P3)

**Goal**: Layout writes (geometry, log height) persist silently; only meaningful
settings changes raise the toast. Persistence content is unchanged.

**Independent test**: Move/resize the window and drag the log divider (no toast);
toggle a setting (one toast); relaunch (geometry and log height restored)
(quickstart Scenario 3).

- [x] T019 [P] [US3] In `tests/app_session_state.rs`, extend the `SaveScheduler`
  tests: a layout-only mark settles dirty with `notify == false`; a meaningful
  mark settles with `notify == true`; a mixed batch settles with `notify ==
  true`; `take()` clears the notify flag; the invariant `notify => dirty` holds.
  Covers contract C4 and FR-009/FR-010/FR-013.
- [x] T020 [US3] In `src/app/mod.rs`, add `dirty_notify: bool` to `SaveScheduler`;
  have `mark_config` / `mark_session` set it, add silent
  `mark_config_layout` / `mark_session_layout` that do not, and surface the flag
  from `take()` / `maybe_flush`. Ensure T019 passes.
- [x] T021 [US3] In `src/app/mod.rs`, route the two layout intents to the silent
  marks: `SetLogHeight` -> `mark_config_layout`, `SetWindowGeometry` ->
  `mark_session_layout`. Leave all other intents on the meaningful marks. Confirm
  persistence content is unchanged (same stores written).
- [x] T022 [US3] In `src/app/ui.rs`, gate the toast at the flush site on the
  meaningful-change flag returned by `maybe_flush`, so the "Settings saved" toast
  shows only when a meaningful change was in the batch.

**Checkpoint**: Toast fires only for real settings changes; geometry and log
height still persist.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Record decisions, update the changelog, and verify end to end.

- [x] T023 Update `CHANGELOG.md` `[Unreleased]` section: Fixed entries for issues
  #4, #5, #6 and a Changed entry for issue #7 (control heights), plus a dated
  decision line recording the chosen control-height reduction figure and the
  `LOG_WIDTH_BONUS`. UTF-8 no BOM, LF, no em/en dashes.
- [x] T024 Run full CI parity in the foreground to completion: `cargo fmt --all --
  --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo
  test --all --locked`. All green.
- [~] T025 Execute the manual quickstart scenarios 1-4 in both light and dark
  themes (`cargo run`), confirming SC-001..SC-007; optionally capture a
  before/after screenshot via the PowerShell `CopyFromScreen` approach as pre-push
  evidence. PARTIAL: the app was launched and confirmed to start and render its
  window with no panic (the runtime viewport-command paths execute cleanly), but
  the interactive drag-to-minimum, log-toggle, resize-clamp, toast-suppression,
  and both-theme visual confirmations require the operator at the interactive
  desktop and are owed. The pure sizing/notify logic behind every scenario is
  covered by the passing automated tests.

---

## Dependencies & completion order

- Setup (T001-T002) before everything.
- US4 (T003-T006) sequenced first per plan D1/D2 (sets control metrics the floor
  measures). Independently testable.
- US1 (T007-T011) after US4 so the measured floor reflects the reduced controls;
  US1 tests (T007) are pure and do not depend on US4.
- US2 (T012-T018) depends on US1's `content_min` for the log-pane `max_size`
  (T016) and disable-floor (T017); US2 pure-helper tests (T012, T013) are
  independent.
- US3 (T019-T022) is fully independent of the sizing work and may run in parallel
  with US1/US2.
- Polish (T023-T025) last.

## Parallel opportunities

- All test-authoring tasks marked [P] (T003, T007, T012, T013, T019) touch test
  files only and can be written together up front (TDD red phase).
- US3 (T019-T022) can proceed concurrently with US1/US2 since it touches the
  scheduler and toast gate, not the sizing helpers or viewport commands.

## Suggested MVP

US1 alone (the P1 clipping fix) plus its prerequisite US4 control-height change is
a viable, shippable increment: the window always shows every control. US2 and US3
are additive improvements on top.
