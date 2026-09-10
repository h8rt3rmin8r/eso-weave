# Feature Specification: Privacy-Minimized Encounter Capture

**Feature Branch**: `codex/s075-encounter-capture`

**Created**: 2026-09-09

**Status**: Ready for implementation

**Input**: Issue #132 and the S069 encounter model

## User Scenarios and Testing

### User Story 1 - Capture one encounter locally (Priority: P1)

A player explicitly arms the ESO Weave Encounter addon for Live or PTS, enters
combat, and receives one bounded local SavedVariables capture containing the
observed encounter without names or other personal identifiers.

**Why this priority**: Without a trustworthy raw capture there is no input for
the later import, calculation, interface, or recommendation slices.

**Independent Test**: Arm the addon in a deterministic ESO API harness, emit a
representative encounter, end combat, and verify a complete envelope containing
ordered encounter boundaries and every required event family.

**Acceptance Scenarios**:

1. **Given** an idle addon, **When** the user enters `/ewencounter arm live` and
   combat starts, **Then** the addon creates a new anonymous session and begins
   recording exactly one encounter.
2. **Given** an active capture, **When** damage, healing, effect, resource,
   action-slot, weapon-pair, life-state, boss, performance, and quickslot
   observations arrive, **Then** normalized events retain numeric game facts in
   authoritative sequence order.
3. **Given** an active capture, **When** combat ends, **Then** the addon records
   an encounter end, marks the envelope complete or partial, disarms, and tells
   the user that ESO must flush SavedVariables before desktop import.

---

### User Story 2 - Preserve truth when capture is incomplete (Priority: P1)

A player whose capture reaches a limit, reloads, changes zones, or observes a
clock reset receives an explicit partial result with declared loss rather than a
silent or falsely complete encounter.

**Why this priority**: Hidden event loss would make later metrics look precise
while being mathematically incorrect.

**Independent Test**: Inject record, byte, reload, deactivation, and backward
clock boundaries and verify that each surviving stream has an ordered
discontinuity and terminal boundary without exceeding its limits.

**Acceptance Scenarios**:

1. **Given** a capture near its event or byte limit, **When** another source
   observation arrives, **Then** later observations are counted as omitted and a
   reserved discontinuity plus encounter-end record fit within the bound.
2. **Given** the game clock moves backward, **When** the next event is observed,
   **Then** the exported duration clock remains nondecreasing and the reset is
   declared as a discontinuity.
3. **Given** the player reloads or deactivates while capturing, **When** the addon
   receives the boundary callback, **Then** it finalizes a partial envelope with
   a stable reason before SavedVariables are written.

---

### User Story 3 - Control and clear local capture state (Priority: P2)

A player can inspect capture status, disarm before combat, stop an active
capture, and explicitly clear the saved envelope without affecting PixelBeacon,
the discovery collector, or any desktop data.

**Why this priority**: Local ownership requires understandable consent,
retention, and deletion controls.

**Independent Test**: Exercise arm, disarm, stop, status, and clear commands in
each legal state and prove that only the encounter SavedVariables root changes.

**Acceptance Scenarios**:

1. **Given** an armed addon outside combat, **When** the user disarms it, **Then**
   no later combat begins capture until the user arms it again.
2. **Given** an active encounter, **When** the user stops capture, **Then** a
   partial terminal envelope is retained with a user-stopped reason.
3. **Given** a terminal envelope, **When** the user confirms the clear command,
   **Then** only the encounter capture is replaced with an idle envelope.

### Edge Cases

- Arming while already in combat waits for the next clean combat boundary.
- Repeated arm or stop commands are idempotent and report the current state.
- An API callback with zero or absent ability or actor identity remains usable
  through explicit unknown numeric values and encounter-local actor allocation.
- Combat callbacks containing account, character, ability, effect, or unit names
  never persist those strings.
- Boss unit tags disappearing between samples do not fabricate health values.
- A resource or effect callback for an unsupported unit does not leak its unit
  tag and is either normalized to a local actor or ignored by documented policy.
- A reload before any combat retains armed intent only when the user explicitly
  armed the addon; it never starts capture merely because the addon loaded.
- A full capture is never overwritten by a new arm command. The user must clear
  it explicitly before capturing another encounter.

## Clarifications

### Session 2026-09-09

- Q: Is encounter capture automatic whenever the addon is installed? A: No.
  The user explicitly arms one Live or PTS capture, and the addon disarms after
  one encounter.
- Q: Does this slice add desktop import or persistence? A: No. S075 defines and
  emits the bounded SavedVariables handoff. Issue #133 owns hostile-data parsing,
  atomic import, and the user-owned encounter store.
- Q: How is privacy enforced? A: Persist only numeric facts, stable enumerations,
  timestamps, and encounter-local opaque actor IDs. Ignore all callback names,
  account handles, character names, chat, guild, and location.
- Q: How does the addon prove a cryptographic content hash? A: It does not.
  The future desktop importer canonicalizes the fixed envelope and calculates
  SHA-256. The addon records bounded source facts and counters only.
- Q: Does S075 manage installation from the desktop? A: No. It adds a distinct
  versioned addon artifact and confinement contract. A future desktop lifecycle
  may install it only through a separately marker-gated manager.
- Q: How is overflow represented within a hard bound? A: Capacity is reserved
  for one discontinuity and one encounter-end event. Once regular capacity is
  exhausted, source sequence and omission counters continue, then finalization
  emits the declared missing range and terminal event.

## Requirements

### Functional Requirements

- **FR-001**: The repository MUST contain a dedicated `EsoWeaveEncounter` addon
  with its own manifest, SavedVariables root, version, and managed identity,
  separate from PixelBeacon and ESO Weave Collector.
- **FR-002**: Capture MUST remain dormant after addon load and MUST require the
  explicit `/ewencounter arm live|pts` command.
- **FR-003**: Arming MUST select Live or PTS explicitly, MUST NOT infer or promote
  a channel, and MUST capture at most one subsequent clean encounter.
- **FR-004**: Starting an encounter MUST create opaque session and encounter IDs
  without account, character, guild, or location identity.
- **FR-005**: Every raw event MUST contain session ID, encounter ID, sequence,
  nondecreasing monotonic milliseconds, kind, and a fixed kind-specific numeric
  or enumerated payload under a versioned capture envelope.
- **FR-006**: Sequence MUST be the ordering authority. Wall-clock timestamps MAY
  describe provenance but MUST NOT order observations or calculate durations.
- **FR-007**: The capture MUST support `encounter-start`, `encounter-end`,
  `damage`, `healing`, `effect`, `resource`, `cast`, `bar-change`, `death`,
  `resurrection`, `boss-health`, `performance`, `quickslot`, and
  `discontinuity` event kinds.
- **FR-008**: Damage and healing MUST retain the action result, ability ID,
  amount, overflow, damage or power type, source and target local actor IDs, and
  combat-unit types when the ESO API supplies them.
- **FR-009**: Effect events MUST retain change type, ability ID, stack count,
  begin and end timing facts, effect/status/ability types, target local actor ID,
  and source combat-unit type without persisting effect or unit names.
- **FR-010**: Resource events MUST retain local actor ID, power type, value,
  maximum, and effective maximum without persisting unit tags.
- **FR-011**: Cast, bar-change, quickslot, boss-health, performance, death, and
  resurrection events MUST expose the minimum numeric facts identified by the
  S069 contract without adding personal text.
- **FR-012**: Actor mapping MUST be encounter-local, opaque, deterministic within
  one capture, and discarded from runtime memory after finalization or clear.
- **FR-013**: The capture MUST enforce explicit event, estimated-byte, actor, and
  string bounds before appending an event.
- **FR-014**: The event and byte budgets MUST reserve enough capacity for one
  discontinuity and one encounter-end record.
- **FR-015**: Overflow, reload, player deactivation, clock reset, user stop, and
  internal callback failure MUST become stable partial reasons or declared
  discontinuities rather than silent loss.
- **FR-016**: A discontinuity MUST identify the immediately preceding omitted
  source-sequence range and a stable reason.
- **FR-017**: Finalization MUST set terminal status, start and end clocks, first
  and last sequence, stored and omitted counts, estimated bytes, and warnings.
- **FR-018**: A complete existing capture MUST NOT be overwritten by arming.
  The user MUST explicitly clear it first.
- **FR-019**: Commands MUST support arm, disarm, stop, status, clear confirmation,
  and help with stable state-aware messages.
- **FR-020**: Clearing MUST mutate only `EsoWeaveEncounterSaved`; it MUST NOT
  remove addon files or touch discovery, catalog, PixelBeacon, configuration,
  input, or automation state.
- **FR-021**: Capture MUST use only documented public addon APIs and local
  SavedVariables. It MUST NOT access process memory, packets, files, network,
  protected actions, item use, equipment changes, movement, input synthesis, or
  multi-account control.
- **FR-022**: Bulk encounter data MUST NOT use Pixel Bus and MUST NOT import,
  calculate metrics, drive recommendations, or authorize action automation.
- **FR-023**: Runtime callbacks MUST remain bounded and sampling MUST use a fixed
  cadence so capture cannot intentionally stall the interface thread.
- **FR-024**: Callback errors MUST be contained, recorded as a partial result,
  and unregister active event and sampling handlers.
- **FR-025**: Repository tests MUST verify addon identity, required event
  coverage, privacy exclusions, bounds, explicit consent, separation, callback
  teardown, and forbidden API absence.
- **FR-026**: Canonical documentation MUST explain arming, capture status,
  SavedVariables flush, privacy, loss, clearing, and the handoff to issue #133.
- **FR-027**: The constitution and agent guidance MUST explicitly permit the
  third narrow encounter addon while preserving separate confinement and all
  existing safety gates.
- **FR-028**: S075 MUST update the active chronological build plan, migration
  ledger, and changelog without claiming live Combat Metrics parity.

### Key Entities

- **Capture State**: Dormant, armed, capturing, complete, or partial state plus
  the explicitly chosen channel and user authority.
- **Capture Envelope**: One bounded SavedVariables document containing schema,
  addon/API/game provenance, privacy profile, terminal counters, and events.
- **Raw Event**: Ordered numeric observation with one fixed event kind and
  kind-specific payload.
- **Actor Registry**: Runtime-only mapping from callback-local identities to
  opaque encounter-local integers.
- **Discontinuity**: Stored declaration of an immediately preceding omitted
  source-sequence range and stable cause.
- **Capture Budget**: Maximum regular events, actors, estimated serialized bytes,
  and reserved terminal capacity.

## Success Criteria

### Measurable Outcomes

- **SC-001**: A deterministic representative encounter produces all fourteen
  required event kinds in strictly increasing sequence with nondecreasing time.
- **SC-002**: Privacy tests find zero persisted personal names, account handles,
  character names, chat, guild, location, or callback unit-tag values.
- **SC-003**: Event, byte, and actor overflow fixtures exceed zero configured
  limits while every exported envelope remains within all bounds and declares
  its exact omitted range.
- **SC-004**: One hundred percent of complete and partial terminal paths
  unregister capture callbacks and sampling updates.
- **SC-005**: Repeated command-state tests prove no capture starts without an
  explicit arm and no completed capture is overwritten without explicit clear.
- **SC-006**: Static confinement tests find no Pixel Bus transport, upload,
  protected action, item use, equipment mutation, input synthesis, automation,
  process-memory, or packet API surface.
- **SC-007**: The existing PixelBeacon and discovery collector addon artifacts
  remain byte-identical throughout S075 tests and lifecycle operations.
- **SC-008**: Documentation, JSON, UTF-8, forbidden-dash, mojibake, Rust format,
  strict Clippy, and full locked test gates pass.

## Assumptions

- S069 remains the authority for event vocabulary, ordering, loss, privacy,
  storage-plane separation, and later live parity evidence.
- API 101050 Live and 101051 PTS event signatures from the pinned ESOUI source
  are sufficient for repository implementation. Issue #131 owns same-parse live
  comparison and issue #129 owns broader field visibility verification.
- ESO writes SavedVariables only at supported reload, logout, or exit boundaries.
- Issue #133 may evolve the serialized importer contract only through an
  explicit versioned migration. S075 does not parse its own output on desktop.
- Estimated bytes are a conservative runtime guard, not a claim about exact ESO
  serialization size. Live measurements remain verification work.

## Out of Scope

- Desktop import, hostile Lua parsing, local encounter database, migration,
  backup, deletion UI, or cryptographic raw-content hashing.
- Metric calculation, Combat Metrics parity claims, encounter presentation,
  build analysis, recommendations, or generated gameplay action.
- Automatic capture, continuous history, remote upload, sharing, telemetry, or
  network transfer.
- Stable cross-encounter actor identity or opt-in personal-name capture.
- Desktop installation controls for the encounter addon.
- Any change to PixelBeacon protocol, discovery collector categories, catalog
  candidates, user catalog updates, or input authorization.
