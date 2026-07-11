# Implementation Plan: Brand and UX Polish

**Branch**: `012-brand-ux-polish` (integrates direct to `main`) | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/012-brand-ux-polish/spec.md`

## Summary

Establish an "Arcane teal on ink" brand standard and apply it across the running
app, the runtime/exe icon, and the Windows and Linux installers, replacing the
antique two-fish gold identity. The change is presentation and packaging only: the
unit-tested view-model in `src/app/mod.rs` is untouched. Work is gated by a design
sign-off (a rendered visual proof) before any egui or WiX code changes, then
implemented as a custom egui theme plus a new abstract weave-knot mark, window and
exe icons, aligned skill columns, pointer cursors, and installer polish (readable
license page, opt-in desktop shortcut, branded wizard art).

## Technical Context

**Language/Version**: Rust (workspace pin in `rust-toolchain.toml`).

**Primary Dependencies**: `eframe`/`egui` 0.35 (glow renderer, default_fonts,
x11/wayland). New: a Windows resource crate as a `build.rs` build-dependency
(`winresource`) for exe-icon embedding; the Inter font shipped as a bundled asset
(no runtime crate). Icon rasterization uses ImageMagick and/or `rsvg-convert` at
authoring time (not a runtime dependency). Installer: WiX via `cargo-wix`
(`WixUI_InstallDir`), unchanged toolchain.

**Storage**: N/A (config unchanged; the desktop-shortcut choice persists in the
existing per-user `HKCU\Software\ESO Weave` registry value on Windows).

**Testing**: `cargo test --all --locked`; presentation is validated by the manual
UI quickstart plus visual inspection. No change to the safety-critical test
surfaces.

**Target Platform**: Windows 10/11 x64 and Linux x64.

**Project Type**: Single-crate desktop application.

**Performance Goals**: No regression; theme/font load once at startup, repaint
cadence unchanged (250ms request already in place).

**Constraints**: CI parity (`fmt`/`clippy`/`test`) before commit; text hygiene
(UTF-8 no BOM, LF, no em/en dashes); pinned artifacts (`wix/main.wxs`,
`packaging/**`) require dated `CHANGELOG.md` decisions; direct-to-`main`
integration; the exe-icon build step must be a no-op on non-Windows targets.

**Scale/Scope**: One window, seven skill rows, one installer wizard, three icon
targets plus wordmark/banner; one new brand document and one new theme module.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Full spec-kit sequence is running for this
  slice (`specs/012-brand-ux-polish/`). It traces to the master specification's
  GUI section (theme dark-default/light, colorized logs), the application-icon
  requirement (`assets/icon.ico` in the MSI), and the installer/first-run slice
  (011). A `docs/plans/plan-002.md` row registers slice order.
- **II. Safety-Critical Surfaces Are Sacrosanct**: PASS. No safety surface is
  touched: no input hooks, suppression scope, worker-thread handoff, beacon
  uninstall, AddOns discovery, or fishing degrade logic changes. Presentation only.
- **III. Test-First With Explicit Seams**: PASS (scoped). The view-model and its
  unit tests are unchanged; `ui.rs` is the manually validated thin layer by design.
  New pure helpers (theme token mapping) get unit tests where they carry logic;
  the WiX/asset changes are validated by build and manual inspection.
- **IV. CI Parity Before Every Commit**: PASS. `fmt`/`clippy`/`test` run in the
  foreground before each commit that touches Rust; doc/asset-only commits still
  obey text hygiene.
- **V. Bounded Scope: Outside The Game**: PASS. No new in-game interaction; the
  PixelBeacon contract is unchanged.
- **Pinned artifacts**: `wix/main.wxs`, `packaging/windows/License.rtf`,
  `packaging/linux/**`, `packaging/appimage/**` change under dated `CHANGELOG.md`
  decisions with the operator's authorization (the plan approval plus the
  pre-push halt). No release/tag actions in this slice.

Result: PASS, no violations, Complexity Tracking not required.

## Project Structure

### Documentation (this feature)

```text
specs/012-brand-ux-polish/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (brand tokens, theme, icon set)
├── quickstart.md        # Phase 1 output (validation guide)
├── contracts/
│   └── brand-and-ui.md  # UI + installer presentation contract
├── checklists/
│   ├── requirements.md  # Spec quality (from specify)
│   └── brand-ux.md      # Requirements-quality checklist (from checklist)
└── tasks.md             # Phase 2 output (/speckit-tasks)
```

### Source Code (repository root)

```text
src/
├── main.rs              # window IconData via ViewportBuilder::with_icon
├── build.rs             # NEW: winresource exe-icon embed (Windows-only)
└── app/
    ├── mod.rs           # view-model (UNTOUCHED)
    ├── ui.rs            # apply theme, egui::Grid skill table, pointer cursors
    ├── theme.rs         # NEW: brand tokens -> egui::Visuals + spacing/rounding + fonts
    ├── log_view.rs      # retune log-level colors to palette
    └── beacon_light.rs  # (status colors sourced from theme)

assets/
├── brand/               # NEW: weave-knot SVG master + badged variant + wordmark
│   └── fonts/           # NEW: Inter TTF + OFL license
├── icon.ico             # regenerated from new mark
├── eso-weave-banner.png # refreshed
└── eso-weave-logo-*.png # refreshed

packaging/
├── windows/License.rtf  # reformatted (proportional, headed)
├── windows/*.bmp        # NEW: WixUIBannerBmp + WixUIDialogBmp brand art
├── linux/eso-weave.png  # regenerated
└── appimage/AppDir/eso-weave.png  # regenerated

wix/main.wxs             # desktop-shortcut opt-in + wizard bitmaps + license var
docs/brand/ESO-Weave-Brand-v1.md   # NEW artifact of record
docs/plans/plan-002.md             # NEW slice registration
scripts/ or assets/brand/README    # icon-generation recipe (reproducible)
```

**Structure Decision**: Single-crate desktop app; presentation changes localized to
`src/main.rs`, `src/app/ui.rs`, and a new `src/app/theme.rs`, plus `build.rs`.
Assets and packaging follow existing directory conventions. No workspace split.

## Complexity Tracking

Not required: Constitution Check passes with no violations.
