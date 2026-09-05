# Data Model

## TravelState

| State | Meaning | Gate |
| --- | --- | --- |
| `Unknown` | Telemetry cannot prove travel safety | Closed |
| `Inactive` | Valid lifecycle data shows no pending travel | Open when world is also active |
| `Pending` | A recall or jump attempt is underway | Closed |

## Addon Detector State

- Last recall cooldown sample
- Pending source: recall or jump
- Pending start timestamp
- Watchdog deadline
- Lifecycle-valid flag

## Safety Invariant

Generated input is allowed only when `WorldState::Active` and `TravelState::Inactive`. Unknown data is always fail-safe.
