# Phase 0 Research: Packaging and Distribution

All decisions were made under the Build-Phase Autopilot Protocol against the
constitution, the master specification (section 13), and the pinned release
pipeline. None were escalated.

## Decision: Conform to the pinned pipeline; create only the referenced files

- **Decision**: Treat `.github/workflows/release.yml` and `docs/releasing.md` as
  the fixed contract and create exactly the files they reference that are missing:
  `scripts/changelog-section.sh`, `scripts/linux-build-deps.sh`, `release.toml`,
  `wix/main.wxs`, cargo-deb metadata, `packaging/appimage/AppDir`, and
  `assets/icon.ico`. Do not modify the pinned files.
- **Rationale**: The pipeline is already pinned and correct; the slice's job is to
  complete it, not redesign it. Conforming avoids drift and keeps the tag-version
  check and changelog gate intact.
- **Alternatives considered**: Rewriting the workflow to match hand-made packaging
  (rejected: modifies a pinned artifact unnecessarily and risks the gates). Using a
  different packaging layout (rejected: the pinned steps hardcode the paths).

## Decision: WiX source at `wix/main.wxs` (cargo-wix default)

- **Decision**: Place the MSI source at `wix/main.wxs`, the location `cargo wix`
  resolves by default, so the pinned `cargo wix --no-build --nocapture` (no path
  argument) finds it. Point `[package.metadata.wix]` at `assets/icon.ico` for the
  product icon. Hand-write the `main.wxs` from the standard cargo-wix template:
  a single `File` for the application binary, a Start Menu shortcut, upgrade-in-place
  via a fixed `UpgradeCode` and `MajorUpgrade`, and install under Program Files.
- **Rationale**: The pinned step has no path argument, so the file must be at the
  tool default. cargo-wix is not installed here to run `cargo wix init`, so the
  well-known template is hand-written and validated as well-formed XML; the real
  MSI build proves it in CI.
- **Alternatives considered**: `packaging/wix/` (rejected: the pinned step would
  not find it without a path argument). Installing cargo-wix to run `init`
  (rejected: unnecessary network/build; the template is standard and the CI build
  is the real proof).

## Decision: Generate `assets/icon.ico` with ImageMagick from the logo art

- **Decision**: Generate a multi-resolution `assets/icon.ico` from
  `assets/eso-weave-logo-clear.png` using the locally available ImageMagick
  (`magick ... -define icon:auto-resize=256,128,64,48,32,16`).
- **Rationale**: A valid Windows icon with standard sizes is required for the MSI
  and app icon. ImageMagick is available locally and produces a correct
  multi-size ICO; the result is validated by its ICO magic bytes and committed as a
  binary asset.
- **Alternatives considered**: A one-off Rust program using the `image` crate
  (works but heavier than a single `magick` call). Embedding a single PNG as the
  icon (rejected: fewer sizes, worse small-icon rendering).

## Decision: cargo-deb metadata in `Cargo.toml`, packaging a udev rule

- **Decision**: Add `[package.metadata.deb]` with the binary asset, the desktop
  entry, the icon, and the `packaging/linux/70-eso-weave-uinput.rules` udev rule
  (installed under `/usr/lib/udev/rules.d/`), plus a short extended description and
  the appropriate section/priority. Document the evdev requirement (input-group
  membership or the packaged rule) in the README.
- **Rationale**: cargo-deb reads its config from `Cargo.toml`, and section 13
  requires the `.deb` and the evdev-permission coverage. The uinput rule grants the
  synthesis path (`/dev/uinput`) group access; reading `/dev/input/event*` uses the
  standard `input` group, documented alongside.
- **Alternatives considered**: A standalone `debian/` directory (rejected:
  cargo-deb uses `Cargo.toml` metadata, which the pinned `cargo deb --no-build`
  expects). Shipping no rule and documenting only the group (rejected: the rule is
  the turnkey option and section 13 asks for "a provided udev rule").

## Decision: AppDir under `packaging/appimage/AppDir` matching the pinned step

- **Decision**: Provide `packaging/appimage/AppDir` with an `AppRun` that execs
  `usr/bin/eso-weave`, an `eso-weave.desktop`, and `eso-weave.png`. The pinned step
  installs the built binary into `AppDir/usr/bin/eso-weave` and runs appimagetool.
- **Rationale**: appimagetool assembles an AppImage from an AppDir that contains an
  `AppRun`, a top-level `.desktop`, and an icon; the pinned step supplies the
  binary, so the AppDir carries the rest.
- **Alternatives considered**: linuxdeploy plugins (rejected: the pinned step uses
  appimagetool directly on a prepared AppDir).

## Decision: `changelog-section.sh` matches a bracketed heading and prints the body

- **Decision**: `changelog-section.sh <heading>` prints the lines between
  `## [<heading>...]` and the next `## ` heading, excluding both, with empty output
  when the section is absent. It matches on the bracketed version, tolerating a
  trailing ` - DATE`. Implemented with `awk` for portability.
- **Rationale**: This is exactly what the pinned verify gate and release-notes step
  need (`changelog-section.sh "$version"`), and it is runnable and testable now
  against the live `CHANGELOG.md` (`Unreleased`).
- **Alternatives considered**: `sed` range extraction (workable but awkward for the
  bracket-plus-trailing-date match). A Rust helper (rejected: a shell script is what
  the pinned workflow invokes).

## Decision: `linux-build-deps.sh` is the single dependency source

- **Decision**: An `apt-get`-based script installing the GUI windowing and GL
  libraries and the X11/XCB, xkbcommon, Wayland, and evdev/udev development
  packages the eframe/glow GUI and the input and pixel-bus backends need, run with
  `sudo` as the pinned workflow does.
- **Rationale**: The pinned workflow and `releasing.md` state the dependency list
  lives only in this script so CI and developer machines cannot drift.
- **Alternatives considered**: Inlining the list in the workflow (rejected:
  duplicates it and the workflow is pinned). A distro-agnostic installer (rejected:
  CI is Ubuntu; `apt-get` is sufficient and simple).

## Decision: `release.toml` drives the rollover; validated with `--dry-run`

- **Decision**: `release.toml` configures cargo-release to bump `Cargo.toml`,
  rewrite `CHANGELOG.md` via `pre-release-replacements` (rename `## [Unreleased]`
  to `## [X.Y.Z] - DATE` and open a fresh `## [Unreleased]`), commit as
  `release: v{{version}}`, and tag `v{{version}}` and push. Validate here with
  `cargo release <next> --dry-run` (no `--execute`), which the installed
  cargo-release supports.
- **Rationale**: This matches the procedure `docs/releasing.md` documents, and
  `--dry-run` proves the config parses and the replacements resolve without cutting
  a tag.
- **Alternatives considered**: Driving the rollover from the workflow (rejected:
  `releasing.md` says the rollover happens locally before the tag push so the
  workflow never writes to `main`).
