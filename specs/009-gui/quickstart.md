# Quickstart: Graphical User Interface

The automated gate covers the view-model, settings mapping, routing, and log view.
The egui rendering and real windowing/threading are validated with the manual
checklist below, since a native window cannot be exercised in the automated
environment.

## Prerequisites

- The repository builds (features 001, 002, 003, 005, 006, 007 are present).
- `cargo` is available; run all commands from the repo root.

## Automated gate (CI parity)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
```

- `tests/app_view_model.rs`, `tests/app_settings.rs`, and `tests/app_log_view.rs`
  cover the derivations, intents, routing, settings round-trip, and log view.
- A clean `cargo build` proves the eframe app and all threading wiring compile.

## Automated scenarios (view-model)

- Status controls: toggling suspend flips `InputEngine::is_suspended` and the
  derived label; `SetFishing(true)` enables the fishing controller and the label
  becomes "Stop Fishing".
- Beacon light: each `BeaconCondition` yields the defined color and tooltip;
  `uninstall_enabled` is false for NotInstalled and AddonsNotFound.
- Skills: `skill_rows` labels slot 6 "Ultimate (R)" and slot 7 "Synergy (X)";
  `EditSkill` updates active, weave type, and overrides in the weave config.
- Settings: `SettingsForm::load(&settings).apply(&mut s2)` reproduces every 10.3
  category; an invalid field falls back with a notice.
- Routing: `route_reader_event` sets weave latency on Latency, clears it and
  disables fishing on SignalLost, and forwards fishing events to the controller.
- Log view: `build_log_view` filters by level and colors each row per the mapping.

## Manual validation checklist (real hardware)

Run `cargo run` and confirm:

1. A resizable window opens with a menu bar, a status region, and a skills region.
2. Suspend/Resume toggles the app-state indicator and the input engine suspends.
3. Go Fish/Stop Fishing toggles the fishing indicator.
4. The PixelBeacon light is green when installed and current, red otherwise, and
   its tooltip states the exact condition; Install installs or updates; Uninstall
   asks for confirmation and is disabled when not installed.
5. Each skill row shows its label, active checkbox, weave-type dropdown (four
   entries), and override control, and edits take effect.
6. View > Live Log attaches a bottom panel; events are colorized by level; the
   view autoscrolls at the bottom and pauses when scrolled up; the level filter
   works; unchecking removes the panel.
7. File > Settings edits every category; Apply persists to the config file and the
   values survive a restart; changing theme and always-on-top takes effect.
8. File > Exit closes the window.

## Notes

- The worker loop that feeds reader `Latency`/`SignalLost`/fishing events into the
  engines runs on its own thread and is exercised in-game; the routing it calls is
  unit-tested here.
