# Phase 1 Data Model: Weapon-Bar-Aware Adaptive Timing

## Decoded signal types (pixelbus)

### ActiveBar (enum)

- `Unknown`, `Front`, `Back`. Decoded from the B3 blue channel (0, 1, 2). Anything
  else is `Unknown`.

### WeaponClass (enum)

- `Unknown`, `DualWield`, `TwoHanded`, `SwordAndShield`, `Bow`, `DestructionStaff`,
  `RestorationStaff`. Fixed integer codes 0..6, mirrored exactly in the addon Lua and
  the reader. An out-of-range nibble decodes to `Unknown`.

### WeaponBarSignal (struct)

- `bar: ActiveBar`, `front: WeaponClass`, `back: WeaponClass`. Produced by
  `decode_weapon_bar(sample, tolerance) -> Option<WeaponBarSignal>`, which returns
  `Some` only when the green channel matches the `0x5A` marker within tolerance.

### PixelBusEvent (extended)

- Add `WeaponBar(WeaponBarSignal)`. Emitted by `observe` only on a change from the last
  decoded weapon-bar signal (edge-detected in the reader, like fishing), and only while
  a heartbeat is present.

### ReaderConfig (extended)

- Add `weapon_point: (u32, u32)` default `(56, 8)`. Fixed geometry, not a user setting
  (like the other sample points), so the opaque `pixelbus` config is unchanged.

## Persisted timing model (weave config)

### WeaveConfig (extended, additive and serde-defaulted)

- Existing `timing: TimingConfig` becomes the **front** (primary) profile.
- Add `timing_back: TimingConfig` (default equals the front default), the **back**
  profile.
- Add `auto_timing: bool` (default false): when true, each bar's `d_heavy` follows the
  detected weapon-class preset.

Back-compat: `store` writes the two profiles and the flag; `load` reads them with the
front default when absent, so an older config still loads (the back profile defaults to
the front default and auto is off).

## Runtime state (weave engine, not persisted)

- `active_bar: ActiveBar`, `front_class: WeaponClass`, `back_class: WeaponClass`,
  updated by `set_weapon_bar(signal)` when a `WeaponBar` event is routed. Default
  `Unknown`.

## Derivations (pure, tested)

### effective_timing(config, active_bar, class_for_active_bar) -> TimingConfig

- Base profile = `timing_back` when `active_bar == Back`, else `timing` (front). This
  also covers `Unknown` (falls back to the front profile per FR-013).
- If `auto_timing`, override `base.d_heavy` with `heavy_preset(class)` when the class is
  known; otherwise leave the base unchanged.

### heavy_preset(class) -> Option<u32>

- Dual wield 640, two handed 1050, sword and shield 900, bow 1380, destruction staff
  1180, restoration staff 1360; `Unknown` -> `None` (keep the profile value).

## Consumption

- `WeaveEngine::handle` computes `effective_timing(...)` once per action and passes it
  to the existing `sequence_for_adapted`, replacing the direct `&self.config.timing`
  read, so the global-cooldown gate and the sequence both use the active bar's timing.

## GUI (thin, derived)

- A status line shows the active bar and each bar's weapon class (or Unknown), derived
  in the tested view-model (a `WeaponBarView`).
- The settings Combat-timing cluster gains an "auto timing from weapon" toggle and,
  when auto is off, a back-bar timing group mirroring the front timing fields.

## Invariants

- The class codes are identical in the Lua addon and the Rust reader (a comment and a
  test document the shared table).
- Decoding requires the marker; a bad or absent B3 yields no signal and the app uses
  the front profile with unknown classes.
- No pinned block meaning changes; only a new block and a new sample point are added.
