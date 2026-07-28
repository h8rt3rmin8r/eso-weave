# Tasks: Auto-Potion

**Input**: Design documents from `/specs/039-auto-potion/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md),
[research.md](research.md), [data-model.md](data-model.md),
[contracts/trigger-rule.md](contracts/trigger-rule.md)

**Tests**: required. Constitution Principle III makes a failing test before the
code non-optional, and this feature is on a NON-NEGOTIABLE surface, so the safety
tests are the point rather than a follow-up.

**Organization**: by user story. US2 (never fires when it must not) is P1
alongside US1 and its tests are written in the same phase as the rule, not after
it.

## Format

`- [ ] [ID] [P?] [Story?] Description with file path`

---

## Phase 1: Setup

- [ ] T001 Confirm a clean tree and a green baseline: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, foreground

---

## Phase 2: Foundational (blocking prerequisites)

The key and the action block everything else, and both have existing tests that
enumerate the expected sets, so both fail loudly first.

- [ ] T002 Add `Key::F3` to `src/input/key.rs` in all five places: the enum variant, `as_str` (`"f3"`), `display_name` (`"F3"`), `parse`, and `ALL`
- [ ] T003 Add `Action::ToggleAutoPotion` to `src/input/action.rs`: the variant, `ALL` (now 10), `as_str` (`"toggle_auto_potion"`), and `default_key` (`Key::F3`)
- [ ] T004 Extend **both** predicates in `src/input/action.rs`: `suspend_exempt` and `is_app_toggle` must include `ToggleAutoPotion`. Without the first the hotkey dies exactly when the operator needs it; without the second the action routes to the weave worker and tries to run a weave sequence for it. Both have existing enumerating unit tests in the same file that must be updated deliberately.
- [ ] T005 [P] Update the existing key and action tests in `tests/input_engine.rs` for the new variants

**Checkpoint**: the new key and action exist and route like the toggles they mirror.

---

## Phase 3: User Story 1 + User Story 2 - The rule and its safety (Priority: P1)

**Goal**: the trigger rule fires when it must and never when it must not.

**Independent test**: the truth table in
[contracts/trigger-rule.md](contracts/trigger-rule.md), driven by a pure function
with a virtual clock and no game.

### Tests for the rule

- [ ] T006 [US1] Create `tests/potion.rs` and assert each of the **seven** conditions failing in isolation, with all six others satisfied, checking both that nothing is emitted and that the reported `Block` is the expected variant. Asserting only the absence of a keypress would let a test pass because a different condition was accidentally false; this is the failure the safety checklist's CHK012 names.
- [ ] T007 [P] [US1] Assert the OR across resources three times in `tests/potion.rs`: exactly one resource enabled and low, the other two enabled and high, for each resource in turn
- [ ] T008 [P] [US2] Assert unknown is never low in `tests/potion.rs`, for every threshold 0 to 100 inclusive, for each of the three resources
- [ ] T009 [P] [US1] Assert the at-or-below boundary in `tests/potion.rs`: fires at `p == threshold` and `p == threshold - 1`, does not at `p == threshold + 1`
- [ ] T010 [P] [US1] Assert thresholds 0 and 100 are both valid and behave as specified in `tests/potion.rs`
- [ ] T011 [P] [US2] Assert that with every watch disabled and all three resources at zero, nothing ever fires, in `tests/potion.rs`
- [ ] T012 [P] [US2] Assert an unreadable quickslot is not a potion and an unreadable cooldown is not zero, in `tests/potion.rs`
- [ ] T013 [P] [US1] Assert the retry interval against a virtual clock in `tests/potion.rs`: fires, does not fire at `last + interval - 1`, fires at `last + interval`
- [ ] T014 [P] [US1] Assert exactly one press and one release per trigger (two sink operations) in `tests/potion.rs`, and that repeated evaluation across at least twenty ticks under unchanging eligible conditions emits only what the retry interval and cooldown allow (SC-001)
- [ ] T014a [P] [US2] Assert the shipped defaults in `tests/potion.rs`: `AutoPotionConfig::default()` has all three watches disabled, and a freshly constructed controller is not enabled, so a fresh install never fires (FR-013). Raised by the analyze gate: T011 asserts an all-disabled config never fires, which is a different claim from the default being disabled.
- [ ] T014b [P] [US2] Assert in `tests/potion.rs` that with the controller never enabled, no tick under any combination of readings emits anything, which is the unit-level half of SC-006; the suite-level half is the existing weave and fishing tests continuing to pass unchanged at T034

### Implementation

- [ ] T015 [US1] Create `src/potion/mod.rs` with `ResourceWatch`, `AutoPotionConfig`, `PotionInputs`, and `Block`, per data-model.md
- [ ] T016 [US1] Implement `evaluate` in `src/potion/mod.rs` exactly per contracts/trigger-rule.md, in the specified condition order
- [ ] T017 [US1] Implement `AutoPotionController` in `src/potion/mod.rs`: `set_enabled`, `set_gated`, `set_suspended`, `tick`, `enabled`, and `last_attempt_ms`. No state enum (plan.md D4).
- [ ] T018 [US1] Implement `AutoPotionSink`, `MockAutoPotionSink`, and `RealAutoPotionSink` in `src/potion/mod.rs`, identical in shape to the fishing sinks, and register the module in `src/lib.rs`
- [ ] T019 [US2] Ensure the real sink is the only place this feature reaches synthesis, going through `InputBackend::synthesize` so focus scoping and recursion flagging are inherited rather than re-implemented (FR-008)

**Checkpoint**: the rule is correct and every blocking condition is proven in isolation.

---

## Phase 4: User Story 2 (continued) - Wiring the gates (Priority: P1)

- [ ] T020 [P] [US2] Assert in `tests/app_view_model.rs` that a `MenuGate` event sets the controller's gate directly, not only the input engine's
- [ ] T021 [P] [US2] Assert in `tests/app_view_model.rs` that a `SignalLost` event disables the controller
- [ ] T022 [US2] Add `PixelBusEvent::MenuGate` and `SignalLost` handling for the controller to `src/app/routing.rs`, alongside the existing input-engine and fishing-controller gating. The gate must reach the controller directly, carrying the slice 032 lesson that a controller acting on its own timers never passes through interception (FR-009).
- [ ] T023 [US2] Tick the controller on the existing worker loop in `src/main.rs`, beside `fishing.tick`, pushing the current suspend state in first. No new thread and no new timer (FR-012).

**Checkpoint**: every gate reaches the controller by the path the spec requires.

---

## Phase 5: User Story 3 - Controls and configuration (Priority: P2)

- [ ] T024 [P] [US3] Assert `AutoPotionConfig` load and store round-trips in `tests/app_settings.rs`, including out-of-range thresholds and an unparsable key degrading to defaults with a notice
- [ ] T025 [P] [US3] Assert in `tests/app_view_model.rs` that `UiIntent::SetAutoPotion` and the toggle hotkey reach the same state
- [ ] T026 [US3] Implement `AutoPotionConfig::load` and `store` in `src/potion/mod.rs`, following `FishingConfig` including the degradation notices
- [ ] T027 [US3] Add `potion: AutoPotionConfig` to `SettingsForm` in `src/app/settings_form.rs`, loaded and stored beside `fishing`
- [ ] T028 [US3] Add `UiIntent::SetAutoPotion(bool)` and its `apply_intent` arm to `src/app/mod.rs`, following `SetFishing`, and map `Action::ToggleAutoPotion` in `app_toggle_intent` in `src/app/routing.rs`
- [ ] T029 [US3] Add the auto-potion status readout and toggle control to `src/app/ui.rs` and their strings to `src/app/strings.rs`, adding the new strings to the hygiene lists
- [ ] T030 [US3] Add the Auto-potion settings group (per-resource enable and threshold, quickslot key, retry interval) to `src/app/ui.rs`. **Check the FR-017 half-visible modal bound after adding**: the settings body is close to it and slice 038 already had to trim a caption for exactly this reason.

---

## Phase 6: Polish and cross-cutting

- [ ] T031 [P] Document the feature and its safety conditions in `docs/ESO-Weave-Specification-v0.2.0.md`
- [ ] T032 [P] Add an Auto-potion section to `README.md`: what it does, that it defaults off, the OR rule, and that it never fires on an unreadable resource
- [ ] T033 [P] Add the `[Unreleased]` changelog entry in `CHANGELOG.md`, plus dated Decisions entries for unknown-is-not-low (D1), the retry interval versus the cooldown (D2), the typed block reason (D3), and not restoring the enable (D5)
- [ ] T034 Run the full merge gate in the foreground and watch it to completion: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`

---

## Dependencies

- Phase 1 blocks everything.
- Phase 2 blocks every story. T002 blocks T003; T003 blocks T004.
- Phase 3 depends on Phase 2. T015 blocks T016; T016 blocks T017; T017 blocks T018.
- Phase 4 depends on T017 existing.
- Phase 5 depends on Phase 3 and Phase 4. T026 blocks T027; T028 depends on T017.
- Phase 6 depends on everything.

## Parallel opportunities

- T007 through T014 are eight independent test additions in one new file; they may be written in any order but all before T015.
- T020, T021, T024, T025 are independent test additions across two files.
- T031, T032, T033 are three independent documentation files.

## Implementation strategy

There is no partial MVP worth shipping here. The rule without its gates is a
feature that presses keys at the wrong moment, which is worse than no feature, so
Phases 3 and 4 land together and neither is complete alone. Phase 5 makes it
usable; Phase 6 makes it explicable.

The single most important property of this slice is **SC-002**: each blocking
condition tested in isolation with every other condition satisfied, asserting
which condition blocked. T006 is that task, and it is the one to review first.

## Analyze gate

Run before implementation. Result: 0 CRITICAL, 0 HIGH, 2 MEDIUM, 1 LOW, with
100 percent requirement coverage. Both MEDIUM findings are closed above:

- **The eight-versus-seven mismatch.** SC-002 enumerates eight blocking
  conditions while the contract defines seven `Block` variants. They agree, but
  the mapping was unstated, so a reviewer counting one against the other would
  find a condition apparently untested. `contracts/trigger-rule.md` now carries
  the mapping table, including why signal loss is deliberately handled at the
  routing layer rather than inside the rule.
- **The defaults had no test.** FR-013 and SC-006 had no task. T011 asserts an
  all-disabled config never fires, which is not the same claim as the shipped
  default being disabled. T014a and T014b close both halves.

The LOW finding (SC-001's twenty-evaluation count absent from T014) is folded
into T014 rather than tracked separately.
