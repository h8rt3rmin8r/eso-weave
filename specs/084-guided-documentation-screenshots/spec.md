# Feature Specification: Guided Documentation Screenshots

**Feature Branch**: `codex/s084-guided-documentation-screenshots`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Work slice S084 implements issue #125 using the S083 deterministic screenshot sandbox and the maintainer-supplied Windows MSI Properties capture.

## Clarifications

### Session 2026-09-11

- The supplied MSI Properties PNG is used as-is. It already obscures the account-specific segment of the path and visibly identifies the Unblock control.
- Application images are selected from a fresh S083 capture run. They do not capture the desktop, run ESO, inspect a user profile, or control native windows.
- Published application images use the dark, wide variants for a consistent visual language and enough source resolution for responsive display.
- The screenshot set contains seven deterministic application captures, one supplied operating-system capture, and one locally authored synthetic PixelBeacon overlay illustration.
- The synthetic overlay is clearly labeled and explains placement and solid-color telemetry without presenting fabricated gameplay as a live capture.
- Images supplement complete adjacent instructions. Readers must not need color vision or the image itself to perform the documented task.

## User Scenarios & Testing

### User Story 1 - Recognize installation and first-launch states (Priority: P1)

A Windows reader can identify the optional MSI Unblock control and then recognize the first-launch, healthy, lost-signal, and unmanaged-addon states in ESO Weave.

**Independent Test**: Build the manual and follow Installation and First Launch using only the adjacent text, then verify each image confirms the named control or state without exposing personal data.

**Acceptance Scenarios**:

1. **Given** an MSI that Windows marked as downloaded, **When** the reader follows Installation, **Then** the supplied Properties image shows the General tab, security notice, and checked Unblock control.
2. **Given** a new ESO Weave setup, **When** the reader follows First Launch, **Then** deterministic images distinguish the initial interface, a healthy baseline, lost PixelBeacon signal, and unmanaged addon state.

---

### User Story 2 - Configure Weaving and Auto Potion safely (Priority: P1)

A reader can recognize the fields and statuses involved in Weaving and Auto Potion without starting live automation or requiring ESO.

**Independent Test**: Inspect the Weaving and Auto Potion pages at narrow and wide documentation widths and verify the images show the documented controls while the prose contains every required instruction and safety condition.

**Acceptance Scenarios**:

1. **Given** the Weaving guide, **When** the reader reaches configuration, **Then** an image shows enabled slots, weave types, and effective delay in deterministic application state.
2. **Given** the Auto Potion guide, **When** the reader configures watches and diagnoses a blocker, **Then** ready and blocked images show thresholds, Quickslot prerequisites, and the missing-signal recovery distinction.

---

### User Story 3 - Understand PixelBeacon without a live game capture (Priority: P2)

A reader can recognize addon lifecycle controls and understand the overlay's fixed top-left placement without seeing real gameplay or personal account content.

**Independent Test**: Review the PixelBeacon page and prove that the healthy and unmanaged application images plus the labeled synthetic overlay explain installation, safe ownership, and placement without implying that the illustration is a live screenshot.

**Acceptance Scenarios**:

1. **Given** a managed or unmanaged addon state, **When** the reader uses PixelBeacon guidance, **Then** application captures show the available lifecycle control and the state where mutation is intentionally refused.
2. **Given** the overlay explanation, **When** the reader sees the illustration, **Then** its caption and embedded label identify it as synthetic and the adjacent prose explains the same placement rule.

### Edge Cases

- A missing, renamed, zero-byte, malformed, or unexpectedly dimensioned asset fails policy validation.
- A documentation image reference that escapes `docs/src/assets/` or points to a generated `target/` capture fails policy validation.
- An application capture whose filename, receipt, dimensions, or SHA-256 digest does not match the curated provenance manifest fails validation.
- The supplied MSI image remains byte-identical to the user-provided source for the initial S084 publication.
- Captions and adjacent prose continue to make sense when images do not load.
- Images scale down within the content column without causing page-level horizontal overflow.
- Public and bundled documentation resolve the same local files without a network dependency.

## Requirements

### Functional Requirements

- **FR-001**: The repository MUST contain a screenshot plan mapping every image to a reader question, page, source scene, theme, viewport, alternative text, and update trigger.
- **FR-002**: Installation MUST publish the maintainer-supplied MSI Properties PNG unchanged and explain that Unblock appears only when Windows marks the file as downloaded.
- **FR-003**: First Launch MUST illustrate first launch, healthy System and State, lost PixelBeacon signal, and unmanaged PixelBeacon states.
- **FR-004**: Weaving MUST illustrate enabled slots, weave-type selection, and effective delay without enabling live or unsafe automation.
- **FR-005**: Auto Potion MUST illustrate resource watches, thresholds, Quickslot readiness, and a blocked missing-signal recovery path.
- **FR-006**: PixelBeacon MUST show lifecycle controls through deterministic application images and MUST use a clearly labeled synthetic overlay illustration rather than live gameplay capture.
- **FR-007**: Application captures MUST be generated through the S083 test-only sandbox from deterministic fixture state and MUST use the dark, wide variant.
- **FR-008**: Curated application PNG files MUST be derived from a successful capture set through documented lossless crop rectangles and tracked with scene, theme, viewport, source receipt, crop, published dimensions, SHA-256 digest, destination, and update trigger.
- **FR-009**: Every image MUST have meaningful alternative text, an adjacent caption, and complete adjacent instructions that do not depend on color or image availability.
- **FR-010**: Published assets MUST use stable descriptive filenames beneath `docs/src/assets/`, remain local to the manual, and be available identically in public and bundled builds.
- **FR-011**: The manual MUST include a maintainer update checklist covering capture regeneration, curation, provenance refresh, review, optimization, and documentation validation.
- **FR-012**: Automated policy checks MUST verify the curated asset inventory, file signatures, expected dimensions, provenance agreement, local references, and byte identity of the supplied MSI source copy.
- **FR-013**: Screenshot presentation MUST constrain width responsively, preserve aspect ratio, expose visible focus treatment for any linked image, and cause no page-level horizontal overflow at supported widths.
- **FR-014**: S084 MUST NOT add a production capture mode, capture the Windows desktop, run ESO, publish personal account data, or couple screenshot generation to physical or synthesized input.
- **FR-015**: Documentation build, policy, link, accessibility, UTF-8, LF, forbidden-dash, mojibake, and diff-integrity gates MUST pass.

### Key Entities

- **Screenshot plan entry**: Reader question, page, source, selected variant, alternative text, caption, and update trigger.
- **Curated capture**: Repository PNG selected from an S083 generated capture receipt.
- **Supplied capture**: User-provided MSI Properties PNG preserved byte-for-byte.
- **Synthetic illustration**: Repository-authored SVG that is explicitly labeled as non-live and contains no game-owned image content.
- **Provenance manifest**: Canonical record connecting published assets to sources, dimensions, hashes, and maintenance triggers.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Nine planned visual assets are published: seven deterministic application PNG files, one supplied Windows PNG, and one synthetic SVG.
- **SC-002**: Installation, First Launch, Weaving, Auto Potion, and PixelBeacon each contain at least one task-focused visual with meaningful alternative text and adjacent instructions.
- **SC-003**: All seven application PNG files trace to matching S083 capture receipts and match the curated provenance manifest by crop rectangle, published dimensions, and SHA-256 digest.
- **SC-004**: The published MSI Properties PNG is byte-identical to the supplied source at implementation time.
- **SC-005**: Automated documentation policy rejects missing, malformed, untracked, or provenance-divergent screenshot assets.
- **SC-006**: The built manual has no page-level horizontal overflow caused by these assets at representative narrow and wide widths.
- **SC-007**: Public and bundled documentation builds use the same local asset paths and pass all merge gates.

## Assumptions

- The user-provided MSI Properties screenshot is authorized for repository publication as-is.
- Dark, wide S083 captures provide the most legible and consistent source for the initial curated set.
- Application cropping and PNG optimization must be lossless so provenance can name the exact published bytes and derivation.
- The existing documentation theme is the correct shared presentation layer for public and bundled manuals.

## Out of Scope

- New S083 fixture scenes, a production screenshot mode, or changes to application runtime behavior.
- Live ESO screenshots, third-party game artwork, account content, character names, or desktop capture.
- Replacing or redacting the supplied MSI screenshot in this initial publication.
- A complete responsive-table redesign assigned to issue #127.
- General work-slice reference normalization assigned to issue #126.
