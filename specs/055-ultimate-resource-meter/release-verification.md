Validates implementation: #71

Blocked by: #71 and a published release containing S055

## Outcome

Verify exact Ultimate telemetry, bar-aware threshold selection, and stable meter
presentation against a fresh release. This issue is field verification only;
repository implementation belongs to #71.

## Verification matrix

Run and record:

- zero, partial, full, spent, and regenerated Ultimate against the game's displayed value
- unequal front and back Ultimate costs with swaps in both directions
- no Ultimate slotted on each bar independently
- an inactive-bar slot change followed by a swap
- cost-changing buffs, passives, equipment, and applicable transformations
- below, exactly at, and above each cast cost
- death, resurrection, loading, travel, signal loss, and recovery
- special or temporary hotbars, which must hide threshold and Ready rather than use stale data
- dark and light themes at narrow and wide dashboard layouts
- pointer, keyboard, and screen-reader access to exact charge, maximum, cost, bar, and readiness

## Acceptance criteria

- [ ] The exact `current/maximum` readout matches ESO throughout gain and spend.
- [ ] The active threshold matches the effective slotted cost on both bars and updates on the first frame after a swap.
- [ ] Green Ready appears exactly at `current >= cost`, stays right of the numbers, and causes no layout movement.
- [ ] Empty, special, unavailable, and lost states hide threshold and Ready without stale values.
- [ ] Quarter landmarks remain subtle and the cost threshold remains distinct when they overlap.
- [ ] Ultimate never changes auto-potion, weaving, fishing, timing, or safety behavior.
- [ ] The issue records release version, tester, result, screenshots, and bounded diagnostics.

## Boundaries

Do not change implementation while collecting the receipt. Any discovered defect
gets a separate bug issue linked here.
