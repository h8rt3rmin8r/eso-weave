---
description: "Task list for the fishing interaction detection rewrite (slice 025)"
---

# Tasks: Fishing Interaction Detection Rewrite

**Input**: Design documents from `specs/025-fishing-interaction-detection/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md,
contracts/detection.md

**Tests**: The controller change is log-only; the existing fishing and
pixelbus suites are the regression gate and must pass unchanged (FR-010).
The addon Lua has no test harness; its contract is validated by the
official-source citations in research.md and the in-game protocol in
quickstart.md, consistent with slices 004/014/016/024.

## Phase 1: Controller observability (US2)

- [x] T001 [US2] In `src/fishing/mod.rs` add DEBUG `tracing` lines (target
  `eso_weave::fishing`) per the data-model log map: in `set_enabled` (enable),
  in `cast()` (cast keypress sent, armed with deadline ms), in `on_event` for
  `FishingStarted` from Armed/Recast (cast detected, waiting), `BiteDetected`
  from Waiting (bite, reeling after reel delay), `FishingStopped` from Waiting
  (cast ended, recasting), in `tick()` for the reel deadline (reel sent,
  recast delay) and recast deadline (recasting), and in `disable()` with the
  `StopReason` variant. No state, event, or timer change.
- [x] T002 [US2] Run `cargo test --test fishing --locked` and confirm every
  existing test passes with zero modifications to `tests/fishing.rs`
  (FR-010 gate before proceeding).

## Phase 2: Addon detection rewrite (US1)

- [x] T003 [US1] In `addon/PixelBeacon/PixelBeacon.lua` replace the
  event-driven fishing detection with the poll-authoritative machine from
  `contracts/detection.md`: add a `FISHING_UPDATE_MS = 100` tick registered
  via `em:RegisterForUpdate(ADDON_NAME .. "Fishing", ...)` in `onAddOnLoaded`;
  the tick samples `GetInteractionType() == INTERACTION_FISH`, drives idle to
  waiting and waiting/bite to idle (clearing the bite timer via the existing
  `clearBite`/timeout machinery), and while waiting compares
  `GetGameCameraInteractableActionInfo()` against
  `GetString(SI_GAMECAMERAACTIONTYPE17)` to drive waiting to bite; the tick
  never demotes bite to waiting. Remove the
  `EVENT_CLIENT_INTERACT_RESULT` registration and the `onInteractResult`
  handler entirely; drop the now-unneeded `fishingInteractionActive` flag in
  favor of the polled state. `renderFishing`, colors, and block geometry are
  untouched.
- [x] T004 [US1] In `addon/PixelBeacon/PixelBeacon.lua` scope the secondary
  bite signal: `onInventorySlotUpdate` accepts the `itemSoundCategory`
  parameter and drives a bite only when `stackCountChange == -1`,
  `itemSoundCategory == ITEM_SOUND_CATEGORY_LURE`, the polled fishing state
  is not idle, and no menu is open; `isNewItem` still clears the bite
  (catch resolved). Keep the `EVENT_CHATTER_END` cleanup handler.
- [x] T005 [US1] In `addon/PixelBeacon/PixelBeacon.lua` remove the stale
  unused `ADDON_VERSION` local and update the header comment to describe the
  polling detection; in `addon/PixelBeacon/PixelBeacon.txt` advance
  `## Version:` and `## AddOnVersion:` from 3 to 4, leaving the managed-marker
  line untouched (US3 delivery; FR-008, FR-012).

## Phase 3: Documentation

- [x] T006 In `docs/ESO-Weave-Specification-v0.2.0.md` update the fishing
  detection language (sections 9/10.3 wording that describes the addon's
  cast and bite detection) to the polling contract: interaction-type poll for
  waiting, reel-in prompt comparison as primary bite, lure-scoped bait
  consumption as secondary, with the interact-result event explicitly retired.
- [x] T007 Update `CHANGELOG.md` `[Unreleased]` with a `Fixed` entry (fishing
  casts are now detected; the addon no longer listens on the interaction
  error-alert event) and a dated `Decisions` entry recording the
  detection-contract change and its official-source basis, plus an `Added`
  entry for the fishing-controller transition logging.

## Phase 4: Verification

- [x] T008 Run CI parity in the foreground and watch to completion:
  `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --all --locked`. Fix any findings.
- [ ] T009 In-game validation per `quickstart.md` (owed to the operator, as
  slice 024 T010 was): update the addon to version 4, `/reloadui`, then run
  the cast-detection, bite-cycle, false-bite, no-bait, interruption, and stop
  scenarios, confirming SC-001 through SC-004 and capturing the debug log.

---

## Dependencies & Execution Order

- T001 before T002 (the regression gate covers the new logging).
- T003 before T004 (the scoped inventory handler reads the polled state);
  T005 last in the addon phase (version advances only over final content).
- Phase 3 after Phase 2 (documentation describes the landed contract).
- T008 gates the commit; T009 is the operator's in-game verification and the
  release gate for the fishing feature.

## Notes

- Safety-critical surfaces are untouched: signal-loss degradation, the
  beacon managed-marker uninstall, and the input engine are not modified;
  T002/T008 prove the suites pass unchanged.
- T008 finding: the manifest advance required updating the version-pin test
  (`tests/beacon.rs`, `embedded_manifest_version_is_three` renamed to
  `embedded_manifest_version_is_four`), the same pin update slice 016 made
  for 2 to 3. `tests/fishing.rs` and `tests/pixelbus.rs` are unmodified, so
  the FR-010 regression gate holds.
- The rendered signal contract (colors, positions, geometry) is frozen; any
  diff to `renderFishing` colors or `positionBlock` geometry is out of scope
  and would violate FR-007.
- Commit as `feat(025): fishing interaction detection rewrite` after T008;
  halt before push per the autopilot protocol. No pinned artifacts are
  touched.
