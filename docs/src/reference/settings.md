# Settings Reference

Open **File > Settings**. Changes are saved automatically after a short settle
period; there is no Save button. Exact labels below match the interface.

## Appearance

| Setting | Choices and default | Effect |
| --- | --- | --- |
| Theme | Dark (default), Light | Changes the application color scheme immediately |
| Always on Top | Off by default | Keeps the ESO Weave window above other windows immediately |

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

The stored interact key defaults to `E`, but the current modal has no editor for
it. This UI and application-timing gap is tracked in
[issue #95](https://github.com/h8rt3rmin8r/eso-weave/issues/95).

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
the redeploy writer refuses it without changing its contents.

## Auto Potion

| Setting | Default and accepted value | Effect |
| --- | --- | --- |
| Watch Health (Threshold %) | Off, 35%; threshold 0 through 100 | Qualifies when fresh Health is at or below the threshold |
| Watch Magicka (Threshold %) | Off, 35%; threshold 0 through 100 | Qualifies when fresh Magicka is at or below the threshold |
| Watch Stamina (Threshold %) | Off, 35%; threshold 0 through 100 | Qualifies when fresh Stamina is at or below the threshold |
| Quickslot Key | `Q`; supported key list below | Key pressed for one eligible attempt |
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

## Keybindings

| Action | Default |
| --- | --- |
| Skill 1 through Skill 5 | `1` through `5` |
| Ultimate | `R` |
| Synergy | `X` |
| Toggle Suspend | `F1` |
| Toggle Fishing | `F2` |
| Toggle Auto Potion | `F3` |

Supported choices are `1` through `5`, `E`, `R`, `X`, `Q`, Space, `F1`, `F2`,
and `F3`. A binding conflict is rejected and the previous assignment remains.
All bindings remain scoped to the focused ESO window. Linux advertises every
supported binding and preserves the selected physical keyboard's other keys.

## Main-window controls

Each Skills row has **Enabled**, **Weave**, **Override**, and **Delay (ms)**.
Enabled, selected weave type, and delay override persist. **Cooldown** is a
read-only observation. The System and State expanded preference, Live Log panel
height, and window geometry also persist. System and State defaults expanded.

The Running suspension and Fishing request persist in session state. Auto Potion
request never persists and always starts off.

## Application timing

| Change | Saved | Current runtime effect |
| --- | --- | --- |
| Appearance, bindings, weaving, Auto Potion, logging, AddOns override, environment | Yes | Applied to the relevant current application component |
| Fishing timing or stored interact key | Yes | Restart ESO Weave; live propagation is missing in issue #95 |
| Color Tolerance or sample intervals | Yes | Restart ESO Weave; live propagation is missing in issue #95 |
| Block Size | Yes | Managed addon redeploy is attempted; reload ESO and restart ESO Weave |
| Window, disclosure, and log height | Yes | Applied immediately as layout state |

A **Settings saved** toast confirms persistence, not that a restart-dependent
component has reconfigured. See [Configuration and Session State](configuration.md)
for file ownership and recovery.
