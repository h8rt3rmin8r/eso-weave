# Contract: Public Library API

The library surface this slice exposes. Signatures are language-neutral; the tasks
phase fixes exact Rust types. Downstream slices depend on these seams.

## config module

- `load(config_dir) -> LoadOutcome`
  - Reads the settings file under the given config directory. On success returns the
    parsed Settings and any notices. On a parse error, preserves the bad file
    (`.invalid`, with numeric discriminator on collision), returns default Settings,
    and includes a corrupt_config notice. On a missing file, returns default
    Settings with no notice. On a read error, returns default Settings and an
    unwritable notice. Never panics.
  - Applies forward migration when schema_version is below current, adding a
    migrated notice. Captures unknown top-level keys and adds an unknown_keys notice.

- `save(config_dir, settings) -> Result<(), ConfigError>`
  - Writes settings as pretty JSON, UTF-8 without BOM, LF endings, trailing newline.
    Creates the directory if absent. On write failure returns a ConfigError (the
    caller turns this into an unwritable notice).

- `LoadOutcome { settings: Settings, notices: Vec<Notice> }`
- `Settings::default() -> Settings` (schema_version = current, logging = default)

## logging module

- `init(prefs, log_dir) -> LogHandle`
  - Installs the tracing subscriber (level filter with reload, ring-buffer layer,
    optional month-file layer). The active level and file-enabled state come from
    `prefs`. The month-file layer writes under `log_dir`. Idempotent per process.
- `build(prefs, log_dir) -> (Subscriber, LogHandle)`
  - The test seam behind `init`: composes the same subscriber and handle without
    setting the global default, so tests can install it in a scope via
    `tracing::subscriber::with_default` (the global default may be set only once
    per process). `init` calls `build` and then sets the global default.

- `LogHandle::set_level(level)` changes the active level immediately (FR-011).
- `LogHandle::set_file_enabled(enabled)` toggles the month file sink (FR-013).
- `LogHandle::set_input_suppressed(suppressed)` sets the input suppression control
  (FR-015).
- `LogHandle::recent(limit) -> Vec<LogEvent>` returns the most recent buffered
  events, newest last, independent of the file sink (FR-012).
- `LogHandle::current_prefs() -> LoggingPrefs` returns the live level and
  file-enabled state so the caller can persist them on save (FR-017).

## Error contract

- `ConfigError` is a typed error (variants for io and serialization failures). It is
  returned only from `save`; `load` never fails, it degrades to defaults plus
  notices.

## Notes

- No function in this slice reads or writes game memory, network, or any in-game
  surface (Constitution V).
- `load` and `save` take the directory as a parameter so tests can target a
  temporary directory rather than the real platform path.
