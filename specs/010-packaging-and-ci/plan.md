# Implementation Plan: Packaging and Distribution

**Branch**: `010-packaging-and-ci` | **Date**: 2026-07-11 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/010-packaging-and-ci/spec.md`

## Summary

Create the packaging artifacts that the already-pinned `release.yml` and
`releasing.md` reference but that do not yet exist, so the release pipeline is
complete: a WiX MSI source at the cargo-wix default location, `assets/icon.ico`
generated from the existing logo art, cargo-deb metadata in `Cargo.toml` with a
desktop entry, icon, and a packaged udev rule, an AppImage `AppDir`, the two
supporting scripts (`changelog-section.sh`, `linux-build-deps.sh`), `release.toml`
for cargo-release, and Linux evdev-permission documentation. The pinned
`release.yml`, `releasing.md`, and `rust-toolchain.toml` are not modified. The MSI,
`.deb`, and AppImage are built only by the release CI on a tag; here, verification
is static consistency with the pinned workflow, the scripts running,
`cargo release --dry-run` accepting `release.toml`, a valid icon, a clean release
build, and CI parity. Creating the pinned `scripts/**`, `packaging/**`, and
`release.toml` is recorded as dated CHANGELOG decisions. No tag is cut.

## Technical Context

**Language/Version**: Rust 1.96.0, edition 2021 (unchanged). This slice adds no
Rust code; it adds `Cargo.toml` metadata, shell scripts, a WiX source, an AppDir,
a udev rule, an icon, and `release.toml`.

**Primary Dependencies**: The pinned pipeline mandates cargo-wix (MSI), cargo-deb
(`.deb`), appimagetool (AppImage), and cargo-release (rollover); those run in CI
or as an operator action. Local tooling used for verification here: ImageMagick
(`magick`) to generate the icon, and the installed `cargo release` (`--dry-run`)
to validate `release.toml`.

**Storage**: N/A (no runtime data). The MSI writes only under its install
location and never to game or Documents directories.

**Testing**: `cargo test` (Rust unchanged, so the suite is unaffected).
Packaging-specific verification: `bash -n` on the scripts, running
`changelog-section.sh` against `CHANGELOG.md`, `cargo release --dry-run`,
confirming `assets/icon.ico` has the ICO magic, well-formedness of the WiX XML and
the desktop entry, and `cargo build --release --bin eso-weave`.

**Target Platform**: Windows 10 and 11 x64 (MSI), Linux x64 (`.deb`, AppImage).

**Project Type**: Single desktop-application crate (unchanged).

**Performance Goals**: N/A.

**Constraints**: Do not modify the pinned `release.yml`, `releasing.md`, or
`rust-toolchain.toml`. Do not cut a tag or run cargo-release with `--execute`.
Record creation of pinned artifacts as dated CHANGELOG decisions. Keep the version
single-sourced in `Cargo.toml`.

**Scale/Scope**: One WiX source, one icon, cargo-deb metadata, one AppDir, one
udev rule, two scripts, one `release.toml`, and documentation.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

- **I. Spec-Driven Development**: PASS. Derived from `spec.md` (master spec section
  13) and the pinned pipeline, bounded by `docs/plans/plan-001.md` slice 010.
- **II. Safety-Critical Surfaces**: PASS. No safety-critical code surface is
  touched. The MSI's never-write-to-game-or-Documents guarantee aligns with the
  bounded-scope principle.
- **III. Test-First With Explicit Seams**: PASS in spirit. This slice adds no Rust
  logic to unit-test; the verifiable behaviors (the changelog extractor, the
  release config) are exercised by running them, and the rest is static
  consistency with the pinned workflow. The one script with real logic
  (`changelog-section.sh`) is run against the actual changelog.
- **IV. CI Parity Before Every Commit**: PASS. `cargo fmt`, `clippy`, and `test`
  are unaffected by the metadata and file additions and are run before commit.
- **V. Bounded Scope: Outside The Game**: PASS. Packaging installs only the
  application; the MSI never touches game or Documents directories.
- **Platform and Text Hygiene Constraints**: PASS. Shell scripts and configs are
  UTF-8 without BOM, LF, no em/en dashes. Creating the pinned `scripts/**`,
  `packaging/**`, and `release.toml` is recorded as dated CHANGELOG decisions, as
  the constitution and build-autopilot protocol require.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/010-packaging-and-ci/
├── plan.md, research.md, data-model.md, quickstart.md
├── contracts/
│   └── pipeline.md   # the mapping of each pinned release.yml/releasing.md reference to the file that satisfies it
├── checklists/{requirements.md, pipeline-contract.md}
├── spec.md
└── tasks.md
```

### Source Code (repository root)

```text
wix/
└── main.wxs                       # cargo-wix MSI source (default location; single binary, icon, Start Menu shortcut, upgrade)
assets/
└── icon.ico                       # generated from assets/eso-weave-logo-clear.png with ImageMagick
packaging/
├── appimage/
│   └── AppDir/
│       ├── AppRun                 # launches usr/bin/eso-weave
│       ├── eso-weave.desktop      # desktop entry
│       └── eso-weave.png          # AppImage icon
└── linux/
    └── 70-eso-weave-uinput.rules  # udev rule packaged by the .deb and documented for the AppImage
scripts/
├── changelog-section.sh           # prints the CHANGELOG body for a "## [heading]" section
└── linux-build-deps.sh            # installs Linux GUI + backend build dependencies
Cargo.toml                         # add [package.metadata.deb] and [package.metadata.wix] (icon path)
release.toml                       # cargo-release config (bump, changelog roll, commit, tag, push)
README.md                          # add the Linux evdev-permission section (not pinned)
CHANGELOG.md                       # dated decisions for the pinned artifacts
```

**Structure Decision**: Files land where the pinned workflow and tools resolve
them: the WiX source at `wix/main.wxs` (the cargo-wix default, so the pinned
`cargo wix --no-build` with no path finds it), the AppDir under
`packaging/appimage/AppDir` (matching the pinned AppImage step), the scripts under
`scripts/` (matching the pinned workflow and docs), and cargo-deb metadata inside
`Cargo.toml` (where cargo-deb reads it). The udev rule lives under
`packaging/linux/` and is referenced by the cargo-deb asset list.

## Complexity Tracking

No constitution violations. No entries.
