# Phase 1 Data Model: Brand and UX Polish

This slice is presentation and packaging polish; it introduces no runtime data
structures in the persisted config or the view-model. The "entities" here are
design and asset artifacts and the small in-code token structures that express
them. The persisted config schema and its `schema_version` are unchanged.

## Brand tokens (in code: `src/app/theme.rs`)

A per-theme set of color roles plus shared metrics. Not serialized; compiled
constants.

| Role | Meaning | Dark (default) | Light |
|------|---------|----------------|-------|
| `bg_base` | Window/base surface | ink (approx `#0E1116`) | near-white |
| `bg_panel` | Panels, controls | approx `#151A21` | light panel |
| `bg_elevated` | Hover/active fills | slightly lighter than panel | slightly darker than panel |
| `accent` | Primary accent | teal (approx `#2DD4BF`) | teal (contrast-adjusted) |
| `accent_hover` | Accent hover/active | lighter teal (approx `#5EEAD4`) | darker teal |
| `text` | Primary text | approx `#E6EDF3` | near-black |
| `text_muted` | Labels, secondary | approx `#7D8794` | mid slate |
| `stroke` | Borders/separators | low-contrast slate | low-contrast slate |
| `status_ok` | Running/green | on-palette green | on-palette green |
| `status_warn` | Warn/amber | on-palette amber | on-palette amber |
| `status_err` | Error/signal lost | on-palette red | on-palette red |

Shared metrics (both themes): spacing scale (item spacing, button padding),
corner-radius scale (controls, panels), stroke widths. Exact hex values are fixed
in the brand document during the design sign-off; the table records roles and
approximate values so the theme structure is unambiguous.

**Validation/rules**:
- Every role has a value in both themes (no `None`).
- `accent` vs `bg_base` and `text` vs `bg_base` meet a legible contrast target in
  both themes.
- Status and log-level colors are derived from these roles, not hard-coded inline
  as they are today (`ui.rs` status dot, `log_view.rs` level colors).

## App theme (applied configuration)

Derived from brand tokens into `egui::Visuals` + `egui::style::Spacing` + widget
corner-radius + `egui::FontDefinitions`.

**State transition**: `UiPrefs.theme` (Dark/Light) selects the token set at the
`apply_prefs` seam; switching theme in Settings re-applies visuals and repaints. No
persisted change beyond the existing `theme` pref.

## Brand mark and icon set (assets)

- **Weave-knot master**: `assets/brand/eso-weave-mark.svg` (abstract interlocking
  strands), plus `assets/brand/eso-weave-mark-badge.svg` (theme-safe badged form).
- **Wordmark/banner**: refreshed `assets/eso-weave-banner.png` and
  `assets/eso-weave-logo-*.png`.
- **Rasterized icons** (regenerated from the badge master):
  - `assets/icon.ico` (16, 32, 48, 64, 128, 256) - installer + shortcuts + exe.
  - `packaging/linux/eso-weave.png` (256) - deb hicolor icon.
  - `packaging/appimage/AppDir/eso-weave.png` (256) - AppImage icon.
  - Window icon PNG (bundled via `include_bytes!`) for `egui::IconData`.

**Rules**: all rasters trace to the SVG master via the committed generation recipe;
the mark must not reintroduce the two-fish gold identity (FR-019).

## Installer presentation (WiX)

- **License asset**: `packaging/windows/License.rtf`, proportional/headed, Apache-2.0
  verbatim.
- **Desktop-shortcut choice**: `INSTALLDESKTOPSHORTCUT` property, default `"0"`;
  component `<Condition>` gates creation; persisted via `HKCU\Software\ESO Weave`.
  States: unchecked (default) -> no desktop shortcut; checked -> desktop shortcut
  created. Start Menu shortcut is always present.
- **Wizard art**: `WixUIBannerBmp` (493x58), `WixUIDialogBmp` (493x312).

## Skill-row layout (view unchanged; presentation only)

The seven `SkillRow` view items (from `src/app/mod.rs`, unchanged) render into a
five-column grid: label, active, weave-type, override, value. The value column is
always allocated. No change to `SkillRow` fields, `UiIntent`, or `SkillEdit`.
