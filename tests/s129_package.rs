use eso_weave::beacon::{self, BeaconStatus, Environment, RunningState};
use std::fs;

#[test]
fn managed_legacy_data_migrates_without_touching_saved_variables_or_neighbors() {
    let sandbox = tempfile::tempdir().unwrap();
    let addons = sandbox.path().join("live/AddOns");
    fs::create_dir_all(&addons).unwrap();
    let legacy = addons.join("EsoWeaveData");
    fs::create_dir(&legacy).unwrap();
    fs::write(
        legacy.join("EsoWeaveData.txt"),
        "## Version: 2\n## X-ESO-Weave-Data-Managed: true\n",
    )
    .unwrap();
    for file in ["EsoWeaveData.lua", "Catalog.lua", "Encounter.lua"] {
        fs::write(legacy.join(file), "-- legacy bytes\n").unwrap();
    }
    let saved = sandbox.path().join("live/SavedVariables/EsoWeaveData.lua");
    fs::create_dir_all(saved.parent().unwrap()).unwrap();
    fs::write(
        &saved,
        "EsoWeaveDataSaved = { catalog = {}, encounter = { sentinel = 1 } }\n",
    )
    .unwrap();
    let before = fs::read(&saved).unwrap();
    let neighbor = addons.join("Neighbor");
    fs::create_dir(&neighbor).unwrap();
    fs::write(neighbor.join("keep"), b"untouched").unwrap();
    beacon::install_configured(
        &addons,
        RunningState::Running,
        101051,
        5,
        true,
        Environment::Live,
    )
    .unwrap();
    assert!(!legacy.exists());
    assert_eq!(beacon::status(&addons), BeaconStatus::ManagedUpToDate);
    let presence = beacon::inspect_presence(&addons).unwrap();
    assert!(presence.eligible());
    assert!(presence.startup_config.unwrap().catalog_disabled);
    assert_eq!(fs::read(&saved).unwrap(), before);
    assert_eq!(fs::read(neighbor.join("keep")).unwrap(), b"untouched");
    assert!(!addons.join("PixelBeacon/Encounter.lua").exists());
}

#[test]
fn manual_combined_presence_does_not_grant_mutation_ownership() {
    let sandbox = tempfile::tempdir().unwrap();
    beacon::install_configured(
        sandbox.path(),
        RunningState::NotRunning,
        101051,
        3,
        false,
        Environment::Live,
    )
    .unwrap();
    let manifest = sandbox.path().join("PixelBeacon/PixelBeacon.txt");
    fs::write(
        &manifest,
        fs::read_to_string(&manifest)
            .unwrap()
            .replace(beacon::MANAGED_MARKER, ""),
    )
    .unwrap();
    assert!(beacon::inspect_presence(sandbox.path()).unwrap().eligible());
    assert_eq!(beacon::status(sandbox.path()), BeaconStatus::Unmanaged);
    assert!(beacon::uninstall(sandbox.path(), RunningState::NotRunning).is_err());
    let other_files = [
        beacon::MANIFEST_FILE,
        beacon::LUA_FILE,
        beacon::BOOTSTRAP_FILE,
        beacon::CATALOG_FILE,
    ];
    let before = other_files
        .map(|name| fs::read(sandbox.path().join(beacon::SUBFOLDER).join(name)).unwrap());
    let changed = beacon::write_startup_config(sandbox.path(), true, Environment::Live).unwrap();
    assert_eq!(changed.status, BeaconStatus::Unmanaged);
    assert!(
        beacon::inspect_presence(sandbox.path())
            .unwrap()
            .startup_config
            .unwrap()
            .catalog_disabled
    );
    let after = other_files
        .map(|name| fs::read(sandbox.path().join(beacon::SUBFOLDER).join(name)).unwrap());
    assert_eq!(before, after);
}

#[test]
fn startup_preferences_and_block_size_survive_idempotent_deployments_and_pts_isolation() {
    let sandbox = tempfile::tempdir().unwrap();
    let live = sandbox.path().join("live/AddOns");
    let pts = sandbox.path().join("pts/AddOns");
    fs::create_dir_all(&live).unwrap();
    fs::create_dir_all(&pts).unwrap();
    beacon::install_configured(
        &live,
        RunningState::Unknown,
        101051,
        5,
        true,
        Environment::Live,
    )
    .unwrap();
    beacon::install_configured(
        &pts,
        RunningState::NotRunning,
        101051,
        4,
        false,
        Environment::Pts,
    )
    .unwrap();
    let pts_before = fs::read(pts.join("PixelBeacon/StartupConfig.lua")).unwrap();
    let outcome = beacon::install_sized(&live, RunningState::Running, 101099, 5).unwrap();
    assert!(!outcome.reload_required);
    assert!(
        beacon::inspect_presence(&live)
            .unwrap()
            .startup_config
            .unwrap()
            .catalog_disabled
    );
    beacon::write_startup_config(&live, false, Environment::Live).unwrap();
    let lua = fs::read_to_string(live.join("PixelBeacon/PixelBeacon.lua")).unwrap();
    assert_eq!(beacon::parse_lua_constant(&lua, "BLOCK_PX"), Some(5));
    assert_eq!(
        fs::read(pts.join("PixelBeacon/StartupConfig.lua")).unwrap(),
        pts_before
    );
    assert!(!beacon::MANIFEST.contains("Encounter.lua"));
}

#[test]
fn automatic_migration_covers_neither_pixel_data_both_current_and_conflicts() {
    for (pixel, data) in [(false, false), (true, false), (false, true), (true, true)] {
        let sandbox = tempfile::tempdir().unwrap();
        if pixel {
            let directory = sandbox.path().join("PixelBeacon");
            fs::create_dir(&directory).unwrap();
            fs::write(
                directory.join("PixelBeacon.txt"),
                "## Version: 23\n## X-ESO-Weave-Managed: true\n",
            )
            .unwrap();
            fs::write(directory.join("PixelBeacon.lua"), "local BLOCK_PX = 5\n").unwrap();
        }
        if data {
            let directory = sandbox.path().join("EsoWeaveData");
            fs::create_dir(&directory).unwrap();
            fs::write(
                directory.join("EsoWeaveData.txt"),
                "## Version: 2\n## X-ESO-Weave-Data-Managed: true\n",
            )
            .unwrap();
            for file in ["EsoWeaveData.lua", "Catalog.lua", "Encounter.lua"] {
                fs::write(directory.join(file), "-- legacy\n").unwrap();
            }
        }
        assert_eq!(
            beacon::migrate_legacy_if_needed(sandbox.path(), true, Environment::Live).unwrap(),
            pixel || data
        );
        if pixel || data {
            assert!(beacon::inspect_presence(sandbox.path()).unwrap().eligible());
            assert!(!sandbox.path().join("EsoWeaveData").exists());
            assert!(
                !beacon::migrate_legacy_if_needed(sandbox.path(), true, Environment::Live).unwrap()
            );
            if pixel {
                assert_eq!(beacon::installed_block_px(sandbox.path()), Some(5));
            }
        } else {
            assert!(!sandbox.path().join("PixelBeacon").exists());
        }
    }
    let sandbox = tempfile::tempdir().unwrap();
    let directory = sandbox.path().join("PixelBeacon");
    fs::create_dir(&directory).unwrap();
    fs::write(
        directory.join("PixelBeacon.txt"),
        "## X-ESO-Weave-Managed: true\n",
    )
    .unwrap();
    fs::write(directory.join("user-notes"), "preserve unknown").unwrap();
    assert!(!beacon::migrate_legacy_if_needed(sandbox.path(), false, Environment::Live).unwrap());
    assert_eq!(
        fs::read_to_string(directory.join("user-notes")).unwrap(),
        "preserve unknown"
    );
}

#[test]
fn legacy_partial_unknown_api_and_corrupt_capabilities_cannot_claim_combined_presence() {
    let sandbox = tempfile::tempdir().unwrap();
    beacon::install(sandbox.path(), RunningState::NotRunning, 101051).unwrap();
    let directory = sandbox.path().join("PixelBeacon");
    let catalog = fs::read(directory.join("Catalog.lua")).unwrap();
    fs::write(
        directory.join("Catalog.lua"),
        "local ADDON_NAME = \"PixelBeacon\"\nlocal MODULE_NAMESPACE = ADDON_NAME .. \"Catalog\"\n",
    )
    .unwrap();
    assert!(!beacon::inspect_presence(sandbox.path()).unwrap().eligible());
    fs::write(directory.join("Catalog.lua"), b"corrupt").unwrap();
    assert!(!beacon::inspect_presence(sandbox.path()).unwrap().eligible());
    fs::write(directory.join("Catalog.lua"), catalog).unwrap();
    let manifest = fs::read_to_string(directory.join("PixelBeacon.txt")).unwrap();
    fs::write(
        directory.join("PixelBeacon.txt"),
        manifest.replace("## APIVersion: 101051 101050", "## APIVersion: 101099"),
    )
    .unwrap();
    assert!(!beacon::inspect_presence(sandbox.path()).unwrap().eligible());
    fs::write(directory.join("PixelBeacon.txt"), &manifest).unwrap();
    fs::remove_file(directory.join("StartupConfig.lua")).unwrap();
    assert!(!beacon::inspect_presence(sandbox.path()).unwrap().eligible());
    assert_eq!(
        beacon::status(sandbox.path()),
        BeaconStatus::ManagedVersionMismatch
    );
    assert!(beacon::migrate_legacy_if_needed(sandbox.path(), false, Environment::Live).unwrap());
    assert!(beacon::inspect_presence(sandbox.path()).unwrap().eligible());
}
