# Quickstart: Hotkey and Weapon-Bar Detection Fixes

**Feature**: 015-hotkey-detection-fixes | **Date**: 2026-07-11

How to exercise and validate this slice.

## Automated checks (headless, CI parity)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

New unit tests cover:

- `Action::is_app_toggle`: true only for `ToggleSuspend`/`ToggleFishing`.
- `app_toggle_intent`: `ToggleSuspend` -> `ToggleSuspend`; `ToggleFishing` ->
  `SetFishing(!fishing_on)` for both current states; non-toggle actions -> `None`.
- Existing decode/observe tests still pass unchanged (no decode behavior change).

## In-game validation (operator, the parts no harness can cover)

Focus the game window first; input is scoped to the focused game window.

1. **F1 suspend parity**: press F1. The app Status flips between running and
   suspended, matching the Status button. Press again to flip back. Hold F1 and
   confirm it toggles once per press, not repeatedly.
2. **F2 fishing parity**: press F2. The Fishing line flips between enabled and
   disabled, matching the Fishing button. Press again to flip back.
3. **Unfocused guard**: with the game window not focused, press F1/F2 and confirm
   nothing changes.
4. **Rebound key**: rebind suspend to another key; confirm the new key toggles
   and F1 no longer does.

## Diagnosing weapon-bar detection in-game

If the weapon-bar line still reads "Not detected":

1. Set the log level to DEBUG (or TRACE for raw samples) in Settings.
2. Watch the log while swapping weapon bars in-game:
   - A DEBUG "weapon bar detected" line with a bar and classes means the signal
     decodes and the readout should show it.
   - TRACE raw-sample lines showing a present heartbeat but a B3 that does not
     carry the `0x5A` marker means the addon is not rendering B3. Reinstall the
     PixelBeacon addon from the app (Beacon Manager) so the slice-014 B3 block is
     present, then retry.
   - No heartbeat lines at all mean the whole pixel strip is not being read
     (addon disabled, or the beacon strip is off-screen or occluded).
3. Restore the log level when done.

Detection is display and diagnostics only in this slice; it changes no weave
timing and synthesizes no input.

## Release

This slice culminates in a v0.4.2 patch release. Cutting the tag and running the
release follow `docs/releasing.md` and require separate explicit authorization.
