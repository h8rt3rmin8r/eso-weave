# Research: Fishing Bite Signal Correction

## R1: The false bite (root cause, proven by the operator's field log)

**Decision**: The reticle prompt matching
`GetString(SI_GAMECAMERAACTIONTYPE17)` ("Reel In") is not a bite indicator
and must never be used as one.

**Rationale**: The v0.6.1 session of 2026-07-13 20:20 (addon v4 confirmed
installed) is conclusive. The fishing-engine log added in slice 025 shows,
for every cast in the session:

```text
20:20:24.450  cast interact sent; armed with a 5000 ms cast-confirmation window
20:20:25.252  bite detected; reeling in 100 ms      (802 ms after the cast)
20:20:25.360  reel interact sent; recast in 3000 ms
20:20:25.577  cast ended without a resolved bite; recasting
20:20:25.577  cast interact sent; ...
20:20:25.794  bite detected; ...                    (217 ms after the recast)
... (the cycle repeats at ~2 casts per second until 20:20:27.743)
20:20:32.835  fishing disabled: NoCastDetected
```

"Bite detected" arrived on the first 100 ms poll tick after every cast, and
"cast detected; waiting for bite" never appeared (the waiting and bite
colors rendered within the same tick, so the reader's first observed state
was bite). A fish cannot strike 200 ms after every cast; the prompt matched
because it is the standing interact prompt for the entire time the line is
in the water, which is precisely how a player reels in early manually.
Slice 025's decision (2026-07-13, CHANGELOG) inferred bite semantics for
that string; this slice's dated decision corrects it.

The bait cost: bait attaches to the cast, so each app-initiated early reel
wasted one bait, at roughly two per second.

## R2: The correct bite signal (both proven references agree)

**Decision**: The sole bite signal is the bait-consumption inventory event:
`EVENT_INVENTORY_SINGLE_SLOT_UPDATE` with `stackCountChange == -1` and
`itemSoundCategory == ITEM_SOUND_CATEGORY_LURE`, while a cast is active and
no menu is open. This is already implemented, byte-for-byte, in addon v4;
it simply stops being shadowed by the false trigger.

**Rationale**:

- **InfoPanel 1.63** (manifest APIVersion 101050, installed and current on
  the operator's machine): its reel alert triggers on the inventory event
  with the lure sound category (InfoPanel.lua:875) and only then consults
  the prompt as a "currently fishing" confirmation (line 877). The prompt
  is the confirmation, the bait consumption is the bite.
- **fishyboteso/FishingStateMachine** (the detection core of a working
  external fishing automation; source re-verified this session): the reelin
  state is entered on `EVENT_INVENTORY_SINGLE_SLOT_UPDATE` while in the
  fishing state, and the library never compares the action string for the
  bite:

  ```lua
  EVENT_MANAGER:RegisterForEvent(this.name .. "OnSlotUpdate",
  EVENT_INVENTORY_SINGLE_SLOT_UPDATE, function()
      if this.currentState == this.state.fishing then
      _changeState(this.state.reelin) end
  end)
  ```

- The game consumes the bait at the moment the fish takes it, which is what
  makes the inventory decrease the one reliable "fish hooked" observable
  available to addons (the interact function itself is private).

**Alternatives considered**: keeping the prompt sample as a redundant
waiting confirmation (rejected: `GetInteractionType()` already answers
that, and keeping the call invites the same misreading later); a
minimum-wait heuristic before accepting a bite (rejected: neither reference
ships one, the lure scoping is precise, and it could suppress legitimately
fast bites).

## R3: Cast tracking is correct and unchanged

The slice 025 poll (`GetInteractionType() == INTERACTION_FISH` on the
100 ms tick) worked in the field: the log shows the fishing cadence and the
recast loop responding to interaction start and end within a tick. The tick
keeps exactly these duties.

## R4: Residual unknown and its failure mode

The lure event firing on a real strike is the one link no field session has
yet exercised (every session so far died upstream). If it did not fire, the
corrected addon's failure mode is benign and diagnosable: the app stays in
Waiting (FR-004 allows indefinite waiting), the fish escapes, the
interaction ends, the poll returns idle, FishingStopped recasts, and the
debug log shows casts with no bite line. No reel fires, so no bait is spent
by the application. The quickstart's first live catch closes this unknown.

## R5: Delivery

Manifest `## Version`/`## AddOnVersion` advance 4 to 5 (the update control
offers the corrected addon); the version-pin test in `tests/beacon.rs`
advances with it, as in slices 016 and 025. No other Rust change.
