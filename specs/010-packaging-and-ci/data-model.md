# Phase 1 Artifact Inventory: Packaging and Distribution

This slice produces files and configuration, not runtime data types. The
"model" is the inventory of artifacts, each with its location, format, and
purpose.

## Created artifacts

| Artifact | Path | Format | Purpose |
| --- | --- | --- | --- |
| WiX MSI source | `wix/main.wxs` | WiX XML | MSI definition: install the binary, icon, Start Menu shortcut, upgrade-in-place. Resolved by the pinned `cargo wix --no-build`. |
| App icon | `assets/icon.ico` | Windows ICO | Application and installer icon, generated from the logo art. |
| cargo-deb metadata | `Cargo.toml` `[package.metadata.deb]` | TOML | `.deb` definition: binary, desktop entry, icon, udev rule, description. |
| cargo-wix metadata | `Cargo.toml` `[package.metadata.wix]` | TOML | Points the product icon at `assets/icon.ico`. |
| AppRun | `packaging/appimage/AppDir/AppRun` | POSIX shell | AppImage entry point; execs `usr/bin/eso-weave`. |
| Desktop entry (AppImage) | `packaging/appimage/AppDir/eso-weave.desktop` | Desktop entry | AppImage `.desktop`. |
| Icon (AppImage) | `packaging/appimage/AppDir/eso-weave.png` | PNG | AppImage icon. |
| udev rule | `packaging/linux/70-eso-weave-uinput.rules` | udev rule | Grants group access to `/dev/uinput` for synthesis; packaged by the `.deb`. |
| Changelog section script | `scripts/changelog-section.sh` | POSIX shell (awk) | Prints the `## [heading]` section body of `CHANGELOG.md`. |
| Linux deps script | `scripts/linux-build-deps.sh` | POSIX shell | Installs the Linux GUI and backend build dependencies. |
| Release config | `release.toml` | TOML | cargo-release: bump, changelog roll, commit, tag, push. |
| Desktop entry (deb) | `packaging/linux/eso-weave.desktop` | Desktop entry | Menu entry installed by the `.deb`. |
| Evdev documentation | `README.md` (Linux section) | Markdown | States the input-device permission requirement and how to satisfy it. |

## Not modified (pinned)

| Artifact | Path | Note |
| --- | --- | --- |
| Release workflow | `.github/workflows/release.yml` | The contract; unchanged. |
| Release procedure | `docs/releasing.md` | The contract; unchanged. |
| Toolchain pin | `rust-toolchain.toml` | Unchanged. |

## Key content rules

- **`main.wxs`**: one `Component` with the `eso-weave.exe` `File`; a Start Menu
  `Shortcut` with `assets/icon.ico`; a fixed `UpgradeCode` GUID and `MajorUpgrade`
  for upgrade-in-place; install under `ProgramFiles64Folder`. No custom actions
  that write outside the install location.
- **`[package.metadata.deb]`**: `assets` list mapping the release binary to
  `/usr/bin/eso-weave`, the desktop entry to `/usr/share/applications/`, the icon
  to `/usr/share/icons/hicolor/256x256/apps/`, and the udev rule to
  `/usr/lib/udev/rules.d/`; a `depends` of `$auto`; a `section`, `priority`, and
  an `extended-description` noting the evdev requirement.
- **`AppRun`**: `#!/bin/sh` exec of `"$(dirname "$0")/usr/bin/eso-weave" "$@"`.
- **`*.desktop`**: `Name=ESO Weave`, `Exec=eso-weave`, `Icon=eso-weave`,
  `Type=Application`, `Categories=Utility;Game;`.
- **`70-eso-weave-uinput.rules`**: `KERNEL=="uinput", SUBSYSTEM=="misc",
  GROUP="input", MODE="0660"` (plus a `static_node` option) so the interception
  backend can open `/dev/uinput`.
- **`changelog-section.sh`**: `awk` that starts printing after a line matching
  `^## \[<arg>` and stops at the next `^## `, excluding both heading lines; empty
  output if no match.
- **`linux-build-deps.sh`**: `apt-get install -y` of the windowing/GL, X11/XCB,
  xkbcommon, Wayland, and evdev/udev `-dev` packages, run under `sudo`.
- **`release.toml`**: `pre-release-replacements` for `CHANGELOG.md`,
  `tag-name = "v{{version}}"`, `tag-message`, `pre-release-commit-message =
  "release: v{{version}}"`, `push = true`, `publish = false`.
