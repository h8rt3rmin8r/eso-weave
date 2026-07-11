# Implementation Plan: Input Engine

**Branch**: `002-input-engine` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/002-input-engine/spec.md`

## Summary

Add an `input` module to the crate: a platform-agnostic `InputEngine` core that
holds the binding table and the focus and suspend state and makes the
safety-critical classification decision (suppress or pass, and whether to hand off
an action) for each key event, plus a non-blocking bounded hand-off channel to a
worker. The OS-specific interception and synthesis live behind an `InputBackend`
seam with three implementations: a `MockBackend` for tests, a Windows backend
(low-level keyboard hook, `SendInput`, injected-input flagging, raised timer
resolution), and a Linux backend (evdev grab, uinput virtual device). All
safety-critical logic sits in the testable core; the OS adapters are thin.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged from S001).

**Primary Dependencies**: Adds `windows-sys` (Windows target only: low-level hook,
`SendInput`, `timeBeginPeriod`) and `evdev` (Linux target only: keyboard grab and
uinput). Reuses S001 crates. Both new dependencies are declared under
target-specific tables so a Windows build never compiles the Linux backend and
vice versa.

**Storage**: Bindings persist through the S001 Config Store as an additive
`bindings` settings section (backward compatible, no schema version bump).

**Testing**: `cargo test`. The `InputEngine` core and `MockBackend` cover every
safety-critical behavior and success criterion without OS hooks. The OS adapters
are thin and validated manually against the running game (documented limitation).

**Target Platform**: Windows 10 and 11 x64, Linux x64.

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: The classification path is O(1) (a table lookup plus atomic
state reads and a non-blocking channel send). It must never block the OS hook
thread.

**Constraints**: Interception path never sleeps or blocks; synthesized input is
flagged so it is never re-intercepted; interception only while the ESO window is
focused. Bindings are settings-only and text-hygienic.

**Scale/Scope**: A handful of actions and bindings; a single worker channel.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec
  section 6), bounded by `docs/plans/plan-001.md`.
- **II. Safety-Critical Surfaces**: PASS and central. Injected-input recursion
  breaking, focused-window-only suppression, and no blocking work on the hook
  thread are implemented in the testable core and covered by required tests
  against the `MockBackend`.
- **III. Test-First With Explicit Seams**: PASS. The `InputBackend` trait is the
  seam; the core is unit-tested with the mock. Tests precede implementation.
- **IV. CI Parity Before Every Commit**: PASS on the host (Windows) target. The
  Linux backend is `cfg`-gated and additionally type-checked with
  `cargo check --target x86_64-unknown-linux-gnu` where the target is available;
  any residual gap is reported at the pre-push halt.
- **V. Bounded Scope: Outside The Game**: PASS. No process memory, network, or
  in-game functionality. The engine only intercepts and synthesizes OS-level keys
  while the game is focused.
- **Platform and Text Hygiene Constraints**: PASS. Additive settings-only
  bindings; per-platform backends behind the existing module seam.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/002-input-engine/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   ├── input-backend.md   # the InputBackend seam and InputEngine core API
│   └── bindings-schema.md  # the additive bindings settings section
├── checklists/{requirements.md, input-engine.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
src/input/
├── mod.rs        # InputEngine core, Decision, KeyEvent, Transition, Origin, hand-off channel, InputBackend trait, InputError
├── key.rs        # platform-neutral Key identifier, name parsing and display
├── action.rs     # Action enum and its default bindings
├── bindings.rs   # Binding, BindingTable (defaults, rebind with conflict rejection, lookup, load/save via config)
├── mock.rs       # MockBackend: feed synthetic events, capture synthesized output
├── windows.rs    # #[cfg(windows)] WH_KEYBOARD_LL + SendInput + timeBeginPeriod adapter
└── linux.rs      # #[cfg(target_os = "linux")] evdev grab + uinput adapter
tests/
└── input_engine.rs  # safety-critical core tests via MockBackend
```

The `config` module gains an additive `bindings` section (a new optional field on
`Settings`, defaulting to the section 6.4 defaults) so existing files load
unchanged.

**Structure Decision**: All safety-critical logic lives in `src/input/mod.rs`,
`key.rs`, `action.rs`, and `bindings.rs`, which are platform-agnostic and fully
tested through `MockBackend`. `windows.rs` and `linux.rs` are thin adapters that
translate OS events into `KeyEvent`s, call `InputEngine::classify`, and act on the
returned `Decision`. This keeps the untestable OS surface minimal and the
verifiable surface maximal, satisfying constitution Principles II and III.

## Complexity Tracking

No constitution violations. No entries.
