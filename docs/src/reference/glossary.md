# Glossary

Use this alphabetical reference for ESO Weave interface terms, ESO player
phrases, platform vocabulary, and protocol concepts. Each entry names the
canonical term first, keeps likely search wording beside it, and links to the
page that owns the complete explanation.

<nav class="glossary-index" aria-label="Glossary alphabet">
<a href="#a">A</a>
<a href="#b">B</a>
<a href="#c">C</a>
<a href="#e">E</a>
<a href="#f">F</a>
<a href="#g">G</a>
<a href="#h">H</a>
<a href="#i">I</a>
<a href="#l">L</a>
<a href="#m">M</a>
<a href="#n">N</a>
<a href="#p">P</a>
<a href="#q">Q</a>
<a href="#r">R</a>
<a href="#s">S</a>
<a href="#t">T</a>
<a href="#u">U</a>
<a href="#w">W</a>
<a href="#x">X</a>
</nav>

## A

### API Version

**Aliases:** APIVersion, outdated addon, ESO client version

The ESO addon interface revision declared by an addon and reported by the game.
ESO Weave compares it with the supported PixelBeacon revision before treating
addon observations as compatible.

**Related:** [PixelBeacon lifecycle and compatibility](../features/pixelbeacon.md)

### Auto Potion

**Aliases:** auto pot, potion trigger, automatic quickslot

The optional controller that watches selected resources and submits the active
quickslot only when its threshold, retry, focus, and safety conditions authorize
an attempt.

**Related:** [Auto Potion behavior and setup](../features/auto-potion.md)

## B

### Bash Attack

**Aliases:** bash weave, interrupt, block bash

A weave type that combines block and attack input around a skill activation. It
is distinct from Block Casting and follows the configured bash timing sequence.

**Related:** [Weaving types and timing](../features/weaving.md)

### Block Casting

**Aliases:** block cast, cast while blocking

A weave type that holds block during a skill activation. It is selected per
skill slot and does not mean that all physical input is globally suppressed.

**Related:** [Weaving types and timing](../features/weaving.md)

## C

### Capture Tolerance

**Aliases:** color tolerance, compositor drift, decode tolerance

The bounded color difference the pixel-bus decoder accepts when compositing or
capture changes a sampled color slightly. Protocol markers and exact data fields
retain their documented validation rules.

**Related:** [Pixel Bus Protocol decoding](pixel-bus-protocol.md)

### Checksum

**Aliases:** SHA256, SHA256SUMS, download verification

A published cryptographic digest used to confirm that a downloaded release file
matches the artifact produced by the release workflow before installation.

**Related:** [Installation and package verification](../getting-started/installation.md)

### Configuration

**Aliases:** settings file, config.json, preferences

User-selected ESO Weave behavior stored in the versioned configuration file.
Runtime observations and session-only state do not belong in this settings data.

**Related:** [Configuration files and recovery](configuration.md)

## E

### ESO Weave

**Aliases:** desktop companion

The local desktop companion that coordinates configured weaving, fishing, and
resource-watch features while enforcing focus and observed game-state safety.

**Related:** [ESO Weave documentation home](../)

## F

### File Logging

**Aliases:** log file, monthly log, debug log, trace log

Optional persistent diagnostic output written at the selected level with monthly
rotation. Input contents remain subject to the project's logging privacy rules.

**Related:** [Logging behavior and privacy](logging.md)

### Fishing

**Aliases:** auto fishing, cast, bite, reel, recast

The opt-in state machine that casts, waits for a valid PixelBeacon bite signal,
reels, and recasts through the configured Interact Key and timing boundaries.

**Related:** [Fishing setup and states](../features/fishing.md)

## G

### Game Context

**Aliases:** gameplay detection, menu state, focused game

The combined observation of focus and whether ESO is in normal gameplay rather
than a native menu or text-entry state. Unknown context cannot authorize input.

**Related:** [Game Observation states](../concepts/game-observation.md)

### Global Cooldown

**Aliases:** GCD, cooldown window

ESO's approximately 1000 ms skill-activation window. Weave timing places an
attack, block, or bash around the skill while respecting this shared cooldown.

**Related:** [Weaving timing model](../features/weaving.md)

## H

### Heartbeat

**Aliases:** beacon freshness, status block, B0

The recurring PixelBeacon status signal used to prove that screen observations
are still current. A heartbeat timeout moves dependent observations to Signal Lost.

**Related:** [Pixel Bus Protocol heartbeat](pixel-bus-protocol.md)

### Heavy Attack

**Aliases:** HA, hold attack

A weave type that holds the primary attack input for a weapon-aware duration
before the associated skill activation.

**Related:** [Weaving types and weapon timing](../features/weaving.md)

## I

### Input Safety

**Aliases:** key interception, key suppression, input hook, pass through

The authorization boundaries that keep interception focused on ESO, break
generated-input recursion, and let physical input pass through whenever a feature
or safety gate does not own it.

**Related:** [Input Safety guarantees](../concepts/input-safety.md)

### Interact Key

**Aliases:** use key, action key, E key

The ESO binding used for world interactions, E by default. Fishing uses this
configured action for casting, reeling, and recasting.

**Related:** [Fishing controls](../features/fishing.md)

### Invalid Configuration

**Aliases:** corrupt config, reset to defaults, .invalid file

A settings file that could not be parsed or migrated safely. ESO Weave preserves
the original with an `.invalid` suffix, loads defaults, and reports the recovery.

**Related:** [Configuration recovery](configuration.md)

## L

### Latency Adaptation

**Aliases:** ping adjustment, latency scaling, k factor

An optional adjustment that adds a bounded fraction of observed latency to
eligible weave delays without changing heavy-attack duration or the global cooldown.

**Related:** [Latency-aware weaving](../features/weaving.md)

### Layout Header

**Aliases:** H0, H1, H2, geometry negotiation, protocol version

The Pixel Bus header blocks that announce layout geometry and protocol identity
before payload blocks are interpreted.

**Related:** [Pixel Bus Protocol layout](pixel-bus-protocol.md)

### Life State

**Aliases:** alive, dead, reincarnating, player death

The observed player lifecycle used to distinguish Alive, Dead, Recovering, and
Unknown. Only a coherent fresh Alive state can contribute to action authorization.

**Related:** [Game Observation life states](../concepts/game-observation.md)

### Light Attack

**Aliases:** LA, left click, primary click

A weave type that taps the primary attack input before the associated skill
activation, using the configured light-attack timing.

**Related:** [Weaving types and timing](../features/weaving.md)

### Linux Input

**Aliases:** evdev, uinput, input group, udev rule

The Linux backend that reads focused physical input through evdev and emits
authorized generated input through uinput after device permissions are configured.

**Related:** [Scope and Platform support](../concepts/scope-and-platform.md)

### Live Log

**Aliases:** log viewer, in-memory log, ring buffer

The always-available in-application diagnostic view backed by a bounded memory
buffer, independent of whether persistent File Logging is enabled.

**Related:** [Logging surfaces](logging.md)

## M

### Managed Marker

**Aliases:** X-ESO-Weave-Managed, managed addon ownership, Unmanaged

The manifest line `## X-ESO-Weave-Managed: true` that proves ESO Weave owns an
addon subtree. Install-over-existing, update, and removal refuse an Unmanaged target.

**Related:** [PixelBeacon managed lifecycle](../features/pixelbeacon.md)

### Menu Gate

**Aliases:** chat protection, typing protection, text field, native menu

A safety authority that prevents interception and generated input while ESO
reports a native menu or text field, so ordinary typing remains untouched.

**Related:** [Action Authorization gates](../concepts/action-authorization.md)

## N

### No Cast Detected

**Aliases:** bait missing, arm timeout, cast timeout

A fishing outcome indicating that the expected cast observation did not arrive
within the arm window, commonly because bait or another cast prerequisite is absent.

**Related:** [Fishing recovery states](../features/fishing.md)

## P

### Payload Block

**Aliases:** beacon block, B0 through B28, marker, checksum

One solid-color square at a protocol-defined physical-pixel position. Its marker
identifies the carried signal, while payload channels and checksums protect data.

**Related:** [Pixel Bus Protocol blocks](pixel-bus-protocol.md)

### Pixel Bus

**Aliases:** pixelbus, screen telemetry, color protocol

The bounded block protocol rendered by PixelBeacon and sampled by ESO Weave to
carry minimal local game observations through on-screen colors.

**Related:** [Pixel Bus Protocol reference](pixel-bus-protocol.md)

### PixelBeacon

**Aliases:** addon, pixel addon, telemetry overlay, color blocks

The separately managed ESO addon that publishes a minimal set of game observations
as on-screen color blocks for local sampling by the desktop companion.

**Related:** [PixelBeacon feature guide](../features/pixelbeacon.md)

### PixelBeacon Status

**Aliases:** install addon, update addon, unmanaged addon

The desktop lifecycle state that distinguishes a missing, current, outdated, or
Unmanaged PixelBeacon installation and exposes only the safe applicable actions.

**Related:** [PixelBeacon installation states](../features/pixelbeacon.md)

## Q

### Quickslot

**Aliases:** potion slot, consumable wheel, Q key

ESO's active consumable slot, normally triggered with Q. Auto Potion can submit
that binding but does not choose or change the slotted item.

**Related:** [Auto Potion quickslot prerequisites](../features/auto-potion.md)

## R

### Ready

**Aliases:** enough Ultimate, cast-ready, readiness indicator

An Ultimate display state meaning the observed resource meets the active bar's
known cost. Ready describes evidence and does not itself submit an input.

**Related:** [Ultimate Resource readiness](../features/ultimate-resource.md)

### Release Package

**Aliases:** MSI, deb, AppImage, tarball

An official platform artifact produced from a tagged release, such as the Windows
installer or a Linux package, portable image, or archive.

**Related:** [Installation package choices](../getting-started/installation.md)

### Release Pipeline

**Aliases:** release workflow, tag build, publish release

The governed tag-triggered process that builds, checks, packages, signs where
configured, and publishes official ESO Weave artifacts.

**Related:** [Release and Packaging process](../development/release-and-packaging.md)

### Resource Watch

**Aliases:** health threshold, magicka threshold, stamina threshold, low resource

One independently enabled Health, Magicka, or Stamina threshold used by Auto
Potion to decide whether a resource condition requests an attempt.

**Related:** [Auto Potion resource watches](../features/auto-potion.md)

### Retry Interval

**Aliases:** retry floor, repeat protection, potion delay

The minimum interval between Auto Potion attempts in the same low-resource
episode, preventing rapid repeated quickslot submissions.

**Related:** [Auto Potion timing](../features/auto-potion.md)

### Roll Dodge

**Aliases:** dodge roll, dodge state, roll gate

The observed dodge movement state that temporarily gates action-driving features
and invalidates queued work from the earlier authorization epoch.

**Related:** [Game Observation movement states](../concepts/game-observation.md)

## S

### Session State

**Aliases:** state.json, saved window position, suspend persistence

Restart-restorable state kept separately from user settings, including supported
window placement and selected runtime intents such as suspension.

**Related:** [Configuration and session separation](configuration.md)

### Signal Lost

**Aliases:** beacon missing, heartbeat timeout, overlay hidden

The fail-closed condition raised when a fresh PixelBeacon heartbeat is absent.
Fishing disables rather than acting from stale screen observations.

**Related:** [Troubleshooting missing signals](../getting-started/troubleshooting.md)

### Skill Slot

**Aliases:** ability slot, skills 1 through 5, Ultimate slot, Synergy slot

One of seven automatable inputs: skills 1 through 5, Ultimate, and
Synergy. Each slot can be disabled or assigned its own Weave Type and delay override.

**Related:** [Weaving skill controls](../features/weaving.md)

### Sprinting

**Aliases:** sprint, movement state, mounted movement

The observed movement family that gates action-driving features while the player
is sprinting, including the separately interpreted mounted movement state.

**Related:** [Game Observation movement states](../concepts/game-observation.md)

### Startup Failure

**Aliases:** failed to start, startup panic, error dialog

An initialization error that prevents the desktop application from opening
normally and is surfaced through a visible platform dialog plus diagnostic logs.

**Related:** [Troubleshooting startup](../getting-started/troubleshooting.md)

### Suspension

**Aliases:** suspend, pause automation, F1

The operator-controlled pause state, toggled by the configured binding, that
prevents action-driving features while preserving safe control access.

**Related:** [Input Safety suspension](../concepts/input-safety.md)

### Synthesized Input

**Aliases:** generated input, injected input, SendInput, uinput

Keyboard or mouse events emitted by ESO Weave after authorization. They are
marked so the input engine never intercepts and recursively processes its own output.

**Related:** [Input Safety synthesis boundary](../concepts/input-safety.md)

## T

### Test Strategy

**Aliases:** unit tests, mock backend, headless UI, contract tests

The repository's layered evidence model combining pure state tests, platform
seams, contract fixtures, headless interface checks, and hosted integration gates.

**Related:** [Test Strategy](../development/test-strategy.md)

### Travel

**Aliases:** teleport, wayshrine, recall, zone change, jump pending

The requested and observed world-transition family covering recalls, wayshrines,
cross-zone jumps, and other loading boundaries that invalidate earlier input authority.

**Related:** [Game Observation travel states](../concepts/game-observation.md)

### Troubleshooting

**Aliases:** not working, no input, signal unavailable, game not active

The task-oriented diagnostic reference for startup, focus, input, addon, signal,
and feature-state problems, with safe checks before configuration changes.

**Related:** [Troubleshooting guide](../getting-started/troubleshooting.md)

## U

### Ultimate

**Aliases:** ult, Ultimate points, Ultimate resource

ESO's accumulated combat resource for the Ultimate skill slot. PixelBeacon can
publish the current amount and bar-specific known costs for display.

**Related:** [Ultimate Resource display](../features/ultimate-resource.md)

### Ultimate Cost

**Aliases:** cast cost, front-bar cost, back-bar cost, readiness tick

The observed point requirement for the Ultimate on each weapon bar. ESO Weave
uses the active bar's cost when deriving its non-acting readiness indicator.

**Related:** [Ultimate Resource costs](../features/ultimate-resource.md)

### Unknown

**Aliases:** unavailable, missing evidence, invalid signal, fail closed

A state meaning current evidence cannot authorize a conclusion. Unknown is not
equivalent to Ready, Active, safe, false, or a valid zero measurement.

**Related:** [Action Authorization evidence states](../concepts/action-authorization.md)

## W

### Weapon Bar

**Aliases:** front bar, primary bar, back bar, backup bar, bar swap

ESO's front or back weapon set. The active bar and each bar's weapon class affect
heavy-attack timing and Ultimate readiness.

**Related:** [Weapon-aware weaving](../features/weaving.md)

### Weave Delay

**Aliases:** d_weave, attack delay, skill delay

The configured pause between the selected attack action and skill activation in
a weave sequence, optionally adjusted by the bounded latency feature.

**Related:** [Weaving delay controls](../features/weaving.md)

### Weave Type

**Aliases:** LA, HA, BA, BL, attack pattern

The attack pattern assigned to a Skill Slot: Light Attack, Heavy Attack, Bash
Attack, or Block Casting, represented by LA, HA, BA, and BL in compact contexts.

**Related:** [Weaving types](../features/weaving.md)

### Weaving

**Aliases:** weave, animation cancel, LA weave

Fitting a basic attack, block, or bash into the same global-cooldown window as a
skill activation through an explicitly configured per-slot sequence.

**Related:** [Weaving feature guide](../features/weaving.md)

### Windows Input

**Aliases:** keyboard hook, WH_KEYBOARD_LL, SendInput

The Windows backend that uses a focused low-level keyboard hook for physical
events and SendInput for authorized generated events.

**Related:** [Scope and Platform support](../concepts/scope-and-platform.md)

### World State

**Aliases:** loading screen, world transition, player activation

The observed relationship between active gameplay and loading or activation
boundaries. Transition states invalidate queued authority until fresh evidence returns.

**Related:** [Game Observation world states](../concepts/game-observation.md)

## X

### XWayland

**Aliases:** Wayland, X11 compatibility, game capture

The X11 compatibility path used when ESO runs in a Wayland desktop session and
screen sampling or focus detection requires an X11-visible game surface.

**Related:** [Scope and Platform support](../concepts/scope-and-platform.md)
