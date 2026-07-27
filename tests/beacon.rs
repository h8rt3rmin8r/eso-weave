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
    reload_reminder, render_lua, render_manifest, rewrite_api_version, steam, BeaconPrefs,
    BeaconStatus, DiscoveryError, Environment, LifecycleError, RunningState, DEFAULT_API_VERSION,
    LUA_FILE, MANAGED_MARKER, MANIFEST, MANIFEST_FILE, SUBFOLDER,
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
fn embedded_manifest_version_is_twelve() {
    // Slice 038 adds the four quickslot blocks (B16 to B19), so an operator
    // running version 11 draws none of them and the companion must report the
    // quickslot as unknown rather than guess. The bump is what makes the beacon
    // manager offer them the update.
    assert_eq!(embedded_version(), 12);
    assert_eq!(parse_manifest_version(MANIFEST), Some(12));
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

// Slice 028: block-size Lua templating and the managed-only re-deploy.

#[test]
fn render_lua_default_matches_embedded() {
    // At the default size the rendered Lua is byte for byte the embedded Lua, so
    // an unchanged install has no behavior change.
    assert_eq!(render_lua(16), beacon::LUA);
}

#[test]
fn render_lua_rewrites_only_block_px_line() {
    let updated = render_lua(8);
    assert!(updated.lines().any(|l| l.trim() == "local BLOCK_PX = 8"));
    assert!(!updated.lines().any(|l| l.trim() == "local BLOCK_PX = 16"));
    // Every other line is identical, and no line was inserted or removed.
    let mut diffs = 0;
    for (before, after) in beacon::LUA.lines().zip(updated.lines()) {
        if before != after {
            diffs += 1;
            assert!(before.trim_start().starts_with("local BLOCK_PX"));
        }
    }
    assert_eq!(diffs, 1);
    assert_eq!(beacon::LUA.lines().count(), updated.lines().count());
}

#[test]
fn install_sized_writes_block_px_and_keeps_marker() {
    let root = tmp();
    beacon::install_sized(
        root.path(),
        RunningState::NotRunning,
        DEFAULT_API_VERSION,
        8,
    )
    .unwrap();
    let dir = beacon_dir(root.path());
    let lua = fs::read_to_string(dir.join(LUA_FILE)).unwrap();
    assert!(lua.lines().any(|l| l.trim() == "local BLOCK_PX = 8"));
    let manifest = fs::read_to_string(dir.join(MANIFEST_FILE)).unwrap();
    assert!(has_managed_marker(&manifest));
    assert_eq!(beacon::status(root.path()), BeaconStatus::ManagedUpToDate);
}

#[test]
fn redeploy_updates_managed_folder_block_px() {
    let root = tmp();
    beacon::install_sized(
        root.path(),
        RunningState::NotRunning,
        DEFAULT_API_VERSION,
        16,
    )
    .unwrap();

    let outcome = beacon::redeploy_for_block_size(
        root.path(),
        RunningState::NotRunning,
        DEFAULT_API_VERSION,
        4,
    )
    .unwrap();
    assert!(matches!(outcome, beacon::RedeployOutcome::Redeployed(_)));

    let dir = beacon_dir(root.path());
    let lua = fs::read_to_string(dir.join(LUA_FILE)).unwrap();
    assert!(lua.lines().any(|l| l.trim() == "local BLOCK_PX = 4"));
    let manifest = fs::read_to_string(dir.join(MANIFEST_FILE)).unwrap();
    assert!(has_managed_marker(&manifest));
}

#[test]
fn redeploy_refuses_unmanaged_folder() {
    let root = tmp();
    // An unmanaged folder (no marker) with a distinctive Lua stub.
    write_beacon(root.path(), "## Title: PixelBeacon\n## Version: 1\n");

    let outcome = beacon::redeploy_for_block_size(
        root.path(),
        RunningState::NotRunning,
        DEFAULT_API_VERSION,
        8,
    )
    .unwrap();
    assert_eq!(outcome, beacon::RedeployOutcome::SkippedUnmanaged);
    // Nothing was written or removed: the stub Lua and unmanaged status survive.
    assert_eq!(
        fs::read_to_string(beacon_dir(root.path()).join(LUA_FILE)).unwrap(),
        "-- stub"
    );
    assert_eq!(beacon::status(root.path()), BeaconStatus::Unmanaged);
}

#[test]
fn redeploy_skips_when_not_installed() {
    let root = tmp();
    let outcome = beacon::redeploy_for_block_size(
        root.path(),
        RunningState::NotRunning,
        DEFAULT_API_VERSION,
        8,
    )
    .unwrap();
    assert_eq!(outcome, beacon::RedeployOutcome::SkippedNotInstalled);
    assert!(!beacon_dir(root.path()).exists());
}

// Slice 031: the addon and the companion state the pixel-bus contract once each,
// in two different languages. These assert they agree, so a divergence fails the
// build instead of shipping as a silently dead signal.

#[test]
fn parse_lua_constant_reads_decimal_and_hex() {
    let src = "local A = 5\nlocal B = 0x2D\n  local C   =   17  \nlocal D = notanumber\n";
    assert_eq!(beacon::parse_lua_constant(src, "A"), Some(5));
    assert_eq!(beacon::parse_lua_constant(src, "B"), Some(0x2D));
    assert_eq!(beacon::parse_lua_constant(src, "C"), Some(17));
    assert_eq!(beacon::parse_lua_constant(src, "D"), None);
    assert_eq!(beacon::parse_lua_constant(src, "MISSING"), None);
}

#[test]
fn parse_lua_constant_does_not_match_a_longer_name() {
    // `local NUM_BLOCKS_EXTRA` must not satisfy a lookup for `NUM_BLOCKS`.
    let src = "local NUM_BLOCKS_EXTRA = 9\nlocal NUM_BLOCKS = 5\n";
    assert_eq!(beacon::parse_lua_constant(src, "NUM_BLOCKS"), Some(5));
}

#[test]
fn addon_and_companion_agree_on_the_pixel_bus_contract() {
    use eso_weave::pixelbus::NUM_BLOCKS;

    let lua = beacon::LUA;
    assert_eq!(
        beacon::parse_lua_constant(lua, "NUM_BLOCKS"),
        Some(NUM_BLOCKS),
        "the addon and the companion disagree on the block count"
    );
    // Slice 035: the column count the grid wraps at. A disagreement here would
    // not degrade, it would shift every block from row 1 onward and the
    // companion would read valid, checksum-passing colours from the wrong
    // blocks, so the build refuses to ship one.
    assert_eq!(
        beacon::parse_lua_constant(lua, "COLUMNS"),
        Some(eso_weave::pixelbus::COLUMNS),
        "the addon and the companion disagree on the grid column count"
    );
    // The combat block colors, shared byte for byte. The companion keeps these
    // private, so the expected values are the contract in
    // specs/031-combat-state-block/contracts/pixel-bus-b4.md.
    assert_eq!(beacon::parse_lua_constant(lua, "COMBAT_MARKER"), Some(0x2D));
    assert_eq!(beacon::parse_lua_constant(lua, "COMBAT_IN_RED"), Some(0xE0));
    assert_eq!(
        beacon::parse_lua_constant(lua, "COMBAT_OUT_RED"),
        Some(0x20)
    );
    assert_eq!(beacon::parse_lua_constant(lua, "WEAPON_MARKER"), Some(0x5A));
    // Slice 032: the menu block's constants.
    assert_eq!(beacon::parse_lua_constant(lua, "MENU_MARKER"), Some(0xD2));
    assert_eq!(beacon::parse_lua_constant(lua, "MENU_CODE_STEP"), Some(24));
    assert_eq!(beacon::parse_lua_constant(lua, "MENU_CODE_MAX"), Some(10));
    // Slice 033: the three resource markers and the unavailable payload.
    assert_eq!(beacon::parse_lua_constant(lua, "HEALTH_MARKER"), Some(0x16));
    assert_eq!(
        beacon::parse_lua_constant(lua, "STAMINA_MARKER"),
        Some(0x6D)
    );
    assert_eq!(
        beacon::parse_lua_constant(lua, "MAGICKA_MARKER"),
        Some(0xBB)
    );
    assert_eq!(
        beacon::parse_lua_constant(lua, "RESOURCE_UNAVAILABLE"),
        Some(u32::from(eso_weave::pixelbus::RESOURCE_UNAVAILABLE))
    );
    assert_eq!(
        beacon::parse_lua_constant(lua, "RESOURCE_MAX_PERCENT"),
        Some(100)
    );
    // Slice 036: the movement block's constants. Only the two live codes are
    // shared; the reserved sprint codes are companion-only by design, which
    // `addon_defines_no_sprint_constant` below is what enforces.
    assert_eq!(
        beacon::parse_lua_constant(lua, "MOVEMENT_MARKER"),
        Some(0x43)
    );
    assert_eq!(
        beacon::parse_lua_constant(lua, "MOVEMENT_ON_FOOT_RED"),
        Some(0x20)
    );
    assert_eq!(
        beacon::parse_lua_constant(lua, "MOVEMENT_MOUNTED_RED"),
        Some(0x60)
    );
    // Slice 037: the six cooldown marks and the quantization contract. The marks
    // are the midpoints of the six widest gaps left in the green registry; the
    // expected values are the contract in
    // specs/037-cooldown-blocks/contracts/cooldown-blocks.md.
    for (name, expected) in [
        ("COOLDOWN_SKILL1_MARKER", 0x0B),
        ("COOLDOWN_SKILL2_MARKER", 0x21),
        ("COOLDOWN_SKILL3_MARKER", 0x4E),
        ("COOLDOWN_SKILL4_MARKER", 0x92),
        ("COOLDOWN_SKILL5_MARKER", 0xC6),
        ("COOLDOWN_ULTIMATE_MARKER", 0xE8),
    ] {
        assert_eq!(
            beacon::parse_lua_constant(lua, name),
            Some(expected),
            "the addon and the companion disagree on {name}"
        );
    }
    assert_eq!(
        beacon::parse_lua_constant(lua, "COOLDOWN_STEP_MS"),
        Some(50)
    );
    assert_eq!(
        beacon::parse_lua_constant(lua, "COOLDOWN_MAX_STEPS"),
        Some(254)
    );
    assert_eq!(
        beacon::parse_lua_constant(lua, "COOLDOWN_UNAVAILABLE"),
        Some(255)
    );
    // Slice 038: the four quickslot marks. The expected values are the contract
    // in specs/038-quickslot-blocks/contracts/quickslot-blocks.md.
    //
    // The quantization constants are deliberately absent from this block: B16
    // reuses COOLDOWN_STEP_MS, COOLDOWN_MAX_STEPS, and COOLDOWN_UNAVAILABLE under
    // their existing names on both sides, already pinned above. A second name for
    // the same number is how two numbers eventually become different.
    for (name, expected) in [
        ("QUICKSLOT_MARKER", 0x38),
        ("QUICKSLOT_ID_HI_MARKER", 0xB0),
        ("QUICKSLOT_ID_MID_MARKER", 0xDD),
        ("QUICKSLOT_ID_LO_MARKER", 0xF3),
    ] {
        assert_eq!(
            beacon::parse_lua_constant(lua, name),
            Some(expected),
            "the addon and the companion disagree on {name}"
        );
    }
}

/// Slice 038: the manifest advances so the beacon manager offers the update, and
/// says what the new blocks carry.
///
/// The version bump is the entire mechanism by which an operator is offered the
/// new addon. Without it the companion samples four blocks the installed addon
/// does not draw, which is exactly the User Story 2 case: correct, but permanently
/// unknown, with nothing telling the operator why.
#[test]
fn the_manifest_advances_for_the_quickslot_blocks() {
    let manifest = beacon::MANIFEST;
    assert!(
        manifest.contains("## Version: 12"),
        "the manifest version should have advanced to 12"
    );
    assert!(
        manifest.contains("## AddOnVersion: 12"),
        "the addon version should have advanced to 12"
    );
    assert!(
        manifest.contains("quickslot"),
        "the manifest description should name the new signal"
    );
}

/// Slice 036: the reserved sprint codes exist on the companion side only.
///
/// The sprint axis has no observable in the game API (see the verification
/// recorded in `specs/036-movement-state-block/spec.md`), so the addon never
/// emits it and defines no constant for it. Were the addon to define one, the
/// agreement check above would need a special case for a value living on one
/// side of the contract, which is exactly the kind of exception that makes a
/// cross-language check stop being trustworthy. This asserts the absence so the
/// design decision is enforced rather than assumed.
#[test]
fn addon_defines_no_sprint_constant() {
    let lua = beacon::LUA;
    for name in [
        "MOVEMENT_SPRINT_ON_FOOT_RED",
        "MOVEMENT_SPRINT_MOUNTED_RED",
        "SPRINT_MARKER",
    ] {
        assert_eq!(
            beacon::parse_lua_constant(lua, name),
            None,
            "the addon defines {name}, but the sprint axis is companion-side reserved only"
        );
    }
    assert!(
        !lua.contains("IsUnitSprinting") && !lua.contains("EVENT_SPRINT"),
        "the addon references a sprint API that the verification found does not exist"
    );
}

/// Slice 034: the settings file the display detector reads sits beside the
/// AddOns directory, so its path is derived from the resolution that already
/// exists rather than from a second copy of the same path logic.
#[test]
fn user_settings_path_is_the_addons_directory_sibling() {
    let documents = Path::new("C:/Users/someone/Documents");
    let addons = addons_dir_under_documents(documents, Environment::Live);
    let settings = beacon::user_settings_path(&addons).expect("a path with a parent");
    assert_eq!(
        settings,
        documents
            .join("Elder Scrolls Online")
            .join("live")
            .join("UserSettings.txt")
    );
}

#[test]
fn user_settings_path_has_no_answer_for_a_parentless_root() {
    assert_eq!(beacon::user_settings_path(Path::new("/")), None);
}

/// The constitution treats this tree as safety-critical, and detection has no
/// business writing in it. Resolving the path must not create the file, its
/// parent, or anything else.
#[test]
fn user_settings_path_creates_nothing() {
    let dir = tmp();
    let addons = dir
        .path()
        .join("Elder Scrolls Online")
        .join("live")
        .join("AddOns");
    let before = fs::read_dir(dir.path()).unwrap().count();
    let settings = beacon::user_settings_path(&addons).expect("a path with a parent");
    assert!(!settings.exists());
    assert!(!addons.exists());
    assert!(!addons.parent().unwrap().exists());
    let after = fs::read_dir(dir.path()).unwrap().count();
    assert_eq!(before, after);
}
