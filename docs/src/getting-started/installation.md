# Installation

Prebuilt installers are published on the
[GitHub Releases page](https://github.com/h8rt3rmin8r/eso-weave/releases).
Windows releases provide an x64 MSI. Linux x86_64 releases provide a `.deb`, an
AppImage, and a tarball.

## Windows

Download the MSI. If Windows marks it as downloaded from the internet, right
click it, choose Properties, and select Unblock before running it. The final
installer page can launch ESO Weave immediately.

Shortcuts are placed on the desktop and in the Start Menu under ESO Weave. The
default installation directory is `C:\Program Files\ESO Weave\`. When file
logging is enabled, logs are written to
`%APPDATA%\eso-weave\logs\YYYY-MM.log`.

## Linux

ESO Weave reads keyboard devices and synthesizes input through `/dev/uinput`.
The user therefore needs the appropriate device permission.

- Add the user to the `input` group, then sign out and back in:
  `sudo usermod -aG input "$USER"`
- Or install the udev rule for `/dev/uinput`. The `.deb` installs
  `/usr/lib/udev/rules.d/70-eso-weave-uinput.rules`. For the AppImage or
  tarball, copy `packaging/linux/70-eso-weave-uinput.rules` there, then run
  `sudo udevadm control --reload && sudo udevadm trigger`.

Without this permission, key interception does nothing.

## Package guarantees

Windows packages use `cargo-wix` and support installation, removal,
upgrade-in-place, and Start Menu and desktop shortcuts. The MSI does not write
to ESO or Documents directories. PixelBeacon installation is a separate action
inside ESO Weave.

Linux releases include a `.deb`, AppImage, and tarball. Release assets include a
combined `SHA256SUMS` file. Release binaries are produced by CI from tagged
versions, and the version is sourced from `Cargo.toml`.

Continue to [Responsible Use](responsible-use.md) before running automation with
a live account.
