# Quickstart Validation: Weapon-Bar-Aware Adaptive Timing

## Automated gate (foreground, watched to completion)

```
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

Relevant tests:

- `pixelbus`: `decode_weapon_bar` marker gating and code decoding across all classes
  and bars; tolerance boundary; `observe` emits a `WeaponBar` event only on change and
  only with a heartbeat; the existing status/fishing/latency decode is unaffected.
- `weave`: `effective_timing` selects the front or back profile by active bar and holds
  the front profile on `Unknown`; `heavy_preset` orders (dual wield < two handed <
  staves and bow); auto on applies the preset per bar; auto off uses the manual profile;
  the back profile and auto flag round-trip through settings with a back-compat default.
- `routing`: a `WeaponBar` event updates the engine's active bar and classes.
- GUI view-model: the weapon-bar status derivation shows bar and classes or Unknown.

## Manual validation (app, no game)

```
cargo run
```

- With the log open, confirm the app runs; the weapon-bar status line shows an unknown
  state when no signal is present (degrades to the front profile).
- In Settings, Combat timing, confirm the "auto timing from weapon" toggle and, with
  auto off, a back-bar timing group; changes auto-save.

## In-game validation (owed, cannot be done offline)

Install the updated PixelBeacon in ESO and verify:

- The B3 block renders at x=48 and encodes the active bar and both weapon classes;
  swapping bars flips the reported bar, and changing equipped weapons updates the
  classes, with no churn on every attack.
- The reported bar re-baselines correctly after a loading screen and on death and
  revive, and holds the last good value while a swap is locked.
- The heavy-attack presets feel right per weapon; measure and correct the
  one-hand-and-shield value and any preset that is off against live timing.
- The existing status, fishing, and latency signals still decode, and PixelBeacon
  uninstall still verifies the managed marker before deleting.

## Text hygiene

- All new and changed files (including `PixelBeacon.lua`) are UTF-8 without BOM, LF, and
  contain no em-dashes or en-dashes.
