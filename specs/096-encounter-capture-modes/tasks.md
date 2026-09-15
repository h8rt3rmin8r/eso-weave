# Tasks: Encounter Capture Modes

**Input**: Design documents from `specs/096-encounter-capture-modes/`
**Tests**: Required by the constitution and issue #183.

## Phase 1: Governance and Specification

- [x] T001 Amend `.specify/memory/constitution.md` from 4.0.0 to 5.0.0 for exactly two explicit bounded capture modes.
- [x] T002 Synchronize `CLAUDE.md`, `docs/src/getting-started/responsible-use.md`, and process wording with the amended authority contract.
- [x] T003 Reconcile issue #183 acceptance language with the approved in-game control and historical desktop boundary.
- [x] T004 Update Plan 042 to make S096 the active final epic child.

## Phase 2: Addon Controller Tests

- [x] T005 Add failing default, mode, channel, toggle, and unsupported-mode tests in `tests/encounter_addon.rs`.
- [x] T006 Add failing single-before-combat and single-during-combat tests with exact API authority and partial-prefix assertions.
- [x] T007 Add failing continuous three-encounter tests for shared session identity, contiguous ordinals, and reset local sequences.
- [x] T008 Add failing toggle-off tests while waiting and capturing, including non-destructive partial finalization.
- [x] T009 Add failing aggregate event, raw, byte, encounter, marker, and terminal-reserve pressure tests.
- [x] T010 Add failing reload/relog/recovery tests for waiting and active single and continuous modes.
- [x] T011 Add failing callback, clock, invalid-state, no-auto-retry, and unknown-version preservation tests.
- [x] T012 Add static failing tests for exactly two modes and zero command-ingress or binding surfaces.

## Phase 3: Addon Controller Implementation

- [x] T013 Introduce the outer state-schema-v1/addon-v4 controller and selected/requested/effective state fields in `addon/EsoWeaveData/Encounter.lua`.
- [x] T014 Implement mode/channel selection and one toggle transition, with narrow legacy command compatibility.
- [x] T015 Refactor capture initialization to accept stable session identity and authoritative ordinal while retaining nested schema-v2/addon-v3 records.
- [x] T016 Implement truthful mid-combat start from `IsUnitInCombat("player")` and controlled `started-mid-combat` partial status.
- [x] T017 Implement continuous finalization, waiting gaps, shared session identity, records, and local sequence/actor reset.
- [x] T018 Implement aggregate accounting, current and outer reserves, and hard pressure failure without eviction.
- [x] T019 Implement interruption markers, reload/relog recovery, single stop, continuous resume, and hard-failure no-retry.
- [x] T020 Implement legacy singleton idle/armed/capturing/terminal adaptation without rewriting terminal evidence.
- [x] T021 Make addon controller tests green and refactor duplicated transition logic.

## Phase 4: Parser and Import Tests

- [x] T022 Add failing state-schema dispatch and valid single/continuous spool parsing tests in `tests/encounter_import.rs`.
- [x] T023 Add failing hostile wrapper tests for enums, unknown fields, identity/channel drift, keys, ordinals, markers, counts, and bounds.
- [x] T024 Add failing tests proving nonterminal current evidence is never imported or echoed.
- [x] T025 Add failing atomic two-record, growing-session, repeated-session, identity-collision, ordinal-collision, and bad-second-member tests.
- [x] T026 Add failing monotonic snapshot revision and exact-prefix tests.
- [x] T027 Add failing store-v3-to-v4 and read-only legacy tests that preserve canonical bytes and hashes.
- [x] T028 Add failing 128-byte identifier and value-free error tests.

## Phase 5: Parser and Store Implementation

- [x] T029 Add capture mode, controller state, session disposition, interruption, failure, ordered member, snapshot, and batch receipt models in `src/encounter/model.rs`.
- [x] T030 Add explicit legacy-versus-state-schema dispatch and strict whole-spool validation in `src/encounter/mod.rs` and `src/encounter/validate.rs`.
- [x] T031 Admit exact mid-combat opening authority only for controlled partial records while leaving complete S095 replay unchanged.
- [x] T032 Add batch canonicalization and value-free validated session summaries.
- [x] T033 Advance `src/encounter/store.rs` to schema v4 with immutable mode/ordinal metadata and append-only session snapshots.
- [x] T034 Implement byte-preserving v1-v3 write migration and synthetic single-mode,
  stable deterministic per-session ordinal read compatibility (normally ordinal 1).
- [x] T035 Implement full preflight plus one immediate transaction for encounters and session snapshot.
- [x] T036 Implement idempotent growing snapshot prefixes and reject every collision atomically.
- [x] T037 Keep `parse_capture` and `import_encounter` exact-one compatibility paths for existing callers and tests.
- [x] T038 Make parser, replay, store, and import tests green and refactor checked validation helpers.

## Phase 6: History and Desktop Tests

- [x] T039 Add failing session grouping and authoritative ordinal-order tests in `tests/encounter_history.rs`.
- [x] T040 Add failing batch import receipt, pluralization, and retry tests in `tests/app_encounter_history.rs`.
- [x] T041 Add failing desktop presentation tests for mode, disposition, ordinal, interruption/failure, and last-saved wording.
- [x] T042 Retain failing assertions that live data-addon encounter state remains unconfirmed and no desktop toggle/config authority exists.
- [x] T043 Add failing two-encounter same-session metric tests proving encounter-local projections remain unchanged.

## Phase 7: History and Desktop Implementation

- [x] T044 Extend encounter summaries and history snapshots with immutable mode, ordinal, and grouped session facts.
- [x] T045 Update `EncounterHistoryService` to use atomic batch import and grouped snapshots while preserving per-encounter detail/delete.
- [x] T046 Update the encounter-history worker with count-based batch results and safe singular/plural messages.
- [x] T047 Render grouped session history and a clearly labeled last-saved capture-state panel in `src/app/ui.rs`.
- [x] T048 Preserve the live-unconfirmed Data Details boundary and add no desktop capture control or requested-mode config.
- [x] T049 Make history, application, sizing, metrics, and CLI tests green.

## Phase 8: Canonical Documentation and Contracts

- [x] T050 Update `docs/src/features/encounter-capture.md` with the exact two-mode workflow, commands, limits, and recovery truth.
- [x] T051 Update architecture, encounter reference/status, troubleshooting, responsible-use, and test-strategy documentation.
- [x] T052 Update `docs/project/encounter-model.json` for state schema, modes, session order, interruptions, and aggregate limits.
- [x] T053 Update addon package/version documentation and any deterministic fixtures that depend on the embedded source.
- [x] T054 Update `CHANGELOG.md` with S096 Added and dated Decisions entries, including the issue wording deviation and constitution amendment.
- [x] T055 Record S096 implementation and official PR linkage in chronological Plan 042 and the build-plan index.
- [x] T056 Run docs policy, docs tests, mdBook tests/build, render smoke, and typo checks.

## Phase 9: Analysis and Local Validation

- [x] T057 Complete `analysis.md` traceability and resolve every spec-kit consistency finding.
- [x] T058 Run `check-prerequisites.ps1 -Json -RequireTasks -IncludeTasks` successfully.
- [x] T059 Run `cargo fmt --all -- --check`.
- [x] T060 Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [x] T061 Run `cargo test --all --locked`.
- [x] T062 Run the release build, `git diff --check`, UTF-8/BOM/mojibake, forbidden-dash, and static transport scans.

## Phase 10: Independent and Hosted Review

- [x] T063 Run independent code, security, and domain/UX reviews and fix every high-confidence finding.
- [ ] T064 Commit with the required S096 subject, changelog evidence, and Codex co-author trailer.
- [ ] T065 Push the authorized feature branch and publish an official PR linked to #183.
- [ ] T066 Move issue and PR project items to S096 PR review.
- [ ] T067 Wait for all CI, CodeQL, dependency, Codex, and security results; address every comment and resolve every thread.
- [ ] T068 Trigger at most one authorized second `@Codex review` round and address its results.
- [ ] T069 Confirm all required checks green, zero unresolved review threads, and ask the operator for final review and merge.

## Dependencies

- Phase 1 precedes implementation because the current constitution forbids continuous mode.
- Addon tests precede addon implementation; parser/store tests precede Rust import changes.
- History/UI changes depend on the stable batch models and store schema.
- Documentation and final analysis depend on the implemented behavior.
- Remote publication depends on every local gate and independent review.
