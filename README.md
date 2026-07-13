<p align="center">
  <img alt="ESO Weave" src="assets/eso-weave-banner.png" width="720">
</p>
<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/version-0.6.0-2ea44f">
  <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue">
</p>
<p align="center">Cross-platform desktop companion for The Elder Scrolls Online.</p>

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

## Fishing

The fishing routine casts, waits for a bite, reels in the catch, and recasts for
you, over and over, while you stand at a fishing hole. It works by reading a small
on-screen signal rendered by the bundled PixelBeacon companion addon, so it needs
that addon installed and loaded to see when a cast has started and when a fish
bites.

### How it works (read this first)

The fishing hotkey casts the line for you. You do not cast first. Stand aimed at
the fishing hole with the in-game interact prompt showing (the "Fish" prompt),
then press F2. ESO Weave sends the interact key to cast, watches the beacon for
the bite, sends the interact key again to reel in, waits, and recasts. Press F2
again to stop.

### Before you start

For fishing to work, all of the following must be true:

- You have fishing bait selected in game. ESO will not cast the line without
  bait, so if no bait is selected the F2 automation cannot start a cast and
  fishing will not run. Select a bait before you begin.
- The PixelBeacon addon is installed. Use the app's Pixel Beacon (Addon) control
  to install it, and confirm the app shows it as installed and current.
- The addon is enabled in the in-game AddOns menu and is not flagged "Out of
  Date". If ESO shows it as out of date, either update ESO Weave (which refreshes
  the addon) or tick "Allow out of date AddOns" in the AddOns menu. After the app
  refreshes the addon, reload the UI in game (type `/reloadui`) or log out and
  back in so ESO picks up the new files.
- The beacon strip the addon draws is on screen and not covered by other UI.
- The ESO window is focused. ESO Weave only sends input while the game window is
  the active window.

### Using it

1. Confirm the Pixel Beacon (Addon) status in the app looks healthy.
2. Select fishing bait in game. Without bait selected the cast fails and fishing
   will not start.
3. In game, walk up to a fishing hole and face the water so the interact prompt
   appears.
4. Press F2 (or use the Fishing toggle in the app). Do not cast the line
   yourself first.
5. Watch the Fishing status in the app move through the routine and leave it
   running. Press F2 again to stop.

### What the status means

While fishing is running, the Fishing status shows, in order:

- Casting: the cast was sent and the app is waiting for the beacon to confirm a
  cast is active.
- Fishing (waiting for a bite): the cast is active and the app is waiting for a
  fish.
- Reeling in: a bite was seen and the app is reeling.
- Recasting: the catch was collected and the app is casting again.

When fishing is off, the status shows Idle. If it stopped on its own it also tells
you why: Idle (no cast detected) if a cast was never confirmed, or Idle (signal
lost) if the beacon signal went away.

### Settings

- Interact key: the key ESO Weave presses to cast, reel, and recast. It defaults
  to E, which is the default ESO interact bind. If you rebound interact in game,
  set the same key here.
- Arm timeout: how long to wait for a cast to be confirmed before giving up
  (default 8000 ms).
- Reel delay: how long after a bite before reeling (default 100 ms).
- Recast delay: how long after a catch before casting again (default 3000 ms).

### Troubleshooting

If the Fishing status turns to Idle within a few seconds of starting, the app is
not seeing the beacon signal, or the cast never started. In order, check that:

- You have fishing bait selected. With no bait the cast fails, so the app never
  sees a cast start and stops with Idle (no cast detected).
- The PixelBeacon addon is enabled and not flagged "Out of Date" in the in-game
  AddOns menu. A stale addon that ESO refuses to load produces exactly this
  symptom. If the app just refreshed the addon, remember to `/reloadui` or relog.
- The beacon strip is visible on screen and not covered.
- The ESO window is focused.
- You pressed F2 while aimed at the hole with the interact prompt up, and did not
  cast the line yourself first.

An Idle (no cast detected) or Idle (signal lost) status points to the same
checks. Automating gameplay input may violate the ESO Terms of Service; see the
Disclaimer below and use fishing at your own risk.

## Weaving

Weaving runs your combat rotation with tighter timing than manual play. While the
ESO window is focused, ESO Weave watches for your skill keypress and, in its
place, performs a short sequence: a basic attack woven together with that skill,
timed so the light attack lands just before the ability. You play as normal; the
app supplies the weave.

Press F1 to suspend and resume the weave engine at any time. While suspended, no
input is sent.

### Skill slots

ESO Weave maps seven action slots:

- Slots 1 to 5 are your skill bar abilities, bound to the 1 through 5 keys. They
  are active by default.
- Slot 6 is the Ultimate (bound to R) and slot 7 is a Synergy (bound to X). Both
  are inactive by default; enable them if you want them woven.

Every slot uses a Light Attack weave by default. Each slot's weave type and, if
you want, a custom delay can be changed in the Skills area.

### Weave types

- Light Attack (the default): a light attack woven with the skill.
- Heavy Attack: a heavy attack held, then the skill.
- Bash Attack: a light attack, the skill, then a bash.
- Block Casting: the skill cast while blocking.

### Default timings

The engine ships with these default delays, all editable in Settings:

| Timing | Default | What it controls |
| --- | --- | --- |
| Global cooldown | 500 ms | Minimum interval between weave executions. |
| Light attack delay | 50 ms | Gap between the basic attack and the skill key. |
| Heavy attack delay | 1000 ms | How long a heavy attack is held before the skill. |
| Bash delay | 125 ms | Gap before the bash in a bash attack. |

Latency adaptation, which shortens delays as measured latency rises, is off by
default and can be enabled in Settings.

### A note on weapon bars

ESO Weave can detect the active weapon bar and adjust timing per bar, but the
multi-bar (dual-bar) weaving behavior is not yet finalized and is out of scope for
this documentation. This section covers single-bar weaving only.

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

## License

Licensed under the [Apache License 2.0](LICENSE).
