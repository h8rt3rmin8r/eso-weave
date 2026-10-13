# Feature Specification: Retained Recording Sessions

**Feature Branch**: codex/s126-retained-recording-sessions
**Created**: 2026-10-13
**Status**: Ready for planning
**Input**: Owner kickoff S126, implementing [issue #273](https://github.com/h8rt3rmin8r/eso-weave/issues/273) after S125 / PR #274.


## Clarifications

### Session 2026-10-13

Autopilot answers, evaluated against #273 and constitution 5.0.1:

- Q: Does start resume or begin another session? A: Start while off always begins a distinct session; explicit resume continues only the latest stopped continuous session.
- Q: Which retention policy replaces manual clearing? A: A bounded ordered retained spool preserves earlier terminal data; no overwrite, expiry or import-proof requirement.
- Q: How should mixed environments import? A: Validate every session, then import only the selected environment in retained order; provenance remains per session.
- Q: What should discard remove? A: Exactly the identified inactive session after confirmation, or all addon sessions only through clear confirm; preserve catalog and desktop history.
- Q: What happens to older/invalid data? A: Supported data migrates without terminal changes; invalid or unsupported data remains inactive and recoverable.

Coverage: functional scope, identity, transitions, interaction, exceptions, security,
integration and completion signals are resolved. Numeric budgets and schema
representation are planning decisions, with no unresolved owner decision.

## User Scenarios & Testing

### User Story 1 - Record another fight without deleting the first (Priority: P1)

A player completes a single-fight session, then explicitly starts another session.
Both finished recordings remain available until deliberate discard.

**Why this priority**: Ordinary recording must not depend on deletion or desktop import.
**Independent Test**: Complete two single sessions, save and reload, then import both in their original order.

**Acceptance Scenarios**:

1. **Given** a completed session, **When** another session starts and completes, **Then** both retain their identities, exact data and order.
2. **Given** retained completed or valid failed data, **When** a new session starts, **Then** that data remains unchanged and the new session has a distinct identity.
3. **Given** recording is already enabled, **When** start repeats, **Then** it reports current state without creating another session.
4. **Given** stopped recording, **When** mode or environment changes, **Then** the selection applies to the next session and does not relabel older evidence.

### User Story 2 - Resume or begin a new continuous session deliberately (Priority: P1)

A player stops continuous recording, then chooses either to resume that same
session or to begin a new session.

**Why this priority**: These actions have different chronology and identity effects.
**Independent Test**: Record, stop, resume, record, stop and start new; verify session and encounter identities.

**Acceptance Scenarios**:

1. **Given** a stopped continuous session, **When** resume is requested, **Then** existing terminal fights remain unchanged and subsequent fights continue its ordinal sequence and original environment.
2. **Given** a stopped continuous session, **When** start is requested, **Then** the prior session is retained and a distinct session begins.
3. **Given** a single, failed, absent or archived session, **When** resume is requested, **Then** a specific explanation is shown without modifying retained data.
4. **Given** resumed recording begins during combat, **When** it captures, **Then** missing earlier observations remain explicitly partial.

### User Story 3 - Understand capacity and discard only what is intended (Priority: P2)

A player can see retained-session counts and available capacity, and deliberately
discard a selected inactive session or all addon recordings.

**Why this priority**: Bounded retention must stop transparently without hidden replacement.
**Independent Test**: Fill a small test capacity, attempt start, discard one selected session and retry.

**Acceptance Scenarios**:

1. **Given** capacity is full, **When** start or resume is attempted, **Then** it leaves all prior evidence unchanged and explains save/import and explicit discard choices.
2. **Given** multiple retained sessions, **When** a selected inactive session is discarded with confirmation, **Then** only that session is removed, identities and order of survivors are unchanged, and catalog data and desktop history remain intact.
3. **Given** no confirmation, an unknown identifier or active recording, **When** discard is attempted, **Then** no recording is deleted.
4. **Given** explicit clear-all confirmation, **When** clearing completes, **Then** addon recordings are removed, catalog data stays intact, and later sessions cannot reuse prior identities.

### User Story 4 - Recover older data and import retained sessions (Priority: P1)

A player upgrades with existing saved recordings, then saves, reloads, inspects
and imports a multi-session recording source.

**Why this priority**: Retention must preserve user data and existing duplicate behavior.
**Independent Test**: Upgrade legacy terminal and controller sources, retain another session, import twice and compare the source and history.

**Acceptance Scenarios**:

1. **Given** supported existing recordings, **When** the addon upgrades, **Then** terminal capture identities and payloads remain unchanged; unsupported or invalid evidence remains preserved and inactive.
2. **Given** a valid source containing multiple environments, **When** importing for one selected environment, **Then** only its terminal recordings import in retained session and fight order; the other environment remains preserved.
3. **Given** repeated imports or a later resumed session, **When** import repeats, **Then** existing history is preserved and only newly completed fights are added.
4. **Given** malformed data in any retained session, **When** import is requested, **Then** the entire import is rejected before writing history.
5. **Given** an unfinished fight, **When** last-saved source is inspected, **Then** it stays distinct from completed importable fights and current in-game state.

### Edge Cases

- Empty stopped sessions, partial fights, valid failed sessions and runtime interruption.
- Same timestamp or game-clock values across starts, reloads and explicit deletion.
- Aggregate observation, byte, fight, interruption and session ceilings across retained and current data.
- Capacity refusal before an existing session is archived or user selections change.
- Selected environment differs from the latest session but matches an earlier retained session.
- Unknown versions, nested retained containers, duplicate session or encounter identities, ordering gaps after deliberate discard and counter overflow.
- Invalid discard arguments, attempted active-session deletion and stale desktop source results.
- Chat initialization prevents delivery of the existing recording warning.
- Exact raw values remain local; status and public test evidence contain synthetic counts and identities only.

## Requirements

### Functional Requirements

- **FR-001**: Start MUST begin a distinct session after stop/completion without requiring import or deletion, preserving all earlier terminal records.
- **FR-002**: Session creation order and stable session/fight identities MUST survive save, reload, migration, resume, repeated import and discard.
- **FR-003**: Explicit resume MUST continue only the most recent stopped continuous session, with original environment and next encounter ordinal; start MUST always mean a new session when recording is off.
- **FR-004**: Stopped mode/environment selection MUST affect only future sessions; active requests MUST remain idempotent and explain stop-first when selections differ.
- **FR-005**: Retention MUST have documented aggregate ceilings for sessions, fights, observations, interruptions and estimated bytes; all limits MUST preserve terminal/failure reserves, declare loss and prohibit silent eviction.
- **FR-006**: Supported saved data MUST migrate without changing terminal payloads or identities. Invalid/unsupported data MUST remain recoverable and inactive rather than being silently replaced.
- **FR-007**: Selected-session discard and clear-all MUST require explicit confirmation and stopped recording, preserve surviving data, catalog state and desktop history, and never delete the shared source file from the desktop.
- **FR-008**: Import MUST validate the entire retained source before one atomic history transaction, filter by selected environment, preserve authoritative order and remain deterministic under duplicate imports.
- **FR-009**: In-game status/session listing and desktop last-saved inspection MUST report retained counts, per-session provenance and useful start/resume/capacity/discard actions without exposing raw payloads.
- **FR-010**: Desktop inspection MUST remain read-only and serialized, reject stale path/environment/generation results, and distinguish saved source from current game state.
- **FR-011**: Recording authority, warning delivery, exact callback retention, partial/loss semantics, managed addon inventory and catalog isolation MUST retain their established guarantees.
- **FR-012**: Canonical and bundled workflow documentation MUST explain start versus resume, saved-source lag, retention limits, migration, environment-filtered import and explicit discard effects.
- **FR-013**: This slice MUST use repository regression evidence and bounded external review; it MUST NOT introduce native-log ingestion, live tailing, input changes, automatic import acknowledgment, a release or owner field-verification tasks.

### Key Entities

- **Recording session**: Stable identity, creation order, environment, mode, lifecycle and ordered encounters.
- **Retained recording spool**: Current session and immutable earlier sessions, bounded as a whole.
- **Terminal fight**: Complete or explicitly partial evidence belonging to exactly one session and ordinal.
- **Saved source summary**: Last-saved per-environment counts and source provenance, distinct from game state.
- **Discard request**: Explicitly confirmed selection that removes addon evidence only.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Two successive single sessions retain and import both fights without any intervening deletion.
- **SC-002**: Stop/resume retains one continuous identity; stop/start creates another, with zero changes to previously finished fights.
- **SC-003**: Reload and supported migration preserve every terminal fight identity and exact payload.
- **SC-004**: Reimport adds zero duplicates; importing again after another completed fight adds exactly that new fight.
- **SC-005**: Every tested capacity refusal leaves prior recordings unchanged; confirmed selected discard removes exactly one session.
- **SC-006**: Mixed-environment import adds zero recordings from the unselected environment and preserves the complete source.
- **SC-007**: Invalid-source imports make zero history writes and expose no raw values in diagnostics.

## Assumptions

- ESO continues to persist addon data on reload, logout or exit; desktop import does not acknowledge or clear in-memory addon data.
- Existing single/continuous capture and raw-record semantics remain authoritative.
- Only the most recent session can be resumed; archived sessions remain immutable.
- Native terminal-file import belongs to #272 and is not a dependency of S126.
- Owner merge and publication remain separate; automatic branch push and PR publication are explicitly authorized.
