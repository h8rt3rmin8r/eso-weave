# Implementation Plan: ESO API Version Check Automation

**Branch**: `018-eso-api-version-check` | **Date**: 2026-07-12 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/018-eso-api-version-check/spec.md`

## Summary

Keep the PixelBeacon manifest API version current without manual patch-day edits.
On startup, off the GUI thread, a worker fetches the live ESO game version string
from the official esoui/esoui GitHub live branch as a bump-detection signal, then
ensures the on-disk manifest declares the best known numeric API version. The
numeric value resolves as the maximum of the stored last known value and a
compiled default; the network fetch never supplies the number, only detects that a
newer client shipped so the app can advise updating. Every manifest write is gated
by the managed marker, never downgrades, and preserves all other manifest lines.
The check never blocks startup and never panics on network or parse failure.

## Technical Context

**Language/Version**: Rust 1.96 (edition 2021), single crate.

**Primary Dependencies**: existing `serde`/`serde_json`, `tracing`, `eframe`
(egui 0.35). New: `ureq` 2.x with rustls TLS for a blocking HTTPS GET. No async
runtime is introduced; concurrency stays `std::thread` + `mpsc`.

**Storage**: JSON files in the app config dir. Derived runtime state
(`state.json`, `SessionState`) gains the last known numeric API version and last
seen game version as an additive `serde(default)` section. User settings
(`config.json`) are untouched, per the settings-only rule.

**Testing**: `cargo test` with pure-function unit tests and a mock
`GameVersionSource`; filesystem behavior exercised against a `tempfile` AddOns
root, following the existing `tests/beacon.rs` pattern.

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop.

**Project Type**: Single-crate desktop application with per-OS modules.

**Performance Goals**: The check is off the GUI thread; the window must appear and
respond with no wait on the network regardless of latency.

**Constraints**: Never block startup; never panic on network or parse failure;
never write a manifest lacking the managed marker; never downgrade the on-disk API
version; introduce no async runtime.

**Scale/Scope**: One background check per startup; one manifest file; a handful of
new pure functions and one new module.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. This slice ran specify, clarify, checklist,
  and now plan; tasks, analyze, and implement follow before code lands.
- **II. Safety-Critical Surfaces**: PASS and EXTENDED. The managed-marker gate now
  governs manifest edits as well as uninstall; this is added to the tested safety
  surface (`beacon::has_managed_marker` reused). No blocking work is added to the
  input hook thread; the new work runs on its own `std::thread`. AddOns writes stay
  confined to the `PixelBeacon` subfolder.
- **III. Test-First With Explicit Seams**: PASS. Networking sits behind a
  `GameVersionSource` trait seam with a mock; all parsing, rendering, rewriting,
  and resolution logic is pure and unit-tested.
- **IV. CI Parity Before Every Commit**: PASS. fmt, clippy (`-D warnings`), and
  `cargo test --all --locked` run in the foreground before commit.
- **V. Bounded Scope: Outside The Game**: PASS. The feature makes one outbound
  HTTPS GET to a public GitHub endpoint. It performs no game-memory access, no
  network interception, and no in-game action; it only reads a public version
  string and edits a local file ESO Weave owns.

**Pinned artifacts**: `Cargo.toml` is not a pinned artifact. Adding a networked
dependency is architecture-affecting and is recorded as a dated `CHANGELOG.md`
`### Decisions` entry naming `ureq` and the GitHub source. No pinned artifact is
modified.

## Project Structure

### Documentation (this feature)

```text
specs/018-eso-api-version-check/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── api-check.md     # Phase 1 output: internal seam and manifest contract
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
src/
├── beacon/
│   ├── mod.rs           # + DEFAULT_API_VERSION, DEFAULT_GAME_VERSION constants;
│   │                    #   pure render_manifest, rewrite_api_version, token rules
│   └── api_check.rs     # NEW: GameVersionSource trait, GithubLiveSource (ureq),
│                        #   GameVersion type, parse_commit_message_version, run_check
├── config/
│   └── state.rs         # + additive api_version section on SessionState
└── main.rs              # + startup std::thread running the check

tests/
└── beacon.rs            # updated: assert rendered manifest; new marker-gate and
                         # token-rule and resolution-order tests
addon/PixelBeacon/PixelBeacon.txt   # unchanged bytes; now the render template source
Cargo.toml               # + ureq dependency
CHANGELOG.md             # + Added line and dated Decisions entry
```

**Structure Decision**: Single-crate layout, matching the existing beacon module
and its platform backends. Pure logic lives in `beacon`; the networked source is a
sibling module behind a trait; startup wiring lives in `main.rs` alongside the
existing worker threads.

## Complexity Tracking

No constitution violations. No entries required.
