# Quickstart: PixelBeacon Movement-State Block

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

Two validation passes. The desk pass proves the contract and the failure modes
without the game; the in-game pass proves the signal is real. Both are required
before the feature is considered done, but only the desk pass gates the commit.

## Desk validation (no game required)

Run the merge gate exactly as CI does, in the foreground, watched to completion.
Never background these; cargo buffers test output until the run ends, so a
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

Expected: all three green, with no test weakened, skipped, or made conditional.

### What the new tests prove

| Area | Proves |
| --- | --- |
| `tests/pixelbus.rs` decode | Each live code decodes to its state; the reserved sprint codes decode to unavailable; a wrong marker, a broken checksum, and an unrecognized code each decode to unavailable. |
| `tests/pixelbus.rs` observe | A change emits exactly one event; a steady state emits none; a non-decoding sample clears to unavailable; signal loss clears to unavailable. |
| `tests/pixelbus.rs` geometry | Block 9's center resolves to row 0, column 9; the capture extent is one row whenever `NUM_BLOCKS <= COLUMNS`, asserted on that dependency rather than on the number ten. |
| `tests/pixelbus.rs` registry | Every pair in `BLOCK_CENTER_GREENS`, now including `0x43`, is separated by more than the default tolerance. |
| `tests/beacon.rs` | The addon source and the companion agree on `NUM_BLOCKS`, `COLUMNS`, and every movement constant; the embedded manifest is version 10. |

### Targeted runs

```bash
cargo test --test pixelbus movement
```

```bash
cargo test --test beacon
```

## In-game validation

Prerequisites: the game installed, ESO Weave built from this branch, and the
addon updated to version 10 through the application's own Update button.

1. **Install the update.** Launch the application, open the beacon manager, and
   confirm it offers the update from version 9 to 10. Apply it and reload the
   interface in game (`/reloadui`).
2. **Set the log level to DEBUG** so movement changes appear in the live log.
3. **Mount and dismount.** Confirm the readout follows on both transitions and
   that each transition produces exactly one log line. Repeat at least ten times
   for SC-001.
4. **Hold still.** With movement state unchanged, confirm the log produces no
   further movement lines over at least a minute (SC-002).
5. **Zone while mounted.** Take a loading screen mounted, and confirm the readout
   agrees with the game afterwards rather than showing the pre-load state
   (SC-003). Repeat dismounted.
6. **Check independence.** While mounted, enter combat, open the map, and open
   chat entry in turn. Confirm the movement readout is unaffected by each.
7. **Confirm the interface wording.** The readout shows "Mounted" or "On foot" in
   the active role, beside the combat and weapon-bar fields.

## Backward-compatibility validation

This is the failure mode most likely to reach the field, so it is validated
explicitly rather than assumed.

1. Downgrade the installed addon to version 9 (or decline the update).
2. Reload the interface and confirm the application shows movement as "Not
   detected" in the muted role.
3. Confirm no movement events appear in the log at DEBUG, and that combat,
   weapon bar, latency, fishing, and the three resource readouts all continue to
   work unchanged (SC-004, SC-005).

## Expected outcomes

| Criterion | How this guide proves it |
| --- | --- |
| SC-001 | In-game step 3, ten transitions |
| SC-002 | In-game step 4 |
| SC-003 | In-game step 5 |
| SC-004 | Backward-compatibility steps 2 and 3, plus the desk decode tests |
| SC-005 | Backward-compatibility step 3, plus the full existing suite staying green |
| SC-006 | Desk `tests/beacon.rs` and the geometry tests |
| SC-007 | Desk decode tests for the reserved codes |
| SC-008 | The three merge-gate commands above |
