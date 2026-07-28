# Phase 1 Data Model: Auto-Potion

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

## New types

### `ResourceWatch` (`src/potion`)

One resource's participation in the rule.

| Field | Type | Default | Meaning |
| --- | --- | --- | --- |
| `enabled` | `bool` | `false` | whether this resource contributes at all |
| `threshold` | `u8` | `35` | fire at or below this percentage |

Defaults to disabled per FR-013. The threshold default is a starting point the
operator is expected to tune; it is only reachable once they enable the resource,
so it never acts on its own.

### `AutoPotionConfig` (`src/potion`)

| Field | Type | Default | Meaning |
| --- | --- | --- | --- |
| `health` | `ResourceWatch` | disabled, 35 | the health watch |
| `magicka` | `ResourceWatch` | disabled, 35 | the magicka watch |
| `stamina` | `ResourceWatch` | disabled, 35 | the stamina watch |
| `quickslot_key` | `Key` | `Key::Q` | the key synthesized to drink |
| `retry_interval_ms` | `u32` | `1500` | minimum time between attempts |

Loads and stores through the same opaque-settings-value pattern as
`FishingConfig`, with the same out-of-range and unparsable-key degradation to
defaults plus a notice (FR-018). A threshold above 100 is out of range; 0 and 100
are both valid (see the spec's edge cases).

### `PotionInputs` (`src/potion`)

Everything the rule reads, gathered explicitly so the rule is a pure function of
its arguments and every blocking condition is visible at the call site.

| Field | Type | Meaning |
| --- | --- | --- |
| `resources` | `ResourceSet` | the three decoded pools |
| `quickslot` | `QuickslotState` | the decoded quickslot |
| `suspended` | `bool` | whether the application is suspended |
| `gated` | `bool` | whether a game menu or text field is up |

### `Block` (`src/potion`)

Why the rule declined, so a decision is explainable rather than a bare `false`.

`Disabled`, `Suspended`, `Gated`, `NoResourceLow`, `NoPotion`, `OnCooldown`,
`RetryTooSoon`.

This is not decoration: SC-002 requires each blocking condition to be tested in
isolation, and asserting *which* condition blocked is what stops a test passing
because a different condition happened to be false. That is the failure CHK012 in
the safety checklist warns about, and a bare boolean cannot distinguish it.

### `AutoPotionController` (`src/potion`)

| Field | Type | Meaning |
| --- | --- | --- |
| `config` | `AutoPotionConfig` | thresholds, key, interval |
| `enabled` | `bool` | the operator's toggle, starts `false` |
| `gated` | `bool` | menu gate, pushed in |
| `suspended` | `bool` | suspend state, pushed in |
| `last_attempt_ms` | `Option<u64>` | when the key was last pressed |

No state enum. Unlike `FishingController` there is no sequence to be partway
through: every evaluation is a pure function of the current readings plus
`last_attempt_ms`. See research.md R3.

### `AutoPotionSink` (`src/potion`)

`fn key(&mut self, key: Key, transition: Transition)`, with
`MockAutoPotionSink` recording ops and `RealAutoPotionSink<B>` driving
`InputBackend::synthesize`. Identical in shape to `FishingSink`, deliberately: it
is the same seam serving the same purpose, and the real implementation is the
*only* place this feature reaches synthesis (FR-008).

## The rule

```
evaluate(inputs, config, enabled, last_attempt_ms, now_ms) -> Result<(), Block>
```

Ordered so the cheapest and most categorical conditions come first, and so the
`Block` returned names the most fundamental reason rather than an incidental one:

1. not `enabled`                      -> `Disabled`
2. `suspended`                        -> `Suspended`
3. `gated`                            -> `Gated`
4. retry interval not elapsed         -> `RetryTooSoon`
5. quickslot has no usable potion     -> `NoPotion`
6. quickslot cooldown is not `Ready`  -> `OnCooldown`
7. no enabled resource is readable and at or below its threshold -> `NoResourceLow`
8. otherwise                          -> fire

Step 7 is the OR (FR-002). A resource contributes only when its watch is enabled
**and** its level is `Percent(p)` with `p <= threshold`; `ResourceLevel::Unknown`
never contributes (FR-004). Steps 5 and 6 read `QuickslotState`, where
`has_potion()` is already `cooldown != Unknown`, so an unreadable quickslot is
categorically not a potion without a second check.

## Extended types

### `Action`

Gains `ToggleAutoPotion`. `ALL` grows to 10; `as_str` yields
`"toggle_auto_potion"`; `default_key` yields `Key::F3`; **both `suspend_exempt`
and `is_app_toggle` gain the variant** (FR-016). The existing unit tests
enumerate the expected sets, so a miss fails loudly.

### `Key`

Gains `F3` in the enum, `as_str` (`"f3"`), `display_name` (`"F3"`), `parse`, and
`ALL`.

### `UiIntent`

Gains `SetAutoPotion(bool)`, following `SetFishing(bool)`, so the hotkey and the
interface control reach one shared state through one path.

### `SettingsForm`

Gains `potion: AutoPotionConfig`, loaded and stored beside `fishing`.

### `AppView`

Gains `auto_potion_active: bool` and the derived label, so the operator can see
whether it is on.

## Lifecycle

```
worker loop tick
  -> reader decodes resources and quickslot into the weave engine
  -> controller.set_suspended(input.is_suspended())
  -> controller.tick(inputs, now, &mut sink)
       -> evaluate(...)  -> Ok  -> sink.key(quickslot_key, Down/Up)
                                -> last_attempt_ms = now
                          -> Err(block) -> nothing
```

`SignalLost` reaches the controller through the existing routing and disables it
(FR-011), matching fishing. The menu gate reaches it through the same
`route_reader_event` branch that already gates the input engine and the fishing
controller (FR-009).
