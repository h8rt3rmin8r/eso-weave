# Feature Specification: Accessible Troubleshooting Decision Tree

**Feature Branch**: `codex/s112-troubleshooting-decision-tree`

**Created**: 2026-09-17

**Status**: Implemented

**Input**: Work slice S112 implements GitHub issue #220 after the S111 visualization audit approved candidate `troubleshooting-decision-tree`.

## Clarifications

### Session 2026-09-17

- Q: What does the figure diagnose? -> A: It does not diagnose a root cause. It routes the reader from the first failing observation to the next authoritative evidence check.
- Q: Which branches are required? -> A: Application launch, game observation, PixelBeacon lifecycle or signal, input or binding evidence, and encounter capture or import.
- Q: What happens when none of the five families matches? -> A: The figure directs the reader to the feature-specific status and Live Log, matching the existing shared flow.
- Q: Does the figure replace or shorten the current prose? -> A: No. The existing shared diagnostic sequence and symptom sections remain the complete text equivalent and behavioral authority.
- Q: Which visual and interaction system applies? -> A: The existing repository-owned SVG, `.docs-flow-diagram`, and shared native-dialog system apply without a second renderer, viewer, or dependency.
- Q: How is future drift prevented? -> A: The source policy, generated-site policy, five-diagram rendering matrix, layout receipt, finite figure inventory, and explicit authority record all change in the same slice.

## User Scenarios & Testing

### User Story 1 - Choose the next evidence check (Priority: P1)

A reader starts with the first thing they can observe failing and selects one of five symptom families. The figure directs them to the next evidence boundary without skipping shared prerequisites or claiming that the selected branch is the root cause.

**Why this priority**: Troubleshooting currently requires scanning a shared sequence and several distant symptom sections before the reader can identify the next check.

**Independent Test**: Compare every visible branch and endpoint with the canonical troubleshooting prose and verify that the five required symptom families each lead to one named next check.

**Acceptance Scenarios**:

1. **Given** ESO Weave does not open, **when** the reader follows the launch branch, **then** the endpoint directs them to startup evidence and distinguishes pre-GUI notification from post-GUI logging.
2. **Given** ESO is not detected, active, focused, or in an available game context, **when** the reader follows the game-observation branch, **then** the endpoint directs them to installation, runtime, focus, and lifecycle evidence before feature diagnosis.
3. **Given** PixelBeacon is not current or its signal is missing, **when** the reader follows the PixelBeacon branch, **then** the endpoint directs them to managed lifecycle, overlay geometry, heartbeat, and freshness evidence.
4. **Given** input or native binding evidence is unavailable, **when** the reader follows the input branch, **then** the endpoint directs them to platform, focus, device, and ESO binding evidence without suggesting a safety bypass.
5. **Given** encounter capture or import does not complete, **when** the reader follows the encounter branch, **then** the endpoint directs them to addon status, saved authority, receipt, loss, and validation evidence.
6. **Given** none of the five first-observation families matches, **when** the reader reaches the continuation endpoint, **then** the figure directs them to the feature-specific status and Live Log.

### User Story 2 - Use the figure accessibly on every documentation surface (Priority: P1)

A reader can understand the figure at the normal documentation width, in the shared expanded view, at narrow viewport widths, at 200 percent browser zoom, with either maintained theme, and without relying on color alone.

**Why this priority**: A diagnostic figure that becomes unreadable or ambiguous on a maintained surface increases troubleshooting effort instead of reducing it.

**Independent Test**: Build the generated site and run the existing browser rendering, layout, figure, no-script, and zoom checks with the fifth diagram included.

**Acceptance Scenarios**:

1. **Given** navy or light theme, **when** the figure renders normally or expanded at 320 or 1280 CSS pixels, **then** it remains proportionate, contained, nonblank, and locally delivered.
2. **Given** 200 percent browser zoom, **when** the reader opens the shared figure dialog, **then** labels and routes remain readable and the complete prose remains available beside the source figure.
3. **Given** a reader cannot perceive color, **when** the decision tree is inspected, **then** visible words, node shapes, route labels, and arrow direction carry every distinction.
4. **Given** assistive technology reads the source, **when** it reaches the SVG, **then** the root exposes a meaningful title and description while the nearby prose supplies the complete equivalent.

### User Story 3 - Fail publication when the contract drifts (Priority: P2)

A maintainer receives an actionable local and hosted failure when the asset, placement, alternative, text equivalent, topology, finite inventory, generated bytes, or authority record diverges.

**Why this priority**: The figure summarizes five independently evolving subsystems, so silent drift would turn a comprehension aid into misleading guidance.

**Independent Test**: Mutate each governed record or generated observation and prove the documentation policy or browser receipt rejects the change.

**Acceptance Scenarios**:

1. **Given** the asset, page reference, alternative, text-equivalent anchors, or required visible labels are removed, **when** source policy runs, **then** it fails with the affected troubleshooting contract.
2. **Given** the generated asset is missing, remotely sourced, or differs from the checked-in SVG, **when** generated-site policy runs, **then** publication fails.
3. **Given** the fifth diagram is absent from the rendering or layout receipt, **when** browser evidence is validated, **then** the complete matrix fails.
4. **Given** a future change adds or removes a symptom family, changes a first check, renames a status, or transfers subsystem ownership, **when** maintainers inspect the authority record, **then** it names the figure as requiring review.

### Edge Cases

- A reader may report a later subsystem symptom while an earlier shared prerequisite is unavailable. The prose remains authoritative and explicitly stops diagnosis at the first failing observation.
- PixelBeacon lifecycle state and Pixel Bus signal state share one family but remain distinct evidence checks.
- Native binding evidence may be unavailable because PixelBeacon is stale. The input branch must point back to shared signal evidence instead of implying an independent fallback.
- Encounter capture state and desktop import validation are different boundaries. The endpoint must name both without implying the desktop can control capture.
- Startup failures before GUI initialization and failures after GUI initialization surface through different evidence channels.
- The figure may use concise labels, but it must not omit a family, invent a status, prescribe unsafe remediation, or claim a root cause.
- JavaScript may be blocked. The static figure and complete prose must remain visible without interaction enhancement.

## Requirements

### Functional Requirements

- **FR-001**: S112 MUST add exactly one repository-owned SVG at `docs/src/assets/diagrams/troubleshooting-decision-tree.svg` and MUST add no other graphic.
- **FR-002**: The troubleshooting page MUST place the figure inside the existing `.docs-flow-diagram` wrapper immediately beside the shared diagnostic flow.
- **FR-003**: The figure MUST route exactly five first-observation families: application launch, game observation, PixelBeacon lifecycle or signal, input or binding evidence, and encounter capture or import.
- **FR-004**: Each family MUST terminate at a named next evidence check or deeper section and MUST NOT claim a diagnosis or guaranteed fix.
- **FR-005**: The figure MUST include one continuation endpoint for unmatched symptoms that directs the reader to feature-specific status and the Live Log.
- **FR-006**: The current indented shared diagnostic sequence and symptom-specific sections MUST remain complete and authoritative. S112 MAY add only the figure, its wrapper, an introductory sentence, and an explicit text-equivalent heading or linkage needed by policy.
- **FR-007**: The SVG MUST expose an accessible root with `role="img"`, `focusable="false"`, matching title and description IDs, top-down flow direction, explicit intrinsic geometry, and an opaque local canvas.
- **FR-008**: Every visible label MUST be at least 14 SVG units, every route distinction MUST use visible words and direction in addition to color, and the SVG MUST contain no script, animation, event handler, remote URL, external font, embedded image, iframe, or foreign object.
- **FR-009**: Stable node, stage, edge, source, destination, branch, and edge-label metadata MUST expose the complete topology to the existing layout collector.
- **FR-010**: The diagram MUST satisfy the established layout contract: at least 36 SVG units between successive stages, at least 10 units between an edge and unrelated nodes, no crossing or undeclared shared segment, complete branch labels, and all visible content inside the canvas.
- **FR-011**: Documentation source policy MUST govern the exact asset, placement, meaningful alternative, text-equivalent anchors, intrinsic geometry, accessible root, visible labels, offline content, and topology-compatible source.
- **FR-012**: Generated-site policy MUST prove local SVG delivery, expected mdBook figure structure, source-byte identity, meaningful alternative retention, text-equivalent retention, and absence of a remote renderer.
- **FR-013**: The finite figure inventory MUST advance from 20 to 21 meaningful placements and from four to five flow diagrams while retaining 11 screenshots or illustrations, five brand images, one decorative wordmark, and 13 captions.
- **FR-014**: The browser rendering receipt MUST advance from 32 to 40 diagram observations across five diagrams, two themes, two viewport widths, and normal plus expanded states.
- **FR-015**: The browser layout receipt MUST require five unique diagram observations and apply the complete established geometry contract to the new decision tree.
- **FR-016**: The maintained figure-system and rendering-compatibility records MUST identify the fifth asset, its accessible behavior, its text-equivalent ownership, and the expanded matrices.
- **FR-017**: The S111 visualization audit record MUST remain the candidate authority and issue #220 MUST close through the official S112 pull request.
- **FR-018**: S112 MUST record these update triggers: a symptom-family change, first-check change, status-name change, troubleshooting prose change, or ownership change in `src/startup/mod.rs`, `src/game/mod.rs`, `src/beacon/mod.rs`, `src/pixelbus/mod.rs`, `src/input/mod.rs`, `src/input/bindings.rs`, `src/encounter/mod.rs`, `src/encounter/validate.rs`, or `src/app/encounter_history.rs`.
- **FR-019**: S112 MUST archive completed Plan 047 with PR #224 evidence, establish the next active build plan, update the migration ledger, and record the figure in the `[Unreleased]` changelog.
- **FR-020**: S112 MUST add no application, addon, input, automation, transport, configuration, package, runtime dependency, remote renderer, or second figure viewer behavior.
- **FR-021**: S112 MUST publish an official pull request that closes #220, process every review comment, invoke no more than the authorized second Codex review, and preserve operator merge authority.

### Key Entities

- **Symptom family**: One of the five first-observation categories that selects a diagnostic branch.
- **Evidence endpoint**: The next authoritative state, receipt, log, or deeper guide to inspect without asserting cause.
- **Decision-tree node**: A stable labelled box with one stage identity and a visible role in branch selection or evidence handoff.
- **Decision-tree edge**: An orthogonal connector with declared source and destination nodes and a visible branch label where a choice occurs.
- **Text equivalent**: The existing shared diagnostic sequence and symptom sections that preserve the complete behavior when the graphic is unavailable.
- **Authority record**: The maintained mapping from the figure to its prose and implementation sources plus the events that require review.

## Success Criteria

### Measurable Outcomes

- **SC-001**: All five required symptom families and the unmatched continuation are visible, route to the intended evidence endpoints, and match the canonical prose without an unsupported diagnosis.
- **SC-002**: The fifth SVG passes source policy with exact placement, alternative, text-equivalent anchors, intrinsic geometry, accessible labelling, at least four painted colors, visible labels of at least 14 units, and no active or external content.
- **SC-003**: The complete 40-cell rendering matrix passes across five diagrams, two themes, two viewport widths, and normal plus expanded states.
- **SC-004**: The complete five-diagram layout receipt passes with zero crossings, zero undeclared shared segments, complete topology metadata, at least 36 units between stages, and at least 10 units of unrelated-node clearance.
- **SC-005**: The normal, expanded, narrow, no-script, print, and 200 percent zoom evidence preserves a readable, contained, locally delivered figure and complete prose fallback.
- **SC-006**: Focused mutation tests reject a missing fifth diagram, wrong inventory count, missing family or evidence label, lost text-equivalent anchor, remote content, byte drift, and incomplete rendering or layout receipts.
- **SC-007**: Documentation policy, Node tests, mdBook test/build/link checks, browser smoke, spelling, UTF-8, mojibake, repository trust checks, and hosted CI pass.
- **SC-008**: The official pull request closes #220, every review thread is resolved, no more than one authorized second Codex review is requested, and the branch remains mergeable for the operator.

## Assumptions

- The existing troubleshooting prose is correct and remains the behavioral authority for S112.
- The established diagram design language, shared native dialog, CSS, and direct-SVG browser collector can accept a fifth asset without a new component.
- A concise 400-unit-wide top-down decision tree can remain readable through the existing normal and expanded presentation contract.
- The S111 audit already supplied the comprehension warrant, so S112 evaluates implementation accuracy rather than reopening whether the figure should exist.

## Out of Scope

- Changing diagnostic behavior, status semantics, source ownership, application logging, startup handling, game detection, PixelBeacon lifecycle, signal decoding, input handling, native bindings, encounter capture, or encounter import.
- Rewriting feature-specific troubleshooting or removing the complete prose equivalent.
- Implementing the catalog lifecycle, encounter lineage, or local-extension authority figures from issues #221 through #223.
- Adding remote assets, raster screenshots, a diagram generator, a runtime rendering dependency, a second expansion interaction, or analytics.
