# Tasks: Ultimate Resource Meter

**Input**: S055 design documents in `specs/055-ultimate-resource-meter/`
**Tests**: Required by the constitution and issue contract

## Phase 1: Spec-kit setup and analysis

- [x] T001 Sync main, create `codex/s055-ultimate-resource-meter`, and bind #71
- [x] T002 Complete specification and requirements checklist
- [x] T003 Resolve wire precision in research, model, contract, and quickstart
- [x] T004 Add chronological build plan 025 and run blocking cross-artifact analysis

## Phase 2: Red protocol and lifecycle tests

- [x] T005 [US1] Add v5 negotiation, frozen v1-v4, and 29-block geometry tests
- [x] T006 [US1] Add exact two-byte codec and corruption tests
- [x] T007 [US1] Add atomic event, partial availability, loss, and recovery tests
- [x] T008 [US1] Add addon API, event, backstop, rebaseline, and manifest tests
- [x] T009 [US2] Add routing, engine storage, clear, and automation-inertness tests

## Phase 3: Protocol and model implementation

- [x] T010 [US1] Advance PixelBeacon to addon version 20 and protocol v5
- [x] T011 [US1] Publish B25-B28 from live charge and both hotbar costs
- [x] T012 [US1] Add protocol v5 negotiation, sampling, codec, and atomic event
- [x] T013 [US1] Clear and republish Ultimate across lifecycle boundaries
- [x] T014 [US2] Store and route the display-only Ultimate aggregate

## Phase 4: Red presentation tests

- [x] T015 [US2] Add current/max, active-cost, swap, and Ready projection tests
- [x] T016 [US3] Add theme color and contrast tests
- [x] T017 [US3] Add quarter, overlap, protrusion, and stable-slot geometry tests
- [x] T018 [US3] Add rendered order, reflow, card, and accessibility tests

## Phase 5: Presentation implementation

- [x] T019 [US2] Add display-only Ultimate view projection from typed ActiveBar
- [x] T020 [US3] Add purple theme tokens and generic meter presentation descriptor
- [x] T021 [US3] Paint shared quarter marks and optional cost threshold
- [x] T022 [US3] Add exact readout and permanently reserved green Ready slot
- [x] T023 [US3] Append Ultimate to Live HUD and preserve S054 geometry contracts

## Phase 6: Documentation and local validation

- [x] T024 Update master specification, README, changelog, plan 025, and plan index
- [x] T025 Add the Ultimate feature announcement
- [x] T026 Create the separate live release-verification issue
- [x] T027 Run focused protocol, addon, engine, model, theme, and sizing suites
- [x] T028 Run fmt, clippy with warnings denied, and all locked tests
- [x] T029 Run diff, forbidden-text, UTF-8-no-BOM, LF, generated-file, and secret audits
- [x] T030 Complete telemetry checklist and post-implementation analysis

## Phase 7: Independent review and delivery

- [x] T031 Run parallel code, security, and UI reviews and fix findings at 80% confidence
- [x] T032 Commit with the S055 conventional subject and required coauthor trailer
- [ ] T033 Push and publish a pull request with `Closes #71`
- [ ] T034 Resolve hosted CI and every first-round review comment
- [ ] T035 Request exactly one second `@Codex` review and resolve every result
- [ ] T036 Confirm all checks and threads are green, then request maintainer merge ritual

## Dependencies and Execution Order

- Phase 1 is the blocking spec gate.
- Protocol/model tests fail before transport implementation.
- Presentation tests fail before widget implementation.
- Protocol support precedes model routing; model routing precedes presentation.
- Documentation and all local validation precede commit and publication.
- The second review request occurs only after first-round CI and feedback settle.
