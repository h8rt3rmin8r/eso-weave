# Implementation Plan: Fishing Reliability and Status Collaboration

**Branch**: `016-fishing-reliability` | **Date**: 2026-07-12 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/016-fishing-reliability/spec.md`

## Summary

Fix the fishing feature so a cast at a fishing hole reliably progresses to a
catch and reel-in, and make the app explain what it is doing and why it stopped.
Four coordinated changes: (1) refresh the embedded PixelBeacon manifest so the
game stops flagging the addon out of date and loads it; (2) poll the beacon and
tick the fishing state machine at the fast fishing cadence while a session is
active instead of always at the idle cadence; (3) evaluate fishing deadlines on
the same monotonic clock they are stamped on; and (4) present plain-language
fishing status with a persisted stop reason. Correctness logic lands in tested
pure helpers and the fishing controller; the egui layer stays thin. All existing
safety-critical behaviors are preserved and stay tested.

## Technical Context

**Language/Version**: Rust (edition and toolchain pinned by `rust-toolchain.toml`)

**Primary Dependencies**: egui/eframe (GUI), existing internal modules (`beacon`,
`fishing`, `pixelbus`, `app`, `input`); no new external crates

**Storage**: user config JSON (unchanged by this feature); addon files written
under the resolved AddOns directory by the beacon manager

**Testing**: `cargo test --all --locked`; unit tests colocated with modules and
integration tests under `tests/` (existing `tests/beacon.rs`)

**Target Platform**: Windows 10 and 11 x64, Linux x64 (desktop app)

**Project Type**: single-crate desktop application

**Performance Goals**: while fishing is active, sample the beacon and advance the
routine at roughly the fishing interval (about 100 ms) so transient cast and bite
signals are observed and the reel action is not delayed by up to a second

**Constraints**: no blocking work on the input hook thread; input suppression
scoped to the focused game window only; addon deletion only after managed-marker
verification; UTF-8 without BOM, LF, no em- or en-dashes

**Scale/Scope**: one feature slice touching `addon/PixelBeacon/PixelBeacon.txt`,
`src/main.rs`, `src/fishing/mod.rs`, `src/app/mod.rs`, `src/app/strings.rs`, and a
small testable cadence helper, plus tests

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- I. Spec-Driven Development: PASS. This slice runs the full spec-kit sequence
  under build plan 004, traces to master specification sections 8 (Fishing), 9
  (pixel bus and blocks), and 10 (GUI), and will pass the `/speckit.analyze` gate
  before implementation.
- II. Safety-Critical Surfaces Are Sacrosanct: PASS. The feature preserves every
  listed surface and keeps them tested: no blocking on the hook thread (all timed
  fishing work stays on the worker), focused-window-only suppression (untouched),
  SignalLost cancels the pending interact rather than firing blind (unchanged
  path, retested), and PixelBeacon uninstall verifies the managed-marker line
  (the marker line is not modified; a beacon test asserts it is still present).
- III. Test-First With Explicit Seams: PASS. New correctness logic (the cadence
  helper, the stop-reason state, and the status derivation) is written test-first
  through pure functions and the existing `FishingSink`/`BiteDetector` seams; no
  live game is required to verify it.
- IV. CI Parity Before Every Commit: PASS. The full gate (`cargo fmt`, `cargo
  clippy -D warnings`, `cargo test --all --locked`) runs in the foreground before
  the commit.
- V. Bounded Scope: Outside The Game: PASS. No process memory, no packet capture,
  no new in-game functionality beyond the existing PixelBeacon screen-signal
  contract; the manifest change only updates version metadata.

Text hygiene and pinned-artifact discipline: the addon manifest is not in the
pinned-artifacts list, but the API version change is a screen-signal-contract
adjacent change and closes open item R4, so it is recorded as a dated decision in
`CHANGELOG.md`, consistent with how slice 014 handled manifest changes.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/016-fishing-reliability/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (fishing status and cadence contracts)
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
addon/PixelBeacon/PixelBeacon.txt   # manifest: bump Version/AddOnVersion, APIVersion
src/
├── main.rs             # pixel-bus worker loop: adaptive sleep + shared clock
├── fishing/
│   └── mod.rs          # FishingConfig defaults; stop-reason state on the controller
├── pixelbus/
│   └── mod.rs          # poll_interval() pure helper (cadence selection)
├── app/
│   ├── mod.rs          # fishing_label()/status_line_fishing(): plain-language + reason
│   └── strings.rs      # new fishing status/label strings and tooltips
└── beacon/
    └── mod.rs          # embeds the manifest (no code change; version parsed at runtime)

tests/
└── beacon.rs           # add manifest APIVersion + managed-marker assertions
```

**Structure Decision**: Single crate, unchanged. Correctness logic is added as
pure functions in `pixelbus` (cadence) and as tested state and derivation in
`fishing` and `app`; `main.rs` wiring stays thin and the egui layer only renders
derived view-model values.

## Complexity Tracking

No constitution violations; no entries.
