# Feature Specification: World Transition State

**Feature Branch**: `codex/049-world-transition-state`
**Created**: 2026-09-05
**Status**: Implemented
**Input**: GitHub issue #56 and build plan 019

## Scope and clarification decisions

S049 publishes one authoritative world lifecycle observable from the ESO addon
through the desktop application. It does not add pre-loading travel detection or
change any synthesized-input gate. Those behaviors remain in #59.

All routine choices were resolved under the build-phase autopilot policy:

- B22 is a discrete marker and complement-checksum block, matching the established
  protocol rather than overloading heartbeat or life state.
- Unknown is a first-class wire value before initial player activation and the
  companion fallback for missing or invalid evidence.
- Transitioning is published directly from `EVENT_PLAYER_DEACTIVATED`.
- Active is published only at the end of the activation callback, after every
  dependent player payload has been freshly recomputed and rendered.
- No periodic function is allowed to infer Active. Events own the lifecycle;
  signal validation owns Unknown.
- The state belongs to the shared game observation model, not to a synthesis
  controller, because this slice observes and presents without consuming it.

## User Scenarios and Testing

### User Story 1 - See when the world is changing (Priority: P1)

As a player, I want ESO Weave to distinguish an active world from a loading or
zoning transition so the dashboard does not imply that stale gameplay readings
describe my current character state.

**Independent Test**: Decode every valid B22 value and invalid evidence, route it
to the shared game observation model, and assert the matching System and State text.

**Acceptance Scenarios**:

1. **Given** the character is active after a complete baseline, **When** B22 is
   sampled, **Then** the application presents World state as Active.
2. **Given** ESO begins a loading transition, **When** the deactivation event is
   received, **Then** B22 immediately publishes Transitioning.
3. **Given** B22 is absent, corrupt, or lost, **When** the application renders,
   **Then** World state reads Not detected.
4. **Given** the ESO process is inactive, **When** the application renders,
   **Then** World state reads Game not active.

---

### User Story 2 - Never claim a stale baseline is active (Priority: P1)

As an operator, I want Active to mean that every dependent PixelBeacon payload
has been refreshed for the current world so consumers can later use the state as
a reliable safety boundary.

**Independent Test**: Inspect and exercise the addon lifecycle callbacks and
assert that deactivation renders Transitioning immediately and activation renders
Active only after the full baseline function returns.

**Acceptance Scenarios**:

1. **Given** the player deactivates, **When** the callback runs, **Then** the
   world block changes before any later loading interval.
2. **Given** the player activates, **When** the callback runs, **Then** weapon,
   combat, menu, resources, movement, cooldowns, quickslot, life, and fishing
   state are refreshed before Active is rendered.
3. **Given** the addon has loaded but player activation has not occurred,
   **When** B22 is visible, **Then** it publishes Unknown rather than Active.

## Edge Cases

- A loading screen or compositor obscures the addon before Transitioning is
  sampled. The reader eventually reports Unknown through invalid evidence or
  heartbeat loss, never a fabricated transition value.
- An older addon has only B0 through B21. The reader reports Unknown for B22 and
  the existing managed-addon Update action remains available.
- Activation occurs after a reload. The addon starts Unknown, rebuilds every
  payload, then publishes Active.
- A payload API returns unavailable data during activation. The payload's own
  unavailable encoding is still a fresh baseline, so Active remains truthful.
- Duplicate activation or deactivation events are idempotent and emit no reader
  event unless the normalized state changes.
- Game process exit clears the shared observation to Unknown even before a
  pixel-bus timeout.

## Requirements

### Functional Requirements

- **FR-001**: PixelBeacon MUST publish B22 with distinct Unknown, Transitioning,
  and Active payloads, a unique green marker, and the complement checksum.
- **FR-002**: Addon and companion constants MUST agree on block count, index,
  marker, payloads, and checksum behavior.
- **FR-003**: The addon MUST initialize world state to Unknown and MUST NOT infer
  Active during addon construction or a periodic tick.
- **FR-004**: `EVENT_PLAYER_DEACTIVATED` MUST set and render Transitioning
  immediately.
- **FR-005**: `EVENT_PLAYER_ACTIVATED` MUST recompute and render every dependent
  player payload before setting and rendering Active.
- **FR-006**: The activation baseline MUST include weapon, combat, menu,
  resources, movement, cooldowns, quickslot, life, and fishing observations.
- **FR-007**: The companion MUST model Unknown, Transitioning, and Active as one
  typed runtime-only value.
- **FR-008**: Missing, invalid, corrupt, or signal-lost B22 evidence MUST become
  Unknown rather than retaining the previous value.
- **FR-009**: The reader MUST emit only real world-state transitions and MUST log
  them at a diagnostic level that does not flood the default log.
- **FR-010**: Reader routing MUST store world state in the shared game observation
  model and clear it on signal loss or inactive game runtime.
- **FR-011**: System and State MUST display Active, Transitioning, Not detected, or Game
  not active with roles consistent with other state fields.
- **FR-012**: The managed addon manifest MUST advance from version 15 to 16 and
  describe world-state publication.
- **FR-013**: The master specification, README where applicable, and changelog
  MUST document the new block and lifecycle contract.
- **FR-014**: Automated tests MUST cover all wire values, marker and checksum
  rejection, absence, signal loss, duplicate events, addon callback ordering,
  cross-language constants, routing, process exit, and dormant presentation.
- **FR-015**: S049 MUST NOT implement TravelPending, change synthesis gates, or
  absorb roll-dodge, sprint, potion-cooldown, or effect-database scope.

### Key Entities

- **WorldState**: Unknown, Transitioning, or Active.
- **B22 world-state block**: The marker, red payload, and blue complement that
  transport the addon lifecycle.
- **Activation baseline**: The ordered recomputation and rendering of all
  dependent payloads before Active becomes visible.
- **WorldStateView**: The text and semantic status role shown in System and State.

## Success Criteria

- **SC-001**: All three valid B22 values round-trip and every invalid marker,
  checksum, payload, or missing sample resolves to Unknown.
- **SC-002**: The addon and companion contract agrees on exactly 23 payload blocks.
- **SC-003**: Static lifecycle tests prove Transitioning is rendered from
  deactivation and Active occurs only after the complete baseline call.
- **SC-004**: Reader tests emit one event per real transition and clear a prior
  value exactly once on signal loss.
- **SC-005**: View tests cover Active, Transitioning, Not detected, and Game not
  active without changing the Skills section.
- **SC-006**: Existing safety-critical and full CI parity tests remain green.

## Assumptions

- The supported ESO API exposes `EVENT_PLAYER_DEACTIVATED` before loading and
  `EVENT_PLAYER_ACTIVATED` when the character is ready after login, reload, or
  zone transition.
- A completed baseline can contain explicit unavailable payload values; freshness
  means recomputed for the current world, not necessarily detected.
- The negotiated layout has capacity for one additional payload block without a
  protocol-version change.
