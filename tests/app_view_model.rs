//! View-model tests for the GUI: derivations, routing, skills, and intents.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use eso_weave::app::{
    app_state_label, beacon_light, default_delay_for, fishing_label, override_edit_for,
    route_reader_event, skill_rows, status_line_app, status_line_beacon, status_line_fishing,
    uninstall_enabled, AppModel, BeaconCondition, SkillEdit, StatusRole, UiIntent,
};
use eso_weave::beacon::{self, BeaconPrefs, Environment};
use eso_weave::config::{LoggingPrefs, Settings};
use eso_weave::fishing::{FishingConfig, FishingController, FishingState, MockFishingSink};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::InputEngine;
use eso_weave::logging;
use eso_weave::pixelbus::PixelBusEvent;
use eso_weave::weave::{LatencyConfig, WeaveConfig, WeaveEngine, WeaveType};

// Derivations.

#[test]
fn app_state_label_reflects_suspend() {
    assert_eq!(app_state_label(false).indicator, "Running");
    assert_eq!(app_state_label(false).button, "Suspend");
    assert_eq!(app_state_label(true).indicator, "Suspended");
    assert_eq!(app_state_label(true).button, "Resume");
}

#[test]
fn fishing_label_reflects_state() {
    assert_eq!(fishing_label(FishingState::Disabled).indicator, "Idle");
    assert_eq!(fishing_label(FishingState::Disabled).button, "Go Fish");
    let armed = fishing_label(FishingState::Armed);
    assert_eq!(armed.indicator, "Armed");
    assert_eq!(armed.button, "Stop Fishing");
}

#[test]
fn beacon_light_maps_every_condition() {
    let current = beacon_light(BeaconCondition::InstalledCurrent);
    assert!(current.green);
    assert_eq!(current.tooltip, "installed and current");

    let outdated = beacon_light(BeaconCondition::InstalledOutdated);
    assert!(!outdated.green);
    assert_eq!(outdated.tooltip, "installed but outdated");

    assert_eq!(
        beacon_light(BeaconCondition::NotInstalled).tooltip,
        "not installed"
    );
    assert!(!beacon_light(BeaconCondition::NotInstalled).green);

    assert_eq!(
        beacon_light(BeaconCondition::AddonsNotFound).tooltip,
        "AddOns directory not found"
    );

    assert!(uninstall_enabled(BeaconCondition::InstalledCurrent));
    assert!(uninstall_enabled(BeaconCondition::InstalledOutdated));
    assert!(!uninstall_enabled(BeaconCondition::NotInstalled));
    assert!(!uninstall_enabled(BeaconCondition::AddonsNotFound));
}

#[test]
fn skill_rows_label_ultimate_and_synergy() {
    let rows = skill_rows(&WeaveConfig::default());
    assert_eq!(rows.len(), 7);
    assert_eq!(rows[0].label, "Skill 1");
    assert_eq!(rows[5].label, "Ultimate (R)");
    assert_eq!(rows[6].label, "Synergy (X)");
}

// Status-line derivations (US1).

#[test]
fn status_line_app_reflects_suspend() {
    let running = status_line_app(false);
    assert_eq!(running.title, "Status");
    assert_eq!(running.state_text, "Running");
    assert_eq!(running.role, StatusRole::Healthy);

    let suspended = status_line_app(true);
    assert_eq!(suspended.state_text, "Suspended");
    assert_eq!(suspended.role, StatusRole::Warning);
}

#[test]
fn status_line_fishing_reflects_state() {
    let idle = status_line_fishing(FishingState::Disabled);
    assert_eq!(idle.title, "Fishing");
    assert_eq!(idle.state_text, "Idle");
    assert_eq!(idle.role, StatusRole::Muted);

    let waiting = status_line_fishing(FishingState::Waiting);
    assert_eq!(waiting.state_text, "Waiting");
    assert_eq!(waiting.role, StatusRole::Active);
}

#[test]
fn status_line_beacon_maps_conditions() {
    assert_eq!(
        status_line_beacon(BeaconCondition::InstalledCurrent).role,
        StatusRole::Healthy
    );
    assert_eq!(
        status_line_beacon(BeaconCondition::InstalledOutdated).role,
        StatusRole::Warning
    );
    assert_eq!(
        status_line_beacon(BeaconCondition::NotInstalled).role,
        StatusRole::Muted
    );
    assert_eq!(
        status_line_beacon(BeaconCondition::AddonsNotFound).role,
        StatusRole::Error
    );
    assert_eq!(
        status_line_beacon(BeaconCondition::InstalledCurrent).title,
        "Pixel Beacon (Addon)"
    );
}

// Skill effective-delay display (US1).

#[test]
fn skill_row_shows_inherited_default_when_no_override() {
    let config = WeaveConfig::default();
    let rows = skill_rows(&config);
    // Default slots are light attacks; the effective delay is the global d_weave
    // default, and the row is not marked as overridden (so it is shown muted,
    // never as a literal zero).
    assert!(!rows[0].is_override);
    assert_eq!(rows[0].effective_delay, config.timing.d_weave);
    assert_eq!(
        default_delay_for(&config.timing, WeaveType::LightAttack),
        config.timing.d_weave
    );
}

#[test]
fn skill_override_targets_the_rows_weave_type() {
    // A heavy-attack override edits d_heavy, not d_weave.
    assert_eq!(
        override_edit_for(WeaveType::HeavyAttack, Some(640)),
        SkillEdit::OverrideDHeavy(Some(640))
    );
    assert_eq!(
        override_edit_for(WeaveType::BashAttack, Some(125)),
        SkillEdit::OverrideDBash(Some(125))
    );
    assert_eq!(
        override_edit_for(WeaveType::LightAttack, Some(50)),
        SkillEdit::OverrideDWeave(Some(50))
    );

    let mut config = WeaveConfig::default();
    config.slots[0].weave_type = WeaveType::HeavyAttack;
    config.slots[0].overrides.d_heavy = Some(700);
    let rows = skill_rows(&config);
    assert!(rows[0].is_override);
    assert_eq!(rows[0].effective_delay, 700);
}

// Reader-event routing.

#[test]
fn routing_directs_events_to_the_right_subsystems() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    weave.set_latency_config(LatencyConfig {
        enabled: true,
        k: 0.25,
    });
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut sink = MockFishingSink::new();

    fishing.set_enabled(true, 0, &mut sink); // Armed
    assert_eq!(fishing.state(), FishingState::Armed);

    route_reader_event(
        PixelBusEvent::Latency(120),
        &mut weave,
        &mut fishing,
        1,
        &mut sink,
    );
    assert_eq!(weave.current_latency(), Some(120));
    assert_eq!(
        fishing.state(),
        FishingState::Armed,
        "latency does not touch fishing"
    );

    route_reader_event(
        PixelBusEvent::FishingStarted,
        &mut weave,
        &mut fishing,
        2,
        &mut sink,
    );
    assert_eq!(fishing.state(), FishingState::Waiting);

    route_reader_event(
        PixelBusEvent::BiteDetected,
        &mut weave,
        &mut fishing,
        3,
        &mut sink,
    );
    assert_eq!(fishing.state(), FishingState::Reeling);

    route_reader_event(
        PixelBusEvent::SignalLost,
        &mut weave,
        &mut fishing,
        4,
        &mut sink,
    );
    assert_eq!(
        fishing.state(),
        FishingState::Disabled,
        "signal loss disables fishing"
    );
    assert_eq!(
        weave.current_latency(),
        None,
        "signal loss clears weave latency"
    );

    route_reader_event(
        PixelBusEvent::Heartbeat,
        &mut weave,
        &mut fishing,
        5,
        &mut sink,
    );
    assert_eq!(
        fishing.state(),
        FishingState::Disabled,
        "heartbeat is a no-op"
    );
}

// AppModel intents.

fn model_with_beacon_root(root: &std::path::Path) -> AppModel {
    let (engine, _rx) = InputEngine::new(BindingTable::default(), 16);
    let weave = Arc::new(Mutex::new(WeaveEngine::new(WeaveConfig::default())));
    let fishing = Arc::new(Mutex::new(FishingController::new(FishingConfig::default())));
    let (_dispatch, log) = logging::build(&LoggingPrefs::default(), PathBuf::from("."));

    let prefs = BeaconPrefs {
        path_override: Some(root.to_path_buf()),
        environment: Environment::Live,
    };
    let settings = Settings {
        beacon: beacon::prefs_to_value(&prefs),
        ..Settings::default()
    };

    AppModel::new(
        Arc::new(engine),
        weave,
        fishing,
        Box::new(MockFishingSink::new()),
        log,
        settings,
        None,
    )
}

#[test]
fn toggle_suspend_intent_flips_input_engine() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());
    assert_eq!(model.view().app_state.indicator, "Running");
    model.apply_intent(UiIntent::ToggleSuspend);
    assert_eq!(model.view().app_state.indicator, "Suspended");
    model.apply_intent(UiIntent::ToggleSuspend);
    assert_eq!(model.view().app_state.indicator, "Running");
}

#[test]
fn set_fishing_intent_enables_controller() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());
    assert_eq!(model.view().fishing.button, "Go Fish");
    model.apply_intent(UiIntent::SetFishing(true));
    assert_eq!(model.view().fishing.button, "Stop Fishing");
    model.apply_intent(UiIntent::SetFishing(false));
    assert_eq!(model.view().fishing.button, "Go Fish");
}

#[test]
fn install_and_uninstall_beacon_intents() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());
    assert_eq!(model.view().beacon_condition, BeaconCondition::NotInstalled);

    model.apply_intent(UiIntent::InstallBeacon);
    assert_eq!(
        model.view().beacon_condition,
        BeaconCondition::InstalledCurrent
    );
    assert!(model.view().uninstall_enabled);

    model.apply_intent(UiIntent::UninstallBeacon);
    assert_eq!(model.view().beacon_condition, BeaconCondition::NotInstalled);
    assert!(!model.view().uninstall_enabled);
}

#[test]
fn edit_skill_intent_updates_weave_config() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());

    model.apply_intent(UiIntent::EditSkill(1, SkillEdit::Active(false)));
    assert!(!model.view().skills[0].active);

    model.apply_intent(UiIntent::EditSkill(
        1,
        SkillEdit::WeaveType(WeaveType::HeavyAttack),
    ));
    assert_eq!(model.view().skills[0].weave_type, WeaveType::HeavyAttack);

    model.apply_intent(UiIntent::EditSkill(1, SkillEdit::OverrideDWeave(Some(200))));
    assert_eq!(model.view().skills[0].override_d_weave, Some(200));

    model.apply_intent(UiIntent::EditSkill(1, SkillEdit::OverrideDWeave(None)));
    assert_eq!(model.view().skills[0].override_d_weave, None);
}
