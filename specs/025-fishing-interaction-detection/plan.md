# Implementation Plan: Fishing Interaction Detection Rewrite

**Branch**: `025-fishing-interaction-detection` | **Date**: 2026-07-13 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/025-fishing-interaction-detection/spec.md`

## Summary

Slice 024 proved the capture path works (heartbeat and weapon-bar blocks decode
on the live game) and scoped the remaining fault: the fishing block never turns
blue because the addon's detection never fires. The root cause, verified
against the current official game interface source, is that PixelBeacon's
entire fishing chain hangs on `EVENT_CLIENT_INTERACT_RESULT`, which the
official interface registers as an error-alert handler; a clean successful
cast never produces it, and the addon has no polling fallback. Rewrite the
addon's fishing detection to be poll-authoritative, mirroring the game's own
reticle: a periodic tick samples `GetInteractionType() == INTERACTION_FISH`
for the waiting state and compares the reticle action against the localized
"Reel In" string for the primary bite signal, with bait consumption retained
as a scoped secondary bite signal. Instrument the Rust fishing controller with
debug transition logging so a failed session is diagnosable from the log
alone. The rendered signal contract and the Rust decoder are byte-for-byte
unchanged.

## Technical Context

**Language/Version**: Rust 1.96 (edition 2021) for the controller logging; ESO
addon Lua 5.1 (APIVersion 101050) for the detection rewrite.

**Primary Dependencies**: `tracing` (already a dependency) on the Rust side.
On the addon side, current ESO API surfaces only: `EVENT_MANAGER:RegisterForUpdate`,
`GetInteractionType()`, `INTERACTION_FISH`,
`GetGameCameraInteractableActionInfo()`, `GetString(SI_GAMECAMERAACTIONTYPE17)`,
`EVENT_INVENTORY_SINGLE_SLOT_UPDATE`, `ITEM_SOUND_CATEGORY_LURE`,
`EVENT_CHATTER_END`, `SCENE_MANAGER:IsShowing`. No libraries.

**Storage**: none. No config or persisted change; the persisted
`arm_timeout_ms` is deliberately not migrated (see decision log).

**Testing**: `cargo test` for the fishing controller (existing tests must pass
unchanged; new assertions only where they do not weaken existing surfaces).
The addon Lua has no test harness; correctness rests on the official-source
citations in research.md and the in-game protocol in quickstart.md.

**Target Platform**: Windows 10/11 x64 and Linux x64 (Rust); the ESO live
client (addon).

**Performance Goals**: One interaction-type sample and at most one
action-info sample per 100 ms tick inside the game's UI frame; the game's own
reticle performs the same checks every frame, so the cost is negligible.

**Constraints**: Outside the game beyond the screen-signal contract; rendered
colors, block positions, and geometry byte-identical to the current contract;
safety-critical surfaces untouched; text hygiene holds (UTF-8 no BOM, LF, no
em/en dashes).

**Scale/Scope**: One addon Lua rewrite of the fishing-detection section, one
manifest version bump, debug logging in one Rust module, one spec-language
update, one changelog decision.

## Constitution Check

- **I. Spec-Driven Development**: Full spec-kit sequence; traces to master
  spec sections 8 and 9 (fishing) and 10.3 (signal contract) via build plan
  007. PASS.
- **II. Safety-Critical Surfaces**: Fishing still degrades to disabled on
  SignalLost (controller paths unchanged; logging is side-effect free); the
  beacon managed-marker uninstall is untouched; no input-engine change. PASS.
- **III. Test-First With Explicit Seams**: The controller change is log-only
  and covered by the existing test suite, which must stay green unchanged; a
  new test asserts transitions still behave identically with logging in
  place. The addon Lua sits outside the trait-seam test surface by nature
  (documented exception, consistent with slices 004/014/016/024); its
  contract is validated by the quickstart in-game protocol. PASS.
- **IV. CI Parity Before Every Commit**: fmt, clippy (-D warnings), test
  --all --locked in the foreground before commit. PASS.
- **V. Bounded Scope: Outside The Game**: The addon change stays within the
  PixelBeacon screen-signal contract (same blocks, same colors); detection
  reads only public addon API state. PASS.

No violations. Complexity Tracking is empty.

## Project Structure

### Documentation (this feature)

```text
specs/025-fishing-interaction-detection/
|-- plan.md              # This file
|-- spec.md              # Feature specification
|-- research.md          # Phase 0 root-cause evidence and API citations
|-- data-model.md        # Phase 1 addon state machine and controller log map
|-- quickstart.md        # Phase 1 in-game validation protocol
|-- contracts/
|   `-- detection.md     # Addon fishing-detection behavioral contract
|-- checklists/
|   |-- requirements.md
|   `-- detection.md
`-- tasks.md             # Created next
```

### Source Code (repository root)

```text
addon/
`-- PixelBeacon/
    |-- PixelBeacon.lua  # Fishing detection rewritten poll-authoritative;
    |                    #   rendering, geometry, weapon-bar, latency, and
    |                    #   status blocks unchanged
    `-- PixelBeacon.txt  # ## Version / ## AddOnVersion advance to 4
src/
`-- fishing/
    `-- mod.rs           # DEBUG transition logging (target eso_weave::fishing)
docs/
`-- ESO-Weave-Specification-v0.2.0.md   # fishing-detection contract language
CHANGELOG.md             # dated decision + Fixed entry
```

**Structure Decision**: Single crate retained; the change is confined to the
embedded addon, the fishing module, and docs. No new dependency.

## Key Decisions (autopilot decision log)

- **Poll-authoritative detection at 100 ms.** The addon registers a dedicated
  `RegisterForUpdate` fishing tick at 100 ms (spec bound: no coarser than
  150 ms). 100 ms matches the reader's fishing-cadence sample interval, so a
  state change is rendered within one reader sample; the game's own reticle
  runs the identical checks every frame, so a 100 ms tick is conservatively
  cheap. Chosen over 150 ms (no cost saving worth the added latency) and over
  piggybacking on the existing 1 Hz latency tick (too coarse; would eat most
  of the arm window).
- **`EVENT_CLIENT_INTERACT_RESULT` removed, not demoted.** The official
  interface registers it as an error-alert formatter; keeping it as an
  "accelerator" would preserve a false mental model and it contributes
  nothing the 100 ms poll does not. The chatter-end handler stays as cheap
  cleanup, and menu-open suppression stays on the inventory signal.
- **Two bite signals, either fires, poll wins on clearing.** Primary: while
  the fishing interaction is active, the reticle action equals
  `GetString(SI_GAMECAMERAACTIONTYPE17)` ("Reel In"); this is the mechanism a
  currently maintained addon (InfoPanel 1.63, APIVersion 101050) ships today,
  and it is locale-safe because both comparands come from the game's string
  table. Secondary: the equipped bait's stack decreases by one
  (`ITEM_SOUND_CATEGORY_LURE` scoping added; the current code accepts any
  stack decrease, a false-positive defect). The poll never demotes bite to
  waiting; bite clears on new-item gain, the existing 5 s safety timeout, or
  the interaction ending (poll observes `INTERACTION_FISH` false).
- **Controller logging at DEBUG under `eso_weave::fishing`.** One line per
  transition: cast keypress sent (with armed deadline), Armed to Waiting,
  Waiting to Reeling (bite), reel sent (with recast deadline), recast cast,
  and every disable with its `StopReason`. DEBUG keeps the idle log quiet at
  INFO while making every field session diagnosable at the default
  trace/debug capture the operator already runs. The log sink has no target
  filtering, so no logging-infrastructure change is needed.
- **No arm-timeout migration.** The operator's persisted `arm_timeout_ms`
  of 5000 stays. With the poll, the waiting signal lands within ~200 ms of
  the cast (one addon tick plus one reader sample), so 5000 is ample margin;
  a schema migration for a value that now works is churn without benefit.
- **Addon manifest advances to version 4.** Both `## Version` and
  `## AddOnVersion` advance so the beacon manager's existing update path
  offers the new addon; the stale, never-read `ADDON_VERSION` local in the
  Lua is removed rather than synchronized, so the manifest stays the single
  version authority.

## Phasing

- Phase 0 (research.md): the official-source evidence chain (error-alert
  registration, reticle polling, string-table constants, absent lure events)
  and the decision rationale.
- Phase 1 (data-model.md, contracts/detection.md, quickstart.md): the addon
  state machine, the detection contract, the controller log map, and the
  in-game validation protocol.
- Phase 2 (tasks.md): test-first task list.

## Complexity Tracking

No constitution violations; no entries.
