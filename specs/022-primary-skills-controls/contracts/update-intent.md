# Contract: Update Beacon Intent

This slice adds no network or IPC surface. Its unit-tested contract is the model
Update intent; the rest is presentation validated observationally.

## UiIntent::UpdateBeacon (unit-tested)

```rust
// src/app/mod.rs, in AppModel::apply_intent
UiIntent::UpdateBeacon => {
    self.uninstall_beacon();
    self.install_beacon();
    Vec::new()
}
```

Behavioral contract:

1. **Reinstall**: from an installed state (current or outdated managed folder),
   applying `UpdateBeacon` leaves the addon installed and current
   (`BeaconCondition::InstalledCurrent`).
2. **Safety preserved**: `uninstall_beacon` continues to delete only a folder whose
   managed marker verifies; an unmanaged folder is not deleted by Update.
3. **No persistence**: the intent writes no config or session state and returns no
   notices (the beacon paths log their own outcomes).

Required unit test (in `tests/app_view_model.rs`, mirroring the existing
install/uninstall test):

- Install the beacon (`InstalledCurrent`), apply `UpdateBeacon`, and assert the
  condition is still `InstalledCurrent` and `uninstall_enabled` is true.

## Presentation (validated observationally, not unit-tested)

- The Update button is enabled exactly when `view.uninstall_enabled` is true
  (a folder is present) and greyed otherwise.
- The Weave and log-level dropdowns keep a fixed width across selections.
- The Weapon Bar title and state align with the rows above.
- The Delay header reads `Delay (ms)`; both delay states render as a same-width,
  right-aligned, four-digit field (editable when overriding, greyed read-only
  otherwise) showing the actual current value.
