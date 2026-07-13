# Contract: Addon Fishing Detection

The behavioral contract between the game, the PixelBeacon addon, and the
application's pixel-bus reader. The reader side is unchanged from the slice
004 contract (`specs/004-pixelbeacon-addon/contracts/pixel-bus.md`) and the
master specification section 10.3; this contract governs how the addon
decides what to render on block B1.

## Rendered signal (frozen)

| Signal | B1 color | Meaning |
| --- | --- | --- |
| idle | hidden | no active fishing interaction |
| waiting | #0080FF | a cast is active, awaiting a bite |
| bite | #00FF00 | a fish is hooked, reel now |

Block geometry, positions, and the other blocks (B0 status, B2 latency, B3
weapon bar) are byte-for-byte unchanged.

## Detection obligations

1. **Waiting is a polled fact, not an event memory.** The addon MUST sample
   the game's interaction type on a periodic tick of 100 ms (bound: no
   coarser than 150 ms) and MUST render waiting while and only while the
   sampled interaction type is the fishing interaction, subject to the bite
   precedence rule. The one-shot interact-result event MUST NOT be consulted
   for any fishing decision.
2. **Bite, primary.** While waiting, if the sampled reticle action equals the
   game's localized reel-in string, the addon MUST render bite within the
   same tick. The comparison MUST use the game's own string table on both
   sides (locale independence).
3. **Bite, secondary.** While waiting or bite, an inventory single-slot
   update with a stack decrease of exactly one carrying the lure item-sound
   category, with no menu open, MUST also drive bite. Stack decreases of any
   other category MUST be ignored (false-bite guard).
4. **Bite precedence.** Once bite is rendered, the periodic tick MUST NOT
   demote it to waiting. Bite ends only by: an item gain (catch resolved,
   back to waiting), the 5000 ms safety timeout (back to waiting), or the
   interaction ending (to idle).
5. **Idle return.** When the sampled interaction type is no longer the
   fishing interaction, the addon MUST render idle within one tick and clear
   any pending bite timer.
6. **Latency budget.** From a state change in the game to the rendered block
   color changing: at most one tick (100 ms). From render to the
   application's decoded event: at most one reader sample (100 ms at fishing
   cadence). Total worst case 200 ms, comfortably inside the persisted
   5000 ms arm window.

## Reader expectations (unchanged, for reference)

- Decoder: `fishing_signal()` maps #0080FF to Waiting and #00FF00 to Bite
  with per-channel tolerance 2; anything else, including a hidden block, is
  None.
- Reader events on change while the heartbeat is present: Waiting emits
  FishingStarted, Bite emits BiteDetected, None emits FishingStopped.
- Controller: FishingStarted moves Armed/Recast to Waiting; BiteDetected
  moves Waiting to Reeling; sustained signal absence disables fishing
  (safety invariant, unchanged).

## Controller logging obligations

Every controller state transition and every disable (with its stop reason)
MUST emit exactly one DEBUG line under the `eso_weave::fishing` target, with
no change to emitted events, timers, or key synthesis. Existing tests pass
unchanged.
