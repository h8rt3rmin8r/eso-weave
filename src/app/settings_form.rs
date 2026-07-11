//! The editable settings form and its mapping to and from the persisted
//! configuration, covering every master specification section 10.3 category.

use crate::beacon::{self, BeaconPrefs};
use crate::config::{LoggingPrefs, Notice, NoticeKind, Settings, Theme};
use crate::fishing::FishingConfig;
use crate::input::bindings::BindingTable;
use crate::pixelbus::{self, ReaderConfig};
use crate::weave::{LatencyConfig, WeaveConfig, WeaveEngine};

/// The default live-log panel height in points.
pub const DEFAULT_LOG_HEIGHT: u32 = 160;

/// GUI preferences (the `ui` settings section).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiPrefs {
    /// The active theme.
    pub theme: Theme,
    /// Whether the window stays on top.
    pub always_on_top: bool,
    /// The persisted live-log panel height in points (a user layout preference).
    pub log_panel_height: u32,
}

impl Default for UiPrefs {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            always_on_top: false,
            log_panel_height: DEFAULT_LOG_HEIGHT,
        }
    }
}

/// Reads the `ui` section into [`UiPrefs`], falling back with a notice on an
/// invalid theme.
pub fn ui_from_value(value: &serde_json::Value) -> (UiPrefs, Vec<Notice>) {
    let mut notices = Vec::new();
    let defaults = UiPrefs::default();
    if value.is_null() {
        return (defaults, notices);
    }
    let theme = match value.get("theme").and_then(|v| v.as_str()) {
        None => defaults.theme,
        Some("dark") => Theme::Dark,
        Some("light") => Theme::Light,
        Some(other) => {
            notices.push(Notice {
                kind: NoticeKind::InvalidValue,
                message: format!("ui theme '{other}' is not recognized; using default"),
            });
            defaults.theme
        }
    };
    let always_on_top = value
        .get("always_on_top")
        .and_then(|v| v.as_bool())
        .unwrap_or(defaults.always_on_top);
    let log_panel_height = value
        .get("log_panel_height")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(defaults.log_panel_height);
    (
        UiPrefs {
            theme,
            always_on_top,
            log_panel_height,
        },
        notices,
    )
}

/// Serializes [`UiPrefs`] to the `ui` section value.
pub fn ui_to_value(prefs: &UiPrefs) -> serde_json::Value {
    let theme = match prefs.theme {
        Theme::Dark => "dark",
        Theme::Light => "light",
    };
    serde_json::json!({
        "theme": theme,
        "always_on_top": prefs.always_on_top,
        "log_panel_height": prefs.log_panel_height,
    })
}

/// An editable in-memory copy of every section-10.3 setting.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsForm {
    /// Keybindings (action to key).
    pub bindings: BindingTable,
    /// Weave slots and global timing.
    pub weave: WeaveConfig,
    /// Latency adaptation (enabled and k).
    pub latency: LatencyConfig,
    /// Fishing timings and interact key.
    pub fishing: FishingConfig,
    /// Pixel bus sampling tolerance and intervals.
    pub reader: ReaderConfig,
    /// AddOns path override and environment.
    pub beacon: BeaconPrefs,
    /// Log level and file logging.
    pub logging: LoggingPrefs,
    /// Theme and always-on-top.
    pub ui: UiPrefs,
}

impl SettingsForm {
    /// Loads the form from the configuration, collecting fallback notices.
    pub fn load(settings: &Settings) -> (SettingsForm, Vec<Notice>) {
        let mut notices = Vec::new();

        let (bindings, binding_notices) = BindingTable::from_settings_map(&settings.bindings);
        notices.extend(binding_notices);

        let mut engine = WeaveEngine::new(WeaveConfig::default());
        notices.extend(engine.load(settings));
        let weave = engine.config().clone();
        let latency = *engine.latency_config();

        let fishing = FishingConfig::load(&settings.fishing, &mut notices);
        let reader = pixelbus::load_reader_config(&settings.pixelbus, &mut notices);
        let beacon = beacon::prefs_from_value(&settings.beacon);
        let logging = settings.logging.clone();
        let (ui, ui_notices) = ui_from_value(&settings.ui);
        notices.extend(ui_notices);

        (
            SettingsForm {
                bindings,
                weave,
                latency,
                fishing,
                reader,
                beacon,
                logging,
                ui,
            },
            notices,
        )
    }

    /// Writes the form back into the configuration, reusing each subsystem's
    /// store. The caller persists the result with `config::save`.
    pub fn apply(&self, settings: &mut Settings) {
        settings.bindings = self.bindings.to_settings_map();

        let mut engine = WeaveEngine::new(self.weave.clone());
        engine.set_latency_config(self.latency);
        engine.store(settings);

        settings.fishing = self.fishing.store();
        settings.pixelbus = pixelbus::store_reader_config(&self.reader);
        settings.beacon = beacon::prefs_to_value(&self.beacon);
        settings.logging = self.logging.clone();
        settings.ui = ui_to_value(&self.ui);
    }
}
