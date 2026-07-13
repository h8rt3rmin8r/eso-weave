# Phase 1 Data Model: Primary and Skills Panel Controls

This slice adds no persisted data. It adds one view-model intent and one piece of
transient GUI state.

## New intent: UiIntent::UpdateBeacon

`src/app/mod.rs`. A user intent with no payload. Handled in `apply_intent` by
running the existing `uninstall_beacon()` then `install_beacon()` in sequence and
returning no notices (the two paths log their own outcomes). Reuses the existing
beacon module functions; the managed-marker uninstall gate is unchanged.

## Transient GUI state: delay edit buffer

`src/app/ui.rs`, on `EsoWeaveApp`.

| Field | Type | Meaning |
|-------|------|---------|
| `delay_edit` | `Option<(u8, String)>` | The skill slot index currently being edited in the Delay column and the digits typed so far. `None` when no delay field is focused. |

Lifecycle per frame, for the focused override-on row:

- If `delay_edit` names this row, the field shows the buffered string; otherwise it
  shows the model's `effective_delay`.
- On change: filter to ASCII digits, keep at most four, store back into
  `delay_edit`, parse to `u32` (empty -> 0), and raise
  `EditSkill(index, override_edit_for(weave_type, Some(value)))`.
- On focus loss: clear `delay_edit`, so the field reflects the model value again.

No change to `SkillRow`, `SkillEdit`, the persisted config, or `SessionState`.

## String changes

`src/app/strings.rs`:

- `SKILL_COLUMNS[4].0`: `"Delay"` -> `"Delay (ms)"` (header rename; no underscore,
  passes the hygiene test).
- New `BEACON_UPDATE_TOOLTIP` constant, added to `all_tooltips()` for the non-empty
  coverage test. The Update button uses it as its hover text.
