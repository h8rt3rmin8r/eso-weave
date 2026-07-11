# Tasks: Packaging and Distribution

**Feature**: `specs/010-packaging-and-ci` | **Branch**: `010-packaging-and-ci`

This slice adds packaging configuration and scripts, not Rust logic, so it is
verified by running the scripts and configs that can run here, checking
well-formedness and consistency with the pinned `release.yml`, generating and
validating the icon, building the release binary, and CI parity. The installers
build only in the release CI on a tag. Creating the pinned `scripts/**`,
`packaging/**`, and `release.toml` is recorded as dated CHANGELOG decisions. Paths
are repository-relative.

## Phase 1: Setup

- [x] T001 Create the directories `wix/`, `packaging/appimage/AppDir/`, `packaging/linux/`, and `scripts/` (creating `scripts/` and `packaging/` as first-time pinned directories).

## Phase 2: Release scripts (US1, US2, US3)

- [x] T002 [P] [US2] Write `scripts/changelog-section.sh`: an awk-based script that, given a `<heading>`, prints the body of the `## [<heading>...]` section of `CHANGELOG.md` (between that heading line and the next `## `, excluding both), tolerating a trailing ` - DATE`, and prints nothing when absent. Make it executable.
- [x] T003 [US2] Verify `scripts/changelog-section.sh`: `bash -n`; run it for `Unreleased` (prints the current body) and for a nonexistent heading (prints nothing).
- [x] T004 [P] [US3] Write `scripts/linux-build-deps.sh`: a `sudo apt-get`-based installer of the GUI windowing/GL, X11/XCB, xkbcommon, Wayland, and evdev/udev `-dev` packages, run as the pinned workflow invokes it. Make it executable. Verify with `bash -n` and a dependency-category grep.

## Phase 3: Windows MSI and icon (US1)

- [x] T005 [US1] Generate `assets/icon.ico` from `assets/eso-weave-logo-clear.png` with ImageMagick (`magick ... -define icon:auto-resize=256,128,64,48,32,16`); verify the ICO magic bytes `00 00 01 00`.
- [x] T006 [US1] Write `wix/main.wxs`: a cargo-wix MSI source installing `eso-weave.exe` under `ProgramFiles64Folder`, a Start Menu shortcut using `assets/icon.ico`, a fixed `UpgradeCode` with `MajorUpgrade` (upgrade-in-place), and no writes outside the install location. Verify it is well-formed XML.

## Phase 4: Linux packages (US1, US4)

- [x] T007 [US4] Write the udev rule `packaging/linux/70-eso-weave-uinput.rules` granting `input`-group access to `/dev/uinput`, and the desktop entry `packaging/linux/eso-weave.desktop`.
- [x] T008 [US1] Write the AppImage AppDir: `packaging/appimage/AppDir/AppRun` (execs `usr/bin/eso-weave`, executable), `packaging/appimage/AppDir/eso-weave.desktop`, and `packaging/appimage/AppDir/eso-weave.png` (from the logo art). Verify the desktop entry begins with `[Desktop Entry]`.
- [x] T009 [US1] Add `[package.metadata.deb]` to `Cargo.toml` (binary to `/usr/bin/eso-weave`, the desktop entry, the icon to `hicolor/256x256`, the udev rule to `/usr/lib/udev/rules.d/`, `depends = "$auto"`, section, priority, extended description noting the evdev requirement) and `[package.metadata.wix]` pointing the icon at `assets/icon.ico`. Confirm `cargo build --release --bin eso-weave` still builds and `cargo metadata` parses.

## Phase 5: Release configuration (US1)

- [x] T010 [US1] Write `release.toml`: cargo-release config with `pre-release-replacements` that roll `CHANGELOG.md` (`## [Unreleased]` to `## [X.Y.Z] - DATE`, opening a fresh `## [Unreleased]`), `pre-release-commit-message = "release: v{{version}}"`, `tag-name = "v{{version}}"`, `push = true`, `publish = false`. Verify with `cargo release 0.1.1 --dry-run` (cuts nothing).

## Phase 6: Documentation and evdev (US4)

- [x] T011 [US4] Add a Linux section to `README.md` documenting the evdev permission requirement (membership in the `input` group or the packaged udev rule) and how to satisfy it.

## Phase 7: Polish and cross-cutting

- [x] T012 Update `CHANGELOG.md` `[Unreleased]` with an Added line for the packaging artifacts and dated Decisions entries for creating the pinned `scripts/**`, `packaging/**`, and `release.toml`, and for adding the cargo-deb/cargo-wix `Cargo.toml` metadata and generating `assets/icon.ico`.
- [x] T013 Run the verification gate: enumerate that every pinned-pipeline-referenced path now exists; `bash -n` the scripts; run `changelog-section.sh Unreleased` and an absent heading; `cargo release 0.1.1 --dry-run`; validate `wix/main.wxs` XML and the desktop entries; confirm `assets/icon.ico` magic; `cargo build --release --bin eso-weave`; and CI parity (`cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all --locked`, plus `cargo check --target x86_64-unknown-linux-gnu`). Confirm the pinned `release.yml`, `releasing.md`, and `rust-toolchain.toml` are unchanged in the diff.

## Dependencies and order

- Setup (T001) first. The release scripts (US1/US2/US3, T002 to T004), the MSI and
  icon (T005 to T006), the Linux packages (T007 to T009), the release config
  (T010), and the docs (T011) are largely independent and can proceed in any order
  after setup, except T005 (icon) precedes T006/T008/T009 that reference it, and
  T007 (udev rule) precedes T009 (which packages it). Polish (T012 to T013) last;
  T013 is the gate.

## Parallel opportunities

- The two scripts (T002, T004), the icon (T005), the udev rule/desktop entries
  (T007), and the AppDir (T008) touch different files and can be written in
  parallel.

## MVP scope

The release scripts (US1/US2), the WiX source and icon (US1), and the cargo-deb
metadata (US1) complete the parts of the pinned pipeline that gate a release; the
AppImage AppDir, the udev rule and evdev documentation (US4), and `release.toml`
finish the distribution surface.
