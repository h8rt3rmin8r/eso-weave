# Implementation Plan: Fishing Bite Signal Correction

**Branch**: `026-fishing-bite-signal` | **Date**: 2026-07-13 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/026-fishing-bite-signal/spec.md`

## Summary

Slice 025 fixed cast detection but introduced a false bite: it treated the
reticle prompt matching the localized reel-in string as the primary bite
signal, when that prompt is in fact the standing interact prompt for the
entire time the line is in the water. The v0.6.1 field log proves it: "bite
detected" fired on the first poll tick 200 to 800 ms after every cast, the
app reeled 100 ms later, recast, and looped at about two casts per second,
consuming bait each cycle. The correction is a deletion: remove the
prompt-comparison block from the addon's fishing tick and let the already
implemented, correctly scoped bait-consumption inventory event be the sole
bite signal, exactly as both proven references do. Addon advances to
version 5; no Rust behavior changes; spec section 10.2 and the changelog
are corrected.

## Technical Context

**Language/Version**: ESO addon Lua 5.1 (APIVersion 101050) for the fix;
Rust 1.96 only for the version-pin test constant.

**Primary Dependencies**: none new. The addon keeps
`EVENT_MANAGER:RegisterForUpdate` (100 ms tick), `GetInteractionType()`,
`INTERACTION_FISH`, `EVENT_INVENTORY_SINGLE_SLOT_UPDATE`,
`ITEM_SOUND_CATEGORY_LURE`, `EVENT_CHATTER_END`, `SCENE_MANAGER:IsShowing`.
`GetGameCameraInteractableActionInfo` and `SI_GAMECAMERAACTIONTYPE17` leave
the addon entirely.

**Storage**: none.

**Testing**: `cargo test` (existing suites; only `tests/beacon.rs` version
pin advances 4 to 5). The addon Lua has no harness; the corrected contract
rests on the field-log evidence and the two reference implementations in
research.md, validated by the quickstart in-game protocol.

**Target Platform**: ESO live client (addon); Windows/Linux app unchanged.

**Performance Goals**: strictly less work per tick than v4 (the prompt
sample and string comparison are removed).

**Constraints**: rendered colors, block geometry, decoder, controller
behavior, and timings byte-identical; safety-critical surfaces untouched;
text hygiene holds.

**Scale/Scope**: one Lua block deleted plus comment corrections, one
manifest bump, one test constant, one spec subsection, one changelog entry
set.

## Constitution Check

- **I. Spec-Driven Development**: full spec-kit sequence; traces to master
  spec sections 8, 9, and 10.2 via build plan 008. PASS.
- **II. Safety-Critical Surfaces**: nothing in scope touches them; fishing
  still degrades to disabled on SignalLost; managed-marker uninstall
  untouched. PASS.
- **III. Test-First With Explicit Seams**: no Rust behavior change; the
  suites are the regression gate and stay green (only the version-pin
  constant advances, as in slices 016 and 025). The addon Lua remains the
  documented out-of-harness exception; its contract is validated in-game.
  PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), test
  --all --locked in the foreground before commit. PASS.
- **V. Bounded Scope**: the change narrows in-game behavior (removes a
  sample), stays inside the screen-signal contract. PASS.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/026-fishing-bite-signal/
|-- plan.md              # This file
|-- spec.md              # Feature specification
|-- research.md          # Phase 0 field-log evidence and reference citations
|-- data-model.md        # Phase 1 corrected addon state machine
|-- quickstart.md        # Phase 1 in-game validation protocol
|-- contracts/
|   `-- bite.md          # Corrected bite-signal behavioral contract
|-- checklists/
|   |-- requirements.md
|   `-- bite.md
`-- tasks.md             # Created next
```

### Source Code (repository root)

```text
addon/
`-- PixelBeacon/
    |-- PixelBeacon.lua  # Prompt-comparison bite block removed from the
    |                    #   fishing tick; comments corrected
    `-- PixelBeacon.txt  # ## Version / ## AddOnVersion advance to 5
tests/
`-- beacon.rs            # Version pin 4 -> 5
docs/
`-- ESO-Weave-Specification-v0.2.0.md   # section 10.2 corrected
CHANGELOG.md             # Fixed entry + dated decision correcting slice 025
```

**Structure Decision**: single crate retained; no new dependency; no Rust
source change outside the test constant.

## Key Decisions (autopilot decision log)

- **Deletion over redesign.** The bait-consumption event was already
  implemented and correctly scoped in v4; the false trigger is simply
  removed. This restores exact parity with both proven references
  (InfoPanel 1.63's reel alert; FishingStateMachine's reelin transition)
  rather than inventing a third contract.
- **The prompt leaves the addon entirely.** Keeping the prompt sample as a
  "waiting confirmation" was rejected: `GetInteractionType()` already
  answers that question authoritatively, and retaining the call invites the
  same misreading later.
- **No minimum-wait heuristic.** Both references ship without one; the lure
  scoping makes the signal precise; a heuristic could suppress legitimately
  fast bites. Recorded as considered and rejected.
- **Controller bite-from-Armed acceptance retained.** A real bite can land
  between the waiting render and the reader's next 100 ms sample, so green
  before blue is legitimate at the reader; tightening it would drop real
  bites for no safety gain.
- **Benign failure mode accepted for the residual unknown.** If the lure
  event does not fire on a real strike (field-unverified until the in-game
  run), the app waits, the fish escapes, the interaction ends, and
  FishingStopped recasts; no reel fires, no bait is spent by the app, and
  the debug log shows casts with no bite line, cleanly scoping any
  follow-up.

## Phasing

- Phase 0 (research.md): field-log evidence, prompt semantics, reference
  citations.
- Phase 1 (data-model.md, contracts/bite.md, quickstart.md): corrected
  state machine, bite contract, in-game validation.
- Phase 2 (tasks.md): task list.

## Complexity Tracking

No constitution violations; no entries.
