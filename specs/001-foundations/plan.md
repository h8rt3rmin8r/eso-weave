# Implementation Plan: Foundations (Project Bootstrap, Config Store, Logging)

**Branch**: `001-foundations` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/001-foundations/spec.md`

## Summary

Stand up the single Rust crate that all later slices build on, then implement the
two foundational subsystems it must carry: a Config Store that persists
user-settings-only JSON with corruption resilience and forward migration, and a
Logging subsystem with runtime-selectable level, an always-on in-memory ring
buffer, and an optional monthly file sink under a strict privacy rule. The crate
is a single package exposing a library (for testability) plus a thin binary. A
per-platform module seam is established now so the Input Engine and sampling
backends in later slices slot in without restructuring.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021, pinned via `rust-toolchain.toml`
(the active stable on this machine, so CI parity runs against the same toolchain).

**Primary Dependencies**: `serde` and `serde_json` (settings load/save), `tracing`
and `tracing-subscriber` with the `reload` layer (structured logging and runtime
level change), `dirs` (platform config and state directories that match the
specification paths exactly), `time` (UTC ISO-8601 timestamps), `thiserror`
(typed errors). Dev dependency: `tempfile` (filesystem tests in isolation).

**Storage**: Local filesystem. One JSON settings file at the platform config
directory under an `eso-weave` folder; monthly `YYYY-MM.log` files at the platform
log directory.

**Testing**: `cargo test` with unit tests in-module and integration tests under
`tests/`. Filesystem behavior is exercised against temporary directories.

**Target Platform**: Windows 10 and 11 x64, Linux x64.

**Project Type**: Single desktop-application crate (library target plus a thin
binary target in one package).

**Performance Goals**: Not latency-critical in this slice. Config load and save
and log-event handling are lightweight and off any future hot path.

**Constraints**: Settings file holds user settings only (no runtime or session
state). All text output is UTF-8 without BOM with LF endings. Logging never emits
input contents above debug and exposes a suppression control.

**Scale/Scope**: Single-user desktop. Small settings document; ring buffer default
capacity 1000 events.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. This plan derives from `spec.md`, which
  traces to master specification sections 5, 11, 12, and 14. Slice bounded by
  `docs/plans/plan-001.md`.
- **II. Safety-Critical Surfaces**: PASS (scoped). None of the enumerated input or
  beacon surfaces exist in this slice. The one safety-adjacent guarantee present,
  the logging privacy rule (FR-015), is covered by a required test.
- **III. Test-First With Explicit Seams**: PASS. Config Store and Logging are
  pure-Rust modules testable without the game or devices; the platform path
  differences sit behind a `platform` module seam. Tests are written before
  implementation per the tasks ordering.
- **IV. CI Parity Before Every Commit**: PASS. `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`, and
  `cargo test --all --locked` run in the foreground before the commit.
- **V. Bounded Scope: Outside The Game**: PASS. No process memory, network, or
  in-game functionality. No PixelBeacon dependency.
- **Platform, Configuration, and Text Hygiene Constraints**: PASS. Config is
  settings-only JSON, UTF-8 without BOM, LF, pretty-printed, with schema_version
  and forward migration; single crate with a per-platform module seam.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/001-foundations/
├── plan.md              # This file
├── research.md          # Phase 0 output (decisions and rationale)
├── data-model.md        # Phase 1 output (entities and validation)
├── quickstart.md        # Phase 1 output (build and validation guide)
├── contracts/           # Phase 1 output (public API, config schema, log format)
│   ├── public-api.md
│   ├── config-schema.md
│   └── log-format.md
├── checklists/
│   ├── requirements.md  # spec quality (from /speckit.specify)
│   └── foundations.md   # requirements quality (from /speckit.checklist)
├── spec.md
└── tasks.md             # Phase 2 output (/speckit.tasks, not this command)
```

### Source Code (repository root)

```text
Cargo.toml               # single package: [lib] + [[bin]]; version single-sourced
rust-toolchain.toml      # pin 1.96.0 + rustfmt, clippy
src/
├── main.rs              # thin binary: init logging, load settings, log startup, exit 0
├── lib.rs               # library root; re-exports config, logging, platform
├── platform/
│   ├── mod.rs           # cfg-selected backend; exposes config_dir(), log_dir()
│   ├── windows.rs       # #[cfg(windows)] path specifics (logs under %APPDATA%)
│   └── linux.rs         # #[cfg(unix)] path specifics (XDG config and state)
├── config/
│   └── mod.rs           # Settings, defaults, schema_version, load/save/migrate
└── logging/
    └── mod.rs           # init, reload level handle, ring buffer layer, month file sink
tests/
├── config.rs           # persistence, corruption fallback, migration, encoding
└── logging.rs          # runtime level, ring buffer independence, privacy, file sink
```

**Structure Decision**: A single package with a library target (`src/lib.rs`) and
a thin binary (`src/main.rs`). The library carries all logic so it is unit and
integration testable without launching the binary, satisfying the test-first seam
principle. The `platform` module is the concrete per-platform seam that later
input and sampling backends extend, satisfying FR-003 without restructuring.

## Complexity Tracking

No constitution violations. No entries.
