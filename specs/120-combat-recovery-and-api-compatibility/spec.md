# Feature Specification: Combat Recovery and API Compatibility

**Feature Branch**: `codex/s120-combat-recovery-and-api-compatibility`

**Created**: 2026-10-05

**Status**: Specification validated

**Input**: Implement the approved S120 bundle, issues #248, #251, and #250, under autopilot with automatic push and PR publication, at most two external Codex review rounds, and owner merge.

## User Scenarios & Testing

### User Story 1 - Restore configured light attack weaving (Priority: P1)

A player using valid detected controls and an enabled light attack slot receives the configured attack/skill sequence while the game permits input.

**Why this priority**: The owner reports complete failure of the application's core feature; timing adjustments did not help.

**Independent Test**: Deterministic binding, observation, input, and sequence fixtures trace admission through synthesis without running ESO.

**Acceptance Scenarios**:

1. **Given** valid native controls and current playable state, **When** the player activates an enabled slot, **Then** the intended attack and skill events execute with the applicable timing.
2. **Given** unchanged valid control evidence continues arriving, **When** an admitted sequence waits between events, **Then** equivalent observations do not cancel it.
3. **Given** changed controls or a real blocking state, **When** a sequence is pending, **Then** obsolete work is cancelled and application-owned pressed controls are released.
4. **Given** an inactive slot or unavailable control evidence, **When** the physical input arrives, **Then** ordinary input handling remains correct and no guessed control is synthesized.

### User Story 2 - Understand compatibility after a game update (Priority: P2)

A player can distinguish a newer client release from a changed addon API and see whether each companion package needs an update or whether compatibility is unknown.

**Why this priority**: A new ESO version was reported without an understandable warning; unresolved compatibility must survive restarts.

**Independent Test**: Version-source, persistence, managed-addon, and presentation fixtures cover current/newer/offline/malformed Live and PTS states.

**Acceptance Scenarios**:

1. **Given** the client release changes but the addon API remains supported, **When** compatibility is checked, **Then** these facts are reported separately.
2. **Given** a documented newer API outside supported declarations, **When** the application starts repeatedly, **Then** incompatibility and the update action remain visible in existing addon details.
3. **Given** unavailable, stale, malformed, or mismatched source evidence, **When** the check finishes, **Then** unknown is distinguished from supported and existing user data is preserved.
4. **Given** separate Live and PTS packages, **When** installing or updating an owned addon, **Then** the selected environment and supported declarations remain correct without affecting neighboring packages.

### User Story 3 - Keep the dashboard stationary (Priority: P3)

A player tabbing out or losing a signal keeps the existing retained HUD behavior without a temporary notification shifting the gauges.

**Why this priority**: A localized shipped layout regression can be repaired within the same runtime diagnostic slice.

**Independent Test**: Headless presentation fixtures and captured structured logging cover loss, cause change, expiry, and recovery.

**Acceptance Scenarios**:

1. **Given** a coherent HUD, **When** focus or signal is lost, **Then** no freshness notification, placeholder, or replacement popup appears.
2. **Given** retained values, **When** their retention expires or fresh evidence returns, **Then** the notification cannot move the gauges and relevant transitions remain in logs.
3. **Given** repeated renders, **When** the stale age increases, **Then** logs do not grow for every render or countdown second.

### Edge Cases

- Repeated identical native-binding snapshots versus genuinely changed or unavailable bindings.
- Gate closure between admission and execution, physical modifiers, cancellation cleanup, and repeated key events.
- Source lag, source failure, unsupported numeric API, newer client on the same API, persisted unresolved compatibility, and Live/PTS disagreement.
- Foreign, linked, unreadable, or newer managed addon manifests; install/repair must preserve ownership and user data.
- Repeated stale frames, a changed loss cause, zero retention, shortened retention, expiry, and immediate recovery.

## Clarifications

### Session 2026-10-05

- Q: Does S120 require live game reproduction? A: No; use deterministic repository seams and disclose installed/game behavior as unobserved.
- Q: Is a game patch number enough to claim API support? A: No; use independently sourced numeric API evidence, with unknown on unavailable or stale evidence.
- Q: Should the removed row be relocated? A: No; delete its UI presentation entirely and retain transition information in logs.
- Q: Does equivalent binding publication revoke queued work? A: No; only changed binding facts or real gate transitions invalidate existing authorization.
- Q: Does this slice include #249? A: No; update only copy directly affected by the three approved issues.

## Requirements

### Functional Requirements

- **FR-001**: Identify a concrete failure path for reported weaving and repair it without relying on tuning guesses.
- **FR-002**: Valid enabled light attack sequences MUST execute through the complete admission-to-synthesis path with applicable latency/bar timings.
- **FR-003**: Equivalent current binding evidence MUST NOT revoke an otherwise valid pending sequence.
- **FR-004**: Actual binding changes and blocking state transitions MUST invalidate obsolete work and preserve owned-control cleanup, focus scoping, recursion handling, and pass-through.
- **FR-005**: Determine the reported game's client/API facts from versioned sources or record an explicit evidence gap; never derive API numbers arithmetically from patch numbers.
- **FR-006**: Client release, numeric addon API, package, protocol, and data-schema identities MUST remain distinct.
- **FR-007**: Both companion addons MUST expose supported, incompatible, and unknown compatibility with actionable existing-detail text.
- **FR-008**: Persisted observations MUST NOT suppress unresolved incompatibility on subsequent starts.
- **FR-009**: Failed, stale, malformed, and environment-mismatched checks MUST NOT report confirmed current compatibility.
- **FR-010**: Addon install/update/repair MUST preserve ownership, supported declaration tokens, environment identity, neighboring addons, and saved user data.
- **FR-011**: Audit the published APIs/events used by both addons and record required adjustments or unresolved source limitations.
- **FR-012**: Remove the temporary HUD freshness notification entirely, without a replacement notification or blank reserved row.
- **FR-013**: Preserve HUD retention and live input gating while retaining meaningful loss/cause/expiry/recovery transitions in logs without render/countdown spam.
- **FR-014**: Update affected canonical guidance and report repository evidence separately from unobserved installed real-game behavior.

### Key Entities

- Binding snapshot: current detected game controls and their validity.
- Compatibility observation: channel, client release, numeric API, provenance/freshness, and package support result.
- Managed addon package: supported declarations and ownership-scoped lifecycle.
- Retention interval: loss cause, deadline, and presentation-only remembered values.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All deterministic supported light attack scenarios execute their complete intended event order; equivalent observations cause zero spurious cancellations.
- **SC-002**: Every compatibility matrix case produces the documented supported/incompatible/unknown state for both packages, including repeated starts.
- **SC-003**: Zero temporary freshness notifications or placeholders appear across loss, ageing, expiry, and recovery states.
- **SC-004**: Repeated unchanged stale frames produce zero additional transition log records.
- **SC-005**: Every requirement maps to implementation tasks and repository evidence; all mandatory automated merge gates pass before publication.

## Assumptions

- The owner report is accepted without requiring another live game session. The exact installed build was not provided.
- #249's complete messaging audit and documentation diagrams #221-#223 remain separate work.
- Existing native-binding consumption is established product behavior. Legacy constitution V language claiming PixelBeacon is fishing-only predates the completed S100-S102 program; S120 repairs existing behavior and adds no new addon surface.
- No live-game or installed-application checks are proposed. Automated repository checks remain required under the approved autopilot protocol.
- Push, official PR publication, review replies, and one additional Codex review request are authorized; merge and release are not.
