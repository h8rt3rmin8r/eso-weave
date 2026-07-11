# Research: Foundations

Phase 0 decisions for the project bootstrap, Config Store, and Logging. Each entry
records the decision, the rationale, and the alternatives rejected. There are no
open NEEDS CLARIFICATION items; the spec and its Clarifications section resolved
them.

## Toolchain and crate shape

**Decision**: One package pinned to Rust 1.96.0 (edition 2021) via
`rust-toolchain.toml` with the `rustfmt` and `clippy` components, built as a
library target plus a thin binary target. Version single-sourced from
`Cargo.toml`.

**Rationale**: 1.96.0 is the active stable toolchain on the build machine, so CI
parity runs against exactly what is installed. A library target makes every module
unit and integration testable without launching the binary, which the test-first
principle requires. One package keeps the "single crate" convention from master
specification section 14.

**Alternatives considered**: A binary-only crate (rejected: integration tests would
have to shell out or duplicate logic). A Cargo workspace (rejected: section 14
requires a documented justification and there is none yet).

## Platform paths

**Decision**: Resolve directories with the `dirs` crate and join `eso-weave`.
Config file at `dirs::config_dir()/eso-weave/config.json`. Logs at
`dirs::state_dir()/eso-weave/logs/` on Linux and `dirs::config_dir()/eso-weave/logs/`
on Windows (where `state_dir` is not defined). The Windows-versus-Linux log
location difference is the one genuine platform fork and lives behind the
`platform` module seam.

**Rationale**: `dirs::config_dir()` returns `%APPDATA%` on Windows and
`$XDG_CONFIG_HOME` (falling back to `~/.config`) on Linux, which matches master
specification section 11 exactly. `dirs::state_dir()` returns `$XDG_STATE_HOME`
(falling back to `~/.local/state`) on Linux and `None` on Windows, matching
section 12.

**Alternatives considered**: The `directories` crate `ProjectDirs` (rejected: it
injects qualifier and organization subfolders, producing paths that do not match
the specification's exact `eso-weave/config.json` layout). Hand-parsing
environment variables (rejected: brittle and re-implements a solved problem).

## Config serialization, migration, and unknown keys

**Decision**: Serialize with `serde` and `serde_json` using a pretty printer, then
write bytes explicitly with LF endings, no byte order mark, and a trailing
newline. A top-level integer `schema_version` gates a stepwise forward migration.
Unknown top-level keys are captured via a flattened `extra` map, and their presence
raises a warning notice before they are dropped; recognized keys are never dropped.

**Rationale**: `serde_json` emits UTF-8 without a BOM and does not force platform
line endings, so writing bytes ourselves guarantees LF on Windows too. Capturing
unknown keys through a flattened map lets FR-008 be satisfied (warn rather than
silently discard) without adopting `deny_unknown_fields`, which would reject files
instead of migrating them.

**Alternatives considered**: `deny_unknown_fields` (rejected: turns forward-compat
into a hard failure, violating FR-008). Preserving unknown keys across save
(deferred: adds round-trip complexity with no consumer yet; a warning meets the
requirement now).

## Corruption fallback and preserved name

**Decision**: On parse failure, log a warn notice, rename the bad file by appending
`.invalid` to its path, and continue on defaults. If `<path>.invalid` already
exists, append a numeric discriminator (`.invalid.2`, `.invalid.3`, ...).

**Rationale**: Directly implements FR-009 and the corresponding edge case, ensuring
no previously preserved file is overwritten.

**Alternatives considered**: Overwriting a single `.invalid` file (rejected: loses
an earlier preserved copy). Deleting the corrupt file (rejected: destroys data the
operator may want).

## Logging: runtime level, ring buffer, monthly file, privacy

**Decision**: Build on `tracing` with a `tracing-subscriber` registry composed of
(a) a level filter wrapped in `tracing_subscriber::reload` so the active level
changes at runtime through a stored reload handle, (b) a custom ring-buffer layer
holding the most recent events in a bounded `VecDeque` (default 1000, oldest
evicted first) behind a shared lock, and (c) an optional custom month-file layer
guarded by a flag that writes `YYYY-MM.log`, reopening when the month changes.
Timestamps come from `time::OffsetDateTime::now_utc()` formatted RFC-3339 (an
ISO-8601 profile). The active level and file-enabled flag initialize from persisted
settings; a suppression control gates any future input-target events.

**Rationale**: The `reload` layer is the standard `tracing` mechanism for changing
verbosity without a restart (FR-011). A custom ring-buffer layer keeps the in-memory
view independent of the file sink (FR-012). `tracing-appender` offers hourly, daily,
minutely, and never rotation but not monthly, so the month file is a small custom
layer (FR-013). RFC-3339 satisfies the ISO-8601 UTC requirement (FR-014).

**Alternatives considered**: `tracing-appender` rolling files (rejected for the file
sink: no monthly cadence; a custom layer is simpler than post-processing). `log`
plus `env_logger` (rejected: no structured fields or runtime reload). `chrono` for
timestamps (rejected: `time` is lighter and already idiomatic with `tracing`).

## Errors and outcomes

**Decision**: Typed errors via `thiserror`. Config load returns an outcome carrying
the resulting settings plus a list of notices (warn-level events such as a
corruption fallback or unknown-key warning) that the caller emits through logging.

**Rationale**: A typed outcome makes "surface a notice" testable without a GUI
(FR-018) and keeps notice production separate from notice presentation, which the
GUI slice will add.

**Alternatives considered**: Panicking or returning bare `io::Error` (rejected:
loses the notice channel and the resilience guarantees). Logging directly from deep
inside config load (rejected: couples the store to a global logger and is harder to
test).
