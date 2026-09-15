# Feature Specification: Lossless Subscribed-Event Envelope

**Feature Branch**: `codex/s094-lossless-event-envelope`

**Created**: 2026-09-15

**Status**: Approved for implementation

**Input**: Issue #186, first implementation tranche under epic #182.

## Slice Boundary

S094 establishes the lossless authority for the encounter sources already
subscribed by ESO Weave Data. It does not add new event families, continuous
capture modes, native-log ingestion, metrics, recommendation rules, uploads, or
automation. Issue #186 remains open for later expansion and complete Rust-side
renormalization.

## User Scenarios & Testing

### User Story 1 - Retain exact subscribed observations (Priority: P1)

As a user who explicitly arms one encounter, I want every scalar value delivered
by the selected callbacks and every API result used by normalization retained in
its original order, including names, identifiers, nils, booleans, strings,
fractions, and unknown numeric values.

**Why this priority**: The raw stream must become a trustworthy source before
capture modes or additional metrics can build on it.

**Independent Test**: Execute the production Lua in the Lua 5.1 harness, deliver
each subscribed callback with distinctive positional values, serialize the
SavedVariables table, import it, and compare every tagged value after SQLite
reload.

**Acceptance Scenarios**:

1. **Given** an explicitly armed capture, **When** a selected callback delivers
   values, **Then** the capture stores the event code, stable source identifier,
   API and contract versions, monotonic time, exact argument count, and every
   argument in order without redaction, pseudonymization, renaming, rounding,
   coercion, or omission.
2. **Given** an unknown combat-result value or extra future scalar callback
   argument, **When** the callback is delivered, **Then** the raw observation is
   retained even when no normalized fact can be produced.
3. **Given** a getter used to produce an existing normalized fact, **When** the
   getter is called, **Then** its inputs and ordered outputs are retained as an
   API-sample observation.

---

### User Story 2 - Know exactly when capture is incomplete (Priority: P1)

As a user, I want hard limits and unsupported runtime values to produce explicit
loss evidence so incomplete raw data is never presented as complete.

**Why this priority**: Losslessness is credible only when every exception is
bounded and declared.

**Independent Test**: Force record, byte, oversized-string, unsupported-type,
callback-failure, and clock-reset paths in the Lua harness and verify a partial
terminal capture with exact loss or discontinuity evidence.

**Acceptance Scenarios**:

1. **Given** an observation that cannot fit within a hard bound, **When** capture
   encounters it, **Then** the whole observation is omitted, its source sequence
   is counted in one exact loss range, and terminal capacity remains available.
2. **Given** a nonfinite number or non-scalar callback value, **When** capture
   encounters it, **Then** the value is never stringified or coerced and the
   observation becomes declared unsupported-value loss.
3. **Given** a clock rollback, **When** monotonic tracking detects it, **Then** a
   temporal-discontinuity source observation and partial status are retained.

---

### User Story 3 - Preserve existing history through the upgrade (Priority: P2)

As a user with schema-v1 captures and encounter history, I want the new capture
format installed and imported without erasing or rewriting prior evidence.

**Why this priority**: An unimported terminal SavedVariables capture and immutable
SQLite history are user-owned data.

**Independent Test**: Upgrade a terminal and interrupted v1 SavedVariables value,
migrate a populated v1 SQLite store, import v2 beside v1, and verify legacy bytes,
hashes, list/load behavior, backups, and immutability.

**Acceptance Scenarios**:

1. **Given** a terminal v1 SavedVariables capture, **When** the v2 addon loads,
   **Then** it preserves the capture until the user imports or explicitly clears
   it.
2. **Given** a valid v1 encounter store, **When** a v2 capture is imported, **Then**
   migration is atomic and every v1 canonical byte and hash remains unchanged.
3. **Given** equivalent v1 normalized facts and v2 compatibility projections,
   **When** metrics and recommendations run, **Then** their current values and
   evidence gates remain stable.

## Edge Cases

- Interior and trailing nil callback arguments remain distinct from missing
  arguments, false, zero, and empty strings.
- IEEE-754 negative zero, subnormal values, exponent extremes, and ordinary
  fractional values have a deterministic integer-only tagged representation.
- Public fixtures use synthetic canaries. Receipts, errors, logs, and UI status
  never echo retained payload values.
- A malformed or future structural envelope is rejected without execution,
  while future scalar values inside a valid raw observation remain retainable.
- A v1 capture is never upgraded by fabricating raw observations.
- Worst-case accepted records remain inside file, token, entry, depth, string,
  event, and estimated-byte limits.

## Requirements

### Functional Requirements

- **FR-001**: The addon MUST preserve one-shot explicit arming and MUST warn that
  raw local capture can contain names and identifiers.
- **FR-002**: Capture schema v2 MUST store a separately identifiable raw source
  stream as the authoritative record while retaining the current normalized
  event stream as a compatibility projection.
- **FR-003**: Each raw source observation MUST carry source sequence, monotonic
  time, capture API version, source kind, stable source identifier, optional
  callback event code, source-contract version, argument count, return count,
  and contiguous one-based tagged values.
- **FR-004**: Tagged values MUST losslessly represent nil, boolean, UTF-8 string,
  and every finite Lua number without relying on floating-point JSON parsing.
  One observation MUST be bounded at 256 tagged values; a larger callback MUST
  be rejected whole as declared record-limit loss.
- **FR-005**: The raw stream MUST retain all delivered scalar callback values,
  including personal names, unit tags, numeric identifiers, locations when a
  selected source supplies them, unknown enum values, and extra future values.
- **FR-006**: API reads used by the existing normalization path MUST be retained
  with ordered inputs and outputs.
- **FR-007**: Normalized compatibility events MUST identify their primary raw
  source sequence and projection ordinal. Unknown raw observations MAY have no
  projection.
- **FR-008**: Limits MUST reject a whole observation rather than truncate or
  partially retain it, and MUST declare the exact missing raw sequence range and
  reason.
- **FR-009**: Capture status MUST be partial after raw overflow,
  unsupported-value loss, callback failure, or a temporal discontinuity.
- **FR-010**: Capture MUST reserve enough capacity to store a terminal raw
  lifecycle observation, normalized discontinuity when applicable, and
  normalized terminal event.
- **FR-011**: The restricted SavedVariables parser MUST remain non-executing,
  bounded, stable-read protected, and strict about structural envelopes.
- **FR-012**: Import MUST accept valid v1 and v2 captures through explicit
  version dispatch and MUST reject unsupported versions.
- **FR-013**: Store schema v2 migration MUST preserve v1 canonical bytes and
  hashes exactly, support mixed v1/v2 records, and retain atomicity,
  immutability, idempotency, collision detection, backup, and deletion behavior.
- **FR-014**: Existing metrics, recommendations, and history MUST continue to
  consume normalized facts without displaying raw payload values.
- **FR-015**: Raw data MUST remain local and user-owned, with no upload,
  telemetry, Pixel Bus transport, input synthesis, equipment change, item use,
  navigation, or other gameplay authority.
- **FR-016**: The subscription matrix MUST document every selected callback and
  normalization-dependent API observation, its signature, rate or trigger,
  filter, projection relationship, and explicit inclusion or exclusion.
- **FR-017**: Current canonical documentation, the machine-readable encounter
  contract, governance guidance, and changelog MUST describe the v2 boundary.

### Key Entities

- **Raw source observation**: One callback delivery, API query, or capture
  lifecycle fact with ordered tagged values and source/version provenance.
- **Tagged raw value**: A one-based positional nil, boolean, exact string, or
  exact finite-number descriptor.
- **Raw loss range**: One exact contiguous range of source observations omitted
  after capture can no longer retain complete raw values.
- **Compatibility projection**: A normalized v1-style event linked to its primary
  raw source sequence for current metrics and recommendations.
- **Versioned encounter record**: A v1 legacy capture or v2 raw-authority capture
  stored immutably with its own canonical-format version.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Harness fixtures round-trip 100 percent of values and positions for
  all 11 selected encounter callbacks and every normalization-dependent API read.
- **SC-002**: Unknown result values and extra scalar arguments remain byte-for-byte
  equivalent for strings and numerically identical for all finite Lua numbers
  after Lua, parser, canonical JSON, SQLite, and reload.
- **SC-003**: Every forced incomplete path results in partial status and exact,
  machine-validated loss or temporal-discontinuity evidence.
- **SC-004**: A populated v1 store migrates with identical legacy canonical blobs
  and content hashes and can coexist with v2 records.
- **SC-005**: Existing equivalent metrics and recommendation tests remain
  unchanged in outcome, and no raw value appears in diagnostics or history UI.
- **SC-006**: Production ceiling tests remain within the 128 MiB shared file
  boundary and parser token, entry, depth, and string ceilings.
- **SC-007**: The full local merge gate and hosted CI complete successfully.

## Assumptions

- S094 covers Live API 101050 and PTS API 101051, the versions pinned by the
  managed addon manifest.
- Selected callback values are Lua nil, boolean, number, or string. Other runtime
  types are unsupported-value loss.
- Existing normalized facts remain a compatibility surface in S094. A later
  issue #186 tranche can move all normalization into a pure Rust reprocessor.
- Historical specifications remain historical; current docs and this slice
  supersede their privacy-minimized raw-capture assumptions.
