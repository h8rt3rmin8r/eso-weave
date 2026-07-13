# Research: Fishing Interaction Detection Rewrite

All game-API findings below were verified on 2026-07-13 against the official
ESO user-interface source dump, `github.com/esoui/esoui`, branch `live`,
pushed 2026-06-29, which matches the installed game (version 12.0.6,
APIVersion 101050 per the application's persisted state). This supersedes the
older third-party references consulted in earlier slices; the operator
correctly insisted on current evidence.

## R1: Why the fishing block never turns blue (root cause)

**Decision**: Treat `EVENT_CLIENT_INTERACT_RESULT` as unusable for detecting
a successful cast, and remove it from the addon.

**Rationale**: The 2026-07-13 field log shows the slice 024 capture fix
working (heartbeat acquired, weapon bar decoding, fast fishing cadence
engaging on arm) while every cast ended in the arm timeout with
`NoCastDetected`: the fishing block genuinely never rendered the waiting
color. The addon renders waiting only from its `EVENT_CLIENT_INTERACT_RESULT`
handler. In the official interface source, that event's sole consumer is the
alert-text system:

```lua
-- esoui/ingame/alerttext/alerthandlers.lua:1387
AlertHandlers[EVENT_CLIENT_INTERACT_RESULT] = function(result, interactTargetName)
    local formatString = GetString("SI_CLIENTINTERACTRESULT", result)
    if formatString ~= "" then
        return ERROR, zo_strformat(formatString, interactTargetName),
            ZO_ClientInteractResultSpecificSound[result] or SOUNDS.GENERAL_ALERT_ERROR
    end
end
```

It carries `CLIENT_INTERACT_RESULT_*` failure codes rendered as red error
alerts with an error sound. A clean successful cast produces no such alert,
so the handler never runs in the success path, `fishingInteractionActive`
stays false, the waiting color is never rendered, and the bite path is gated
off behind it. Slice 024's research anticipated exactly this residual fault
("present heartbeat, never-blue fishing block: an addon interaction-detection
problem") and deferred it to this slice.

**Alternatives considered**: Keeping the event as an accelerator alongside a
poll was rejected; it preserves a false mental model and adds nothing a
100 ms poll does not.

## R2: The correct cast/waiting signal

**Decision**: Poll `GetInteractionType() == INTERACTION_FISH` on a dedicated
100 ms `RegisterForUpdate` tick as the authoritative waiting signal.

**Rationale**: The game's own reticle does precisely this, every frame:

- `esoui/ingame/reticle/reticle.lua:59` installs an `OnUpdate` handler;
  `:348` calls `UpdateInteractText` from it; `:310` reads
  `local interactionType = GetInteractionType()`.
- `:334` computes the busy state as
  `interactionType ~= INTERACTION_NONE and interactionType ~= INTERACTION_FISH
  and ...`, keeping the interact button enabled while fishing, which is how
  the player can reel in. This proves `GetInteractionType()` returns
  `INTERACTION_FISH` for the entire cast-wait-bite window in the current
  client, and that per-frame polling of it is an officially sanctioned cost.

A 100 ms tick (10 Hz) is far coarser than the game's own per-frame usage and
matches the reader's fishing-cadence sample interval, so an addon state
change reaches the rendered block within one reader sample.

**Alternatives considered**: Event-driven detection was retired by R1. No
fishing-start event exists: a search of the official source for
`EVENT_FISHING_LURE` returns zero hits, so there is no cleaner event to
subscribe to. Piggybacking the existing 1 Hz latency tick was rejected as too
coarse against a 5000 ms arm window.

## R3: The bite signal

**Decision**: Primary signal: while the fishing interaction is active, the
reticle action string from `GetGameCameraInteractableActionInfo()` equals
`GetString(SI_GAMECAMERAACTIONTYPE17)`. Secondary signal: the equipped bait's
stack decreases by one, scoped by `itemSoundCategory ==
ITEM_SOUND_CATEGORY_LURE`. Either fires the bite; the poll never demotes it.

**Rationale**: The current string table defines the fishing action pair:

```lua
-- esoui/lang/en_client.lua:3257-3258
SAS(SI_GAMECAMERAACTIONTYPE16, "Fish", 0)
SAS(SI_GAMECAMERAACTIONTYPE17, "Reel In", 0)
```

While a fish is hooked the reticle action becomes "Reel In"; comparing
against `GetString(SI_GAMECAMERAACTIONTYPE17)` is locale-safe because both
comparands come from the same localized table. This exact mechanism ships
today in InfoPanel 1.63 (manifest APIVersion 101050, installed and working in
the operator's own AddOns folder), whose reel alert fires on
`itemSoundCategory == ITEM_SOUND_CATEGORY_LURE` inventory updates and then
confirms `GetGameCameraInteractableActionInfo() ==
GetString(SI_GAMECAMERAACTIONTYPE17)`.

The bait-consumption signal is kept as a belt-and-braces secondary because it
does not depend on the reticle at the instant of the bite. The current addon's
version of it accepts any stack decrease of one anywhere in the bag while
fishing (a potion drink or mount feed would register as a bite); adding the
`ITEM_SOUND_CATEGORY_LURE` scoping fixes that false-positive defect and
matches the contract's "equipped bait" wording.

**Alternatives considered**: Polling the selected lure's stack via
`GetFishingLureInfo(GetFishingLure())` (third return is the stack count,
per `esoui/ingame/fishing/fishing.lua:27`) would also work but adds a second
poll target for no advantage over the event the game already delivers.

## R4: Supporting current-API facts

- `GetGameCameraInteractableActionInfo()`'s fifth return identifies a fishing
  hole under the reticle: `additionalInfo ==
  ADDITIONAL_INTERACT_INFO_FISHING_NODE`
  (`esoui/ingame/fishing/fishing.lua:13-15`, `reticle.lua:159`). Not needed
  for the signal contract, recorded for completeness.
- `GetFishingLure()` returns 0 when no bait is selected
  (`fishing.lua:15-16`); with no bait the game never starts the interaction,
  which is the documented no-cast precondition from slice 024's quickstart.
- `IsGameCameraActive()` and `SCENE_MANAGER:IsShowing("hud"/"hudui")` remain
  current; menu-open suppression on the inventory signal is retained.
- `EVENT_CHATTER_END` remains registered as cheap cleanup on conversation
  end; it is not load-bearing since the poll is authoritative.

## R5: Controller observability gap

**Decision**: Add DEBUG `tracing` lines (target `eso_weave::fishing`) at
every controller transition; no behavior change.

**Rationale**: `src/fishing/mod.rs` contains exactly one tracing statement
(a synthesis-failure warn at line 240). Across four failed field sessions the
log carried zero fishing-engine lines at any level, while the pixel bus
narrated every sample; diagnosing this slice's root cause required
reconstructing controller behavior from cadence side effects. The logging
layer (`src/logging/mod.rs:145-182`) applies only a global level threshold
with no target filtering, so new lines under a new target need no
infrastructure change. The constitution's logging constraint (input contents
never above DEBUG) is respected: transition lines name states and reasons,
never key contents; the interact key name is config data, not captured input,
and appears only at DEBUG.

**Alternatives considered**: INFO-level transitions were rejected to keep
routine fishing quiet at the default level; TRACE was rejected because the
transitions are exactly what the operator needs at the same level as the
existing pixel-bus DEBUG diagnostics.

## R6: Delivery of the corrected addon

**Decision**: Advance `## Version` and `## AddOnVersion` from 3 to 4 in
`PixelBeacon.txt`; remove the stale `ADDON_VERSION = 2` local from the Lua.

**Rationale**: The beacon manager compares the embedded manifest against the
installed one and the slice 022 Update control performs uninstall-then-install
through the existing safety-gated intents; a version advance is what makes
the update offer appear. The Lua-side `ADDON_VERSION` local is read by
nothing (verified by search) and already disagrees with the manifest (2
versus 3); deleting it leaves the manifest as the single version authority.

**Alternatives considered**: Synchronizing the local to 4 was rejected;
an unused constant that must be manually kept in step is exactly how it came
to be wrong.
