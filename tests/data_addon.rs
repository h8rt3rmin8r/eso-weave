use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use eso_weave::data_addon::{
    install, saved_variables_path, status, uninstall, DataAddonStatus, RunningState, BOOTSTRAP,
    BOOTSTRAP_FILE, CATALOG, CATALOG_FILE, DATA_ADDON_SUBFOLDER, ENCOUNTER, ENCOUNTER_FILE,
    MANAGED_MARKER, MANIFEST_FILE,
};
use mlua::Lua;

struct Sandbox(PathBuf);

static NEXT_SANDBOX: AtomicU64 = AtomicU64::new(1);

impl Sandbox {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "eso-weave-data-addon-lifecycle-{}-{}",
            std::process::id(),
            NEXT_SANDBOX.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn addons(&self) -> &Path {
        &self.0
    }

    fn data_addon(&self) -> PathBuf {
        self.0.join(DATA_ADDON_SUBFOLDER)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn repository_and_managed_package_have_exact_inventories() {
    let addon_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("addon");
    let mut directories = fs::read_dir(&addon_root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    directories.sort();
    assert_eq!(directories, ["EsoWeaveData", "PixelBeacon"]);

    let sandbox = Sandbox::new();
    install(sandbox.addons(), RunningState::NotRunning, 101051).unwrap();
    let mut files = fs::read_dir(sandbox.data_addon())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    files.sort();
    let mut expected = [BOOTSTRAP_FILE, CATALOG_FILE, ENCOUNTER_FILE, MANIFEST_FILE];
    expected.sort();
    assert_eq!(files, expected);
}

#[test]
fn modules_are_namespaced_dormant_and_clear_only_catalog_state() {
    assert!(CATALOG.contains("MODULE_NAMESPACE = ADDON_NAME .. \"Catalog\""));
    assert!(ENCOUNTER.contains("MODULE_NAMESPACE = ADDON_NAME .. \"Encounter\""));
    assert!(!CATALOG.contains("EVENT_COMBAT_EVENT"));
    assert!(ENCOUNTER.contains("local CAPTURE_EVENTS"));
    assert!(ENCOUNTER.contains("registerCaptureHandlers()"));
    assert!(CATALOG.contains("EsoWeaveDataSaved.catalog = nil"));
    assert!(CATALOG.contains("Encounter data was preserved"));
    assert!(!CATALOG.contains("EsoWeaveDataSaved.encounter = nil"));
}

#[test]
fn both_modules_execute_with_isolated_namespaces_and_clear_state() {
    let lua = Lua::new();
    lua.load(
        r#"
        EVENT_ADD_ON_LOADED = 1
        EVENT_PLAYER_COMBAT_STATE = 2
        EVENT_COMBAT_EVENT = 3
        EVENT_EFFECT_CHANGED = 4
        EVENT_POWER_UPDATE = 5
        EVENT_ACTION_SLOT_ABILITY_USED = 6
        EVENT_ACTIVE_WEAPON_PAIR_CHANGED = 7
        EVENT_PLAYER_DEAD = 8
        EVENT_PLAYER_ALIVE = 9
        EVENT_BOSSES_CHANGED = 10
        EVENT_ACTIVE_QUICKSLOT_CHANGED = 11
        EVENT_PLAYER_DEACTIVATED = 12
        ACTION_RESULT_DAMAGE = 100
        ACTION_RESULT_CRITICAL_DAMAGE = 101
        ACTION_RESULT_DOT_TICK = 102
        ACTION_RESULT_DOT_TICK_CRITICAL = 103
        ACTION_RESULT_BLOCKED_DAMAGE = 104
        ACTION_RESULT_DAMAGE_SHIELDED = 105
        ACTION_RESULT_HEAL = 200
        ACTION_RESULT_CRITICAL_HEAL = 201
        ACTION_RESULT_HOT_TICK = 202
        ACTION_RESULT_HOT_TICK_CRITICAL = 203
        ACTION_RESULT_DIED = 300
        ACTION_RESULT_DIED_XP = 301
        ACTION_RESULT_RESURRECT = 302
        POWERTYPE_HEALTH = 0
        HOTBAR_CATEGORY_PRIMARY = 1
        HOTBAR_CATEGORY_QUICKSLOT_WHEEL = 9
        COMBAT_UNIT_TYPE_PLAYER = 1
        SLASH_COMMANDS = {}
        EVENT_MANAGER = { events = {}, updates = {} }
        function EVENT_MANAGER:RegisterForEvent(namespace, event, callback)
            self.events[event] = self.events[event] or {}
            self.events[event][namespace] = callback
        end
        function EVENT_MANAGER:UnregisterForEvent(namespace, event)
            if self.events[event] then self.events[event][namespace] = nil end
        end
        function EVENT_MANAGER:RegisterForUpdate(namespace, interval, callback)
            self.updates[namespace] = callback
        end
        function EVENT_MANAGER:UnregisterForUpdate(namespace) self.updates[namespace] = nil end
        function __fire(event, ...)
            local callbacks = {}
            for _, callback in pairs(EVENT_MANAGER.events[event] or {}) do
                table.insert(callbacks, callback)
            end
            for _, callback in ipairs(callbacks) do callback(event, ...) end
        end
        function d(_) end
        function GetTimeStamp() return 1 end
        "#,
    )
    .exec()
    .unwrap();
    lua.load(BOOTSTRAP).exec().unwrap();
    lua.load(CATALOG).exec().unwrap();
    lua.load(ENCOUNTER).exec().unwrap();
    lua.load("__fire(EVENT_ADD_ON_LOADED, 'EsoWeaveData')")
        .exec()
        .unwrap();
    lua.load(
        r#"
        assert(EVENT_MANAGER.updates["EsoWeaveDataCatalogUpdate"] == nil)
        assert(EVENT_MANAGER.updates["EsoWeaveDataEncounterSamples"] == nil)
        for event = EVENT_COMBAT_EVENT, EVENT_ACTIVE_QUICKSLOT_CHANGED do
            assert(next(EVENT_MANAGER.events[event] or {}) == nil)
        end
        EsoWeaveDataSaved.catalog = { sentinel = "catalog" }
        EsoWeaveDataSaved.encounter.sentinel = "encounter"
        SLASH_COMMANDS["/ewcollect"]("clear confirm")
        assert(EsoWeaveDataSaved.catalog == nil)
        assert(EsoWeaveDataSaved.encounter.sentinel == "encounter")
        EsoWeaveDataSaved.catalog = { sentinel = "catalog" }
        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        assert(EsoWeaveDataSaved.catalog.sentinel == "catalog")
        assert(EsoWeaveDataSaved.encounter.status == "idle")
        "#,
    )
    .exec()
    .unwrap();
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

    assert_eq!(status(sandbox.addons()), DataAddonStatus::NotInstalled);
    let installed = install(sandbox.addons(), RunningState::Running, 101050).unwrap();
    assert_eq!(installed.status, DataAddonStatus::ManagedUpToDate);
    assert!(installed.reload_required);
    assert_eq!(status(sandbox.addons()), DataAddonStatus::ManagedUpToDate);
    let manifest = fs::read_to_string(sandbox.data_addon().join(MANIFEST_FILE)).unwrap();
    assert!(manifest.lines().any(|line| line.trim() == MANAGED_MARKER));

    let updated = install(sandbox.addons(), RunningState::NotRunning, 101051).unwrap();
    assert!(!updated.reload_required);
    assert!(fs::read_to_string(sandbox.data_addon().join(MANIFEST_FILE))
        .unwrap()
        .contains("## APIVersion: 101051 101050"));

    let removed = uninstall(sandbox.addons(), RunningState::Unknown).unwrap();
    assert!(removed.reload_required);
    assert!(!sandbox.data_addon().exists());
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
    fs::create_dir(sandbox.data_addon()).unwrap();
    let foreign = sandbox.data_addon().join(MANIFEST_FILE);
    fs::write(&foreign, b"## Title: Foreign\n").unwrap();

    assert_eq!(status(sandbox.addons()), DataAddonStatus::Unmanaged);
    assert!(install(sandbox.addons(), RunningState::NotRunning, 101050).is_err());
    assert!(uninstall(sandbox.addons(), RunningState::NotRunning).is_err());
    assert_eq!(fs::read(&foreign).unwrap(), b"## Title: Foreign\n");
}

#[test]
fn managed_targets_with_foreign_entries_are_never_removed() {
    let sandbox = Sandbox::new();
    install(sandbox.addons(), RunningState::NotRunning, 101050).unwrap();
    let foreign = sandbox.data_addon().join("user-note.txt");
    fs::write(&foreign, b"keep me\n").unwrap();

    assert_eq!(status(sandbox.addons()), DataAddonStatus::Unmanaged);
    assert!(uninstall(sandbox.addons(), RunningState::NotRunning).is_err());
    assert_eq!(fs::read(foreign).unwrap(), b"keep me\n");
}

#[test]
fn proven_managed_target_can_repair_a_missing_lua_file() {
    let sandbox = Sandbox::new();
    install(sandbox.addons(), RunningState::NotRunning, 101050).unwrap();
    fs::remove_file(
        sandbox
            .data_addon()
            .join(eso_weave::data_addon::CATALOG_FILE),
    )
    .unwrap();

    assert_eq!(
        status(sandbox.addons()),
        DataAddonStatus::ManagedVersionMismatch
    );
    install(sandbox.addons(), RunningState::NotRunning, 101050).unwrap();
    assert_eq!(status(sandbox.addons()), DataAddonStatus::ManagedUpToDate);
}

#[test]
fn altered_manifest_contract_is_detected_and_repaired() {
    let sandbox = Sandbox::new();
    install(sandbox.addons(), RunningState::NotRunning, 101051).unwrap();
    let manifest_path = sandbox.data_addon().join(MANIFEST_FILE);
    let altered = fs::read_to_string(&manifest_path).unwrap().replace(
        "## SavedVariables: EsoWeaveDataSaved",
        "## SavedVariables: ForeignSaved",
    );
    fs::write(&manifest_path, altered).unwrap();

    assert_eq!(
        status(sandbox.addons()),
        DataAddonStatus::ManagedVersionMismatch
    );
    install(sandbox.addons(), RunningState::NotRunning, 101051).unwrap();
    assert_eq!(status(sandbox.addons()), DataAddonStatus::ManagedUpToDate);
    let repaired = fs::read_to_string(manifest_path).unwrap();
    assert!(repaired.contains("## SavedVariables: EsoWeaveDataSaved"));
    assert!(!repaired.contains("ForeignSaved"));
}

#[test]
fn non_directory_roots_and_non_directory_targets_are_refused() {
    let sandbox = Sandbox::new();
    let missing = sandbox.addons().join("missing");
    assert!(install(&missing, RunningState::NotRunning, 101050).is_err());

    fs::write(sandbox.data_addon(), b"foreign-file").unwrap();
    assert_eq!(status(sandbox.addons()), DataAddonStatus::Unmanaged);
    assert!(install(sandbox.addons(), RunningState::NotRunning, 101050).is_err());
    assert_eq!(fs::read(sandbox.data_addon()).unwrap(), b"foreign-file");
}

#[test]
fn linked_addons_root_and_nonregular_expected_entry_are_refused() {
    let sandbox = Sandbox::new();
    let actual = sandbox.addons().join("actual-addons");
    let linked = sandbox.addons().join("linked-addons");
    fs::create_dir(&actual).unwrap();
    if create_dir_symlink(&actual, &linked).is_ok() {
        assert!(install(&linked, RunningState::NotRunning, 101051).is_err());
        assert!(!actual.join(DATA_ADDON_SUBFOLDER).exists());
    }

    install(sandbox.addons(), RunningState::NotRunning, 101051).unwrap();
    let catalog = sandbox.data_addon().join(CATALOG_FILE);
    fs::remove_file(&catalog).unwrap();
    fs::create_dir(&catalog).unwrap();
    fs::remove_file(sandbox.data_addon().join(ENCOUNTER_FILE)).unwrap();
    assert_eq!(status(sandbox.addons()), DataAddonStatus::Unmanaged);
    assert!(uninstall(sandbox.addons(), RunningState::NotRunning).is_err());
    assert!(catalog.is_dir());
    assert!(sandbox.data_addon().join(MANIFEST_FILE).is_file());
}

#[test]
fn uninstall_repairs_ownership_semantics_when_an_expected_file_is_missing() {
    let sandbox = Sandbox::new();
    install(sandbox.addons(), RunningState::NotRunning, 101051).unwrap();
    fs::remove_file(sandbox.data_addon().join(ENCOUNTER_FILE)).unwrap();
    assert_eq!(
        status(sandbox.addons()),
        DataAddonStatus::ManagedVersionMismatch
    );
    uninstall(sandbox.addons(), RunningState::NotRunning).unwrap();
    assert!(!sandbox.data_addon().exists());
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
                .join("SavedVariables/EsoWeaveData.lua")
        );
    }
    assert!(saved_variables_path(Path::new("/")).is_none());
}

#[test]
fn linked_collector_target_is_refused_when_supported() {
    let sandbox = Sandbox::new();
    let elsewhere = sandbox.addons().join("elsewhere");
    fs::create_dir(&elsewhere).unwrap();
    if create_dir_symlink(&elsewhere, &sandbox.data_addon()).is_err() {
        return;
    }
    assert_eq!(status(sandbox.addons()), DataAddonStatus::Unmanaged);
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
