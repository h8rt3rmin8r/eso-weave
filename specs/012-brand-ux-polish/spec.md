# Feature Specification: Brand and UX Polish

**Feature Branch**: `012-brand-ux-polish`

**Created**: 2026-07-11

**Status**: Draft

**Input**: User description: "Establish a professional 'Arcane teal on ink' brand
standard and apply it across the desktop app UI, the runtime/exe icon, and the
Windows and Linux installers, keeping the ESO lineage but reading ultra-modern and
clean instead of antique."

## Clarifications

### Session 2026-07-11

- Q: Which typeface is bundled as the primary UI font? (decided under autopilot)
  -> A: Inter, under the SIL Open Font License 1.1, bundled with the application;
  the wordmark uses a tracked-out uppercase treatment of the same family.
- Q: How is the desktop shortcut made opt-in in the Windows installer? (decided
  with the operator) -> A: The desktop shortcut is its own installer feature, off
  by default (not installed unless the user selects it in the Custom Setup step),
  rather than a single checkbox on the install page. A single checkbox would
  require replacing the built-in installer dialog set, which cannot be validated
  without a local installer build. On a major upgrade the user chooses again in
  the wizard (default off); the choice is not silently changed within an install.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - A polished, cohesive application window (Priority: P1)

A user launches ESO Weave and sees a coherent, professionally styled window: a
consistent color scheme and typography, skill rows whose controls line up in tidy
columns, and clickable controls that respond to the mouse with a pointer cursor.
The window carries the product's branding mark in its title bar and the taskbar.

**Why this priority**: The "app looks sloppy" complaint is the core of this slice.
A cohesive themed window with aligned controls is the single most visible
improvement and stands on its own as a deliverable.

**Independent Test**: Launch the app on a dark-themed system and a light-themed
system; confirm the themed palette, aligned skill columns, pointer cursor on
hover, and the window/taskbar icon are all present, with no dependence on the
installer or the brand document.

**Acceptance Scenarios**:

1. **Given** the app is running in its default (dark) theme, **When** the user
   views the main window, **Then** surfaces, text, accent, and controls follow a
   single documented palette rather than default framework styling.
2. **Given** the Skills section is shown, **When** the user scans down the seven
   rows (Skill 1 to 5, Ultimate, Synergy), **Then** the active toggle, weave-type
   selector, override toggle, and value field each align in a fixed column, and
   rows with and without an override value are the same width.
3. **Given** the pointer hovers a button, checkbox, or selector, **When** it is
   over the interactive region, **Then** the cursor changes to a pointer (hand).
4. **Given** the app window is open, **When** the user looks at the title bar and
   the taskbar, **Then** the product branding mark is displayed in both.
5. **Given** the user switches to the light theme in Settings, **When** the theme
   applies, **Then** the same brand identity is expressed in a light variant with
   legible contrast.

---

### User Story 2 - A modern, legible brand mark everywhere it appears (Priority: P1)

The product uses one modern brand mark, legible from a 16px taskbar icon up to a
256px shell icon and on both light and dark backgrounds. The same mark appears on
the application window, the Windows executable file in the file manager, the
Start Menu and desktop shortcuts, and the Linux application-menu launcher.

**Why this priority**: The current mark collapses into an unreadable blob at small
sizes and is low-contrast on some themes. A single legible mark is foundational to
every other branded surface.

**Independent Test**: Inspect the mark at 16, 32, 48, and 256px on light and dark
backgrounds and confirm it stays recognizable and legible; confirm the same mark
appears on the executable in the file manager and on each shortcut/launcher.

**Acceptance Scenarios**:

1. **Given** the icon is rendered at 16, 32, 48, and 256px, **When** viewed on
   both light and dark backgrounds, **Then** the mark remains recognizable and its
   silhouette does not degrade into an indistinct shape.
2. **Given** the Windows executable is viewed in the file manager, **When** the
   user looks at its icon, **Then** it shows the product mark rather than a generic
   executable icon.
3. **Given** the app is installed, **When** the user opens the Start Menu shortcut,
   the (opted-in) desktop shortcut, and the Linux application menu, **Then** each
   shows the same product mark.

---

### User Story 3 - A trustworthy, respectful installer experience (Priority: P2)

A user runs the Windows installer and reads a clean, legible license page, is
asked whether to place a desktop shortcut (default off), and moves through a wizard
whose artwork is branded rather than generic.

**Why this priority**: First impressions at install time matter, but the installer
is used once and depends on the mark from User Story 2 being ready first.

**Independent Test**: Build and run the installer; confirm the license page is
readable, the desktop-shortcut choice defaults to off and is honored, and the
wizard artwork is branded.

**Acceptance Scenarios**:

1. **Given** the installer license page is shown, **When** the user reads it,
   **Then** the full license text is presented in a clean, proportional, well
   spaced layout rather than raw small monospace text.
2. **Given** the installer offers a desktop shortcut, **When** the user reaches
   that choice, **Then** it defaults to not creating one, and the shortcut is
   created only if the user opts in.
3. **Given** the install completes without opting in, **When** the user looks at
   the desktop, **Then** no desktop shortcut exists, while the Start Menu shortcut
   is present.
4. **Given** the wizard is displayed, **When** the user views its banner and
   dialog artwork, **Then** it shows product branding rather than default
   installer art.

---

### User Story 4 - A brand standard that keeps future work consistent (Priority: P3)

A contributor opens a single brand standard document and finds the authoritative
color tokens, typography, spacing, and iconography rules, so later features apply
the identity without re-deriving it.

**Why this priority**: Durable value and prevents drift, but it enables future
consistency rather than delivering an immediate user-visible change.

**Independent Test**: Open the brand document and confirm it defines the palette
(dark and light), typography, spacing/radius scale, and iconography usage in a
form a contributor can apply directly.

**Acceptance Scenarios**:

1. **Given** the brand document exists, **When** a contributor needs a color or
   type decision, **Then** the document provides the authoritative token or rule.
2. **Given** the app theme and the icon assets, **When** compared to the brand
   document, **Then** they match the documented tokens and rules.

---

### Edge Cases

- The mark must stay legible against an arbitrary desktop wallpaper behind a
  taskbar icon; a self-contained badged form is used so it never relies on the
  surface behind it.
- The light theme must preserve sufficient contrast for status colors (running,
  suspended, signal lost) and log levels that were originally tuned for dark.
- The exe icon must embed on Windows without breaking the Linux build (the
  embedding step is Windows-only and must be a no-op elsewhere).
- Upgrading from the prior version must not resurrect a desktop shortcut the user
  declined, nor orphan a shortcut created by the prior installer.
- The desktop-shortcut opt-out applies to the Windows installer only; Linux
  packages ship an application-menu launcher, not a desktop-surface shortcut, and
  are unaffected.

## Requirements *(mandatory)*

### Functional Requirements

Brand standard:

- **FR-001**: The project MUST provide a brand standard document as the artifact
  of record, defining color tokens for a dark (default) and a light theme, a
  primary UI typeface, a spacing and corner-radius scale, and iconography and
  accent-usage rules.
- **FR-002**: The brand palette MUST be "Arcane gold on ink": near-black ink
  surfaces with a warm gold primary action color (bridging the ESO heritage), a
  teal supporting accent, and muted slate text. It modernizes the antique-gold
  identity rather than discarding it.

Brand mark and icons:

- **FR-003**: The project MUST define a new logo mark shaped as a "W" (for Weave)
  built from two monoline strands (one gold, one teal) that meet at a shared apex
  and work in tandem, as a scalable master, replacing the prior two-fish ouroboros
  mark.
- **FR-004**: The mark MUST have a self-contained, theme-safe form (a contained
  badge) that stays legible at 16, 32, 48, and 256px on light and dark
  backgrounds.
- **FR-005**: The Windows multi-resolution icon, the Linux package icon, and the
  AppImage icon MUST be regenerated from the new mark.
- **FR-006**: Every existing brand asset in `assets/` (wordmark, banner, logo
  variants) MUST be reworked to the new mark, typography, and palette; no stale
  antique-gold artwork remains.
- **FR-006a**: The project MUST add a GitHub social-share image (1280x640) derived
  from the new brand, committed under `assets/` (it is not represented in the repo
  today); it is applied to the repository social-preview setting.

Application window and theme:

- **FR-007**: The application window MUST display the brand mark as its window
  icon (title bar and taskbar).
- **FR-008**: The Windows executable MUST embed the brand mark as its file icon.
- **FR-009**: The application MUST apply a custom theme in the brand palette for
  both the default dark and the optional light mode, replacing default framework
  visuals, including consistent surfaces, accent, spacing, and corner radius, and
  a bundled primary UI typeface.
- **FR-010**: Status indicators (for example running, suspended, signal lost) and
  log-level colors MUST be retuned to remain legible and on-palette in both
  themes.
- **FR-011**: The seven skill rows MUST align their controls (active toggle,
  weave-type selector, override toggle, and value field) in fixed columns, with
  the value cell reserved so rows with and without an override value share the
  same width.
- **FR-012**: All clickable controls (buttons, toggles, selectors) MUST present a
  pointer (hand) cursor while hovered.

Installer:

- **FR-013**: The Windows installer license page MUST present the full, unmodified
  Apache-2.0 license text in a clean, proportional, well spaced layout.
- **FR-014**: The Windows installer MUST offer the desktop shortcut as an opt-in
  choice that defaults to not creating one; the Start Menu shortcut remains always
  installed.
- **FR-015**: The desktop shortcut MUST be created only when the user opts in
  (a default-off installer feature selected in the Custom Setup step); it is never
  created without that selection.
- **FR-016**: The Windows installer wizard MUST use branded banner and dialog
  artwork in place of the default installer artwork.

Cross-cutting and process:

- **FR-017**: Changes to pinned packaging artifacts (the Windows installer
  definition, license asset, and packaging icons/artwork) MUST be accompanied by a
  dated decision recorded in the changelog.
- **FR-018**: The correctness-bearing, unit-tested application view-model MUST
  remain behaviorally unchanged; this slice changes presentation only.
- **FR-019**: The new mark and refreshed assets MUST NOT reintroduce the antique
  two-fish gold identity as the product's primary mark.

### Key Entities

- **Brand standard**: The authoritative definition of palette (dark and light),
  typography, spacing/radius, and iconography rules; referenced by the app theme
  and all icon assets.
- **Brand mark**: The abstract weave-knot logo in a scalable master plus a
  theme-safe badged form; source for all rasterized icons.
- **Icon set**: The platform icon renderings (Windows multi-resolution, Linux,
  AppImage) and the embedded executable icon derived from the mark.
- **App theme**: The applied visual configuration (surfaces, accent, text,
  spacing, radius, typeface, status/log colors) for dark and light modes.
- **Installer presentation**: The license page rendering, the desktop-shortcut
  opt-in choice, and the wizard artwork.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: On both a dark-themed and a light-themed system, the running app
  presents a single cohesive palette and typeface across every surface, with no
  control still showing default framework styling.
- **SC-002**: Across all seven skill rows, each control column shares a common
  left edge, and a row with an override value is the same width as a row without
  one.
- **SC-003**: Every clickable control shows a pointer cursor on hover; no clickable
  control leaves the default arrow cursor.
- **SC-004**: The brand mark is recognizable and legible at 16, 32, 48, and 256px
  on both light and dark backgrounds.
- **SC-005**: The window, taskbar, executable file, Start Menu shortcut, opted-in
  desktop shortcut, and Linux launcher all display the same brand mark.
- **SC-006**: In the installer, the license page is fully readable as flowing
  proportional text, the desktop-shortcut choice defaults to off and is honored,
  and the wizard shows branded artwork.
- **SC-007**: A contributor can determine any brand color or type decision solely
  from the brand standard document, and the shipped theme and icons match it.
- **SC-008**: The application's existing behavior and its automated verification
  gate are unaffected by the presentation changes.

## Assumptions

- The visual direction is fixed as "Arcane teal on ink" with an abstract
  weave-knot mark; these were chosen by the operator and are not open questions in
  this slice.
- A rendered visual proof (a static mockup of the themed window on light and dark,
  the mark at all sizes, and the installer wizard) is produced and approved before
  the app and installer code are changed; final exact color tokens are fixed at
  that step within the stated palette.
- The GUI framework's theming and window-icon facilities are sufficient to express
  the palette, typeface, spacing, and radius without changing the window's overall
  layout structure or the view-model.
- Bundling one open-license UI typeface (Inter, SIL OFL 1.1) with the application
  is acceptable for distribution; its license file ships alongside it.
- The desktop-shortcut opt-out is scoped to the Windows installer; Linux desktop
  integration is unchanged aside from the refreshed icon.
- This slice is presentation and packaging polish; it introduces no new runtime
  behavior, input handling, or safety-critical surface, and it traces to the
  master specification's GUI, application-icon, and installer requirements.
