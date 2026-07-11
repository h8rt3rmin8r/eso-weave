//! Settings-form and config-section tests for the GUI.

use eso_weave::app::settings_form::{ui_from_value, ui_to_value, SettingsForm, UiPrefs};
use eso_weave::config::{LevelName, NoticeKind, Settings, Theme};
use eso_weave::pixelbus::{load_reader_config, store_reader_config, ReaderConfig};
use eso_weave::weave::LatencyConfig;

// Config-section round-trips.

#[test]
fn pixelbus_section_round_trips_and_defaults() {
    let mut notices = Vec::new();
    assert_eq!(
        load_reader_config(&serde_json::Value::Null, &mut notices),
        ReaderConfig::default()
    );
    assert!(notices.is_empty());

    let custom = ReaderConfig {
        tolerance: 5,
        interval_fishing_ms: 80,
        interval_idle_ms: 800,
        ..ReaderConfig::default()
    };
    let value = store_reader_config(&custom);
    let mut notices = Vec::new();
    assert_eq!(load_reader_config(&value, &mut notices), custom);
    assert!(notices.is_empty());
}

#[test]
fn pixelbus_invalid_interval_falls_back_with_notice() {
    let mut notices = Vec::new();
    let value = serde_json::json!({ "interval_fishing_ms": 0 });
    let config = load_reader_config(&value, &mut notices);
    assert_eq!(
        config.interval_fishing_ms,
        ReaderConfig::default().interval_fishing_ms
    );
    assert!(notices.iter().any(|n| n.kind == NoticeKind::InvalidValue));
}

#[test]
fn ui_section_round_trips_and_defaults() {
    let (prefs, notices) = ui_from_value(&serde_json::Value::Null);
    assert_eq!(prefs, UiPrefs::default());
    assert!(notices.is_empty());

    let custom = UiPrefs {
        theme: Theme::Light,
        always_on_top: true,
    };
    let (loaded, notices) = ui_from_value(&ui_to_value(&custom));
    assert_eq!(loaded, custom);
    assert!(notices.is_empty());
}

#[test]
fn ui_invalid_theme_falls_back_with_notice() {
    let value = serde_json::json!({ "theme": "neon" });
    let (prefs, notices) = ui_from_value(&value);
    assert_eq!(prefs.theme, Theme::default());
    assert!(notices.iter().any(|n| n.kind == NoticeKind::InvalidValue));
}

// Full settings form round-trip across every section-10.3 category.

#[test]
fn settings_form_round_trips_defaults_without_notice() {
    let (_form, notices) = SettingsForm::load(&Settings::default());
    assert!(notices.is_empty());
}

#[test]
fn settings_form_round_trips_custom_values() {
    let (form, _n) = SettingsForm::load(&Settings::default());

    let mut edited = form;
    edited.ui.theme = Theme::Light;
    edited.ui.always_on_top = true;
    edited.fishing.arm_timeout_ms = 4000;
    edited.reader.tolerance = 5;
    edited.latency = LatencyConfig {
        enabled: true,
        k: 0.5,
    };
    edited.logging.level = LevelName::Debug;
    edited.logging.file_enabled = true;
    edited.weave.timing.d_weave = 77;
    edited.weave.slots[0].active = false;

    let mut settings = Settings::default();
    edited.apply(&mut settings);

    let (loaded, notices) = SettingsForm::load(&settings);
    assert!(notices.is_empty());
    assert_eq!(loaded.ui.theme, Theme::Light);
    assert!(loaded.ui.always_on_top);
    assert_eq!(loaded.fishing.arm_timeout_ms, 4000);
    assert_eq!(loaded.reader.tolerance, 5);
    assert_eq!(
        loaded.latency,
        LatencyConfig {
            enabled: true,
            k: 0.5
        }
    );
    assert_eq!(loaded.logging.level, LevelName::Debug);
    assert!(loaded.logging.file_enabled);
    assert_eq!(loaded.weave.timing.d_weave, 77);
    assert!(!loaded.weave.slots[0].active);
}
