# Quickstart: Validating Brand and UX Polish

This is a validation/run guide, not implementation. It proves the slice end to end.

## Prerequisites

- Rust toolchain per `rust-toolchain.toml`.
- Windows for the MSI and exe-icon checks; Linux for the deb/AppImage icon checks.
- ImageMagick and/or `rsvg-convert` if regenerating raster assets from the SVG.

## 0. Design sign-off (gate, before any code)

- Open the rendered visual proof (themed window on dark and light, the mark at
  16/32/48/256px, and the installer wizard mock).
- Confirm the operator approves the exact hex tokens and the mark before egui/WiX
  code is changed. This is a hard gate.

## 1. CI parity (before every Rust-touching commit)

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: all green. The view-model unit tests are unchanged and still pass.

## 2. Run the app (theme, columns, cursor, window icon)

```powershell
cargo run
```

Confirm:
- Dark theme by default: teal-on-ink palette across every surface (no default
  egui grey).
- Settings -> Theme -> Light: the same identity in a light variant, legible.
- Skills section: the active / weave-type / override / value controls line up in
  columns across all seven rows; a row with an override value is the same width as
  one without.
- Hovering any button, checkbox, or dropdown shows a pointer (hand) cursor.
- The title bar and taskbar show the new weave-knot mark.

## 3. Executable icon (Windows)

- Build a release exe and view it in File Explorer:

```powershell
cargo build --release
```

Confirm `target\release\eso-weave.exe` shows the weave-knot icon, not a generic
exe icon.

## 4. Icon legibility

- Inspect `assets/icon.ico` at 16, 32, 48, and 256px on both a light and a dark
  background; confirm the mark stays recognizable and does not degrade into a blob.

## 5. Installer (Windows MSI)

- Build the MSI the same way the release pipeline does (cargo-wix no-build), then
  run it and confirm:
  - License page: full Apache-2.0 text, clean proportional layout, readable.
  - Desktop-shortcut checkbox present and unchecked by default.
  - Finish without checking it: no desktop shortcut on the desktop; Start Menu
    shortcut present. Repeat with it checked: desktop shortcut created.
  - Upgrade over a prior install preserves the user's shortcut choice.
  - Wizard shows branded banner and dialog artwork.

## 6. Linux icon (deb/AppImage)

- Confirm `packaging/linux/eso-weave.png` and
  `packaging/appimage/AppDir/eso-weave.png` are the regenerated weave-knot mark;
  the application-menu launcher shows it.

## 7. Process checks

- `CHANGELOG.md` has dated `### Decisions` bullets for the pinned changes
  (`wix/main.wxs`, `License.rtf`, packaging icons/bitmaps).
- All new/edited text files are UTF-8 without BOM, LF, and contain no em/en dashes.
- `docs/brand/ESO-Weave-Brand-v1.md` and the `docs/plans/plan-002.md` row exist.
