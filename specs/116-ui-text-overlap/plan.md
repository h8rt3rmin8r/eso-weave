# Implementation Plan: Collision-Safe Status Rows

**Branch**: `codex/s116-ui-text-overlap` | **Date**: 2026-09-17 | **Spec**: [spec.md](spec.md)

**Input**: GitHub issue #231 and the S116 feature specification

## Summary

Replace the fixed dashboard label allocation with font-measured shared columns for Live HUD, System and State, and ESO Weave Data Details. Bound those columns against value and action reserves, explicitly clip every row cell, preserve full accessible strings when titles truncate, and add deterministic geometry and painted-text collision tests to the existing Rust CI suite.

## Technical Context

**Language/Version**: Rust 2021, pinned toolchain 1.96.0

**Primary Dependencies**: eframe/egui 0.36, egui_kittest 0.36.1

**Storage**: None

**Testing**: `tests/app_ui_sizing.rs`, semantic AccessKit rectangles, egui painted text shapes, full locked Cargo gate

**Target Platform**: Windows 10/11 x64 and Linux x64 desktop

**Constraints**: No new dependency, screenshot-only gate, pinned workflow edit, behavior change, release action, duplicate issue, em/en dash, BOM, or non-LF text

**Scale/Scope**: Shared dashboard row implementation, UI sizing tests, changelog, and the S116 spec packet

## Constitution Check

*GATE: Passed before implementation and must be re-checked after implementation.*

- **I. Spec-driven development**: PASS. Issue #231 is the sole tracker and this packet supplies specify, clarify, checklist, plan, tasks, and analysis artifacts before implementation.
- **II. Safety-critical surfaces**: PASS. Input, addon deletion, capture authority, and automation behavior are unchanged. Their tests remain mandatory.
- **III. Test first**: PASS. The first code change adds a collision test that fails against the fixed 118-point allocation before production layout changes.
- **IV. CI parity**: PASS. Fmt, clippy, and the complete locked test suite run in the foreground before commit.
- **V. Bounded scope**: PASS. This is a desktop presentation correction with no new addon, transport, input, automation, or persistence surface.
- **Accessibility**: PASS. Full semantic title/value strings and tooltips remain available when visible text truncates.
- **Text hygiene**: PASS. All touched text remains UTF-8 without BOM, LF-only, and free of forbidden dash characters.
- **Project tracking**: PASS. S116 intentionally interrupts the stale documentation numbering because #231 is a shipped P1 regression. S115 remains reserved and no duplicate tracker or repository project plan is created.
- **Publication authority**: PASS. The current kickoff authorizes local autopilot work but not the first remote push. The protocol halts once after local verification.

No complexity exception is required.

## Project Structure

```text
specs/116-ui-text-overlap/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── analysis.md
├── contracts/text-collision.md
└── checklists/
    ├── requirements.md
    └── collision-safety.md

src/app/ui.rs
tests/app_ui_sizing.rs
CHANGELOG.md
```

## Design Decisions

- **D1, shared measured columns**: Measure the longest title separately for Live HUD, System and State, and Data Details using the exact title font. This retains alignment without wasting the half-width Live HUD card on modal-only labels.
- **D2, bounded responsive width**: Cap the measured column against the available width after preserving gaps, the largest group interaction reserve, and a minimum value cell. Titles yield through truncation before status or actions disappear.
- **D3, clip as invariant**: Set each child UI clip to its exact cell. Truncation is the intended visual behavior, while clipping prevents any future widget from painting into a neighbor.
- **D4, scoped collision oracle**: Compare semantic and painted text within an explicit row/container. Do not globally compare flattened output because modal background paint is intentionally present behind the modal.
- **D5, existing CI path**: Put the helper and fixtures in the Rust UI test suite so the existing required Cargo command enforces the gate without a pinned workflow edit.

## Phases

### Phase 1: Red tests

Add the row geometry fields, title inventory fixtures, and a component collision oracle. Record the expected failure for long titles on the current implementation.

### Phase 2: Layout implementation

Add exact-font measurement, group width bounding, row clipping, title truncation, and full title tooltips. Pass measured widths into every affected surface.

### Phase 3: Surface matrix

Exercise both themes, required widths, addon states, the modal, and enlarged logical text. Preserve current actions and accessibility behavior.

### Phase 4: Verification and delivery

Update the changelog, run focused and complete gates, complete review, commit locally, then stop at the required pre-push authorization boundary.
