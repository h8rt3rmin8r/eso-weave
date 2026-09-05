<p align="center">
  <img alt="ESO Weave" src="assets/eso-weave-banner.png" width="720">
</p>
<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/version-0.12.0-2ea44f">
  <img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue">
</p>
<p align="center">Cross-platform desktop companion for The Elder Scrolls Online.</p>

ESO Weave runs beside the game, never inside it. It watches for your keypresses
while the ESO window is focused and supplies three things:

- **Weaving.** Your skill press becomes a basic attack woven with that skill,
  timed so the light attack lands just before the ability.
- **Fishing.** Casts, waits for the bite, reels in, and recasts, over and over,
  while you stand at a fishing hole.
- **Auto-potion.** Drinks your quickslotted potion when a resource you are
  watching runs low.

Weaving works on its own. Fishing and auto-potion read a small on-screen signal
drawn by **PixelBeacon**, a companion addon that ships inside the application and
installs from its interface with one click.

The app never reads or writes game memory and never touches network traffic.

| Hotkey | Does |
| --- | --- |
| `F1` | Suspend and resume everything |
| `F2` | Start and stop fishing |
| `F3` | Turn auto-potion on and off |

All three work from inside the game, and all three are rebindable. Input is only
ever sent while the ESO window is the active window.

## Contents

- [Installation](#installation)
- [Game state and context](#game-state-and-context)
- [Weaving](#weaving)
- [Fishing](#fishing)
- [Auto-potion](#auto-potion)
- [The PixelBeacon overlay](#the-pixelbeacon-overlay)
- [References](#references)
- [Disclaimer](#disclaimer)

## Installation

Prebuilt installers are published on the
[Releases](https://github.com/h8rt3rmin8r/eso-weave/releases) page: a Windows x64
MSI, and for Linux x86_64 a `.deb` package, an AppImage, and a tarball.

### Windows

Download the `.msi`, right click it, choose Properties, and tick Unblock if the
file was marked as coming from the internet, then run it. The installer walks
through a short wizard. On the final page you can leave "Launch ESO Weave" ticked
to start straight away.

You will find shortcuts on your desktop and in the Start Menu under "ESO Weave".
The application installs to `C:\Program Files\ESO Weave\`. With file logging
enabled, logs are written to `%APPDATA%\eso-weave\logs\YYYY-MM.log`, which is the
first place to look if something misbehaves.

### Linux

Input interception reads keyboard devices and synthesizes input through
`/dev/uinput`, which needs device access. Grant it either way:

- Add yourself to the `input` group and log in again:
  `sudo usermod -aG input "$USER"`
- Or install the udev rule that grants the `input` group access to `/dev/uinput`.
  The `.deb` places it at `/usr/lib/udev/rules.d/70-eso-weave-uinput.rules` for
  you. For the AppImage or the tarball, copy
  `packaging/linux/70-eso-weave-uinput.rules` there yourself and reload with
  `sudo udevadm control --reload && sudo udevadm trigger`.

**Without this permission, key interception silently does nothing.**

## Game state and context

The main window separates information by the question it answers. **Live HUD**
shows resources, Game Context, combat, movement, roll-dodge state, life state,
weapon setup, and the selected quickslot. **System and State** shows
world-transition state and
whether the game, ESO Weave,
PixelBeacon, fishing, and auto-potion are ready and why an action is blocked. The
two sections sit side by side in a wide window and stack with Live HUD first in a
narrow one. System and State can be collapsed from its full keyboard-accessible
header, and that preference survives restart. The Skills table remains below it.

The Game row reports runtime and installation provider together. Installation is
detected from provider-owned evidence for the ESO Store, Steam, Epic Games, or
Steam Proton, then checked against the expected launcher and client files. Runtime
reports **Inactive**, **Launcher open**, **Active**, or **Unknown**. The game client
takes precedence, so closing the launcher after ESO starts does not make an active
session disappear.

**World state** reports **Active** only after ESO finishes player activation and
PixelBeacon refreshes every player-derived payload for the current world. It
reports **Transitioning** from player deactivation through the loading interval,
and **Not detected** when current lifecycle evidence is unavailable. PixelBeacon
version 16 advertises protocol version 2 for this field; older negotiated layouts
remain readable but are never sampled beyond their last defined block.

**Roll dodge** reports **Active** from the player's dodge-roll combat event until
its matching effect fade. A 1500 ms watchdog clears a rejected dodge that emits a
gain without a fade. Loading, death, or lost telemetry clears the observation to
**Not detected**; resurrection in place establishes a fresh **Inactive** baseline.
PixelBeacon version 17 advertises protocol version 3 for this field; version 1 and
2 layouts remain readable at their original extents.

**Game Context** combines four independent observations: whether the game is
active, whether its window is focused, whether PixelBeacon is fresh, and which
in-game surface PixelBeacon reports. **Gameplay** appears only when all four are
authoritative and the addon reports no menu. Missing or invalid addon evidence is
shown as **Signal unavailable**, never as Gameplay.

When ESO is not active, live metrics show **Game not active** and no weave,
fishing, or auto-potion input is sent. ESO Weave resumes observation automatically
when the client starts again and regains focus. Requested fishing and auto-potion
toggles are preserved across game inactivity and focus loss, while their existing
signal-loss rules still switch them off.

Health, Stamina, and Magicka use labeled bars with exact percentages. Observed
0% is a real empty value. **Game not active** and **Signal unavailable** are
separate non-numeric states, so missing telemetry never looks like an empty
resource. A bar says **Low** only when that resource is enabled in auto-potion
settings and has reached its configured threshold.

## Weaving

Weaving runs your rotation with tighter timing than hand-play. While the ESO
window is focused, ESO Weave intercepts your skill keypress and, in its place,
performs a short sequence: a basic attack woven with that skill. You play as
normal and the app supplies the weave.

Press `F1` to suspend and resume at any time. While suspended, nothing is sent.

Weaving requires the current PixelBeacon addon, a freshly detected Alive state,
and an explicit Inactive roll-dodge state. While a roll dodge is active or its
state is unavailable, bound skill keys pass through to the game unchanged instead
of starting a weave. A roll that begins during an already queued sequence cancels
remaining generated input and releases any held mouse buttons.

### Skill slots

Seven action slots, configured in the Skills area:

| Slots | Bound to | Active by default |
| --- | --- | --- |
| 1 to 5 | `1` `2` `3` `4` `5` | Yes |
| Ultimate | `R` | No |
| Synergy | `X` | No |

Enable the Ultimate and Synergy slots if you want them woven. An inactive slot
passes its key straight through to the game.

### Weave types

Every slot uses Light Attack by default. Change the type per slot in the Skills
area.

| Type | What it does |
| --- | --- |
| Light Attack | A light attack woven with the skill |
| Heavy Attack | A heavy attack held, then the skill |
| Bash Attack | A light attack, the skill, then a bash |
| Block Casting | The skill cast while blocking |

### Timing

The defaults, all editable in Settings, with per-slot overrides available for
skills that need them:

| Timing | Default | Controls |
| --- | --- | --- |
| Global cooldown | 500 ms | Minimum interval between weaves |
| Light attack delay | 50 ms | Gap between the basic attack and the skill key |
| Heavy attack delay | 1000 ms | How long a heavy attack is held before the skill |
| Bash delay | 125 ms | Gap before the bash in a bash attack |

**Weapon bars.** ESO Weave keeps a separate timing profile for your front and
back bars and applies whichever is active. Turn on weapon-aware timing and the
heavy-attack delay follows the weapon class on the active bar instead, since a
dual wield heavy attack and a bow heavy attack are nowhere near the same length.
Detecting the bar needs PixelBeacon; without it, your front-bar profile applies.

**Latency adaptation.** Off by default. When enabled, delays grow with measured
server latency so a weave that lands cleanly at 30 ms still lands at 120 ms. This
also needs PixelBeacon.

## Fishing

The fishing routine casts, waits for a bite, reels in the catch, and recasts, over
and over, while you stand at a fishing hole.

**The hotkey casts for you. Do not cast first.** Stand aimed at the fishing hole
with the interact prompt showing, then press `F2`. ESO Weave sends the interact
key to cast, watches for the bite, reels in, waits, and recasts. Press `F2` again
to stop.

### Before you start

- **Select fishing bait in game.** ESO will not cast without it, so with no bait
  the automation never starts a cast.
- **Install PixelBeacon** from the app's Pixel Beacon (Addon) control, and confirm
  the app shows it as installed and current.
- **Enable the addon in game** and make sure ESO has not flagged it "Out of Date".
  If it has, either update ESO Weave, which refreshes the addon, or tick "Allow out
  of date AddOns". After a refresh, `/reloadui` or relog so ESO picks up the new
  files.
- **Keep the beacon overlay visible.** See
  [The PixelBeacon overlay](#the-pixelbeacon-overlay).
- **Keep the ESO window focused.**

### Using it

1. Confirm the Pixel Beacon status in the app looks healthy.
2. Select bait in game.
3. Walk up to a fishing hole and face the water so the interact prompt appears.
4. Press `F2`, or use the Fishing toggle in the app.
5. Leave it running. Press `F2` again to stop.

### What the status means

| Status | Meaning |
| --- | --- |
| Casting | The cast was sent; waiting for the beacon to confirm it |
| Fishing (waiting for a bite) | The cast is active |
| Reeling in | A bite was seen |
| Recasting | The catch was collected; casting again |
| Idle | Fishing is off |
| Idle (no cast detected) | A cast was never confirmed |
| Idle (signal lost) | The beacon signal went away |
| Idle (game not active) | The ESO client exited |

### Settings

- **Interact key:** the key pressed to cast, reel, and recast. Defaults to `E`. If
  you rebound interact in game, set the same key here.
- **Arm timeout:** how long to wait for a cast to be confirmed before giving up.
  Default 8000 ms.
- **Reel delay:** how long after a bite before reeling. Default 100 ms.
- **Recast delay:** how long after a catch before casting again. Default 3000 ms.

### If it goes Idle within a few seconds

Either the app is not seeing the beacon, or the cast never started. In order:

1. Is bait selected? With none, the cast fails and you get Idle (no cast
   detected).
2. Is the addon enabled and not "Out of Date"? A stale addon ESO refuses to load
   produces exactly this. If the app just refreshed it, `/reloadui` or relog.
3. Is the beacon overlay visible and uncovered?
4. Is the ESO window focused?
5. Did you press `F2` while aimed at the hole with the prompt up, without casting
   yourself first?

## Auto-potion

Auto-potion drinks the potion in your active quickslot when a resource you are
watching runs low. It is the one part of ESO Weave that acts on what the addon
reports rather than just showing it to you, so it is deliberately cautious.

The System and State row separates the request switch from the effective result. It shows
Off, a dormant game condition, the current safety or observation blocker, Ready,
or the resource that just triggered an attempt.

**It ships off, and it is off again after every restart.** Turning it on takes two
steps: enable at least one resource in Settings, then press `F3`. With no resource
enabled it never fires, whatever else is true.

Each resource has its own tick box and its own threshold, and the rule is an
**OR**: it fires when *any* enabled resource is at or below *its own* threshold.
Waiting for health, magicka, and stamina to all be low would mean firing only once
the potion no longer helps.

It presses the key only when all of these hold:

- Auto-potion is on, and an enabled resource is at or below its threshold
- ESO is active and focused, and the PixelBeacon signal is fresh
- PixelBeacon authoritatively reports the player Alive
- The active quickslot explicitly holds a usable potion, and its cooldown has finished
- The minimum retry interval since the last attempt has passed
- ESO Weave is not suspended, and Game Context is positively detected as Gameplay

### What it will not do

- **It never fires on a reading it cannot make.** If the beacon signal drops, the
  addon reloads, or a loading screen clears the overlay, an unreadable resource
  counts as *not* low. It does nothing rather than something.
- **It blocks when the signal is lost** without forgetting that you requested it.
  Fresh game, focus, beacon, resource, and quickslot evidence must return before it
  can act again.
- **It does not know what your potion restores.** The game exposes that only as
  tooltip text, so you pick the stats to watch instead. Slotting a
  tri-restoration potion? Enable all three.
- **It does not choose between potions or change your quickslot.**

### Settings

- **Watch health / magicka / stamina:** a tick box and a threshold percentage
  each.
- **Quickslot key:** defaults to `Q`, the game's default quickslot bind.
- **Minimum retry interval:** the floor between two attempts, default 1500 ms. It
  covers the gap between pressing the key and the game reporting the resulting
  cooldown. Raise it if potions go faster than you expect.

## The PixelBeacon overlay

Fishing, auto-potion, weapon-bar detection, and latency adaptation all read the
same thing: a small grid of colored squares that PixelBeacon draws in the
**top-left corner of the game client**.

That grid is the entire channel between the addon and the app. It has to be
visible, because anything drawn over it reads as a missing signal.

At the default square size it covers **432 by 16 physical pixels**: a three-cell
layout header followed by twenty-four signal squares in one row. PixelBeacon uses
the current client width and wraps only when the complete next square would cross
the right edge.

**To make it smaller**, lower **Block size (px)** in Settings, under the beacon
group. The app shows the running reader's detected footprint beside that setting
and records layout transitions in the log. At the smallest supported size the
current overlay is 52 by 2 pixels. A newly selected size is not mixed with the old
live column count: it redeploys the addon and becomes the reported geometry after
a `/reloadui` and an app restart.

### Quickslot diagnostics

PixelBeacon version 13 distinguishes an empty slot, each supported non-potion
wheel kind, a depleted or blocked potion, a usable potion, and an unavailable
observation. Run `/pbquickslot` in ESO for one bounded snapshot of the numeric
slot facts. Run `/pbquickslot watch` to print only changed snapshots while testing,
then run it again to turn watching off. The receipt intentionally omits localized
item names and descriptions.

**The overlay cannot be moved.** Its position is part of the contract the addon and
the app share. Relocating it would mean changing both sides at once, and a
disagreement about where the grid starts is not something either side can detect.

## References

External sources of truth for the ESO integration:

- [esoui/esoui](https://github.com/esoui/esoui): the published source of the ESO
  user interface and the canonical definition of the game's Lua API that
  PixelBeacon targets. ESO Weave tracks the head of the `live` branch to keep the
  addon manifest's API version current.
- [ESOUI](https://www.esoui.com/): the ESO add-on community and index.
- [ESOUI Wiki](https://wiki.esoui.com/): human-readable documentation for the ESO
  add-on Lua API.
- [The Elder Scrolls Online](https://www.elderscrollsonline.com/): the official
  game site.

Project sources of truth:

- [Master specification](docs/ESO-Weave-Specification.md): the architecture of
  record; every feature traces to it.
- [Changelog](CHANGELOG.md): the dated record of releases and pinned-artifact
  decisions.

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
