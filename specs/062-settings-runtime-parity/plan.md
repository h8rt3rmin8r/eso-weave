# Implementation Plan: Settings Runtime Parity

**Branch**: `codex/s062-settings-runtime-parity` | **Date**: 2026-09-07 | **Spec**: `specs/062-settings-runtime-parity/spec.md`

## Summary

Close #95 by live-applying safe Fishing and scalar Pixel Bus settings, exposing the persisted Fishing interact key, retaining staged block geometry, and aligning interface and canonical documentation with the per-setting runtime contract.

## Technical Context

- **Language/Version**: Rust 2021, MSRV 1.96
- **Primary Dependencies**: Rust standard-library `mpsc`, eframe/egui, existing controller and Pixel Bus modules
- **Storage**: Existing JSON settings, no schema change
- **Testing**: Rust unit/integration tests, headless egui tests, Node documentation policy, mdBook
- **Target Platform**: Windows and Linux desktop
- **Performance Goals**: Never block the GUI on reader updates; coalesce rapid edits; keep worker locks outside sampling and subsystem calls
- **Constraints**: No new dependency or thread; no input during Fishing reconfiguration; block geometry remains startup-fixed; UTF-8 without BOM; LF endings
- **Scale/Scope**: One issue, three user stories, two runtime subsystems, one settings modal, canonical documentation

## Constitution Check

- PASS: Spec-kit artifacts and analysis precede source changes.
- PASS: Generated input remains fail-closed across configuration generations.
- PASS: Reader updates remain off the input-hook thread and do not block the GUI.
- PASS: Running geometry stays synchronized with the add-on contract.
- PASS: User-facing behavior and lifecycle evidence are documented and testable.

## Project Structure

```text
src/fishing/mod.rs              atomic safe controller reconfiguration
src/pixelbus/mod.rs             live reader subset and reader reconfiguration
src/app/settings_form.rs        effective persisted configuration
src/app/mod.rs                  runtime application and update publication
src/app/ui.rs                   Fishing key control and timing copy
src/app/strings.rs              stable labels and help
src/main.rs                     wakeable coalescing reader-worker updates
tests/                          controller, model, UI, strings, and policy regressions
docs/src/                       canonical settings and safety behavior
docs/project/                   coverage and plan lifecycle
specs/062-settings-runtime-parity/
```

## Implementation Strategy

1. Write failing Fishing reconfiguration, AppModel parity, reader-update, UI, and semantic-copy tests.
2. Add safe Fishing configuration replacement that cancels changed requested work without emitting input and is a no-op for identical values.
3. Add the Fishing Interact Key control and ensure the form persists the same sanitized effective configuration applied at runtime.
4. Add a complete-value reader update channel, wakeable worker wait, latest-value coalescing, reader/poll parity, and tolerance safety invalidation.
5. Preserve startup block geometry and heartbeat timeout while clarifying staged geometry copy and diagnostics.
6. Update canonical documentation, coverage, plan lifecycle, and changelog.
7. Complete post-implementation analysis, independent reviews, and full validation before delivery.

## Complexity Tracking

| Deviation | Why needed | Simpler alternative rejected |
| --- | --- | --- |
| Per-setting hybrid application contract | Block geometry crosses process boundaries, while scalar values have safe live ownership | Blanket live apply risks geometry mismatch; blanket restart preserves the #95 stale-state defect |
| Wakeable latest-value channel | Settings auto-apply can issue rapid edits while the worker is sleeping | Shared immutable startup state never updates; blocking or bounded sends can stall or lose the newest UI edit |
| Cancel changed active Fishing work | Old deadlines and newly selected keys are different configuration generations | Mutating in place could emit a new key from old scheduled intent |
