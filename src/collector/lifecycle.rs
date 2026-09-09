//! Managed lifecycle for the collector addon, deliberately separate from PixelBeacon.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

use super::CollectorError;

pub const COLLECTOR_SUBFOLDER: &str = "EsoWeaveCollector";
pub const MANIFEST_FILE: &str = "EsoWeaveCollector.txt";
pub const LUA_FILE: &str = "EsoWeaveCollector.lua";
pub const SAVED_VARIABLES_FILE: &str = "EsoWeaveCollector.lua";
pub const MANAGED_MARKER: &str = "## X-ESO-Weave-Collector-Managed: true";
const CHECKSUM_PREFIX: &str = "local COLLECTOR_CHECKSUM = \"";

pub const MANIFEST: &str = include_str!("../../addon/EsoWeaveCollector/EsoWeaveCollector.txt");
pub const LUA: &str = include_str!("../../addon/EsoWeaveCollector/EsoWeaveCollector.lua");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollectorStatus {
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
    pub status: CollectorStatus,
    pub reload_required: bool,
}

pub fn embedded_checksum() -> String {
    collector_checksum_from_source(LUA).expect("embedded collector contains checksum literal")
}

pub fn collector_checksum_from_source(source: &str) -> Result<String, CollectorError> {
    let start = source
        .find(CHECKSUM_PREFIX)
        .map(|index| index + CHECKSUM_PREFIX.len())
        .ok_or_else(|| {
            CollectorError::Validation("collector checksum literal is missing".into())
        })?;
    let end = source[start..]
        .find('"')
        .map(|index| start + index)
        .ok_or_else(|| {
            CollectorError::Validation("collector checksum literal is unterminated".into())
        })?;
    let literal = &source[start..end];
    if literal.len() != 64 || !literal.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(CollectorError::Validation(
            "collector checksum literal must contain 64 hexadecimal characters".into(),
        ));
    }
    let mut normalized = source.as_bytes().to_vec();
    normalized[start..end].fill(b'0');
    Ok(sha256_bytes(&normalized))
}

pub fn status(addons_root: &Path) -> CollectorStatus {
    let directory = addons_root.join(COLLECTOR_SUBFOLDER);
    let metadata = match fs::symlink_metadata(&directory) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return CollectorStatus::NotInstalled;
        }
        Err(_) => return CollectorStatus::Unmanaged,
    };
    if metadata_is_link(&metadata) || !metadata.is_dir() {
        return CollectorStatus::Unmanaged;
    }
    let manifest_path = directory.join(MANIFEST_FILE);
    let lua_path = directory.join(LUA_FILE);
    let Ok(manifest_metadata) = fs::symlink_metadata(&manifest_path) else {
        return CollectorStatus::Unmanaged;
    };
    if metadata_is_link(&manifest_metadata) || !manifest_metadata.is_file() {
        return CollectorStatus::Unmanaged;
    }
    let Ok(entries) = fs::read_dir(&directory) else {
        return CollectorStatus::Unmanaged;
    };
    for entry in entries {
        let Ok(name) = entry.map(|entry| entry.file_name()) else {
            return CollectorStatus::Unmanaged;
        };
        if name != MANIFEST_FILE && name != LUA_FILE {
            return CollectorStatus::Unmanaged;
        }
    }
    let Ok(manifest) = fs::read_to_string(manifest_path) else {
        return CollectorStatus::Unmanaged;
    };
    if !manifest.lines().any(|line| line.trim() == MANAGED_MARKER) {
        return CollectorStatus::Unmanaged;
    }
    let lua_metadata = match fs::symlink_metadata(&lua_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return CollectorStatus::ManagedVersionMismatch;
        }
        Err(_) => return CollectorStatus::Unmanaged,
    };
    if metadata_is_link(&lua_metadata) || !lua_metadata.is_file() {
        return CollectorStatus::Unmanaged;
    }
    let version_matches = parse_addon_version(&manifest) == Some(super::COLLECTOR_VERSION);
    let lua_matches = fs::read_to_string(lua_path)
        .ok()
        .and_then(|source| collector_checksum_from_source(&source).ok())
        .is_some_and(|checksum| checksum == embedded_checksum());
    if version_matches && lua_matches {
        CollectorStatus::ManagedUpToDate
    } else {
        CollectorStatus::ManagedVersionMismatch
    }
}

pub fn install(
    addons_root: &Path,
    running: RunningState,
    api_version: u32,
) -> Result<LifecycleOutcome, CollectorError> {
    if !addons_root.is_dir() {
        return rejected("resolved AddOns root does not exist");
    }
    let directory = addons_root.join(COLLECTOR_SUBFOLDER);
    let existing = status(addons_root);
    let fresh = existing == CollectorStatus::NotInstalled;
    match existing {
        CollectorStatus::NotInstalled => fs::create_dir(&directory)?,
        CollectorStatus::ManagedUpToDate | CollectorStatus::ManagedVersionMismatch => {}
        CollectorStatus::Unmanaged => return rejected("refusing to modify an unmanaged collector"),
    }
    let manifest_path = directory.join(MANIFEST_FILE);
    let lua_path = directory.join(LUA_FILE);
    validate_regular_or_missing(&manifest_path)?;
    validate_regular_or_missing(&lua_path)?;
    let old_manifest = if fresh {
        None
    } else {
        fs::read(&manifest_path).ok()
    };
    let old_lua = if fresh {
        None
    } else {
        fs::read(&lua_path).ok()
    };

    if let Err(error) = fs::write(&lua_path, LUA.as_bytes()) {
        restore_after_failure(
            &directory,
            fresh,
            &manifest_path,
            old_manifest.as_deref(),
            &lua_path,
            old_lua.as_deref(),
        );
        return Err(error.into());
    }
    if let Err(error) = fs::write(&manifest_path, render_manifest(api_version).as_bytes()) {
        restore_after_failure(
            &directory,
            fresh,
            &manifest_path,
            old_manifest.as_deref(),
            &lua_path,
            old_lua.as_deref(),
        );
        return Err(error.into());
    }
    Ok(LifecycleOutcome {
        status: CollectorStatus::ManagedUpToDate,
        reload_required: reload_reminder(running),
    })
}

pub fn uninstall(
    addons_root: &Path,
    running: RunningState,
) -> Result<LifecycleOutcome, CollectorError> {
    if !matches!(
        status(addons_root),
        CollectorStatus::ManagedUpToDate | CollectorStatus::ManagedVersionMismatch
    ) {
        return rejected("refusing to remove an unmanaged or absent collector");
    }
    let directory = addons_root.join(COLLECTOR_SUBFOLDER);
    fs::remove_file(directory.join(MANIFEST_FILE))?;
    fs::remove_file(directory.join(LUA_FILE))?;
    fs::remove_dir(directory)?;
    Ok(LifecycleOutcome {
        status: CollectorStatus::NotInstalled,
        reload_required: reload_reminder(running),
    })
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

fn validate_regular_or_missing(path: &Path) -> Result<(), CollectorError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata_is_link(&metadata) || !metadata.is_file() => {
            rejected("collector target contains a linked or non-regular file")
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
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

fn restore_after_failure(
    directory: &Path,
    fresh: bool,
    manifest_path: &Path,
    old_manifest: Option<&[u8]>,
    lua_path: &Path,
    old_lua: Option<&[u8]>,
) {
    if fresh {
        let _ = fs::remove_file(manifest_path);
        let _ = fs::remove_file(lua_path);
        let _ = fs::remove_dir(directory);
        return;
    }
    restore_file(manifest_path, old_manifest);
    restore_file(lua_path, old_lua);
}

fn restore_file(path: &Path, old: Option<&[u8]>) {
    match old {
        Some(bytes) => {
            let _ = fs::write(path, bytes);
        }
        None => {
            let _ = fs::remove_file(path);
        }
    }
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
