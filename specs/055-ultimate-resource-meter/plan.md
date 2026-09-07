# Implementation Plan: Ultimate Resource Meter

**Branch**: `codex/s055-ultimate-resource-meter` | **Date**: 2026-09-07 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification for S055, closing issue #71

## Summary

Advance PixelBeacon and its reader to protocol v5 with exact, independently
validated Ultimate charge and front/back costs. Carry one display-only aggregate
through the model, select the active cost from the typed weapon bar, and render a
purple fourth meter with shared quarter marks, a protruding threshold, exact
numbers, and a fixed green Ready slot.

## Technical Context

**Language/Version**: Rust 2021 (MSRV 1.96), ESO Lua
**Primary Dependencies**: eframe/egui 0.36, egui_kittest 0.36.1
**Storage**: No persisted schema change
**Testing**: Rust unit, integration, source-contract, accessibility, and rendered-frame tests
**Target Platform**: Windows 10/11 x64, Linux x64, ESO addon runtime
**Performance Goals**: One atomic Ultimate event per change, no new UI frame pass
**Constraints**: Display only, no automation behavior, no skill-ID catalogue
**Scale/Scope**: Four payload blocks, one aggregate, one fourth meter, one issue

## Constitution Check

*GATE: Passed before research and re-checked after design.*

- **I. Spec-Driven Development**: PASS. The complete S055 spec-kit packet and
  chronological build plan precede tests and implementation.
- **II. Safety-Critical Surfaces**: PASS. Ultimate is structurally excluded from
  auto-potion and synthesis; frozen protocol layouts and loss clearing are tested.
- **III. Test-First With Explicit Seams**: PASS. Codec, projection, and pure meter
  geometry receive red tests before source changes.
- **IV. CI Parity**: PASS. Formatting, clippy with warnings denied, and all locked
  tests are mandatory before publication.
- **V. Bounded Scope**: PASS. One end-to-end observable and its UI are included;
  live release verification remains a separate governed issue.
- **Text hygiene**: PASS. UTF-8 without BOM, LF, and forbidden-character scans apply.

No complexity exception is required. Four 9-bit blocks preserve exact values,
field identity, checksum validation, and the one-row maximum geometry invariant.

## Project Structure

### Documentation

```text
specs/055-ultimate-resource-meter/
|-- spec.md
|-- plan.md
|-- research.md
|-- data-model.md
|-- quickstart.md
|-- analysis.md
|-- contracts/
|   `-- ultimate-telemetry.md
|-- checklists/
|   |-- requirements.md
|   `-- telemetry-safety.md
`-- tasks.md
docs/plans/plan-025.md
website/content/blog/ultimate-resource-meter.md
```

### Source

```text
addon/PixelBeacon/PixelBeacon.lua
addon/PixelBeacon/PixelBeacon.txt
src/pixelbus/mod.rs
src/weave/mod.rs
src/app/routing.rs
src/app/mod.rs
src/app/theme.rs
src/app/strings.rs
src/app/widgets.rs
src/app/ui.rs
tests/pixelbus.rs
tests/beacon.rs
tests/weave_engine.rs
tests/app_view_model.rs
tests/app_ui_sizing.rs
```

**Structure Decision**: Extend the existing addon, protocol, store, and app seams.
Do not broaden `ResourceSet`; its three fields are automation inputs. Introduce
one Ultimate-specific aggregate and one reusable meter geometry descriptor.

## Delivery Phases

### Phase 0: Specification and design

Bind #71 to S055, record the raw-value deviation, complete all spec-kit artifacts,
and pass blocking analysis before source work.

### Phase 1: Red protocol and model tests

Add failing tests for v5 negotiation, frozen v1-v4 extents, exact two-byte codecs,
partial corruption, atomic events, loss/recovery, routing, and automation inertness.

### Phase 2: Protocol and lifecycle implementation

Publish current, maximum, front cost, and back cost as B25 through B28. Refresh
from immediate events, rebaseline, and periodic backstop. Decode and route one
typed atomic display-only observation.

### Phase 3: Red presentation tests

Add failing projection, theme, pure geometry, rendered layout, readiness, active
bar, accessibility, and resize-stability tests.

### Phase 4: Presentation implementation

Project the cached cost for the active typed weapon bar. Add purple theme colors,
quarter marks on every meter, the Ultimate threshold, exact readout, and fixed
green Ready slot without moving existing geometry.

### Phase 5: Documentation and local validation

Advance the addon manifest, update protocol and user documentation, add the
feature announcement and separate verification issue, then run focused and full
gates plus text, encoding, secret, and scoped-diff audits.

### Phase 6: Publication and hosted review

Commit as `feat(055): add ultimate resource meter`, push, publish one PR closing
#71, resolve first-round CI and all feedback, request exactly one second Codex
review, resolve it, and stop for the maintainer's merge ritual.

## Risk Controls

- **False readiness**: Compare exact raw current and cost, never rounded percentages.
- **Protocol drift**: Freeze v4 at 25 blocks and gate B25-B28 behind v5 support.
- **Partial corruption**: Each scalar has two reserved marker variants and a
  complement checksum; one invalid cell does not erase other valid values.
- **Automation coupling**: Ultimate is not a `ResourceSet` member and routes only
  to the display model; inertness tests cover output and gate equivalence.
- **Visual movement**: Pure geometry reserves threshold depth and Ready width for
  every state before any conditional paint.
- **Overlap ambiguity**: Cost threshold paints after quarter marks with stronger
  contrast and protrusion.
- **Lifecycle staleness**: Activation rebaseline precedes world Active, and signal
  loss clears the entire atomic observation.

## Completion Gate

Done when #71 is linked for closure, the separate live verification issue exists,
every contract has automated evidence, local and hosted CI are green, two review
rounds are fully resolved, and the PR awaits only maintainer review and merge.
