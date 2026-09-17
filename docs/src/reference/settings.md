# Settings Reference

Open **File > Settings**. Changes are saved automatically after a short settle
period; there is no Save button. Exact labels below match the interface.

## Appearance

| Setting | Choices and default | Effect |
| --- | --- | --- |
| Theme | Dark (default), Light | Changes the application color scheme immediately |
| Always on Top | Off by default | Keeps the ESO Weave window above other windows immediately |
| Stale Retention (seconds) | 120; 0 through 999 | Keeps the last coherent player-state presentation visible after runtime, focus, or signal loss; 0 clears immediately |

Retained values show a **HUD Freshness** row with their stale cause and
whole-second age. Fresh coherent observations replace them immediately. This
setting changes presentation only; current game evidence still blocks weaving,
Fishing, Auto Potion, and every other input-producing path at once.

## Combat Timing

| Setting | Default and accepted value | Effect |
| --- | --- | --- |
| Global Cooldown (ms) | 500; 0 through 60000 | Minimum interval between submitted weaves |
| Light Attack Delay (ms) | 50; 0 through 60000 | Gap before the skill for Light Attack and Block Casting |
| Heavy Attack Delay (ms) | 1000; 0 through 60000 | Heavy Attack hold before the skill |
| Bash Delay (ms) | 125; 0 through 60000 | Additional gap before the bash part of Bash Attack |
| Auto Timing from Weapon | Off | Replaces each known bar's Heavy Attack delay with its weapon preset |
| Adapt to Latency | Off | Adds a bounded latency allowance to Light Attack and Bash delays |
| Latency Factor | 0.25; finite 0 through 4 | Multiplies current latency before the allowance is rounded and capped at 300 ms |

When Auto Timing is off, the modal also shows separate back-bar Light Attack,
Heavy Attack, and Bash delays. The active bar selects the profile. Latency
adaptation adds its scaled allowance to Light Attack and Bash only, with a
300 ms cap.

## Fishing

| Setting | Default and accepted value | Effect |
| --- | --- | --- |
| Arm Timeout (ms) | 8000; 0 through 60000 | Maximum wait for cast confirmation |
| Reel Delay (ms) | 100; 0 through 60000 | Wait after a detected bite before reeling |
| Recast Delay (ms) | 3000; 0 through 60000 | Wait after a catch or timeout before recasting |
| Detected Interact Binding | Current ESO evidence; read-only | Chord generated for cast, reel, and recast actions; non-valid evidence blocks output |

The former statement that "All four Fishing controls apply live" now narrows to
the three timing controls because the fourth row is read-only. If a value changes while Fishing is
requested or active, ESO Weave stops that session without sending another input.
Start Fishing explicitly after the edit to use the new configuration. Applying
an unchanged form does not interrupt a session.

## PixelBeacon and Bus

| Setting | Choices and default | Effect |
| --- | --- | --- |
| AddOns Folder Override | Blank by default; existing directory | Takes precedence over automatic AddOns discovery |
| Game Environment | Live (default), PTS | Selects the ESO environment directory |
| Block Size (px) | 2, 4, 8, 16 (default), 32 | Changes each overlay square and capture geometry |
| Color Tolerance | 2 by default; 0 through 255 | Permits bounded per-channel sample variation; layout metadata caps its effective tolerance at 15 |
| Fast Sample Interval (ms) | 100; 1 through 60000 | Polling while Fishing or interception is active; safety-authoritative interception caps it at 375 ms |
| Sample Interval While Idle (ms) | 1000; 1 through 60000 | Polling while no fast condition applies |

Changing Block Size redeploys only a managed addon. Run `/reloadui` or relog and
restart ESO Weave. An unmanaged PixelBeacon target has no lifecycle buttons, and
the redeploy writer refuses it without changing its contents. Color Tolerance
and both sample intervals apply to the running reader. A tolerance change closes
cached safety evidence until a fresh sample is decoded.

## Auto Potion

| Setting | Default and accepted value | Effect |
| --- | --- | --- |
| Watch Health (Threshold %) | Off, 35%; threshold 0 through 100 | Qualifies when fresh Health is at or below the threshold |
| Watch Magicka (Threshold %) | Off, 35%; threshold 0 through 100 | Qualifies when fresh Magicka is at or below the threshold |
| Watch Stamina (Threshold %) | Off, 35%; threshold 0 through 100 | Qualifies when fresh Stamina is at or below the threshold |
| Detected Quickslot Binding | Current ESO evidence; read-only | Chord pressed for one eligible attempt; non-valid evidence blocks output |
| Minimum Retry Interval (ms) | 1500; 0 through 600000 | Floor between attempts, independent of observed cooldown |

The enabled watches use OR, not AND. Any one fresh enabled resource at or below
its own threshold can qualify after all earlier safety checks pass.

## Logging

| Setting | Choices and default | Effect |
| --- | --- | --- |
| Log Level | OFF, ERROR, WARN, INFO (default), DEBUG, TRACE | Changes the global captured level immediately and persists it |
| Write Log to File | Off by default | Adds or removes the monthly file sink immediately |

The Live Log level selector controls this same persisted global capture level
for both the in-memory ring and optional file logging.

## Local API and MCP

**Local API and MCP Server** is off by default and starts or stops both
authenticated loopback transports together. While running, Settings shows the
current lifecycle state, HTTP and MCP endpoints, and **Copy Credential** for a
trusted local client. Connected clients can read live player state and query ESO
Weave application data, so enable it only for local clients you trust. See
[Local API and MCP](local-api-and-mcp.md) for discovery, authentication,
schemas, examples, limits, compatibility, and troubleshooting.

## Keybindings

| Action | Default |
| --- | --- |
| Toggle Suspend | `F1` |
| Toggle Fishing | `F2` |
| Toggle Auto Potion | `F3` |

Supported choices are `1` through `5`, `E`, `R`, `X`, `Q`, Space, `F1`, `F2`,
and `F3`. A binding conflict is rejected and the previous assignment remains.
All bindings remain scoped to the focused ESO window. Combat bindings are
configured only in ESO and consumed from current PixelBeacon evidence. Legacy
combat entries in desktop settings are discarded and never become fallbacks.
Linux advertises every supported application binding and preserves unrelated
events from the selected physical devices.

## Main-window controls

Each Skills row has **Enabled**, **Weave**, **Override**, and **Delay (ms)**.
Enabled, selected weave type, and delay override persist. **Cooldown** is a
read-only observation. The System and State expanded preference, Live Log panel
height, and window geometry also persist. System and State defaults expanded.

The Running suspension, Fishing request, and Auto Potion request persist in
session state. Restored requests remain subject to all current runtime and
telemetry safety gates.

## Application timing

| Change | Saved | Current runtime effect |
| --- | --- | --- |
| Appearance, stale retention, bindings, weaving, Auto Potion, logging, AddOns override, environment | Yes | Applied to the relevant current application component |
| Fishing timing | Yes | Applied live; a changed configuration safely turns Fishing off |
| Detected Interact and Quickslot bindings | No | Read-only current ESO evidence; updates with PixelBeacon observations |
| Color Tolerance or sample intervals | Yes | Applied live at the next reader-worker iteration |
| Block Size | Yes | Managed addon redeploy is attempted; reload ESO and restart ESO Weave |
| Window, disclosure, and log height | Yes | Applied immediately as layout state |

A **Settings saved** toast confirms persistence. It does not claim that staged
Block Size geometry is active. See
[Configuration and Session State](configuration.md) for file ownership and
recovery.
