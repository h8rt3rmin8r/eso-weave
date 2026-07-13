# Quickstart: Fishing Capture Hardening (In-Game Validation Protocol)

The capture fix can only be confirmed against the running game on a real
accelerated surface. This protocol is the primary verification; the automated tests
cover the pure decoder/reader logic and the new pixel-extraction helper.

## Automated checks

```
cargo test --all --locked
```

Confirm:

- All existing pixel-bus decoder and reader tests pass unchanged.
- The new `strip_pixel` tests pass (channel order, out-of-range, truncated buffer).

## Preconditions to rule out first

These four produce the identical "Casting then Idle (no cast detected)" symptom and
must be satisfied before judging the capture:

1. Bait is selected. Without bait, ESO never starts a fishing interaction, so the
   addon never shows the waiting signal.
2. The PixelBeacon addon is the current version and loaded (not flagged Out of
   Date). Reinstall or Update it from the app and run /reloadui if unsure.
3. The beacon strip (top-left of the game window) is visible and unobstructed.
4. The ESO window is focused.

## Validation run

1. Set the log level to the most verbose (TRACE) from the live-log dropdown or
   settings.
2. Start the game to a fishing spot with bait equipped, addon loaded, strip visible.
3. Watch the live log:
   - Confirm a "pixel bus heartbeat acquired" message appears and the per-sample
     trace shows the status block (B0) as magenta-ish bytes, not near-black. This
     confirms the capture now reads the accelerated surface (SC-002).
4. Cast at a fishing hole (or press the fishing hotkey):
   - Confirm the fishing block (B1) traces as blue and the decoded fishing signal
     becomes Waiting, and the status advances from Casting to Fishing. (SC-001.)
   - On a bite, confirm the status advances to Reeling and the line reels in.
5. Repeat across roughly ten casts and confirm it does not revert to Idle (no cast
   detected).

## If it still fails, read the evidence

- No "heartbeat acquired" and B0 near-black with all preconditions met: still a
  capture problem (for example the strip is being covered, or an unusual
  monitor/compositor configuration). Note the configuration.
- "heartbeat acquired" present but the fishing signal never becomes Waiting on a
  real cast: the capture is fixed and the remaining problem is the addon
  interaction detection (`EVENT_CLIENT_INTERACT_RESULT`/`INTERACTION_FISH`). This is
  the evidence that scopes the follow-up slice, rather than another guess.

## Regression

6. Cover the strip or unfocus the window and confirm fishing degrades to disabled
   (signal lost) rather than firing input, and the log shows the missing heartbeat.
