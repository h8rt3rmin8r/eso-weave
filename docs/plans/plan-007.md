# Build Plan 007: Fishing Interaction Detection Rewrite

Plan: 007
Status: active
Master specification: `docs/ESO-Weave-Specification-v0.2.0.md`
Constitution: `.specify/memory/constitution.md`

## Purpose

Build plan 006 closed with slice 024 hardening the pixel-bus screen capture,
and the field evidence from 2026-07-13 confirms that fix worked: the heartbeat
block and the weapon-bar block now decode correctly on the live game, bar
swaps update in the interface immediately, and the reader's fast fishing
cadence engages on arm. The same evidence shows the fishing block itself never
turns blue: four consecutive casts each ran the fast cadence for the full arm
window and ended in `disable(NoCastDetected)`. Slice 024's research scoped
exactly this outcome as a follow-up at the addon interaction-detection
contract, and this plan is that follow-up.

The root cause is now established against the current official game interface
source (github.com/esoui/esoui, branch `live`, pushed 2026-06-29, matching the
installed game 12.0.6 at APIVersion 101050). PixelBeacon's fishing detection
hangs entirely on `EVENT_CLIENT_INTERACT_RESULT`, which the official interface
registers as an error alert handler: it carries `CLIENT_INTERACT_RESULT_*`
failure codes formatted through the `SI_CLIENTINTERACTRESULT` error strings.
A clean successful cast never produces it, the addon has no polling fallback,
and so the waiting signal is never rendered. The game's own reticle
demonstrates the correct pattern: it polls `GetInteractionType()` every frame
and treats `INTERACTION_FISH` as the live interaction type for the whole
cast-wait-bite window, and the current string table defines the reticle action
"Reel In" (`SI_GAMECAMERAACTIONTYPE17`) that appears while a fish is hooked.

This plan traces to the master specification's fishing module (sections 8 and
9) and the pixel-bus and PixelBeacon signal contract (section 10.3). It
contains one slice. The slice touches the addon Lua (unchanged since slice
014), adds observability to the fishing controller (which today emits a single
warning line and narrated nothing across four failed field sessions), and
updates the specification's detection-contract language with a dated decision
in `CHANGELOG.md`. The pixel colors, block geometry, and the Rust decode
contract are unchanged; the reader already expects exactly what the corrected
addon will render.

## Slices

### Slice 025: Fishing Interaction Detection Rewrite

Scope: replace PixelBeacon's one-shot event-driven fishing detection with
poll-authoritative detection mirroring the game's own reticle, and instrument
the fishing controller so a failure can never again produce a silent log.

In the addon, a registered update tick (on the order of 100 to 150
milliseconds) evaluates `GetInteractionType() == INTERACTION_FISH` as the
authoritative cast signal: true drives idle to waiting, false drives waiting
or bite back to idle and clears the bite safety timer. While the fishing
interaction is active, the same tick compares the action returned by
`GetGameCameraInteractableActionInfo()` against
`GetString(SI_GAMECAMERAACTIONTYPE17)` ("Reel In"), a locale-safe comparison
because both sides are localized, and a match drives waiting to bite as the
primary bite signal. The bait-consumption inventory event remains as the
secondary bite signal, now scoped to the equipped bait via
`itemSoundCategory == ITEM_SOUND_CATEGORY_LURE` (the current code accepts any
single-stack decrease, which violates the contract's equipped-bait wording).
The poll never demotes an active bite back to waiting; a bite clears on
new-item gain, on the existing five-second safety timeout, or on the
interaction ending. The `EVENT_CLIENT_INTERACT_RESULT` dependency is removed
entirely, the chatter-end cleanup and menu-open suppression stay, the stale
unused `ADDON_VERSION` local is removed, and the manifest version advances so
the beacon manager offers the update.

In the application, the fishing controller gains debug-level transition
logging under the `eso_weave::fishing` target: cast sent, armed entered,
armed to waiting, bite to reeling, reel to recast, and every disable with its
stop reason. This is log-only with no behavior change; the existing
controller tests pass unchanged.

The master specification's fishing-detection language is updated to the
polling contract with the official-source citations, and `CHANGELOG.md`
records a dated decision. The feature's `research.md` records the esoui
source citations (the alert-handler registration, the reticle poll, the
string-table entries, and the absence of any fishing lure events in the
current API) so the API basis is auditable. Safety invariants are restated
and unchanged: fishing degrades to disabled on signal loss, PixelBeacon
uninstall still verifies the managed-marker line before deleting, and nothing
reaches beyond the screen-signal contract. Migrating the operator's persisted
`arm_timeout_ms` of 5000 to the newer 8000 default was considered and
rejected: with polling, the waiting signal lands well under one second after
the cast, so the shorter window is ample. The feature `plan.md` defines the
in-game validation run: install the updated addon and reload the interface,
select bait, arm fishing at a hole, and confirm through the new fishing
debug lines and the existing pixel-bus trace that the fishing block turns
blue within the poll interval, the interface advances from Casting to Fishing
to Reeling, and the recast cycle loops. Feature under `specs/025-<name>/`.
