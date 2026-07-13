# Quickstart: Validating the Fishing Bite Signal Correction

## Prerequisites

- ESO Weave built from this feature (addon manifest version 5 embedded).
- The ESO live client on the same machine, at a fishing hole, with a
  reasonable bait stack (10+ recommended; the session should consume one
  bait per catch and none otherwise).

## Automated verification (no game required)

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Expected: green, with `tests/fishing.rs` and `tests/pixelbus.rs`
unmodified; only the `tests/beacon.rs` version pin advanced.

## Addon update

1. Launch ESO Weave; the addon panel reports installed 4, embedded 5.
2. Activate Update (managed-marker uninstall, then install of version 5).
3. In game: `/reloadui`.

## In-game validation protocol

Log level debug or lower. Note your bait count before starting.

1. **No early reel (SC-001, the defect's regression test)**: F2 at the
   hole. Expected: Casting advances to Fishing, and then NOTHING happens -
   no reel keypress, no recast - for as long as the fish takes. The log
   shows "cast interact sent", then "cast detected; waiting for bite", then
   silence until the strike. Ten consecutive casts must all survive to a
   real bite (or to you stopping); any reel within the first seconds of a
   cast without a strike is a failure.
2. **Real bite reels (SC-002, closes the one unverified link)**: when the
   fish strikes, expect "bite detected; reeling in 100 ms", the reel, the
   catch, and a recast after the recast delay. Run at least 3 consecutive
   full cycles; bait consumed must equal catches exactly.
3. **False-bite guard (SC-003)**: during a waiting phase, drink a potion or
   eat food. Expected: no bite line in the log, no reel.
4. **Escape path (residual-risk probe)**: optionally let one bite go
   unreeled by toggling fishing off at the strike; separately, if a cast
   ever waits until the interaction ends on its own, the log must show
   "cast ended without a resolved bite; recasting" with no reel line - the
   benign failure mode.
5. **Stop**: F2 off mid-wait. Expected: immediate "fishing disabled:
   UserStop", nothing further.

## Reading the result

- Casts survive to real bites, one bait per catch: the fishing feature is
  validated end to end for the first time.
- Casts wait forever and bites never register (no "bite detected" on a real
  strike): the lure-event assumption failed; the log evidence (casts with
  no bite lines) scopes the follow-up - but no bait is wasted by the app.
- Any reel without a strike: capture the log; the bite line's timestamp
  relative to the cast identifies the trigger.
