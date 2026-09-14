//! Managed lifecycle for the shared ESO Weave Data addon.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::collector::CollectorError;

pub const DATA_ADDON_SUBFOLDER: &str = "EsoWeaveData";
pub const MANIFEST_FILE: &str = "EsoWeaveData.txt";
pub const BOOTSTRAP_FILE: &str = "EsoWeaveData.lua";
pub const CATALOG_FILE: &str = "Catalog.lua";
pub const ENCOUNTER_FILE: &str = "Encounter.lua";
pub const SAVED_VARIABLES_FILE: &str = "EsoWeaveData.lua";
pub const MAX_SAVED_VARIABLES_BYTES: u64 = 128 * 1024 * 1024;
pub const MAX_SAVED_VARIABLES_TOKENS: usize = 4_000_000;
pub const MAX_SAVED_VARIABLES_ENTRIES: usize = 2_600_000;
pub const SAVED_VARIABLES_SCHEMA_VERSION: u64 = 1;
pub const DATA_ADDON_VERSION: u64 = 1;
pub const MANAGED_MARKER: &str = "## X-ESO-Weave-Data-Managed: true";
const CHECKSUM_PREFIX: &str = "local COLLECTOR_CHECKSUM = \"";

pub const MANIFEST: &str = include_str!("../addon/EsoWeaveData/EsoWeaveData.txt");
pub const BOOTSTRAP: &str = include_str!("../addon/EsoWeaveData/EsoWeaveData.lua");
pub const CATALOG: &str = include_str!("../addon/EsoWeaveData/Catalog.lua");
pub const ENCOUNTER: &str = include_str!("../addon/EsoWeaveData/Encounter.lua");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataAddonStatus {
    NotInstalled,
    ManagedUpToDate,
    ManagedVersionMismatch,
    Unmanaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunningState {
    Running,
    NotRunning,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct LifecycleOutcome {
    pub status: DataAddonStatus,
    pub reload_required: bool,
}

pub fn embedded_checksum() -> String {
    catalog_checksum_from_source(CATALOG)
        .expect("embedded catalog module contains checksum literal")
}

pub fn catalog_checksum_from_source(source: &str) -> Result<String, CollectorError> {
    let start = source
        .find(CHECKSUM_PREFIX)
        .map(|index| index + CHECKSUM_PREFIX.len())
        .ok_or_else(|| CollectorError::Validation("catalog checksum literal is missing".into()))?;
    let end = source[start..]
        .find('"')
        .map(|index| start + index)
        .ok_or_else(|| {
            CollectorError::Validation("catalog checksum literal is unterminated".into())
        })?;
    let literal = &source[start..end];
    if literal.len() != 64 || !literal.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(CollectorError::Validation(
            "catalog checksum literal must contain 64 hexadecimal characters".into(),
        ));
    }
    let mut normalized = source.as_bytes().to_vec();
    normalized[start..end].fill(b'0');
    Ok(sha256_bytes(&normalized))
}

pub fn status(addons_root: &Path) -> DataAddonStatus {
    let Ok(root_metadata) = fs::symlink_metadata(addons_root) else {
        return DataAddonStatus::NotInstalled;
    };
    if metadata_is_link(&root_metadata) || !root_metadata.is_dir() {
        return DataAddonStatus::Unmanaged;
    }
    status_directory(&addons_root.join(DATA_ADDON_SUBFOLDER))
}

fn status_directory(directory: &Path) -> DataAddonStatus {
    let metadata = match fs::symlink_metadata(directory) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return DataAddonStatus::NotInstalled;
        }
        Err(_) => return DataAddonStatus::Unmanaged,
    };
    if metadata_is_link(&metadata) || !metadata.is_dir() {
        return DataAddonStatus::Unmanaged;
    }
    let manifest_path = directory.join(MANIFEST_FILE);
    let Ok(manifest_metadata) = fs::symlink_metadata(&manifest_path) else {
        return DataAddonStatus::Unmanaged;
    };
    if metadata_is_link(&manifest_metadata) || !manifest_metadata.is_file() {
        return DataAddonStatus::Unmanaged;
    }
    let Ok(entries) = fs::read_dir(directory) else {
        return DataAddonStatus::Unmanaged;
    };
    let expected = [MANIFEST_FILE, BOOTSTRAP_FILE, CATALOG_FILE, ENCOUNTER_FILE];
    let mut found = Vec::new();
    for entry in entries {
        let Ok(name) = entry.map(|entry| entry.file_name()) else {
            return DataAddonStatus::Unmanaged;
        };
        if !expected.iter().any(|expected| name == *expected) {
            return DataAddonStatus::Unmanaged;
        }
        found.push(name);
    }
    let Ok(manifest) = fs::read_to_string(manifest_path) else {
        return DataAddonStatus::Unmanaged;
    };
    if !manifest.lines().any(|line| line.trim() == MANAGED_MARKER) {
        return DataAddonStatus::Unmanaged;
    }
    let version_matches =
        parse_addon_version(&manifest) == Some(crate::collector::COLLECTOR_VERSION);
    let manifest_matches = parse_primary_api_version(&manifest)
        .is_some_and(|api_version| manifest == render_manifest(api_version));
    let mut files_match = true;
    for (name, expected) in [
        (BOOTSTRAP_FILE, BOOTSTRAP.as_bytes()),
        (CATALOG_FILE, CATALOG.as_bytes()),
        (ENCOUNTER_FILE, ENCOUNTER.as_bytes()),
    ] {
        let path = directory.join(name);
        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata_is_link(&metadata) || !metadata.is_file() => {
                return DataAddonStatus::Unmanaged;
            }
            Ok(_) => files_match &= fs::read(path).ok().as_deref() == Some(expected),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => files_match = false,
            Err(_) => return DataAddonStatus::Unmanaged,
        }
    }
    if found.len() != expected.len() {
        files_match = false;
    }
    if version_matches && manifest_matches && files_match {
        DataAddonStatus::ManagedUpToDate
    } else {
        DataAddonStatus::ManagedVersionMismatch
    }
}

pub fn install(
    addons_root: &Path,
    running: RunningState,
    api_version: u32,
) -> Result<LifecycleOutcome, CollectorError> {
    let root_metadata = fs::symlink_metadata(addons_root)?;
    if metadata_is_link(&root_metadata) || !root_metadata.is_dir() {
        return rejected("resolved AddOns root does not exist");
    }
    let directory = addons_root.join(DATA_ADDON_SUBFOLDER);
    let existing = status(addons_root);
    match existing {
        DataAddonStatus::NotInstalled => {
            install_fresh_with(addons_root, &directory, api_version, || {})?;
            return Ok(LifecycleOutcome {
                status: DataAddonStatus::ManagedUpToDate,
                reload_required: reload_reminder(running),
            });
        }
        DataAddonStatus::ManagedUpToDate | DataAddonStatus::ManagedVersionMismatch => {}
        DataAddonStatus::Unmanaged => {
            return rejected("refusing to modify an unmanaged data addon")
        }
    }
    let snapshot = snapshot_managed_package(&directory)?;
    update_existing_with(
        addons_root,
        &directory,
        api_version,
        &snapshot,
        || {},
        |_, _| {},
        rename_directory_no_replace,
    )?;
    Ok(LifecycleOutcome {
        status: DataAddonStatus::ManagedUpToDate,
        reload_required: reload_reminder(running),
    })
}

fn update_existing_with(
    addons_root: &Path,
    directory: &Path,
    api_version: u32,
    expected: &ManagedPackageSnapshot,
    before_move: impl FnOnce(),
    after_move: impl FnOnce(&Path, &Path),
    commit: impl FnOnce(&Path, &Path) -> std::io::Result<()>,
) -> Result<(), CollectorError> {
    let transaction = tempfile::Builder::new()
        .prefix(".eso-weave-data-update-")
        .tempdir_in(addons_root)?;
    let replacement = transaction.path().join("replacement");
    write_package(&replacement, api_version)?;
    let previous = transaction.path().join("previous");

    before_move();
    rename_directory_no_replace(directory, &previous)?;
    after_move(&previous, directory);
    if !matches!(
        snapshot_managed_package(&previous),
        Ok(ref actual) if actual == expected
    ) {
        if let Err(rollback) = restore_moved_directory(&previous, directory) {
            let recovery = transaction.keep();
            return rejected(format!(
                "data addon changed before update; quarantined package remains at {} because restoration failed ({rollback})",
                recovery.join("previous").display()
            ));
        }
        return rejected("data addon changed before the atomic update commit");
    }
    if let Err(error) = commit(&replacement, directory) {
        if let Err(rollback) = restore_moved_directory(&previous, directory) {
            let recovery = transaction.keep();
            return rejected(format!(
                "data addon update failed ({error}); prior package remains at {} because rollback failed ({rollback})",
                recovery.join("previous").display()
            ));
        }
        return Err(error.into());
    }
    if let Err(error) = remove_package(&previous) {
        let recovery = transaction.keep();
        return rejected(format!(
            "data addon updated, but prior-package cleanup failed ({error}); recovery artifact: {}",
            recovery.display()
        ));
    }
    Ok(())
}

pub(crate) fn take_module(
    root: &mut serde_json::Value,
    module: &str,
) -> Result<serde_json::Value, String> {
    let object = root
        .as_object_mut()
        .ok_or_else(|| "shared SavedVariables root must be a table".to_string())?;
    if object
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        != Some(SAVED_VARIABLES_SCHEMA_VERSION)
        || object
            .get("addon_version")
            .and_then(serde_json::Value::as_u64)
            != Some(DATA_ADDON_VERSION)
    {
        return Err("shared SavedVariables root has an unsupported version".into());
    }
    object
        .remove(module)
        .ok_or_else(|| format!("shared SavedVariables root has no {module} module"))
}

pub fn uninstall(
    addons_root: &Path,
    running: RunningState,
) -> Result<LifecycleOutcome, CollectorError> {
    let existing = status(addons_root);
    if !matches!(
        existing,
        DataAddonStatus::ManagedUpToDate | DataAddonStatus::ManagedVersionMismatch
    ) {
        return rejected("refusing to remove an unmanaged or absent collector");
    }
    let directory = addons_root.join(DATA_ADDON_SUBFOLDER);
    let snapshot = snapshot_managed_package(&directory)?;
    uninstall_with(addons_root, &directory, &snapshot, || {}, |_, _| {})?;
    Ok(LifecycleOutcome {
        status: DataAddonStatus::NotInstalled,
        reload_required: reload_reminder(running),
    })
}

fn uninstall_with(
    addons_root: &Path,
    directory: &Path,
    expected: &ManagedPackageSnapshot,
    before_move: impl FnOnce(),
    after_move: impl FnOnce(&Path, &Path),
) -> Result<(), CollectorError> {
    let transaction = tempfile::Builder::new()
        .prefix(".eso-weave-data-remove-")
        .tempdir_in(addons_root)?;
    let removed = transaction.path().join("removed");
    before_move();
    rename_directory_no_replace(directory, &removed)?;
    after_move(&removed, directory);
    if !matches!(
        snapshot_managed_package(&removed),
        Ok(ref actual) if actual == expected
    ) {
        if let Err(rollback) = restore_moved_directory(&removed, directory) {
            let recovery = transaction.keep();
            return rejected(format!(
                "data addon changed before uninstall; quarantined package remains at {} because restoration failed ({rollback})",
                recovery.join("removed").display()
            ));
        }
        return rejected("data addon changed before the atomic uninstall quarantine");
    }
    if let Err(error) = remove_package(&removed) {
        let recovery = transaction.keep();
        return rejected(format!(
            "data addon was quarantined but cleanup failed ({error}); recovery artifact: {}",
            recovery.display()
        ));
    }
    Ok(())
}

pub fn saved_variables_path(addons_root: &Path) -> Option<PathBuf> {
    addons_root.parent().map(|environment| {
        environment
            .join("SavedVariables")
            .join(SAVED_VARIABLES_FILE)
    })
}

fn parse_addon_version(manifest: &str) -> Option<u32> {
    manifest.lines().find_map(|line| {
        line.trim()
            .strip_prefix("## AddOnVersion:")
            .and_then(|value| value.trim().parse().ok())
    })
}

fn parse_primary_api_version(manifest: &str) -> Option<u32> {
    manifest.lines().find_map(|line| {
        line.trim()
            .strip_prefix("## APIVersion:")
            .and_then(|value| value.split_whitespace().next())
            .and_then(|value| value.parse().ok())
    })
}

fn render_manifest(api_version: u32) -> String {
    let mut versions = vec![api_version];
    for known in [101051, 101050] {
        if !versions.contains(&known) {
            versions.push(known);
        }
    }
    let replacement = format!(
        "## APIVersion: {}",
        versions
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
    MANIFEST
        .lines()
        .map(|line| {
            if line.starts_with("## APIVersion:") {
                replacement.as_str()
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn reload_reminder(running: RunningState) -> bool {
    matches!(running, RunningState::Running | RunningState::Unknown)
}

fn metadata_is_link(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if metadata.file_attributes() & 0x0400 != 0 {
            return true;
        }
    }
    false
}

fn install_fresh_with(
    addons_root: &Path,
    directory: &Path,
    api_version: u32,
    before_commit: impl FnOnce(),
) -> Result<(), CollectorError> {
    let staging = tempfile::Builder::new()
        .prefix(".eso-weave-data-")
        .tempdir_in(addons_root)?;
    write_package(staging.path(), api_version)?;
    let staging_path = staging.keep();
    before_commit();
    if let Err(error) = rename_directory_no_replace(&staging_path, directory) {
        let _ = fs::remove_dir_all(staging_path);
        return Err(error.into());
    }
    Ok(())
}

fn write_package(directory: &Path, api_version: u32) -> std::io::Result<()> {
    fs::create_dir_all(directory)?;
    for (name, bytes) in [
        (BOOTSTRAP_FILE, BOOTSTRAP.as_bytes().to_vec()),
        (CATALOG_FILE, CATALOG.as_bytes().to_vec()),
        (ENCOUNTER_FILE, ENCOUNTER.as_bytes().to_vec()),
        (MANIFEST_FILE, render_manifest(api_version).into_bytes()),
    ] {
        let path = directory.join(name);
        let mut file = fs::File::create(path)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ManagedPackageSnapshot(Vec<(String, Option<Vec<u8>>)>);

fn snapshot_managed_package(directory: &Path) -> Result<ManagedPackageSnapshot, CollectorError> {
    if !matches!(
        status_directory(directory),
        DataAddonStatus::ManagedUpToDate | DataAddonStatus::ManagedVersionMismatch
    ) {
        return rejected("data addon package is not safely managed");
    }
    let mut files = Vec::new();
    for name in [MANIFEST_FILE, BOOTSTRAP_FILE, CATALOG_FILE, ENCOUNTER_FILE] {
        let bytes = match fs::read(directory.join(name)) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.into()),
        };
        files.push((name.to_string(), bytes));
    }
    Ok(ManagedPackageSnapshot(files))
}

fn restore_moved_directory(moved: &Path, directory: &Path) -> std::io::Result<()> {
    rename_directory_no_replace(moved, directory)
}

#[cfg(target_os = "linux")]
fn rename_directory_no_replace(source: &Path, destination: &Path) -> std::io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    let source = CString::new(source.as_os_str().as_bytes())
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
    let destination = CString::new(destination.as_os_str().as_bytes())
        .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidInput))?;
    // SAFETY: both C strings are NUL-terminated and remain alive for the call.
    let result = unsafe {
        libc::renameat2(
            libc::AT_FDCWD,
            source.as_ptr(),
            libc::AT_FDCWD,
            destination.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(windows)]
fn rename_directory_no_replace(source: &Path, destination: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{MoveFileExW, MOVEFILE_WRITE_THROUGH};

    let source = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let destination = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    // SAFETY: both owned buffers are NUL-terminated and remain alive for the call.
    let result = unsafe {
        MoveFileExW(
            source.as_ptr(),
            destination.as_ptr(),
            MOVEFILE_WRITE_THROUGH,
        )
    };
    if result != 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn rename_directory_no_replace(source: &Path, destination: &Path) -> std::io::Result<()> {
    if fs::symlink_metadata(destination).is_ok() {
        return Err(std::io::Error::from(std::io::ErrorKind::AlreadyExists));
    }
    fs::rename(source, destination)
}

fn remove_package(directory: &Path) -> std::io::Result<()> {
    for name in [BOOTSTRAP_FILE, CATALOG_FILE, ENCOUNTER_FILE] {
        match fs::remove_file(directory.join(name)) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    fs::remove_file(directory.join(MANIFEST_FILE))?;
    fs::remove_dir(directory)
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut result = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut result, "{byte:02x}").expect("write to String");
    }
    result
}

fn rejected<T>(message: impl Into<String>) -> Result<T, CollectorError> {
    Err(CollectorError::Validation(message.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_directory_commit_restores_every_prior_byte_and_neighbor() {
        let root = tempfile::tempdir().unwrap();
        let addons = root.path().join("AddOns");
        fs::create_dir(&addons).unwrap();
        install(&addons, RunningState::NotRunning, 101051).unwrap();
        let directory = addons.join(DATA_ADDON_SUBFOLDER);
        fs::write(directory.join(BOOTSTRAP_FILE), b"prior bootstrap\n").unwrap();
        fs::write(directory.join(CATALOG_FILE), b"prior catalog\n").unwrap();
        let neighbor = addons.join("PixelBeacon.txt");
        fs::write(&neighbor, b"neighbor\n").unwrap();

        let paths = [BOOTSTRAP_FILE, CATALOG_FILE, ENCOUNTER_FILE, MANIFEST_FILE];
        let before = paths
            .iter()
            .map(|name| fs::read(directory.join(name)).unwrap())
            .collect::<Vec<_>>();
        let snapshot = snapshot_managed_package(&directory).unwrap();
        let result = update_existing_with(
            &addons,
            &directory,
            101050,
            &snapshot,
            || {},
            |_, _| {},
            |_, _| Err(std::io::Error::other("injected commit failure")),
        );
        assert!(result.is_err());
        for (name, expected) in paths.iter().zip(before) {
            assert_eq!(fs::read(directory.join(name)).unwrap(), expected);
        }
        assert_eq!(fs::read(neighbor).unwrap(), b"neighbor\n");
    }

    #[test]
    fn update_stages_all_files_and_rejects_a_swapped_directory() {
        let root = tempfile::tempdir().unwrap();
        let addons = root.path().join("AddOns");
        fs::create_dir(&addons).unwrap();
        install(&addons, RunningState::NotRunning, 101051).unwrap();
        let directory = addons.join(DATA_ADDON_SUBFOLDER);
        let original = addons.join("original-away");
        let snapshot = snapshot_managed_package(&directory).unwrap();

        let result = update_existing_with(
            &addons,
            &directory,
            101050,
            &snapshot,
            || {
                fs::rename(&directory, &original).unwrap();
                fs::create_dir(&directory).unwrap();
                fs::write(directory.join("foreign.txt"), b"foreign\n").unwrap();
            },
            |_, _| {},
            |replacement, destination| {
                let names = fs::read_dir(replacement)?
                    .map(|entry| entry.map(|entry| entry.file_name()))
                    .collect::<Result<Vec<_>, _>>()?;
                assert_eq!(names.len(), 4);
                fs::rename(replacement, destination)
            },
        );
        assert!(result.is_err());
        assert_eq!(
            fs::read(directory.join("foreign.txt")).unwrap(),
            b"foreign\n"
        );
        assert_eq!(snapshot_managed_package(&original).unwrap(), snapshot);
    }

    #[test]
    fn uninstall_rejects_a_swapped_directory_before_deletion() {
        let root = tempfile::tempdir().unwrap();
        let addons = root.path().join("AddOns");
        fs::create_dir(&addons).unwrap();
        install(&addons, RunningState::NotRunning, 101051).unwrap();
        let directory = addons.join(DATA_ADDON_SUBFOLDER);
        let original = addons.join("original-away");
        let snapshot = snapshot_managed_package(&directory).unwrap();

        let result = uninstall_with(
            &addons,
            &directory,
            &snapshot,
            || {
                fs::rename(&directory, &original).unwrap();
                fs::create_dir(&directory).unwrap();
                fs::write(directory.join("foreign.txt"), b"foreign\n").unwrap();
            },
            |_, _| {},
        );
        assert!(result.is_err());
        assert_eq!(
            fs::read(directory.join("foreign.txt")).unwrap(),
            b"foreign\n"
        );
        assert_eq!(snapshot_managed_package(&original).unwrap(), snapshot);
    }

    #[test]
    fn failed_update_rollback_retains_the_quarantined_package() {
        let root = tempfile::tempdir().unwrap();
        let addons = root.path().join("AddOns");
        fs::create_dir(&addons).unwrap();
        install(&addons, RunningState::NotRunning, 101051).unwrap();
        let directory = addons.join(DATA_ADDON_SUBFOLDER);
        let snapshot = snapshot_managed_package(&directory).unwrap();

        let result = update_existing_with(
            &addons,
            &directory,
            101050,
            &snapshot,
            || {},
            |_, _| {},
            |replacement, destination| {
                fs::create_dir(destination)?;
                rename_directory_no_replace(replacement, destination)
            },
        );
        assert!(result.is_err());
        let previous = recovery_child(&addons, ".eso-weave-data-update-", "previous");
        assert_eq!(snapshot_managed_package(&previous).unwrap(), snapshot);
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
    }

    #[test]
    fn fresh_install_never_replaces_a_raced_in_empty_directory() {
        let root = tempfile::tempdir().unwrap();
        let addons = root.path().join("AddOns");
        fs::create_dir(&addons).unwrap();
        let directory = addons.join(DATA_ADDON_SUBFOLDER);

        let result = install_fresh_with(&addons, &directory, 101051, || {
            fs::create_dir(&directory).unwrap();
        });
        assert!(result.is_err());
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
        assert_eq!(status(&addons), DataAddonStatus::Unmanaged);
    }

    #[test]
    fn failed_uninstall_restore_retains_the_quarantined_package() {
        let root = tempfile::tempdir().unwrap();
        let addons = root.path().join("AddOns");
        fs::create_dir(&addons).unwrap();
        install(&addons, RunningState::NotRunning, 101051).unwrap();
        let directory = addons.join(DATA_ADDON_SUBFOLDER);
        let snapshot = snapshot_managed_package(&directory).unwrap();
        let mut wrong_snapshot = snapshot.clone();
        wrong_snapshot.0[0].1 = Some(b"different manifest\n".to_vec());

        let result = uninstall_with(
            &addons,
            &directory,
            &wrong_snapshot,
            || {},
            |_, destination| {
                fs::create_dir(destination).unwrap();
                fs::write(destination.join("foreign.txt"), b"foreign\n").unwrap();
            },
        );
        assert!(result.is_err());
        let removed = recovery_child(&addons, ".eso-weave-data-remove-", "removed");
        assert_eq!(snapshot_managed_package(&removed).unwrap(), snapshot);
        assert_eq!(
            fs::read(directory.join("foreign.txt")).unwrap(),
            b"foreign\n"
        );
    }

    fn recovery_child(addons: &Path, prefix: &str, child: &str) -> PathBuf {
        fs::read_dir(addons)
            .unwrap()
            .map(|entry| entry.unwrap())
            .find(|entry| entry.file_name().to_string_lossy().starts_with(prefix))
            .expect("retained recovery transaction")
            .path()
            .join(child)
    }
}
