//! Tests for session-state persistence and the coalesced save scheduler (US2).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eso_weave::app::{app_toggle_intent, AppModel, SaveScheduler, SkillEdit, UiIntent};
use eso_weave::beacon::{self, BeaconPrefs, Environment};
use eso_weave::config::state::{self, SessionState};
use eso_weave::config::{LoggingPrefs, Settings};
use eso_weave::fishing::{FishingConfig, FishingController, MockFishingSink};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::Action;
use eso_weave::input::InputEngine;
use eso_weave::logging;
use eso_weave::weave::{WeaveConfig, WeaveEngine};

// Session state file.

#[test]
fn session_state_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let state = SessionState {
        schema_version: 1,
        suspended: true,
        fishing: true,
        api_version: Default::default(),
        window: None,
    };
    state::save(dir.path(), &state).unwrap();
    let (loaded, notices) = state::load(dir.path());
    assert_eq!(loaded, state);
    assert!(notices.is_empty());
}

#[test]
fn missing_session_file_yields_defaults_without_notice() {
    let dir = tempfile::tempdir().unwrap();
    let (loaded, notices) = state::load(dir.path());
    assert_eq!(loaded, SessionState::default());
    assert!(!loaded.suspended);
    assert!(!loaded.fishing);
    assert!(notices.is_empty());
}

#[test]
fn invalid_session_file_falls_back_to_defaults_with_notice() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join(state::STATE_FILE_NAME), b"not json").unwrap();
    let (loaded, notices) = state::load(dir.path());
    assert_eq!(loaded, SessionState::default());
    assert!(
        !notices.is_empty(),
        "an invalid file should surface a notice"
    );
}

// Save scheduler predicate.

#[test]
fn scheduler_flushes_only_after_settle() {
    let settle = Duration::from_millis(400);
    let mut s = SaveScheduler::new(settle);
    let t0 = Instant::now();

    assert!(!s.should_flush(t0), "nothing dirty");
    s.mark_config(t0);
    assert!(!s.should_flush(t0), "not settled yet");
    assert!(!s.should_flush(t0 + Duration::from_millis(399)));
    assert!(s.should_flush(t0 + settle));

    let (cfg, sess) = s.take();
    assert!(cfg && !sess);
    assert!(!s.should_flush(t0 + Duration::from_secs(10)), "cleared");
}

#[test]
fn scheduler_coalesces_repeated_changes() {
    let settle = Duration::from_millis(400);
    let mut s = SaveScheduler::new(settle);
    let t0 = Instant::now();

    s.mark_config(t0);
    // A later change resets the settle window, so a drag coalesces to one write.
    s.mark_config(t0 + Duration::from_millis(300));
    assert!(!s.should_flush(t0 + Duration::from_millis(400)));
    assert!(s.should_flush(t0 + Duration::from_millis(700)));
}

// Session restore through the model.

fn model_with_dir(dir: Option<PathBuf>, root: &Path) -> AppModel {
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
        dir,
        std::time::Instant::now(),
    )
}

#[test]
fn restore_suspended_keeps_engine_suspended() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(None, dir.path());
    assert!(!model.view().suspended);

    model.restore_session(SessionState {
        schema_version: 1,
        suspended: true,
        fishing: false,
        api_version: Default::default(),
        window: None,
    });
    // The engine is suspended, so the weave worker produces no input regardless
    // of focus; combined with the backend's focus-scoped synthesis this upholds
    // the "no input while unfocused" invariant on restore.
    assert!(model.view().suspended);
    assert_eq!(model.view().app_state.indicator, "Suspended");
}

#[test]
fn restore_fishing_marks_active_and_round_trips() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(None, dir.path());
    model.restore_session(SessionState {
        schema_version: 1,
        suspended: false,
        fishing: true,
        api_version: Default::default(),
        window: None,
    });
    assert!(model.view().fishing_active);
    // The persisted intent is a single on/off flag.
    let state = model.current_session_state();
    assert!(state.fishing);
    assert!(!state.suspended);
}

#[test]
fn window_geometry_persists_and_restores_through_the_model() {
    use eso_weave::config::state::WindowGeometry;

    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(Some(dir.path().to_path_buf()), dir.path());

    let geo = WindowGeometry {
        x: 120,
        y: 64,
        width: 820,
        height: 900,
        maximized: false,
    };
    model.apply_intent(UiIntent::SetWindowGeometry(geo));
    // The geometry change marks the session dirty and persists on settle, but is a
    // layout-only change so it does not raise the save confirmation (issue #6).
    let out = model.maybe_flush(Instant::now() + Duration::from_millis(500));
    assert!(out.wrote, "geometry still persists");
    assert!(!out.notify, "a window move/resize does not confirm");

    let (loaded, _) = state::load(dir.path());
    assert_eq!(loaded.window, Some(geo));

    // A fresh model restores the geometry and round-trips it back out.
    let mut restored = model_with_dir(Some(dir.path().to_path_buf()), dir.path());
    restored.restore_session(loaded);
    assert_eq!(restored.current_session_state().window, Some(geo));
}

#[test]
fn flush_session_now_writes_immediately_without_settle() {
    use eso_weave::config::state::WindowGeometry;

    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(Some(dir.path().to_path_buf()), dir.path());

    let geo = WindowGeometry {
        x: 5,
        y: 5,
        width: 640,
        height: 480,
        maximized: true,
    };
    model.apply_intent(UiIntent::SetWindowGeometry(geo));
    // Nothing has settled, but a forced flush (used on window close) writes now.
    assert!(!model.maybe_flush(Instant::now()).wrote);
    assert!(model.flush_session_now());
    let (loaded, _) = state::load(dir.path());
    assert_eq!(loaded.window, Some(geo));
}

// Hotkey-driven toggles reach the same state as the GUI buttons (feature 015).

/// Applies a hotkey action exactly as the GUI drain loop does: map it to an
/// intent against the live fishing state, then apply it through the model.
fn press(model: &mut AppModel, action: Action) {
    if let Some(intent) = app_toggle_intent(action, model.fishing_on(), model.auto_potion_on()) {
        model.apply_intent(intent);
    }
}

#[test]
fn hotkey_suspend_toggles_like_the_button() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(Some(dir.path().to_path_buf()), dir.path());
    assert!(!model.view().suspended);

    press(&mut model, Action::ToggleSuspend);
    assert!(model.view().suspended, "first press suspends");
    assert_eq!(model.view().app_state.indicator, "Suspended");

    press(&mut model, Action::ToggleSuspend);
    assert!(!model.view().suspended, "second press resumes");

    // The suspend flip marks the session dirty and persists on settle, exactly
    // like the button path.
    press(&mut model, Action::ToggleSuspend);
    let out = model.maybe_flush(Instant::now() + Duration::from_millis(500));
    assert!(out.wrote, "hotkey suspend persists to session state");
    assert!(out.notify, "toggling suspend is a meaningful change");
    let (loaded, _) = state::load(dir.path());
    assert!(loaded.suspended, "hotkey suspend persists to session state");
}

#[test]
fn hotkey_fishing_toggles_like_the_button() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(Some(dir.path().to_path_buf()), dir.path());
    assert!(!model.view().fishing_active);

    press(&mut model, Action::ToggleFishing);
    assert!(model.view().fishing_active, "first press enables fishing");

    press(&mut model, Action::ToggleFishing);
    assert!(
        !model.view().fishing_active,
        "second press disables fishing"
    );
}

#[test]
fn skill_edit_persists_config_after_settle() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(Some(dir.path().to_path_buf()), dir.path());

    model.apply_intent(UiIntent::EditSkill(1, SkillEdit::Active(false)));
    // Nothing has settled yet.
    assert!(!model.maybe_flush(Instant::now()).wrote);
    // After the settle interval, exactly one write occurs and it is a meaningful
    // settings change (raises the save confirmation).
    let out = model.maybe_flush(Instant::now() + Duration::from_millis(500));
    assert!(out.wrote, "a settled config change should flush");
    assert!(out.notify, "a skill edit is a meaningful change");
    assert!(dir.path().join("config.json").exists());
}

// Save-confirmation gating: layout writes persist silently, real changes confirm
// (feature 027 / issue #6).

#[test]
fn scheduler_notifies_only_for_meaningful_changes() {
    let settle = Duration::from_millis(400);
    let t0 = Instant::now();

    // A layout-only change is dirty and flushes, but does not notify.
    let mut s = SaveScheduler::new(settle);
    s.mark_session_layout(t0);
    assert!(s.should_flush(t0 + settle));
    assert!(!s.pending_notify(), "a layout-only change does not confirm");
    let (_cfg, sess) = s.take();
    assert!(sess, "a layout change still persists");
    assert!(!s.pending_notify(), "take clears the notify flag");

    // A meaningful change notifies.
    let mut s = SaveScheduler::new(settle);
    s.mark_config(t0);
    assert!(s.pending_notify(), "a meaningful change confirms");

    // A mixed batch notifies, because a real settings change occurred in it.
    let mut s = SaveScheduler::new(settle);
    s.mark_config_layout(t0);
    s.mark_session(t0);
    assert!(s.pending_notify(), "a mixed batch confirms");
}

#[test]
fn log_height_change_persists_but_does_not_confirm() {
    let dir = tempfile::tempdir().unwrap();
    let mut model = model_with_dir(Some(dir.path().to_path_buf()), dir.path());

    model.apply_intent(UiIntent::SetLogHeight(240));
    let out = model.maybe_flush(Instant::now() + Duration::from_millis(500));
    assert!(out.wrote, "the log height still persists");
    assert!(!out.notify, "resizing the log pane does not confirm");
    assert!(dir.path().join("config.json").exists());
}
