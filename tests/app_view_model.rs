//! View-model tests for the GUI: derivations, routing, skills, and intents.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eso_weave::app::{
    app_state_label, auto_potion_view, beacon_light, beacon_primary_action, beacon_signal_line,
    combat_view, dashboard_layout, default_delay_for, fishing_label, life_state_view, menu_view,
    modal_extent, override_edit_for, quickslot_view, resource_view, resource_view_with_watch,
    route_reader_event, skill_rows, status_line_app, status_line_beacon, status_line_fishing,
    uninstall_enabled, weapon_bar_view, AppModel, BeaconCondition, BeaconPrimaryAction,
    DashboardLayout, ResourcePresentation, SkillEdit, StatusRole, UiIntent,
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
    ActiveBar, BusLayout, CombatSignal, LayoutState, LifeState, MenuSurface, PixelBusEvent,
    QuickslotClassification, QuickslotNonPotionKind, QuickslotPotionAvailability, QuickslotState,
    QuickslotUnavailableReason, ResourceLevel, ResourceSet, SlotCooldown, WeaponBarSignal,
    WeaponClass,
};
use eso_weave::weave::{LatencyConfig, WeaveConfig, WeaveEngine, WeaveType};

use eso_weave::potion::{
    AutoPotionResource, AutoPotionState, BlockReason, DormantReason, ResourceWatch, TriggerCause,
};

fn active_fishing_controller() -> FishingController {
    let mut controller = FishingController::new(FishingConfig::default());
    let mut sink = MockFishingSink::new();
    controller.set_game_environment(true, true, 0, &mut sink);
    controller.set_life_state(LifeState::Alive);
    controller
}

// Derivations.

#[test]
fn app_state_label_reflects_suspend() {
    assert_eq!(app_state_label(false).indicator, "Running");
    assert_eq!(app_state_label(false).button, "Suspend");
    assert_eq!(app_state_label(true).indicator, "Suspended");
    assert_eq!(app_state_label(true).button, "Resume");
}

#[test]
fn dashboard_breakpoint_is_exact_and_point_based() {
    assert_eq!(dashboard_layout(879.9), DashboardLayout::Narrow);
    assert_eq!(dashboard_layout(880.0), DashboardLayout::Wide);
    assert_eq!(dashboard_layout(f32::NAN), DashboardLayout::Narrow);
}

#[test]
fn life_state_view_names_every_protocol_state() {
    for (state, text, role) in [
        (LifeState::Unknown, "Not detected", StatusRole::Muted),
        (LifeState::Alive, "Alive", StatusRole::Healthy),
        (LifeState::Dead, "Dead", StatusRole::Warning),
        (
            LifeState::Reincarnating,
            "Reincarnating",
            StatusRole::Warning,
        ),
    ] {
        let view = life_state_view(state);
        assert_eq!(view.state, text);
        assert_eq!(view.role, role);
    }
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
    assert_eq!(running.title, "ESO Weave");
    assert_eq!(running.state_text, "Active");
    assert_eq!(running.role, StatusRole::Healthy);

    let suspended = status_line_app(true);
    assert_eq!(suspended.state_text, "Suspended");
    assert_eq!(suspended.role, StatusRole::Warning);
}

#[test]
fn beacon_signal_is_independent_from_addon_installation() {
    use eso_weave::game::{BeaconFreshness, GameRuntime};

    let inactive = beacon_signal_line(GameRuntime::Inactive, BeaconFreshness::NeverObserved);
    assert_eq!(inactive.title, "PixelBeacon signal");
    assert_eq!(inactive.state_text, "Game not active");
    assert_eq!(inactive.role, StatusRole::Muted);

    let waiting = beacon_signal_line(GameRuntime::Active, BeaconFreshness::NeverObserved);
    assert_eq!(waiting.state_text, "Not detected");
    assert_eq!(waiting.role, StatusRole::Warning);

    let fresh = beacon_signal_line(GameRuntime::Active, BeaconFreshness::Fresh);
    assert_eq!(fresh.state_text, "Signal detected");
    assert_eq!(fresh.role, StatusRole::Healthy);

    let lost = beacon_signal_line(GameRuntime::Active, BeaconFreshness::Lost);
    assert_eq!(lost.state_text, "Signal lost");
    assert_eq!(lost.role, StatusRole::Error);
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
        "PixelBeacon installation"
    );
}

#[test]
fn beacon_primary_action_matches_the_current_installation_need() {
    assert_eq!(
        beacon_primary_action(BeaconCondition::NotInstalled),
        Some(BeaconPrimaryAction::Install)
    );
    assert_eq!(
        beacon_primary_action(BeaconCondition::AddonsNotFound),
        Some(BeaconPrimaryAction::Install)
    );
    assert_eq!(
        beacon_primary_action(BeaconCondition::InstalledOutdated),
        Some(BeaconPrimaryAction::Update)
    );
    assert_eq!(
        beacon_primary_action(BeaconCondition::InstalledCurrent),
        None
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
    let mut fishing = active_fishing_controller();
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
fn routing_life_state_updates_every_synthesis_consumer() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = active_fishing_controller();
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    route_reader_event(
        PixelBusEvent::Life(LifeState::Alive),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );
    assert_eq!(weave.life(), LifeState::Alive);
    assert!(!input.is_life_gated());
    assert_eq!(potion.life_state(), LifeState::Alive);

    fishing.set_enabled(true, 2, &mut sink);
    assert_eq!(fishing.state(), FishingState::Armed);
    route_reader_event(
        PixelBusEvent::Life(LifeState::Dead),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        3,
        &mut sink,
    );
    assert_eq!(weave.life(), LifeState::Dead);
    assert!(input.is_life_gated());
    assert_eq!(fishing.state(), FishingState::Disabled);
    assert_eq!(fishing.stop_reason(), Some(StopReason::PlayerUnavailable));
    assert_eq!(potion.life_state(), LifeState::Dead);
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
    let mut fishing = active_fishing_controller();
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
    model_with_clock_and_potion(root, clock).0
}

fn model_with_clock_and_potion(
    root: &std::path::Path,
    clock: Instant,
) -> (
    AppModel,
    Arc<Mutex<eso_weave::potion::AutoPotionController>>,
) {
    let (engine, _rx) = InputEngine::new(BindingTable::default(), 16);
    engine.set_game_active(true);
    engine.set_life_gated(false);
    let mut weave_engine = WeaveEngine::new(WeaveConfig::default());
    weave_engine.set_life(LifeState::Alive);
    let weave = Arc::new(Mutex::new(weave_engine));
    let mut fishing_controller = FishingController::new(FishingConfig::default());
    let mut init_sink = MockFishingSink::new();
    fishing_controller.set_game_environment(true, true, 0, &mut init_sink);
    fishing_controller.set_life_state(LifeState::Alive);
    let fishing = Arc::new(Mutex::new(fishing_controller));
    let (_dispatch, log) = logging::build(&LoggingPrefs::default(), PathBuf::from("."));

    let prefs = BeaconPrefs {
        path_override: Some(root.to_path_buf()),
        environment: Environment::Live,
    };
    let settings = Settings {
        beacon: beacon::prefs_to_value(&prefs),
        ..Settings::default()
    };

    let mut potion_controller = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    potion_controller.set_life_state(LifeState::Alive);
    let potion = Arc::new(Mutex::new(potion_controller));
    let model = AppModel::new(
        Arc::new(engine),
        weave,
        fishing,
        Box::new(MockFishingSink::new()),
        potion.clone(),
        log,
        settings,
        None,
        clock,
    );
    (model, potion)
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
fn model_projects_runtime_context_and_dormant_live_fields_truthfully() {
    use eso_weave::game::{
        FocusObservation, GameRuntime, Presence, ProcessObservation, SurfaceObservation,
    };

    let root = tempfile::tempdir().unwrap();
    let model = model_with_beacon_root(root.path());
    let initial = model.view();
    assert_eq!(initial.runtime_line.state_text, "Unknown");
    assert_eq!(initial.menu.state, "Unknown");
    assert_eq!(initial.combat.state, "Game not active");
    assert_eq!(initial.resources.health.text, "Game not active");
    assert_eq!(
        initial.resources.health.presentation,
        ResourcePresentation::Dormant
    );
    assert_eq!(initial.beacon_signal_line.state_text, "Unknown");
    assert_eq!(initial.weapon_bar.active_bar, "Game not active");
    assert!(initial
        .skills
        .iter()
        .all(|skill| skill.cooldown.text == "Game not active"));

    let game = model.game_state();
    game.update_processes(ProcessObservation {
        game: Presence::Present,
        launcher: Presence::Present,
        focus: FocusObservation::Focused,
    });
    assert_eq!(game.snapshot().runtime, GameRuntime::Active);
    let unavailable = model.view();
    assert_eq!(unavailable.menu.state, "Signal unavailable");
    assert_eq!(
        unavailable.resources.health.presentation,
        ResourcePresentation::Unavailable
    );
    assert_eq!(unavailable.beacon_signal_line.state_text, "Not detected");
    game.observe_heartbeat();
    game.observe_surface(SurfaceObservation::Observed(MenuSurface::None));
    let active = model.view();
    assert_eq!(active.runtime_line.state_text, "Active");
    assert_eq!(active.menu.state, "Gameplay");
    assert_eq!(active.beacon_signal_line.state_text, "Signal detected");
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
fn system_state_disclosure_intent_updates_persisted_ui_preferences() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());
    assert!(model.ui_prefs().system_state_expanded);

    model.apply_intent(UiIntent::SetSystemStateExpanded(false));
    assert!(!model.ui_prefs().system_state_expanded);

    model.apply_intent(UiIntent::SetSystemStateExpanded(true));
    assert!(model.ui_prefs().system_state_expanded);
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
fn set_auto_potion_intent_separates_request_from_effective_state() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());
    let initial = model.view();
    assert!(!initial.auto_potion_requested);
    assert_eq!(initial.auto_potion.text, "Off");

    model.apply_intent(UiIntent::SetAutoPotion(true));
    let requested = model.view();
    assert!(requested.auto_potion_requested);
    assert_eq!(requested.auto_potion.text, "Dormant: game inactive");

    model.apply_intent(UiIntent::SetAutoPotion(false));
    let disabled = model.view();
    assert!(!disabled.auto_potion_requested);
    assert_eq!(disabled.auto_potion.text, "Off");
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
fn applying_settings_refreshes_the_live_auto_potion_controller() {
    let dir = tempfile::tempdir().unwrap();
    let (mut model, potion) = model_with_clock_and_potion(dir.path(), Instant::now());
    let mut form = model.settings_form();
    form.potion.health = eso_weave::potion::ResourceWatch {
        enabled: true,
        threshold: 42,
    };
    form.potion.quickslot_key = eso_weave::input::Key::X;
    form.potion.retry_interval_ms = 2345;

    model.apply_intent(UiIntent::ApplySettings(Box::new(form)));

    let config = *potion.lock().unwrap().config();
    assert!(config.health.enabled);
    assert_eq!(config.health.threshold, 42);
    assert_eq!(config.quickslot_key, eso_weave::input::Key::X);
    assert_eq!(config.retry_interval_ms, 2345);
}

#[test]
fn drafted_block_size_does_not_relabel_the_running_reader_geometry() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_beacon_root(dir.path());
    assert_eq!(model.runtime_block_px(), 16);

    let mut form = model.settings_form();
    form.reader.block_px = 32;
    model.apply_intent(UiIntent::ApplySettings(Box::new(form)));

    assert_eq!(model.settings_form().reader.block_px, 32);
    assert_eq!(model.runtime_block_px(), 16);
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
    let mut fishing = active_fishing_controller();
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

#[test]
fn routing_a_layout_event_stores_it_for_display_only() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = active_fishing_controller();
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);
    let layout = LayoutState::Ready(BusLayout::negotiated(120).unwrap());

    route_reader_event(
        PixelBusEvent::Layout(layout),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        1,
        &mut sink,
    );

    assert_eq!(weave.layout(), layout);
    assert_eq!(fishing.state(), FishingState::Disabled);
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
    assert_eq!(other.state, "Other menu");
    assert_eq!(other.role, StatusRole::Active);
}

#[test]
fn routing_a_menu_event_gates_both_synthesis_paths() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = active_fishing_controller();
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    assert!(!input.is_menu_gated(), "the default must be ungated");

    route_reader_event(
        PixelBusEvent::MenuGate(Some(MenuSurface::Mail)),
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
        PixelBusEvent::MenuGate(Some(MenuSurface::None)),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        2,
        &mut sink,
    );
    assert!(!input.is_menu_gated());
    assert_eq!(weave.menu(), MenuSurface::None);

    // Missing or corrupt surface evidence is not gameplay and must fail closed.
    route_reader_event(
        PixelBusEvent::MenuGate(None),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        3,
        &mut sink,
    );
    assert!(input.is_menu_gated());

    potion.set_game_active(true);
    potion.set_focused(true);
    potion.on_heartbeat();
    potion.set_enabled(true);
    let mut potion_sink = eso_weave::potion::MockAutoPotionSink::new();
    assert_eq!(
        potion.tick(
            eso_weave::potion::PotionReadings {
                resources: ResourceSet::new_unknown(),
                quickslot: QuickslotState::new_unknown(),
            },
            3,
            &mut potion_sink,
        ),
        AutoPotionState::Blocked(BlockReason::GameContext)
    );
}

// Slice 033: the resource readouts and routing.

#[test]
fn resource_view_distinguishes_observed_empty_from_unavailable() {
    let full = resource_view(ResourceLevel::Percent(100));
    assert_eq!(full.presentation, ResourcePresentation::Observed(100));
    assert_eq!(full.percent(), Some(100));
    assert_eq!(full.fraction(), Some(1.0));
    assert_eq!(full.text, "100%");
    assert_eq!(full.role, StatusRole::Active);

    // Zero is a real reading, not an absent one. It must render as a number and
    // stay in the detected role, or an empty pool would look like a missing addon.
    let empty = resource_view(ResourceLevel::Percent(0));
    assert_eq!(empty.presentation, ResourcePresentation::Observed(0));
    assert_eq!(empty.percent(), Some(0));
    assert_eq!(empty.fraction(), Some(0.0));
    assert_eq!(empty.text, "0%");
    assert_eq!(empty.role, StatusRole::Active);

    let unknown = resource_view(ResourceLevel::Unknown);
    assert_eq!(unknown.presentation, ResourcePresentation::Unavailable);
    assert_eq!(unknown.percent(), None);
    assert_eq!(unknown.text, "Signal unavailable");
    assert_eq!(unknown.role, StatusRole::Warning);
}

#[test]
fn resource_low_state_comes_only_from_an_enabled_watch() {
    let enabled = ResourceWatch {
        enabled: true,
        threshold: 35,
    };
    let disabled = ResourceWatch {
        enabled: false,
        threshold: 35,
    };

    let low = resource_view_with_watch(ResourceLevel::Percent(35), enabled);
    assert_eq!(low.presentation, ResourcePresentation::Low(35));
    assert_eq!(low.text, "Low: 35%");
    assert_eq!(low.role, StatusRole::Warning);

    let above = resource_view_with_watch(ResourceLevel::Percent(36), enabled);
    assert_eq!(above.presentation, ResourcePresentation::Observed(36));

    let unwatched = resource_view_with_watch(ResourceLevel::Percent(1), disabled);
    assert_eq!(unwatched.presentation, ResourcePresentation::Observed(1));
}

#[test]
fn resource_boundary_fractions_are_exact() {
    for (percent, fraction) in [(0, 0.0), (1, 0.01), (50, 0.5), (99, 0.99), (100, 1.0)] {
        let view = resource_view(ResourceLevel::Percent(percent));
        assert_eq!(view.percent(), Some(percent));
        assert_eq!(view.fraction(), Some(fraction));
        assert_eq!(view.text, format!("{percent}%"));
    }
}

#[test]
fn routing_a_resource_event_stores_it_without_touching_fishing() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = active_fishing_controller();
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
    let mut fishing = active_fishing_controller();
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    route_reader_event(
        PixelBusEvent::MenuGate(Some(MenuSurface::Inventory)),
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
    potion.set_game_active(true);
    potion.set_focused(true);
    potion.on_heartbeat();
    potion.set_enabled(true);
    let mut potion_sink = eso_weave::potion::MockAutoPotionSink::new();
    let readings = eso_weave::potion::PotionReadings {
        resources: ResourceSet {
            health: ResourceLevel::Percent(0),
            stamina: ResourceLevel::Percent(0),
            magicka: ResourceLevel::Percent(0),
        },
        quickslot: QuickslotState {
            classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
            cooldown: SlotCooldown::Ready,
            item_id: Some(1),
        },
    };
    assert_eq!(
        potion.tick(readings, 1000, &mut potion_sink),
        AutoPotionState::Blocked(BlockReason::GameContext)
    );
    assert!(potion_sink.ops.is_empty());
}

#[test]
fn a_signal_lost_event_blocks_auto_potion_without_clearing_the_request() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = active_fishing_controller();
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    potion.set_game_active(true);
    potion.set_focused(true);
    potion.on_heartbeat();
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
    assert!(potion.enabled());
    assert_eq!(
        potion.state(),
        AutoPotionState::Blocked(BlockReason::BeaconUnavailable)
    );

    route_reader_event(
        PixelBusEvent::Heartbeat,
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        2,
        &mut sink,
    );
    let mut potion_sink = eso_weave::potion::MockAutoPotionSink::new();
    assert_eq!(
        potion.tick(
            eso_weave::potion::PotionReadings {
                resources: ResourceSet::new_unknown(),
                quickslot: QuickslotState::new_unknown(),
            },
            2,
            &mut potion_sink,
        ),
        AutoPotionState::Blocked(BlockReason::GameContext),
        "heartbeat must not substitute for a fresh gameplay-surface observation"
    );

    route_reader_event(
        PixelBusEvent::MenuGate(Some(MenuSurface::None)),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        3,
        &mut sink,
    );
    assert_eq!(
        potion.tick(
            eso_weave::potion::PotionReadings {
                resources: ResourceSet::new_unknown(),
                quickslot: QuickslotState::new_unknown(),
            },
            3,
            &mut potion_sink,
        ),
        AutoPotionState::Blocked(BlockReason::PlayerUnavailable(LifeState::Unknown)),
        "gameplay-surface evidence cannot release the independent life-state gate"
    );

    route_reader_event(
        PixelBusEvent::Life(LifeState::Alive),
        &mut weave,
        &mut fishing,
        &mut potion,
        &input,
        4,
        &mut sink,
    );
    assert_eq!(
        potion.tick(
            eso_weave::potion::PotionReadings {
                resources: ResourceSet::new_unknown(),
                quickslot: QuickslotState::new_unknown(),
            },
            4,
            &mut potion_sink,
        ),
        AutoPotionState::Blocked(BlockReason::NoWatchedResource),
        "fresh gameplay-surface and life-state evidence may release both blockers"
    );
}

#[test]
fn s043_auto_potion_view_names_every_effective_family() {
    let cases = [
        (AutoPotionState::Off, "Off", StatusRole::Muted),
        (
            AutoPotionState::Dormant(DormantReason::GameInactive),
            "Dormant: game inactive",
            StatusRole::Muted,
        ),
        (
            AutoPotionState::Dormant(DormantReason::Unfocused),
            "Dormant: game unfocused",
            StatusRole::Muted,
        ),
        (
            AutoPotionState::Blocked(BlockReason::BeaconUnavailable),
            "Blocked: beacon unavailable",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::Suspended),
            "Blocked: input suspended",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::GameContext),
            "Blocked: game context",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::NoWatchedResource),
            "Blocked: no watched resource",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::ResourcesUnavailable),
            "Blocked: resources unavailable",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::QuickslotUnavailable),
            "Blocked: quickslot unavailable",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::NoPotion),
            "Blocked: no potion selected",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::PotionUnavailable),
            "Blocked: potion unavailable",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::PotionCooldown),
            "Blocked: potion cooldown",
            StatusRole::Warning,
        ),
        (
            AutoPotionState::Blocked(BlockReason::RetryInterval),
            "Blocked: retry interval",
            StatusRole::Warning,
        ),
        (AutoPotionState::Ready, "Ready", StatusRole::Healthy),
    ];
    for (state, expected_text, expected_role) in cases {
        let view = auto_potion_view(state);
        assert_eq!(view.text, expected_text);
        assert_eq!(view.role, expected_role);
    }

    let triggered = auto_potion_view(AutoPotionState::Triggered(TriggerCause {
        resource: AutoPotionResource::Health,
        observed_percent: 20,
        threshold_percent: 35,
    }));
    assert_eq!(triggered.text, "Triggered: Health at 20% (threshold 35%)");
    assert_eq!(triggered.role, StatusRole::Active);
}

// Slice 042: the explicitly classified quickslot readout.

#[test]
fn quickslot_view_shows_potion_availability_and_cooldown_independently() {
    let view = quickslot_view(QuickslotState {
        classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
        cooldown: SlotCooldown::Ready,
        item_id: Some(64_500),
    });
    assert_eq!(view.cooldown.text, "Ready");
    assert_eq!(view.cooldown.role, StatusRole::Active);
    assert_eq!(view.state.text, "Potion");
    assert_eq!(view.availability.text, "Usable");
    assert_eq!(view.availability.role, StatusRole::Active);

    let counting = quickslot_view(QuickslotState {
        classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
        cooldown: SlotCooldown::RemainingMs(4500),
        item_id: Some(1),
    });
    assert_eq!(counting.cooldown.text, "4.5s");
    assert_eq!(counting.cooldown.role, StatusRole::Warning);
}

#[test]
fn quickslot_view_names_unavailable_empty_and_non_potion_states() {
    let view = quickslot_view(QuickslotState::new_unknown());
    assert_eq!(view.state.text, "Not detected");
    assert_eq!(view.availability.text, "Not applicable");

    for (classification, expected) in [
        (QuickslotClassification::Empty, "Empty"),
        (
            QuickslotClassification::NonPotion(QuickslotNonPotionKind::Collectible),
            "Non-potion (Collectible)",
        ),
        (
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::LegacyProtocol),
            "Addon update required",
        ),
        (
            QuickslotClassification::Unavailable(QuickslotUnavailableReason::CorruptProtocol),
            "Unreadable signal",
        ),
    ] {
        let state = QuickslotState {
            classification,
            cooldown: SlotCooldown::Ready,
            item_id: Some(1),
        };
        let view = quickslot_view(state);
        assert_eq!(view.state.text, expected);
        assert_eq!(view.cooldown.text, "Not applicable");
    }
}

#[test]
fn quickslot_view_classification_survives_missing_identity() {
    let view = quickslot_view(QuickslotState {
        classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
        cooldown: SlotCooldown::RemainingMs(2000),
        item_id: None,
    });
    assert_eq!(view.cooldown.text, "2.0s");
    assert_eq!(view.cooldown.role, StatusRole::Warning);
    assert_eq!(view.state.text, "Potion");
    assert_eq!(view.availability.text, "Usable");
}

#[test]
fn quickslot_events_reach_the_engine_and_nothing_else() {
    let mut weave = WeaveEngine::new(WeaveConfig::default());
    let mut fishing = active_fishing_controller();
    let mut potion = eso_weave::potion::AutoPotionController::new(
        eso_weave::potion::AutoPotionConfig::default(),
    );
    let mut sink = MockFishingSink::new();
    let (input, _input_rx) = InputEngine::new(BindingTable::default(), 16);

    fishing.set_enabled(true, 0, &mut sink);
    assert_eq!(fishing.state(), FishingState::Armed);

    let state = QuickslotState {
        classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
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
