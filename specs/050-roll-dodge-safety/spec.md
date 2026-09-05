# Feature Specification: Roll-Dodge Safety

**Feature Branch**: `codex/050-roll-dodge-safety`
**Created**: 2026-09-05
**Status**: Implemented
**Input**: GitHub issues #57 and #60, build plan 020

## Scope and clarification decisions

S050 publishes a bounded roll-dodge observation and consumes it only at the
generated weave-skill boundary. It does not gate auto-potion, fishing, or direct
physical input, and it does not add sprint detection.

The current ESO event contract is sufficient without coordinate or resource
heuristics. Ability 28549, filtered to the player target, reports effect gained
at roll entry and effect faded at normal completion. A roll rejected while
sprinting can report gained without faded. The addon therefore treats gained as
Active, faded as Inactive, and expires a missing completion after a conservative
1,500 ms watchdog. Death and player deactivation invalidate the observation;
activation and in-place resurrection establish a fresh Inactive baseline.

All generated weave work fails closed while the value is Active or Unknown. A
physical skill key passes through the established interception path during that
time, and a queued or partly running generated sequence is discarded rather than
replayed after recovery.

## User Scenarios and Testing

### User Story 1 - Observe roll dodge truthfully (Priority: P1)

As a player, I want ESO Weave to distinguish an active roll dodge from a known
inactive state and unavailable evidence so dependent safety rules are grounded
in the game event rather than movement guesses.

**Independent Test**: Exercise the addon state machine with gained, faded,
duplicate, missing-completion, death, resurrection, deactivation, and activation events, then
round-trip all values through B23 and the companion UI.

**Acceptance Scenarios**:

1. **Given** the player begins a valid roll dodge, **When** effect gained for
   ability 28549 arrives, **Then** B23 immediately publishes Active.
2. **Given** the roll completes normally, **When** effect faded arrives, **Then**
   B23 publishes Inactive and cancels the watchdog.
3. **Given** sprint rejection or another anomaly produces gained without faded,
   **When** 1,500 ms elapses, **Then** B23 recovers to Inactive.
4. **Given** death, zoning, invalid capture, or signal loss, **When** the state is
   rendered, **Then** it is Unknown or Not detected rather than stale.

---

### User Story 2 - Suppress generated weaves during roll dodge (Priority: P1)

As a player, I want generated weave actions stopped during a roll dodge so the
application neither emits blocked skills nor queues a burst for recovery.

**Independent Test**: Drive the input hook, worker, and real-sink cancellation
seams through Active, Unknown, and Inactive transitions and inspect emitted input.

**Acceptance Scenarios**:

1. **Given** roll state is Active or Unknown, **When** a bound skill key is
   pressed, **Then** the physical event passes through and no weave is handed off.
2. **Given** a sequence is waiting, **When** roll becomes Active, **Then** no new
   generated down event is emitted and any already-held generated input is released.
3. **Given** a key is pressed during roll dodge, **When** state later becomes
   Inactive, **Then** that old request is not replayed.
4. **Given** state is Inactive and all existing gates allow work, **When** a new
   eligible skill key is pressed, **Then** its normal weave sequence executes.

## Edge Cases

- Duplicate gained is idempotent and refreshes the same fixed recovery deadline.
- Duplicate faded is idempotent.
- A faded event with no preceding gained establishes Inactive safely.
- Death during a dodge publishes Unknown and cancels the watchdog.
- In-place resurrection publishes Inactive even when no player activation follows.
- Player deactivation publishes Unknown before loading; activation publishes
  Inactive only after the complete player baseline.
- Roll events delivered after death or deactivation are ignored until a fresh
  activation or in-place resurrection baseline.
- A configured fast interval longer than the bounded Active window is capped while
  interception is active so the companion observes Active before watchdog recovery.
- An older addon advertises protocol version 2 and only B0 through B22. The
  companion reads its earlier payloads but never samples an ordinary screen pixel
  as B23, so roll state remains Unknown and generated weaves remain fail closed.
- Inactive ESO runtime overrides the row with Game not active.
- Existing suspension, focus, menu, life-state, and cooldown behavior retains
  precedence and does not become less restrictive.

## Requirements

### Functional Requirements

- **FR-001**: PixelBeacon MUST publish B23 with distinct Unknown, Inactive, and
  Active payloads, a unique green marker, and the complement checksum.
- **FR-002**: Addon and companion constants MUST agree on layout protocol version,
  payload count, index, marker, state payloads, and checksum behavior.
- **FR-003**: The addon MUST filter `EVENT_COMBAT_EVENT` to target
  `COMBAT_UNIT_TYPE_PLAYER` and ability ID 28549.
- **FR-004**: `ACTION_RESULT_EFFECT_GAINED` MUST enter Active immediately and
  `ACTION_RESULT_EFFECT_FADED` MUST enter Inactive immediately.
- **FR-005**: Active MUST recover to Inactive after 1,500 ms when no completion
  arrives, covering the observed sprint-rejection missing-end sequence.
- **FR-006**: Death and player deactivation MUST cancel the watchdog and publish
  Unknown; completed activation and in-place resurrection MUST publish Inactive.
- **FR-007**: The companion MUST model Unknown, Inactive, and Active as one typed,
  runtime-only value and MUST fail invalid, absent, corrupt, or lost evidence to Unknown.
- **FR-008**: The negotiated header MUST advance to protocol version 3 with 24
  payload blocks while preserving version 1 at 22 and version 2 at 23.
- **FR-009**: Protocol versions 1 and 2 MUST retain earlier payload decoding and
  MUST NOT sample B23.
- **FR-010**: The reader MUST emit only real state transitions and clear a prior
  state exactly once on signal loss.
- **FR-011**: System presentation MUST name Active, Inactive, Not detected, and
  Game not active truthfully and explain that Active or unavailable evidence
  blocks generated weaves.
- **FR-012**: Active and Unknown MUST make the input classifier pass bound
  physical skill events through without handing generated weave work to the worker.
- **FR-013**: Active and Unknown MUST make the weave worker drop already-handed-off
  actions without advancing its cooldown.
- **FR-014**: A transition to Active or Unknown during a generated sequence MUST
  cancel future down events while permitting releases for generated inputs already held.
- **FR-015**: Requests received while gated MUST NOT replay after Inactive returns.
- **FR-016**: Application toggle hotkeys MUST remain exempt, and existing game,
  focus, suspension, menu, life, and action-activity gates MUST remain fail closed.
- **FR-017**: The managed addon manifest MUST advance from version 16 to 17 and
  describe roll-dodge publication.
- **FR-018**: Automated tests MUST cover normal completion, duplicate events,
  missing completion, watchdog recovery, death, resurrection, zoning, signal loss, protocol
  compatibility, hook pass-through, worker drops, mid-sequence cancellation,
  non-replay, recovery, routing, process exit, and dormant presentation.
- **FR-019**: S050 MUST NOT add sprint detection, auto-potion gating, effect
  databases, configurable remapping, or coordinate/resource inference.
- **FR-020**: Lifecycle-invalidated combat events MUST NOT reopen the gate, and
  every supported interception polling cadence MUST sample multiple times inside
  the 1,500 ms Active watchdog window.

### Key Entities

- **RollDodgeState**: Unknown, Inactive, or Active.
- **B23 roll-dodge block**: Marker, red state payload, and blue complement.
- **Roll watchdog**: A fixed 1,500 ms recovery deadline armed by effect gained.
- **Roll gate**: A hook and weave-worker gate that blocks only generated weave work.
- **RollDodgeView**: Truthful text and semantic role in the Live HUD.

## Success Criteria

- **SC-001**: All three B23 values round-trip; every invalid marker, checksum,
  payload, missing block, legacy protocol, and lost signal resolves to Unknown.
- **SC-002**: Addon and companion agree on protocol version 3 and 24 payload blocks.
- **SC-003**: Static and state-machine tests prove gained/faded handling, a bounded
  1,500 ms missing-end recovery, lifecycle invalidation, and activation rebaseline.
- **SC-004**: Active and Unknown produce no generated weave down events, including
  when they arrive during a wait, while physical keys and toggle hotkeys behave as specified.
- **SC-005**: No request made during a gated interval executes after recovery.
- **SC-006**: Live HUD and dormant UI tests cover every state and identify the
  roll-dodge weave blocker.
- **SC-007**: Existing safety-critical and full CI parity tests remain green.
- **SC-008**: Tests prove late roll events remain Unknown after lifecycle
  invalidation and a maximum configured fast interval is safety-capped to 375 ms.

## Assumptions

- Current live ESO API version 101050 retains `EVENT_COMBAT_EVENT`, target combat
  unit and ability filters, player lifecycle events, and `GetGameTimeMilliseconds`.
- Current addon evidence for ability 28549 establishes effect gained/faded as the
  normal sequence and gained-only as the sprint-rejection defect.
- 1,500 ms is deliberately longer than the ordinary roll window observed by the
  event pair; the completion event remains authoritative when it arrives earlier.
