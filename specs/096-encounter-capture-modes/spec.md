# Feature Specification: Encounter Capture Modes

**Feature Branch**: `codex/s096-encounter-capture-modes`
**Created**: 2026-09-15
**Status**: Approved for implementation
**Input**: Issue #183, final implementation child under epic #182.

## Slice Boundary

S096 replaces the one-shot encounter controller with exactly two explicitly
selected modes: one bounded encounter and continuous-until-disabled capture.
Each combat period remains an independently validated, replayed, and stored
encounter, while a bounded outer session spool preserves shared identity,
authoritative order, controller state, interruptions, and hard failure facts.

S096 does not add a desktop-to-addon command transport, custom binding,
synthesized input, live SavedVariables write, Pixel Bus bulk transport, native
Encounter.log ingestion, source family, metric, recommendation, upload,
telemetry, gameplay action, or third capture mode. Issue #190 remains independent
Release verification for native-log qualification.

## Clarifications and Approved Deviation

The operator approved the S096 approach after S092 established that the minimum
safe desktop-to-addon command vocabulary is zero. Issue #183 still says the
desktop main interface should toggle capture. That literal requirement conflicts
with the reviewed S092 no-go decision and is deliberately replaced as follows:

- Capture authority stays inside ESO through the existing `/ewencounter` command
  surface. A selected mode plus one toggle command provides the user control;
  legacy arm, disarm, and stop commands remain narrow compatibility aliases.
- The desktop never claims or changes current addon state. It imports terminal
  records and presents spool metadata only as last-saved historical evidence.
- No binding, generated input, clipboard path, native-action piggyback, or live
  SavedVariables mutation is introduced to imitate a desktop control.

This deviation is necessary to avoid an unreliable and potentially
account-associated transport. It preserves the user outcome, explicit control
of single or continuous capture, without weakening the established safety
boundary. Constitution 4.0.0 also encodes one-shot capture and therefore requires
a major amendment authorizing exactly these two bounded, user-controlled modes.

## User Scenarios & Testing

### User Story 1 - Capture one encounter on demand (Priority: P1)

As a user, I want single mode to begin either before or during combat and stop
at the next combat exit so I can capture the fight I am currently planning or
already performing.

**Why this priority**: Single mode preserves the bounded workflow while removing
the current surprising refusal to start during combat.

**Independent Test**: Execute the production addon outside and inside combat,
toggle single mode, drive the next combat exit, and import the resulting terminal
record with exact authority, loss, state, and replay assertions.

**Acceptance Scenarios**:

1. **Given** single mode is selected outside combat, **When** the user enables it,
   **Then** the controller waits for the next authoritative combat entry and
   stops after that encounter exits combat.
2. **Given** the player is already in combat, **When** the user enables single
   mode, **Then** capture begins immediately from an exact combat-state API sample,
   records the unknown pre-activation prefix, and stops at the next combat exit.
3. **Given** single mode is waiting or capturing, **When** the user toggles it off,
   **Then** waiting authority is cancelled or the active record is finalized as
   partial without deleting retained evidence.

---

### User Story 2 - Capture a continuous session (Priority: P1)

As a user, I want continuous mode to span repeated combat periods until I turn
it off so a dungeon, trial, or practice session does not require rearming.

**Why this priority**: This is the unattended multi-fight outcome that issue #183
exists to deliver.

**Independent Test**: Enable continuous mode once, drive multiple combat entries
and exits with gaps, disable it, and verify one session identity, contiguous
ordinals, independent encounter replay, and complete ordered import.

**Acceptance Scenarios**:

1. **Given** continuous mode is enabled, **When** multiple combat periods occur,
   **Then** each period becomes a separate terminal encounter under one session
   with a unique identity and contiguous ordinal.
2. **Given** a combat period ends, **When** the controller is still enabled,
   **Then** detailed handlers are removed during the gap and the next combat
   entry starts automatically without another user action.
3. **Given** continuous mode is waiting or capturing, **When** the user toggles
   it off, **Then** the session stops, and an active encounter is finalized as a
   truthful user-stopped partial record.

---

### User Story 3 - Preserve truth through interruption and pressure (Priority: P1)

As a user, I want reloads, relogs, failures, and storage pressure represented
explicitly so the product never calls an interrupted session continuous.

**Why this priority**: Continuous collection magnifies silent-loss risk and must
fail closed before convenience can be trusted.

**Independent Test**: Interrupt waiting and active modes, exhaust each aggregate
budget, inject callback failure and malformed persisted state, and verify exact
markers, terminal records, controller transitions, and non-destructive recovery.

**Acceptance Scenarios**:

1. **Given** a durably saved active continuous encounter, **When** the addon
   reloads or the player relogs, **Then** that encounter becomes partial, one
   bounded interruption marker is retained, and explicit continuous authority
   returns to waiting under the same session.
2. **Given** single mode is interrupted while active, **When** recovery runs,
   **Then** its record becomes partial and the controller stops rather than
   silently starting another encounter.
3. **Given** aggregate storage cannot reserve a complete terminal record and
   failure marker, **When** another observation or encounter is attempted,
   **Then** retained records remain unchanged and the controller enters a visible
   hard-failure state with no silent eviction or overwrite.

---

### User Story 4 - Import and inspect ordered session history (Priority: P2)

As a user, I want the desktop to import every terminal encounter from the last
saved spool atomically and show its session context without claiming live control.

**Why this priority**: Multiple captures are useful only when they remain
independently trustworthy and authoritatively ordered after ingestion.

**Independent Test**: Import growing, repeated, malformed, colliding, and partial
session spools and verify all-or-nothing writes, idempotent receipts, legacy
compatibility, grouped order, and last-saved wording.

**Acceptance Scenarios**:

1. **Given** a valid spool with multiple terminal encounters, **When** it is
   imported, **Then** every member is validated and replayed before one atomic
   transaction stores imported or already-present records in ordinal order.
2. **Given** any invalid member, duplicate ordinal, identity collision, or
   malformed controller fact, **When** import is attempted, **Then** no member is
   written and existing history remains unchanged.
3. **Given** the spool contains an active encounter, **When** the desktop reads
   it, **Then** only terminal members are imported and the active facts are shown
   solely as last-saved historical state.
4. **Given** a legacy v1 or v2 singleton capture, **When** it is imported, **Then**
   it remains one ordinal-1 single session with unchanged canonical bytes and hash.

### Edge Cases

- Enabling a mode while already in combat must use an API or control observation,
  never fabricated callback arguments.
- A combat exit can occur immediately after mid-combat activation.
- Continuous gaps can include zone transitions, addon reload, relog, and desktop exit.
- A raw-loss partial encounter can be retained while continuous mode proceeds;
  callback failure or aggregate exhaustion enters a persisted hard-failure
  state. A malformed same-version controller is preserved unchanged and treated
  as an inactive `state-invalid` hard failure because rewriting it would destroy
  user-owned evidence.
- Timestamps can repeat or move backward; ordinal order remains authoritative.
- Re-importing a growing spool must not rewrite or reorder already-present records.
- Empty Lua arrays, sparse or non-integer ordinals, duplicate identities, and
  unsupported modes or states fail closed.
- Diagnostics may name controlled states, ordinals, and counts but never raw values.
- An operating-system or game crash before ESO flushes SavedVariables cannot be
  reconstructed and is documented as an external durability boundary.

## Requirements

### Functional Requirements

- **FR-001**: The encounter controller MUST expose exactly `single` and
  `continuous` selected modes and reject every other value.
- **FR-002**: Only explicit user action inside ESO MAY enable, disable, or change
  capture mode. The desktop command vocabulary MUST remain zero.
- **FR-003**: One in-game toggle command MUST enable the selected mode when
  stopped and disable it when waiting or capturing. Existing arm, disarm, and
  stop forms MAY remain compatibility aliases without adding a mode.
- **FR-004**: Controller state MUST distinguish selected mode, requested
  enablement, effective stopped/waiting/capturing/interrupted/failed state, active mode and
  channel, current encounter, last interruption, and hard failure.
- **FR-005**: Single mode enabled outside combat MUST wait for one encounter and
  stop after its combat exit.
- **FR-006**: Single mode enabled during combat MUST start immediately, declare
  the unknown prefix, and stop at the next combat exit.
- **FR-007**: Continuous mode MUST retain one session identity, create one
  independently replayable record per combat period, assign contiguous ordinals,
  wait through gaps, and continue until explicit disablement or hard failure.
- **FR-008**: Detailed high-frequency handlers MUST be registered only while an
  encounter is actively capturing.
- **FR-009**: A versioned outer spool MUST contain controller facts, terminal
  encounter members, at most one active member, bounded interruption markers,
  aggregate counts, and explicit failure metadata.
- **FR-010**: Every terminal member MUST remain a self-contained schema-v2
  capture with exact raw values, explicit loss, structural validation, and
  deterministic replay appropriate to its producer profile.
- **FR-011**: Mid-combat activation MUST add exact supported authority and replay
  rules without fabricating `EVENT_PLAYER_COMBAT_STATE` input.
- **FR-012**: Aggregate spool use MUST remain within 100,000 normalized events,
  100,000 raw observations, 32 MiB estimated encounter data, 1,024 terminal
  encounters, and 1,024 interruption markers, with reserved terminal failure space.
- **FR-013**: No aggregate limit MAY silently evict, overwrite, truncate, or
  ring-buffer retained evidence. Exhaustion MUST stop capture as a visible hard failure.
- **FR-014**: Reload or relog recovery MUST finalize a durably saved active member
  as partial, record one interruption, resume only an explicitly enabled
  continuous session, and stop an interrupted single session.
- **FR-015**: Desktop exit MUST NOT change addon authority. Unflushed game or OS
  crash loss MUST be documented and never inferred as observed.
- **FR-016**: Import MUST validate controller structure and every terminal member
  before writing, then commit all imported records and session facts atomically.
- **FR-017**: Re-import MUST be idempotent. Same identity and content is
  already-present; same identity with different content rejects the whole batch.
- **FR-018**: The store MUST persist mode, authoritative ordinal, and bounded
  session interruption/failure facts while preserving v1-v3 database migration
  and all legacy canonical bytes and hashes.
- **FR-019**: History MUST order encounters by session identity and ordinal, keep
  metrics and recommendations encounter-local, and expose batch import counts.
- **FR-020**: Desktop UI and diagnostics MUST label spool controller facts as
  last-saved or historical and MUST NOT present them as live state or controls.
- **FR-021**: Capture, import, history, and UI diagnostics MUST remain value-free.
- **FR-022**: Tests MUST execute production Lua and cover both modes, every
  transition, mid-combat activation, repeated encounters, recovery, each bound,
  malformed hostile input, atomic retry, legacy dispatch, replay, UI provenance,
  and production ceilings.
- **FR-023**: The constitution, canonical manuals, architecture, status reference,
  test strategy, machine encounter model, changelog, and chronological build plan
  MUST describe the delivered boundary without preserving obsolete one-shot claims.
- **FR-024**: Local merge gates, hosted CI, code review, and security review MUST
  pass with all review threads resolved before operator merge approval.

### Key Entities

- **Capture mode**: The selected `single` or `continuous` operating policy.
- **Encounter spool**: The bounded SavedVariables controller and retained evidence.
- **Capture session**: One explicit enablement period with stable identity and mode.
- **Session member**: One terminal, independently replayable encounter plus ordinal.
- **Interruption marker**: Bounded evidence of a reload, relog, or recovery gap.
- **Session failure**: A controlled hard stop caused by pressure, callback failure,
  or invalid recovered state.
- **Batch import receipt**: Imported and already-present counts plus last-saved
  session facts, without raw values.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Production-Lua tests cover 100 percent of defined transitions for
  both modes before combat, during combat, between encounters, and on disablement.
- **SC-002**: A continuous fixture retains at least three independently replayed
  encounters under one session with contiguous ordinals after one user enablement.
- **SC-003**: Every aggregate and per-encounter limit has a boundary test proving
  preserved evidence, reserved terminal facts, and no silent eviction.
- **SC-004**: Malformed member and identity-collision tests produce zero partial
  database writes, while repeated and growing valid imports are idempotent.
- **SC-005**: All legacy capture/store fixtures preserve canonical bytes, hashes,
  and encounter-local metric results through migration.
- **SC-006**: Desktop tests find zero capture mutation paths and label 100 percent
  of spool state facts as last-saved or historical.
- **SC-007**: Source inspection finds exactly two modes and zero custom bindings,
  generated-input commands, live SavedVariables writers, or new bulk transports.
- **SC-008**: The full local merge gate and hosted CI pass with every review resolved.

## Assumptions

- The existing supported SavedVariables path remains the capture and import
  fallback while issue #190 independently evaluates native logging.
- Explicit continuous enablement persists across addon reload and relog until
  disablement or hard failure; this persistence is part of the selected mode.
- Each encounter resets actor interning, sequences, raw-loss state, and replay
  assessment even when it shares a continuous session.
- The outer spool advances independently of the inner capture schema so old
  terminal records remain self-contained and byte-preserving.
- The canonical manual and changelog remain the publication surface. The
  repository has no blog subsystem, and adding one would be disproportionate.
