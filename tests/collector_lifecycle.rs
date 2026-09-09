use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use eso_weave::collector::lifecycle::{
    install, saved_variables_path, status, uninstall, CollectorStatus, RunningState,
    COLLECTOR_SUBFOLDER, MANAGED_MARKER, MANIFEST_FILE,
};

struct Sandbox(PathBuf);

static NEXT_SANDBOX: AtomicU64 = AtomicU64::new(1);

impl Sandbox {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "eso-weave-collector-lifecycle-{}-{}",
            std::process::id(),
            NEXT_SANDBOX.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn addons(&self) -> &Path {
        &self.0
    }

    fn collector(&self) -> PathBuf {
        self.0.join(COLLECTOR_SUBFOLDER)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn install_update_and_remove_are_marker_gated_and_separate() {
    let sandbox = Sandbox::new();
    let pixelbeacon = sandbox.addons().join("PixelBeacon");
    fs::create_dir(&pixelbeacon).unwrap();
    fs::write(pixelbeacon.join("PixelBeacon.txt"), b"pixel-manifest\n").unwrap();
    fs::write(pixelbeacon.join("PixelBeacon.lua"), b"pixel-lua\n").unwrap();
    let pixel_manifest = fs::read(pixelbeacon.join("PixelBeacon.txt")).unwrap();
    let pixel_lua = fs::read(pixelbeacon.join("PixelBeacon.lua")).unwrap();

    assert_eq!(status(sandbox.addons()), CollectorStatus::NotInstalled);
    let installed = install(sandbox.addons(), RunningState::Running, 101050).unwrap();
    assert_eq!(installed.status, CollectorStatus::ManagedUpToDate);
    assert!(installed.reload_required);
    assert_eq!(status(sandbox.addons()), CollectorStatus::ManagedUpToDate);
    let manifest = fs::read_to_string(sandbox.collector().join(MANIFEST_FILE)).unwrap();
    assert!(manifest.lines().any(|line| line.trim() == MANAGED_MARKER));

    let updated = install(sandbox.addons(), RunningState::NotRunning, 101051).unwrap();
    assert!(!updated.reload_required);
    assert!(fs::read_to_string(sandbox.collector().join(MANIFEST_FILE))
        .unwrap()
        .contains("## APIVersion: 101051 101050"));

    let removed = uninstall(sandbox.addons(), RunningState::Unknown).unwrap();
    assert!(removed.reload_required);
    assert!(!sandbox.collector().exists());
    assert_eq!(
        fs::read(pixelbeacon.join("PixelBeacon.txt")).unwrap(),
        pixel_manifest
    );
    assert_eq!(
        fs::read(pixelbeacon.join("PixelBeacon.lua")).unwrap(),
        pixel_lua
    );
}

#[test]
fn unmanaged_targets_are_never_modified_or_removed() {
    let sandbox = Sandbox::new();
    fs::create_dir(sandbox.collector()).unwrap();
    let foreign = sandbox.collector().join(MANIFEST_FILE);
    fs::write(&foreign, b"## Title: Foreign\n").unwrap();

    assert_eq!(status(sandbox.addons()), CollectorStatus::Unmanaged);
    assert!(install(sandbox.addons(), RunningState::NotRunning, 101050).is_err());
    assert!(uninstall(sandbox.addons(), RunningState::NotRunning).is_err());
    assert_eq!(fs::read(&foreign).unwrap(), b"## Title: Foreign\n");
}

#[test]
fn managed_targets_with_foreign_entries_are_never_removed() {
    let sandbox = Sandbox::new();
    install(sandbox.addons(), RunningState::NotRunning, 101050).unwrap();
    let foreign = sandbox.collector().join("user-note.txt");
    fs::write(&foreign, b"keep me\n").unwrap();

    assert_eq!(status(sandbox.addons()), CollectorStatus::Unmanaged);
    assert!(uninstall(sandbox.addons(), RunningState::NotRunning).is_err());
    assert_eq!(fs::read(foreign).unwrap(), b"keep me\n");
}

#[test]
fn proven_managed_target_can_repair_a_missing_lua_file() {
    let sandbox = Sandbox::new();
    install(sandbox.addons(), RunningState::NotRunning, 101050).unwrap();
    fs::remove_file(
        sandbox
            .collector()
            .join(eso_weave::collector::lifecycle::LUA_FILE),
    )
    .unwrap();

    assert_eq!(
        status(sandbox.addons()),
        CollectorStatus::ManagedVersionMismatch
    );
    install(sandbox.addons(), RunningState::NotRunning, 101050).unwrap();
    assert_eq!(status(sandbox.addons()), CollectorStatus::ManagedUpToDate);
}

#[test]
fn non_directory_roots_and_non_directory_targets_are_refused() {
    let sandbox = Sandbox::new();
    let missing = sandbox.addons().join("missing");
    assert!(install(&missing, RunningState::NotRunning, 101050).is_err());

    fs::write(sandbox.collector(), b"foreign-file").unwrap();
    assert_eq!(status(sandbox.addons()), CollectorStatus::Unmanaged);
    assert!(install(sandbox.addons(), RunningState::NotRunning, 101050).is_err());
    assert_eq!(fs::read(sandbox.collector()).unwrap(), b"foreign-file");
}

#[test]
fn saved_variables_path_is_derived_from_any_environment_root() {
    for environment in ["live", "liveeu", "pts"] {
        let addons = Path::new("Documents")
            .join("Elder Scrolls Online")
            .join(environment)
            .join("AddOns");
        assert_eq!(
            saved_variables_path(&addons).unwrap(),
            addons
                .parent()
                .unwrap()
                .join("SavedVariables/EsoWeaveCollector.lua")
        );
    }
    assert!(saved_variables_path(Path::new("/")).is_none());
}

#[test]
fn linked_collector_target_is_refused_when_supported() {
    let sandbox = Sandbox::new();
    let elsewhere = sandbox.addons().join("elsewhere");
    fs::create_dir(&elsewhere).unwrap();
    if create_dir_symlink(&elsewhere, &sandbox.collector()).is_err() {
        return;
    }
    assert_eq!(status(sandbox.addons()), CollectorStatus::Unmanaged);
    assert!(install(sandbox.addons(), RunningState::NotRunning, 101050).is_err());
    assert!(uninstall(sandbox.addons(), RunningState::NotRunning).is_err());
    assert!(elsewhere.exists());
}

#[cfg(unix)]
fn create_dir_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_dir_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
