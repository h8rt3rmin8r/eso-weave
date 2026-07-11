# Quickstart: Beacon Manager

This validates the beacon lifecycle end to end without a real ESO install, using
the injected AddOns root seam.

## Prerequisites

- The repository builds (feature 001 foundations and the embedded addon files
  under `addon/PixelBeacon/` are present).
- `cargo` is available; run all commands from the repo root.

## Build and verify (CI parity)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

The `beacon` tests in `tests/beacon.rs` cover the scenarios below; a green run is
the primary validation.

## Scenario 1: Install then verify (US1, US2)

- Create a temporary directory to act as the AddOns root.
- Call `beacon::install(&addons_root, RunningState::NotRunning)`.
- Expected: `Ok(LifecycleOutcome { status: ManagedUpToDate, reload_required: false })`,
  and `addons_root/PixelBeacon/` now contains `PixelBeacon.txt` (with the marker
  line and the embedded `## Version:`) and `PixelBeacon.lua` byte-equal to the
  embedded copies.
- Call `beacon::status(&addons_root)` -> `ManagedUpToDate`.

## Scenario 2: Safe over-install / update (US1)

- Starting from an installed folder, hand-edit the on-disk `PixelBeacon.txt` to an
  older `## Version:` value.
- `status` -> `ManagedVersionMismatch`.
- `install(..)` again -> `ManagedUpToDate`, files restored to the embedded copies.

## Scenario 3: Status classification (US2)

- Empty AddOns root -> `status` = `NotInstalled`.
- Folder with a manifest lacking the marker line -> `Unmanaged`.
- Folder with the marker line but a differing version -> `ManagedVersionMismatch`.

## Scenario 4: Marker-gated uninstall (US3, safety-critical)

- Managed folder (marker present): `uninstall(&addons_root, RunningState::NotRunning)`
  -> `Ok(status: NotInstalled)`, `addons_root/PixelBeacon` removed.
- Unmanaged folder (no marker line): `uninstall(..)` -> `Err(Unmanaged)`, folder and
  files untouched.
- Folder with no manifest: `uninstall(..)` -> `Err(Unmanaged)`, nothing deleted.
- Confinement: place a sibling sentinel `addons_root/OtherAddon/keep.txt`; after any
  install and any uninstall it still exists.

## Scenario 5: Reload reminder (US4)

- `install`/`uninstall` with `RunningState::Running` -> `reload_required == true`.
- With `RunningState::Unknown` -> `reload_required == true` (fail safe).
- With `RunningState::NotRunning` -> `reload_required == false`.

## Scenario 6: Steam VDF library extraction (Linux discovery, host-testable)

- `beacon::steam::library_paths_for_app(vdf, "306130")` on crafted VDF text:
  - single library containing the app -> that library path.
  - multiple libraries, only one containing the app -> just that path.
  - no library containing the app -> empty.

## On real hardware (manual, out of automated scope)

- On Windows, `resolve_addons_dir(&BeaconPrefs::default())` resolves to the
  Documents-based `Elder Scrolls Online/live/AddOns`; relocating Documents still
  resolves correctly.
- On Linux (Proton), it resolves under the correct Steam library's compatdata.
- `probe_game_running()` returns `Running` while ESO is open, `NotRunning`
  otherwise; a failure returns `Unknown`.
