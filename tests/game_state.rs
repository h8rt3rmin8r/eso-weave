use std::path::{Path, PathBuf};

use eso_weave::game::{
    epic_candidate, reconcile, steam, BeaconFreshness, CandidateSource, FocusObservation,
    GameContext, GameObservations, GameRuntime, GameState, InstallationCandidate,
    InstallationProvider, InstallationState, Presence, ProcessObservation, SurfaceObservation,
};
use eso_weave::pixelbus::MenuSurface;

fn candidate(provider: InstallationProvider, root: impl Into<PathBuf>) -> InstallationCandidate {
    InstallationCandidate {
        provider,
        root: root.into(),
        source: if provider == InstallationProvider::EsoStore {
            CandidateSource::GenericUninstall
        } else {
            CandidateSource::SteamManifest
        },
    }
}

#[test]
fn stronger_provider_wins_over_generic_entry_for_the_same_root() {
    let root = PathBuf::from("C:/Games/Zenimax Online");
    let state = reconcile(
        vec![
            candidate(InstallationProvider::EsoStore, &root),
            candidate(InstallationProvider::Steam, &root),
        ],
        false,
    );
    assert!(matches!(
        state,
        InstallationState::Detected(InstallationCandidate {
            provider: InstallationProvider::Steam,
            ..
        })
    ));
}

#[test]
fn distinct_roots_and_conflicting_strong_providers_are_ambiguous() {
    assert_eq!(
        reconcile(
            vec![
                candidate(InstallationProvider::Steam, "C:/Games/ESO"),
                candidate(InstallationProvider::Steam, "D:/Games/ESO"),
            ],
            false,
        ),
        InstallationState::Ambiguous
    );
    assert_eq!(
        reconcile(
            vec![
                candidate(InstallationProvider::Steam, "C:/Games/ESO"),
                candidate(InstallationProvider::Epic, "C:/Games/ESO"),
            ],
            false,
        ),
        InstallationState::Ambiguous
    );
}

#[test]
fn empty_reconciliation_distinguishes_a_clean_negative_from_source_failure() {
    assert_eq!(reconcile(Vec::new(), false), InstallationState::NotDetected);
    assert_eq!(reconcile(Vec::new(), true), InstallationState::Unknown);
}

#[test]
fn runtime_reduction_honors_game_precedence_over_launcher() {
    let cases = [
        (Presence::Present, Presence::Present, GameRuntime::Active),
        (Presence::Present, Presence::Absent, GameRuntime::Active),
        (Presence::Present, Presence::Unknown, GameRuntime::Active),
        (Presence::Unknown, Presence::Present, GameRuntime::Unknown),
        (
            Presence::Absent,
            Presence::Present,
            GameRuntime::LauncherOpen,
        ),
        (Presence::Absent, Presence::Absent, GameRuntime::Inactive),
        (Presence::Absent, Presence::Unknown, GameRuntime::Unknown),
    ];
    for (game, launcher, expected) in cases {
        assert_eq!(
            ProcessObservation {
                game,
                launcher,
                focus: FocusObservation::Unknown
            }
            .runtime(),
            expected
        );
    }
}

fn observations(
    runtime: GameRuntime,
    focus: FocusObservation,
    freshness: BeaconFreshness,
    surface: SurfaceObservation,
) -> GameObservations {
    GameObservations {
        installation: InstallationState::NotDetected,
        runtime,
        focus,
        freshness,
        surface,
    }
}

#[test]
fn gameplay_requires_every_authoritative_axis() {
    let valid = observations(
        GameRuntime::Active,
        FocusObservation::Focused,
        BeaconFreshness::Fresh,
        SurfaceObservation::Observed(MenuSurface::None),
    );
    assert_eq!(valid.context(), GameContext::Gameplay);
    assert_eq!(
        observations(
            GameRuntime::Inactive,
            valid.focus,
            valid.freshness,
            valid.surface
        )
        .context(),
        GameContext::NotDetected
    );
    assert_eq!(
        observations(
            valid.runtime,
            FocusObservation::Unfocused,
            valid.freshness,
            valid.surface
        )
        .context(),
        GameContext::Unfocused
    );
    assert_eq!(
        observations(
            valid.runtime,
            valid.focus,
            BeaconFreshness::Lost,
            valid.surface
        )
        .context(),
        GameContext::SignalUnavailable
    );
    assert_eq!(
        observations(
            valid.runtime,
            valid.focus,
            valid.freshness,
            SurfaceObservation::Unavailable
        )
        .context(),
        GameContext::SignalUnavailable
    );
}

#[test]
fn named_surface_and_unknown_axes_project_truthfully() {
    assert_eq!(
        observations(
            GameRuntime::Active,
            FocusObservation::Focused,
            BeaconFreshness::Fresh,
            SurfaceObservation::Observed(MenuSurface::Inventory),
        )
        .context(),
        GameContext::Surface(MenuSurface::Inventory)
    );
    assert_eq!(
        observations(
            GameRuntime::Unknown,
            FocusObservation::Focused,
            BeaconFreshness::Fresh,
            SurfaceObservation::Observed(MenuSurface::None),
        )
        .context(),
        GameContext::Unknown
    );
}

#[test]
fn leaving_active_clears_focus_freshness_and_surface() {
    let state = GameState::default();
    state.update_processes(ProcessObservation {
        game: Presence::Present,
        launcher: Presence::Present,
        focus: FocusObservation::Focused,
    });
    state.observe_heartbeat();
    state.observe_surface(SurfaceObservation::Observed(MenuSurface::None));
    state.update_processes(ProcessObservation {
        game: Presence::Absent,
        launcher: Presence::Absent,
        focus: FocusObservation::Unfocused,
    });
    let snapshot = state.snapshot();
    assert_eq!(snapshot.runtime, GameRuntime::Inactive);
    assert_eq!(snapshot.focus, FocusObservation::Unknown);
    assert_eq!(snapshot.freshness, BeaconFreshness::NeverObserved);
    assert_eq!(snapshot.surface, SurfaceObservation::Unavailable);
}

#[test]
fn repeated_observations_are_change_detected() {
    let state = GameState::default();
    let processes = ProcessObservation {
        game: Presence::Absent,
        launcher: Presence::Present,
        focus: FocusObservation::Unknown,
    };
    assert!(state.update_processes(processes));
    assert!(!state.update_processes(processes));
    assert!(state.update_installation(InstallationState::NotDetected));
    assert!(!state.update_installation(InstallationState::NotDetected));
}

#[test]
fn steam_manifest_parser_reads_install_dir() {
    let text = r#""AppState" { "appid" "306130" "installdir" "Zenimax Online" }"#;
    assert_eq!(
        steam::install_dir_from_manifest(text),
        Some(PathBuf::from("Zenimax Online"))
    );
}

fn create_game_root(root: &Path) {
    let client = root.join("The Elder Scrolls Online/game/client");
    let launcher = root.join("Launcher");
    std::fs::create_dir_all(&client).unwrap();
    std::fs::create_dir_all(&launcher).unwrap();
    std::fs::write(client.join("eso64.exe"), []).unwrap();
    std::fs::write(launcher.join("Bethesda.net_Launcher.exe"), []).unwrap();
}

#[test]
fn epic_manifest_requires_identity_and_validated_artifacts() {
    let temp = tempfile::tempdir().unwrap();
    create_game_root(temp.path());
    let value = serde_json::json!({
        "DisplayName": "The Elder Scrolls Online",
        "AppName": "some-platform-catalog-id",
        "InstallLocation": temp.path(),
    });
    let candidate = epic_candidate(&value).expect("validated ESO manifest");
    assert_eq!(candidate.provider, InstallationProvider::Epic);

    let missing = tempfile::tempdir().unwrap();
    let invalid = serde_json::json!({
        "DisplayName": "The Elder Scrolls Online",
        "InstallLocation": missing.path(),
    });
    assert!(epic_candidate(&invalid).is_none());
}
