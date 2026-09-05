# Research

## Decision

Use a bounded composite detector:

1. Observe `GetRecallCooldown()` on the addon's existing 100 ms fast tick.
2. Treat an upward change of at least 500 ms as recall initiation.
3. Reinforce all jump-style travel with `EVENT_PREPARE_FOR_JUMP`.
4. Clear explicit failures through `EVENT_JUMP_FAILED`, recall cancellation through resumed movement after a short grace period, and all stale attempts through a watchdog.
5. Invalidate state across death and loading lifecycle boundaries.

## Evidence

- The current ESO API documents `GetRecallCooldown()` with remaining and duration values.
- The current ESO API documents `EVENT_PREPARE_FOR_JUMP` and `EVENT_JUMP_FAILED`.
- ESO's shared loading-screen implementation shows the screen on prepare and hides it on failure, confirming those events bracket the jump handoff.
- Historical techniques based on `EVENT_ABILITY_COOLDOWN_UPDATED` were rejected because that event is not present in the current API contract.

## Alternatives Rejected

- Hooking every travel UI action would be incomplete and would enlarge the protected-hook surface.
- Treating player deactivation as the first signal is too late to prevent automation from cancelling the cast interval.
- Inferring travel from stamina, speed, or arbitrary effects is unrelated and fragile.
- Replaying blocked automation after travel would create stale actions and violate existing safety behavior.

## Validation Boundary

Unit and integration tests can prove state transitions and suppression. Release validation must still exercise paid recall, free wayshrine travel, group and guild jumps, house travel, doors, cancellation, failure, and loading completion in the live game.
