# Implementation Plan: Responsive Live HUD Dashboard

**Branch**: `codex/046-live-hud-dashboard` | **Date**: 2026-09-04 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification for S046, closing issues #28 and #29

## Summary

Replace the unheaded pre-Skills status grid with two task-oriented sections.
Live HUD owns game-derived observations and a shared resource-meter component.
System and automation owns installation, lifecycle, signal readiness, requested
automation, effective blockers, and context-appropriate addon actions. The
sections render side by side when enough point width exists and stack in the
same reading order when narrow. Skills remains functionally unchanged.

## Technical Context

**Language/Version**: Rust 2021, MSRV 1.96
**Primary Dependencies**: eframe/egui 0.36, egui_kittest 0.36.1
**Storage**: Existing settings and session JSON only, no schema change
**Testing**: Rust unit, integration, and headless rendered-frame tests
**Target Platform**: Windows 10/11 x64 and Linux x64
**Project Type**: Single desktop crate plus embedded ESO addon
**Performance Goals**: No new polling, allocation bounded to one frame, no animation
**Constraints**: Truthful dormant states, accessibility metadata, no Skills redesign,
no live-log overlap, no input or PixelBeacon protocol changes
**Scale/Scope**: One main-window region, one reusable meter, two GitHub issues

## Constitution Check

*GATE: Must pass before research and again after design.*

- **I. Spec-Driven Development**: PASS. S046 has specify, checklist, plan,
  research, model, contract, quickstart, tasks, and analysis artifacts before code.
- **II. Safety-Critical Surfaces**: PASS. Input synthesis, hook scoping, fishing
  safety, addon uninstall validation, and AddOns discovery are unchanged. Their
  full suites remain required.
- **III. Test-First With Explicit Seams**: PASS. Pure state and breakpoint
  projections plus the existing headless egui harness provide red-first seams.
- **IV. CI Parity**: PASS. fmt, clippy, and locked full tests run before commit.
- **V. Bounded Scope**: PASS. Work is desktop presentation only and consumes
  existing public observations.
- **Text hygiene**: PASS. UTF-8 without BOM, LF, and forbidden dash scan required.

No complexity exception is required.

## Project Structure

### Documentation

```text
specs/046-live-hud-dashboard/
|-- spec.md
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- analysis.md
|-- contracts/
|   `-- dashboard-presentation.md
|-- checklists/
|   |-- requirements.md
|   `-- dashboard-safety.md
`-- tasks.md
```

### Source

```text
src/app/mod.rs              # typed display projections and breakpoint helper
src/app/strings.rs          # section and field language
src/app/theme.rs            # semantic resource colors and contrast checks
src/app/widgets.rs          # reusable ResourceMeter renderer
src/app/ui.rs               # two sections and responsive arrangement
tests/app_view_model.rs     # truthful state and resource boundary tests
tests/app_ui_sizing.rs      # real narrow/wide geometry and regression tests
docs/ESO-Weave-Specification.md
docs/plans/plan-016.md
README.md
CHANGELOG.md
```

**Structure Decision**: Keep presentation inside the existing `app` module.
The meter is reusable UI, while state derivation remains in `app::mod`; a new
crate or architectural layer would add no useful seam.

## Delivery Phases

### Phase 0: Research

Resolve task grouping, status semantics, resource accessibility, contrast,
responsive thresholds, action hierarchy, and sizing-test migration in
[research.md](research.md).

### Phase 1: Design and contracts

Define the view entities in [data-model.md](data-model.md), the rendering and
state contract in [contracts/dashboard-presentation.md](contracts/dashboard-presentation.md),
and the verification path in [quickstart.md](quickstart.md).

### Phase 2: Test-first implementation

1. Add failing pure tests for resource and signal projections.
2. Add failing rendered-frame tests for layout mode and stable geometry.
3. Implement view-model entities, strings, palette tokens, and meter widget.
4. Replace only the pre-Skills status render with responsive section helpers.
5. Update sizing assertions to the documented responsive contract.
6. Update architecture and user documentation.

### Phase 3: Verification and publication

Run focused suites, full CI parity, text hygiene, spec-kit analysis, diff audit,
then publish one pull request containing separate `Closes #28` and `Closes #29`
references. Resolve every hosted review and run at most the authorized second
Codex review round.

## Risk Controls

- **Responsive minimum ratchet**: Minimum width is based on intrinsic children,
  never the current window-sized section container; transition tests exercise it.
- **False zero**: Dormant and unavailable states carry no numeric value and
  expose explicit text; observed zero remains a progress value.
- **Color dependence**: Every state has visible text and accessible role/value.
- **Contrast**: Palette tests enforce 4.5:1 text and 3:1 meaningful graphics.
- **Skills regression**: The Skills render block and intent wiring remain intact,
  with existing row and sizing suites required.
- **Uninstall regression**: The same confirmation and model intent remain; only
  action prominence and placement change.

## Completion Gate

Done when issues #28 and #29 close from the merged pull request, the dashboard
contract is fully covered, CI and reviews are green, and #27 remains open only
for its documentation screenshot and epic-level completion decision.
