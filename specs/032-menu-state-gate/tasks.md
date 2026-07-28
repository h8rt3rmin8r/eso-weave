# Tasks: PixelBeacon Menu-State Input Gate

**Feature**: 032-menu-state-gate | **Date**: 2026-07-27
**Input**: [spec.md](spec.md), [plan.md](plan.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/pixel-bus-b5.md](contracts/pixel-bus-b5.md), [quickstart.md](quickstart.md)

Test-first per constitution principle III. The safety tasks are ordered so the
one-way property is asserted before the guard that must satisfy it exists.

## Phase 1: Setup

- [x] T001 Confirm the baseline is green before any change: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, all in the foreground.

## Phase 2: Foundational

- [x] T002 Raise `NUM_BLOCKS` to 6 in `src/pixelbus/mod.rs`, add `ReaderConfig::menu_point()` from `block_center(block_px, 5)`, and update the two tests that pin the count and the geometry contract table in `tests/pixelbus.rs` (the `assert_eq!(NUM_BLOCKS, 5)` pin and `block_center_and_capture_dims_match_contract_table`, whose cases enumerate five centers and six-block capture widths). [FR-020]
- [x] T003 Add the `menu: Option<Rgb>` field to `BlockSamples` in `src/pixelbus/mod.rs`. No existing construction changes, because the struct derives `Default`; confirm that is still true after the edit. [FR-020]

## Phase 3: User Story 3, the gate can never make things worse (P1, safety)

Sequenced first among the stories despite being third in the spec, because it is
the constitutional obligation and because writing its proof before the guard is
what makes the guard's correctness a test result rather than a claim.

### Tests

- [x] T004 [US3] Write the failing exhaustive safety test in `tests/input_engine.rs`: enumerate the full cross product of the interception decision's inputs (injected versus real origin, focused versus not, a bound-and-active key, a bound-and-inactive key, an unbound key, a suspend-exempt key, suspended versus not, key down versus up), evaluate each with the gate off and on, and assert (a) an ungated pass implies a gated pass and (b) the gate never yields a suppression where the ungated decision passed. [FR-015, SC-005]
- [x] T005 [P] [US3] Write the failing focus-invariant test in `tests/input_engine.rs`: with the window unfocused, every key passes regardless of gate, suspend, binding, or transition. [FR-016]
- [x] T006 [P] [US3] Write the failing default-value test in `tests/input_engine.rs`: a freshly constructed engine is ungated, and its decisions match the pre-feature behavior for every input. [FR-013, SC-006]

### Implementation

- [x] T007 [US3] Add the `menu_gated` atomic and `set_menu_gated` to `InputEngine` in `src/input/mod.rs`, and one early-pass guard in `classify` alongside the existing suspend check, honoring the same exempt-action rule. Do not reorder the existing checks. [FR-008, FR-010, FR-015, FR-016, FR-017]

**Checkpoint**: the safety property is proven and the weave path is gated.

## Phase 4: User Story 1, typing is not disturbed (P1)

### Tests

- [x] T008 [P] [US1] Write failing decoder tests in `tests/pixelbus.rs`: every surface code decodes to its surface; a wrong marker, a failed checksum, and a red value between codes all yield no surface. [FR-007]
- [x] T009 [P] [US1] Write the failing arbitrary-color test in `tests/pixelbus.rs`: sweep the color cube and assert nothing outside the encoding decodes as a surface, including the five other blocks' rendered colors. [FR-007, SC-006]
- [x] T010 [P] [US1] Write failing reader tests in `tests/pixelbus.rs`: a decoded change emits one event, a steady state emits none, and the surface clears on signal loss and on a non-decoding block. [FR-005, FR-013]
- [x] T011 [P] [US1] Write the failing fishing-gate tests in `tests/fishing.rs`: a gated controller initiates no interact keypress on cast or reel, an ungated one still does, and a controller gated mid-cycle still completes work already in progress. [FR-009a, FR-011]
- [x] T012 [P] [US1] Write the failing cadence test in `tests/pixelbus.rs`: `poll_interval` returns the fast interval while fishing, the fast interval while able to intercept, and the idle interval only when neither holds, so neither setting is dead. [FR-019]

### Implementation

- [x] T013 [US1] Add `MenuSurface`, `MENU_MARKER = 0xD2`, the code step and maximum, the `BLOCK_CENTER_GREENS` entry, and `decode_menu` to `src/pixelbus/mod.rs`, following the B4 marker-and-checksum pattern. [FR-006, FR-007]
- [x] T014 [US1] Add the reader's menu state, the `PixelBusEvent::MenuGate` variant, decode-and-emit-on-change wiring in `observe`, the sample in `sample_and_observe`, and the debug log line on transitions. [FR-005, FR-013, FR-022]
- [x] T015 [US1] Clone the input engine into the pixel-bus worker thread in `src/main.rs` so routing can reach it. [FR-008]
- [x] T016 [US1] Add the `gated` flag and `set_gated` to `FishingController` in `src/fishing/mod.rs`, with early returns at the points that initiate an interact keypress. Work already in progress is untouched. [FR-009a, FR-011]
- [x] T017 [US1] Route `PixelBusEvent::MenuGate` in `src/app/routing.rs` to both gate holders, adding the input engine as a parameter, and update the existing call sites in `src/main.rs` and `tests/app_view_model.rs` mechanically. Confirm `map_event` returns `None` for the new variant. [FR-008, FR-009a]
- [x] T018 [US1] Extend `poll_interval` in `src/pixelbus/mod.rs` with the intercepting condition, and update its caller in `src/main.rs` to pass whether the application can currently intercept. [FR-019]
- [x] T019 [US1] In `addon/PixelBeacon/PixelBeacon.lua`: add `local NUM_BLOCKS = 6` update, the menu constants, the B5 block and its placement, and a change-detected `renderMenu()` deriving the boolean from `IsGameCameraUIModeActive()` ORed with `ZO_GetChatSystem():IsTextEntryOpen()`, then mapping the current scene to a code with a fallback to the generic one. Never hidden to express a state. [FR-001, FR-002, FR-003, FR-004, FR-005]
- [x] T020 [US1] Rename the addon's fishing tick to a general fast tick and drive the menu block from it, so the publish delay is one fast interval rather than one second. [FR-019a, SC-002]
- [x] T021 [US1] Advance `addon/PixelBeacon/PixelBeacon.txt` to version 7 on both version lines, extend the description, and advance the version-pin test in `tests/beacon.rs`. [FR-021]

**Checkpoint**: typing is not disturbed, on both synthesis paths.

## Phase 5: User Story 2, normal play resumes cleanly (P1)

- [x] T022 [P] [US2] Write the failing resumption tests: in `tests/input_engine.rs`, ungating restores the pre-gate decision for every input; in `tests/fishing.rs`, ungating lets the controller initiate again. [FR-012]
- [x] T023 [US2] Confirm no residual state is introduced by either guard (each is a flag read, not a stored decision), and that the reader emits the ungate transition exactly once. [FR-012]

## Phase 6: User Story 4, the operator can see the surface (P3)

- [x] T024 [P] [US4] Write the failing `menu_view` tests in `tests/app_view_model.rs` for a named surface, the generic surface, gameplay, and not-detected. [FR-022]
- [x] T025 [US4] Add `MenuView` and `menu_view` to `src/app/mod.rs`, the `AppView` field, `MENU_TITLE` and `MENU_TOOLTIP` to `src/app/strings.rs`, and the row after combat in `src/app/ui.rs`. [FR-022]

## Phase 7: Polish and Cross-Cutting

- [x] T026 [P] Extend the cross-language agreement test in `tests/beacon.rs` to the menu constants (`NUM_BLOCKS`, `MENU_MARKER`, the code step and maximum). [FR-020]
- [x] T027 [P] Update section 10.3 of `docs/ESO-Weave-Specification.md` with B5, the code table, the gate semantics, the two-path coverage, and the cadence change. [FR-023]
- [x] T028 [P] Update `CHANGELOG.md`: an `Added` entry, plus dated decisions for the two-path gating, the cadence condition, and the default-inactive design. [FR-023]
- [x] T029 Run the full merge gate in the foreground. [Constitution IV, SC-008]
- [x] T030 Verify the safety boundary against the diff: no existing safety test's assertions or scenarios changed, focus scoping untouched, the decision still synchronous and non-blocking with no added timed work, the fishing signal-loss degrade path unchanged, and no user-facing setting introduced for the gate. [FR-014, FR-017, FR-018, SC-008]
- [x] T031 Verify text hygiene across every touched file: UTF-8 without BOM, LF, no em-dashes or en-dashes including in the Lua.

## Owed after merge

- [ ] T032 In-game validation per [quickstart.md](quickstart.md), scenarios 1 through 7. Operator-owned. Scenario 1 (chat) and scenario 4 (fishing gated) are the two that would have caught the design defects this slice found on paper; neither is exercised by any automated test.

## Dependencies

```text
T001 -> T002, T003 -> Phase 3 (T004 to T006 then T007)
                   -> Phase 4 (T008 to T012 then T013 to T021)
                   -> Phase 5, Phase 6
                   -> Phase 7 (T026 to T028 parallel; then T029, T030, T031)
```

Phase 3 precedes Phase 4 deliberately: the safety proof should exist before the
guard it constrains.

## Implementation strategy

Phases 1 to 4 are the shippable feature. Phase 3 is not optional and not deferrable
to a follow-up; it is the reason this slice is allowed to touch the interception
decision at all. Phase 6 is diagnostic and could slip without affecting
correctness, though it is cheap.

## Analyze gate record (2026-07-27)

Zero CRITICAL findings, so no early halt. Four resolved:

- **T015 and T018 were ordered so the tasks did not compile.** The cadence change
  needed the input engine inside the reader thread, which a later task provided.
  Swapped.
- **FR-014 (no user setting) had no task.** A negative requirement verified by
  absence; folded into T030.
- **FR-002's core guarantee is not desk-verifiable.** That the signal is true while
  chat entry is open is game behavior no automated test here can reach. Recorded in
  the spec's edge cases and called out in the quickstart, so it is not mistaken for
  covered. This is the single largest residual risk in the slice.
- **Resuming from suspend with a surface open** can leave the gate stale for one
  idle interval. Added as an edge case with its bound.

Final coverage: 25 of 25 functional requirements have at least one task.

## Completion record (2026-07-27)

T001 to T031 complete; T032 (in-game validation) is owed and operator-owned.

Deviations recorded during implementation:

- **FR-009a was narrowed from "no new fishing interaction" to "no autonomous
  fishing interact".** The reel and recast fire on the controller's own timers and
  are deferred; the initial cast is the direct, immediate result of the operator
  pressing the fishing hotkey, which is itself exempt from the gate, so suppressing
  it would mean the operator presses a key and nothing happens. The spec was
  updated to draw the distinction as autonomous versus operator-initiated rather
  than new versus continuing.
- **Deferral, not suppression, for the two autonomous interacts.** Dropping them
  would advance the state machine past an interact the game never received.
  Re-arming the same deadline keeps controller state and game state consistent and
  lets a session resume on its own when the surface closes.
- **The arm timeout is deliberately not deferred.** It ends a session rather than
  synthesizing input, and deferring it would let a gated session hang waiting for a
  cast confirmation that will never arrive. Covered by a test.
- **T002's contract-table half was initially missed** and caught by the suite: the
  geometry table still enumerated five block centers. Extended to six.
- The menu surface is stored on the weave engine for display, alongside combat,
  because the gate flags themselves are booleans and would have collapsed the
  readout to "something is open" rather than naming the surface, which FR-022 asks
  for.
