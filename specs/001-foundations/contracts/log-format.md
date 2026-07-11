# Contract: Log Line and File Format

## Persisted file

- One file per calendar month named `YYYY-MM.log` (for example `2026-07.log`).
- Located at the platform log directory under `eso-weave/logs/`:
  - Linux: `$XDG_STATE_HOME/eso-weave/logs/` (fallback `~/.local/state/eso-weave/logs/`).
  - Windows: `%APPDATA%/eso-weave/logs/`.
- UTF-8 without a byte order mark, LF line endings, one event per line, appended.
- Written only while the file sink is enabled. Crossing a month boundary begins a
  new file named for the new month.

## Line format

Each line contains, in order:

1. A coordinated-universal-time timestamp in RFC-3339 / ISO-8601 form
   (for example `2026-07-11T00:14:22Z`).
2. The level (`ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`).
3. The target (source module or subsystem).
4. The message.

Example:

```text
2026-07-11T00:14:22Z  INFO  eso_weave::config  loaded settings (schema_version=1)
```

## Privacy rule

- No line at `INFO` or higher (above `DEBUG`) contains specific input contents.
- The facility exposes a suppression control; the suspend-driven use of it (no
  keystroke logging while suspended) is wired by the later Input Engine slice.

## In-memory buffer

- The always-on ring buffer holds the most recent events (default capacity 1000),
  independent of whether the file sink is enabled. The oldest event is evicted when
  the buffer is full.
