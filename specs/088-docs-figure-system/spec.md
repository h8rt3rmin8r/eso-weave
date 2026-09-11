# Feature Specification: Documentation Figure System

**Feature Branch**: `codex/s088-docs-figure-system`

**Created**: 2026-09-11

**Status**: Implemented

**Input**: Issues #155 and #156 plus the approved S088 work-slice proposal.

## User Scenarios & Testing

### User Story 1 - Inspect every meaningful figure (Priority: P1)

A documentation reader can recognize that a meaningful screenshot, diagram, illustration, or brand example is expandable and can open it at the largest useful size without distortion.

**Why this priority**: The current raw HTML screenshots and brand examples cannot be enlarged, while the existing diagram-only checkbox behavior is inconsistent and difficult to discover.

**Independent Test**: Build the manual and inspect every current image placement. All 20 meaningful placements expose the same visible expansion affordance and open through pointer input, while the decorative landing wordmark remains non-interactive and hidden from assistive technology.

**Acceptance Scenarios**:

1. **Given** a meaningful figure, **When** a reader activates its visible expansion control, **Then** one modal presents the same local image at the largest contained size without stretching beyond its intrinsic resolution.
2. **Given** a decorative identity image with empty alternative text, **When** the page is enhanced, **Then** it remains non-interactive and does not become an unnecessary control.
3. **Given** a page containing multiple meaningful figures, **When** each control is inspected, **Then** every control has a meaningful accessible name derived from its image alternative and no duplicate modal implementation exists.

---

### User Story 2 - Operate the expanded view accessibly (Priority: P1)

A keyboard or assistive-technology user can open, understand, and close the expanded view without losing focus or interacting with the page behind it.

**Why this priority**: Expansion is not complete when it works only as a pointer effect. Focus containment, background inertness, close behavior, and return focus are part of the core interaction.

**Independent Test**: Focus representative diagram and screenshot controls, open each with Enter or Space, traverse the modal, close it with the close button and Escape, and verify that focus returns to the invoking control while background content remains inert.

**Acceptance Scenarios**:

1. **Given** a focused figure control, **When** Enter or Space is pressed, **Then** the expanded modal opens and focus moves to its obvious close control.
2. **Given** an open expanded modal, **When** Escape, the close control, or the modal backdrop is activated, **Then** the modal closes and focus returns to the exact invoking control.
3. **Given** an open expanded modal, **When** keyboard focus advances, **Then** it remains inside the modal and background controls cannot receive interaction.
4. **Given** an expanded image, **When** an assistive technology inspects the modal, **Then** the image alternative and optional figure caption describe one active image without a duplicate accessible clone.

---

### User Story 3 - Read captions as supporting information (Priority: P2)

A reader can immediately distinguish figure captions from surrounding body prose while retaining comfortable size, contrast, and semantic association.

**Why this priority**: Existing screenshot and brand captions inherit body scale and compete visually with the prose they support.

**Independent Test**: Inspect all visible captions in every supported theme at narrow and wide widths and at 200 percent browser zoom. Captions use one shared hierarchy, remain at least 0.875rem, satisfy AA contrast, and stay associated with their figure.

**Acceptance Scenarios**:

1. **Given** a screenshot or brand caption, **When** the page renders, **Then** the caption is visibly subordinate to body text through size, spacing, and weight without relying on color alone.
2. **Given** a caption with a strong lead-in, **When** the shared treatment applies, **Then** the lead-in remains useful and the complete caption remains readable.
3. **Given** a narrow viewport or 200 percent zoom, **When** a caption wraps, **Then** it remains unclipped, readable, and visually attached to its figure.

---

### User Story 4 - Preserve local delivery and print output (Priority: P2)

A reader receives the same figure and caption experience from GitHub Pages and bundled offline documentation, while printed documentation keeps clean static figures without interactive chrome.

**Why this priority**: The documentation has one shared source and must not acquire a network dependency or a broken print presentation.

**Independent Test**: Run the generated-site policy and hidden-browser matrix with networking disabled, then inspect print styles. Pages and bundled output use only the checked-in theme assets, and print hides the dialog and expansion affordance while retaining each source image and caption.

**Acceptance Scenarios**:

1. **Given** the generated site with networking disabled, **When** a figure opens, **Then** no external script, stylesheet, image, font, or package is required.
2. **Given** print output, **When** interactive enhancement has run, **Then** the original figures and captions remain visible without modal controls, overlays, or button decoration.
3. **Given** JavaScript that cannot run, **When** the page renders, **Then** source figures and captions remain readable as static content.

### Edge Cases

- mdBook 0.5.4 emits a hidden checkbox, a primary image, and a duplicate expanded clone only for Markdown images.
- Raw HTML screenshots and brand examples bypass mdBook's generated zoom structure.
- A brand surface contains multiple meaningful images that share one group caption.
- The same screenshot asset appears on more than one page and every placement still needs its own control.
- Portrait and landscape assets require different limiting dimensions without upscaling either asset beyond its intrinsic size.
- A source image may not have completed decoding when the reader first activates it.
- Re-running the local theme script or navigating through browser history must not create duplicate controls or dialogs.
- The decorative landing wordmark has empty alternative text and must remain non-interactive.
- Print rendering may execute JavaScript before printing, so print styles must neutralize the enhanced controls.
- Reduced-motion preferences must not be defeated by modal animation.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST inventory exactly 20 current meaningful image placements: 11 screenshot or illustration placements, four flow diagrams, and five brand-example placements.
- **FR-002**: The decorative landing wordmark MUST remain non-interactive, retain empty alternative text, and be explicitly excluded from the meaningful inventory.
- **FR-003**: Every meaningful image MUST receive one consistent semantic button with a persistent visible expansion affordance and a meaningful accessible name derived from the image alternative.
- **FR-004**: The enhancement MUST operate idempotently and MUST create exactly one shared modal dialog per document.
- **FR-005**: The dialog MUST use the selected image's existing local source, MUST preserve its intrinsic aspect ratio, and MUST not upscale it beyond intrinsic resolution.
- **FR-006**: The dialog MUST expose the selected image alternative as its accessible name and MUST include the associated figure caption as its description when one exists.
- **FR-007**: The active modal MUST expose one image meaning to assistive technology and MUST NOT retain mdBook's duplicate expanded image as a second accessible or visual modal.
- **FR-008**: Pointer activation, Enter, and Space MUST open the modal from the figure control.
- **FR-009**: Escape, an obvious close button, and backdrop pointer activation MUST close the modal.
- **FR-010**: Opening MUST move focus to the close control, modal focus MUST remain contained, and closing MUST return focus to the exact invoking control.
- **FR-011**: The modal MUST make the rest of the document inert through the browser's native modal behavior or an equivalently verified local mechanism.
- **FR-012**: Focus-visible, hover, pointer, and persistent affordance treatments MUST make the interaction discoverable without relying on hover or color alone.
- **FR-013**: All current visible `figcaption` elements MUST participate in one shared caption hierarchy, with documented component-specific color exceptions for dark and light brand surfaces.
- **FR-014**: Caption text MUST use a `0.9em` body-relative scale, compute to at least 14 CSS pixels, remain visually subordinate to body copy and readable at 200 percent zoom, and provide at least 4.5:1 contrast against its background in all five supported themes.
- **FR-015**: Strong lead-ins in brand captions MUST retain their semantic and visual emphasis without applying italics to the complete caption.
- **FR-016**: Caption wrapping MUST remain contained and associated with its figure at 320 and 1280 CSS pixels.
- **FR-017**: Print output MUST retain source images and captions while hiding the shared dialog, close control, and visible expansion affordance.
- **FR-018**: GitHub Pages and bundled offline documentation MUST use the same checked-in JavaScript and CSS with no CDN, browser download, package dependency, or network-only resource.
- **FR-019**: Automated source and generated policy MUST reject missing or duplicate figure controls, an interactive decorative image, duplicate dialogs, retained mdBook clone behavior, missing accessible names, lost captions, undersized captions, low contrast, or print chrome.
- **FR-020**: A hidden-browser smoke MUST cover one Markdown diagram, one landscape screenshot, one portrait screenshot, one illustration, and one brand example at narrow and wide widths in light and dark themes.
- **FR-021**: Browser evidence MUST cover pointer open and close, keyboard open with Enter, keyboard close with Escape, close-button focus, focus return, modal state, containment, aspect ratio, non-upscaling, caption association, and absence of duplicate modal implementations.
- **FR-022**: The work MUST change no application, addon, gameplay, input, telemetry, catalog, encounter, or release behavior.
- **FR-023**: Issue #127 MUST remain the owner of general responsive table and page-overflow work.

### Key Entities

- **Figure Placement**: Page, source image, figure class, alternative text, optional caption, intrinsic dimensions, semantic kind, and interactive or decorative classification.
- **Expansion Control**: The one semantic button bound to a meaningful image, with its accessible label, visible affordance, enhancement marker, and invoking-focus identity.
- **Figure Dialog**: The one document-level native modal containing a close control, one decorative visual image, an accessible image name, and an optional cloned caption description.
- **Caption Treatment**: Shared font size, line height, spacing, weight, contrast, wrapping, and component-specific surface colors.
- **Figure Observation**: Case, theme, viewport, input method, modal state, focus state, accessible labeling, dimensions, containment, caption metrics, and result.
- **Figure Receipt**: Complete browser matrix, inventory totals, failures, and pass sentinel.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Source and generated policy account for exactly 20 meaningful interactive placements and one explicitly decorative non-interactive wordmark.
- **SC-002**: Every meaningful placement exposes exactly one `.docs-figure-trigger`, and every rendered document exposes at most one shared figure dialog with no remaining mdBook checkbox or clone inside an enhanced figure.
- **SC-003**: All 20 figure matrix cells across five representative cases, two themes, and two viewport widths pass geometry, modal, focus, labeling, caption, and containment checks.
- **SC-004**: Trusted pointer hit tests, Enter, Space, forward and reverse Tab containment, Escape, close-button, backdrop, and focus-return tests pass on representative raw HTML and Markdown-generated figures.
- **SC-005**: Every visible caption computes to at least 14 CSS pixels at the default 16-pixel root, at least 4.5:1 contrast, and no clipping at 320 or 1280 CSS pixels.
- **SC-006**: Documentation build, generated policy, browser smoke, accessibility, 200 percent scale, print, blocked-script, UTF-8, mojibake, text-hygiene, and repository gates pass with zero remote dependencies.

## Assumptions

- Native HTML dialog support is available in the currently supported desktop browsers and provides the smallest correct modal and inertness boundary.
- The existing image alternatives are authoritative accessible names and the existing captions are authoritative supporting descriptions.
- The five brand-example placements are meaningful because the Brand Standard asks readers to inspect their rendering on two surfaces.
- The decorative landing wordmark remains adequately represented by the adjacent visually hidden heading.
- Existing mdBook checkbox zoom behavior is insufficient for the complete accessibility contract and may be removed from enhanced figures at runtime while preserving static source rendering.

## Out of Scope

- Adding GLightbox, another third-party image viewer, an npm dependency, or a downloaded browser.
- Editing image pixels, replacing screenshots, changing diagram content, or creating additional assets.
- General table responsiveness, general page overflow remediation, or changes owned by issue #127.
- Application UI, addon UI, game capture, release packaging, or telemetry behavior.
