# Phase 0 Research: Brand and UX Polish

All decisions below resolve the Technical Context so implementation has no open
unknowns. No item remains marked NEEDS CLARIFICATION.

## Window icon (runtime, cross-platform)

- **Decision**: Set the window icon in `src/main.rs` by building
  `eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_icon(...) }`
  where the icon is an `egui::IconData { rgba, width, height }` decoded at startup
  from a bundled PNG (`include_bytes!`) using the `image` crate with only the `png`
  feature.
- **Rationale**: `image` 0.25 and `png` 0.18 are already in `Cargo.lock`; a
  `default-features = false, features = ["png"]` direct dependency keeps codecs
  minimal and avoids enabling eframe's broader `image` feature. Decoding to RGBA is
  a three-line helper, testable in isolation.
- **Alternatives**: Enable eframe's `image` feature and use
  `eframe::icon_data::from_png_bytes` (pulls the full `image` default codecs,
  heavier); decode with the raw `png` crate (works but more boilerplate than
  `image::load_from_memory(..).to_rgba8()`).

## Executable icon (Windows file icon)

- **Decision**: Add a `build.rs` that, only under `cfg(windows)`, uses the
  `winresource` crate to embed `assets/icon.ico` as the exe icon. Declare it under
  `[target.'cfg(windows)'.build-dependencies]` so non-Windows builds skip it
  entirely; the `build.rs` body is a no-op on other targets.
- **Rationale**: `winresource` is the maintained successor to `winres`, integrates
  as a build-dependency, and is the standard way to set a Windows exe icon without
  a linker script. Windows-only scoping keeps the Linux build unaffected
  (Constitution: cross-platform via per-target config).
- **Alternatives**: `winres` (unmaintained), `embed-resource` (needs a hand-written
  `.rc` file), manual linker flags (fragile).

## egui theme, fonts, spacing (dark default + light)

- **Decision**: New `src/app/theme.rs` defines brand tokens (teal-on-ink) and
  returns a fully configured `egui::Visuals` per theme, plus `egui::style::Spacing`
  and widget corner-radius, applied at the existing `apply_prefs` seam
  (`src/app/ui.rs:82`) in place of bare `Visuals::dark()/light()`. Fonts are
  installed once via `egui::FontDefinitions` + `ctx.set_fonts(..)` at first paint,
  registering bundled Inter (`include_bytes!`) as the proportional family with the
  existing default fonts kept as fallback.
- **Rationale**: `apply_prefs` already gates on changed prefs and is the natural
  one-shot place to set visuals; fonts must be set on the context once. Keeping
  default fonts as fallback preserves glyph coverage for the log text.
- **Alternatives**: `ctx.set_style` with a hand-built `Style` (more surface than
  needed); a third-party egui theme crate (unnecessary dependency; the palette is
  small and fixed).

## Skill-table column alignment

- **Decision**: Replace the per-row `ui.horizontal(...)` loop (`src/app/ui.rs:199`)
  with a single `egui::Grid::new("skills").show(ui, ..)`, one `ui.end_row()` per
  skill, columns: label, active toggle, weave-type selector, override toggle, value.
  The value cell is always allocated (render the `DragValue` when an override is
  set, otherwise allocate an equally sized blank/disabled placeholder) so override
  and non-override rows share width.
- **Rationale**: `egui::Grid` enforces shared column x-positions across rows, which
  the independent horizontals cannot. Reserving the value cell fixes the ragged
  right edge.
- **Alternatives**: `ui.columns(n, ..)` (less control over per-cell widgets);
  manual `ui.allocate_space` (reimplements Grid).

## Pointer cursor on interactive controls

- **Decision**: A small helper (extension-style fn) applies
  `.on_hover_cursor(egui::CursorIcon::PointingHand)` to the `Response` of buttons,
  checkboxes, and combo-box headers throughout `ui.rs`.
- **Rationale**: egui does not switch to a hand cursor for buttons by default;
  `on_hover_cursor` on the returned `Response` is the supported per-widget hook.
- **Alternatives**: Global cursor override (would misfire over non-interactive
  areas); restyling only (does not change the cursor).

## Brand mark and icon generation

- **Decision**: Author the abstract weave-knot as an SVG master under
  `assets/brand/` plus a badged theme-safe variant (rounded-square ink badge with
  the teal knot and a subtle stroke so it reads on light and dark). Rasterize with
  ImageMagick (`magick`) and/or `rsvg-convert` to rebuild `assets/icon.ico`
  (16/32/48/64/128/256), `packaging/linux/eso-weave.png` and
  `packaging/appimage/AppDir/eso-weave.png` (256), and refresh the wordmark/banner.
  Commit a reproducible generation recipe (a documented script) so the raster
  assets can be regenerated from the SVG.
- **Rationale**: A vector master is the missing source today; committing it plus a
  recipe makes every raster reproducible. A self-contained badge satisfies the
  16px-on-any-background legibility requirement (FR-004).
- **Alternatives**: Hand-edit rasters (not reproducible); keep the two-fish mark
  (rejected by the brand direction, FR-019).

## Installer: license page reformat

- **Decision**: Rewrite `packaging/windows/License.rtf` with a proportional font
  (e.g., a common sans/serif face available on Windows), real heading styling, and
  paragraph spacing, preserving the full Apache-2.0 text verbatim; keep the stock
  `WixUILicenseRtf` variable pointing at it.
- **Rationale**: The jank is purely the 8pt Courier RTF; a well-formed RTF renders
  cleanly in the existing WixUI license pane with no WiX flow change.
- **Alternatives**: Custom license dialog (unneeded); shorter EULA (rejected in
  favor of verbatim Apache text).

## Installer: desktop-shortcut opt-in (WiX v3)

- **Decision**: The MSI uses WiX v3 (`schemas.microsoft.com/wix/2006/wi`,
  `WixUI_InstallDir`). Add `<Property Id="INSTALLDESKTOPSHORTCUT" Value="0"
  Secure="yes" />`, gate the `ApplicationDesktopShortcut` component with
  `<Condition>INSTALLDESKTOPSHORTCUT = "1"</Condition>`, and surface a checkbox
  (default unchecked) bound to that property on the install-location step by
  customizing the `InstallDirDlg` (a project-local dialog copy that adds one
  CheckBox control). The Start Menu shortcut stays unconditional. Persist the
  choice through the existing per-user `HKCU\Software\ESO Weave` registry value so
  the component's install state is stable across upgrade.
- **Rationale**: A single checkbox on a page the user already sees is the least
  surprising "ask me" UX; component `<Condition>` is the canonical WiX v3 gate.
- **Alternatives**: Make the shortcut its own `Feature` with `Level="2"` shown via
  `WixUI_FeatureTree` (works but shows a feature tree instead of a simple
  checkbox); ExitDialog checkbox (too late; the shortcut is authored during
  `InstallExecute`). The FeatureTree route is the fallback if the dialog copy is
  brittle in the pinned cargo-wix/WiX toolchain.

## Installer: branded wizard art

- **Decision**: Add `<WixVariable Id="WixUIBannerBmp" Value="packaging\windows\banner.bmp" />`
  (493x58) and `<WixVariable Id="WixUIDialogBmp" Value="packaging\windows\dialog.bmp" />`
  (493x312), authored from the brand system as BMPs.
- **Rationale**: These are the documented WixUI override points; no dialog surgery
  needed. Sizes are the WiX v3 standard.
- **Alternatives**: None needed.

## Bundled typeface

- **Decision**: Inter, SIL Open Font License 1.1, shipped as a TTF under
  `assets/brand/fonts/` with its OFL license text alongside.
- **Rationale**: Modern, highly legible UI face; OFL permits redistribution inside
  the binary and packages. Recorded in the spec Clarifications.
- **Alternatives**: Geist (also OFL, but it is the parent-company font in the
  installed house-style skills, not a product-neutral choice); system fonts (not
  reproducible across platforms).

## Process and pinned artifacts

- **Decision**: `wix/main.wxs`, `packaging/windows/License.rtf`, the new
  `packaging/windows/*.bmp`, and the regenerated `packaging/linux` and
  `packaging/appimage` PNGs are pinned; each change is recorded as a dated
  `### Decisions` bullet in `CHANGELOG.md` under `[Unreleased]`. The design
  sign-off (a rendered visual proof) precedes any egui/WiX code and fixes the final
  exact hex tokens within the stated palette.
- **Rationale**: Matches the Constitution's pinned-artifact governance and the
  operator's mockup-first instruction.
