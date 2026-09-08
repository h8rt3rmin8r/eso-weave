# Implementation Plan: Linux Input and Copy Parity

**Branch**: `codex/s061-linux-input-copy-parity` | **Date**: 2026-09-07 | **Spec**: `specs/061-linux-input-copy-parity/spec.md`

## Summary

Close #93 and #96 by making Linux virtual-device capabilities exhaustive and physical-key preserving, enforcing fail-closed initial menu evidence across generated-input controllers, and aligning shipped/canonical copy with verified runtime behavior.

## Technical Context

- **Language/Version**: Rust 2021, MSRV 1.96
- **Primary Dependencies**: evdev 0.12 on Linux, eframe/egui, tracing
- **Storage**: Existing JSON settings and in-memory state, no schema change
- **Testing**: Rust unit/integration tests, Linux-target tests, Node documentation policy, mdBook
- **Target Platform**: Windows and Linux desktop; Linux device work under `cfg(target_os = "linux")`
- **Performance Goals**: Preserve constant-time input classification and direct key forwarding
- **Constraints**: No real `/dev/input` or `/dev/uinput` dependency in tests; no new dependencies; UTF-8 without BOM; LF endings
- **Scale/Scope**: Two issues, three user stories, one Linux backend, three safety controllers, UI strings, canonical documentation

## Constitution Check

- PASS: Spec-kit artifacts and analysis precede source changes.
- PASS: Input interception remains non-blocking and structurally recursion-safe.
- PASS: Unknown safety evidence is fail-closed and physical input remains pass-through.
- PASS: Platform-specific behavior has deterministic pure seams and Linux CI coverage.
- PASS: User-facing behavior and decisions are documented with issue/plan traceability.

## Project Structure

```text
src/input/key.rs                 canonical application key universe
src/input/linux.rs               Linux capabilities and forwarding
src/input/mod.rs                 initial input menu gate
src/fishing/mod.rs               initial Fishing menu gate
src/pixelbus/mod.rs              menu evidence transitions and diagnostics
src/app/strings.rs               latency and Live Log copy
src/app/mod.rs                   logging API documentation
tests/                           behavior and semantic regression tests
docs/src/                        canonical behavior documentation
docs/project/                    coverage and plan lifecycle
specs/061-linux-input-copy-parity/
```

## Implementation Strategy

1. Write failing exhaustive key/capability and startup-gate tests.
2. Add `Key::ALL`, pure Linux capability helpers, E inbound mapping, pre-grab capability union/upgrade, key-only forwarding, and explicit key emission errors.
3. Initialize input and Fishing menu gates closed and publish the first valid gameplay menu observation.
4. Write failing semantic copy tests, then correct strings, API comments, diagnostics, tests, and canonical pages.
5. Complete documentation coverage and plan lifecycle records.
6. Run focused and full validation, post-implementation analysis, and independent reviews before delivery.

## Complexity Tracking

| Deviation | Why needed | Simpler alternative rejected |
| --- | --- | --- |
| Correct startup gating while issue #96 requested copy parity | Existing behavior contradicts its claimed fail-closed premise and the safety constitution | Copy-only changes would publish a false guarantee |
| Replace an early app-only virtual device before grab | Synthesis can create it before keyboard discovery | Keeping it would still lose physical-only keys |
