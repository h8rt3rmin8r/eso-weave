# Feature Specification: Bounded ESO Discovery Exporter

**Feature Branch**: `codex/s071-bounded-discovery-exporter`

**Created**: 2026-09-09

**Status**: Draft

**Input**: Issue #115 and the S068 source, coverage, redistribution, and
collector-input contracts, implemented against the S070 catalog compiler.

## User Scenarios & Testing

### User Story 1 - Capture API-visible catalog data safely (Priority: P1)

A user installs the separately managed ESO Weave Collector, explicitly starts a
selected discovery run outside combat, sees bounded progress, and receives a
complete SavedVariables export after the game reaches a supported save boundary.

**Why this priority**: No catalog refresh is possible until public API results
can cross the game boundary without changing gameplay or burdening PixelBeacon.

**Independent Test**: Run the addon against a deterministic mocked ESO API and
prove frame-budgeted enumeration, stable record ordering, chunk checksums,
pause/resume/cancel behavior, hard limits, and complete versus incomplete state.

**Acceptance Scenarios**:

1. **Given** an idle player and approved iterator categories, **When** the user
   starts discovery, **Then** the collector enumerates only those categories
   across bounded update ticks and emits versioned, provenance-bearing chunks.
2. **Given** combat begins during collection, **When** the next tick runs,
   **Then** work pauses without losing the checkpoint and resumes only after the
   user explicitly requests it outside combat.
3. **Given** a record, byte, string, or chunk limit would be exceeded, **When**
   collection reaches that boundary, **Then** the export becomes incomplete
   with a specific reason and cannot be mistaken for publishable input.
4. **Given** unchanged API results, **When** the same selection is captured
   again, **Then** normalized records and chunk payloads are byte-identical.

---

### User Story 2 - Validate and stage a flushed capture (Priority: P1)

A maintainer or user points the desktop importer at a flushed collector
SavedVariables file and receives either a deterministic S070 catalog bundle in
staging or a precise rejection that leaves existing files unchanged.

**Why this priority**: Safe capture has no product value unless the desktop can
consume it as hostile data without executing Lua or publishing directly.

**Independent Test**: Import valid live and PTS fixtures and reject executable,
referential, malformed, partial, oversized, deeply nested, duplicate, missing,
out-of-order, checksum-mismatched, and channel-mismatched fixtures.

**Acceptance Scenarios**:

1. **Given** a complete capture with valid checksums, **When** import runs for
   its declared channel, **Then** the importer atomically writes deterministic
   catalog-bundle JSON containing source, coverage, entities, relationships,
   localized text, attributes, and icon references.
2. **Given** arbitrary Lua syntax or an unsupported table shape, **When** import
   runs, **Then** parsing fails before any output is written or replaced.
3. **Given** an incomplete capture or broken chunk sequence/checksum, **When**
   import runs, **Then** staging fails with an actionable categorized error.
4. **Given** a PTS capture and a live expectation, **When** import runs, **Then**
   the mismatch is rejected and channel provenance remains unchanged.

---

### User Story 3 - Manage the collector independently (Priority: P2)

A user can install, inspect, update, and remove the ESO Weave Collector without
changing PixelBeacon or overwriting an addon that ESO Weave does not own.

**Why this priority**: Bulk data collection has different lifecycle and privacy
properties from the latency-sensitive safety publisher.

**Independent Test**: Exercise lifecycle operations in a temporary AddOns root
and prove marker-gated replacement/removal, path confinement, symlink refusal,
separate folder identity, and accurate reload reminders.

**Acceptance Scenarios**:

1. **Given** a valid AddOns root with no collector, **When** install runs, **Then**
   only the dedicated managed collector subtree is created.
2. **Given** an unmanaged or linked target, **When** install or uninstall runs,
   **Then** mutation is refused and every existing byte remains unchanged.
3. **Given** PixelBeacon is installed, **When** any collector lifecycle action
   runs, **Then** PixelBeacon files and status remain unchanged.

---

### User Story 4 - Understand scope, privacy, and save boundaries (Priority: P2)

A user can follow documentation that explains what is collected, why coverage
is bounded, how to flush and import data, how to cancel or remove the collector,
and which data never leaves the computer.

**Why this priority**: Consent and truthful coverage are part of the data
contract, not optional presentation polish.

**Independent Test**: Documentation policy checks confirm install, start,
progress, combat pause, save boundary, import, retry, privacy, and removal
instructions plus the live/PTS and no-upload boundaries.

**Acceptance Scenarios**:

1. **Given** a user has completed capture in game, **When** they read the status
   and guide, **Then** they are told to use `/reloadui`, logout, or exit before
   importing flushed SavedVariables.
2. **Given** a successful local import, **When** the user reviews the result,
   **Then** category coverage, capture provenance, warnings, and retained local
   text are visible without claiming exhaustive global data.

### Edge Cases

- The SavedVariables file is still open, truncated, or stale because ESO has not
  reached a save boundary.
- ESO serializes tables in an arbitrary key order or adds harmless whitespace.
- A string contains quotes, slashes, newlines, non-ASCII text, or Lua escape
  sequences.
- A table repeats a field, mixes array and map keys, references another value,
  or contains a function, expression, metatable, long string, or non-finite
  number.
- Chunk sequences are duplicated, missing, out of order, empty, or claim false
  counts and checksums.
- Live and PTS files coexist, or their declared API version is unexpected.
- Collection is paused by combat, loading, logout, cancellation, or an API error.
- An iterator returns zero, duplicate, invalid, or previously unseen IDs.
- A localized string or virtual icon path exceeds the contract limit.
- The staged output path aliases the input or an existing protected artifact.
- Disk exhaustion or interruption occurs before atomic staging publication.

## Requirements

### Functional Requirements

- **FR-001**: The collector MUST be a dedicated `EsoWeaveCollector` addon with
  its own manifest, managed marker, SavedVariables name, version, and lifecycle.
- **FR-002**: Collector installation and removal MUST stay confined to its
  resolved AddOns subtree and MUST refuse unmanaged, linked, or non-regular
  targets.
- **FR-003**: Collector lifecycle actions MUST never read, write, remove, or
  reconfigure PixelBeacon.
- **FR-004**: Collection MUST start only from an explicit user command with an
  explicit live or PTS selection, refuse to start in combat, pause on combat or
  unsafe transitions, and require an
  explicit resume.
- **FR-005**: Enumeration MUST be split across update ticks with both a record
  budget and elapsed-time budget per tick.
- **FR-006**: The initial adapters MUST cover only the approved bounded iterator
  families: player skills, crafted abilities, item sets, champion skills, and
  class/race/companion identity.
- **FR-007**: Every adapter MUST preserve stable numeric IDs, discard iterator
  indexes as identity, label its visibility boundary, and report warnings or
  unsupported entry points without manufacturing records.
- **FR-008**: The export MUST include format and collector versions, collector
  checksum, API and game versions, immutable live/PTS channel, locale, platform,
  megaserver when material, pseudonymous scope, capture times, selection,
  category coverage, warnings, cancellation reason, and completion state.
- **FR-009**: Records MUST be normalized and sorted before deterministic
  JSON-line chunking. Each chunk MUST include sequence, record count, byte count,
  payload checksum, and payload.
- **FR-010**: Collection MUST enforce the provisional 64 MiB snapshot, 500,000
  record, and 64 KiB string limits plus smaller per-chunk and nesting limits.
- **FR-011**: Cancellation or failure MUST retain a parseable incomplete envelope
  and checkpoint that cannot satisfy desktop publication rules.
- **FR-012**: The desktop MUST parse only assignment, scalar, and table syntax
  required by the collector envelope. It MUST never evaluate Lua or accept
  functions, computed expressions, references, metatables, or unknown roots.
- **FR-013**: Desktop parsing MUST enforce byte, token, depth, table-entry,
  string, record, and numeric limits before or during allocation.
- **FR-014**: Import MUST require one complete envelope, contiguous unique chunk
  sequences, exact counts and byte lengths, valid checksums, supported versions,
  and matching channel provenance.
- **FR-015**: The importer MUST convert approved records into an S070 catalog
  bundle staged as deterministic UTF-8 JSON with LF endings and no BOM.
- **FR-016**: Import MUST retain raw input SHA-256, collector identity, capture
  scope, warnings, category coverage, and per-record provenance while keeping
  user-localized strings local.
- **FR-017**: Staging MUST use a same-directory temporary file, durable flush,
  and atomic replacement. A failure MUST leave an existing output byte-identical.
- **FR-018**: Input and output paths MUST be distinct under canonical path and
  Windows Unicode case equivalence rules.
- **FR-019**: Live and PTS captures and staged bundles MUST remain structurally
  distinct. No import may promote PTS records to live.
- **FR-020**: No collector or importer path may upload data, synthesize game
  input, mutate equipment, consume items, inspect process memory, or intercept
  network traffic.
- **FR-021**: User documentation MUST cover install, initiation, category scope,
  progress, combat pause, save boundaries, import, cancel, retry, privacy,
  staging, removal, limits, and truthful coverage.
- **FR-022**: Multi-class live and PTS fixtures MUST demonstrate that identical
  stable IDs normalize consistently while bounded visibility is never labeled
  exhaustive.

### Key Entities

- **Collector Envelope**: One versioned capture with provenance, state, limits,
  category coverage, warnings, checkpoint, and ordered chunks.
- **Collector Chunk**: A contiguous deterministic payload unit with sequence,
  counts, bytes, checksum, and JSON-line records.
- **Collector Record**: One typed stable entity observation plus optional parent
  relationship, numeric attributes, local text, and virtual icon reference.
- **Coverage Declaration**: The category, visibility scope, bounded completeness,
  record count, and limitations for one adapter.
- **Import Receipt**: Input hash, versions, channel, counts, warnings, output
  hash, and staging result without personal names or sensitive local paths.
- **Checkpoint**: Adapter and cursor state sufficient for bounded continuation
  without claiming a partial snapshot is complete.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Deterministic mocked captures of unchanged data produce identical
  ordered record payloads and chunk checksums in 100 percent of repeated runs.
- **SC-002**: No update tick processes more than the configured record budget or
  continues after its configured elapsed-time budget.
- **SC-003**: Every malformed, executable, incomplete, oversized, checksum-bad,
  or channel-mismatched fixture is rejected before staging replacement.
- **SC-004**: Valid live and PTS fixtures independently produce S070 bundles that
  pass catalog model validation and deterministic compiler builds.
- **SC-005**: All lifecycle mutation tests prove writes and removals remain under
  `EsoWeaveCollector`, with zero PixelBeacon byte changes.
- **SC-006**: Every emitted category has a bounded coverage declaration and zero
  emitted categories claim exhaustive global coverage.
- **SC-007**: Importing a representative maximum-size-boundary fixture stays
  within the declared input, record, string, depth, and chunk limits.
- **SC-008**: The full formatting, strict Clippy, locked test, documentation,
  text-hygiene, and package-policy gates pass on Windows and Linux CI.

## Assumptions

- Issue #129 may later tune provisional limits or narrow visibility claims but
  does not block this contract or implementation.
- The game serializes SavedVariables only at supported lifecycle boundaries;
  the desktop cannot read in-memory addon state.
- User-collected localized text remains local and is not added to the bundled
  baseline catalog or repository fixtures beyond invented test strings.
- Adler-32 is sufficient for in-addon per-chunk accidental-corruption detection;
  the desktop computes SHA-256 over the full raw input and normalized staging
  output for durable provenance.
- A command-driven addon and compiler import command are the complete S071 user
  surfaces. Polished desktop update orchestration belongs to #118.
- Observation-session combat/effect records belong to #132 and are excluded.
- Icon image acquisition belongs to #116; S071 records virtual paths only.
- Automated source acquisition and reviewed candidate publication belong to
  #117 and are excluded.
