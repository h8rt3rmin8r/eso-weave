# Contract: Brand, UI, and Installer Presentation

This is a UI/presentation contract for a desktop app (no network or API surface).
It defines the observable, testable presentation guarantees this slice must meet.

## Brand standard document

- `docs/brand/ESO-Weave-Brand-v1.md` exists and defines: the dark and light color
  tokens (exact hex per role), the primary typeface (Inter, OFL 1.1) and its usage,
  a spacing and corner-radius scale, and iconography and accent-usage rules.
- The shipped theme tokens and icon assets match the document.

## Application window and theme

- The window sets a brand window icon (title bar and taskbar) via the GUI
  framework's viewport icon facility.
- Default theme is dark; a light theme is available via the existing Settings
  toggle. Both express the teal-on-ink identity with legible contrast.
- No control renders with default framework styling; surfaces, accent, text,
  spacing, and corner radius follow the tokens.
- Status indicators and log-level colors are sourced from the theme tokens and
  remain legible in both themes.

## Skill table

- The seven rows render in one aligned grid with columns: label, active, weave
  type, override, value.
- Each column shares a common left edge across all rows.
- The value column is always allocated: a row with an override value and a row
  without one have the same width.
- Row semantics (which intents fire on edit) are unchanged from the current
  view-model.

## Cursor feedback

- Every clickable control (buttons, checkboxes, combo-box headers) shows a pointer
  (hand) cursor while hovered.
- Non-interactive labels and surfaces keep the default cursor.

## Icon set

- The mark is legible and recognizable at 16, 32, 48, and 256px on both light and
  dark backgrounds (self-contained badged form).
- The same mark appears on: the window/taskbar, the Windows exe file icon, the
  Start Menu shortcut, the opted-in desktop shortcut, and the Linux launcher.

## Installer (Windows MSI)

- The license page shows the full Apache-2.0 text as clean, proportional, spaced
  text (no raw small monospace).
- A desktop-shortcut checkbox defaults to unchecked; the desktop shortcut is
  created only when checked. The Start Menu shortcut is always created.
- The choice is honored on install and not overridden on upgrade.
- The wizard shows branded banner and dialog artwork.

## Invariants (must not change)

- The persisted config schema and `schema_version` are unchanged.
- The view-model (`src/app/mod.rs`) behavior and its unit tests are unchanged.
- No safety-critical surface (input hooks, suppression scope, worker-thread
  handoff, beacon uninstall, AddOns discovery, fishing degrade) is touched.
- Linux desktop integration is unchanged except for the refreshed icon; the
  desktop-shortcut opt-out is Windows-only.
