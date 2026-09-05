# Tasks: Negotiated Width-Aware Pixel Geometry

**Input**: Design documents from `specs/045-negotiated-width-geometry/`

**Tests**: Required for all protocol, lifecycle, capture, compatibility, and UI
truthfulness behavior.

## Phase 1: Setup and specification

- [x] T001 Synchronize `main` and create `codex/045-negotiated-width-geometry`
- [x] T002 Inspect issues #42 and #43, the fixed grid, display descriptor, sampler seam, addon deployment, and settings footprint
- [x] T003 Complete specify, clarify, checklists, plan, research, data model, contract, quickstart, tasks, and analysis artifacts
- [x] T004 Run the spec-kit and text-hygiene gates before implementation

## Phase 2: Foundational protocol tests

- [x] T005 [P] Add failing layout-header encode/decode tests in `tests/pixelbus.rs`
- [x] T006 [P] Add failing negotiated point, extent, uniqueness, exact-boundary, and minimum-width tests in `tests/pixelbus.rs`
- [x] T007 [P] Add failing corruption, version, bounds, surface-fit, and legacy-selection tests in `tests/pixelbus.rs`
- [x] T008 [P] Add failing batch preparation and layout-transition tests using `MockSampler`
- [x] T009 [P] Add failing addon constant, manifest version, resize registration, and width-capacity source tests in `tests/beacon.rs`

## Phase 3: User Story 1 - Width-aware one-row layout

- [x] T010 [US1] Add shared header constants, `LayoutMode`, and `BusLayout` geometry in `src/pixelbus/mod.rs`
- [x] T011 [US1] Replace fixed payload point and capture derivation with validated `BusLayout`
- [x] T012 [US1] Add the invariant header controls and dynamic column calculation to `addon/PixelBeacon/PixelBeacon.lua`
- [x] T013 [US1] Reflow header and payload cells on resize and periodic width changes
- [x] T014 [US1] Advance `addon/PixelBeacon/PixelBeacon.txt` to managed protocol version 14

## Phase 4: User Story 2 - Fail-closed reader and bounded capture

- [x] T015 [US2] Implement typed header decoding and unavailable reasons in `src/pixelbus/mod.rs`
- [x] T016 [US2] Make `SurfaceSampler::prepare` accept an extent and record requests in `MockSampler`
- [x] T017 [US2] Make `GdiSampler` capture the per-batch requested extent and keep X11 compatible
- [x] T018 [US2] Implement reader bootstrap, cached validation, conditional recapture, and payload suppression
- [x] T019 [US2] Change-detect and log layout transitions without normal-sample log spam

## Phase 5: User Story 3 - Legacy upgrade and truthful footprint

- [x] T020 [US3] Implement heartbeat-gated legacy 16-column selection
- [x] T021 [US3] Route `LayoutState` through `PixelBusEvent` into `WeaveEngine`
- [x] T022 [US3] Replace the fixed settings footprint with negotiated, legacy, waiting, and unavailable captions
- [x] T023 [US3] Update app and routing tests for layout state and caption truthfulness

## Phase 6: Documentation and integration

- [x] T024 Supersede fixed-column geometry in `docs/ESO-Weave-Specification.md`
- [x] T025 Create chronological build plan `docs/plans/plan-015.md` and update its index
- [x] T026 Update `CHANGELOG.md` with the feature and dated authority/protocol decision
- [x] T027 Update stale fixed-grid code comments and operator diagnostics
- [x] T028 Set issues #42 and #43 to S045 and In progress in the delivery Project

## Phase 7: Validation and publication

- [x] T029 Run focused protocol, addon, display, routing, and settings tests
- [x] T030 Run cargo fmt, clippy, and the full locked test suite
- [x] T031 Run spec-kit, encoding, long-dash, secret, placeholder, and diff audits
- [ ] T032 Commit with the S045 conventional commit and attribution trailer
- [ ] T033 Push and publish a pull request with separate `Closes #42` and `Closes #43` lines
- [ ] T034 Move both Project items to PR review and confirm GitHub closing references
- [ ] T035 Address every first-round Codex review comment and CI failure
- [ ] T036 Trigger at most one second `@Codex review` round and address every resulting comment
- [ ] T037 Confirm green checks and resolved threads, then request the final merge ritual

## Dependencies and Execution Order

- Phase 2 freezes the contract before implementation.
- Phase 3 establishes geometry needed by reader lifecycle work in Phase 4.
- Phase 5 depends on typed layout state from Phase 4.
- Documentation records the implemented result and follows the code phases.
- Issue #44 remains Release verification and is not closed by the S045 pull request.
