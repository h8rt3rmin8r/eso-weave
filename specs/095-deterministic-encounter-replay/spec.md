# Feature Specification: Deterministic Encounter Replay

**Feature Branch**: `codex/s095-deterministic-encounter-replay`
**Created**: 2026-09-15
**Status**: Approved for implementation
**Input**: Issue #200, final implementation tranche under issue #186 and epic #182.

## Slice Boundary

S095 completes the reviewed include and exclude decisions for the encounter
subscription surface established in S094 and makes capture-schema-v2 normalized
facts independently reproducible from raw observations. It does not add source
families, continuous capture modes, command ingress, native-log ingestion, new
metrics or recommendations, uploads, telemetry, or gameplay automation.

## User Scenarios & Testing

### User Story 1 - Verify normalized facts from raw authority (Priority: P1)

As a user importing a complete schema-v2 encounter, I want ESO Weave to replay
the raw source observations and independently verify every normalized fact
before the encounter becomes active data.

**Why this priority**: A raw authority is meaningful only when derived facts do
not have to be trusted on import.

**Independent Test**: Execute the production Lua capture with synthetic runtime
constants, import its serialized envelope, and compare replay output with every
compatibility projection by source sequence, projection ordinal, time, kind,
and payload.

**Acceptance Scenarios**:

1. **Given** a complete current schema-v2 capture, **When** it is imported,
   **Then** pure Rust replay reconstructs every projection from raw observations
   and verifies an exact ordered match before storage.
2. **Given** any modified compatibility projection field, omitted projection,
   extra projection, or changed order, **When** it is imported, **Then** import
   rejects it without exposing compared values; structurally valid disagreement
   is divergent, while invalid links, ordinals, timing, or shapes fail structural
   validation before replay.
3. **Given** an unknown raw callback result or extra future scalar argument,
   **When** replay processes it, **Then** the raw value remains preserved and
   replay produces only facts defined by the pinned normalization profile.

### User Story 2 - Distinguish verified, partial, and legacy evidence (Priority: P1)

As a user with partial or historical captures, I want replay certainty stated
honestly so missing observations and older profiles are never described as
verified.

**Why this priority**: Raw loss can change actor numbering, API correlation, and
the shared event budget, so guessing would create false evidence.

**Independent Test**: Import complete current, partial current, pre-profile v2,
and v1 captures and assert their replay outcome, storage behavior, and metric
quality without changing legacy canonical bytes.

**Acceptance Scenarios**:

1. **Given** declared raw loss or a temporal discontinuity, **When** replay is
   requested, **Then** the result is indeterminate with a controlled reason and
   the partial capture remains importable under existing degraded-evidence rules.
2. **Given** a pre-profile schema-v2 capture or a schema-v1 capture, **When** it
   is imported, **Then** compatibility remains supported and replay is explicitly
   unavailable rather than fabricated.
3. **Given** an unsupported current normalization profile, **When** import is
   attempted, **Then** validation fails closed before replay or storage.

### User Story 3 - Review the complete capture decision surface (Priority: P2)

As a maintainer, I want one complete, pinned include and exclude matrix so future
capture expansion begins from explicit cost, privacy, value, and replay decisions.

**Why this priority**: Issue #186 cannot close while candidate encounter sources
remain implicit or the documented cost model is inaccurate.

**Independent Test**: Validate the human-readable matrix and machine contract
against the selected Lua registrations, API calls, Live and PTS signatures, and
the explicit excluded-family inventory.

**Acceptance Scenarios**:

1. **Given** the encounter-relevant source universe, **When** the contract is
   reviewed, **Then** every selected callback and API read records its signature,
   trigger or maximum frequency, filter, cost, fanout, projection correlation,
   anticipated value, evidence pin, and rationale.
2. **Given** an excluded candidate family, **When** the contract is reviewed,
   **Then** it has an explicit exclusion rationale and conditions for later
   reconsideration.
3. **Given** Live and PTS API snapshots, **When** the machine contract is checked,
   **Then** both pinned revisions and known signature differences are represented.

### Edge Cases

- Lua zero and empty strings remain truthy where the production normalizer uses truthiness.
- Millisecond conversion exactly matches `floor(seconds * 1000 + 0.5)`.
- Actor interning follows first normalized use, not first raw appearance.
- API samples use bounded source-specific batches; malformed batches are not guessed.
- Any declared raw loss makes whole-capture replay indeterminate.
- Replay uses existing actor/event ceilings and performs no I/O or mutation.
- Diagnostics name controlled locations and counts but never compared values.

## Requirements

### Functional Requirements

- **FR-001**: Current captures MUST include a bounded, versioned normalization
  profile with runtime enum values required for result classification.
- **FR-002**: The profile MUST remain inside schema v2, identify replay algorithm
  and capture API versions, and be strictly validated before use.
- **FR-003**: Replay MUST derive solely from structurally validated raw observations
  and profile metadata, never from supplied compatibility events.
- **FR-004**: Replay MUST reproduce every existing normalized source family,
  state transition, actor intern, API correlation, rounding rule, projection
  ordinal, and terminal behavior in deterministic source order.
- **FR-005**: A complete current capture MUST exactly compare replayed and supplied
  projections before import succeeds.
- **FR-006**: Any complete-current disagreement MUST reject import before storage.
- **FR-007**: Raw loss, temporal discontinuity, or incomplete API correlation MUST
  be indeterminate rather than verified or divergent.
- **FR-008**: Valid v1 and pre-profile v2 captures MUST remain importable with
  replay unavailable and canonical bytes and hashes unchanged.
- **FR-009**: Unsupported or malformed profiles MUST fail closed. Enum sets MUST
  be bounded, unique finite integers with valid semantic separation.
- **FR-010**: Replay MUST use checked numeric conversion, actor/event ceilings,
  linear processing, and bounded diagnostics.
- **FR-011**: Errors, receipts, logs, fixtures, history, and UI MUST NOT expose raw
  values or expected and actual projection values.
- **FR-012**: Metrics and recommendations MUST retain behavior, use verified facts
  for complete current captures, and keep degraded treatment for partial captures.
- **FR-013**: The capture set MUST remain 11 callbacks, six API source identifiers,
  and two lifecycle observations.
- **FR-014**: The subscription matrix MUST document every included and excluded
  family with Live/PTS evidence, rate, filter, cost, fanout, correlation, value,
  rationale, and reconsideration condition.
- **FR-015**: Tests MUST execute production Lua and cover every selected source,
  projection mutation, nil/numeric/future edges, loss, legacy dispatch, storage
  rejection, value-free diagnostics, and production ceilings.
- **FR-016**: Current manuals, architecture, tests, machine contract, changelog,
  and chronological plan MUST describe the delivered boundary and issue trace.
- **FR-017**: Local merge gates, hosted CI, code review, and security review MUST
  pass before operator merge approval.

### Key Entities

- **Normalization profile**: Runtime enum semantics plus replay algorithm identity.
- **Replay projection**: An event derived only from raw authority and the profile.
- **Replay assessment**: Verified, divergent, indeterminate, or unavailable.
- **Subscription decision**: An included source or excluded family with evidence.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Differential tests replay 100 percent of projections for all 11
  callbacks, six API sources, and two lifecycle sources.
- **SC-002**: Mutating every compared projection component rejects 100 percent of
  complete current captures before storage.
- **SC-003**: Loss, discontinuity, malformed batch, unsupported profile,
  pre-profile v2, and v1 cases have distinct value-free outcomes.
- **SC-004**: Replay remains linear and bounded at 100,000 raw observations and
  existing actor/event limits.
- **SC-005**: Existing metric, recommendation, store, and legacy compatibility
  outcomes remain stable.
- **SC-006**: Human and machine contracts account for every selected source and
  reviewed excluded family at both pinned API versions.
- **SC-007**: The full local merge gate and hosted CI pass with all reviews resolved.

## Assumptions

- The producer advances from addon version 2 to 3 while capture schema stays v2.
- Pre-profile v2 is historical compatibility data; it has no fabricated replay guarantee.
- Runtime metadata is required because authoritative ESO UI source does not
  publish stable numeric values for every required engine global.
- Raw loss is capture-wide replay indeterminacy because later actor IDs and the
  shared byte budget may depend on an omitted observation.
- Partial metrics remain degraded compatibility evidence; divergence is unusable.
- The canonical manual and changelog remain the publication surface. Adding a
  separate blog subsystem would be disproportionate.
