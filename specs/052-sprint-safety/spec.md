# Feature Specification: Sprint Safety

**Slice**: S052
**Issues**: #17, #61

## User Scenarios

### User Story 1: Observe bounded sprint state

As a keyboard-mode player, I want ESO Weave to distinguish ordinary on-foot movement from sprinting without pretending that unsupported evidence is authoritative.

### User Story 2: Avoid rejected potion attempts

As a player, I want auto-potion to wait while an explicit sprint is active and re-evaluate immediately when sprint ends so ESO does not reject the attempt or spam chat.

### User Story 3: Diagnose inference quality

As a tester, I want transition-level diagnostics and a live validation matrix so false classifications can be investigated without continuously logging coordinates or input contents.

## Requirements

- **FR-001**: Protocol v4 MUST retain 25 blocks and reuse the reserved B9 on-foot sprint code `0xA0` without changing geometry or legacy layouts.
- **FR-002**: B9 MUST decode `0x20` as `OnFoot`, `0x60` as `Mounted`, and `0xA0` as `Sprinting`.
- **FR-003**: The reserved mounted sprint code `0xE0` MUST remain unsupported and decode as `Unknown`.
- **FR-004**: The addon MUST infer on-foot sprint only in keyboard mode while the player is moving, is not mounted, swimming, falling, dead, reincarnating, or roll dodging, and every ordinary action slot reports a non-cost state failure.
- **FR-005**: Sprint entry MUST require 200 ms of continuous candidate evidence. Ambiguous loss of action-slot evidence MUST require 200 ms before exit, while stopping movement or entering an exclusion MUST clear sprint immediately.
- **FR-006**: Explicit sprint state MUST expire after 1,500 ms without qualifying evidence.
- **FR-007**: `EVENT_ACTION_SLOT_STATE_UPDATED` and the existing 100 ms active-world backstop MUST drive reevaluation without adding a new polling thread.
- **FR-008**: Gamepad mode and contradictory, missing, stale, or malformed evidence MUST NOT publish an explicit sprint state.
- **FR-009**: Death, deactivation, loading, signal loss, and unsupported layout MUST clear sprint evidence.
- **FR-010**: The application MUST present the explicit `Sprinting` movement value and retain truthful `Unknown` handling.
- **FR-011**: Auto-potion MUST report `Blocked: sprinting` and emit no quickslot input while movement is explicitly `Sprinting`.
- **FR-012**: A low-resource condition that remains valid when sprint ends MUST pass through the normal current-state eligibility rule without requiring a second threshold crossing.
- **FR-013**: Resource recovery, quickslot unavailability, cooldown, focus loss, game deactivation, suspension, life, world, travel, and game-context gates MUST remain authoritative and prevent a stale post-sprint attempt.
- **FR-014**: Unknown movement MUST NOT block auto-potion indefinitely. Only explicit, watchdog-bounded `Sprinting` blocks this consumer.
- **FR-015**: The addon contract version MUST advance to 19 while protocol version remains 4.
- **FR-016**: Tests MUST cover every wire value, corrupt evidence, detector debounce, hysteresis, watchdog, exclusions, lifecycle reset, signal loss, potion suppression, recovery, blocker precedence, and no repeated synthesis.
- **FR-017**: This slice MUST NOT transmit raw coordinates, infer sprint from stamina alone, claim gamepad support, publish mounted gallop, change sprint bindings, or create a comprehensive effects database.

## Assumptions

- `ActionSlotHasNonCostStateFailure` across ordinary active-bar slots is the best currently supported keyboard-mode inference, but it is not an authoritative game API.
- A 200 ms debounce filters transient slot refreshes while remaining short relative to player perception and the potion retry interval.
- The 1,500 ms watchdog bounds stale positive evidence and matches the existing short safety-watchdog scale.
- Live ESO validation remains required because deterministic tests cannot reproduce protected engine state.

## Success Criteria

- Ordinary running remains `OnFoot`, qualifying keyboard sprint becomes `Sprinting`, and unsupported modes never fabricate sprint.
- Auto-potion emits no quickslot transition during explicit sprint and fires through its ordinary rule after sprint ends only when all other current conditions remain eligible.
- Existing B9 values, all protocol layouts, and every unrelated automation gate remain compatible.
- Transition diagnostics support the live matrix without continuous raw-coordinate or key logging.
