//! Application view-model: the testable core of the GUI.
//!
//! Everything correctness-bearing (display derivation, UI-intent handling, the
//! settings mapping in [`settings_form`], the reader-event routing in
//! [`routing`], and the log view in [`log_view`]) lives here and is unit-tested
//! against the project's in-memory subsystems. The egui rendering in [`ui`] is a
//! thin layer that reads this view-model and raises intents; it is validated
//! manually because a native window cannot be exercised headlessly.

pub mod beacon_light;
pub mod log_view;
pub mod routing;
pub mod settings_form;
pub mod theme;
pub mod ui;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::beacon::{self, BeaconPrefs, BeaconStatus};
use crate::config::{self, LevelName, Notice, Settings};
use crate::fishing::{FishingController, FishingSink, FishingState};
use crate::input::InputEngine;
use crate::logging::LogHandle;
use crate::weave::{WeaveConfig, WeaveEngine, WeaveType};

pub use beacon_light::{beacon_light, uninstall_enabled, BeaconCondition, BeaconLight};
pub use log_view::{build_log_view, level_color, LogColor, LogRow};
pub use routing::route_reader_event;
pub use settings_form::{SettingsForm, UiPrefs};

/// The application-state indicator and its toggle button label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppStateLabel {
    /// The state indicator text.
    pub indicator: &'static str,
    /// The toggle button label (the action it performs).
    pub button: &'static str,
}

/// Derives the app-state label from the suspend state.
pub fn app_state_label(suspended: bool) -> AppStateLabel {
    if suspended {
        AppStateLabel {
            indicator: "Suspended",
            button: "Resume",
        }
    } else {
        AppStateLabel {
            indicator: "Running",
            button: "Suspend",
        }
    }
}

/// The fishing-state indicator and its toggle button label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FishingLabel {
    /// The fishing state indicator text.
    pub indicator: String,
    /// The toggle button label (the action it performs).
    pub button: &'static str,
}

/// Derives the fishing label from the controller state.
pub fn fishing_label(state: FishingState) -> FishingLabel {
    match state {
        FishingState::Disabled => FishingLabel {
            indicator: "Idle".to_string(),
            button: "Go Fish",
        },
        active => FishingLabel {
            indicator: format!("{active:?}"),
            button: "Stop Fishing",
        },
    }
}

/// A view of one skill slot for the skills region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillRow {
    /// The slot index (1 through 7).
    pub index: u8,
    /// The display label (e.g. "Skill 1", "Ultimate (R)", "Synergy (X)").
    pub label: String,
    /// Whether the slot is active.
    pub active: bool,
    /// The slot's weave type.
    pub weave_type: WeaveType,
    /// The per-slot `d_weave` override, if any.
    pub override_d_weave: Option<u32>,
    /// The per-slot `d_heavy` override, if any.
    pub override_d_heavy: Option<u32>,
    /// The per-slot `d_bash` override, if any.
    pub override_d_bash: Option<u32>,
}

/// Derives the skill rows from the weave configuration.
pub fn skill_rows(config: &WeaveConfig) -> Vec<SkillRow> {
    config
        .slots
        .iter()
        .map(|slot| SkillRow {
            index: slot.index,
            label: slot_label(slot.index),
            active: slot.active,
            weave_type: slot.weave_type,
            override_d_weave: slot.overrides.d_weave,
            override_d_heavy: slot.overrides.d_heavy,
            override_d_bash: slot.overrides.d_bash,
        })
        .collect()
}

fn slot_label(index: u8) -> String {
    match index {
        6 => "Ultimate (R)".to_string(),
        7 => "Synergy (X)".to_string(),
        n => format!("Skill {n}"),
    }
}

/// An edit to a skill slot from the skills region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillEdit {
    /// Set the slot's active flag.
    Active(bool),
    /// Set the slot's weave type.
    WeaveType(WeaveType),
    /// Set or clear the `d_weave` override.
    OverrideDWeave(Option<u32>),
    /// Set or clear the `d_heavy` override.
    OverrideDHeavy(Option<u32>),
    /// Set or clear the `d_bash` override.
    OverrideDBash(Option<u32>),
}

/// A user action the [`AppModel`] applies.
pub enum UiIntent {
    /// Toggle the input engine suspend state.
    ToggleSuspend,
    /// Enable or disable the fishing controller.
    SetFishing(bool),
    /// Install or update the beacon addon.
    InstallBeacon,
    /// Uninstall the beacon addon (the UI has already confirmed).
    UninstallBeacon,
    /// Edit a skill slot.
    EditSkill(u8, SkillEdit),
    /// Apply and persist the settings form.
    ApplySettings(Box<SettingsForm>),
    /// Attach or detach the live log panel.
    ToggleLogPanel(bool),
    /// Set the panel-local minimum log level.
    SetLogFilter(LevelName),
}

/// The derived display state for one frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppView {
    /// The app-state indicator and button.
    pub app_state: AppStateLabel,
    /// The fishing indicator and button.
    pub fishing: FishingLabel,
    /// The beacon status light.
    pub beacon: BeaconLight,
    /// The derived beacon condition.
    pub beacon_condition: BeaconCondition,
    /// Whether the Uninstall control is enabled.
    pub uninstall_enabled: bool,
    /// One row per skill slot.
    pub skills: Vec<SkillRow>,
    /// Whether the log panel is attached.
    pub log_panel_open: bool,
    /// The panel-local minimum log level.
    pub log_filter: LevelName,
}

/// The application model: holds the shared subsystem handles, derives the view,
/// and applies UI intents.
pub struct AppModel {
    input: Arc<InputEngine>,
    weave: Arc<Mutex<WeaveEngine>>,
    fishing: Arc<Mutex<FishingController>>,
    fishing_sink: Box<dyn FishingSink + Send>,
    clock: Instant,
    log: LogHandle,
    settings: Settings,
    config_dir: Option<PathBuf>,
    beacon_prefs: BeaconPrefs,
    log_panel_open: bool,
    log_filter: LevelName,
}

impl AppModel {
    /// Creates the model over the given shared subsystems and configuration.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        input: Arc<InputEngine>,
        weave: Arc<Mutex<WeaveEngine>>,
        fishing: Arc<Mutex<FishingController>>,
        fishing_sink: Box<dyn FishingSink + Send>,
        log: LogHandle,
        settings: Settings,
        config_dir: Option<PathBuf>,
    ) -> Self {
        let beacon_prefs = beacon::prefs_from_value(&settings.beacon);
        let log_filter = settings.logging.level;
        Self {
            input,
            weave,
            fishing,
            fishing_sink,
            clock: Instant::now(),
            log,
            settings,
            config_dir,
            beacon_prefs,
            log_panel_open: false,
            log_filter,
        }
    }

    /// A shared handle to the logging facility (for the log panel snapshot).
    pub fn log_handle(&self) -> LogHandle {
        self.log.clone()
    }

    /// The current GUI preferences (theme and always-on-top).
    pub fn ui_prefs(&self) -> UiPrefs {
        settings_form::ui_from_value(&self.settings.ui).0
    }

    /// A fresh settings form seeded from the current configuration.
    pub fn settings_form(&self) -> SettingsForm {
        SettingsForm::load(&self.settings).0
    }

    /// The current derived display state.
    pub fn view(&self) -> AppView {
        let condition = self.beacon_condition();
        let fishing_state = self.fishing.lock().unwrap().state();
        let skills = skill_rows(self.weave.lock().unwrap().config());
        AppView {
            app_state: app_state_label(self.input.is_suspended()),
            fishing: fishing_label(fishing_state),
            beacon: beacon_light(condition),
            beacon_condition: condition,
            uninstall_enabled: uninstall_enabled(condition),
            skills,
            log_panel_open: self.log_panel_open,
            log_filter: self.log_filter,
        }
    }

    /// Derives the beacon condition from discovery plus the on-disk status.
    pub fn beacon_condition(&self) -> BeaconCondition {
        match beacon::resolve_addons_dir(&self.beacon_prefs) {
            Ok(root) => BeaconCondition::from_status(beacon::status(&root)),
            Err(_) => BeaconCondition::AddonsNotFound,
        }
    }

    /// Applies a UI intent, returning any notices to surface.
    pub fn apply_intent(&mut self, intent: UiIntent) -> Vec<Notice> {
        match intent {
            UiIntent::ToggleSuspend => {
                let now = self.input.is_suspended();
                self.input.set_suspended(!now);
                Vec::new()
            }
            UiIntent::SetFishing(enabled) => {
                let now = self.now_ms();
                self.fishing
                    .lock()
                    .unwrap()
                    .set_enabled(enabled, now, self.fishing_sink.as_mut());
                Vec::new()
            }
            UiIntent::InstallBeacon => {
                self.install_beacon();
                Vec::new()
            }
            UiIntent::UninstallBeacon => {
                self.uninstall_beacon();
                Vec::new()
            }
            UiIntent::EditSkill(index, edit) => {
                self.edit_skill(index, edit);
                Vec::new()
            }
            UiIntent::ApplySettings(form) => self.apply_settings(*form),
            UiIntent::ToggleLogPanel(open) => {
                self.log_panel_open = open;
                Vec::new()
            }
            UiIntent::SetLogFilter(level) => {
                self.log_filter = level;
                Vec::new()
            }
        }
    }

    fn now_ms(&self) -> u64 {
        self.clock.elapsed().as_millis() as u64
    }

    fn edit_skill(&mut self, index: u8, edit: SkillEdit) {
        {
            let mut weave = self.weave.lock().unwrap();
            if let Some(slot) = weave
                .config_mut()
                .slots
                .iter_mut()
                .find(|slot| slot.index == index)
            {
                match edit {
                    SkillEdit::Active(active) => slot.active = active,
                    SkillEdit::WeaveType(weave_type) => slot.weave_type = weave_type,
                    SkillEdit::OverrideDWeave(value) => slot.overrides.d_weave = value,
                    SkillEdit::OverrideDHeavy(value) => slot.overrides.d_heavy = value,
                    SkillEdit::OverrideDBash(value) => slot.overrides.d_bash = value,
                }
            }
            weave.apply_activity(&self.input);
        }
    }

    fn install_beacon(&mut self) {
        match beacon::resolve_addons_dir(&self.beacon_prefs) {
            Ok(root) => match beacon::install(&root, beacon::probe_game_running()) {
                Ok(outcome) => {
                    if outcome.reload_required {
                        tracing::info!(
                            target: "eso_weave::app",
                            "PixelBeacon installed; run /reloadui or relog for it to take effect"
                        );
                    } else {
                        tracing::info!(target: "eso_weave::app", "PixelBeacon installed");
                    }
                }
                Err(err) => {
                    tracing::warn!(target: "eso_weave::app", "install failed: {err}");
                }
            },
            Err(_) => {
                tracing::warn!(target: "eso_weave::app", "install: AddOns directory not found");
            }
        }
    }

    fn uninstall_beacon(&mut self) {
        match beacon::resolve_addons_dir(&self.beacon_prefs) {
            Ok(root) => match beacon::uninstall(&root, beacon::probe_game_running()) {
                Ok(outcome) => {
                    if outcome.reload_required {
                        tracing::info!(
                            target: "eso_weave::app",
                            "PixelBeacon removed; run /reloadui or relog for it to take effect"
                        );
                    } else {
                        tracing::info!(target: "eso_weave::app", "PixelBeacon removed");
                    }
                }
                Err(err) => {
                    tracing::warn!(target: "eso_weave::app", "uninstall refused: {err}");
                }
            },
            Err(_) => {
                tracing::warn!(target: "eso_weave::app", "uninstall: AddOns directory not found");
            }
        }
    }

    fn apply_settings(&mut self, form: SettingsForm) -> Vec<Notice> {
        form.apply(&mut self.settings);
        let notices = self.reload_from_settings();
        if let Some(dir) = &self.config_dir {
            if let Err(err) = config::save(dir, &self.settings) {
                tracing::warn!(target: "eso_weave::config", "could not save settings: {err}");
            }
        }
        notices
    }

    /// Reloads the live subsystems from the current settings, returning fallback
    /// notices. Shared with startup.
    pub fn reload_from_settings(&mut self) -> Vec<Notice> {
        let mut notices = Vec::new();
        notices.extend(self.input.load_bindings(&self.settings));
        notices.extend(self.weave.lock().unwrap().load(&self.settings));
        self.weave.lock().unwrap().apply_activity(&self.input);
        self.beacon_prefs = beacon::prefs_from_value(&self.settings.beacon);
        self.log.set_level(self.settings.logging.level);
        self.log
            .set_file_enabled(self.settings.logging.file_enabled);
        notices
    }

    /// Whether the beacon status is currently installed (for enabling controls).
    pub fn beacon_installed(&self) -> bool {
        !matches!(self.beacon_condition(), BeaconCondition::NotInstalled)
            && self.beacon_status().is_some()
    }

    fn beacon_status(&self) -> Option<BeaconStatus> {
        beacon::resolve_addons_dir(&self.beacon_prefs)
            .ok()
            .map(|root| beacon::status(&root))
    }
}
