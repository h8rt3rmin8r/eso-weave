# Feature Specification: Travel Safety

**Slice**: S051  
**Issue**: #59

## User Scenarios

### User Story 1: Preserve travel attempts

As a player, I want ESO Weave to stop all synthesized input while a travel attempt is pending so automation cannot cancel recall, wayshrine, group, guild, house, door, or other jump travel.

### User Story 2: Recover safely

As a player, I want automation to recover after cancellation, failure, timeout, or completed loading without replaying work that was suppressed during travel.

### User Story 3: See the blocker

As an operator, I want the application to display the observed travel state and distinguish travel suppression from world-transition suppression.

## Requirements

- **FR-001**: B24 MUST publish `Unknown`, `Inactive`, or `Pending` travel state.
- **FR-002**: A material upward recall-cooldown edge MUST begin recall-sourced pending travel. Initial and decreasing samples MUST NOT begin travel.
- **FR-003**: `EVENT_PREPARE_FOR_JUMP` MUST begin jump-sourced pending travel and `EVENT_JUMP_FAILED` MUST clear it when lifecycle data remains valid.
- **FR-004**: Recall-sourced pending travel MUST clear when movement resumes after a 250 ms grace period. Every pending state MUST have a 15 second watchdog.
- **FR-005**: Death and player deactivation MUST invalidate travel state. Player activation MUST rebaseline recall cooldown before publishing `Inactive`.
- **FR-006**: Protocol v4 MUST contain 25 blocks while v1, v2, and v3 retain 22, 23, and 24 blocks respectively.
- **FR-007**: Corrupt, legacy, or lost telemetry MUST produce `Unknown` travel state.
- **FR-008**: Every synthesized-input path MUST remain closed unless world state is `Active` and travel state is `Inactive`.
- **FR-009**: Physical input and user toggles MUST pass through unchanged.
- **FR-010**: Suppressed automation MUST NOT be replayed, and injected keys MUST be released when the gate closes.
- **FR-011**: Recovery MUST update controller and worker state before reopening the input-hook gate.
- **FR-012**: The System and State panel MUST show Travel and diagnostics MUST identify travel pending separately from world transition.
- **FR-013**: The addon contract version MUST advance to 18.
- **FR-014**: Deterministic tests MUST cover detector edges, protocol compatibility, corruption, signal loss, lifecycle transitions, every synthesis path, physical passthrough, no replay, and recovery ordering.
- **FR-015**: This slice MUST NOT initiate travel, broaden protected hooks, alter sprint detection, or redesign the map UI.

## Assumptions

- A recall cooldown increase of at least 500 ms is a stable local indicator that recall was initiated.
- A 250 ms cancellation grace avoids interpreting residual movement at initiation as cancellation.
- A 15 second watchdog bounds every attempt even when ESO emits neither completion nor failure.
- Live ESO verification remains necessary for all travel modes because deterministic tests cannot reproduce the game client.

## Success Criteria

- All generated weave, fishing, and auto-potion input stops during pending travel or an unsafe world state.
- Automation resumes only from current state after a safe observation, with no queued replay.
- Existing protocol fixtures continue to decode at their original lengths.
- The UI truthfully reports `Unknown`, `Inactive`, or `Pending`.
