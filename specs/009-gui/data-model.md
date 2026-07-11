# Phase 1 Data Model: Graphical User Interface

New types live in the `app` module (plus two small config additions). The
view-model types and pure derivations are tested; `ui.rs` renders them.

## Theme (config)

| Variant | Meaning |
| --- | --- |
| `Dark` | The default theme. |
| `Light` | Optional light theme. |

- Serialized in the `ui` settings section as `"dark"` / `"light"`.

## UiPrefs (config `ui` section)

| Field | Type | Default | Meaning |
| --- | --- | --- | --- |
| `theme` | `Theme` | `Dark` | The active theme. |
| `always_on_top` | `bool` | `false` | Whether the window stays on top. |

## PixelBusPrefs (config `pixelbus` section)

| Field | Type | Default | Meaning |
| --- | --- | --- | --- |
| `tolerance` | `u8` | 2 | Per-channel color match tolerance. |
| `interval_fishing_ms` | `u64` | 100 | Sampling interval while fishing is enabled. |
| `interval_idle_ms` | `u64` | 1000 | Sampling interval otherwise. |

- Maps to the reader's `ReaderConfig` fields; absent or invalid falls back to the
  `ReaderConfig` defaults with a notice.

## BeaconCondition and BeaconLight (pure derivation)

`BeaconCondition` is derived from the Beacon Manager status and discovery result:

| Condition | Source |
| --- | --- |
| `InstalledCurrent` | `BeaconStatus::ManagedUpToDate` |
| `InstalledOutdated` | `ManagedVersionMismatch` or `Unmanaged` (present but not current/managed) |
| `NotInstalled` | `BeaconStatus::NotInstalled` |
| `AddonsNotFound` | discovery returned `DiscoveryError` |

`beacon_light(condition) -> BeaconLight { green: bool, tooltip: &'static str }`:

| Condition | green | tooltip |
| --- | --- | --- |
| `InstalledCurrent` | true | "installed and current" |
| `InstalledOutdated` | false | "installed but outdated" |
| `NotInstalled` | false | "not installed" |
| `AddonsNotFound` | false | "AddOns directory not found" |

- `uninstall_enabled(condition)` is true only for `InstalledCurrent` and
  `InstalledOutdated` (a folder is present to remove); false for `NotInstalled` and
  `AddonsNotFound`.

## App-state and fishing labels (pure)

- `app_state_label(suspended: bool) -> AppStateLabel { indicator, button }`:
  `false -> { "Running", "Suspend" }`, `true -> { "Suspended", "Resume" }`.
- `fishing_label(state: FishingState) -> FishingLabel { indicator, button }`:
  `Disabled -> { "Idle", "Go Fish" }`; any active state (`Armed`, `Waiting`,
  `Reeling`, `Recast`) -> { the state name, "Stop Fishing" }.

## SkillRow (pure view of a slot)

| Field | Type | Meaning |
| --- | --- | --- |
| `index` | `u8` | Slot index 1 through 7. |
| `label` | `String` | e.g. "Skill 1", "Ultimate (R)", "Synergy (X)". |
| `active` | `bool` | Whether the slot is active. |
| `weave_type` | `WeaveType` | The slot's weave type. |
| `override_d_weave` / `override_d_heavy` / `override_d_bash` | `Option<u32>` | Per-slot overrides. |

- `skill_rows(&WeaveConfig) -> Vec<SkillRow>` derives the rows; editing a row maps
  back to the corresponding slot via a `SkillEdit` intent.

## LogRow and level color (pure)

- `level_color(level: Level) -> LogColor`: ERROR red, WARN amber, INFO neutral,
  DEBUG dim, TRACE dimmer (as RGB constants, theme-independent hues).
- `build_log_view(events: &[LogEvent], min_level: LevelName) -> Vec<LogRow>` filters
  to events at or above `min_level` and maps each to `{ text, color }`.
- Autoscroll rule: `autoscroll(at_bottom: bool) -> bool` is simply `at_bottom` (the
  panel autoscrolls only while the user is at the bottom).

## UiIntent

The user actions the view-model applies:

| Variant | Effect |
| --- | --- |
| `ToggleSuspend` | Flip `InputEngine` suspend. |
| `SetFishing(bool)` | Enable/disable the fishing controller. |
| `InstallBeacon` | Resolve AddOns and install/update. |
| `UninstallBeacon` | Resolve AddOns and uninstall (after confirmation in the UI). |
| `EditSkill(index, SkillEdit)` | Update a slot's active/weave/override in the weave config. |
| `OpenSettings` / `ApplySettings(SettingsForm)` | Load / apply-and-save settings. |
| `ToggleLogPanel(bool)` | Attach/detach the log panel. |
| `SetLogFilter(LevelName)` | Set the panel-local minimum level. |

## SettingsForm

An editable in-memory copy of every section-10.3 value: keybindings (action to
key), global timing, per-slot overrides, latency (`enabled`, `k`), fishing timings
and interact key, pixel bus (tolerance and intervals), beacon (path override,
environment), logging (level, file enabled), theme, and always-on-top.

- `SettingsForm::load(settings, subsystems) -> SettingsForm` reads current values.
- `SettingsForm::apply(self, settings, subsystems) -> Vec<Notice>` writes the form
  back into `Settings` (reusing each subsystem's `store`) and returns fallback
  notices; the caller then saves `Settings` via `config::save`.

## AppModel

Holds the shared handles (`Arc<InputEngine>`, `Arc<Mutex<WeaveEngine>>`,
`Arc<Mutex<FishingController>>`, `LogHandle`, the current `Settings`, the
`config_dir`, and the resolved `BeaconPrefs`), derives the view each frame, and
applies `UiIntent`s. It is constructed in tests with in-memory subsystems.
