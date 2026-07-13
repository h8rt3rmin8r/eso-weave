//! Lifecycle and discovery tests for the Beacon Manager.
//!
//! The safety-critical surfaces (the marker-gated uninstall and the write and
//! delete confinement to the resolved `PixelBeacon` subtree) are exercised here
//! against a temporary AddOns root, per constitution Principle II.

use std::fs;
use std::path::Path;

use eso_weave::beacon::{
    self, addons_dir_under_documents, embedded_version, eso_addons_subpath, has_managed_marker,
    parse_api_version_primary, parse_manifest_version, prefs_from_value, prefs_to_value,
    reload_reminder, render_manifest, rewrite_api_version, steam, BeaconPrefs, BeaconStatus,
    DiscoveryError, Environment, LifecycleError, RunningState, DEFAULT_API_VERSION, LUA_FILE,
    MANAGED_MARKER, MANIFEST, MANIFEST_FILE, SUBFOLDER,
};

fn tmp() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

fn beacon_dir(root: &Path) -> std::path::PathBuf {
    root.join(SUBFOLDER)
}

/// Writes a `PixelBeacon` folder with the given manifest text (and a Lua stub).
fn write_beacon(root: &Path, manifest: &str) {
    let dir = beacon_dir(root);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(MANIFEST_FILE), manifest).unwrap();
    fs::write(dir.join(LUA_FILE), "-- stub").unwrap();
}

// T003: manifest parsing and the pure rules.

#[test]
fn managed_marker_is_detected_exactly() {
    assert!(has_managed_marker(MANIFEST));
    assert!(has_managed_marker(
        "## Title: X\n## X-ESO-Weave-Managed: true\n"
    ));
    assert!(has_managed_marker("  ## X-ESO-Weave-Managed: true  "));
    assert!(!has_managed_marker("## X-ESO-Weave-Managed: false\n"));
    assert!(!has_managed_marker("## Title: X\n"));
}

#[test]
fn version_is_parsed_or_none() {
    assert_eq!(parse_manifest_version("## Version: 7\n"), Some(7));
    assert_eq!(parse_manifest_version("## Version:   42  \n"), Some(42));
    assert_eq!(parse_manifest_version("## Title: X\n"), None);
    assert_eq!(parse_manifest_version("## Version: notanumber\n"), None);
}

#[test]
fn embedded_manifest_is_managed_and_versioned() {
    assert!(has_managed_marker(MANIFEST));
    assert_eq!(parse_manifest_version(MANIFEST), Some(embedded_version()));
    assert!(MANIFEST.contains(MANAGED_MARKER));
}

#[test]
fn embedded_manifest_version_is_four() {
    // Bumped so the app classifies an existing on-disk version-3 install as
    // outdated and refreshes it, delivering the rewritten fishing detection.
    assert_eq!(embedded_version(), 4);
    assert_eq!(parse_manifest_version(MANIFEST), Some(4));
}

#[test]
fn embedded_manifest_declares_current_api_version() {
    // The addon must not be flagged out of date by the live client. The manifest
    // uses the supported multi-value form; confirm it declares a value at least
    // the current live game API version, and keeps the managed marker so safe
    // uninstall still verifies it.
    const LIVE_API_VERSION: u32 = 101050;
    let line = MANIFEST
        .lines()
        .find_map(|l| l.trim().strip_prefix("## APIVersion:"))
        .expect("manifest declares an APIVersion line");
    let versions: Vec<u32> = line
        .split_whitespace()
        .filter_map(|token| token.parse().ok())
        .collect();
    assert!(
        !versions.is_empty(),
        "APIVersion declares at least one value"
    );
    assert!(
        versions.iter().any(|&v| v >= LIVE_API_VERSION),
        "APIVersion {versions:?} declares at least the live value {LIVE_API_VERSION}"
    );
    assert!(has_managed_marker(MANIFEST));
}

#[test]
fn reload_reminder_rule() {
    assert!(reload_reminder(RunningState::Running));
    assert!(reload_reminder(RunningState::Unknown));
    assert!(!reload_reminder(RunningState::NotRunning));
}

// T005: four-state classification.

#[test]
fn status_not_installed_when_absent_or_no_manifest() {
    let root = tmp();
    assert_eq!(beacon::status(root.path()), BeaconStatus::NotInstalled);

    // Folder exists but has no manifest.
    fs::create_dir_all(beacon_dir(root.path())).unwrap();
    assert_eq!(beacon::status(root.path()), BeaconStatus::NotInstalled);
}

#[test]
fn status_managed_up_to_date() {
    let root = tmp();
    write_beacon(root.path(), MANIFEST);
    assert_eq!(beacon::status(root.path()), BeaconStatus::ManagedUpToDate);
}

#[test]
fn status_version_mismatch_when_marker_but_different_version() {
    let root = tmp();
    let older = format!("## Version: 0\n{MANAGED_MARKER}\n");
    write_beacon(root.path(), &older);
    assert_eq!(
        beacon::status(root.path()),
        BeaconStatus::ManagedVersionMismatch
    );

    // Marker present but no readable version -> still managed (mismatch), never unmanaged.
    let no_version = format!("## Title: PixelBeacon\n{MANAGED_MARKER}\n");
    write_beacon(root.path(), &no_version);
    assert_eq!(
        beacon::status(root.path()),
        BeaconStatus::ManagedVersionMismatch
    );
}

#[test]
fn status_unmanaged_when_marker_absent() {
    let root = tmp();
    write_beacon(root.path(), "## Title: PixelBeacon\n## Version: 1\n");
    assert_eq!(beacon::status(root.path()), BeaconStatus::Unmanaged);
}

// T007: install, over-install, missing dir, write confinement.

#[test]
fn install_writes_embedded_files_and_reports_up_to_date() {
    let root = tmp();
    let outcome =
        beacon::install(root.path(), RunningState::NotRunning, DEFAULT_API_VERSION).unwrap();
    assert_eq!(outcome.status, BeaconStatus::ManagedUpToDate);
    assert!(!outcome.reload_required);

    let dir = beacon_dir(root.path());
    assert_eq!(
        fs::read_to_string(dir.join(MANIFEST_FILE)).unwrap(),
        MANIFEST
    );
    assert_eq!(fs::read_to_string(dir.join(LUA_FILE)).unwrap(), beacon::LUA);
    assert_eq!(beacon::status(root.path()), BeaconStatus::ManagedUpToDate);
}

#[test]
fn install_over_older_version_updates_in_place() {
    let root = tmp();
    write_beacon(root.path(), &format!("## Version: 0\n{MANAGED_MARKER}\n"));
    assert_eq!(
        beacon::status(root.path()),
        BeaconStatus::ManagedVersionMismatch
    );

    beacon::install(root.path(), RunningState::NotRunning, DEFAULT_API_VERSION).unwrap();
    assert_eq!(beacon::status(root.path()), BeaconStatus::ManagedUpToDate);
    assert_eq!(
        fs::read_to_string(beacon_dir(root.path()).join(MANIFEST_FILE)).unwrap(),
        MANIFEST
    );
}

#[test]
fn install_fails_when_addons_dir_missing() {
    let root = tmp();
    let missing = root.path().join("does-not-exist");
    let err = beacon::install(&missing, RunningState::NotRunning, DEFAULT_API_VERSION).unwrap_err();
    assert!(matches!(err, LifecycleError::AddonsDirMissing));
    assert!(!missing.exists());
}

#[test]
fn install_writes_only_under_pixelbeacon() {
    let root = tmp();
    // A sibling addon that must never be touched.
    let sentinel = root.path().join("OtherAddon");
    fs::create_dir_all(&sentinel).unwrap();
    fs::write(sentinel.join("keep.txt"), "keep me").unwrap();

    beacon::install(root.path(), RunningState::NotRunning, DEFAULT_API_VERSION).unwrap();

    assert_eq!(
        fs::read_to_string(sentinel.join("keep.txt")).unwrap(),
        "keep me"
    );
    // Only PixelBeacon and OtherAddon exist at the root.
    let mut names: Vec<String> = fs::read_dir(root.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(names, vec!["OtherAddon".to_string(), SUBFOLDER.to_string()]);
}

// T009: marker-gated uninstall (safety-critical) and delete confinement.

#[test]
fn uninstall_removes_managed_folder() {
    let root = tmp();
    beacon::install(root.path(), RunningState::NotRunning, DEFAULT_API_VERSION).unwrap();
    assert_eq!(beacon::status(root.path()), BeaconStatus::ManagedUpToDate);

    let outcome = beacon::uninstall(root.path(), RunningState::NotRunning).unwrap();
    assert_eq!(outcome.status, BeaconStatus::NotInstalled);
    assert!(!beacon_dir(root.path()).exists());
    assert_eq!(beacon::status(root.path()), BeaconStatus::NotInstalled);
}

#[test]
fn uninstall_refuses_unmanaged_folder() {
    let root = tmp();
    write_beacon(root.path(), "## Title: PixelBeacon\n## Version: 1\n");

    let err = beacon::uninstall(root.path(), RunningState::NotRunning).unwrap_err();
    assert!(matches!(err, LifecycleError::Unmanaged));
    // Folder and its files survive untouched.
    assert!(beacon_dir(root.path()).join(MANIFEST_FILE).exists());
    assert!(beacon_dir(root.path()).join(LUA_FILE).exists());
}

#[test]
fn uninstall_refuses_folder_without_manifest() {
    let root = tmp();
    let dir = beacon_dir(root.path());
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("stray.lua"), "-- not a manifest").unwrap();

    let err = beacon::uninstall(root.path(), RunningState::NotRunning).unwrap_err();
    assert!(matches!(err, LifecycleError::Unmanaged));
    assert!(dir.join("stray.lua").exists());
}

#[test]
fn uninstall_never_touches_siblings() {
    let root = tmp();
    let sentinel = root.path().join("OtherAddon");
    fs::create_dir_all(&sentinel).unwrap();
    fs::write(sentinel.join("keep.txt"), "keep me").unwrap();
    beacon::install(root.path(), RunningState::NotRunning, DEFAULT_API_VERSION).unwrap();

    beacon::uninstall(root.path(), RunningState::NotRunning).unwrap();

    assert!(!beacon_dir(root.path()).exists());
    assert_eq!(
        fs::read_to_string(sentinel.join("keep.txt")).unwrap(),
        "keep me"
    );
}

// T011: reminder is wired into both operations across running states.

#[test]
fn lifecycle_reload_required_tracks_running_state() {
    for (state, expected) in [
        (RunningState::Running, true),
        (RunningState::Unknown, true),
        (RunningState::NotRunning, false),
    ] {
        let root = tmp();
        let installed = beacon::install(root.path(), state, DEFAULT_API_VERSION).unwrap();
        assert_eq!(installed.reload_required, expected, "install {state:?}");
        let removed = beacon::uninstall(root.path(), state).unwrap();
        assert_eq!(removed.reload_required, expected, "uninstall {state:?}");
    }
}

// T005/T006: APIVersion parsing, the multi-value token rewrite rule, and render.

#[test]
fn parses_primary_api_version_token() {
    assert_eq!(
        parse_api_version_primary("## APIVersion: 101050 101054\n"),
        Some(101050)
    );
    assert_eq!(parse_api_version_primary(MANIFEST), Some(101050));
    assert_eq!(parse_api_version_primary("## Title: X\n"), None);
    assert_eq!(parse_api_version_primary("## APIVersion: nope\n"), None);
}

#[test]
fn rewrite_sets_primary_keeps_greater_drops_lesser() {
    // Greater token 101054 is kept, lesser token 101040 is dropped, primary set.
    let src = "## APIVersion: 101040 101054\n";
    assert_eq!(
        rewrite_api_version(src, 101050),
        "## APIVersion: 101050 101054\n"
    );
    // Advancing past every token collapses to the single primary.
    assert_eq!(rewrite_api_version(src, 101060), "## APIVersion: 101060\n");
}

#[test]
fn rewrite_preserves_every_other_line_and_marker() {
    let updated = rewrite_api_version(MANIFEST, 101070);
    assert!(has_managed_marker(&updated));
    assert_eq!(parse_api_version_primary(&updated), Some(101070));
    // Only the APIVersion line changed; all other lines are byte for byte equal.
    for (before, after) in MANIFEST.lines().zip(updated.lines()) {
        if before.trim_start().starts_with("## APIVersion:") {
            continue;
        }
        assert_eq!(before, after);
    }
    assert_eq!(parse_manifest_version(&updated), Some(embedded_version()));
}

#[test]
fn render_manifest_with_default_matches_embedded() {
    assert_eq!(render_manifest(DEFAULT_API_VERSION), MANIFEST);
}

#[test]
fn install_writes_resolved_api_version() {
    let root = tmp();
    beacon::install(root.path(), RunningState::NotRunning, 101070).unwrap();
    let manifest = fs::read_to_string(beacon_dir(root.path()).join(MANIFEST_FILE)).unwrap();
    assert_eq!(parse_api_version_primary(&manifest), Some(101070));
    assert!(has_managed_marker(&manifest));
    assert_eq!(beacon::status(root.path()), BeaconStatus::ManagedUpToDate);
}

// T012: Steam VDF library extraction.

#[test]
fn vdf_single_library_with_app() {
    let vdf = r#"
    "libraryfolders"
    {
        "0"
        {
            "path"    "/home/u/.steam/steam"
            "apps"
            {
                "306130"    "12345"
                "228980"    "67890"
            }
        }
    }
    "#;
    let paths = steam::library_paths_for_app(vdf, "306130");
    assert_eq!(
        paths,
        vec![std::path::PathBuf::from("/home/u/.steam/steam")]
    );
}

#[test]
fn vdf_multi_library_only_matching_returned() {
    let vdf = r#"
    "libraryfolders"
    {
        "0"
        {
            "path"    "/library/a"
            "apps" { "228980" "1" }
        }
        "1"
        {
            "path"    "/library/b"
            "apps" { "306130" "1" }
        }
    }
    "#;
    let paths = steam::library_paths_for_app(vdf, "306130");
    assert_eq!(paths, vec![std::path::PathBuf::from("/library/b")]);
}

#[test]
fn vdf_app_absent_returns_empty() {
    let vdf = r#"
    "libraryfolders"
    {
        "0" { "path" "/library/a" "apps" { "228980" "1" } }
    }
    "#;
    assert!(steam::library_paths_for_app(vdf, "306130").is_empty());
}

// T014: discovery override precedence and pure path composition.

#[test]
fn override_directory_wins() {
    let over = tmp();
    let prefs = BeaconPrefs {
        path_override: Some(over.path().to_path_buf()),
        environment: Environment::Live,
    };
    assert_eq!(beacon::resolve_addons_dir(&prefs).unwrap(), over.path());
}

#[test]
fn override_missing_directory_is_not_found() {
    let prefs = BeaconPrefs {
        path_override: Some(std::path::PathBuf::from("/no/such/addons/dir")),
        environment: Environment::Live,
    };
    assert_eq!(
        beacon::resolve_addons_dir(&prefs),
        Err(DiscoveryError::NotFound)
    );
}

#[test]
fn addons_subpath_uses_environment_segment() {
    assert_eq!(
        eso_addons_subpath(Environment::Live),
        Path::new("Elder Scrolls Online")
            .join("live")
            .join("AddOns")
    );
    assert_eq!(
        eso_addons_subpath(Environment::Pts),
        Path::new("Elder Scrolls Online").join("pts").join("AddOns")
    );
    let documents = Path::new("/docs");
    assert_eq!(
        addons_dir_under_documents(documents, Environment::Live),
        Path::new("/docs")
            .join("Elder Scrolls Online")
            .join("live")
            .join("AddOns")
    );
}

// T017: beacon settings round-trip through the opaque section.

#[test]
fn prefs_round_trip_and_default_on_null() {
    assert_eq!(
        prefs_from_value(&serde_json::Value::Null),
        BeaconPrefs::default()
    );

    let prefs = BeaconPrefs {
        path_override: Some(std::path::PathBuf::from("/custom/AddOns")),
        environment: Environment::Pts,
    };
    let value = prefs_to_value(&prefs);
    assert_eq!(prefs_from_value(&value), prefs);
}
