# Tasks: Foundations (Project Bootstrap, Config Store, Logging)

**Feature**: `specs/001-foundations` | **Branch**: `001-foundations`

Test-first is required by constitution Principle III, so each user story writes its
tests before its implementation. In Rust, a test that references a not-yet-existing
API fails by not compiling; that is the red state, resolved when the implementation
lands.

Paths are repository-relative. `[P]` marks tasks that touch different files and have
no incomplete dependency, so they may run in parallel.

## Phase 1: Setup

- [x] T001 Create `Cargo.toml` as a single package (name `eso-weave`, version 0.1.0, edition 2021, a `[lib]` target and a `[[bin]]` target) with dependencies serde, serde_json, tracing, tracing-subscriber (with the `reload` feature), dirs, time (with formatting), thiserror, and dev-dependency tempfile.
- [x] T002 Create `rust-toolchain.toml` pinning channel 1.96.0 with components rustfmt and clippy. Record a dated Decisions entry in `CHANGELOG.md` for this pinned artifact.
- [x] T003 Create `src/lib.rs` declaring the `config`, `logging`, and `platform` modules and re-exporting their public items, plus `src/main.rs` as a thin binary that will call into the library.
- [x] T004 [P] Create compiling stub modules: `src/config/mod.rs`, `src/logging/mod.rs`, `src/platform/mod.rs`, `src/platform/windows.rs`, `src/platform/linux.rs`, each with the minimal items needed to compile with no warnings.

## Phase 2: Foundational (blocking prerequisites)

- [x] T005 Implement platform path resolution in `src/platform/mod.rs` (cfg-selecting `windows`/`linux`) exposing `config_dir()` and `log_dir()`: config via `dirs::config_dir()/eso-weave`, logs via `dirs::state_dir()/eso-weave/logs` on Linux and `dirs::config_dir()/eso-weave/logs` on Windows. The Windows/Linux log fork lives in `src/platform/windows.rs` and `src/platform/linux.rs`.

## Phase 3: User Story 1 - Settings persist and survive corruption (P1)

**Goal**: Durable, resilient user settings (FR-004 through FR-010, FR-016).

**Independent test**: Round-trip a setting across save/load; corrupt the file and
confirm defaults, `.invalid` preservation, and a notice; migrate an older file and
confirm recognized settings survive with warnings.

- [x] T006 [P] [US1] Write failing integration tests in `tests/config.rs` covering: save then load round-trip equality; on-disk file has no byte order mark, LF endings, and a trailing newline; corrupt file yields defaults plus a corrupt_config notice and creates `config.json.invalid` (and `.invalid.2` on collision); older schema_version yields a migrated notice with recognized settings intact; unknown top-level key yields an unknown_keys notice and is not fatal; unwritable directory yields defaults plus an unwritable notice. Tests target a `tempfile` directory.
- [x] T007 [US1] Implement `Settings`, `LoggingPrefs`, `LevelName`, and `Notice` types with serde derives and defaults in `src/config/mod.rs` per `data-model.md` and `contracts/config-schema.md`.
- [x] T008 [US1] Implement `load(config_dir) -> LoadOutcome` in `src/config/mod.rs`: read file, parse, capture unknown top-level keys (flattened map) and warn, apply stepwise forward migration, and on parse failure preserve the file with a `.invalid` (numeric-discriminated) name and fall back to defaults; on read failure fall back to defaults with an unwritable notice. Never panics (FR-009, FR-010).
- [x] T009 [US1] Implement `save(config_dir, settings) -> Result<(), ConfigError>` in `src/config/mod.rs`: create the directory if needed, serialize pretty JSON, and write bytes with no BOM, LF endings, and a trailing newline (FR-005, FR-006). Run `tests/config.rs` to green.

## Phase 4: User Story 2 - Controllable, private diagnostic logging (P2)

**Goal**: Runtime-adjustable, privacy-preserving logging with a ring buffer and an
optional monthly file (FR-011 through FR-015, FR-017, FR-018).

**Independent test**: Change level at runtime and confirm capture changes; read
recent events with the file sink off; enable the file sink and confirm a well-formed
`YYYY-MM.log` line; confirm input contents are absent at info level.

- [x] T010 [P] [US2] Write failing integration tests in `tests/logging.rs` covering: a DEBUG event is absent at INFO and present after raising the level; `recent()` returns events with the file sink disabled and evicts oldest past capacity; enabling the file sink writes a `YYYY-MM.log` line with a UTC RFC-3339 timestamp, level, target, and message to a `tempfile` log dir; an info-level event modeling input activity does not contain the sentinel input content. Tests use a scoped subscriber (`tracing::subscriber::with_default`) to avoid the global-default-once limitation.
- [x] T011 [US2] Implement `LogEvent` and the bounded ring-buffer layer in `src/logging/mod.rs`: a shared `VecDeque` of capacity 1000 that evicts oldest first (FR-012), formatting timestamps with `time` in RFC-3339 UTC (FR-014).
- [x] T012 [US2] Implement `build(prefs, log_dir) -> (Subscriber, LogHandle)` and `init(prefs, log_dir) -> LogHandle` in `src/logging/mod.rs`: compose a reloadable level filter, the ring-buffer layer, and the optional month-file layer; `init` sets the global default, while `build` is used by tests under `with_default`. Expose `LogHandle::set_level` (FR-011).
- [x] T013 [US2] Implement the optional month-file layer and `LogHandle::set_file_enabled` in `src/logging/mod.rs`: write `YYYY-MM.log` under `log_dir`, reopening on month change, UTF-8 no BOM, LF, appended (FR-013, log-format contract).
- [x] T014 [US2] Implement `LogHandle::recent(limit)`, `LogHandle::current_prefs()`, and `LogHandle::set_input_suppressed()` in `src/logging/mod.rs`, and enforce the privacy guarantee that input contents never appear above debug (FR-015, FR-018). Run `tests/logging.rs` to green.

## Phase 5: User Story 3 - Buildable, CI-clean project skeleton (P3)

**Goal**: The crate builds, the quality gate passes, and version is single-sourced
(FR-001, FR-002, FR-003).

**Independent test**: The quality gate passes on a clean checkout; the reported
version comes from one source.

- [x] T015 [US3] Implement `src/main.rs`: resolve platform dirs, load settings, initialize logging from the loaded preferences, emit the loaded notices as warn events and a startup INFO event, then exit 0.
- [x] T016 [US3] Add a `version()` accessor in `src/lib.rs` returning `env!("CARGO_PKG_VERSION")` so the version is single-sourced from `Cargo.toml` (FR-001), and confirm the `platform` module seam compiles on both targets (FR-003).

## Phase 6: Polish and cross-cutting

- [x] T017 [P] Add crate-level and module-level documentation comments across `src/lib.rs`, `src/config/mod.rs`, and `src/logging/mod.rs`.
- [x] T018 Update `CHANGELOG.md` `[Unreleased]`: an Added line for the foundations feature and a dated Decisions entry for the toolchain pin and dependency choices.
- [x] T019 Run CI parity in the foreground and resolve any finding: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`.

## Dependencies and order

- Setup (T001 to T004) precedes everything.
- Foundational T005 precedes US1 and US2 (both need platform paths).
- US1 (T006 to T009) and US2 (T010 to T014) are independent of each other and may
  proceed in parallel once T005 is done.
- US3 (T015 to T016) depends on US1 and US2 (main wires both).
- Polish (T017 to T019) is last; T019 is the CI parity gate before commit.

## Parallel opportunities

- T004 can run alongside T002 and T003 setup work.
- After T005, the US1 test task T006 and the US2 test task T010 can be written in
  parallel; within each story the implementation tasks are mostly sequential because
  they share one module file.

## MVP scope

User Story 1 (settings persistence with corruption resilience) plus the Setup and
Foundational phases is the minimum viable increment: it delivers durable, safe
settings that every later slice depends on. Logging (US2) is the next increment.
