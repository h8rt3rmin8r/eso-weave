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
    });
    assert!(model.view().fishing_active);
    // The persisted intent is a single on/off flag.
    let state = model.current_session_state();
    assert!(state.fishing);
    assert!(!state.suspended);
}

// Hotkey-driven toggles reach the same state as the GUI buttons (feature 015).

/// Applies a hotkey action exactly as the GUI drain loop does: map it to an
/// intent against the live fishing state, then apply it through the model.
fn press(model: &mut AppModel, action: Action) {
    if let Some(intent) = app_toggle_intent(action, model.fishing_on()) {
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
    assert!(model.maybe_flush(Instant::now() + Duration::from_millis(500)));
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
    assert!(!model.maybe_flush(Instant::now()));
    // After the settle interval, exactly one write occurs.
    let saved = model.maybe_flush(Instant::now() + Duration::from_millis(500));
    assert!(saved, "a settled config change should flush");
    assert!(dir.path().join("config.json").exists());
}
