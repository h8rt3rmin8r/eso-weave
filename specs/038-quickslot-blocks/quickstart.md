# Quickstart: PixelBeacon Quickslot-State Blocks

**Feature**: [spec.md](spec.md) | **Date**: 2026-07-27

How to see this feature working, and how to tell a real reading from a plausible
one.

## Confirming it without the game

The whole decode path is reachable from tests with no game, no window, and no
display hardware.

```bash
cargo test --all --locked
```

The tests that matter for this feature:

- `decode_quickslot_*` in `tests/pixelbus.rs`, for the four cases and the
  cross-position rejection.
- `the_capture_region_is_two_rows_now_that_the_count_has_crossed`, for the
  geometry.
- `addon_and_companion_agree_on_the_pixel_bus_contract` in `tests/beacon.rs`,
  which parses the embedded addon source and fails the build if the two sides
  disagree on any constant.

## Confirming it with the game

1. Start the application. Open Settings and update PixelBeacon; the manifest
   advanced to version 12, so the update is offered.
2. In game, run `/reloadui`.
3. Put a potion in a quickslot and make that quickslot active.
4. In the application's Status region, `Quickslot` shows `Ready` and
   `Quickslot item` shows a number.
5. Drink the potion. `Quickslot` shows a decreasing time, then `Ready` again.
6. Switch to a quickslot holding food, or an empty one. Both readouts go muted.

The log at DEBUG carries one entry per changed sample, so raising the log level
and watching the entries is the fastest way to confirm the signal is live.

## What the overlay looks like now

The beacon overlay in the top-left corner of the game client is **two rows tall
for the first time**. At the default square size it is 256 by 32 physical pixels,
where it used to be 256 by 16.

The application tells you its exact footprint in two places:

- Beside **Block size (px)** in Settings, as a caption that follows the value you
  are editing.
- In the log at DEBUG when the sampling thread starts.

To make it smaller, lower **Block size (px)**. At the smallest supported size the
overlay is 32 by 4 physical pixels. Changing it re-deploys the addon so both
sides stay in agreement; it takes effect after a `/reloadui` and an application
restart.

The overlay is not movable. That is deliberate and is explained in
[research.md](research.md) R8.

## Telling a real reading from a plausible one

The failure this feature is most exposed to is an old addon that draws none of
the four blocks, because for the first time the companion samples a strip of
screen one full block row below anything the beacon has ever drawn on.

- Every block carries a validity mark and a complement checksum, and all four
  marks are in the shared registry that the separation check proves.
- An unreadable block reports unknown. It never reports a duration, and it never
  contributes a byte to a partial identity.
- If the cooldown reads but the identity does not, you see a cooldown and a muted
  identity, not a blank readout. A blank readout means the addon is not drawing;
  a half-blank one means a single block is disturbed.

## If nothing decodes

Check the log for `beacon grid does not fit the game client area`. That warning
can now be triggered by height as well as width, which is new: a client area
shorter than two block rows will clip B16 to B19, and a clipped block is captured
as black, fails its marker check, and looks exactly like an addon that was never
installed.
