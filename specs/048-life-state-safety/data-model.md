# Data Model: Life State Safety

## LifeState

| Variant | Wire payload | Meaning | Gates synthesis |
| --- | ---: | --- | --- |
| `Unknown` | none | Missing, invalid, legacy, or lost evidence | Yes |
| `Alive` | `0x20` | Player can perform ordinary actions | No |
| `Dead` | `0x80` | Player is dead | Yes |
| `Reincarnating` | `0xE0` | Player is transitioning back to life | Yes |

`gates()` is exactly `self != Alive`. The wire decoder also validates a distinct
green marker and `red + blue == 255` within tolerance.

## Reader state

`PixelBusReader` retains the last normalized life state only to emit changes.
Every heartbeat sample decodes B21. A non-decoding B21 changes the value to
Unknown. `lose_signal()` also changes it to Unknown exactly once.

## Runtime consumers

- `InputEngine.life_gated`: atomic classifier flag. Non-exempt physical bindings
  pass through while true.
- `WeaveEngine.life`: worker-side guard and view-model storage. Non-Alive drops
  the handed-off action before cooldown bookkeeping.
- `FishingController.life`: effective safety state. A non-Alive transition
  cancels pending work without changing the request toggle.
- `AutoPotionController.life`: rule input and blocker source. Non-Alive wins after
  process, focus, heartbeat, and suspend gates and before resource evaluation.

## LifeStateView

| Input | Display | Role |
| --- | --- | --- |
| Active game plus Alive | Alive | Healthy |
| Active game plus Dead | Dead | Warning |
| Active game plus Reincarnating | Reincarnating | Warning |
| Active game plus Unknown | Not detected | Muted |
| Inactive game | Game not active | Muted |

## UiPrefs

`system_state_expanded: bool`, default true, is stored inside the existing `ui`
JSON object. Older configurations omit it and therefore open the panel. A user
toggle updates the live preference and schedules a silent layout-preference save.

