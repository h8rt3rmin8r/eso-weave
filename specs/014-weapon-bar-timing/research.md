# Phase 0 Research: Weapon-Bar-Aware Adaptive Timing (closes R1)

Sources are community-standard combat references and the ESO add-on API. The exact
timing values are estimates to re-validate in-game; the API signatures and the design
are firm.

## R1: Global cooldown and weave-delay defaults

- **Decision**: ESO's global cooldown is 1000 ms; light and heavy attacks are on a
  parallel track (effectively off-GCD). The practical light-attack-plus-skill weave
  target is about 965 ms (roughly 62 BPM); exceeding 1000 ms drops light attacks, so
  the true lower bound on `d_weave` is dominated by server latency, not local timing.
  Keep `d_weave` small (default 50 ms) and let the latency-adaptive path (S008) do the
  shortening; presets do not change `d_weave`.
- **Rationale**: matches the master spec section 7.3 note that the GCD is 1000 ms and
  the weave lower bound is latency-dominated. Confirms `d_weave` is not the per-weapon
  knob; `d_heavy` is.
- **Sources**: community animation-canceling guides and the ESO forums metronome
  discussion (965 ms / 62 BPM), Combat Metrics addon methodology (light-attacks per
  second and per-cast weave gap).

## R1: Per-weapon heavy-attack channel defaults

- **Decision**: ship these per-class `d_heavy` presets (approximate pure channel;
  re-validate in-game): dual wield 640, two handed 1050, destruction staff 1180,
  restoration staff 1360, bow 1380 (lightning staff also channels longer, folded into
  the destruction-staff class for now), sword and shield 900 (flagged estimate, not in
  sources). Unknown class uses the profile's existing `d_heavy`.
- **Rationale**: dual wield is by far the fastest heavy attack and two handed the next,
  matching the operator's expectation; staves and bow are slowest. These become the
  auto-timing presets. Values are adjustable config, so estimates are safe to ship.
- **Sources**: Combat Metrics community measurements (dpencil1 thread), UESP Heavy
  Attack page. One-hand-and-shield is unquantified in sources: flagged as a TODO to
  measure in-game.
- **Deliverable**: an appendix to `docs/ESO-Weave-Specification-v0.1.0.md` recording
  these defaults and their sources, and section 16 updated to mark R1 closed.

## Detection API (add-on side)

- **Decision**: read the active pair with `GetActiveWeaponPairInfo() -> activeWeaponPair,
  locked` (constants `ACTIVE_WEAPON_PAIR_NONE/MAIN/BACKUP`); react to
  `EVENT_ACTIVE_WEAPON_PAIR_CHANGED`. Read weapon types with `GetItemWeaponType(BAG_WORN,
  slot)` for `EQUIP_SLOT_MAIN_HAND/OFF_HAND/BACKUP_MAIN/BACKUP_OFF`, mapping the
  `WEAPONTYPE_*` constants to a normalized class in Lua (the off/backup-off slot
  distinguishes dual wield vs two handed vs sword and shield).
- **Rationale**: these are the standard in-combat bar-swap and equipment queries; doing
  the class mapping in Lua from named constants means the pixel signal carries a stable
  small code and the reader never needs the raw enum integers (which are not fetched
  offline).
- **Pitfalls handled**: the pair-changed event fires on nearly every attack, so
  edge-detect against `GetActiveWeaponPairInfo()` and re-render only on a real change;
  treat `locked` or `NONE` as indeterminate (hold last good); re-baseline on
  `EVENT_PLAYER_ACTIVATED` (after loading screens) and on death and revive; re-read
  weapon classes on equip/inventory-update events.
- **Sources**: esoui.com wiki (Events, GetItemWeaponType), ESOUI forum threads on
  GetActiveWeaponPairInfo and backup equip slots, and the per-attack-event caveat.

## Encoding (relay)

- **Decision**: one new 16x16 block B3 at x=48, sampled at (56, 8): green = `0x5A`
  marker, red = `(front_class << 4) | back_class`, blue = active bar (0/1/2). Classes
  0..6 fit a nibble.
- **Rationale**: a single RGB carries the bar and both classes, adding one sample point
  and keeping the strip compact; the `0x5A` marker is far from the latency marker
  `0xA5` so tolerance can never confuse them; no checksum is needed because the values
  are small discrete codes gated by the marker.
- **Alternatives considered**: three separate blocks (rejected: wider strip, three more
  samples); packing into spare bits of an existing block (rejected: changes the meaning
  of a pinned block).

## Weapon-class to timing mapping

- **Decision**: a `preset` function maps a weapon class to a `d_heavy` value on top of
  the bar's base profile; `d_weave`, `d_bash`, and `global_cooldown` come from the
  profile. Auto timing, when on, applies the preset per bar from the detected class;
  when off, the manual per-bar profile is used unchanged.
- **Rationale**: only `d_heavy` is weapon-dependent (per R1); keeping the rest from the
  profile preserves the user's other tuning and the latency-adaptive path.
