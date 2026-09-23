# Feature Specification: BrandBuilder 2.0.1 Density Correction

**Feature Branch**: `codex/s119-brandbuilder-2-0-1-density`

**Created**: 2026-09-22

**Status**: In Progress

**Input**: Work slice S119 implements GitHub issue #241 by adopting the corrected native density contract published by ShruggieTech BrandBuilder 2.0.1.

## Clarifications

### Session 2026-09-22

- Q: Which upstream change is authoritative? -> A: The immutable ESO Weave BrandBuilder 2.0.1 kit and its bundled contracts are authoritative. Its published bundle identifies source revision `801ed912aaa8bbc66c12d8a97b5487fcddb0d9da`.
- Q: Which runtime profile applies to ESO Weave? -> A: ESO Weave is a keyboard and precise-pointer desktop application, so its normal theme uses the kit's fine-pointer comfortable profile. The app does not claim touch support it cannot detect or validate.
- Q: Should S119 preserve the global 44-point egui allocation from S117? -> A: No. Upstream explicitly identifies that unconditional allocation as the regression. S119 replaces it with the governed 28-point fine-pointer height, while documenting that coarse, mixed, unknown, and touch profiles remain 44 points in consumers that expose those capabilities.
- Q: What stays unchanged? -> A: Brand identity, palette, typography, resource meters, focus styling, product behavior, safety-critical logic, and platform assets remain unchanged unless the immutable rebuilt kit proves a byte changed.

## User Scenarios & Testing

### User Story 1 - Use compact desktop controls (Priority: P1)

As a keyboard and mouse user, I can operate ESO Weave without ordinary buttons, selectors, switches, checkboxes, and radio-style controls consuming touch-sized vertical space.

**Why this priority**: The current global 44-point control size is a visible regression across the desktop interface.

**Independent Test**: Apply each maintained theme and assert that fine-pointer controls use the governed 28-point comfortable height, 8-by-4 point button padding, and stable visual geometry.

**Acceptance Scenarios**:

1. **Given** the dark or light theme, **when** the normal desktop style is applied, **then** the minimum control height is no greater than 28 logical points at normal text scale.
2. **Given** adjacent control types, **when** they render, **then** they share one compact padding and spacing model without hover reflow.

### User Story 2 - Scan dense operational rows (Priority: P1)

As an operator reading logs and status rows, I can scan consecutive entries without global double-spaced vertical gaps.

**Why this priority**: Operational lists lose useful context when every row inherits 12 points of vertical separation.

**Independent Test**: Apply the theme and verify global item spacing is 8 points horizontally and no more than 2 points vertically, while product-owned resource meter dimensions remain unchanged.

**Acceptance Scenarios**:

1. **Given** consecutive one-line log entries, **when** they use the global style, **then** their generated vertical item spacing is no more than 2 logical points.
2. **Given** the existing health, Magicka, Stamina, and Ultimate meters, **when** S119 is applied, **then** their product-specific presentation is unchanged.

### User Story 3 - Audit the corrected kit (Priority: P2)

As a maintainer, I can prove which BrandBuilder 2.0.1 bytes and native adapter contract govern the application and recover that exact compiler distribution offline.

**Why this priority**: The density correction changes generated native behavior and must not be represented as the older immutable 2.0.0 package.

**Independent Test**: Run the brand policy validator against the updated adoption record, recovery artifact, applicable repository assets, and runtime density values.

**Acceptance Scenarios**:

1. **Given** the adoption record, **when** it is inspected, **then** it identifies BrandBuilder 2.0.1, egui adapter 1.0.1, the exact package checksum, source revision, and release identity.
2. **Given** an accidental regression to 44-point fine-pointer controls or 12-point vertical spacing, **when** repository tests run, **then** they fail with the mismatched value.

### Edge Cases

- Large text must be allowed to grow controls rather than clip labels, even though normal text uses compact fine-pointer sizing.
- Product-specific widgets may request a larger minimum size; S119 changes global defaults, not every intentional local allocation.
- A future touch-capable product surface must use capability-aware conservative targets instead of copying this desktop-only default blindly.
- Identity or platform bytes that are unchanged in the rebuilt kit must remain byte-identical in the repository.
- If the rebuilt package is not yet release-backed, implementation may proceed locally from the CI-certified artifact but publication must wait for an immutable official package URL and checksum.

## Requirements

### Functional Requirements

- **FR-001**: S119 MUST pin the immutable ESO Weave BrandBuilder 2.0.1 package, source revision `801ed912aaa8bbc66c12d8a97b5487fcddb0d9da`, exact archive checksum, and official publication identity.
- **FR-002**: The adoption record MUST identify BrandBuilder compiler 2.0.1 and egui adapter 1.0.1 while preserving unchanged Brand Canon, Interface Canon, component recipe, Web/React adapter, and brand versions.
- **FR-003**: The repository MUST retain and validate the exact BrandBuilder 2.0.1 recovery distribution.
- **FR-004**: The normal ESO Weave desktop style MUST use the governed fine-pointer comfortable control height of 28 logical points at normal text scale.
- **FR-005**: Global item spacing MUST be 8 logical points horizontally and 2 logical points vertically.
- **FR-006**: Global button padding MUST be 8 logical points horizontally and 4 logical points vertically.
- **FR-007**: Dark and light palette roles, focus width, radii, font registrations, status semantics, and hover-stable geometry MUST remain unchanged.
- **FR-008**: Health, Magicka, Stamina, and Ultimate meter presentation MUST remain unchanged.
- **FR-009**: Tests MUST fail if the normal fine-pointer height exceeds 28 points, vertical item spacing exceeds 2 points, or padding diverges from the governed values.
- **FR-010**: Brand policy validation MUST verify the updated package, compiler, adapter, recovery, and runtime density values.
- **FR-011**: Documentation MUST explain the fine-pointer versus conservative-target distinction and remove the claim that every desktop interaction allocation is always 44 points.
- **FR-012**: Existing input, automation, addon, persistence, network, privacy, and safety-critical behavior MUST remain unchanged.
- **FR-013**: All changed text MUST remain UTF-8 without BOM, LF-only, free of mojibake, and free of em or en dash characters.

### Key Entities

- **Brand kit adoption record**: Immutable package identity, release provenance, contract versions, recovery bytes, runtime tokens, and consumed artifact hashes.
- **Fine-pointer density profile**: Governed desktop control height, item spacing, and padding for precise-pointer use.
- **Conservative target profile**: The 44-point target retained by capability-aware consumers for coarse, mixed, unknown, or touch input.

## Success Criteria

### Measurable Outcomes

- **SC-001**: One hundred percent of maintained themes resolve to a 28-point normal control height, 8-by-2 point item spacing, and 8-by-4 point button padding.
- **SC-002**: All existing palette, contrast, focus, typography, and resource-meter tests continue to pass without expectation weakening.
- **SC-003**: The brand validator detects deliberate package-version, adapter-version, recovery-hash, and density-value mutations and passes on committed bytes.
- **SC-004**: Fmt, clippy, all locked tests, release build, documentation checks, encoding checks, and forbidden-dash checks pass.
- **SC-005**: The implementation diff contains no product behavior change outside theme density, provenance, validation, documentation, and the S119 evidence packet.

## Assumptions

- ESO Weave remains a desktop-first keyboard and precise-pointer application and does not currently expose a supported touch input profile.
- The rebuilt 2.0.1 kit preserves identity, palette, typography, and platform asset bytes from 2.0.0.
- Upstream publication will provide an immutable package and recovery artifact before remote publication of S119.

## Out of Scope

- Adding runtime touch or pointer-capability detection.
- Redesigning product-owned resource meters or any identity asset.
- Changing palette, typography, application behavior, automation, capture, addon, or transport surfaces.
- Cutting a release or merging the pull request.
