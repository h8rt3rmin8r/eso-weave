# Research: PixelBeacon Menu-State Input Gate

**Feature**: 032-menu-state-gate | **Date**: 2026-07-27

Phase 0 output. No unknown in the plan's Technical Context is left unresolved.

## R1: The authoritative "a UI surface is active" signal

**Decision**: `IsGameCameraUIModeActive()`, ORed with
`ZO_GetChatSystem():IsTextEntryOpen()`.

**Rationale**: This reverses the leading suggestion in issue #10, which proposed
the addon's existing `isMenuOpen()` scene test (a menu is open when neither the
`hud` nor the `hudui` scene is showing). That test **misses chat entry**, and chat
is the most common in-game text field and the primary use case of the whole issue.
Opening chat does not hide the gameplay scenes.

Verified against the live `esoui/esoui` source this session. The game's own
`ZO_IngameSceneManager:ConsiderExitingUIMode`, in
`esoui/ingame/scenes/ingamescenemanager.lua`, reads:

```lua
if self.hudUISceneHidesAutomatically and self.numTopLevelShown == 0
   and self.numRemoteTopLevelShown == 0 and showingHUDUI and DoesGameHaveFocus()
   and not ZO_GetChatSystem():IsTextEntryOpen() then
    return self:SetInUIMode(false)
end
```

The game refuses to leave UI mode while chat text entry is open. That is direct
evidence that UI mode is the flag which already means what this feature needs, and
that the scene test does not.

Search results: `IsGameCameraUIModeActive` 4 hits, `IsTextEntryOpen` 3 hits
(defined as `SharedChatSystem:IsTextEntryOpen` returning `self.textEntry:IsOpen()`,
in `esoui/ingame/chatsystem/sharedchatsystem.lua`). Note for anyone re-checking
issue #10: `GetGameCameraUIMode` does **not** exist; the call is the `Is...Active`
predicate.

The explicit OR with chat entry is redundant given the scene-manager logic above,
and is kept anyway. It costs one call, it makes the primary use case a stated
guarantee rather than an inference from someone else's control flow, and it holds
even if UI-mode semantics shift in a future patch.

**Alternatives considered**:

- **The scene test alone.** Rejected on the chat-entry evidence above. It would
  have shipped a feature that fails its own headline use case.
- **Enumerating scenes and treating any known menu scene as active.** Rejected: it
  is only correct for scenes someone remembered, and it still misses chat entry.

## R2: Covering the fishing synthesis path

**Decision**: gate the fishing controller separately from the interception
decision, at the points where it initiates an interact keypress.

**Rationale**: The fishing controller does not synthesize in response to an
intercepted key. It reacts to beacon events and its own timers and presses the
interact key through `FishingSink`, entirely bypassing the interception decision. A
gate placed only on interception would therefore leave it free to send a cast or
reel keypress into a chat message the operator is composing.

This is not a hypothetical. Waiting for a bite is precisely the low-attention
moment when someone opens chat, so the untreated path is arguably a more likely
source of the harm than the weave path the issue actually describes.

**Alternatives considered**:

- **A single guard at the synthesis sink.** One place, impossible to forget, and
  wrong: it can block an individual key transition mid-sequence and drop a key-up,
  leaving a key held down in the game. FR-011 exists to prevent exactly that.
- **Disabling fishing while gated**, following the existing degrade-to-disabled on
  signal loss. Rejected: that precedent covers firing inputs with no signal at all.
  Here the signal is healthy and the session is fine; ending it because the
  operator opened the map would be worse than pausing.

## R3: Keeping both cadence settings meaningful

**Decision**: fast cadence when a fishing session is active **or** the application
can currently intercept; idle cadence otherwise.

**Rationale**: The gate needs prompt sampling, but making the fast cadence
unconditional would leave `interval_idle_ms` with no effect at all. That is the
mirror image of the defect slice 016 fixed, where `interval_fishing_ms` was the
dead setting and every fishing session sampled once a second. Reintroducing the
same class of bug in the other direction, in the same function, would be a poor
outcome for a slice that is otherwise about rigor.

The resolution is that the gate only matters when there is something to gate. A
suspended application intercepts nothing and synthesizes nothing, so it does not
need to know about menus, and can sample slowly. Both settings keep a real meaning,
and the extra capture cost is paid only while the application is actually working.

**Alternatives considered**:

- **Always fast.** Simplest; creates the dead setting described above and raises
  capture cost even while suspended.
- **Remove the idle setting and migrate the config.** Honest, but it discards a
  user-tunable value to solve a problem that the condition above solves without a
  schema change.

## R4: Proving the one-way safety property

**Decision**: exhaustive cross-product comparison of the gated and ungated
interception decisions.

**Rationale**: FR-015 is a universally quantified claim ("for every combination of
inputs"), and the risk it guards against is precisely the combination nobody
thought to try. A sampled test cannot discharge it; an exhaustive one can, because
the input space is small and closed. The two assertions are that an ungated pass
implies a gated pass, and that the gate never produces a suppression where the
ungated decision passed.

This is also the shape that survives future edits: anyone adding another condition
to the decision inherits the proof for free, and breaks the test loudly if their
condition can tighten.

**Alternatives considered**: hand-written scenario tests for the cases that seemed
important. Rejected as the weaker form of exactly this test, and as the reason a
prior slice's sizing defects shipped green (a test that asserts the interesting
case rather than the whole space).

## R5: Making the default the safe value

**Decision**: both gates are plain flags defaulting to inactive, set through
methods rather than constructor parameters.

**Rationale**: This is a small choice with a large consequence. Because the default
is inactive and inactive means "behave as today", every existing test constructs
its subject in today's behavior without being touched. Constitution principle II
forbids weakening the safety tests, and FR-018 permits mechanical updates; this
design means not even a mechanical update is needed for them, which is a stronger
position than the requirement demands.

It also makes the failure modes right by construction: an older addon, an
undecodable sample, and a lost signal all leave the flag at its default.

## R6: The surface code table and its robustness

**Decision**: codes spaced 24 apart in the red channel, with a generic code for
anything unenumerated, and the boolean decided before the label.

**Rationale**: The gate must not depend on the table being right. Because the addon
decides "a surface is active" from UI mode first and only then tries to name it, a
scene name that is wrong, renamed by a patch, or simply never enumerated degrades
to the generic code. The consequence of a table error is therefore a less specific
readout, never a gate that fails to engage.

Scene names for the enumerated surfaces were reported as verified in issue #10
itself (`gameMenuInGame`, `worldMap`, `inventory`, and the mail scenes). They are
not re-verified exhaustively here, and deliberately so: the fallback makes the cost
of an error cosmetic, and API search quota is better spent on signals whose
correctness the feature actually depends on, which is what R1 did.

## Open items carried forward

None blocking. One note for slice 033: this feature takes marker `0xD2`, the value
slice 031 reserved. The next block needs a fresh marker chosen against
`BLOCK_CENTER_GREENS`, whose separation test will reject a colliding pick.
