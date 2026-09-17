# Data Model: Ultimate Auto Potion Resource Watch

**Date**: 2026-09-17

## AutoPotionConfig

```text
AutoPotionConfig
|-- health: ResourceWatch
|-- magicka: ResourceWatch
|-- stamina: ResourceWatch
|-- ultimate: ResourceWatch
`-- retry_interval_ms: u32
```

Every `ResourceWatch` has `enabled: bool` and `threshold: u8`. Ultimate uses the shared default `{ enabled: false, threshold: 35 }` and the shared accepted threshold range 0 through 100.

## PotionReadings

```text
PotionReadings
|-- resources: ResourceSet
|   |-- health: ResourceLevel
|   |-- magicka: ResourceLevel
|   `-- stamina: ResourceLevel
|-- ultimate: UltimateTelemetry
|   |-- current: UltimateValue
|   |-- maximum: UltimateValue
|   |-- front_cost: UltimateValue
|   `-- back_cost: UltimateValue
`-- quickslot: QuickslotState
```

Auto Potion reads only `ultimate.current` and `ultimate.maximum`. Costs remain presentation and weave evidence and have no potion meaning.

## Ultimate watch projection

| Current | Maximum | Watch state | Result |
| --- | --- | --- | --- |
| Unknown | Any | Enabled | Unavailable |
| Any | Unknown | Enabled | Unavailable |
| Points | Points(0) | Enabled | Unavailable |
| Points(c) | Points(m > 0) | Disabled | Ignored |
| Points(c) | Points(m > 0) | Enabled | Fresh; low exactly when `c * 100 <= threshold * m` |

For a qualifying cause, diagnostic percent is `ceil(c * 100 / m)`. Current greater than maximum never qualifies because thresholds cannot exceed 100.

## TriggerCause

```text
TriggerCause
|-- resource: Health | Magicka | Stamina | Ultimate
|-- observed_percent: u8
`-- threshold_percent: u8
```

Evaluation order is Health, Magicka, Stamina, Ultimate. The first qualifying enabled watch is the named cause. If at least one enabled watch is fresh and none qualify, the state is Ready. If no enabled watch is fresh, the state is `ResourcesUnavailable`.

## Persisted potion JSON

```json
{
  "health": { "enabled": false, "threshold": 35 },
  "magicka": { "enabled": false, "threshold": 35 },
  "stamina": { "enabled": false, "threshold": 35 },
  "ultimate": { "enabled": true, "threshold": 50 },
  "retry_interval_ms": 1500
}
```

Legacy objects may omit `ultimate`; omission normalizes to the default watch without altering sibling values or producing a migration warning.
