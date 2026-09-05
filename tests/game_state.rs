use std::path::{Path, PathBuf};

use eso_weave::game::{
    epic_candidate, reconcile, runtime_probe_delay_ms, steam, steam_candidates_from_roots,
    BeaconFreshness, CandidateSource, FocusObservation, GameContext, GameObservations, GameRuntime,
    GameState, InstallationCandidate, InstallationProvider, InstallationState, Presence,
    ProcessObservation, SurfaceObservation,
};
use eso_weave::pixelbus::{MenuSurface, WorldState};

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
        world: WorldState::Unknown,
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
    state.observe_world(WorldState::Active);
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
    assert_eq!(snapshot.world, WorldState::Unknown);
}

#[test]
fn world_state_is_runtime_only_and_signal_loss_clears_it() {
    let state = GameState::default();
    assert_eq!(state.snapshot().world, WorldState::Unknown);
    state.observe_world(WorldState::Transitioning);
    assert_eq!(state.snapshot().world, WorldState::Transitioning);
    state.observe_world(WorldState::Active);
    assert_eq!(state.snapshot().world, WorldState::Active);
    state.signal_lost();
    assert_eq!(state.snapshot().world, WorldState::Unknown);
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

fn create_steam_catalog(steam_root: &Path, library: &Path, install_dir: &str) {
    std::fs::create_dir_all(steam_root.join("steamapps")).unwrap();
    let vdf_path = library.to_string_lossy().replace('\\', "\\\\");
    let vdf = format!(
        r#""libraryfolders" {{ "0" {{ "path" "{vdf_path}" "apps" {{ "306130" "1" }} }} }}"#
    );
    std::fs::write(steam_root.join("steamapps/libraryfolders.vdf"), vdf).unwrap();
    std::fs::create_dir_all(library.join("steamapps")).unwrap();
    std::fs::write(
        library.join("steamapps/appmanifest_306130.acf"),
        format!(r#""AppState" {{ "appid" "306130" "installdir" "{install_dir}" }}"#),
    )
    .unwrap();
    create_game_root(&library.join("steamapps/common").join(install_dir));
}

#[test]
fn steam_discovery_inspects_every_root_and_retains_ambiguity() {
    let temp = tempfile::tempdir().unwrap();
    let first_root = temp.path().join("native-steam");
    let second_root = temp.path().join("flatpak-steam");
    let first_library = temp.path().join("library-a");
    let second_library = temp.path().join("library-b");
    create_steam_catalog(&first_root, &first_library, "ESO A");
    create_steam_catalog(&second_root, &second_library, "ESO B");

    let (candidates, failed) = steam_candidates_from_roots(
        vec![first_root, second_root],
        InstallationProvider::SteamProton,
    );
    assert!(!failed);
    assert_eq!(candidates.len(), 2, "both Steam roots must be inspected");
    assert_eq!(reconcile(candidates, false), InstallationState::Ambiguous);
}

#[test]
fn runtime_probe_caps_long_reader_intervals_and_is_immediately_due() {
    assert_eq!(runtime_probe_delay_ms(60_000, 10_000, 11_000), 1000);
    assert_eq!(runtime_probe_delay_ms(100, 10_000, 11_000), 100);
    assert_eq!(runtime_probe_delay_ms(60_000, 11_000, 11_000), 0);
    assert_eq!(runtime_probe_delay_ms(60_000, 12_000, 11_000), 0);
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
