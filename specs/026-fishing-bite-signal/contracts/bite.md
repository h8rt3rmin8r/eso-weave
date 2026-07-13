# Contract: Bite Signal

Delta to the slice 025 detection contract
(`specs/025-fishing-interaction-detection/contracts/detection.md`); the
rendered-signal table, geometry, reader decode, and controller expectations
there remain in force unchanged.

## Corrected obligations

1. **The interact prompt is never a bite signal.** The prompt shown while a
   line is in the water is the standing reel-in prompt for the whole cast.
   The addon MUST NOT sample or compare the interact prompt for any fishing
   decision. (Supersedes obligation 2 of the 025 contract.)
2. **Bite, sole signal.** While waiting or bite, an inventory single-slot
   update with a stack decrease of exactly one carrying the lure item-sound
   category, with no menu open, MUST drive bite. Nothing else may.
3. **Waiting persists indefinitely.** Absent a bite, the waiting state MUST
   hold until the interaction ends or the operator stops; the addon MUST
   NOT synthesize a bite on any timer or prompt condition.
4. **Bite precedence, clearing, and idle return**: unchanged from the 025
   contract (poll never demotes; clears on item gain, the 5000 ms safety
   timeout, or interaction end).
5. **Latency budget for a real bite**: bait-consumption event to rendered
   green within the same event dispatch; rendered green to decoded
   BiteDetected within one reader sample (100 ms at fishing cadence). A
   bite may legitimately be the reader's first observed non-idle state
   (green before blue) when it lands inside one sample interval; the
   controller accepts BiteDetected from Armed for exactly this case.
