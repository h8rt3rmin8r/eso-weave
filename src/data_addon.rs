//! Compatibility facade for the unified PixelBeacon companion package.
//! SavedVariables and existing Catalog/encounter readers keep their identities.

use crate::collector::CollectorError;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub const DATA_ADDON_SUBFOLDER: &str = crate::beacon::SUBFOLDER;
pub const MANIFEST_FILE: &str = crate::beacon::MANIFEST_FILE;
pub const BOOTSTRAP_FILE: &str = crate::beacon::BOOTSTRAP_FILE;
pub const CATALOG_FILE: &str = crate::beacon::CATALOG_FILE;
pub const SAVED_VARIABLES_FILE: &str = "EsoWeaveData.lua";
pub const MAX_SAVED_VARIABLES_BYTES: u64 = 128 * 1024 * 1024;
pub const MAX_SAVED_VARIABLES_TOKENS: usize = 4_000_000;
pub const MAX_SAVED_VARIABLES_ENTRIES: usize = 2_600_000;
pub const SAVED_VARIABLES_SCHEMA_VERSION: u64 = 1;
pub const DATA_ADDON_VERSION: u64 = 1;
pub const MANAGED_MARKER: &str = crate::beacon::MANAGED_MARKER;
const CHECKSUM_PREFIX: &str = "local COLLECTOR_CHECKSUM = \"";
pub const MANIFEST: &str = crate::beacon::MANIFEST;
pub const BOOTSTRAP: &str = crate::beacon::BOOTSTRAP;
pub const CATALOG: &str = crate::beacon::CATALOG;

pub fn supports_api(api: u32) -> bool {
    crate::beacon::supports_api(api)
}

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

fn map_status(status: crate::beacon::BeaconStatus) -> DataAddonStatus {
    match status {
        crate::beacon::BeaconStatus::NotInstalled => DataAddonStatus::NotInstalled,
        crate::beacon::BeaconStatus::ManagedUpToDate => DataAddonStatus::ManagedUpToDate,
        crate::beacon::BeaconStatus::ManagedVersionMismatch => {
            DataAddonStatus::ManagedVersionMismatch
        }
        crate::beacon::BeaconStatus::Unmanaged => DataAddonStatus::Unmanaged,
    }
}
fn map_running(running: RunningState) -> crate::beacon::RunningState {
    match running {
        RunningState::Running => crate::beacon::RunningState::Running,
        RunningState::NotRunning => crate::beacon::RunningState::NotRunning,
        RunningState::Unknown => crate::beacon::RunningState::Unknown,
    }
}
fn map_outcome(outcome: crate::beacon::LifecycleOutcome) -> LifecycleOutcome {
    LifecycleOutcome {
        status: map_status(outcome.status),
        reload_required: outcome.reload_required,
    }
}
fn map_error(error: crate::beacon::LifecycleError) -> CollectorError {
    match error {
        crate::beacon::LifecycleError::Io(error) => CollectorError::Io(error),
        other => CollectorError::Validation(other.to_string()),
    }
}

pub fn status(addons_root: &Path) -> DataAddonStatus {
    map_status(crate::beacon::status(addons_root))
}
pub fn inspect(addons_root: &Path) -> std::io::Result<DataAddonStatus> {
    crate::beacon::inspect(addons_root).map(map_status)
}
pub fn install(
    addons_root: &Path,
    running: RunningState,
    api_version: u32,
) -> Result<LifecycleOutcome, CollectorError> {
    let block_px =
        crate::beacon::installed_block_px(addons_root).unwrap_or(crate::pixelbus::DEFAULT_BLOCK_PX);
    crate::beacon::install_sized(addons_root, map_running(running), api_version, block_px)
        .map(map_outcome)
        .map_err(map_error)
}
pub fn uninstall(
    addons_root: &Path,
    running: RunningState,
) -> Result<LifecycleOutcome, CollectorError> {
    crate::beacon::uninstall(addons_root, map_running(running))
        .map(map_outcome)
        .map_err(map_error)
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

pub(crate) fn take_module(
    root: &mut serde_json::Value,
    module: &str,
) -> Result<serde_json::Value, String> {
    take_optional_module(root, module)?
        .ok_or_else(|| format!("shared SavedVariables root has no {module} module"))
}

pub(crate) fn take_optional_module(
    root: &mut serde_json::Value,
    module: &str,
) -> Result<Option<serde_json::Value>, String> {
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
    Ok(object.remove(module))
}

pub fn saved_variables_path(addons_root: &Path) -> Option<PathBuf> {
    addons_root.parent().map(|environment| {
        environment
            .join("SavedVariables")
            .join(SAVED_VARIABLES_FILE)
    })
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
