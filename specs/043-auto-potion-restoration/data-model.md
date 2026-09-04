# Data Model: Auto-potion Restoration

**Feature**: [spec.md](spec.md) | **Date**: 2026-09-03

## AutoPotionController

| Field | Meaning | Rule |
| --- | --- | --- |
| config | Thresholds, watches, binding, retry duration | Existing persisted configuration |
| requested | Session-only user intent | Defaults false; lifecycle loss never clears it |
| game_active | S041 game lifecycle fact | False blocks input |
| focused | S041 focus fact | False blocks input |
| beacon_available | PixelBus heartbeat freshness fact | False blocks input |
| suspended | Application input suspension | True blocks input |
| gated | Disallowed game context | True blocks input |
| last_attempt_ms | Most recent submitted attempt | Enforces retry interval |
| state | Last effective auto-potion state | Sole source for UI and change-only diagnostics |

## AutoPotionState

- `Off`
- `Dormant(GameInactive)`
- `Dormant(Unfocused)`
- `Blocked(BeaconUnavailable)`
- `Blocked(Suspended)`
- `Blocked(GameContext)`
- `Blocked(NoWatchedResource)`
- `Blocked(ResourcesUnavailable)`
- `Blocked(QuickslotUnavailable)`
- `Blocked(NoPotion)`
- `Blocked(PotionUnavailable)`
- `Blocked(PotionCooldown)`
- `Blocked(RetryInterval)`
- `Ready`
- `Triggered(TriggerCause)`

## TriggerCause

| Field | Type | Rule |
| --- | --- | --- |
| resource | Health, Magicka, or Stamina | First low watched resource in deterministic order |
| observed_percent | `u8` | Fresh normalized observation that is at or below threshold |
| threshold_percent | `u8` | Configured threshold for the selected resource |

## State transitions

```text
requested Off -> Off
requested On + lifecycle unavailable -> Dormant or Blocked
requested On + safe runtime + observations above threshold -> Ready
requested On + safe runtime + low resource -> Triggered
Triggered + next evaluation within retry interval -> Blocked(RetryInterval)
Blocked + recovered preconditions -> Ready or Triggered
any state + requested Off -> Off
```

## Invariants

1. Requested enablement and effective state are separate values.
2. Every non-Triggered state submits zero input.
3. Only explicit Potion(Usable) with cooldown Ready can reach Triggered.
4. Unknown, stale, empty, ambiguous, depleted, blocked, and non-potion states cannot reach Triggered.
5. Triggered lasts no longer than the next controller evaluation.
6. A submitted attempt is exactly one Down followed by one Up.
7. Signal loss changes beacon availability and effective state, never requested enablement.
8. The UI reads the controller state instead of duplicating the evaluation rule.
