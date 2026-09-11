# Feature Specification: Deterministic Screenshot Sandbox

**Feature Branch**: `codex/s083-deterministic-screenshot-sandbox`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Work slice S083 implements issue #124 with an isolated, deterministic documentation-capture harness.

## Clarifications

### Session 2026-09-11

- The sandbox is a Cargo test target with `harness = false`, not a production feature, command-line option, or application mode.
- Running the target without a destination validates the catalog and exits without initializing a renderer. An explicit repository-local output directory arms PNG generation.
- The scene catalog owns seven named application states: first launch, healthy System and State, lost PixelBeacon, unmanaged addon, weaving configuration, Auto Potion ready, and Auto Potion blocked.
- Every named scene supports dark and light themes plus narrow and wide documentation viewports. Capture output is generated evidence and is not checked in by this slice.
- Fixtures use only in-memory model state, a no-op fishing sink, an unconsumed input-engine channel, repository data, and an isolated synthetic root. Generation keeps that root below the caller-supplied output directory; validation uses an automatically removed temporary root.
- Final image selection, cropping, optimization, placement, alternatives, and provenance in the manual remain issue #125.

## User Scenarios & Testing

### User Story 1 - Reproduce representative application states (Priority: P1)

A maintainer can generate stable PNGs of every state needed for documentation without running, installing, or focusing ESO.

**Independent Test**: Run the documented capture command twice into separate empty directories and verify both manifests name the same scene, theme, viewport, dimensions, and file set.

**Acceptance Scenarios**:

1. **Given** an explicit output directory, **When** the capture target runs, **Then** it writes every named scene in both themes and both viewports plus one deterministic manifest.
2. **Given** no output directory, **When** the ordinary test suite runs, **Then** the target validates its catalog without initializing graphics or writing capture files.

---

### User Story 2 - Trust the isolation boundary (Priority: P1)

A reviewer can prove the documentation sandbox cannot hook physical input, synthesize input, capture the desktop, mutate an addon, or use a personal configuration directory.

**Independent Test**: Inspect and execute the isolation contract tests, then confirm the production binary exposes no capture argument or feature.

**Acceptance Scenarios**:

1. **Given** any scene, **When** it is built and rendered, **Then** no input action is emitted and no operating-system input or screen backend is constructed.
2. **Given** fixture filesystem state is needed, **When** the scene is prepared, **Then** generation writes stay below the explicit output root, validation writes stay in an automatically removed temporary directory, and neither path uses an addon lifecycle function.
3. **Given** a production build, **When** application arguments and features are inspected, **Then** no documentation-capture entry point exists.

---

### User Story 3 - Select stable documentation variants (Priority: P2)

A documentation author can identify each output by scene, theme, and viewport and can depend on fixed clocks, paths, providers, percentages, labels, and ordering.

**Independent Test**: Validate the catalog directly and inspect generated metadata for all 28 scene variants.

**Acceptance Scenarios**:

1. **Given** a named scene, **When** it is captured in any supported variant, **Then** the image dimensions and filename follow the catalog contract.
2. **Given** a capture manifest, **When** a reviewer reads it, **Then** it contains no account name, user profile, live path, current date, or machine-specific value.

### Edge Cases

- A missing, duplicated, or reordered required scene fails catalog validation.
- An unknown argument, relative output path outside the repository, symlinked output root, or non-directory output target fails closed before fixture creation.
- An output file collision is replaced only for the exact named PNG or manifest below the validated destination; the destination itself is never recursively deleted.
- A renderer or adapter failure exits nonzero and does not label partial output complete.
- A viewport smaller than the application minimum still renders the contracted narrow documentation canvas without opening a native window.
- Repeated captures never read or write the operator's configuration or ESO folders.

## Requirements

### Functional Requirements

- **FR-001**: The repository MUST provide one documented, non-interactive capture command with an explicit absolute or repository-relative output directory.
- **FR-002**: The capture entry point MUST exist only as a non-production Cargo test target and MUST NOT be reachable from the shipped application binary.
- **FR-003**: Running the ordinary test suite without capture arguments MUST validate the catalog without initializing the graphics renderer or writing generated capture files.
- **FR-004**: The catalog MUST contain exactly seven ordered named scenes covering first launch, healthy System and State, lost PixelBeacon, unmanaged addon, weaving configuration, Auto Potion ready, and Auto Potion blocked.
- **FR-005**: Every scene MUST render in dark and light themes at narrow and wide documentation viewports.
- **FR-006**: Captures MUST use the application's real `EsoWeaveApp::frame_ui` rendering seam with bundled fonts.
- **FR-007**: Scene construction MUST use deterministic application and view-model values and MUST contain no personal, account, character, current-time, random, or machine-derived data.
- **FR-008**: The harness MUST NOT install a physical-input hook, call an input synthesis backend, invoke live screen capture, call addon lifecycle mutation, or resolve a user configuration directory.
- **FR-009**: Generated-mode fixture files MUST remain below the validated explicit output root. Validation-only fixture files MUST remain in an automatically removed temporary directory. Neither mode may represent or modify a real ESO AddOns directory.
- **FR-010**: Rendering every scene MUST leave the input-engine output channel empty.
- **FR-011**: The output MUST include a deterministic manifest naming the schema, generator, scene, theme, viewport, dimensions, and PNG path for every successful capture.
- **FR-012**: The capture path MUST open no native window, steal no focus, require no interactive prompt, and launch no project-owned console child process.
- **FR-013**: Tests MUST validate scene completeness, ordering, identifiers, display labels, themes, viewports, model-state expectations, output containment, production isolation, and no emitted input.
- **FR-014**: Generated PNGs and fixture files MUST remain outside version control and MUST NOT be published into the manual by S083.
- **FR-015**: CI parity, UTF-8, LF, forbidden-dash, mojibake, and diff-integrity gates MUST pass.

### Key Entities

- **Capture scene**: Stable identifier, title, fixture kind, expected visible state, and fixed ordering.
- **Capture variant**: One scene combined with one theme and one viewport.
- **Fixture root**: Validated repository-local directory below the explicit output destination.
- **Capture receipt**: Deterministically ordered JSON record for one generated PNG.
- **Isolation contract**: Executable assertions over target placement, forbidden backend symbols, path containment, and empty input output.

## Success Criteria

### Measurable Outcomes

- **SC-001**: One command generates 28 PNGs and one manifest from 7 scenes, 2 themes, and 2 viewports.
- **SC-002**: Two runs produce identical manifest bytes and matching PNG dimensions and content hashes on the same pinned toolchain and renderer backend.
- **SC-003**: All 7 scene fixtures satisfy their contracted `AppView` assertions before rendering.
- **SC-004**: All captures leave the action receiver empty and create files only under the requested destination.
- **SC-005**: `cargo build --locked --release` contains no capture target, argument, mode, or feature.
- **SC-006**: The full local and hosted merge gates pass.

## Assumptions

- The pinned egui test renderer's predictable mode is the appropriate rendering authority for documentation generation on the maintained Windows environment.
- Narrow and wide documentation canvases are 760 by 1000 and 1280 by 900 logical points at one pixel per point.
- Cross-machine pixel identity is not promised because graphics adapters can differ; the catalog, data, dimensions, ordering, and same-environment output remain deterministic.
- Verification-only issues remain non-blocking under current project governance.

## Out of Scope

- Checking generated screenshots into the repository or adding them to documentation pages.
- Screenshot cropping, optimization, alternative text, update policy, or final curation assigned to issue #125.
- Operating-system screenshots such as MSI Properties.
- Live ESO, PixelBeacon overlay, desktop capture, physical input, synthesized input, or addon installation.
- A production demo mode, hidden command-line flag, feature flag, or alternate application startup.
