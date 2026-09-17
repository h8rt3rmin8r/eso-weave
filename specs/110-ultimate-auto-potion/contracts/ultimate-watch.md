# Contract: Ultimate Auto Potion Watch

## Inputs

- Operator configuration supplies an independent enabled flag and threshold from 0 through 100.
- The existing atomic Ultimate observation supplies current and maximum.
- Existing controller state supplies game, focus, heartbeat, suspension, context, life, world, travel, movement, quickslot, cooldown, retry, and native-binding gates.

## Evaluation

1. Evaluate all existing non-resource gates in their current order.
2. Evaluate Health, Magicka, Stamina, and Ultimate as one deterministic OR list.
3. Ultimate is fresh only when current is known and maximum is known and positive.
4. Ultimate is low exactly when `current * 100 <= threshold * maximum` in widened integer arithmetic.
5. The first low watch produces one typed trigger cause. Ultimate causes use the ceiling integer percentage.
6. Continue through the existing quickslot, cooldown, retry, and binding gates before one existing Quickslot action may be submitted.

## Persistence

- Missing Ultimate watch means disabled at 35 percent.
- Invalid threshold uses the shared default and emits the existing invalid-value notice with the Ultimate field name.
- Current saves include the Ultimate watch explicitly.

## Outputs

- Structured state uses `triggered_ultimate` for an Ultimate-authorized attempt.
- User-visible state uses `Triggered: Ultimate at N% (threshold T%)`.
- No raw input content, potion inference, or additional key action is produced.

## Invariants

- Unknown, zero-maximum, stale, dormant, or non-active Ultimate evidence never authorizes input.
- Existing watch meanings and first-cause priority remain unchanged.
- Ultimate cost, active weapon bar, and Ready presentation do not influence Auto Potion.
- Synthesis remains exclusively on the established native Quickslot path.
