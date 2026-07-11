# Contract: Settings JSON Schema

The on-disk settings file. One file per user at
`<config_dir>/eso-weave/config.json`.

## Encoding

- UTF-8 without a byte order mark.
- Line-feed (`\n`) line endings on every platform.
- Pretty-printed (two-space indent) with a trailing newline.
- Contains user settings only. No runtime or session state.

## Shape (schema_version 1)

```json
{
  "schema_version": 1,
  "logging": {
    "level": "info",
    "file_enabled": false
  }
}
```

## Field rules

- `schema_version` (integer, required): known current value is 1. Lower values are
  migrated forward on load; higher values load best effort with a warning.
- `logging.level` (string): one of `off`, `error`, `warn`, `info`, `debug`,
  `trace`. Invalid values fall back to `info` with a warning.
- `logging.file_enabled` (boolean): whether the monthly file sink is enabled.
- Unknown top-level keys: preserved-in-memory only long enough to warn, then
  dropped. Their presence is never a hard error.

## Corruption handling

If the file is not valid JSON or does not match the schema in a way that cannot be
migrated, it is renamed to `config.json.invalid` (or `config.json.invalid.N` if
that already exists), and the application continues on defaults with a warning.
