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
| ESO Weave Data: **Not installed** | The data-addon package is absent | Choose **Install Data** when catalog or encounter workflows are needed |
| ESO Weave Data: **Installed** with Ownership **Managed** and Compatibility **Current** | Exact marker, inventory, version, and embedded bytes match | No lifecycle action is required; Repair and Uninstall remain explicit choices |
| ESO Weave Data: **Installed** with Compatibility **Update available** | Ownership is proven but package content differs | Choose **Update Data** or **Repair Data**, then follow reload guidance |
| ESO Weave Data: Ownership **Unmanaged** | A target exists but ownership or safe shape cannot be proven | No mutation is offered; move or remove only `EsoWeaveData` manually |
| ESO Weave Data: **AddOns folder not found** | No usable directory was resolved | Correct environment or AddOns override |
| Data Addon Enabled or Data Addon Loaded: **Unconfirmed** | No supported current-session account or addon-load source exists | Verify enablement in ESO; reload after lifecycle changes; do not infer from process state |
| Data Addon Reload: **Required** | A lifecycle change occurred while ESO was running or runtime was uncertain | Run `/reloadui` or relog before relying on the change |
| Data Runtime: **Available**, **Unavailable**, or **Unknown** | ESO process evidence only | Never treat this value as addon load or collection evidence |
| Catalog Collection: **Unconfirmed (no live channel)** | Desktop has no current-session catalog channel | Use `/ewcollect status`; disk SavedVariables remains flush-bound historical evidence |
| Encounter Collection: **Last saved MODE / STATE** | A compatible bounded spool was read from the last disk flush | Historical evidence only; use `/ewencounter status` inside ESO for current mode and state |
| Encounter Collection: **Unconfirmed (no saved state)** | No compatible last-saved controller fact is available | Verify and control capture inside ESO; never infer activity from process or package state |
| Data Addon Next Step | Current lifecycle and inspection evidence | Follow the named safe action; unavailable or unmanaged evidence never enables automatic mutation |
| PixelBeacon Signal: **Signal detected** | Fresh heartbeat is present | Field-specific telemetry may now authorize behavior |
| PixelBeacon Signal: **Signal lost** | A previously fresh heartbeat timed out | Telemetry clears and automation stops until recovery |
| PixelBeacon Signal: **Not detected** | Active game, no heartbeat seen | Enable/reload addon and expose overlay |
| Catalog: **VERSION (live/pts, API N)** | A compatible catalog passed read-only schema, integrity, foreign-key, and semantic checksum verification | Typed catalog queries are available for this process |
| Catalog: **Catalog unavailable: REASON** | The package file is missing, corrupt, incompatible, or checksum-invalid | Catalog queries return empty; unrelated application features continue working |

Maintainer review candidates have no application status row. Candidate generation
cannot authenticate its origin. A user-approved candidate appears here only after
the Catalog Update worker verifies, installs, opens, and atomically selects it.
Invalid user selections visibly fall back to the separately verified package
catalog.

The Catalog Update notice distinguishes Catalog current, New Live data
available, Update ready to import, Collector capture required, Offline or stale
check, Unsupported schema, and catalog unavailable. PTS previews are labeled
separately and never participate in Live selection.

The shared ESO Weave Data addon has separate package lifecycle values:

| Lifecycle API value | Meaning | Recovery or effect |
| --- | --- | --- |
| `not-installed` | The shared data-addon package is absent | Install it explicitly when local catalog or encounter capture is needed |
| `managed-up-to-date` | Marker, version, and embedded checksum match | Capture may be started in ESO after any required reload |
| `managed-version-mismatch` | Ownership is proven but content differs | Use the main interface Update or Repair action, or the maintainer CLI |
| `unmanaged` | Ownership, file type, or link safety could not be proven | No change is made; inspect only that exact target manually |

These API values back the first-class lifecycle row. They do not describe
PixelBeacon, configured enablement, addon loading, collection, or automation
state.

The encounter controller exposes exactly two selected modes. `single` records
one combat period and stops; `continuous` retains one bounded session across
combat gaps until explicit disablement or hard failure. Live authority belongs
only to user commands inside ESO:

- `stopped` means no capture request is effective. Select mode and channel, then
  use `/ewencounter toggle`.
- `waiting` means single or continuous is enabled outside combat. Detailed
  handlers stay dormant until combat begins; toggle again to disable.
- `capturing` means one encounter is active. A combat exit completes it;
  toggling off makes it a user-stopped partial record.
- `interrupted` means reload, relog, or recovery opened an explicit observation
  gap. Single stops; valid durably saved continuous authority can return to
  waiting under the same session.
- `failed` means a controlled integrity or storage failure stopped capture.
  Retained evidence stays in place and no automatic retry occurs.

Mode, requested enablement, effective state, active channel, session identity,
current encounter, last interruption, and failure are independent facts. The
desktop may display only validated, bounded versions from the last saved spool,
always under a **Last saved capture state** or historical label. It has no capture
toggle or addon command ingress.

The local icon cache exposes library lookup states for future interface work:

| Icon value | Meaning | Recovery or effect |
| --- | --- | --- |
| `Ready` | A verified user-local PNG or DDS source produced the mapped immutable PNG object | Render the local object |
| `Placeholder` | A manifest explicitly selected the project placeholder without a more specific reason | Render the placeholder |
| `Missing` | The requested mapping or local source file is absent | Render the placeholder; supply the exact local path if desired |
| `Unsupported` | The selected source has an unsupported file type | Render the placeholder; use bounded PNG or DDS input |
| `Failed` | A path, permission, decode, limit, I/O, link, or integrity check failed | Render the placeholder and correct the local source or cache |

These states do not authorize input and do not imply that a cache is active in
the application.

## Game Context and player state

| Field and text | Meaning | Automation impact |
| --- | --- | --- |
| HUD Freshness: **Stale: cause (Ns)** | Player-state values are the last coherent rendered snapshot; cause names runtime, focus, or signal loss and `N` is whole-second age | Display-only; current evidence has already blocked applicable input-producing paths |
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
