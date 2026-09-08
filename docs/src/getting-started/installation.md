# Installation

A Release Package is the MSI, deb, AppImage, or tarball downloaded from a tagged
release. Its Checksum is the SHA256 value recorded in `SHA256SUMS` for download
verification.

Prebuilt installers are published on the
[GitHub Releases page](https://github.com/h8rt3rmin8r/eso-weave/releases).
Windows releases provide an x64 MSI. Linux x86_64 releases provide a `.deb`, an
AppImage, and a tarball.

## Verify a download

Download `SHA256SUMS` from the same release as the package. A checksum confirms
that the downloaded bytes match the published release asset. It does not replace
the [Responsible Use](responsible-use.md) review.

On Windows, run this in PowerShell and compare the displayed hash with the line
for the MSI in `SHA256SUMS`:

```powershell
Get-FileHash .\eso-weave-VERSION-x86_64-windows.msi -Algorithm SHA256
```

On Linux, run this and compare the result with the matching line:

```sh
sha256sum eso-weave-VERSION-x86_64.PACKAGE
```

Replace `VERSION` and `PACKAGE` with the downloaded filename. Do not install a
file whose hash differs.

## Windows

Download the MSI. If Windows marks it as downloaded from the internet, right
click it, choose Properties, and select Unblock before running it. The final
installer page can launch ESO Weave immediately.

The installer always creates a Start Menu shortcut under ESO Weave. A desktop
shortcut is optional on the installer feature page. The default installation
directory is `C:\Program Files\ESO Weave\`. When file
logging is enabled, logs are written to
`%APPDATA%\eso-weave\logs\YYYY-MM.log`.

To update, download the newer MSI and run it. The package supports an upgrade in
place. To remove the application, open Windows Settings, choose Apps, find ESO
Weave, and choose Uninstall. Application removal and PixelBeacon removal are
separate. Use the managed **Uninstall** action in ESO Weave before removing the
desktop application if you also want to remove the addon.

## Linux

ESO Weave reads keyboard devices and synthesizes input through `/dev/uinput`.
The user therefore needs the appropriate device permission.

- Add the user to the `input` group, then sign out and back in:
  `sudo usermod -aG input "$USER"`
- Or install the udev rule for `/dev/uinput`. The `.deb` installs
  `/usr/lib/udev/rules.d/70-eso-weave-uinput.rules`. AppImage and tarball release
  assets do not include a standalone rule file, so use the supported `input`
  group route unless you obtain and review the rule from the matching source tag.

Without this permission, key interception does nothing.

Choose one package:

| Package | Install and launch | Update | Remove |
| --- | --- | --- | --- |
| `.deb` | `sudo apt install ./eso-weave-VERSION-x86_64.deb`, then run `eso-weave` | Install the newer `.deb` with the same command | `sudo apt remove eso-weave` |
| AppImage | Mark the exact downloaded file executable with `chmod +x`, then run that file | Download, verify, and use the newer AppImage | Remove the AppImage file you selected |
| Tarball | Extract the archive and run the `eso-weave` binary inside it | Download, verify, and extract the newer archive separately | Remove the extracted directory you selected |

The `.deb` installs the udev rule. AppImage and tarball users must configure the
`input` group or install a reviewed matching rule themselves. Sign out and back in after changing group
membership. Do not make `/dev/input` or `/dev/uinput` world-writable.

On X11, the application can query the active window and capture the game. On a
Wayland desktop, ESO must run through XWayland. A pure-Wayland game surface does
not provide the focus and capture evidence ESO Weave requires, so interception
stays off. See [Troubleshooting](troubleshooting.md#linux-input-or-focus-does-not-work).

Linux configuration is stored under `$XDG_CONFIG_HOME/eso-weave/`, falling back
to `~/.config/eso-weave/`. Logs use `$XDG_STATE_HOME/eso-weave/logs/` when an XDG
state directory is available, otherwise the platform configuration root.

## Package guarantees

Windows packages use `cargo-wix` and support installation, removal,
upgrade-in-place, and Start Menu and optional desktop shortcuts. The MSI does not write
to ESO or Documents directories. PixelBeacon installation is a separate action inside ESO Weave.

Linux releases include a `.deb`, AppImage, and tarball. Release assets include a
combined `SHA256SUMS` file. Release binaries are produced by CI from tagged
versions, and the version is sourced from `Cargo.toml`.

The Linux virtual input device advertises every supported application key and
mouse control plus every key reported by the selected physical keyboard before
the keyboard is grabbed.

Every release executable contains the complete searchable documentation site.
After launch, choose **Help > Documentation** to open it locally without a network
connection. The `127.0.0.1` address belongs to the running ESO Weave process and
stops when the application exits.

Continue to [Responsible Use](responsible-use.md) before running automation with
a live account, then [complete first launch](first-launch.md).
