# Feature Specification: BrandBuilder 2.0 Kit Adoption

**Feature Branch**: `codex/s117-brand-kit-v2-adoption`

**Created**: 2026-09-21

**Status**: Implemented (awaiting hosted validation)

**Input**: Work slice S117 implements GitHub issue #233 by adopting the complete applicable contract from the official `eso-weave-brand-1.0.0-bb2.0.0` kit.

## Clarifications

### Session 2026-09-21

- Q: What does full implementation mean for this desktop repository? -> A: Adopt every required kit surface and every optional surface that applies to the Rust/egui, Windows, Linux, packaging, and documentation targets. Web/React, Android, Apple, macOS, Tauri, Wails, and MSIX-only outputs remain outside the repository's shipping targets.
- Q: Which source wins when generated adapter output conflicts with higher-level contracts? -> A: Follow the kit's declared precedence. `brand.json`, Interface Canon, and component recipes outrank the generated egui adapter. Record any adapter defect instead of copying it.
- Q: Must the repository retain the full 10 MB kit? -> A: No. Retain an immutable machine-readable adoption record, the exact BrandBuilder recovery distribution, applicable fonts and platform assets, and hashes for every consumed contract. The canonical kit URL and archive hash preserve exact reacquisition without duplicating unrelated targets.
- Q: How are existing domain status and resource colors handled? -> A: Preserve meaningful health, stamina, magicka, and ultimate distinctions as documented domain semantics. All general surfaces, text, actions, focus, destructive state, borders, and interaction states move to governed kit roles.
- Q: Does the kit's 44-point target floor replace the compact visual control size? -> A: Yes for interactive hit targets. Visual treatment may remain compact, but every interactive allocation must meet the 44-point minimum and preserve existing behavior.
- Q: Are the authoritative SVG masters redesigned? -> A: No. Their bytes and hashes are unchanged. S117 corrects usage guidance and replaces only applicable derived platform artifacts supplied by the kit.

## User Scenarios & Testing

### User Story 1 - Use a coherent governed interface (Priority: P1)

A user opens ESO Weave in dark or light mode and sees a consistent semantic palette, readable type, visible focus treatment, and controls whose hit targets meet the kit contract.

**Why this priority**: The current UI uses legacy color roles and control sizing that conflict with the official implementation contract.

**Independent Test**: Resolve the theme in both modes and render representative controls, status rows, dialogs, and cards. Assert exact governed role values, required contrast, focus geometry, and minimum interaction allocations.

**Acceptance Scenarios**:

1. **Given** dark mode, **when** the UI renders, **then** backgrounds, cards, overlays, text, primary actions, emphasis, destructive state, borders, and focus use the exact governed dark roles.
2. **Given** light mode, **when** the UI renders, **then** the light surface roles and accessible gold action role replace the legacy palette without using bright teal as light-surface text.
3. **Given** an interactive control, **when** it renders at either density, **then** its hit allocation is at least 44 by 44 logical points and its status is never communicated by color alone.

### User Story 2 - Read technical information in the intended type system (Priority: P1)

A user can distinguish normal product copy from identifiers, timestamps, paths, status metadata, and technical values through the approved Inter and Geist Mono families.

**Why this priority**: Typography migration is required by the release-impact contract, and the current app has no approved mono family.

**Independent Test**: Inspect installed font definitions and representative metadata widgets, then assert the approved local files, hashes, weights, family fallbacks, and semantic font assignments.

**Acceptance Scenarios**:

1. **Given** body copy or a display heading, **when** it renders, **then** it uses the declared Inter weight without synthesized font weight.
2. **Given** an identifier, timestamp, path, code value, or compact technical status, **when** it renders, **then** it uses bundled Geist Mono Regular with a safe glyph fallback.
3. **Given** the app has no network access, **when** fonts initialize, **then** all approved typography remains available from bundled bytes.

### User Story 3 - Trust shipped identity and platform assets (Priority: P2)

A user sees the approved identity at compliant sizes in application, installer, Linux desktop, and documentation surfaces.

**Why this priority**: Platform assets are optional in the kit, but Windows and Linux are active product targets and therefore applicable to full adoption.

**Independent Test**: Compare every retained master and adopted platform asset to its pinned hash or documented generated manifest and validate packaging references.

**Acceptance Scenarios**:

1. **Given** an authoritative SVG master, **when** integrity checks run, **then** its existing approved bytes remain unchanged.
2. **Given** a reduced mark placement, **when** its documented size is below 32 pixels, **then** guidance requires the reduced mark or text alternative and never claims a 16-pixel minimum for the full mark.
3. **Given** Windows or Linux packaging, **when** artifacts are built, **then** they consume the applicable official kit asset and the dated pinned-artifact decision is recorded.

### User Story 4 - Audit and recover the exact adopted kit (Priority: P2)

A maintainer can determine exactly which kit governed S117, verify every consumed byte offline, and recover BrandBuilder 2.0.0 without substituting a moving latest version.

**Why this priority**: Documentation and recovery are required migration surfaces, and a visual change without provenance becomes a parallel ungoverned design system.

**Independent Test**: Run the repository brand-contract validator against the adoption record, retained recovery archive, fonts, masters, platform artifacts, and runtime token table with networking disabled.

**Acceptance Scenarios**:

1. **Given** the adoption record, **when** it is inspected, **then** it identifies the exact package, archive URL, archive checksum, source revision, contract versions, authority order, and migration classifications.
2. **Given** the retained recovery distribution, **when** it is hashed, **then** it matches `26578eb150a9c24d9e625fb77b192e0415a6ac8faf67c83834ac914f2da15e90`.
3. **Given** an accidental token, font, master, or platform-asset drift, **when** the validator runs, **then** CI fails with the mismatched role or file.

### Edge Cases

- The generated egui adapter reverses the light background and card values and uses the dark destructive color in light mode. Higher-authority brand and Interface Canon values must win, and the deviation must be documented.
- Bright teal is readable on dark surfaces but fails as text on the light base. Light emphasis text must use accessible gold or another governed readable role.
- Existing resource meter colors are product-domain data visualization roles, not general UI roles. They require contrast tests and explicit documentation instead of silent remapping.
- Egui's visual rectangle may be smaller than its response rectangle. Tests must measure the interactive response allocation when enforcing the 44-point target floor.
- Full font replacement must keep framework fallback fonts for symbols and uncommon glyphs.
- Packaging assets are pinned. Any changed Windows or Linux packaging byte requires a dated `[Unreleased]` changelog decision.
- The exact kit may become unavailable online. Local verification and BrandBuilder recovery must still work from retained bytes and recorded hashes.

## Requirements

### Functional Requirements

- **FR-001**: S117 MUST pin package `eso-weave-brand-1.0.0-bb2.0.0`, BrandBuilder `2.0.0`, source revision `f974fbb5c532a3394980be4dd985aee86e7e2c9e`, the canonical download URL, and archive SHA-256 `b37ac1459666ae33d772229bd5247699c2c845971eabed267ff85465b68a1ba1`.
- **FR-002**: The adoption record MUST pin Brand Canon `1.2.1`, Interface Canon `1.0.0`, component recipes `1.1.0`, Web/React adapter `1.1.0`, egui adapter `1.0.0`, compiler `2.0.0`, brand `1.0.0`, and the kit authority order.
- **FR-003**: The repository MUST retain the exact `shruggie-brandbuilder-2.0.0.skill` recovery distribution and verify its published SHA-256.
- **FR-004**: The runtime theme MUST map semantic roles from the higher-authority brand and Interface Canon contracts for dark and light modes.
- **FR-005**: General UI styling MUST use named semantic roles instead of legacy gold-specific field names or scattered raw brand values.
- **FR-006**: Dark primary and emphasis MUST use `#2DD4BF` with black foreground when filled; light primary MUST use `#986000` with white foreground.
- **FR-007**: Surfaces, primary text, muted text, destructive state, border/input, card, overlay, focus, selection, hover, active, and disabled treatments MUST use governed values or transforms.
- **FR-008**: Interactive response allocations MUST meet the 44 logical-point minimum target in both maintained themes and compact application layouts.
- **FR-009**: Focus-visible styling MUST expose a 2-point governed focus indicator and status MUST retain a non-color label or shape.
- **FR-010**: The app MUST bundle Geist Mono Regular with its license and register Inter 400/500/600 plus Geist Mono 400 without runtime network access or synthesized weights.
- **FR-011**: Technical identifiers, timestamps, paths, code-like values, and technical metadata SHOULD use the named mono family where their current widget seam permits semantic assignment without unrelated UI redesign.
- **FR-012**: Authoritative glyph and mark SVG bytes MUST remain unchanged and MUST be covered by integrity checks.
- **FR-013**: The app, Windows, Linux, installer, and documentation assets MUST adopt the applicable supplied kit derivatives or document why an existing byte is already the official reference byte.
- **FR-014**: The brand standard MUST document exact package provenance, semantic palette, typography, spacing, radii, target floor, focus, identity sizes, affiliation boundary, platform adoption, and recovery procedure.
- **FR-015**: Identity guidance MUST state a 32-pixel reduced-mark threshold and MUST prohibit artificial crossing overlays, knockouts, outlines, or substrate separators in single-ink derivatives.
- **FR-016**: A machine-executable validator MUST verify adoption metadata, retained artifact hashes, authoritative master hashes, font hashes, applicable platform asset hashes, and runtime semantic token values.
- **FR-017**: Tests MUST assert WCAG AA text pairs, 3:1 non-text contrast where applicable, action foreground pairs, and the light-theme prohibition on bright teal text.
- **FR-018**: Existing automation, input, addon, persistence, network, and safety behavior MUST remain unchanged.
- **FR-019**: S117 MUST record every pinned packaging artifact change as a dated `[Unreleased]` decision in `CHANGELOG.md`.
- **FR-020**: S117 MUST publish through an official pull request that closes #233, pass all hosted checks, address every review comment, and perform no more than one explicitly authorized second Codex review round.

### Key Entities

- **Brand kit adoption record**: Immutable package identity, source, versions, authority, migration classifications, recovery data, and consumed artifact hashes.
- **Semantic theme**: Named renderer roles for surfaces, text, actions, state, border, focus, status, and product-domain resource visualization.
- **Font role**: Approved local family, weight, file, license, hash, and semantic usage.
- **Asset binding**: A repository destination linked to an official source path, purpose, target platform, and checksum.
- **Conformance result**: A deterministic pass or actionable mismatch for metadata, tokens, fonts, masters, recovery, and platform assets.

## Success Criteria

### Measurable Outcomes

- **SC-001**: One hundred percent of governed runtime semantic roles equal the pinned kit values in both themes.
- **SC-002**: Every tested interactive response is at least 44 by 44 logical points and every required focus indicator is at least 2 points.
- **SC-003**: All required text pairs meet 4.5:1 and all required non-text pairs meet 3:1 at rendered values.
- **SC-004**: All four approved font files and both authoritative SVG masters match their published hashes.
- **SC-005**: Every applicable adopted platform artifact matches its recorded official-kit hash, and every unchanged reference asset has an explicit disposition.
- **SC-006**: The brand validator detects a deliberate metadata, token, font, master, recovery, or platform-asset mismatch in tests and passes on committed bytes.
- **SC-007**: Fmt, clippy, all locked tests, release build, documentation checks, encoding checks, forbidden-dash checks, and hosted CI pass.
- **SC-008**: The official pull request closes #233, has no unresolved review threads, and receives at most two Codex review rounds total.

## Assumptions

- The official site URL and exact archive checksum are stable identifiers for this published package.
- The repository's current eframe/egui 0.36 line is compatible with the kit's egui adapter 1.0.0.
- Retaining the exact recovery distribution plus consumed contracts and hashes satisfies offline recovery without vendoring unrelated target suites.
- The existing dark/light theme preference remains the only user-facing theme selection in this slice.

## Out of Scope

- Redesigning authoritative identity geometry, changing affiliation, changing product behavior, or adding a third theme.
- Adopting Web/React, Android, Apple, macOS, Tauri, Wails, or MSIX assets that the repository does not build.
- Upstreaming the discovered egui adapter defect or any local capability gap without separate human authorization.
- Cutting or installing a release, merging the pull request, or performing post-release visual verification.
