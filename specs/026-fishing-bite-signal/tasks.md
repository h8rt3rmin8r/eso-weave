---
description: "Task list for the fishing bite signal correction (slice 026)"
---

# Tasks: Fishing Bite Signal Correction

**Input**: Design documents from `specs/026-fishing-bite-signal/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md,
contracts/bite.md

**Tests**: No Rust behavior change; the full suite is the regression gate
with `tests/fishing.rs` and `tests/pixelbus.rs` unmodified (SC-004). The
addon Lua has no harness; the corrected contract rests on the field-log
evidence and reference citations in research.md, validated in-game per
quickstart.md.

## Phase 1: Addon correction (US1)

- [x] T001 [US1] In `addon/PixelBeacon/PixelBeacon.lua` remove the
  prompt-comparison bite block from `onFishingTick` (the
  `if fishingState == "waiting" then ... GetGameCameraInteractableActionInfo
  ... GetString(SI_GAMECAMERAACTIONTYPE17) ... onBite()` block), leaving the
  tick with only: interaction-ended to idle (with bite-timer clear) and idle
  to waiting. Correct the file-header comment and the tick comment to state
  that the reel-in prompt is the standing cast prompt and the bite is
  detected solely from bait consumption. No other logic changes; the
  inventory handler, rendering, and geometry stay byte-identical.
- [x] T002 [US1] In `addon/PixelBeacon/PixelBeacon.txt` advance
  `## Version:` and `## AddOnVersion:` from 4 to 5, leaving every other
  line (including the managed marker) untouched (US2 delivery; FR-006,
  FR-008).
- [x] T003 [US1] In `tests/beacon.rs` advance the version-pin test
  (`embedded_manifest_version_is_four` to `..._is_five`, expected value 5),
  matching the slice 016 and 025 precedent.

## Phase 2: Documentation

- [x] T004 In `docs/ESO-Weave-Specification-v0.2.0.md` section 10.2 correct
  the fishing detection contract: delete the reel-in-prompt primary-bite
  bullet, make the lure-scoped bait-consumption event the sole bite signal,
  and add the explicit statement that the reel-in prompt is the standing
  interact prompt for the whole cast and is never consulted.
- [x] T005 Update `CHANGELOG.md` `[Unreleased]` with a `Fixed` entry (the
  app no longer reels immediately after casting; the standing reel-in
  prompt was misread as a bite, wasting bait) and a dated `Decisions` entry
  correcting the 2026-07-13 slice 025 decision (prompt removed from the
  contract; bait consumption is the sole bite signal, citing the field log
  and both references per research.md).

## Phase 3: Verification

- [x] T006 Run CI parity in the foreground and watch to completion:
  `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`. Fix any findings.
- [ ] T007 In-game validation per `quickstart.md` (owed to the operator):
  update the addon to version 5, `/reloadui`, then run the no-early-reel,
  real-bite-cycle, false-bite, escape-path, and stop scenarios, confirming
  SC-001 through SC-003 with one bait per catch.

---

## Dependencies & Execution Order

- T001 before T002 (version advances over final content); T003 with T002.
- Phase 2 after Phase 1 (documentation describes the landed contract).
- T006 gates the commit; T007 is the operator's in-game verification and
  the release gate.

## Notes

- Safety-critical surfaces untouched; `tests/fishing.rs` and
  `tests/pixelbus.rs` must show no diff.
- Commit as `feat(026): fishing bite signal correction` after T006; halt
  before push per the autopilot protocol. No pinned artifacts touched.
