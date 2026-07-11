# Contract: Application View-Model (`app` module)

Signatures are the intended Rust shape; names may be refined during
implementation provided the behavior and the testability seam hold. `ui.rs`
(egui rendering) is out of this contract's tested surface and is validated
manually.

## Pure derivations (unit tested)

```rust
pub enum BeaconCondition { InstalledCurrent, InstalledOutdated, NotInstalled, AddonsNotFound }
pub struct BeaconLight { pub green: bool, pub tooltip: &'static str }

pub fn beacon_light(condition: BeaconCondition) -> BeaconLight;
pub fn uninstall_enabled(condition: BeaconCondition) -> bool;

pub struct AppStateLabel { pub indicator: &'static str, pub button: &'static str }
pub fn app_state_label(suspended: bool) -> AppStateLabel;

pub struct FishingLabel { pub indicator: String, pub button: &'static str }
pub fn fishing_label(state: FishingState) -> FishingLabel;

pub struct SkillRow { /* index, label, active, weave_type, overrides */ }
pub fn skill_rows(config: &WeaveConfig) -> Vec<SkillRow>;
```

- `beacon_light` and `uninstall_enabled` follow the tables in data-model.md.
- `app_state_label(false)` = Running/Suspend; `(true)` = Suspended/Resume.
- `fishing_label(Disabled)` = Idle/"Go Fish"; any active state = state-name/"Stop
  Fishing".
- `skill_rows` labels slot 6 "Ultimate (R)" and slot 7 "Synergy (X)".

## Reader-event routing (unit tested)

```rust
pub fn route_reader_event(
    event: PixelBusEvent,
    weave: &mut WeaveEngine,
    fishing: &mut FishingController,
    now_ms: u64,
    sink: &mut impl FishingSink,
);
```

- `Latency(ms)` -> `weave.set_latency(Some(ms))` (nothing to fishing).
- `SignalLost` -> `weave.set_latency(None)` and `fishing.on_event(SignalLost, ...)`.
- `FishingStarted` / `BiteDetected` / `FishingStopped` -> the mapped
  `DetectorEvent` to `fishing.on_event`.
- `Heartbeat` -> forwarded to the controller (a no-op there).
- Implemented by reusing `fishing::map_event`; latency is set before the map so a
  Latency event never reaches fishing.

## Log view (unit tested)

```rust
pub struct LogRow { pub text: String, pub color: LogColor }
pub fn level_color(level: tracing::Level) -> LogColor;
pub fn build_log_view(events: &[LogEvent], min_level: LevelName) -> Vec<LogRow>;
pub fn autoscroll(at_bottom: bool) -> bool;
```

- `level_color`: ERROR red, WARN amber, INFO neutral, DEBUG dim, TRACE dimmer.
- `build_log_view` keeps events whose level is at or above `min_level`, in order,
  and formats each row's text from the event.

## Settings form (unit tested)

```rust
impl SettingsForm {
    pub fn load(settings: &Settings) -> SettingsForm;         // reads all 10.3 values
    pub fn apply(&self, settings: &mut Settings) -> Vec<Notice>; // writes back + notices
}
```

- `load` populates keybindings, timing, per-slot overrides, latency, fishing,
  pixel bus, beacon, logging, theme, and always-on-top from `Settings` (reusing each
  subsystem's load where one exists).
- `apply` writes every category back into `Settings` (reusing each subsystem's
  store) and returns fallback notices; the caller persists with `config::save`.
- Round-trip property: `SettingsForm::load(settings).apply(&mut s2)` yields a
  `Settings` equal in every 10.3 category to `settings` (given valid inputs).

## Config additions

```rust
pub enum Theme { Dark, Light }                 // src/config
// Settings gains additive `pixelbus` and `ui` opaque sections (Value), default null.
```

- `pixelbus`: `{ tolerance, interval_fishing_ms, interval_idle_ms }` -> `ReaderConfig`.
- `ui`: `{ theme, always_on_top }` -> `UiPrefs`. Both absent/null yield defaults; an
  invalid value falls back with a notice.

## AppModel intents

```rust
pub enum UiIntent { ToggleSuspend, SetFishing(bool), InstallBeacon, UninstallBeacon,
                    EditSkill(u8, SkillEdit), ApplySettings(SettingsForm),
                    ToggleLogPanel(bool), SetLogFilter(LevelName) }

impl AppModel {
    pub fn apply_intent(&mut self, intent: UiIntent) -> Vec<Notice>;
    pub fn view(&self) -> AppView;   // derived display state for the frame
}
```

- `ToggleSuspend` flips `InputEngine` suspend; `SetFishing` calls the fishing
  controller enable/disable; `InstallBeacon`/`UninstallBeacon` resolve the AddOns
  dir and call the manager (uninstall only after the UI confirmation); `EditSkill`
  updates the weave config slot; `ApplySettings` applies the form and saves;
  `ToggleLogPanel`/`SetLogFilter` update the log panel state.
- Tested by constructing an `AppModel` over in-memory subsystems and asserting the
  effect of each intent on the underlying subsystem or config.

## Manual-validation surface (not unit tested)

- `app::ui` egui rendering and `main.rs` windowing/threading. Validated with the
  `quickstart.md` manual checklist on real hardware.
