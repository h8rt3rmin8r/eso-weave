<p align="center">
  <img alt="ESO Weave" src="assets/eso-weave-banner.png" width="720">
</p>
<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/version-0.4.0-2ea44f">
  <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue">
</p>
<p align="center">Cross-platform desktop companion for The Elder Scrolls Online.</p>

## Disclaimer

This project is published for educational purposes only. It exists as a study
in cross-platform input handling, screen-signal protocols, and game-adjacent
tooling architecture. It is not affiliated with, endorsed by, or supported by
ZeniMax Online Studios, ZeniMax Media Inc., Bethesda Softworks, or Microsoft.
The Elder Scrolls® and The Elder Scrolls Online are trademarks or registered
trademarks of ZeniMax Media Inc.

Automating gameplay input may violate the Terms of Service of The Elder
Scrolls Online. Using this software with a live game account is done entirely
at your own risk. You are solely responsible for reviewing and complying with
all agreements that govern your account, and you accept all consequences of
your use of this software, up to and including permanent account suspension.

The author assumes no liability for any account action, data loss, or other
damages arising from the use or misuse of this software. This software is
provided "AS IS", without warranty of any kind, express or implied, in
accordance with the Apache License, Version 2.0 under which it is distributed.

## Installation

Prebuilt installers are published on the [Releases](https://github.com/h8rt3rmin8r/eso-weave/releases)
page: a Windows x64 MSI, and for Linux x86_64 a `.deb` package, an AppImage, and a
tarball.

### Windows (MSI)

Download the `.msi`, right click it, choose Properties, and tick Unblock if the
file was marked as coming from the internet, then run it. The installer walks
through a short wizard (welcome, license, install location, progress, finish). On
the final page you can leave "Launch ESO Weave" ticked to start the app right
away.

After installing you can start ESO Weave from either shortcut:

- Desktop: an "ESO Weave" shortcut on your desktop.
- Start Menu: All apps, under the "ESO Weave" folder.

The application is installed to `C:\Program Files\ESO Weave\`. Logs, when the file
log is enabled, are written to `%APPDATA%\eso-weave\logs\YYYY-MM.log`; check there
first if the app does not behave as expected.

### Linux input permission (evdev)

Input interception on Linux reads keyboard devices and synthesizes input through
`/dev/uinput`, which requires device access. Satisfy this in one of two ways:

- Add your user to the `input` group and log in again:
  `sudo usermod -aG input "$USER"`.
- Or install the provided udev rule that grants the `input` group access to
  `/dev/uinput`. The `.deb` installs it to
  `/usr/lib/udev/rules.d/70-eso-weave-uinput.rules` automatically; for the AppImage
  or tarball, copy `packaging/linux/70-eso-weave-uinput.rules` there yourself and
  reload with `sudo udevadm control --reload && sudo udevadm trigger`.

Without this permission, key interception silently does nothing.

## License

Licensed under the [Apache License 2.0](LICENSE).
