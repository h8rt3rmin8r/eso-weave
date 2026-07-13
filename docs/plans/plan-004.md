# Build Plan 004: Fishing Reliability and Usage Documentation

Plan: 004
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plans 001 through 003 delivered the functional product, brand and UX
polish, a GUI overhaul, and weapon-bar-aware timing (through v0.4.2). Field
testing of the fishing feature then surfaced a real reliability defect: the app
arms fishing but reverts to Idle within a few seconds and never reels in a catch.
This plan adds two slices: a fishing reliability and status-collaboration slice
that fixes the runtime defects and makes the app explain itself, and a
documentation slice that gives users the correct fishing and weaving usage
model in the README.

It traces to the master specification's fishing module (section 8), the pixel
bus and its blocks (section 9), and the GUI layer (section 10). Slice 016 touches
safety-critical surfaces: the fishing controller and its no-blocking-on-the-hook
thread and SignalLost-cancels-pending-interact behavior, and the PixelBeacon
addon manifest (its managed-marker line stays unchanged so marker-gated safe
uninstall is unaffected). The addon manifest change carries a dated decision in
`CHANGELOG.md` and closes open item R4 (confirming the live ESO API version).
Slice 017 is documentation only and touches no source or safety surface.

(Slice 015, hotkey-detection-fixes, exists under `specs/015-hotkey-detection-fixes/`
but was never recorded in a build plan or this index; it is noted here for
traceability and is not in scope for this plan.)

## Slices

### Slice 016: Fishing Reliability and Status Collaboration

Scope: fix the fishing feature so a cast reliably progresses to a catch, and make
the app communicate its state. Refresh the PixelBeacon manifest `## APIVersion`
from the stale 101044 to the current live value plus a future value (ESO's
two-value form) so the game stops flagging the addon Out of Date and loads it,
and bump `## Version` and `## AddOnVersion` so existing installs are refreshed;
this closes R4. Make the pixel-bus worker loop poll at the fishing interval
(roughly 100 ms) while fishing is active instead of always at the idle interval,
so transient beacon signals are sampled and the state machine ticks in time.
Drive the fishing controller from a single shared monotonic clock so deadlines
are stamped and evaluated on one timeline. Tune the default fishing timeouts with
recorded rationale now that polling is fixed. Replace the raw debug state text
with plain-language status labels and tooltips, and record and surface the reason
fishing returned to Idle (user stop, no cast detected, or signal lost) so an
early stop is explained rather than silent. All correctness logic lands in tested
pure helpers and the controller; the egui layer stays thin, and the
safety-critical fishing behaviors stay tested. Feature under `specs/016-<name>/`.

### Slice 017: Fishing and Weaving Usage Documentation

Scope: add two detailed usage sections to the project README, one for fishing and
one for weaving, and reorder the README so the Disclaimer becomes the
next-to-last section immediately before the License. The fishing section
documents the interaction model (F2 casts for you; do not cast manually first),
the PixelBeacon prerequisites (installed, enabled, not Out of Date, beacon strip
visible, window focused), the status progression the user will see, the interact
key and configurable timings, and troubleshooting tied to the reported symptom.
The weaving section documents the single-bar overview, the seven skill slots and
their defaults, the four weave types, and the default timings (global cooldown
500 ms, weave 50 ms, heavy 1000 ms, bash 125 ms); dual-bar weaving mechanics are
on hold and are intentionally not documented. Documentation only; no source
changes. Feature under `specs/017-<name>/`.
