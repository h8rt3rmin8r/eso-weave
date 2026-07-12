# Data Model: Hotkey and Weapon-Bar Detection Fixes

**Feature**: 015-hotkey-detection-fixes | **Date**: 2026-07-11

This slice introduces no new persisted data and changes no schema. It adds one
in-memory channel and reuses existing types. The entities below are the state the
fix touches.

## Persisted state (unchanged)

- **SessionState** (`state.json`): `schema_version`, `suspended: bool`,
  `fishing: bool`. Unchanged. Hotkey toggles reuse the existing
  `mark_session` -> `maybe_flush` -> `current_session_state` write path, so no
  field, version, or migration changes.

## Runtime state and flows

- **Suspend state**: `InputEngine.suspended` (`AtomicBool`), shared via
  `Arc<InputEngine>` across the input thread, the weave worker, and the GUI. The
  only writer remains `AppModel::apply_intent(ToggleSuspend)`; the hotkey now
  reaches that writer via the toggle channel rather than mutating state itself.

- **Fishing enabled state**: `FishingController` behind
  `Arc<Mutex<FishingController>>`, shared with the pixel-bus worker and the GUI.
  The only writer for the on/off intent remains
  `apply_intent(SetFishing(bool))`.

- **App-toggle channel** (new, not persisted): a `std::sync::mpsc` channel.
  Sender held by the weave worker; receiver held by `EsoWeaveApp`. Carries the
  two toggle `Action`s forwarded from the drained action stream. Bounded in
  practice by the human press rate; unbounded capacity is acceptable because the
  GUI drains it every frame.

- **Weapon-bar signal** (unchanged type): `WeaponBarSignal { bar: ActiveBar,
  front: WeaponClass, back: WeaponClass }`, produced by `decode_weapon_bar` and
  consumed by `WeaveEngine::set_weapon_bar` for display only in this slice.

## Pure functions under test (new)

- `Action::is_app_toggle(self) -> bool`: true for `ToggleSuspend` and
  `ToggleFishing`, false for every skill/ultimate/synergy action. Partitions the
  action stream at the weave worker.

- `app_toggle_intent(action: Action, fishing_on: bool) -> Option<UiIntent>`:
  maps `ToggleSuspend` -> `Some(UiIntent::ToggleSuspend)`,
  `ToggleFishing` -> `Some(UiIntent::SetFishing(!fishing_on))`, and any
  non-toggle action -> `None`. The single source of truth for turning a hotkey
  toggle into the same intent a button raises.

## Diagnostics (no data shape)

Reader `observe` emits `tracing` events only: DEBUG on weapon-bar detected/lost
transitions (carrying the decoded bar and classes), TRACE on raw per-sample block
values. These are log records, not stored or contract data.
