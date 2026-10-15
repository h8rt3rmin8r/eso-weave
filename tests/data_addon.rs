//! The historical Data API is a compatibility facade, never a second package.
use eso_weave::{beacon, data_addon};
use std::fs;

#[test]
fn legacy_entry_points_share_the_exact_pixelbeacon_inventory() {
    let repository_addons = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("addon");
    let directories = fs::read_dir(&repository_addons)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    assert_eq!(directories, [std::ffi::OsString::from("PixelBeacon")]);
    let mut sources = fs::read_dir(repository_addons.join("PixelBeacon"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    sources.sort();
    let mut intended = beacon::INVENTORY.map(str::to_owned);
    intended.sort();
    assert_eq!(sources, intended);
    let sandbox = tempfile::tempdir().unwrap();
    let outcome =
        data_addon::install(sandbox.path(), data_addon::RunningState::Running, 101099).unwrap();
    assert_eq!(outcome.status, data_addon::DataAddonStatus::ManagedUpToDate);
    assert!(!outcome.reload_required);
    assert!(!sandbox.path().join("EsoWeaveData").exists());
    assert_eq!(data_addon::DATA_ADDON_SUBFOLDER, beacon::SUBFOLDER);
    assert_eq!(data_addon::MANIFEST, beacon::MANIFEST);
    let mut actual = fs::read_dir(sandbox.path().join(beacon::SUBFOLDER))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    actual.sort();
    let mut expected = beacon::INVENTORY.map(str::to_owned);
    expected.sort();
    assert_eq!(actual, expected);
    assert!(beacon::inspect_presence(sandbox.path()).unwrap().eligible());
    data_addon::uninstall(sandbox.path(), data_addon::RunningState::Unknown).unwrap();
    assert_eq!(
        beacon::status(sandbox.path()),
        beacon::BeaconStatus::NotInstalled
    );
}

#[test]
fn facade_deployment_preserves_geometry_and_startup_preference() {
    let sandbox = tempfile::tempdir().unwrap();
    beacon::install_configured(
        sandbox.path(),
        beacon::RunningState::NotRunning,
        101051,
        5,
        true,
        beacon::Environment::Pts,
    )
    .unwrap();
    data_addon::install(sandbox.path(), data_addon::RunningState::NotRunning, 101051).unwrap();
    assert_eq!(beacon::installed_block_px(sandbox.path()), Some(5));
    let config = beacon::inspect_presence(sandbox.path())
        .unwrap()
        .startup_config
        .unwrap();
    assert!(config.catalog_disabled);
    assert_eq!(config.environment, beacon::Environment::Pts);
}

#[test]
fn saved_data_identity_and_reviewed_api_support_remain_compatible() {
    let root = std::path::Path::new("example/live/AddOns");
    assert_eq!(
        data_addon::saved_variables_path(root).unwrap(),
        std::path::Path::new("example/live/SavedVariables/EsoWeaveData.lua")
    );
    assert!(data_addon::supports_api(101051));
    assert!(!data_addon::supports_api(101099));
    assert_eq!(data_addon::DATA_ADDON_VERSION, 1);
    assert_eq!(data_addon::SAVED_VARIABLES_SCHEMA_VERSION, 1);
    assert_eq!(data_addon::embedded_checksum().len(), 64);
}

#[test]
fn facade_cannot_overwrite_or_remove_foreign_files() {
    let sandbox = tempfile::tempdir().unwrap();
    data_addon::install(sandbox.path(), data_addon::RunningState::NotRunning, 101051).unwrap();
    let foreign = sandbox.path().join("PixelBeacon/user-note.txt");
    fs::write(&foreign, "retain").unwrap();
    assert_eq!(
        data_addon::status(sandbox.path()),
        data_addon::DataAddonStatus::Unmanaged
    );
    assert!(
        data_addon::install(sandbox.path(), data_addon::RunningState::NotRunning, 101051).is_err()
    );
    assert!(data_addon::uninstall(sandbox.path(), data_addon::RunningState::NotRunning).is_err());
    assert_eq!(fs::read_to_string(foreign).unwrap(), "retain");
}

#[test]
fn inspection_preserves_real_filesystem_errors() {
    let sandbox = tempfile::tempdir().unwrap();
    let blocking = sandbox.path().join("file");
    fs::write(&blocking, b"x").unwrap();
    assert!(data_addon::inspect(&blocking.join("child")).is_err());
}
