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
pub mod strings;
pub mod theme;
pub mod ui;
pub mod widgets;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::beacon::{self, BeaconPrefs, BeaconStatus};
use crate::config::state::{SessionState, CURRENT_STATE_VERSION};
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

/// A brand status role, mapping a state to a palette color from one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusRole {
    /// Running and healthy (ok color).
    Healthy,
    /// A warning condition (warn color).
    Warning,
    /// An active operation in progress (accent color).
    Active,
    /// Idle or absent (muted color).
    Muted,
    /// An error or lost signal (error color).
    Error,
}

/// A normalized status line for the top region: a title, a colorized state
/// field, and a tooltip. Derived each frame from the subsystem state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLine {
    /// The section title, shown first on the line.
    pub title: &'static str,
    /// The normalized state text.
    pub state_text: String,
    /// The palette role that colors the state field.
    pub role: StatusRole,
    /// The hover tooltip for the line.
    pub tooltip: &'static str,
}

/// Derives the Status line from the suspend state.
pub fn status_line_app(suspended: bool) -> StatusLine {
    if suspended {
        StatusLine {
            title: strings::STATUS_TITLE,
            state_text: "Suspended".to_string(),
            role: StatusRole::Warning,
            tooltip: strings::STATUS_TOOLTIP,
        }
    } else {
        StatusLine {
            title: strings::STATUS_TITLE,
            state_text: "Running".to_string(),
            role: StatusRole::Healthy,
            tooltip: strings::STATUS_TOOLTIP,
        }
    }
}

/// Derives the Fishing line from the controller state.
pub fn status_line_fishing(state: FishingState) -> StatusLine {
    let (state_text, role) = match state {
        FishingState::Disabled => ("Idle".to_string(), StatusRole::Muted),
        active => (format!("{active:?}"), StatusRole::Active),
    };
    StatusLine {
        title: strings::FISHING_TITLE,
        state_text,
        role,
        tooltip: strings::FISHING_TOOLTIP,
    }
}

/// Derives the Pixel Beacon line from the beacon condition.
pub fn status_line_beacon(condition: BeaconCondition) -> StatusLine {
    let (state_text, role) = match condition {
        BeaconCondition::InstalledCurrent => {
            ("Installed (current)".to_string(), StatusRole::Healthy)
        }
        BeaconCondition::InstalledOutdated => {
            ("Installed (outdated)".to_string(), StatusRole::Warning)
        }
        BeaconCondition::NotInstalled => ("Not installed".to_string(), StatusRole::Muted),
        BeaconCondition::AddonsNotFound => {
            ("AddOns folder not found".to_string(), StatusRole::Error)
        }
    };
    StatusLine {
        title: strings::BEACON_TITLE,
        state_text,
        role,
        tooltip: strings::BEACON_TOOLTIP,
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
    /// Whether the delay for this row's weave type is overridden.
    pub is_override: bool,
    /// The delay in effect for this row's weave type: the override when set,
    /// otherwise the global default for that weave type.
    pub effective_delay: u32,
}

/// The per-slot override (if any) for the delay matching a weave type.
pub fn override_for(overrides: &crate::weave::SlotOverrides, weave_type: WeaveType) -> Option<u32> {
    match weave_type {
        WeaveType::HeavyAttack => overrides.d_heavy,
        WeaveType::BashAttack => overrides.d_bash,
        WeaveType::LightAttack | WeaveType::BlockCasting => overrides.d_weave,
    }
}

/// The global default delay for a weave type.
pub fn default_delay_for(timing: &crate::weave::TimingConfig, weave_type: WeaveType) -> u32 {
    match weave_type {
        WeaveType::HeavyAttack => timing.d_heavy,
        WeaveType::BashAttack => timing.d_bash,
        WeaveType::LightAttack | WeaveType::BlockCasting => timing.d_weave,
    }
}

/// The [`SkillEdit`] that sets or clears the override matching a weave type.
pub fn override_edit_for(weave_type: WeaveType, value: Option<u32>) -> SkillEdit {
    match weave_type {
        WeaveType::HeavyAttack => SkillEdit::OverrideDHeavy(value),
        WeaveType::BashAttack => SkillEdit::OverrideDBash(value),
        WeaveType::LightAttack | WeaveType::BlockCasting => SkillEdit::OverrideDWeave(value),
    }
}

/// Derives the skill rows from the weave configuration.
pub fn skill_rows(config: &WeaveConfig) -> Vec<SkillRow> {
    config
        .slots
        .iter()
        .map(|slot| {
            let over = override_for(&slot.overrides, slot.weave_type);
            SkillRow {
                index: slot.index,
                label: slot_label(slot.index),
                active: slot.active,
                weave_type: slot.weave_type,
                override_d_weave: slot.overrides.d_weave,
                override_d_heavy: slot.overrides.d_heavy,
                override_d_bash: slot.overrides.d_bash,
                is_override: over.is_some(),
                effective_delay: over
                    .unwrap_or_else(|| default_delay_for(&config.timing, slot.weave_type)),
            }
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
    /// The normalized Status line (title, colorized state, tooltip).
    pub status_line: StatusLine,
    /// The normalized Fishing line.
    pub fishing_line: StatusLine,
    /// The normalized Pixel Beacon line.
    pub beacon_line: StatusLine,
    /// Whether the engine is currently suspended (for the Status toggle).
    pub suspended: bool,
    /// Whether fishing is currently active (for the Fishing toggle).
    pub fishing_active: bool,
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

/// Coalesces persistence so a continuous edit results in a single settle-write.
///
/// A change marks the config and/or session store dirty and records the time.
/// [`should_flush`](SaveScheduler::should_flush) becomes true once the store is
/// dirty and the most recent change has settled for the configured interval.
#[derive(Debug)]
pub struct SaveScheduler {
    dirty_config: bool,
    dirty_session: bool,
    last_change: Option<Instant>,
    settle: Duration,
}

impl SaveScheduler {
    /// Creates a scheduler that flushes once a change has settled for `settle`.
    pub fn new(settle: Duration) -> Self {
        Self {
            dirty_config: false,
            dirty_session: false,
            last_change: None,
            settle,
        }
    }

    /// Marks the configuration store dirty as of `now`.
    pub fn mark_config(&mut self, now: Instant) {
        self.dirty_config = true;
        self.last_change = Some(now);
    }

    /// Marks the session store dirty as of `now`.
    pub fn mark_session(&mut self, now: Instant) {
        self.dirty_session = true;
        self.last_change = Some(now);
    }

    /// Whether anything is pending a write.
    pub fn is_dirty(&self) -> bool {
        self.dirty_config || self.dirty_session
    }

    /// Whether a flush is due: dirty and settled for the configured interval.
    pub fn should_flush(&self, now: Instant) -> bool {
        match self.last_change {
            Some(t) => self.is_dirty() && now.duration_since(t) >= self.settle,
            None => false,
        }
    }

    /// Clears the dirty flags and returns which stores need writing
    /// `(config, session)`.
    pub fn take(&mut self) -> (bool, bool) {
        let flags = (self.dirty_config, self.dirty_session);
        self.dirty_config = false;
        self.dirty_session = false;
        self.last_change = None;
        flags
    }
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
    scheduler: SaveScheduler,
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
            scheduler: SaveScheduler::new(Duration::from_millis(400)),
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
        let suspended = self.input.is_suspended();
        AppView {
            app_state: app_state_label(suspended),
            fishing: fishing_label(fishing_state),
            status_line: status_line_app(suspended),
            fishing_line: status_line_fishing(fishing_state),
            beacon_line: status_line_beacon(condition),
            suspended,
            fishing_active: fishing_state != FishingState::Disabled,
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
                self.scheduler.mark_session(Instant::now());
                Vec::new()
            }
            UiIntent::SetFishing(enabled) => {
                let now = self.now_ms();
                self.fishing
                    .lock()
                    .unwrap()
                    .set_enabled(enabled, now, self.fishing_sink.as_mut());
                self.scheduler.mark_session(Instant::now());
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
                self.scheduler.mark_config(Instant::now());
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
        // Persistence is coalesced: mark the config store dirty and let the
        // scheduler flush a single settle-write. There is no explicit save.
        self.scheduler.mark_config(Instant::now());
        notices
    }

    /// Restores the persisted session state (suspend and fishing intents) on
    /// launch. Restoring a running or fishing-on state performs no input while
    /// the game window is unfocused, because synthesis and suppression are scoped
    /// to the focused game window by the input backend.
    pub fn restore_session(&mut self, state: SessionState) {
        if state.suspended != self.input.is_suspended() {
            self.input.set_suspended(state.suspended);
        }
        if state.fishing {
            let now = self.now_ms();
            self.fishing
                .lock()
                .unwrap()
                .set_enabled(true, now, self.fishing_sink.as_mut());
        }
    }

    /// The current session state to persist (suspend flag and fishing on/off
    /// intent, never a transient fishing sub-state).
    pub fn current_session_state(&self) -> SessionState {
        let fishing_on = self.fishing.lock().unwrap().state() != FishingState::Disabled;
        SessionState {
            schema_version: CURRENT_STATE_VERSION,
            suspended: self.input.is_suspended(),
            fishing: fishing_on,
        }
    }

    /// Flushes any pending coalesced writes if they have settled. Returns whether
    /// a write occurred (so the caller can show a save confirmation).
    pub fn maybe_flush(&mut self, now: Instant) -> bool {
        if !self.scheduler.should_flush(now) {
            return false;
        }
        let (write_config, write_session) = self.scheduler.take();
        let Some(dir) = self.config_dir.clone() else {
            return false;
        };
        let mut saved = false;
        if write_config {
            self.store_live_into_settings();
            match config::save(&dir, &self.settings) {
                Ok(()) => saved = true,
                Err(err) => {
                    tracing::warn!(target: "eso_weave::config", "could not save settings: {err}");
                }
            }
        }
        if write_session {
            let state = self.current_session_state();
            match config::state::save(&dir, &state) {
                Ok(()) => saved = true,
                Err(err) => {
                    tracing::warn!(target: "eso_weave::config", "could not save session state: {err}");
                }
            }
        }
        saved
    }

    /// Syncs the live weave engine configuration back into the settings so
    /// main-window skill edits are persisted.
    fn store_live_into_settings(&mut self) {
        let weave = self.weave.lock().unwrap();
        weave.store(&mut self.settings);
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
