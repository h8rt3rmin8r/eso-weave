# Feature Specification: Persistent Data Addon Foundation

**Feature Branch**: `codex/s092-data-addon-foundation`

**Created**: 2026-09-13

**Status**: Implemented

**Input**: Issues #184, #187, and #189 under coordinating epic #182. Establish
the permanent two-addon product topology, decide the supported encounter
ingestion path, and reach a conservative go or no-go decision for any future
desktop-to-addon command path.

## Clarifications

Routine choices are resolved under the build-phase autopilot policy.

- S092 uses one permanent data-addon identity for internally separate catalog
  and encounter modules. PixelBeacon remains the only other installed addon.
- The current development-only Collector and Encounter SavedVariables have no
  user migration path. The maintainers confirmed that no retained user data
  depends on them.
- Consolidation changes package ownership, not encounter semantics. S092 keeps
  the current bounded, explicitly armed, privacy-minimized encounter behavior;
  issue #186 owns any later lossless-capture change.
- Encounter data remains off PixelBus. S092 compares supported bulk-ingestion
  mechanisms and records measured uncertainty rather than stretching the screen
  protocol.
- Command-ingress research cannot create or mutate an ESO binding, test a
  server-persisted binding on a live account, or disguise generated input.
- A no-go decision is a successful command-transport outcome when no supported
  candidate satisfies the reliability and account-risk constraints.
- S092 amends the constitution from the superseded three-addon boundary to the
  explicitly approved two-addon topology before the specification analysis
  gate. The new boundary retains module isolation and every current safety
  restriction.
- Repository and public-API evidence cannot prove native-log disk cadence,
  crash durability, privacy transformations, or Windows-versus-Proton file
  behavior. Those operator runs move to a separate verification issue, and no
  missing row is converted into claimed evidence to close this pull request.

## User Scenarios and Testing

### User Story 1 - Install one durable data addon (Priority: P1)

As an ESO Weave user, I want catalog discovery and encounter capture supplied by
one managed data addon so the product installs exactly two clearly owned addons
without exposing obsolete development packages.

**Why this priority**: A stable package boundary is required before lifecycle
UI, lossless capture, or additional capture modes can be implemented safely.

**Independent Test**: Inspect release and embedded addon inventories, exercise
each data module independently in a deterministic addon harness, and verify
that PixelBeacon plus one managed data addon are the only ESO Weave packages.

**Acceptance Scenarios**:

1. **Given** a clean AddOns directory, **When** ESO Weave installs its addon
   artifacts, **Then** the resulting product topology contains PixelBeacon and
   one permanently named data addon.
2. **Given** only catalog work is requested, **When** the data addon loads,
   **Then** the encounter module remains dormant and registers no capture work.
3. **Given** only one encounter is armed, **When** the data addon loads and
   combat begins, **Then** the catalog module remains dormant and current
   encounter safety and boundedness remain intact.
4. **Given** an unmanaged target at the permanent data-addon path, **When** a
   lifecycle mutation is requested, **Then** ESO Weave refuses to modify it and
   leaves PixelBeacon and all neighboring addons unchanged.

---

### User Story 2 - Choose a truthful encounter-ingestion path (Priority: P1)

As a maintainer, I want a reproducible comparison of supported encounter data
paths so continuous capture and future live processing are based on measured
completeness, latency, and failure behavior.

**Why this priority**: Capture-mode and status designs would otherwise depend on
unverified assumptions about SavedVariables flushing or native encounter-log
availability.

**Independent Test**: Apply the checked comparison matrix to representative
event families and file-lifecycle cases, preserve the evidence receipts, and
reproduce the selected path or its declared blockers from the recorded
environment and procedure.

**Acceptance Scenarios**:

1. **Given** addon callbacks, native encounter logging, and SavedVariables,
   **When** their coverage is compared, **Then** every supported and missing
   event family is recorded without calling one successful parse complete.
2. **Given** encounter boundaries and interruption cases, **When** desktop
   availability is measured, **Then** buffering, flush, rotation, truncation,
   restart, crash, and recovery behavior are recorded with timestamps and loss
   state.
3. **Given** platform evidence is unavailable, **When** the decision is
   reviewed, **Then** the missing row remains explicit and cannot be presented
   as a passing platform result.
4. **Given** the completed evidence matrix, **When** a path is selected, **Then**
   the decision names a fallback hierarchy and atomic implementation follow-ups.

---

### User Story 3 - Decide whether command ingress is acceptable (Priority: P1)

As an operator, I want an explicit go or no-go decision for desktop-to-addon
commands so convenience controls cannot introduce unexplained account-visible
state or unreliable generated-input behavior.

**Why this priority**: The approved single and continuous capture modes need a
control story, but no implementation should assume that a custom binding is a
private transport.

**Independent Test**: Review each documented candidate against direction,
latency, persistence, visibility, reliability, platform behavior, and account
risk, then verify that the recommendation follows the recorded evidence.

**Acceptance Scenarios**:

1. **Given** the approved capture workflows, **When** their control needs are
   reduced, **Then** the decision records the minimum command vocabulary rather
   than speculative commands.
2. **Given** a binding-based candidate, **When** its persistence or visibility
   is server-associated or unknown, **Then** it is rejected unless separately
   approved evidence establishes an acceptable supported use.
3. **Given** passive SavedVariables, user-initiated in-game controls, and other
   documented candidates, **When** they are compared, **Then** direction,
   latency, failure behavior, and supported-platform constraints are explicit.
4. **Given** no candidate meets the constraints, **When** research concludes,
   **Then** a no-go result and safe product fallback are recorded without an
   evasion or camouflage design.

### Edge Cases

- A clean installation encounters old development addon directories that are
  unmanaged or contain unexpected files.
- A data-module callback fails while the other module is idle or active.
- Catalog discovery is requested during combat while encounter capture is
  armed, active, complete, or partial.
- A game update changes an event signature, manifest API version, encounter-log
  shape, or SavedVariables behavior.
- A native log is absent, disabled, anonymized, rotated, truncated, partially
  written, locked, or replaced while being observed.
- Windows evidence succeeds while Linux or Proton evidence is absent or differs.
- An ingestion candidate provides lower latency but omits addon-only fields.
- A custom binding appears functional locally but its persistence or server
  visibility cannot be established from public evidence.
- The desktop closes, ESO reloads, or the addon reloads while a future command
  request is pending.
- A research result conflicts with an existing downstream issue assumption.

## Requirements

### Functional Requirements

- **FR-001**: Release and embedded addon inventories MUST contain exactly
  PixelBeacon and one permanently named data addon.
- **FR-002**: The data addon MUST contain separate catalog-discovery and
  encounter-capture modules with explicit ownership and independent activation.
- **FR-003**: An inactive data module MUST register no unnecessary
  high-frequency callbacks or updates.
- **FR-004**: Catalog discovery MUST retain its current combat prohibition,
  bounded output, explicit user initiation, and independence from automation.
- **FR-005**: Encounter capture MUST retain its current explicit one-shot arm,
  bounded output, privacy-minimized payload, loss declarations, and inability to
  authorize or synthesize gameplay input.
- **FR-006**: S092 MUST remove the obsolete Collector and Encounter package
  manifests, deployment topology, embedded artifacts, and product-facing
  references without adding migration, compatibility, or dual-deployment code.
- **FR-007**: Every data-addon lifecycle mutation MUST stay inside the resolved
  permanent addon subtree, reject links and unexpected file shapes, and require
  its own verified managed marker before replacement or removal.
- **FR-008**: A data-addon lifecycle operation MUST NOT modify PixelBeacon or
  any neighboring addon.
- **FR-009**: Clearing one module's retained output MUST preserve the other
  module's state and MUST NOT delete or rewrite the shared SavedVariables file
  from the desktop while ESO may own its in-memory contents.
- **FR-010**: The shared SavedVariables file read MUST have one explicit outer
  byte bound large enough for both already-bounded module payloads, while each
  module retains its existing schema and content limits.
- **FR-011**: The product constitution and canonical documentation MUST describe
  the approved two-addon topology and preserve separate screen-signal and bulk
  data responsibilities.
- **FR-012**: The ingestion study MUST compare addon callback facts, native
  encounter-log output, and SavedVariables for representative event coverage,
  availability cadence, operational prerequisites, and failure behavior.
- **FR-013**: Ingestion evidence MUST cover creation, buffering, flush,
  rotation, truncation, reload, relog, crash, recovery, file sharing, and path
  behavior on each supported platform for which evidence is claimed.
- **FR-014**: The ingestion decision MUST record end-to-end latency,
  completeness limits, storage and performance tradeoffs, a fallback hierarchy,
  and explicit unknowns.
- **FR-015**: Bulk encounter data MUST remain off PixelBus.
- **FR-016**: The command study MUST inventory each documented candidate's
  directionality, latency, persistence, visibility, reliability, failure
  behavior, and supported-platform behavior.
- **FR-017**: The command study MUST keep desktop application toggles,
  read-only native binding discovery, and addon command ingress as three
  separate systems.
- **FR-018**: S092 MUST NOT create a custom ESO action, binding manifest,
  binding assignment, native-binding mutation, live-account binding experiment,
  or disguised synthetic-only command.
- **FR-019**: The command recommendation MUST derive a minimum vocabulary from
  approved workflows and state go, no-go, or a specifically evidenced
  prerequisite before implementation.
- **FR-020**: Each research decision MUST create or reconcile atomic follow-up
  issues when implementation or independent field evidence remains. Native-log
  platform measurements MUST remain in Release verification until actual
  operator receipts exist for Windows and Linux or Proton.
- **FR-021**: S092 MUST update issues #184, #187, and #189 with reviewable
  decision or implementation evidence while keeping epic #182 as the
  multi-slice coordinator.
- **FR-022**: S092 MUST NOT implement first-class data-addon UI, lossless event
  capture, continuous capture modes, native binding discovery, or a command
  transport.

### Key Entities

- **Data Addon Package**: The permanent managed package containing the catalog
  and encounter modules, one manifest, one ownership marker, and one version.
- **Catalog Module**: The user-started bounded catalog-discovery capability,
  dormant outside an explicit request and prohibited during combat.
- **Encounter Module**: The explicitly armed bounded encounter recorder with
  current privacy, loss, and action-independence restrictions.
- **Module State**: Independent requested, dormant, active, terminal, or failed
  state for one module without implying the other module shares that state.
- **Ingestion Candidate**: A supported mechanism evaluated for coverage,
  latency, lifecycle, platform behavior, failure, and recovery.
- **Ingestion Decision**: The selected mechanism or blocker record with
  evidence receipts, fallback hierarchy, unknowns, and follow-up ownership.
- **Command Candidate**: A documented desktop-to-addon mechanism evaluated
  without mutating a live account binding.
- **Command Decision**: A go or no-go record containing the minimum vocabulary,
  evidence, risk classification, prerequisites, and safe fallback.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Packaged, embedded, and deterministic fixture inventories contain
  exactly two ESO Weave addon directories and zero obsolete package manifests.
- **SC-002**: One hundred percent of module-isolation fixtures prove that an
  inactive module registers no high-frequency work and cannot change the other
  module's state.
- **SC-003**: One hundred percent of unmanaged, linked, unexpected-file, failed
  update, and removal fixtures leave PixelBeacon and neighboring addon bytes
  unchanged.
- **SC-004**: Existing catalog and encounter behavior suites pass against the
  permanent data-addon identity with no compatibility path for unused
  development SavedVariables.
- **SC-005**: Module-specific clear fixtures preserve 100 percent of the other
  module's retained state, and desktop lifecycle tests contain zero shared-file
  delete paths.
- **SC-006**: Every row claimed by the ingestion decision records environment,
  versions, timestamps, event coverage, availability cadence, and failure
  outcome; missing evidence remains visibly unverified.
- **SC-007**: The ingestion decision identifies exactly one preferred path or a
  declared blocker and at least one ordered fallback without using PixelBus for
  bulk data.
- **SC-008**: Every command candidate has all seven required comparison
  dimensions, and the final recommendation contains zero binding mutations or
  live-account custom-binding experiments.
- **SC-009**: Static source inspection finds zero new gameplay mutation,
  synthesized-input, upload, process-memory, packet, or custom-binding paths in
  the data addon.
- **SC-010**: Specification analysis, formatting, linting, full locked tests,
  documentation policy, UTF-8, forbidden-dash, and mojibake gates pass.

## Assumptions

- Only maintainers have installed the development Collector and Encounter
  packages, and neither retained data that requires migration.
- The current catalog and encounter module behavior remains the baseline until
  separately approved issues change it.
- Official ESO documentation and source establish supported capability claims;
  environment-specific latency and file behavior require reproducible field
  receipts.
- A research no-go is acceptable and does not require a placeholder transport.
- First-class lifecycle UI, lossless capture, and continuous capture are later
  slices that consume S092 decisions.

## Out of Scope

- First-class data-addon lifecycle controls or main-window status presentation.
- Lossless subscribed-event capture or removal of the current privacy profile.
- Single-versus-continuous capture mode implementation.
- Native ESO binding discovery or changes to F1, F2, and F3 desktop toggles.
- A production desktop-to-addon command transport.
- Bulk encounter payloads on PixelBus.
- Migration or compatibility support for obsolete development addon state.
- Release publication, version tagging, or merging the pull request.
