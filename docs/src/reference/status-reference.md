# Status Reference

Use the exact visible text to identify the first failing observation. Color and
status dots are supplementary; the words below carry the meaning.

## Common unavailable states

| Text | Meaning | Automation impact and recovery |
| --- | --- | --- |
| **Game not active** | ESO is not running as an active client | No generated feature input; start ESO |
| **Not detected** | No valid observation has been received for this field | Restore its provider or PixelBeacon source |
| **Signal unavailable** | ESO is active but current telemetry is not authoritative | Restore a fresh visible overlay and complete loading |
| **Unknown** | Available evidence cannot authorize a conclusion | Wait for a fresh positive observation; do not treat it as safe |

Observed numeric zero is different from all four states.

## System and State

| Field and text | Meaning | Recovery or effect |
| --- | --- | --- |
| Game: **Inactive** | Launcher and game client are absent | Start ESO |
| Game: **Launcher open** | Launcher is present, game client absent | Start the game |
| Game: **Active** | Game client is present | Focus and telemetry are evaluated separately |
| Game: **Unknown** | Process evidence is inconclusive | Retry and inspect logs |
| Provider: **Detected (ESO Store/Steam/Epic Games/Steam Proton)** | One validated installation provider won reconciliation | No action |
| Provider: **Multiple installs detected** | Strong candidates conflict | Identify the intended install before setup |
| Provider: **Not detected** | No validated installation candidate | Confirm the installation |
| Travel: **Inactive** | No bounded travel attempt is pending | Required for weaving, Fishing, and Auto Potion |
| Travel: **Pending** or **Not detected** | Recall or jump is pending, or cannot be ruled out | Generated input is blocked; complete or cancel travel and await fresh Inactive |
| World State: **Active** | Player activation published a complete baseline | Required for generated features |
| World State: **Transitioning** or **Not detected** | Loading or no complete baseline | Generated input is blocked; wait for Active |
| ESO Weave: **Active** | Application is not suspended | Features may still have other blockers |
| ESO Weave: **Suspended** | New weave interception and Auto Potion input stop; queued or running weave is invalidated; Fishing deadlines cancel while its request is retained | Resume with `F1` or Running toggle; Fishing then requires a fresh manual cast or off-on recovery |
| PixelBeacon Status: **Not installed** | The PixelBeacon target does not exist | Install the bundled addon |
| PixelBeacon Status: **Unmanaged (not modified)** | A target exists but ownership cannot be proven | No lifecycle action is offered; move or remove only that exact target manually |
| PixelBeacon Status: **Installed (current)** | Managed version matches | No action |
| PixelBeacon Status: **Installed (outdated)** | The managed marker exists but the embedded version differs | Update the managed copy in place |
| PixelBeacon Status: **AddOns folder not found** | No usable directory was resolved | Correct environment or AddOns override |
| PixelBeacon Signal: **Signal detected** | Fresh heartbeat is present | Field-specific telemetry may now authorize behavior |
| PixelBeacon Signal: **Signal lost** | A previously fresh heartbeat timed out | Telemetry clears and automation stops until recovery |
| PixelBeacon Signal: **Not detected** | Active game, no heartbeat seen | Enable/reload addon and expose overlay |

## Game Context and player state

| Field and text | Meaning | Automation impact |
| --- | --- | --- |
| Game Context: **Gameplay** | Active, focused game with fresh no-menu observation | Can authorize input with the other gates |
| Game Context: **Unfocused** | ESO lacks keyboard focus | No focused-game interception or autonomous input |
| Game Context: **Signal unavailable** or **Unknown** | Surface evidence is missing or inconclusive | Does not authorize generated input |
| Game Context: **System menu**, **Map**, **Inventory**, **Mail**, **Character**, **Guild store**, **Crown store**, **Journal**, **Chat entry**, or **Other menu** | A native surface or text entry is open | Input is gated; close it and await Gameplay |
| Combat: **In combat** or **Out of combat** | Current player combat observation | Display-only in this release |
| Movement: **On foot** or **Mounted** | Current movement family | Display; Mounted is not treated as Sprinting |
| Movement: **Sprinting** | Bounded keyboard-mode on-foot sprint inference | Defers Auto Potion |
| Movement: **Not detected** | No valid movement value | Does not invent Sprinting |
| Roll Dodge: **Inactive** | No active player dodge | Required for weaving |
| Roll Dodge: **Active** or **Not detected** | Dodge active or unavailable | Physical skill passes through and weave synthesis is blocked |
| Life State: **Alive** | Fresh authoritative alive observation | Required for generated features |
| Life State: **Dead**, **Recovering (ghost)**, **Recovering (world activation)**, **Recovering (no load)**, or **Not detected** | Player cannot yet be authoritatively treated as alive | Generated input is blocked |
| Weapon Bar: **Front** or **Back** with classes | Active bar and both weapon classes were decoded | Selects timing and Ultimate cost |
| Weapon Bar: **Not detected** or class **Unknown** | Bar evidence is absent or partial | Configured timing fallback applies; Ultimate cost may hide |

## Resources and Ultimate

| Text or cue | Meaning |
| --- | --- |
| **N%** | Fresh Health, Stamina, or Magicka percentage |
| **Low: N%** | Fresh watched resource at or below its configured threshold |
| Empty bar with **0%** | Valid observed zero, not unavailable |
| **CURRENT / MAX** on Ultimate | Exact current and game-reported maximum points |
| Protruding Ultimate tick | Exact cost of the Ultimate on the active normal bar |
| **Ready** | Current Ultimate is at or above the exact active cost |
| Hidden Ultimate tick and Ready slot | Cost, active bar, or compatible telemetry is unavailable |

All meters include numeric or textual state, fill, and programmatic progress.
Quarter landmarks are unlabeled orientation aids. Ultimate remains display-only.

## Quickslot

The one Quickslot row composes classification, potion availability, and cooldown.

| Text | Meaning and action |
| --- | --- |
| **Not detected** | No signal; restore PixelBeacon |
| **Addon update required** | Legacy layout lacks classification; update managed PixelBeacon |
| **Unreadable signal** | Recognized but corrupt classification; check overlay and tolerance |
| **Unsupported game API** | Current ESO API cannot supply required facts |
| **Invalid selection** or **Inconsistent game data** | Facts cannot be trusted; reselect and await refresh |
| **Empty** | No selected quickslot entry |
| **Non-potion (Item/Collectible/Quest item/Emote/Quick chat/Other)** | Selected entry is not a potion |
| **Potion \| Depleted** | Potion classification is valid but stock is zero |
| **Potion \| Blocked** | ESO reports the potion unusable |
| **Potion \| Usable** | Potion classification and availability can authorize Auto Potion |
| **Ready** or a duration | Independent potion cooldown observation |
| **Not applicable** | Availability or cooldown does not apply to this classification |

## Fishing

**Casting**, **Fishing (waiting for a bite)**, **Reeling in**, and **Recasting**
name active controller phases. **Idle** can be a user stop or can include **no
cast detected**, **signal lost**, **game not active**, **game unfocused**,
**player unavailable**, **world unavailable**, or **travel pending**. Use the
[Fishing recovery table](../features/fishing.md#status-meanings).

## Auto Potion

**Off**, **Dormant**, **Blocked**, **Ready**, and **Triggered** describe requested
and effective state, not merely the toggle. The full first-blocker list and
recovery actions are in [Auto Potion](../features/auto-potion.md#status-and-recovery).

## Skills cooldown

Each Skill 1 through Skill 5 and Ultimate row shows **Ready**, a remaining
duration, or a dash when unavailable. Synergy always uses the dash because ESO
does not expose its cooldown. These values are display-only and do not control
weave execution.
