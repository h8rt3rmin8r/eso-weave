# Quickstart: PixelBeacon Skill-Cooldown Blocks

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

## Desk validation

Run the merge gate exactly as CI does, in the foreground, watched to completion.
Never background these: cargo buffers test output until the run ends, so a
backgrounded run cannot be told apart from a hung one.

```bash
cargo fmt --all -- --check
```

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

```bash
cargo test --all --locked
```

### What the new tests prove

| Area | Proves |
| --- | --- |
| Decode | Ready, each quantized duration, saturation at the maximum, and the unavailable sentinel each decode correctly; a wrong marker, a broken checksum, and an out-of-range payload each decode to unavailable. |
| Cross-slot | A colour valid for one slot decodes as unavailable at all five other slots' positions, so an off-by-one geometry error is loud rather than silent. |
| Sweep | No arbitrary colour behind any of the six positions decodes as a cooldown. |
| Observe | A change emits exactly one aggregate event; a steady set emits none; a non-decoding block clears that slot; signal loss clears all six. |
| Geometry | Blocks 10 to 15 resolve to row 0, columns 10 to 15; the captured region is exactly one full row wide and one row tall at the shipping constants. |
| Boundary | The compile-time assertion still holds at sixteen blocks, and the existing test that `COLUMNS + 1` starts a second row still describes what happens next. |
| Interface | The skills grid carries its new column, and the window's intrinsic width accounts for it. |
| Cross-language | The addon and companion agree on the block count, all six marks, the quantization step, the maximum, and the sentinel; the manifest is version 11. |

### Targeted runs

```bash
cargo test --test pixelbus cooldown
```

```bash
cargo test --test beacon
```

```bash
cargo test --test app_ui_sizing
```

## In-game confirmation

Prerequisites: the game installed, ESO Weave built from this branch, and the
addon updated to version 11 through the application's own Update button.

1. **Install the update** and confirm the beacon manager offers version 10 to 11.
   Reload the interface in game.
2. **Set the log level to DEBUG** so cooldown changes appear in the live log.
3. **Use a skill** and confirm its row shows a countdown that decreases and then
   reads Ready, and that the other rows do not change as a result.
4. **Stand still** and confirm no further cooldown entries appear once everything
   is ready.
5. **Zone** while a cooldown is running and confirm the values afterwards agree
   with the game rather than with the state from before the load.
6. **Check the Synergy row** shows the muted placeholder rather than a value,
   which is correct: it has no block behind it.
7. **Look at the overlay** in the top-left of the game client and confirm it is
   now sixteen squares wide and still one square tall.

## Backward-compatibility confirmation

1. Downgrade the installed addon to version 10, or decline the update.
2. Reload the interface and confirm all six cooldown cells show the muted
   unavailable placeholder.
3. Confirm no cooldown entries appear in the log at DEBUG, and that every other
   readout continues to work unchanged.

## Expected outcomes

| Criterion | Proven by |
| --- | --- |
| SC-001 | In-game step 3, ten uses |
| SC-002 | In-game step 4 |
| SC-003 | In-game step 3, across every slot |
| SC-004 | Backward-compatibility steps 2 and 3, plus the desk sweep |
| SC-005 | Backward-compatibility step 3, plus the full suite staying green |
| SC-006 | Desk geometry and cross-language tests |
| SC-007 | Desk cross-slot test |
| SC-008 | The three merge-gate commands above |
