# Quickstart: PixelBeacon Addon

This slice delivers Lua addon files, not Rust. Validation is structural plus
manual in-game.

## Structural validation

From the repository root, confirm:

```sh
# The manifest carries the managed marker, a version, and an API version.
grep -F "## X-ESO-Weave-Managed: true" addon/PixelBeacon/PixelBeacon.txt
grep -E "^## Version:" addon/PixelBeacon/PixelBeacon.txt
grep -E "^## APIVersion:" addon/PixelBeacon/PixelBeacon.txt

# The Lua defines the three blocks and the bite-detection events.
grep -E "FF00FF|0080FF|00FF00" addon/PixelBeacon/PixelBeacon.lua        # block colors
grep -F "GetLatency" addon/PixelBeacon/PixelBeacon.lua                   # latency source
grep -F "EVENT_INVENTORY_SINGLE_SLOT_UPDATE" addon/PixelBeacon/PixelBeacon.lua
grep -F "EVENT_CLIENT_INTERACT_RESULT" addon/PixelBeacon/PixelBeacon.lua
grep -F "EVENT_CHATTER_END" addon/PixelBeacon/PixelBeacon.lua
```

Confirm all text files are UTF-8 without BOM, LF, and contain no em or en dashes.

## Rust suite unchanged

This slice touches no Rust; confirm the existing suite is still green:

```sh
cargo test --all --locked
```

## Manual in-game validation (later, on a live client)

1. Install the addon into the AddOns directory (done by the later Beacon Manager)
   and load the game.
2. Confirm the magenta status block appears at the top-left and disappears during
   loading screens (US1).
3. Start fishing and confirm the fishing block shows the waiting color, then the
   bite color on a bite, and is absent when idle (US2, US3).
4. Confirm the latency block updates about once per second and only while the
   status block is shown (US4).
5. Use a consumable outside fishing and confirm no bite is signaled (US3).
