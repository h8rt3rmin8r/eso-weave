# Configuration and Session State

Session State is the `state.json` record for saved window position and suspend
persistence. Invalid Configuration may also be described as a corrupt config,
a reset to defaults, or the preserved `.invalid` file.

ESO Weave stores user settings separately from derived runtime state. Both files
live in `%APPDATA%\eso-weave\` on Windows and
`$XDG_CONFIG_HOME/eso-weave/` (or `~/.config/eso-weave/`) on Linux.

Files are pretty-printed JSON encoded as UTF-8 without a byte order mark, use LF
line endings, and end with a newline.

## `config.json`

The configuration contains user settings only. Module-owned sections include
timing, skills, beacon, fishing, potion, latency, pixelbus, and interface options.
A top-level `schema_version` supports forward migration. Invalid configuration
falls back to safe defaults, preserves the rejected file with an `.invalid`
suffix, and surfaces a notice.

Module validation can replace one invalid setting with its safe default while
retaining other valid fields. Unknown keys or a newer schema produce a notice and
best-effort loading, not a promise of forward migration. Use the
[Settings Reference](settings.md) for exact defaults, ranges, and effect timing.

## `state.json`

Session state holds derived runtime choices and caches, including suspend and
fishing intent, API-version observations, and window geometry. Restoring an
intent never permits input until the focused-game safety conditions are true.
Loading failures fall back safely rather than panicking. Unlike corrupt
`config.json`, a rejected session-state file has no `.invalid` preservation
guarantee.

Auto Potion enablement is not persisted. Its watches, thresholds, key, and retry
interval are settings, but the feature must be requested again after restart.

Writes are coalesced. A change marks the relevant store dirty and one write occurs
after the configured settling interval.

Window position and size, System and State disclosure, and Live Log height are
layout preferences. Closing the window or choosing Exit forces pending window
geometry to the session store so a final move or resize is not lost.

## Persistence matrix

| Value | Store | Restart behavior |
| --- | --- | --- |
| Modal settings, keybindings, Skills configuration | `config.json` | Restored; see Settings for current live versus restart timing |
| Theme, Always on Top, disclosure, Live Log height | `config.json` | Restored |
| Window geometry, suspension, Fishing request, API-version cache | `state.json` | Restored, but input still requires fresh safety evidence |
| Auto Potion request | Neither | Always starts Off |
| Current game, resource, quickslot, cooldown, and controller observations | Neither | Re-established from current runtime evidence |

## Recovery

If configuration is rejected, close ESO Weave and preserve the exact `.invalid`
copy before changing anything. A fresh default file can be generated on the next
save. Do not delete the whole configuration directory merely to repair one field.
The [Troubleshooting guide](../getting-started/troubleshooting.md#configuration-was-rejected)
gives the bounded recovery order.
