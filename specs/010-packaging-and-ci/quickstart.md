# Quickstart: Packaging and Distribution

The installers themselves are built only by the release CI on a tag. Here we
verify that the pipeline's referenced files exist and are well-formed and
consistent, the scripts run, the release config parses, the icon is valid, the
release binary compiles, and CI parity holds.

## Prerequisites

- The repository builds (feature 009 GUI binary present).
- `cargo`, `bash`, and (for the icon) ImageMagick are available; `cargo release`
  is installed for the dry run.

## Referenced-path existence (US1)

Confirm every path the pinned pipeline references now exists:

```sh
ls scripts/changelog-section.sh scripts/linux-build-deps.sh release.toml \
   wix/main.wxs assets/icon.ico \
   packaging/appimage/AppDir/AppRun packaging/appimage/AppDir/eso-weave.desktop \
   packaging/appimage/AppDir/eso-weave.png packaging/linux/70-eso-weave-uinput.rules
grep -q "package.metadata.deb" Cargo.toml
```

## Changelog extractor (US2)

```sh
bash -n scripts/changelog-section.sh
scripts/changelog-section.sh Unreleased        # prints the current Unreleased body
scripts/changelog-section.sh 9.9.9             # prints nothing (absent section)
```

Expected: the first prints the `## [Unreleased]` body only; the second is empty.

## Linux build deps (US3)

```sh
bash -n scripts/linux-build-deps.sh
grep -Eiq "x11|xcb|xkbcommon|wayland|udev|libgl|evdev" scripts/linux-build-deps.sh
```

## Release config (US1)

```sh
cargo release 0.1.1 --dry-run     # parses release.toml, resolves replacements, cuts nothing
```

Expected: a dry-run plan with no tag created and no push.

## Icon validity (SC-004)

```sh
xxd -l 4 assets/icon.ico          # expect 00 00 01 00 (ICO magic)
```

## WiX and desktop well-formedness

```sh
python -c "import xml.dom.minidom,sys; xml.dom.minidom.parse('wix/main.wxs')"
grep -q "^\[Desktop Entry\]" packaging/appimage/AppDir/eso-weave.desktop
```

## Release build and CI parity (SC-005)

```sh
cargo build --release --bin eso-weave
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

## Pinned files unchanged (SC-006)

```sh
git diff --name-only | grep -E "release\.yml|releasing\.md|rust-toolchain\.toml" && echo "PINNED CHANGED (bad)" || echo "pinned untouched"
```

## Not run here

- The actual MSI, `.deb`, and AppImage builds, and the GitHub Release, run in the
  release CI when an operator cuts a `vX.Y.Z` tag (an explicit action outside this
  slice).
