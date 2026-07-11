# Phase 1 Data Model: GUI Ergonomics, Information Design, and Auto-Save

This slice adds little persistent data; most entities are derived view-model values
rendered each frame. Persisted additions are one config field and one new state
file.

## Persisted entities

### Config UI section (existing, extended)

The `ui` section of `config.json` (currently `theme`, `always_on_top`) gains:

- `log_panel_height: f32` (logical points). Optional; defaults to a sensible initial
  height when absent. Re-clamped on load to `[min, max]` for the current window size
  so a smaller window never restores an oversized log. Additive and back-compatible
  via serde defaults.

No other config keys change. Underscore removal is display-only and does not touch
config keys.

### Session state file (new): `state.json`

A new, separate file in the platform config directory, holding session/runtime state
that the constitution forbids in `config.json`.

- `schema_version: u32` (starts at 1; migrates forward like config).
- `suspended: bool`. The live suspend intent at last change. Default `false`.
- `fishing: bool`. The live fishing on/off intent at last change. Default `false`.

Rules:

- Load at startup; on missing file, parse error, or unknown/invalid values, fall
  back to defaults (not suspended, not fishing) and surface a notice, mirroring the
  config loader's tolerant behavior.
- `fishing` is a single on/off intent. It never encodes transient sub-states
  (waiting, reeling, recast). On restore, `true` re-arms the fishing controller from
  a clean baseline; `false` restores idle.
- Written through the coalesced-save trigger, same as config.
- Restoring `suspended = false` or `fishing = true` performs no input while the game
  window is unfocused (focus-scoped invariant).

## Derived (non-persisted) view-model entities

### StatusLine

One per top-region row, derived each frame in the view-model:

- `title: &str` (one of "Status", "Fishing", "Pixel Beacon (Addon)").
- `state_text: String` (normalized state label).
- `state_role: StatusRole` (which palette role colors the state field).
- `tooltip: &str`.

`StatusRole` is an enum mapping to brand palette roles:

- `Healthy` (running, beacon current) -> ok
- `Warning` (suspended, beacon outdated) -> warn
- `Active` (fishing in progress) -> teal accent
- `Muted` (idle, absent) -> muted
- `Error` (beacon not installed, signal lost) -> err

Derivations are pure functions of the existing subsystem state (suspend flag,
`FishingState`, `BeaconCondition`) and are unit-tested for the correct text and
role.

### SkillRowView (extended)

The existing per-row view gains a computed effective-delay for the Delay column:

- `effective_delay: u32` and `is_override: bool`. When the override for the row's
  weave type is set, `effective_delay` is that value and `is_override` is true; when
  not, `effective_delay` is the global default for the row's weave type and
  `is_override` is false (rendered muted and read-only).
- The single override edited by the row targets the delay matching the row's weave
  type (`d_weave` for light, `d_heavy` for heavy, `d_bash` for bash).

### ToggleModel

Not a stored entity; each toggle binds a bool in the view-model or settings form and
raises the existing intent on change. Enumerated toggles: suspend, fishing,
per-skill enabled, per-skill override, and each boolean setting.

### SaveScheduler

View-model state driving coalesced saves:

- `dirty: bool`, `last_change: Instant`, `settle: Duration` (about 400 ms).
- Pure predicate `should_flush(now) -> bool` returns true when `dirty` and
  `now - last_change >= settle`; a flush writes config and/or state and shows one
  toast. Unit-tested over synthetic instants.

### Toast

Transient UI notification: `message: String`, `shown_at: Instant`, `ttl: Duration`.
Coalesced so a burst of saves yields a single toast. Non-persisted.

## Relationships and invariants

- `config.json` holds user settings only; `state.json` holds session state only.
  Neither ever holds the other's data.
- The persisted `logging.level` gates capture; the per-panel log filter is a
  transient view control and is not persisted.
- Log-panel height is clamped on load; session state falls back to safe defaults on
  any problem.
- All derivations that carry correctness (status text and role, effective delay,
  save scheduling, session load and store, height clamping, string coverage) live in
  the tested view-model or subsystem modules, not in the egui layer.
