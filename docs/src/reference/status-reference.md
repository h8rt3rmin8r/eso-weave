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
| PixelBeacon: **Not installed** | Neither a PixelBeacon target nor a recognized managed legacy Data package exists | The ordinary PixelBeacon lifecycle installs the complete package |
| PixelBeacon: **Unmanaged (not modified)** | A target exists but ownership cannot be proven | No lifecycle action is offered; move or remove only that exact target manually |
| PixelBeacon: **Installed (current)** | Complete managed inventory, version, configured geometry, and startup configuration match | No action |
| PixelBeacon: **Installed (outdated)** | A recognized managed package needs update, completion, or migration | Automatic legacy migration or the ordinary Update lifecycle stages a complete replacement |
| PixelBeacon: **AddOns folder not found** | No usable directory was resolved | Correct environment or AddOns override |
| PixelBeacon Signal: **Signal detected** | Fresh heartbeat is present | Field-specific telemetry may now authorize behavior |
| PixelBeacon Signal: **Signal lost** | A previously fresh heartbeat timed out | Telemetry clears and automation stops until recovery |
| PixelBeacon Signal: **Not detected** | Active game, no heartbeat seen | Enable/reload addon and expose overlay |

Completed Catalog snapshots enter desktop maintenance after ESO naturally saves
them. Background work verifies, opens, and atomically selects a compatible
Catalog. Unchanged semantic content retains the current generation. Readers
receive a coherent selection; an interrupted build cannot replace the last
usable Catalog.

Startup recovery checks retained verified selections before the separately
verified packaged fallback. Live and PTS retain independent selections. Disabling
Catalog maintenance retains existing usable data and prevents stale work from
activating a candidate; re-enabling resumes automatically when eligible. Canonical local state reports the selected data. No source-selection, review,
import-ready notice, or refresh action is part of this automatic path.

The historical Data lifecycle API is a compatibility alias for the complete
PixelBeacon package. It never creates a separate ESO Weave Data installation.

| Lifecycle API value | Meaning |
| --- | --- |
| `not-installed` | The combined package and recognized managed legacy targets are absent |
| `managed-up-to-date` | The complete current managed inventory matches |
| `managed-version-mismatch` | An owned package needs update, completion, or legacy migration |
| `unmanaged` | Ownership, exact inventory, regular files, or stable reads cannot be established |

These values describe disk management, not whether Lua is loaded. Compatible
combined presence is evaluated separately for background eligibility; manually
maintained packages can be eligible without granting overwrite or removal
permission. Actual Live and PTS roots are independent.

Both persistent disable preferences are retained through absence. No compatible
combined installation makes both background services dormant and both controls inactive. Desktop Catalog disable stops reads and activation immediately; the
package startup preference applies to Lua at the next natural addon load. An
unavailable inspection does not reset preferences or fabricate package presence.

## Background data observations

The optional local API/MCP exposes canonical maintenance preferences, package
eligibility and actual service observations. Catalog and native processing are
independent. No file's existence establishes that ESO is actively logging.
Stored raw data and supported results remain available while services are
disabled or dormant. Missing native observations remain unavailable, not zero.
The desktop dashboard reserves no Catalog, separate Data or encounter viewer row.

See [Encounter Data and Metrics](encounter-data-and-metrics.md) for source-specific
capabilities and [Local API and MCP](local-api-and-mcp.md) for client access.

## Game Context and player state

The PixelBeacon field keeps ordinary installation text concise, for example
**Installed (current)**. It adds **API unsupported; update addon** with warning
styling only when current selected-environment evidence is excluded by the
installed manifest's valid API declarations. Missing, stale, malformed, or
other-environment evidence does not fabricate incompatibility or add routine API
text. Technical API/client evidence remains separate in canonical diagnostics.

| Field and text | Meaning | Automation impact or next action |
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

Stored-data access remains independent of collection eligibility.
