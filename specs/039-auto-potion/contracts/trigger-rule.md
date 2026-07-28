# Contract: The Auto-Potion Trigger Rule

**Feature**: [../spec.md](../spec.md) | **Date**: 2026-07-27

This is the complete decision procedure. It is stated here in one place because
every clause is a safety condition, and because `tests/potion.rs` is written
directly against this table rather than against the implementation.

## Signature

```
evaluate(
    inputs: PotionInputs,      // resources, quickslot, suspended, gated
    config: &AutoPotionConfig, // watches, key, retry interval
    enabled: bool,
    last_attempt_ms: Option<u64>,
    now_ms: u64,
) -> Result<(), Block>
```

`Ok(())` means fire. `Err(block)` names the first condition that declined, in the
order below.

## The ordered conditions

| # | Condition to proceed | Block when it fails |
| --- | --- | --- |
| 1 | `enabled` | `Disabled` |
| 2 | `!inputs.suspended` | `Suspended` |
| 3 | `!inputs.gated` | `Gated` |
| 4 | `last_attempt_ms` is `None`, or `now_ms - last >= retry_interval_ms` | `RetryTooSoon` |
| 5 | `inputs.quickslot.has_potion()` | `NoPotion` |
| 6 | `inputs.quickslot.cooldown == SlotCooldown::Ready` | `OnCooldown` |
| 7 | any enabled watch is satisfied (see below) | `NoResourceLow` |

All seven must hold. Order matters only for which reason is reported; the outcome
is the conjunction either way.

### Condition 7, the disjunction

A watch is satisfied when **all** of:

- `watch.enabled`
- the corresponding level is `ResourceLevel::Percent(p)` (never `Unknown`)
- `p <= watch.threshold`

Condition 7 holds when **any** of the three watches is satisfied. This is the OR
required by FR-002 and is not configurable.

### Why 5 and 6 are both needed

`has_potion()` is defined as `cooldown != SlotCooldown::Unknown`, so condition 5
rejects an unreadable quickslot and condition 6 rejects a readable one that is
counting down. Together they mean "there is a potion and it can be drunk now".
Neither implies the other: `Unknown` fails 5, and `RemainingMs(n)` passes 5 and
fails 6.

## On firing

1. Emit `key(quickslot_key, Down)` then `key(quickslot_key, Up)` through the sink.
   Exactly one press and one release (FR-007).
2. Set `last_attempt_ms = Some(now_ms)`.

`last_attempt_ms` is set on the **attempt**, not on a confirmed drink, because the
game's confirmation is the quickslot cooldown and that is precisely the reading
that lags.

## How the seven conditions map to the spec's eight

SC-002 lists eight blocking conditions and this contract defines seven `Block`
variants. They agree; the mapping is stated here so no condition looks untested.

| SC-002 condition | Where it is enforced |
| --- | --- |
| feature disabled | condition 1, `Disabled` |
| suspended | condition 2, `Suspended` |
| gated | condition 3, `Gated` |
| no potion | condition 5, `NoPotion` |
| cooldown not zero | condition 6, `OnCooldown` |
| resource unreadable | condition 7, folded into `NoResourceLow` |
| resource disabled | condition 7, folded into `NoResourceLow` |
| **signal lost** | **not an `evaluate` condition** |

Two notes on the rows that do not map one to one.

**Unreadable and disabled resources both yield `NoResourceLow`** because from the
rule's point of view they are the same outcome: the resource did not satisfy its
watch. They are distinguished by the tests that construct them (unknown level
versus disabled watch), not by the reason reported.

**Signal loss is deliberately not a condition here.** It is handled one level up:
the routing layer disables the controller on `SignalLost`, exactly as it does for
fishing, so the rule then declines with `Disabled`. Putting it in `evaluate`
would mean the controller stayed enabled while a stale reading sat in front of
it, which is the opposite of what FR-011 asks for. It is tested at the routing
layer instead.

## Truth-table obligations

`tests/potion.rs` must cover, at minimum:

- **Each of the seven conditions failing in isolation**, with all six others
  satisfied, asserting both that nothing is emitted **and** that the reported
  `Block` is the expected one. Asserting only the absence of a keypress would let
  a test pass because a different condition was accidentally false (safety
  checklist CHK012).
- **The OR**, three times: exactly one resource enabled and low, the other two
  enabled and high, for each resource in turn.
- **Unknown never low**, for every threshold from 0 to 100 inclusive, for each
  resource.
- **The boundary**, at `p == threshold` (fires), `p == threshold + 1` (does not),
  and `p == threshold - 1` (fires).
- **Thresholds 0 and 100**, both valid.
- **Every watch disabled**, with all three resources at zero: never fires.
- **The retry interval** against a virtual clock: fires, then does not fire at
  `last + interval - 1`, then fires at `last + interval`.
- **One press per trigger**: exactly two sink operations (down, up).

## What this contract does not cover

- Which potion is slotted, or what it restores. The operator's per-resource
  enables are the substitute; the restore types are not machine-readable.
- Any interaction with fishing. Both synthesize through the same input engine and
  neither is aware of the other; each keypress is independently gated.
- Restoring the enable across sessions. It always starts off (plan.md D5).
