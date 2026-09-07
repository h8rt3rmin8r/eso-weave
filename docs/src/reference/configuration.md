# Configuration and Session State

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

## `state.json`

Session state holds derived runtime choices and caches, including suspend and
fishing intent plus API-version observations. Restoring an intent never permits
input until the focused-game safety conditions are true. Loading failures fall
back safely rather than panicking.

Auto Potion enablement is not persisted. Its watches, thresholds, key, and retry
interval are settings, but the feature must be requested again after restart.

Writes are coalesced. A change marks the relevant store dirty and one write occurs
after the configured settling interval.
