# Build Plan 008: Fishing Bite Signal Correction

Plan: 008
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plan 007 (slice 025, released as v0.6.1) fixed cast detection and
instrumented the fishing controller, and the field session of 2026-07-13
20:20 proves both worked: the cast was detected and the log narrates the
whole run. The same log pinpoints the one remaining defect, which slice 025
introduced: "bite detected" fired 200 to 800 milliseconds after every cast,
on the first poll tick after the fishing interaction began, and the app
reeled immediately, recast, and repeated at roughly two casts per second,
consuming bait on every cycle until the arm timeout ended the session.

The root cause is a wrong semantic inference in slice 025's detection
contract. The reticle prompt matching the localized reel-in string
(`SI_GAMECAMERAACTIONTYPE17`) was treated as the primary bite signal, but
that prompt is the standing interact prompt for the entire time the line is
in the water (which is how a player reels in early manually); it means "a
cast is active", never "a fish is hooked". Both proven references agree on
the correct signal: InfoPanel 1.63 (APIVersion 101050) triggers its reel
alert on the bait-consumption inventory event and uses the prompt only as a
currently-fishing confirmation, and fishyboteso's FishingStateMachine enters
its reelin state on the inventory event alone, never comparing the action
string. Slice 025 already implemented exactly that event, correctly scoped
to the lure sound category, as its secondary signal; the correction is to
delete the wrong trigger and let the proven signal stand alone.

This plan traces to the master specification's fishing module (sections 8
and 9) and the fishing detection contract (section 10.2). It contains one
slice, addon-plus-docs in scope, with no Rust behavior change.

## Slices

### Slice 026: Fishing Bite Signal Correction

Scope: make the lure-scoped bait-consumption inventory event the sole bite
signal in PixelBeacon, removing the prompt-comparison trigger that slice 025
added.

In the addon, the prompt-comparison block is removed from the fishing tick:
the tick's only duties are driving idle to waiting while
`GetInteractionType() == INTERACTION_FISH`, driving waiting or bite back to
idle when the interaction ends, and never demoting a rendered bite. The bite
is driven solely by the existing inventory handler: a single-stack decrease
carrying `ITEM_SOUND_CATEGORY_LURE` while a cast is active and no menu is
open, byte-for-byte the trigger InfoPanel 1.63 ships today and the
FishingStateMachine reelin transition. The header comment is corrected and
the manifest version advances from 4 to 5 so the beacon manager offers the
update. The rendered colors, block geometry, and the application decoder are
unchanged, as are the controller and its timings; the version-pin test
advances with the manifest as it did in slices 016 and 025. The controller's
acceptance of a bite while armed is deliberately retained: with a correct
addon, a real bite can land between the waiting render and the reader's next
sample.

A minimum-wait heuristic before accepting a bite was considered and
rejected: both proven references ship without one, the lure scoping already
makes the signal precise, and a heuristic could suppress legitimately fast
bites. The known residual risk is recorded: the lure event firing on a real
bite is field-unverified until the in-game run, and if it did not fire the
failure mode is now benign and diagnosable (the app waits, the fish escapes,
the interaction ends, and the controller recasts; no bait is spent by the
application, and the debug log shows casts with no bite line).

The master specification's section 10.2 is corrected to describe the single
bite signal and to document the reel-in prompt as the standing cast prompt
rather than a bite indicator, and `CHANGELOG.md` records a `Fixed` entry
plus a dated decision correcting the slice 025 decision. The feature's
research notes record the field-log evidence and both reference citations.
Safety invariants are unchanged: fishing degrades to disabled on signal
loss, and the managed-marker uninstall guarantee holds. The feature
quickstart defines the in-game validation: update the addon to version 5,
reload the interface, cast with bait, confirm the app waits without reeling
until a real bite, then reels and recasts, across at least three
consecutive catches with exactly one bait consumed per catch, and confirm a
consumable used mid-wait does not trigger a reel. Feature under
`specs/026-<name>/`.
