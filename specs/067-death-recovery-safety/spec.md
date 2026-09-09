# Feature Specification: Death Recovery Safety

**Feature Branch**: `codex/s067-death-recovery-safety`

**Created**: 2026-09-09

**Status**: Implemented, pending external review

**Input**: Issue #109. Keep every automation blocked through a complete death
episode and reset auto-potion retry eligibility before recovery can authorize
new synthesized input.

## Clarifications

All routine decisions were resolved under the build-phase autopilot policy.

- The existing B21 payload remains the life authority. Its former
  `Reincarnating` state becomes `Recovering`, with ghost, world-activation, and
  no-load diagnostic paths.
- `EVENT_PLAYER_ALIVE` starts recovery but never authorizes input by itself.
- Recovery requires authoritative event evidence where available and a later
  coherent addon baseline. A missed ghost event may converge from repeated
  unit-query evidence, while a missing world activation stays fail closed.
- A recovery capture refreshes action-driving observations before Alive opens
  the shared gate.
- Auto-potion starts a new retry episode at recovery. A full configured retry
  interval must elapse after the first coherent Alive capture.
- Issue #110 owns installed-release field verification and is not part of S067.

## User Scenarios and Testing

### User Story 1 - Remain blocked through the complete death episode (Priority: P1)

As a player, I want death to close every automation boundary immediately and
keep it closed through ghost, loading, or instant recovery so no generated key
can fire during an unsafe transition.

**Independent Test**: Drive dead, Alive-event, reincarnation, world activation,
no-load, duplicated, reordered, and missing-event sequences through the addon
contract and reader, then prove only a later coherent Alive baseline opens the
gate.

**Acceptance Scenarios**:

1. **Given** automation is actionable, **When** death is observed, **Then** B21
   publishes Dead and the shared authorization epoch advances before any
   controller lock is acquired.
2. **Given** an Alive event after death, **When** no later coherent baseline has
   completed, **Then** B21 remains Recovering and every synthesis path is closed.
3. **Given** a ghost, load-based, or no-load path satisfies its completion
   evidence, **When** a later baseline confirms the player is neither dead nor
   reincarnating, **Then** B21 may publish Alive.
4. **Given** required completion evidence is missing or contradictory, **When**
   polling continues, **Then** recovery remains fail closed or follows only the
   documented bounded polling fallback.

### User Story 2 - Discard stale work and require fresh recovery inputs (Priority: P1)

As a player, I want every pre-death request, timer, and observation invalidated
so returning to life cannot replay a weave, fishing action, or potion attempt.

**Independent Test**: Arrange queued weave work, each fishing deadline, and an
auto-potion retry before death. Complete recovery and assert zero stale output,
fresh observation ordering, and a full new potion retry interval.

**Acceptance Scenarios**:

1. **Given** a weave request was admitted before death, **When** recovery later
   opens, **Then** its authorization epoch is stale and it emits no new Down.
2. **Given** fishing owns a reel, recast, or arming deadline, **When** life enters
   Dead or Recovering, **Then** the deadline is cancelled and is not replayed.
3. **Given** auto-potion previously attempted or became eligible, **When** Alive
   returns with health still low, **Then** the first recovered tick emits no Q
   and starts a complete new retry interval.
4. **Given** a recovery capture, **When** Alive is routed, **Then** current world,
   menu, travel, movement, resources, cooldowns, and quickslot observations from
   that capture are already available to controllers.

### User Story 3 - Diagnose recovery without personal data (Priority: P2)

As an operator, I want the HUD and logs to identify recovery state, path, epoch,
and sample generation so a field trace can explain why automation remained
blocked or reopened.

**Independent Test**: Decode every recovery wire value, render its accessible
status text, and capture transition logs containing only state-machine metadata.

**Acceptance Scenarios**:

1. **Given** a recognized recovery path, **When** B21 is decoded, **Then** the HUD
   distinguishes Recovering from Dead and includes the path where known.
2. **Given** a life gate transition, **When** diagnostics are emitted, **Then**
   state, path, death epoch, gate state, and reader generation are present without
   account, character, or input contents.

## Edge Cases

- ESO emits Alive before Reincarnated, emits duplicate callbacks, or repeats a
  callback after the state already converged.
- The addon observes death or ghost state by polling after the corresponding
  event was missed.
- Deactivation overlaps death and activation arrives before or after Alive.
- Signal loss, focus loss, suspension, a menu, travel, or roll dodge overlaps
  recovery.
- A recovery frame contains unchanged resource and quickslot values that the
  reader would normally suppress as duplicate events.
- Death occurs after physical admission but before worker execution or during a
  generated hold.
- Auto-potion has never attempted, attempted recently, or has an expired retry
  interval when death begins.
- The installed addon is older and still emits the legacy 0xE0 reincarnating
  payload. The new companion interprets it as fail-closed ghost recovery.

## Requirements

### Functional Requirements

- **FR-001**: The life model MUST be `Unknown`, `Dead`, `Recovering(path)`, or
  `Alive`, and only Alive may authorize synthesized input.
- **FR-002**: PixelBeacon MUST enter Dead synchronously on `EVENT_PLAYER_DEAD`
  and MUST register `EVENT_PLAYER_REINCARNATED`.
- **FR-003**: `EVENT_PLAYER_ALIVE` MUST enter or continue Recovering and MUST NOT
  directly publish Alive.
- **FR-004**: Ghost recovery MUST require reincarnation completion evidence and
  a later coherent baseline; repeated unit-query evidence MAY provide a bounded
  fallback when the callback is missed.
- **FR-005**: A death episode that deactivated the player MUST require
  `EVENT_PLAYER_ACTIVATED`, a complete player rebaseline, and a later coherent
  baseline before Alive.
- **FR-006**: A no-load path MUST require Alive evidence plus at least one later
  coherent generation before Alive.
- **FR-007**: Polling MUST converge missed death and ghost observations without
  allowing one contradictory sample to open recovery.
- **FR-008**: Death, Recovering, Unknown, signal loss, and inactive runtime MUST
  close the shared life gate before controller locks and advance the monotonic
  authorization epoch on the safe-to-unsafe transition.
- **FR-009**: A recovered Alive event MUST be routed after the same capture has
  refreshed every cached observation used by weave, fishing, and auto-potion.
- **FR-010**: Pre-death queued weave work and running sequences MUST be rejected
  by epoch and gate checks, while matching releases for held generated input
  remain permitted.
- **FR-011**: Fishing MUST cancel every autonomous deadline on Dead, Recovering,
  or Unknown and MUST require a fresh eligible observation after recovery.
- **FR-012**: Auto-potion MUST retain the requested toggle but start a new retry
  episode after recovery; the first recovered tick cannot fire and a full retry
  interval must elapse before an attempt.
- **FR-013**: The first post-recovery potion attempt MUST use resource, quickslot,
  cooldown, movement, world, travel, menu, and life evidence from the recovery
  capture or a later capture.
- **FR-014**: B21 MUST retain its marker, checksum, block position, Alive value,
  Dead value, and legacy 0xE0 fail-closed meaning. New recovery-path values MUST
  remain unambiguous under the configured tolerance.
- **FR-015**: PixelBeacon's managed manifest version MUST advance without adding
  SavedVariables, a settings panel, or another payload block.
- **FR-016**: HUD and diagnostics MUST distinguish Dead and Recovering, include a
  known recovery path, and log reader generation plus death authorization epoch
  without personal data.
- **FR-017**: Tests MUST cover ghost, world-activation, no-load, polling fallback,
  duplicate, reordered, missing-event, signal-loss, queued-weave, running-weave,
  fishing-deadline, and auto-potion-retry scenarios.
- **FR-018**: Canonical safety, game-observation, state-machine, status, protocol,
  test-strategy, and changelog documentation MUST describe the new contract.
- **FR-019**: S067 MUST NOT publish v0.15.1, close #110, or perform live ESO field
  verification.

### Key Entities

- **Death episode**: One monotonic unsafe interval beginning at a dead observation
  and ending only after path-specific completion plus a coherent baseline.
- **LifeState**: The companion state carrying Unknown, Dead, Recovering(path), or
  Alive.
- **RecoveryPath**: Ghost, WorldActivation, or NoLoad diagnostic evidence.
- **Reader generation**: The monotonic capture count used to prove transition and
  recovery-observation ordering.
- **Death authorization epoch**: The monotonic companion counter advanced when
  the shared life gate closes from open.
- **Potion retry episode**: The interval beginning at coherent recovery during
  which no automatic quickslot attempt is eligible.

## Success Criteria

- **SC-001**: Every non-Alive state produces zero new generated Down operations
  across weave, fishing, and auto-potion tests.
- **SC-002**: All three recovery families remain blocked until their documented
  evidence and at least one later coherent baseline are present.
- **SC-003**: Every pre-death queued or timed operation emits zero stale output
  after recovery.
- **SC-004**: The first recovered auto-potion tick emits zero operations and the
  earliest possible later attempt is exactly one configured retry interval after
  the coherent recovery tick.
- **SC-005**: Recovery batches route refreshed action-driving observations before
  Alive in deterministic reader tests.
- **SC-006**: Existing input recursion, focus scoping, non-blocking hook,
  PixelBeacon ownership, AddOns containment, and signal-loss suites remain green.
- **SC-007**: Full formatting, Clippy, locked tests, documentation policy, link,
  UTF-8, punctuation, and mojibake gates pass.

## Assumptions

- ESO addon API 101050 and 101054 expose the lifecycle callbacks named by #109.
- B21 has sufficient unused payload values for three recovery paths while
  retaining its existing marker and checksum.
- One later addon baseline is a safety debounce backed by current queries, not a
  time-only authorization rule.
- Field behavior that cannot be proved without ESO remains in #110.
