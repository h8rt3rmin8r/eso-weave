# Implementation Plan: Dashboard Layout Cohesion

**Branch**: `codex/s054-dashboard-layout-cohesion` | **Date**: 2026-09-07 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification for S054, closing issues #72 through #75

## Summary

Unify the responsive dashboard around one collapse-aware arrangement, equal
expanded-card geometry, shared row allocation, and a fixed System and State
interaction column. Let Live HUD values consume available width, separate the
resource and fact groups, and migrate the bounded field-label inventory to
concise title case without adding Ultimate telemetry.

## Technical Context

**Language/Version**: Rust 2021, MSRV 1.96
**Primary Dependencies**: eframe/egui 0.36, egui_kittest 0.36.1
**Storage**: Existing settings JSON only, no schema change
**Testing**: Rust unit, integration, accessibility, and headless rendered-frame tests
**Target Platform**: Windows 10/11 x64 and Linux x64
**Project Type**: Single desktop crate plus embedded ESO addon
**Performance Goals**: No extra frame pass, polling, or unbounded allocation
**Constraints**: No Ultimate protocol, Skills, input, or lifecycle-safety redesign
**Scale/Scope**: One dashboard region, four independently closeable UI issues

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **I. Spec-Driven Development**: PASS. Specify, checklist, research, data model,
  contract, quickstart, plan, tasks, and analysis precede implementation.
- **II. Safety-Critical Surfaces**: PASS. Input, protocol, uninstall guard, and
  confirmation behavior are unchanged and their full suites remain required.
- **III. Test-First With Explicit Seams**: PASS. Pure layout selection, a shared
  row allocator, geometry receipts, and egui_kittest provide red-first seams.
- **IV. CI Parity**: PASS. Formatting, clippy, and all locked tests are required.
- **V. Bounded Scope**: PASS. Four issues alter the same dashboard composition;
  Ultimate telemetry and release work remain excluded.
- **Text hygiene**: PASS. UTF-8 without BOM, LF, and forbidden-text scans required.

No complexity exception is required.

## Project Structure

### Documentation

```text
specs/054-dashboard-layout-cohesion/
|-- spec.md
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- analysis.md
|-- contracts/
|   `-- dashboard-layout.md
|-- checklists/
|   |-- requirements.md
|   `-- layout-safety.md
`-- tasks.md
```

### Source

```text
src/app/mod.rs              # collapse-aware layout projection
src/app/strings.rs          # exact labels and governed label registry
src/app/ui.rs               # shared cards, rows, controls, spacing, receipts
tests/app_view_model.rs     # pure layout and label projections
tests/app_ui_sizing.rs      # rendered geometry, transitions, and access
tests/app_strings.rs        # explicit label policy audit
docs/ESO-Weave-Specification.md
docs/plans/plan-024.md
docs/plans/README.md
README.md
CHANGELOG.md
```

**Structure Decision**: Keep composition in the existing app module. Extract
small presentation helpers only where they create direct pure or rendered test
seams. A new crate or generalized layout framework is unnecessary.

## Delivery Phases

### Phase 0: Specification and design

Bind issues #72 through #75 to S054, resolve routine clarifications through the
autopilot policy, complete all spec-kit artifacts, and pass the blocking analysis.

### Phase 1: Red tests

Add failing pure and rendered checks for collapse-aware layout, equal card
geometry, symmetric resizing, state transitions, control alignment and size,
flexible values, keyboard detail, resource grouping, exact labels, and existing
Skills and log boundaries.

### Phase 2: Geometry and row implementation

Replace the capped asymmetric split with equal allocation. Coordinate both card
bodies around Live HUD's expanded height, force collapsed stacking, and integrate
state transitions with content measurement. Replace rigid dashboard Grids with
shared rows and one maximum-sized trailing control column.

### Phase 3: Copy and information access

Apply the explicit label migrations, add the post-resource boundary gap, let
Live HUD values consume the flexible region, and make constrained full text
available to pointer and keyboard users.

### Phase 4: Documentation and validation

Update master specification, README, changelog, and build plan. Run focused and
full gates, post-implementation analysis, layout checklist, text and encoding
audits, and a scoped diff review.

### Phase 5: Publication and hosted review

Commit as `feat(054): unify responsive dashboard layout`, push the branch, and
publish one pull request with separate closing references for #72 through #75.
Resolve CI and every first-round review, request exactly one authorized second
Codex review, resolve it, then stop for the maintainer's merge ritual.

## Risk Controls

- **Height mismatch**: Live HUD is the explicit expanded authority and rendered
  tests fail if System and State exceeds it or either exterior differs.
- **Transition overlap or ratchet**: Collapse has a measured transition path;
  open-log and repeated-toggle tests cover every rendered frame.
- **Control drift**: One fixed maximum interaction region and common leading
  origin are asserted across addon states.
- **Hidden information**: Flexible allocations and focusable detail preserve
  full text without multi-line dashboard growth.
- **Future meter coupling**: Spacing belongs to a count-agnostic group boundary;
  a synthetic fourth item tests the seam without implementing #71.
- **Copy damage**: A bounded registry prevents title casing prose or statuses.

## Completion Gate

Done when #72 through #75 are linked for closure, every contract has automated
evidence, local and hosted CI are green, two permitted review rounds are fully
resolved, and the pull request awaits only maintainer review and merge.
