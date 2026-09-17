# Configuration and Session State

The persisted local-extension preference includes only enablement and its
bearer credential. While the service runs, a separate non-secret discovery file
publishes loopback endpoints, process, generation, and schema version. See
[Local API and MCP](local-api-and-mcp.md) for ownership, authentication, and
restart rules.

Session State is the `state.json` record for saved window position, application
suspension, and requested Fishing and Auto Potion enablement. Invalid
Configuration may also be described as a corrupt config, a reset to defaults,
or the preserved `.invalid` file.

ESO Weave stores user settings separately from derived runtime state. Both files
live in `%APPDATA%\eso-weave\` on Windows and
`$XDG_CONFIG_HOME/eso-weave/` (or `~/.config/eso-weave/`) on Linux.

Files are pretty-printed JSON encoded as UTF-8 without a byte order mark, use LF
line endings, and end with a newline.

## `config.json`

The configuration contains user settings only. Module-owned sections include
timing, skills, beacon, fishing, potion, latency, pixelbus, and interface options.
A bounded `ui.stale_retention_seconds` preference controls display-only HUD
retention and defaults to 120 when absent.
A top-level `schema_version` supports forward migration. Invalid configuration
falls back to safe defaults, preserves the rejected file with an `.invalid`
suffix, and surfaces a notice.

Module validation can replace one invalid setting with its safe default while
retaining other valid fields. Unknown keys or a newer schema produce a notice and
best-effort loading, not a promise of forward migration. Use the
[Settings Reference](settings.md) for exact defaults, ranges, and effect timing.

## `state.json`

Session state holds operator runtime choices and caches, including suspend,
Fishing intent, Auto Potion intent, API-version observations, and window
geometry. Restoring an intent never permits input until its current focused-game
and telemetry safety conditions are true. Loading failures fall back safely
rather than panicking. Unlike corrupt `config.json`, a rejected session-state
file has no `.invalid` preservation guarantee.

Auto Potion watches, thresholds, and retry interval remain settings. Only
the requested on or off toggle is session state. Effective state, telemetry,
blockers, and retry history are always rebuilt from the running process.

Legacy `fishing.interact_key` and `potion.quickslot_key` members are accepted as
unknown input for compatibility, never used as authority, and omitted on the
next settings save. Current Interact and Quickslot chords come only from live
PixelBeacon evidence of ESO controls.

Writes are coalesced. A change marks the relevant store dirty and one write occurs
after the configured settling interval.

Window position and size, System and State disclosure, and Live Log height are
layout preferences. Closing the window or choosing Exit forces pending window
geometry to the session store so a final move or resize is not lost.

## Persistence matrix

| Value | Store | Restart behavior |
| --- | --- | --- |
| Modal settings, keybindings, Skills configuration | `config.json` | Restored; see Settings for current live versus restart timing |
| Theme, Always on Top, stale-retention interval, disclosure, Live Log height | `config.json` | Restored |
| Window geometry, suspension, Fishing request, Auto Potion request, API-version cache | `state.json` | Restored, but input still requires fresh safety evidence |
| Current game and controller observations | Neither | Re-established from current runtime evidence |
| Retained HUD snapshot, stale cause, age, and deadline | Neither | Process-local only; never restored after ESO Weave exits |

The retained HUD snapshot contains rendered presentation only. It is never
written to either store and never becomes input to a controller or authorization
gate.

## Recovery

If configuration is rejected, close ESO Weave and preserve the exact `.invalid`
copy before changing anything. A fresh default file can be generated on the next
save. Do not delete the whole configuration directory merely to repair one field.
The [Troubleshooting guide](../getting-started/troubleshooting.md#configuration-was-rejected)
gives the bounded recovery order.
