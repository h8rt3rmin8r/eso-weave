# Tasks: Native ESO Binding Evidence

**Input**: Design documents from `/specs/100-native-binding-evidence/`

**Prerequisites**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/`, and `quickstart.md`

**Tests**: Required by issue #206, the specification, and Constitution Principle III.

## Phase 1: Specification and Clarification

- [x] T001 Create the S100 feature context with the repository spec-kit scripts.
- [x] T002 Decompose parent issue #188 into ordered child issues #206, #207, and #208.
- [x] T003 Author prioritized publisher, decoder, and read-only authority scenarios in `spec.md`.
- [x] T004 Resolve conflict precedence, portable controls, fixed-cell encoding, compatibility, and slice boundaries.
- [x] T005 Complete `checklists/requirements.md`.

## Phase 2: Planning and Contracts

- [x] T006 Record ESO API, discovery, transport, compatibility, snapshot, and policy decisions in `research.md`.
- [x] T007 Define native action, control, modifier, chord, state, set, and cell entities in `data-model.md`.
- [x] T008 Define protocol generation 6 in `contracts/binding-evidence.md`.
- [x] T009 Produce the implementation and verification plan in `plan.md`.
- [x] T010 Produce focused, complete, and manual verification instructions in `quickstart.md`.

## Phase 3: Spec-Kit Analysis Gate

- [x] T011 Analyze issue #206, spec, research, model, contract, plan, checklist, and tasks in `analysis.md`.
- [x] T012 Resolve every analysis finding and confirm no unresolved clarification marker remains.
- [x] T013 Run spec-kit prerequisites with tasks required.

## Phase 4: Test-First Foundation

- [x] T014 [P] [US2] Add failing portable action, control, modifier, chord, state, and set tests in `tests/native_bindings.rs`.
- [x] T015 [P] [US2] Add failing protocol 6 geometry, cell decoding, action transposition, malformed evidence, legacy compatibility, and reader snapshot tests in `tests/pixelbus.rs`.
- [x] T016 [P] [US1] Add failing addon manifest, discovery contract, lifecycle refresh, and block-order tests in `tests/beacon.rs`.
- [x] T017 [P] [US3] Add failing static policy tests for mutation APIs, reset APIs, custom binding artifacts, and SavedVariables in `tests/beacon.rs`.
- [x] T018 Record focused red-test evidence before production implementation.

## Phase 5: User Story 1 - Observe Authoritative Native Bindings (P1)

**Goal**: Publish one deterministic read-only binding fact for each required native action.

**Independent Test**: Execute the embedded addon in an ESO-compatible Lua harness across the complete binding-state fixture matrix.

- [x] T019 [US1] Define fixed action order, portable ESO key map, modifier normalization, and discovery precedence in `addon/PixelBeacon/PixelBeacon.lua`.
- [x] T020 [US1] Append B29 through B39, render binding facts idempotently, and advance the negotiated layout constants in `addon/PixelBeacon/PixelBeacon.lua`.
- [x] T021 [US1] Refresh bindings on loaded, set, cleared, initialization, and periodic backstop paths in `addon/PixelBeacon/PixelBeacon.lua`.
- [x] T022 [US1] Bump the managed addon manifest and describe the read-only binding evidence in `addon/PixelBeacon/PixelBeacon.txt`.
- [x] T023 [US1] Make addon publisher and static authority tests green.

## Phase 6: User Story 2 - Decode a Safe Desktop Snapshot (P1)

**Goal**: Decode a coherent typed eleven-action set without allowing malformed or legacy evidence to become a chord.

**Independent Test**: Round-trip every portable control and modifier mask, then corrupt each channel and action position and verify fail-closed results.

- [x] T024 [US2] Implement the portable native action and chord model in `src/input/native.rs` and export it from `src/input/mod.rs`.
- [x] T025 [US2] Freeze protocol 5, advance protocol 6, extend block samples, and add pure binding cell and set decoders in `src/pixelbus/mod.rs`.
- [x] T026 [US2] Store, expose, emit, and clear `NativeBindingSet` in `PixelBusReader` without routing it to controllers.
- [x] T027 [US2] Sample B29 through B39 only for protocol 6 layouts in `src/pixelbus/mod.rs`.
- [x] T028 [US2] Make model, protocol, compatibility, and reader snapshot tests green.

## Phase 7: User Story 3 - Preserve Read-Only Authority (P1)

**Goal**: Permanently guard the addon against native binding mutation or persistence.

**Independent Test**: Inject each prohibited surface into the static validator and verify a precise failure.

- [x] T029 [US3] Finalize the reusable PixelBeacon binding-authority validator and all prohibited fixtures in `tests/beacon.rs`.
- [x] T030 [US3] Audit the complete addon diff for indirect mutation, persistence, custom action, and variable-width transport paths.
- [x] T031 [US3] Confirm current weaving, Fishing, Auto Potion, settings, persistence, and F1 through F3 behavior remain unchanged.

## Phase 8: Documentation and Integration

- [x] T032 Update `docs/src/reference/pixel-bus-protocol.md` with protocol 6 and the exact binding cell contract.
- [x] T033 Update `docs/src/getting-started/troubleshooting.md` with evidence states and safe remediation.
- [x] T034 Update Plan 043 and its index chronologically for S100 and the #188 child sequence.
- [x] T035 Update `CHANGELOG.md` with S100 outcomes and the dated workflow wording decision.
- [x] T036 Correct the protected-main bootstrap notice in `.github/workflows/ci.yml`.

## Phase 9: Local Validation and Review

- [x] T037 Run focused native binding, pixel-bus, beacon publisher, and static authority tests.
- [x] T038 Run `cargo fmt --all -- --check`, strict Clippy, all locked tests, and the release build.
- [x] T039 Run documentation policy, trust policy, encoding, spelling, links, and mdBook gates.
- [x] T040 Run spec prerequisites, diff, UTF-8/BOM/mojibake, forbidden-dash, and scope scans.
- [x] T041 Review the complete diff for false-positive chords, stale evidence, compatibility regressions, mutation authority, and scope drift.

## Phase 10: Publication and Hosted Review

- [x] T042 Commit S100 with changelog evidence and the Codex co-author trailer.
- [ ] T043 Push the authorized branch and publish an official PR that closes #206.
- [ ] T044 Move issue and PR project items to S100 PR review.
- [ ] T045 Wait for CI, CodeQL, dependency, Codex, and security results; address every comment and resolve every thread.
- [ ] T046 Trigger exactly one authorized second `@Codex review` round and address its results.
- [ ] T047 Confirm all required checks green, zero unresolved threads, and ask the operator for final review and merge.

## Dependencies

- Phases 1 and 2 precede Phase 3 because analysis covers the complete design.
- Phase 3 blocks production implementation.
- Phase 4 tests precede production changes in Phases 5 through 7.
- The portable model and wire contract must agree before reader integration.
- Addon and desktop protocol changes ship together under one version bump.
- Documentation follows the verified contract.
- Publication follows all local and hosted gates.
