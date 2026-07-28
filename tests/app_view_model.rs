//! View-model tests for the GUI: derivations, routing, skills, and intents.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eso_weave::app::{
    app_state_label, beacon_light, combat_view, default_delay_for, fishing_label, menu_view,
    modal_extent, override_edit_for, quickslot_view, resource_view, route_reader_event, skill_rows,
    status_line_app, status_line_beacon, status_line_fishing, uninstall_enabled, weapon_bar_view,
    AppModel, BeaconCondition, SkillEdit, StatusRole, UiIntent,
};
use eso_weave::beacon::{self, BeaconPrefs, Environment};
use eso_weave::config::{LevelName, LoggingPrefs, Settings};
use eso_weave::fishing::{
    FishingConfig, FishingController, FishingState, MockFishingSink, StopReason,
};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::InputEngine;
use eso_weave::logging;
use eso_weave::pixelbus::{
    ActiveBar, CombatSignal, MenuSurface, PixelBusEvent, QuickslotState, ResourceLevel,
    ResourceSet, SlotCooldown, WeaponBarSignal, WeaponClass,
};
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
fn fishing_label_uses_plain_language_and_stop_reason() {
    // Idle with no reason.
    let idle = fishing_label(FishingState::Disabled, None);
    assert_eq!(idle.indicator, "Idle");
    assert_eq!(idle.button, "Go Fish");

    // Working states read as plain language, never an internal state name.
    assert_eq!(
        fishing_label(FishingState::Armed, None).indicator,
        "Casting"
    );
    assert_eq!(
        fishing_label(FishingState::Waiting, None).indicator,
        "Fishing (waiting for a bite)"
    );
    assert_eq!(
        fishing_label(FishingState::Reeling, None).indicator,
        "Reeling in"
    );
    assert_eq!(
        fishing_label(FishingState::Recast, None).indicator,
        "Recasting"
    );
    assert_eq!(
        fishing_label(FishingState::Armed, None).button,
        "Stop Fishing"
    );

    // Idle explains why it stopped.
    assert_eq!(
        fishing_label(FishingState::Disabled, Some(StopReason::UserStop)).indicator,
        "Idle"
    );
    assert_eq!(
        fishing_label(FishingState::Disabled, Some(StopReason::NoCastDetected)).indicator,
        "Idle (no cast detected)"
    );
    assert_eq!(
        fishing_label(FishingState::Disabled, Some(StopReason::SignalLost)).indicator,
        "Idle (signal lost)"
    );

    // No indicator is ever a raw debug state name.
    for (state, reason) in [
        (FishingState::Armed, None),
        (FishingState::Waiting, None),
        (FishingState::Reeling, None),
        (FishingState::Recast, None),
        (FishingState::Disabled, Some(StopReason::SignalLost)),
    ] {
        let indicator = fishing_label(state, reason).indicator;
        assert_ne!(indicator, format!("{state:?}"));
    }
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
    let rows = skill_rows(
        &WeaveConfig::default(),
        eso_weave::pixelbus::CooldownSet::new_unknown(),
    );
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
fn status_line_fishing_reflects_state_and_reason() {
    let idle = status_line_fishing(FishingState::Disabled, None);
    assert_eq!(idle.title, "Fishing");
    assert_eq!(idle.state_text, "Idle");
    assert_eq!(idle.role, StatusRole::Muted);

    let waiting = status_line_fishing(FishingState::Waiting, None);
    assert_eq!(waiting.state_text, "Fishing (waiting for a bite)");
    assert_eq!(waiting.role, StatusRole::Active);

    // A clean user stop stays muted; a fault-stop is warned.
    assert_eq!(
        status_line_fishing(FishingState::Disabled, Some(StopReason::UserStop)).role,
        StatusRole::Muted
    );
    let no_cast = status_line_fishing(FishingState::Disabled, Some(StopReason::NoCastDetected));
    assert_eq!(no_cast.state_text, "Idle (no cast detected)");
    assert_eq!(no_cast.role, StatusRole::Warning);
    let lost = status_line_fishing(FishingState::Disabled, Some(StopReason::SignalLost));
    assert_eq!(lost.state_text, "Idle (signal lost)");
    assert_eq!(lost.role, StatusRole::Warning);
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
    let rows = skill_rows(&config, eso_weave::pixelbus::CooldownSet::new_unknown());
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
    let rows = skill_rows(&config, eso_weave::pixelbus::CooldownSet::new_unknown());
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
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    fishing.set_enabled(true, 0, &mut sink); // Armed
    assert_eq!(fishing.state(), FishingState::Armed);

    route_reader_event(
        PixelBusEvent::Latency(120),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
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
        &mut potion,
        &input,
        2,
        &mut sink,
    );
    assert_eq!(fishing.state(), FishingState::Waiting);

    route_reader_event(
        PixelBusEvent::BiteDetected,
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        3,
        &mut sink,
    );
    assert_eq!(fishing.state(), FishingState::Reeling);

    route_reader_event(
        PixelBusEvent::SignalLost,
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
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
        &mut potion,
        &input,
        5,
        &mut sink,
    );
    assert_eq!(
        fishing.state(),
        FishingState::Disabled,
        "heartbeat is a no-op"
    );
}

#[test]
fn weapon_bar_view_shows_detected_and_unknown() {
    let detected = weapon_bar_view(
        ActiveBar::Back,
        WeaponClass::DualWield,
        WeaponClass::RestorationStaff,
    );
    assert!(detected.detected);
    assert_eq!(detected.active_bar, "Back");
    assert_eq!(detected.front, "Dual Wield");
    assert_eq!(detected.back, "Restoration Staff");
    assert_eq!(detected.role, StatusRole::Active);

    let none = weapon_bar_view(
        ActiveBar::Unknown,
        WeaponClass::Unknown,
        WeaponClass::Unknown,
    );
    assert!(!none.detected);
    assert_eq!(none.role, StatusRole::Muted);
}

#[test]
fn routing_a_weapon_bar_event_updates_the_engine() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    route_reader_event(
        PixelBusEvent::WeaponBar(WeaponBarSignal {
            bar: ActiveBar::Back,
            front: WeaponClass::DualWield,
            back: WeaponClass::RestorationStaff,
        }),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );

    assert_eq!(weave.active_bar(), ActiveBar::Back);
    assert_eq!(
        weave.weapon_classes(),
        (WeaponClass::DualWield, WeaponClass::RestorationStaff)
    );
}

// AppModel intents.

fn model_with_beacon_root(root: &std::path::Path) -> AppModel {
    model_with_clock(root, Instant::now())
}

fn model_with_clock(root: &std::path::Path, clock: Instant) -> AppModel {
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
        Arc::new(Mutex::new(eso_weave::potion::AutoPotionController::new(
            eso_weave::potion::AutoPotionConfig::default(),
        ))),
        log,
        settings,
        None,
        clock,
    )
}

#[test]
fn model_uses_the_injected_clock_not_its_own() {
    // The model must stamp fishing time on the shared clock it is given (the same
    // origin the pixel-bus worker evaluates against), not on a clock it creates
    // itself. A model built with an origin 500 ms in the past reports at least
    // 500 ms elapsed immediately.
    let dir = tempfile::tempdir().unwrap();
    let past = Instant::now() - Duration::from_millis(500);
    let model = model_with_clock(dir.path(), past);
    assert!(
        model.now_ms() >= 500,
        "now_ms {} should reflect the injected origin",
        model.now_ms()
    );
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
fn update_beacon_intent_reinstalls() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());

    model.apply_intent(UiIntent::InstallBeacon);
    assert_eq!(
        model.view().beacon_condition,
        BeaconCondition::InstalledCurrent
    );

    // Update removes and reinstalls in one step, leaving the addon installed and
    // current (the managed-marker uninstall gate is reused unchanged).
    model.apply_intent(UiIntent::UpdateBeacon);
    assert_eq!(
        model.view().beacon_condition,
        BeaconCondition::InstalledCurrent
    );
    assert!(model.view().uninstall_enabled);
}

#[test]
fn log_filter_and_settings_level_stay_linked() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());

    // Changing the live-log dropdown also changes the settings Log level.
    model.apply_intent(UiIntent::SetLogFilter(LevelName::Debug));
    assert_eq!(model.view().log_filter, LevelName::Debug);
    assert_eq!(model.settings_form().logging.level, LevelName::Debug);

    // Applying a settings Log level updates the live-log dropdown.
    let mut form = model.settings_form();
    form.logging.level = LevelName::Warn;
    model.apply_intent(UiIntent::ApplySettings(Box::new(form)));
    assert_eq!(model.view().log_filter, LevelName::Warn);

    // Hiding or showing the live-log panel does not change the verbosity.
    model.apply_intent(UiIntent::ToggleLogPanel(true));
    model.apply_intent(UiIntent::ToggleLogPanel(false));
    assert_eq!(model.settings_form().logging.level, LevelName::Warn);
}

#[test]
fn modal_extent_fits_small_grows_and_caps() {
    // A small window: the modal fits inside it (about max_frac of the window).
    let small = modal_extent(480.0, 440.0, 1000.0, 0.92);
    assert!(
        small <= 480.0 * 0.92 + 0.5,
        "small modal must fit the window"
    );

    // A mid window (below the pixel cap): more pixels than the small window, but a
    // smaller fraction of the window.
    let mid = modal_extent(1200.0, 440.0, 1000.0, 0.92);
    assert!(mid > small, "modal grows in pixels with the window");
    assert!(mid < 1000.0, "mid window is below the pixel cap");
    assert!(
        mid / 1200.0 < small / 480.0,
        "modal occupies a smaller fraction as the window grows"
    );

    // A very large window: capped at max_px.
    let large = modal_extent(6000.0, 440.0, 1000.0, 0.92);
    assert_eq!(large, 1000.0, "modal is capped at max_px");
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

// Slice 031: the combat readout.

#[test]
fn combat_view_shows_both_states_and_unknown() {
    let in_combat = combat_view(CombatSignal::InCombat);
    assert!(in_combat.detected);
    assert_eq!(in_combat.state, "In combat");
    assert_eq!(in_combat.role, StatusRole::Active);

    let out = combat_view(CombatSignal::OutOfCombat);
    assert!(out.detected);
    assert_eq!(out.state, "Out of combat");
    assert_eq!(out.role, StatusRole::Active);

    // The unavailable case renders exactly as the weapon-bar readout renders its
    // own, so the two adjacent fields read with one convention.
    let unknown = combat_view(CombatSignal::Unknown);
    assert!(!unknown.detected);
    assert_eq!(unknown.state, "Not detected");
    assert_eq!(unknown.role, StatusRole::Muted);
}

#[test]
fn routing_a_combat_event_stores_it_without_touching_fishing() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    fishing.set_enabled(true, 0, &mut sink);
    assert_eq!(fishing.state(), FishingState::Armed);

    route_reader_event(
        PixelBusEvent::Combat(CombatSignal::InCombat),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );
    assert_eq!(weave.combat(), CombatSignal::InCombat);
    assert_eq!(
        fishing.state(),
        FishingState::Armed,
        "combat state does not touch fishing"
    );
}

// Slice 032: the menu-gate readout and routing.

#[test]
fn menu_view_names_each_surface_and_marks_gating() {
    let gameplay = menu_view(MenuSurface::None);
    assert!(!gameplay.gating);
    assert_eq!(gameplay.state, "Gameplay");
    assert_eq!(gameplay.role, StatusRole::Muted);

    let map = menu_view(MenuSurface::Map);
    assert!(map.gating);
    assert_eq!(map.state, "Map");
    assert_eq!(map.role, StatusRole::Active);

    let chat = menu_view(MenuSurface::ChatEntry);
    assert!(chat.gating);
    assert_eq!(chat.state, "Chat entry");

    // The unenumerated surface still reads as gating, so the operator can see why
    // the application stopped intercepting even when it cannot name the screen.
    let other = menu_view(MenuSurface::Other);
    assert!(other.gating);
    assert_eq!(other.state, "Menu");
    assert_eq!(other.role, StatusRole::Active);
}

#[test]
fn routing_a_menu_event_gates_both_synthesis_paths() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    assert!(!input.is_menu_gated(), "the default must be ungated");

    route_reader_event(
        PixelBusEvent::MenuGate(MenuSurface::Mail),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );
    assert!(input.is_menu_gated());
    assert_eq!(weave.menu(), MenuSurface::Mail);

    // Returning to gameplay releases it.
    route_reader_event(
        PixelBusEvent::MenuGate(MenuSurface::None),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        2,
        &mut sink,
    );
    assert!(!input.is_menu_gated());
    assert_eq!(weave.menu(), MenuSurface::None);
}

// Slice 033: the resource readouts and routing.

#[test]
fn resource_view_renders_a_percentage_or_not_detected() {
    let full = resource_view(ResourceLevel::Percent(100));
    assert!(full.detected);
    assert_eq!(full.text, "100%");
    assert_eq!(full.role, StatusRole::Active);

    // Zero is a real reading, not an absent one. It must render as a number and
    // stay in the detected role, or an empty pool would look like a missing addon.
    let empty = resource_view(ResourceLevel::Percent(0));
    assert!(empty.detected);
    assert_eq!(empty.text, "0%");
    assert_eq!(empty.role, StatusRole::Active);

    let unknown = resource_view(ResourceLevel::Unknown);
    assert!(!unknown.detected);
    assert_eq!(unknown.text, "Not detected");
    assert_eq!(unknown.role, StatusRole::Muted);
}

#[test]
fn routing_a_resource_event_stores_it_without_touching_fishing() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    fishing.set_enabled(true, 0, &mut sink);
    assert_eq!(fishing.state(), FishingState::Armed);

    let set = ResourceSet {
        health: ResourceLevel::Percent(42),
        stamina: ResourceLevel::Percent(7),
        magicka: ResourceLevel::Unknown,
    };
    route_reader_event(
        PixelBusEvent::Resources(set),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );
    assert_eq!(weave.resources(), set);
    assert_eq!(
        fishing.state(),
        FishingState::Armed,
        "resources do not touch fishing"
    );
    assert!(
        !input.is_menu_gated(),
        "resources do not touch the input gate"
    );
}

// Slice 039: the auto-potion gates reach the controller by the routing path.

#[test]
fn a_menu_gate_event_gates_the_potion_controller_directly() {
    // FR-009 and the slice 032 lesson. The controller synthesizes on its own
    // timers and never passes through interception, so gating the input engine
    // alone would leave it firing into a chat message being composed.
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    route_reader_event(
        PixelBusEvent::MenuGate(MenuSurface::Inventory),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );
    assert!(input.is_menu_gated(), "the input engine should be gated");

    // The controller's own gate is what matters here, and it is only observable
    // through the rule: with the gate set, an otherwise eligible tick is blocked
    // as Gated rather than firing.
    potion.set_enabled(true);
    let mut potion_sink = eso_weave::potion::MockAutoPotionSink::new();
    let readings = eso_weave::potion::PotionReadings {
        resources: ResourceSet {
            health: ResourceLevel::Percent(0),
            stamina: ResourceLevel::Percent(0),
            magicka: ResourceLevel::Percent(0),
        },
        quickslot: QuickslotState {
            cooldown: SlotCooldown::Ready,
            item_id: Some(1),
        },
    };
    assert_eq!(
        potion.tick(readings, 1000, &mut potion_sink),
        Err(eso_weave::potion::Block::Gated)
    );
    assert!(potion_sink.ops.is_empty());
}

#[test]
fn a_signal_lost_event_switches_auto_potion_off() {
    // FR-011: without readings there is nothing trustworthy to act on, so it
    // switches off rather than evaluating against stale values.
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    potion.set_enabled(true);
    assert!(potion.enabled());

    route_reader_event(
        PixelBusEvent::SignalLost,
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );
    assert!(
        !potion.enabled(),
        "auto-potion must switch off when the beacon signal is lost"
    );
}

// Slice 038: the quickslot readout.

#[test]
fn quickslot_view_shows_the_cooldown_and_the_identity_as_a_number() {
    let view = quickslot_view(QuickslotState {
        cooldown: SlotCooldown::Ready,
        item_id: Some(64_500),
    });
    assert_eq!(view.cooldown.text, "Ready");
    assert_eq!(view.cooldown.role, StatusRole::Active);
    // The number itself, not a name: the application has no way to resolve one.
    assert_eq!(view.identity.text, "64500");
    assert_eq!(view.identity.role, StatusRole::Active);

    let counting = quickslot_view(QuickslotState {
        cooldown: SlotCooldown::RemainingMs(4500),
        item_id: Some(1),
    });
    assert_eq!(counting.cooldown.text, "4.5s");
    assert_eq!(counting.cooldown.role, StatusRole::Warning);
}

#[test]
fn quickslot_view_is_muted_when_there_is_nothing_to_show() {
    let view = quickslot_view(QuickslotState::new_unknown());
    assert_eq!(view.cooldown.text, "-");
    assert_eq!(view.cooldown.role, StatusRole::Muted);
    assert_eq!(view.identity.text, "-");
    assert_eq!(view.identity.role, StatusRole::Muted);
}

#[test]
fn quickslot_view_halves_degrade_independently() {
    // FR-012: the state where the cooldown read and the identity did not is
    // reachable whenever one identity block is disturbed. Collapsing the whole
    // readout there would throw away a value that was read correctly and make a
    // one-block disturbance look identical to a missing addon.
    let view = quickslot_view(QuickslotState {
        cooldown: SlotCooldown::RemainingMs(2000),
        item_id: None,
    });
    assert_eq!(view.cooldown.text, "2.0s");
    assert_eq!(view.cooldown.role, StatusRole::Warning);
    assert_eq!(view.identity.text, "-");
    assert_eq!(view.identity.role, StatusRole::Muted);
}

#[test]
fn quickslot_events_reach_the_engine_and_nothing_else() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = FishingController::new(FishingConfig::default());
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    fishing.set_enabled(true, 0, &mut sink);
    assert_eq!(fishing.state(), FishingState::Armed);

    let state = QuickslotState {
        cooldown: SlotCooldown::Ready,
        item_id: Some(0x12_3456),
    };
    route_reader_event(
        PixelBusEvent::Quickslot(state),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );
    assert_eq!(weave.quickslot(), state);
    assert_eq!(
        fishing.state(),
        FishingState::Armed,
        "the quickslot does not touch fishing"
    );
    assert!(
        !input.is_menu_gated(),
        "the quickslot does not touch the input gate"
    );
}
