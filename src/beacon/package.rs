//! One exact companion inventory and its recoverable on-disk transactions.

mod configuration;
mod mutation;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{BeaconStatus, Environment, LifecycleError, LifecycleOutcome, RunningState};
use crate::bounded_file::{is_link_like, read_bounded_stable, StableReadError};

pub const BOOTSTRAP_FILE: &str = "EsoWeaveData.lua";
pub const CATALOG_FILE: &str = "Catalog.lua";
pub const STARTUP_CONFIG_FILE: &str = "StartupConfig.lua";
pub const BOOTSTRAP: &str = include_str!("../../addon/PixelBeacon/EsoWeaveData.lua");
pub const CATALOG: &str = include_str!("../../addon/PixelBeacon/Catalog.lua");
pub const PACKAGE_IDENTITY: &str = "## X-ESO-Weave-Package: PixelBeacon/1";
pub const CAPABILITIES: &str = "## X-ESO-Weave-Capabilities: beacon catalog startup-config";
pub const INVENTORY: [&str; 5] = [
    super::MANIFEST_FILE,
    STARTUP_CONFIG_FILE,
    BOOTSTRAP_FILE,
    super::LUA_FILE,
    CATALOG_FILE,
];
pub const LEGACY_DATA_SUBFOLDER: &str = "EsoWeaveData";
const LEGACY_DATA_MANIFEST: &str = "EsoWeaveData.txt";
const LEGACY_DATA_MARKER: &str = "## X-ESO-Weave-Data-Managed: true";
const LEGACY_DATA_INVENTORY: [&str; 4] = [
    LEGACY_DATA_MANIFEST,
    BOOTSTRAP_FILE,
    CATALOG_FILE,
    "Encounter.lua",
];
const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_MANIFEST_BYTES: u64 = 64 * 1024;
const MAX_CONFIG_BYTES: u64 = 256;
const MAX_RECEIPT_BYTES: u64 = 16 * 1024;
const TRANSACTION_PREFIX: &str = ".eso-weave-pixelbeacon-";
const RECEIPT_FILE: &str = "transaction.json";
const COMMITTED_FILE: &str = "committed";
const COMMITTED_BYTES: &[u8] = b"PixelBeacon transaction committed\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartupConfig {
    pub catalog_disabled: bool,
    pub environment: Environment,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct PackagePresence {
    pub compatible: bool,
    pub version: Option<u32>,
    pub catalog: bool,
    pub native_processing: bool,
    pub startup_config: Option<StartupConfig>,
}

impl PackagePresence {
    pub fn eligible(self) -> bool {
        self.compatible && self.catalog && self.native_processing
    }
}

pub fn render_startup_config(config: StartupConfig) -> String {
    format!(
        "ESO_WEAVE_STARTUP_CONFIG = {{ catalog_disabled = {}, channel = \"{}\" }}\n",
        config.catalog_disabled,
        config.environment.segment()
    )
}

fn parse_startup_config(bytes: &[u8]) -> Option<StartupConfig> {
    for environment in [Environment::Live, Environment::Pts] {
        for catalog_disabled in [false, true] {
            let config = StartupConfig {
                catalog_disabled,
                environment,
            };
            if bytes == render_startup_config(config).as_bytes() {
                return Some(config);
            }
        }
    }
    None
}

fn read_regular(root: &Path, path: &Path, maximum: u64) -> std::io::Result<Vec<u8>> {
    read_bounded_stable(root, path, maximum).map_err(|error| match error {
        StableReadError::Io(error) => error,
        StableReadError::Invalid(reason) => {
            std::io::Error::new(std::io::ErrorKind::InvalidData, reason)
        }
    })
}

fn metadata(path: &Path) -> std::io::Result<Option<fs::Metadata>> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            // Windows reports PATH_NOT_FOUND for a regular-file ancestor. It is
            // an inspection failure, not evidence that a package is absent.
            let mut ancestor = path.parent();
            while let Some(parent) = ancestor {
                if parent.as_os_str().is_empty() {
                    break;
                }
                match fs::symlink_metadata(parent) {
                    Ok(value) if !value.is_dir() || is_link_like(&value) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::NotADirectory,
                            "package path has a non-directory or link-like ancestor",
                        ));
                    }
                    Ok(_) => break,
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        ancestor = parent.parent()
                    }
                    Err(error) => return Err(error),
                }
            }
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

fn canonical_root(addons_root: &Path) -> Result<PathBuf, LifecycleError> {
    match metadata(addons_root)? {
        Some(value) if value.is_dir() && !is_link_like(&value) => {
            Ok(fs::canonicalize(addons_root)?)
        }
        _ => Err(LifecycleError::AddonsDirMissing),
    }
}

fn with_inspection<T>(
    addons_root: &Path,
    inspect: impl FnOnce(&Path) -> std::io::Result<T>,
) -> std::io::Result<T> {
    let root = match metadata(addons_root)? {
        Some(value) if value.is_dir() && !is_link_like(&value) => fs::canonicalize(addons_root)?,
        // Inspectors retain their distinct absent/unmanaged outcomes without
        // creating a directory or requiring installation eligibility.
        _ => return inspect(addons_root),
    };
    // Windows can deny directory retirement while even a bounded read handle
    // remains open in its subtree. Keep the complete inspection under the same
    // per-root lease as mutations, including all stable-read identity checks.
    mutation::with_canonical_root(&root, || inspect(&root))
}

/// Capability recognition is deliberately independent of mutation ownership.
pub fn inspect_presence(addons_root: &Path) -> std::io::Result<PackagePresence> {
    inspect_presence_with(addons_root, |_| {})
}

fn inspect_presence_with(
    addons_root: &Path,
    before_inspect: impl FnOnce(&Path),
) -> std::io::Result<PackagePresence> {
    with_inspection(addons_root, |root| {
        before_inspect(root);
        inspect_presence_locked(root)
    })
}

fn inspect_presence_locked(addons_root: &Path) -> std::io::Result<PackagePresence> {
    let Some(root_metadata) = metadata(addons_root)? else {
        return Ok(PackagePresence::default());
    };
    if !root_metadata.is_dir() || is_link_like(&root_metadata) {
        return Ok(PackagePresence::default());
    }
    let root = fs::canonicalize(addons_root)?;
    let directory = root.join(super::SUBFOLDER);
    let Some(directory_metadata) = metadata(&directory)? else {
        return Ok(PackagePresence::default());
    };
    if !directory_metadata.is_dir() || is_link_like(&directory_metadata) {
        return Ok(PackagePresence::default());
    }
    let inspect = || -> std::io::Result<PackagePresence> {
        let manifest = read_regular(
            &root,
            &directory.join(super::MANIFEST_FILE),
            MAX_MANIFEST_BYTES,
        )?;
        let manifest = std::str::from_utf8(&manifest).map_err(invalid_data)?;
        let version = super::parse_manifest_version(manifest);
        if !manifest.lines().any(|line| line.trim() == PACKAGE_IDENTITY)
            || !manifest.lines().any(|line| line.trim() == CAPABILITIES)
            || !manifest
                .lines()
                .any(|line| line.trim() == "## Title: PixelBeacon")
            || !manifest
                .lines()
                .any(|line| line.trim() == "## SavedVariables: EsoWeaveDataSaved")
            || version.is_none_or(|version| version < 24)
            || !manifest
                .lines()
                .find_map(|line| line.strip_prefix("## APIVersion:"))
                .is_some_and(|versions| {
                    versions
                        .split_whitespace()
                        .any(|token| token.parse::<u32>().is_ok_and(super::supports_api))
                })
        {
            return Ok(PackagePresence::default());
        }
        let loaded = manifest
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect::<Vec<_>>();
        if loaded
            != [
                super::LUA_FILE,
                STARTUP_CONFIG_FILE,
                BOOTSTRAP_FILE,
                CATALOG_FILE,
            ]
        {
            return Ok(PackagePresence::default());
        }
        let mut startup_config = None;
        for file in &INVENTORY[1..] {
            let maximum = if *file == STARTUP_CONFIG_FILE {
                MAX_CONFIG_BYTES
            } else {
                MAX_FILE_BYTES
            };
            let bytes = read_regular(&root, &directory.join(file), maximum)?;
            if bytes.is_empty() || bytes.contains(&0) || std::str::from_utf8(&bytes).is_err() {
                return Ok(PackagePresence::default());
            }
            let source = std::str::from_utf8(&bytes).expect("UTF-8 checked above");
            if *file == super::LUA_FILE
                && (!source.contains("local ADDON_NAME = \"PixelBeacon\"")
                    || super::parse_lua_constant(source, "BLOCK_PX").is_none_or(|value| value == 0)
                    || super::parse_lua_constant(source, "NUM_BLOCKS")
                        != Some(crate::pixelbus::NUM_BLOCKS))
            {
                return Ok(PackagePresence::default());
            }
            if *file == BOOTSTRAP_FILE
                && (!source.contains("local ADDON_NAME = \"PixelBeacon\"")
                    || !source.contains("EsoWeaveDataSaved")
                    || !source.contains("EVENT_ADD_ON_LOADED"))
            {
                return Ok(PackagePresence::default());
            }
            if *file == CATALOG_FILE {
                let declared = source
                    .lines()
                    .find_map(|line| line.strip_prefix("local COLLECTOR_CHECKSUM = \""))
                    .and_then(|value| value.strip_suffix('"'));
                let calculated = crate::data_addon::catalog_checksum_from_source(source).ok();
                if !source.contains("local MODULE_NAMESPACE = ADDON_NAME .. \"Catalog\"")
                    || !matches!((calculated.as_deref(), declared), (Some(actual), Some(expected)) if actual == expected)
                {
                    return Ok(PackagePresence::default());
                }
            }
            if *file == STARTUP_CONFIG_FILE {
                startup_config = parse_startup_config(&bytes);
                if startup_config.is_none() {
                    return Ok(PackagePresence::default());
                }
            }
        }
        Ok(PackagePresence {
            compatible: true,
            version,
            catalog: true,
            native_processing: true,
            startup_config,
        })
    };
    match inspect() {
        Ok(presence) => Ok(presence),
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::InvalidData
            ) =>
        {
            Ok(PackagePresence::default())
        }
        Err(error) => Err(error),
    }
}

fn invalid_data(error: impl std::fmt::Display) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Kind {
    Pixel,
    LegacyData,
}

impl Kind {
    fn directory(self) -> &'static str {
        match self {
            Self::Pixel => super::SUBFOLDER,
            Self::LegacyData => LEGACY_DATA_SUBFOLDER,
        }
    }
    fn previous(self) -> &'static str {
        match self {
            Self::Pixel => "previous-pixel",
            Self::LegacyData => "previous-data",
        }
    }
    fn manifest(self) -> &'static str {
        match self {
            Self::Pixel => super::MANIFEST_FILE,
            Self::LegacyData => LEGACY_DATA_MANIFEST,
        }
    }
    fn marker(self) -> &'static str {
        match self {
            Self::Pixel => super::MANAGED_MARKER,
            Self::LegacyData => LEGACY_DATA_MARKER,
        }
    }
    fn inventory(self) -> &'static [&'static str] {
        match self {
            Self::Pixel => &INVENTORY,
            Self::LegacyData => &LEGACY_DATA_INVENTORY,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Snapshot {
    kind: Kind,
    files: Vec<(String, Vec<u8>)>,
}

impl Snapshot {
    fn receipt(&self) -> ReceiptSnapshot {
        ReceiptSnapshot {
            kind: self.kind,
            files: self
                .files
                .iter()
                .map(|(name, bytes)| (name.clone(), digest(bytes)))
                .collect(),
        }
    }
    fn bytes(&self, name: &str) -> Option<&[u8]> {
        self.files
            .iter()
            .find(|(file, _)| file == name)
            .map(|(_, bytes)| bytes.as_slice())
    }
}

fn snapshot(
    root: &Path,
    directory: &Path,
    kind: Kind,
    require_writable: bool,
) -> Result<Option<Snapshot>, LifecycleError> {
    let Some(directory_metadata) = metadata(directory)? else {
        return Ok(None);
    };
    if !directory_metadata.is_dir() || is_link_like(&directory_metadata) {
        return Err(LifecycleError::Unmanaged);
    }
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| LifecycleError::Unmanaged)?;
        if !kind.inventory().contains(&name.as_str()) {
            return Err(LifecycleError::Unmanaged);
        }
        let entry_metadata = fs::symlink_metadata(entry.path())?;
        if !entry_metadata.is_file() || is_link_like(&entry_metadata) {
            return Err(LifecycleError::Unmanaged);
        }
        if require_writable && entry_metadata.permissions().readonly() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "managed addon file is read-only",
            )
            .into());
        }
        let limit = if name == kind.manifest() {
            MAX_MANIFEST_BYTES
        } else if name == STARTUP_CONFIG_FILE {
            MAX_CONFIG_BYTES
        } else {
            MAX_FILE_BYTES
        };
        let bytes = read_regular(root, &entry.path(), limit).map_err(|error| {
            if error.kind() == std::io::ErrorKind::InvalidData {
                LifecycleError::Unmanaged
            } else {
                LifecycleError::Io(error)
            }
        })?;
        files.push((name, bytes));
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    let snapshot = Snapshot { kind, files };
    let manifest = snapshot
        .bytes(kind.manifest())
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .ok_or(LifecycleError::Unmanaged)?;
    if !manifest.lines().any(|line| line.trim() == kind.marker()) {
        return Err(LifecycleError::Unmanaged);
    }
    // A legacy beacon marker cannot authorize removal of newly named files.
    if kind == Kind::Pixel
        && !manifest.lines().any(|line| line.trim() == PACKAGE_IDENTITY)
        && snapshot
            .files
            .iter()
            .any(|(name, _)| name != super::MANIFEST_FILE && name != super::LUA_FILE)
    {
        return Err(LifecycleError::Unmanaged);
    }
    Ok(Some(snapshot))
}

pub fn inspect(addons_root: &Path) -> std::io::Result<BeaconStatus> {
    with_inspection(addons_root, inspect_locked)
}

fn inspect_locked(addons_root: &Path) -> std::io::Result<BeaconStatus> {
    let Some(value) = metadata(addons_root)? else {
        return Ok(BeaconStatus::NotInstalled);
    };
    if !value.is_dir() || is_link_like(&value) {
        return Ok(BeaconStatus::Unmanaged);
    }
    let root = fs::canonicalize(addons_root)?;
    match snapshot(&root, &root.join(super::SUBFOLDER), Kind::Pixel, false) {
        Ok(None) => match snapshot(
            &root,
            &root.join(LEGACY_DATA_SUBFOLDER),
            Kind::LegacyData,
            false,
        ) {
            Ok(Some(_)) => Ok(BeaconStatus::ManagedVersionMismatch),
            Ok(None) => Ok(BeaconStatus::NotInstalled),
            Err(LifecycleError::Io(error)) => Err(error),
            Err(_) => Ok(BeaconStatus::Unmanaged),
        },
        Ok(Some(installed)) => {
            let config = installed
                .bytes(STARTUP_CONFIG_FILE)
                .and_then(parse_startup_config);
            let block_px = installed
                .bytes(super::LUA_FILE)
                .and_then(|bytes| std::str::from_utf8(bytes).ok())
                .and_then(|lua| super::parse_lua_constant(lua, "BLOCK_PX"));
            let current = config.zip(block_px).is_some_and(|(config, block_px)| {
                expected_snapshot(block_px, config).files == installed.files
            });
            // A duplicate legacy package is still a migration to complete.
            let duplicate = metadata(&root.join(LEGACY_DATA_SUBFOLDER))?.is_some();
            Ok(if current && !duplicate {
                BeaconStatus::ManagedUpToDate
            } else {
                BeaconStatus::ManagedVersionMismatch
            })
        }
        Err(LifecycleError::Io(error)) => Err(error),
        Err(_) => Ok(BeaconStatus::Unmanaged),
    }
}

fn expected_snapshot(block_px: u32, config: StartupConfig) -> Snapshot {
    let mut files = vec![
        (
            super::MANIFEST_FILE.to_owned(),
            super::MANIFEST.as_bytes().to_vec(),
        ),
        (
            super::LUA_FILE.to_owned(),
            super::render_lua(block_px).into_bytes(),
        ),
        (BOOTSTRAP_FILE.to_owned(), BOOTSTRAP.as_bytes().to_vec()),
        (CATALOG_FILE.to_owned(), CATALOG.as_bytes().to_vec()),
        (
            STARTUP_CONFIG_FILE.to_owned(),
            render_startup_config(config).into_bytes(),
        ),
    ];
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Snapshot {
        kind: Kind::Pixel,
        files,
    }
}

/// Installed declarations are presentation evidence, never mutation ownership.
pub fn installed_api_support(addons_root: &Path, api: u32) -> Option<bool> {
    with_inspection(addons_root, |root| {
        let bytes = read_regular(
            root,
            &root.join(super::SUBFOLDER).join(super::MANIFEST_FILE),
            MAX_MANIFEST_BYTES,
        )?;
        let manifest = std::str::from_utf8(&bytes).ok();
        let versions = manifest
            .and_then(|text| {
                text.lines()
                    .find_map(|line| line.trim().strip_prefix("## APIVersion:"))
            })
            .and_then(|tokens| {
                tokens
                    .split_whitespace()
                    .map(str::parse::<u32>)
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .filter(|values| !values.is_empty() && values.iter().all(|api| *api > 0));
        Ok(versions.map(|values| values.contains(&api)))
    })
    .ok()
    .flatten()
}

pub fn installed_block_px(addons_root: &Path) -> Option<u32> {
    with_inspection(addons_root, |root| Ok(installed_block_px_locked(root)))
        .ok()
        .flatten()
}

fn installed_block_px_locked(addons_root: &Path) -> Option<u32> {
    let root = canonical_root(addons_root).ok()?;
    let bytes = read_regular(
        &root,
        &root.join(super::SUBFOLDER).join(super::LUA_FILE),
        MAX_FILE_BYTES,
    )
    .ok()?;
    let value = super::parse_lua_constant(std::str::from_utf8(&bytes).ok()?, "BLOCK_PX")?;
    (value > 0).then_some(value)
}

fn retained_config(addons_root: &Path) -> StartupConfig {
    inspect_presence_locked(addons_root)
        .ok()
        .and_then(|presence| presence.startup_config)
        .unwrap_or(StartupConfig {
            catalog_disabled: false,
            environment: if addons_root
                .parent()
                .and_then(Path::file_name)
                .is_some_and(|name| name == "pts")
            {
                Environment::Pts
            } else {
                Environment::Live
            },
        })
}

pub fn install_sized(
    addons_root: &Path,
    _running: RunningState,
    _api_version: u32,
    block_px: u32,
) -> Result<LifecycleOutcome, LifecycleError> {
    mutation::with_root(addons_root, |root| {
        recover_locked(root)?;
        install_locked(root, block_px, retained_config(root))
    })
}

pub fn install_configured(
    addons_root: &Path,
    _running: RunningState,
    _api_version: u32,
    block_px: u32,
    catalog_disabled: bool,
    environment: Environment,
) -> Result<LifecycleOutcome, LifecycleError> {
    mutation::with_root(addons_root, |root| {
        recover_locked(root)?;
        install_locked(
            root,
            block_px,
            StartupConfig {
                catalog_disabled,
                environment,
            },
        )
    })
}

// Internal helpers run under mutation::with_root. They never call public
// mutation entry points, which would attempt to acquire the same mutex twice.
fn install_locked(
    root: &Path,
    block_px: u32,
    config: StartupConfig,
) -> Result<LifecycleOutcome, LifecycleError> {
    let replacement = expected_snapshot(block_px, config);
    transact(root, Some(replacement), |_, _| Ok(()))?;
    Ok(LifecycleOutcome {
        status: BeaconStatus::ManagedUpToDate,
        reload_required: false,
    })
}

pub fn write_startup_config(
    addons_root: &Path,
    catalog_disabled: bool,
    environment: Environment,
) -> Result<LifecycleOutcome, LifecycleError> {
    mutation::with_root(addons_root, |root| {
        recover_locked(root)?;
        configuration::update(
            root,
            StartupConfig {
                catalog_disabled,
                environment,
            },
        )
    })
}

/// Startup reconciliation upgrades recognized owned packages without treating
/// absence or a manual/conflicting package as permission to install or adopt it.
pub fn migrate_legacy_if_needed(
    addons_root: &Path,
    catalog_disabled: bool,
    environment: Environment,
) -> Result<bool, LifecycleError> {
    let result = mutation::with_root(addons_root, |root| {
        migrate_legacy_locked(root, catalog_disabled, environment)
    });
    match result {
        Err(error)
            if matches!(metadata(addons_root), Ok(None))
                && matches!(
                    &error,
                    LifecycleError::AddonsDirMissing | LifecycleError::Io(_)
                ) =>
        {
            Ok(false)
        }
        other => other,
    }
}

fn migrate_legacy_locked(
    root: &Path,
    catalog_disabled: bool,
    environment: Environment,
) -> Result<bool, LifecycleError> {
    recover_locked(root)?;
    let pixel = match snapshot(root, &root.join(super::SUBFOLDER), Kind::Pixel, false) {
        Ok(value) => value,
        Err(LifecycleError::Unmanaged) => return Ok(false),
        Err(error) => return Err(error),
    };
    let data = match snapshot(
        root,
        &root.join(LEGACY_DATA_SUBFOLDER),
        Kind::LegacyData,
        false,
    ) {
        Ok(value) => value,
        Err(LifecycleError::Unmanaged) => return Ok(false),
        Err(error) => return Err(error),
    };
    if pixel.is_none() && data.is_none() {
        return Ok(false);
    }
    if data.is_none() && inspect_locked(root)? == BeaconStatus::ManagedUpToDate {
        return Ok(false);
    }
    let block_px = installed_block_px_locked(root).unwrap_or(crate::pixelbus::DEFAULT_BLOCK_PX);
    install_locked(
        root,
        block_px,
        StartupConfig {
            catalog_disabled,
            environment,
        },
    )?;
    Ok(true)
}

pub fn uninstall(
    addons_root: &Path,
    running: RunningState,
) -> Result<LifecycleOutcome, LifecycleError> {
    uninstall_with(addons_root, running, |_, _| Ok(()))
}

fn uninstall_with(
    addons_root: &Path,
    _running: RunningState,
    boundary: impl FnMut(&str, &Path) -> std::io::Result<()>,
) -> Result<LifecycleOutcome, LifecycleError> {
    mutation::with_root(addons_root, |root| {
        recover_locked(root)?;
        transact(root, None, boundary)?;
        Ok(LifecycleOutcome {
            status: BeaconStatus::NotInstalled,
            reload_required: false,
        })
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReceiptSnapshot {
    kind: Kind,
    files: Vec<(String, String)>,
}

impl ReceiptSnapshot {
    fn valid(&self) -> bool {
        !self.files.is_empty()
            && self.files.len() <= self.kind.inventory().len()
            && self.files.iter().all(|(name, hash)| {
                self.kind.inventory().contains(&name.as_str())
                    && hash.len() == 64
                    && hash.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
            && self.files.windows(2).all(|pair| pair[0].0 < pair[1].0)
            && self
                .files
                .iter()
                .any(|(name, _)| name == self.kind.manifest())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    identity: String,
    root: PathBuf,
    pixel: Option<ReceiptSnapshot>,
    data: Option<ReceiptSnapshot>,
    replacement: Option<ReceiptSnapshot>,
}

fn digest(bytes: &[u8]) -> String {
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        use std::fmt::Write as _;
        write!(&mut result, "{byte:02x}").expect("write to String");
    }
    result
}

fn write_synced(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(bytes)?;
    file.sync_all()
}

fn transact(
    root: &Path,
    replacement: Option<Snapshot>,
    mut boundary: impl FnMut(&str, &Path) -> std::io::Result<()>,
) -> Result<(), LifecycleError> {
    let pixel = snapshot(root, &root.join(super::SUBFOLDER), Kind::Pixel, true)?;
    let data = snapshot(
        root,
        &root.join(LEGACY_DATA_SUBFOLDER),
        Kind::LegacyData,
        true,
    )?;
    if replacement.is_none() && pixel.is_none() && data.is_none() {
        return Ok(());
    }
    if replacement.as_ref() == pixel.as_ref() && data.is_none() {
        return Ok(());
    }
    boundary("before-staging", root)?;
    let transaction = tempfile::Builder::new()
        .prefix(TRANSACTION_PREFIX)
        .tempdir_in(root)?
        .keep();
    let receipt = Receipt {
        identity: "PixelBeacon transaction 1".into(),
        root: root.to_owned(),
        pixel: pixel.as_ref().map(Snapshot::receipt),
        data: data.as_ref().map(Snapshot::receipt),
        replacement: replacement.as_ref().map(Snapshot::receipt),
    };
    // Record ownership before staging. A crash at any later boundary is recoverable.
    write_synced(
        &transaction.join(RECEIPT_FILE),
        &serde_json::to_vec(&receipt).map_err(invalid_data)?,
    )?;
    let mut replacement_committed = false;
    let result = (|| -> Result<(), LifecycleError> {
        if let Some(replacement) = &replacement {
            let directory = transaction.join("replacement");
            fs::create_dir(&directory)?;
            // Write the ownership manifest first. Recovery can then distinguish
            // an interrupted staged inventory from arbitrary unowned files.
            for (name, bytes) in replacement
                .files
                .iter()
                .filter(|(name, _)| name == super::MANIFEST_FILE)
                .chain(
                    replacement
                        .files
                        .iter()
                        .filter(|(name, _)| name != super::MANIFEST_FILE),
                )
            {
                write_synced(&directory.join(name), bytes)?;
            }
        }
        boundary("before-retirement", &transaction)?;
        for expected in [&pixel, &data].into_iter().flatten() {
            let previous = transaction.join(expected.kind.previous());
            rename_no_replace(&root.join(expected.kind.directory()), &previous)?;
            boundary(
                if expected.kind == Kind::Pixel {
                    "after-pixel-retirement"
                } else {
                    "after-data-retirement"
                },
                &transaction,
            )?;
            if snapshot(root, &previous, expected.kind, false)?.as_ref() != Some(expected) {
                return Err(LifecycleError::Integrity(
                    "addon changed before its retirement commit".into(),
                ));
            }
        }
        boundary("before-commit", &transaction)?;
        if replacement.is_some() {
            rename_no_replace(
                &transaction.join("replacement"),
                &root.join(super::SUBFOLDER),
            )?;
            replacement_committed = true;
            boundary("after-commit", &transaction)?;
        }
        write_synced(&transaction.join(COMMITTED_FILE), COMMITTED_BYTES)?;
        Ok(())
    })();
    if let Err(error) = result {
        if replacement_committed {
            return Err(LifecycleError::CleanupPending {
                status: inspect_locked(root).unwrap_or(BeaconStatus::Unmanaged),
                recovery: transaction,
                message: error.to_string(),
            });
        }
        if let Err(recovery) = recover_transaction(root, &transaction) {
            return Err(LifecycleError::RecoveryPending {
                recovery: transaction,
                message: format!("{error}; recovery failed: {recovery}"),
            });
        }
        return Err(error);
    }
    let committed_status = if replacement.is_some() {
        BeaconStatus::ManagedUpToDate
    } else {
        BeaconStatus::NotInstalled
    };
    if let Err(error) = boundary("before-cleanup", &transaction)
        .and_then(|()| cleanup(root, &transaction, &receipt))
    {
        return Err(LifecycleError::CleanupPending {
            status: committed_status,
            recovery: transaction,
            message: error.to_string(),
        });
    }
    Ok(())
}

/// Recover only bounded journals written by this lifecycle; unrelated files stay intact.
pub fn recover(addons_root: &Path) -> Result<(), LifecycleError> {
    mutation::with_root(addons_root, recover_locked)
}

fn recover_locked(root: &Path) -> Result<(), LifecycleError> {
    let mut candidates = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        if entry
            .file_name()
            .to_string_lossy()
            .starts_with(TRANSACTION_PREFIX)
        {
            let value = fs::symlink_metadata(entry.path())?;
            if value.is_dir()
                && !is_link_like(&value)
                && metadata(&entry.path().join(RECEIPT_FILE))?.is_some()
            {
                candidates.push(entry.path());
            }
        }
    }
    candidates.sort();
    for transaction in candidates {
        recover_transaction(root, &transaction)?;
    }
    configuration::recover_all(root)?;
    Ok(())
}

fn receipt(root: &Path, transaction: &Path) -> std::io::Result<Receipt> {
    let metadata = fs::symlink_metadata(transaction)?;
    if !metadata.is_dir() || is_link_like(&metadata) {
        return Err(invalid_data("transaction must be a regular directory"));
    }
    let receipt: Receipt = serde_json::from_slice(&read_regular(
        root,
        &transaction.join(RECEIPT_FILE),
        MAX_RECEIPT_BYTES,
    )?)
    .map_err(invalid_data)?;
    if receipt.identity != "PixelBeacon transaction 1"
        || receipt.root != root
        || receipt
            .pixel
            .as_ref()
            .is_some_and(|snapshot| snapshot.kind != Kind::Pixel || !snapshot.valid())
        || receipt
            .data
            .as_ref()
            .is_some_and(|snapshot| snapshot.kind != Kind::LegacyData || !snapshot.valid())
        || receipt
            .replacement
            .as_ref()
            .is_some_and(|snapshot| snapshot.kind != Kind::Pixel || !snapshot.valid())
    {
        return Err(invalid_data(
            "transaction receipt does not belong to this AddOns root",
        ));
    }
    Ok(receipt)
}

fn recover_transaction(root: &Path, transaction: &Path) -> Result<(), LifecycleError> {
    let receipt = receipt(root, transaction)?;
    let explicit_commit = match metadata(&transaction.join(COMMITTED_FILE))? {
        Some(_) => {
            read_regular(
                root,
                &transaction.join(COMMITTED_FILE),
                COMMITTED_BYTES.len() as u64,
            )? == COMMITTED_BYTES
        }
        None => false,
    };
    let inferred_commit = if let Some(expected) = &receipt.replacement {
        metadata(&transaction.join("replacement"))?.is_none()
            && matches!(snapshot(root, &root.join(super::SUBFOLDER), Kind::Pixel, false), Ok(Some(ref actual)) if actual.receipt() == *expected)
    } else {
        false
    };
    if explicit_commit || inferred_commit {
        let status = if receipt.replacement.is_some() {
            BeaconStatus::ManagedUpToDate
        } else {
            BeaconStatus::NotInstalled
        };
        return cleanup(root, transaction, &receipt).map_err(|error| {
            LifecycleError::CleanupPending {
                status,
                recovery: transaction.to_owned(),
                message: error.to_string(),
            }
        });
    }
    for expected in [&receipt.data, &receipt.pixel].into_iter().flatten() {
        let previous = transaction.join(expected.kind.previous());
        if metadata(&previous)?.is_some() {
            // Rollback renames the entire quarantined entry back without deleting
            // or trusting changed contents. A raced target is never replaced.
            rename_no_replace(&previous, &root.join(expected.kind.directory())).map_err(|error| LifecycleError::RecoveryPending { recovery: transaction.to_owned(), message: format!("previous addon could not be restored without replacing a raced target: {error}") })?;
        }
    }
    cleanup(root, transaction, &receipt)?;
    Ok(())
}

fn cleanup(root: &Path, transaction: &Path, receipt: &Receipt) -> std::io::Result<()> {
    for entry in fs::read_dir(transaction)? {
        let name = entry?.file_name();
        if ![
            RECEIPT_FILE,
            COMMITTED_FILE,
            "replacement",
            "previous-pixel",
            "previous-data",
        ]
        .iter()
        .any(|expected| name == *expected)
        {
            return Err(invalid_data(
                "transaction gained an unexpected entry; preserving recovery files",
            ));
        }
    }
    for (directory, expected) in [
        ("replacement", &receipt.replacement),
        ("previous-pixel", &receipt.pixel),
        ("previous-data", &receipt.data),
    ] {
        let directory = transaction.join(directory);
        if metadata(&directory)?.is_none() {
            continue;
        }
        let expected = expected
            .as_ref()
            .ok_or_else(|| invalid_data("unrecorded recovery directory"))?;
        remove_recorded_directory(root, &directory, expected)?;
    }
    if metadata(&transaction.join(COMMITTED_FILE))?.is_some() {
        if read_regular(
            root,
            &transaction.join(COMMITTED_FILE),
            COMMITTED_BYTES.len() as u64,
        )? != COMMITTED_BYTES
        {
            return Err(invalid_data("commit marker changed"));
        }
        fs::remove_file(transaction.join(COMMITTED_FILE))?;
    }
    fs::remove_file(transaction.join(RECEIPT_FILE))?;
    fs::remove_dir(transaction)
}

fn remove_recorded_directory(
    root: &Path,
    directory: &Path,
    expected: &ReceiptSnapshot,
) -> std::io::Result<()> {
    let value = fs::symlink_metadata(directory)?;
    if !value.is_dir() || is_link_like(&value) {
        return Err(invalid_data("recovery directory changed shape"));
    }
    let mut paths = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| invalid_data("invalid recovery filename"))?;
        let expected_hash = expected
            .files
            .iter()
            .find(|(file, _)| file == &name)
            .map(|(_, hash)| hash)
            .ok_or_else(|| invalid_data("unexpected recovery file"))?;
        let bytes = read_regular(root, &entry.path(), MAX_FILE_BYTES)?;
        if digest(&bytes) != *expected_hash {
            return Err(invalid_data("recovery file changed; preserving it"));
        }
        paths.push(entry.path());
    }
    if !paths.is_empty() {
        let manifest = read_regular(
            root,
            &directory.join(expected.kind.manifest()),
            MAX_MANIFEST_BYTES,
        )?;
        let manifest = std::str::from_utf8(&manifest).map_err(invalid_data)?;
        if !manifest
            .lines()
            .any(|line| line.trim() == expected.kind.marker())
        {
            return Err(invalid_data(
                "recovery package has no verified ownership marker",
            ));
        }
        if expected.kind == Kind::Pixel
            && !manifest.lines().any(|line| line.trim() == PACKAGE_IDENTITY)
            && paths.iter().any(|path| {
                path.file_name()
                    .is_none_or(|name| name != super::MANIFEST_FILE && name != super::LUA_FILE)
            })
        {
            return Err(invalid_data(
                "legacy marker cannot authorize a combined inventory",
            ));
        }
    }
    // A restart after partial cleanup may see a strict subset. Each remaining
    // byte must still match its receipt, and deletion never traverses subtrees.
    paths.sort_by_key(|path| {
        path.file_name()
            .is_some_and(|name| name == expected.kind.manifest())
    });
    for path in paths {
        fs::remove_file(path)?;
    }
    fs::remove_dir(directory)
}

#[cfg(target_os = "linux")]
fn rename_no_replace(source: &Path, destination: &Path) -> std::io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let source = CString::new(source.as_os_str().as_bytes()).map_err(invalid_data)?;
    let destination = CString::new(destination.as_os_str().as_bytes()).map_err(invalid_data)?;
    // SAFETY: both C strings are NUL-terminated and alive throughout the call.
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
fn rename_no_replace(source: &Path, destination: &Path) -> std::io::Result<()> {
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
    // SAFETY: both buffers are NUL-terminated and alive throughout the call.
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
fn rename_no_replace(_source: &Path, _destination: &Path) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "package lifecycle supports Windows and Linux",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn inspection_file_handle_cannot_race_package_retirement() {
        use std::sync::mpsc;
        use std::time::Duration;

        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        install_configured(
            &root,
            RunningState::Unknown,
            101051,
            5,
            false,
            Environment::Live,
        )
        .unwrap();
        let (opened, opened_wait) = mpsc::channel();
        let (release, release_wait) = mpsc::channel();
        let inspected_root = root.clone();
        let reader = std::thread::spawn(move || {
            inspect_presence_with(&inspected_root, |root| {
                // Hold the production bounded reader's actual Windows handle
                // across the point where another thread requests retirement.
                let handle = crate::bounded_file::open_bounded_stable(
                    root,
                    &root.join("PixelBeacon/PixelBeacon.txt"),
                    MAX_MANIFEST_BYTES,
                )
                .unwrap();
                opened.send(()).unwrap();
                release_wait.recv().unwrap();
                drop(handle);
            })
        });
        opened_wait.recv_timeout(Duration::from_secs(5)).unwrap();
        let (attempted, attempted_wait) = mpsc::channel();
        let (completed, completed_wait) = mpsc::channel();
        let uninstall_root = root.join(".");
        let remover = std::thread::spawn(move || {
            attempted.send(()).unwrap();
            let result = uninstall(&uninstall_root, RunningState::Unknown);
            completed.send(()).unwrap();
            result
        });
        attempted_wait.recv_timeout(Duration::from_secs(5)).unwrap();
        let removed_while_reading = completed_wait
            .recv_timeout(Duration::from_millis(150))
            .is_ok();
        release.send(()).unwrap();
        let read_result = reader.join().unwrap();
        let uninstall_result = remover.join().unwrap();
        assert!(
            !removed_while_reading,
            "retirement ran while inspection held an open package file: {uninstall_result:?}"
        );
        assert!(read_result.unwrap().eligible());
        assert_eq!(uninstall_result.unwrap().status, BeaconStatus::NotInstalled);
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
    }

    #[test]
    fn background_recovery_waits_for_active_uninstall_cleanup() {
        use std::sync::mpsc;
        use std::time::Duration;

        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        install_configured(
            &root,
            RunningState::Unknown,
            101051,
            5,
            false,
            Environment::Live,
        )
        .unwrap();
        let (cleanup_entered, cleanup_wait) = mpsc::channel();
        let (release_cleanup, cleanup_release) = mpsc::channel();
        let uninstall_root = root.clone();
        let uninstall = std::thread::spawn(move || {
            uninstall_with(&uninstall_root, RunningState::Unknown, |boundary, _| {
                if boundary == "before-cleanup" {
                    cleanup_entered.send(()).unwrap();
                    cleanup_release.recv().unwrap();
                }
                Ok(())
            })
        });
        cleanup_wait.recv_timeout(Duration::from_secs(5)).unwrap();
        let (attempted, attempt_wait) = mpsc::channel();
        let (completed, completion_wait) = mpsc::channel();
        // A different path spelling must share the same canonical root lock.
        let recovery_root = root.join(".");
        let recovery = std::thread::spawn(move || {
            attempted.send(()).unwrap();
            let result = recover(&recovery_root);
            completed.send(()).unwrap();
            result
        });
        attempt_wait.recv_timeout(Duration::from_secs(5)).unwrap();
        let recovered_during_uninstall = completion_wait
            .recv_timeout(Duration::from_millis(150))
            .is_ok();
        release_cleanup.send(()).unwrap();
        let uninstall_result = uninstall.join().unwrap();
        let recovery_result = recovery.join().unwrap();
        assert!(
            !recovered_during_uninstall,
            "background recovery entered an active uninstall journal: {uninstall_result:?}"
        );
        assert_eq!(uninstall_result.unwrap().status, BeaconStatus::NotInstalled);
        recovery_result.unwrap();
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
    }

    fn legacy(root: &Path, kind: Kind) -> Snapshot {
        let directory = root.join(kind.directory());
        fs::create_dir(&directory).unwrap();
        for file in kind.inventory() {
            if kind == Kind::Pixel
                && *file != super::super::LUA_FILE
                && *file != super::super::MANIFEST_FILE
            {
                continue;
            }
            let bytes = if *file == kind.manifest() {
                format!(
                    "## Title: {}\n## Version: {}\n{}\n",
                    kind.directory(),
                    if kind == Kind::Pixel { 23 } else { 2 },
                    kind.marker()
                )
            } else {
                format!("-- retained {file}\nlocal BLOCK_PX = 5\n")
            };
            fs::write(directory.join(file), bytes).unwrap();
        }
        snapshot(root, &directory, kind, false).unwrap().unwrap()
    }

    fn replacement() -> Snapshot {
        expected_snapshot(
            5,
            StartupConfig {
                catalog_disabled: true,
                environment: Environment::Live,
            },
        )
    }

    #[test]
    fn every_precommit_failure_restores_both_packages_and_neighbor_bytes() {
        for failed in [
            "before-staging",
            "before-retirement",
            "after-pixel-retirement",
            "after-data-retirement",
            "before-commit",
        ] {
            let sandbox = tempfile::tempdir().unwrap();
            let root = fs::canonicalize(sandbox.path()).unwrap();
            let pixel = legacy(&root, Kind::Pixel);
            let data = legacy(&root, Kind::LegacyData);
            fs::write(root.join("neighbor.bin"), b"unchanged").unwrap();
            let result = transact(&root, Some(replacement()), |boundary, _| {
                if boundary == failed {
                    Err(std::io::Error::other("injected transaction failure"))
                } else {
                    Ok(())
                }
            });
            assert!(result.is_err(), "{failed}");
            assert_eq!(
                snapshot(
                    &root,
                    &root.join(super::super::SUBFOLDER),
                    Kind::Pixel,
                    false
                )
                .unwrap()
                .unwrap(),
                pixel,
                "{failed}"
            );
            assert_eq!(
                snapshot(
                    &root,
                    &root.join(LEGACY_DATA_SUBFOLDER),
                    Kind::LegacyData,
                    false
                )
                .unwrap()
                .unwrap(),
                data,
                "{failed}"
            );
            assert_eq!(fs::read(root.join("neighbor.bin")).unwrap(), b"unchanged");
            assert_eq!(fs::read_dir(&root).unwrap().count(), 3, "{failed}");
        }
    }

    #[test]
    fn successful_commit_distinguishes_cleanup_failure_and_retry_recovers() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        legacy(&root, Kind::Pixel);
        legacy(&root, Kind::LegacyData);
        let error = transact(&root, Some(replacement()), |boundary, _| {
            if boundary == "before-cleanup" {
                Err(std::io::Error::other("injected cleanup failure"))
            } else {
                Ok(())
            }
        })
        .unwrap_err();
        assert!(matches!(
            error,
            LifecycleError::CleanupPending {
                status: BeaconStatus::ManagedUpToDate,
                ..
            }
        ));
        assert!(inspect_presence(&root).unwrap().eligible());
        assert_eq!(inspect(&root).unwrap(), BeaconStatus::ManagedUpToDate);
        assert!(!root.join(LEGACY_DATA_SUBFOLDER).exists());
        recover(&root).unwrap();
        assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    }

    #[test]
    fn post_rename_failure_before_commit_marker_reports_installed_state_and_preserves_previous_bytes(
    ) {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let previous = legacy(&root, Kind::Pixel);
        let result = transact(&root, Some(replacement()), |boundary, _| {
            if boundary == "after-commit" {
                Err(std::io::Error::other("injected commit marker failure"))
            } else {
                Ok(())
            }
        });
        let recovery = match result {
            Err(LifecycleError::CleanupPending {
                status: BeaconStatus::ManagedUpToDate,
                recovery,
                ..
            }) => recovery,
            other => panic!("unexpected {other:?}"),
        };
        assert!(inspect_presence(&root).unwrap().eligible());
        assert!(!recovery.join(COMMITTED_FILE).exists());
        assert_eq!(
            snapshot(&root, &recovery.join("previous-pixel"), Kind::Pixel, false)
                .unwrap()
                .unwrap(),
            previous
        );
        recover(&root).unwrap();
        assert!(inspect_presence(&root).unwrap().eligible());
        assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    }

    #[test]
    fn raced_commit_target_is_preserved_with_previous_bytes_until_recovery() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let pixel = legacy(&root, Kind::Pixel);
        let data = legacy(&root, Kind::LegacyData);
        let result = transact(&root, Some(replacement()), |boundary, _| {
            if boundary == "before-commit" {
                fs::create_dir(root.join(super::super::SUBFOLDER))?;
                fs::write(
                    root.join(super::super::SUBFOLDER).join("foreign.txt"),
                    b"foreign raced target",
                )?;
            }
            Ok(())
        });
        assert!(matches!(
            result,
            Err(LifecycleError::RecoveryPending { .. })
        ));
        assert_eq!(
            fs::read(root.join(super::super::SUBFOLDER).join("foreign.txt")).unwrap(),
            b"foreign raced target"
        );
        fs::rename(
            root.join(super::super::SUBFOLDER),
            root.join("foreign-preserved"),
        )
        .unwrap();
        recover(&root).unwrap();
        assert_eq!(
            snapshot(
                &root,
                &root.join(super::super::SUBFOLDER),
                Kind::Pixel,
                false
            )
            .unwrap()
            .unwrap(),
            pixel
        );
        assert_eq!(
            snapshot(
                &root,
                &root.join(LEGACY_DATA_SUBFOLDER),
                Kind::LegacyData,
                false
            )
            .unwrap()
            .unwrap(),
            data
        );
        assert_eq!(
            fs::read(root.join("foreign-preserved/foreign.txt")).unwrap(),
            b"foreign raced target"
        );
    }

    #[test]
    fn directory_swap_before_retirement_restores_the_swapped_entry_without_deleting_it() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let original = legacy(&root, Kind::Pixel);
        let result = transact(&root, Some(replacement()), |boundary, _| {
            if boundary == "before-retirement" {
                fs::rename(root.join(super::super::SUBFOLDER), root.join("original"))?;
                fs::create_dir(root.join(super::super::SUBFOLDER))?;
                fs::write(
                    root.join(super::super::SUBFOLDER).join("unknown"),
                    b"preserve swapped files",
                )?;
            }
            Ok(())
        });
        assert!(result.is_err());
        assert_eq!(
            fs::read(root.join(super::super::SUBFOLDER).join("unknown")).unwrap(),
            b"preserve swapped files"
        );
        assert_eq!(
            snapshot(&root, &root.join("original"), Kind::Pixel, false)
                .unwrap()
                .unwrap(),
            original
        );
    }

    #[test]
    fn changed_file_after_retirement_is_restored_intact_and_replacement_is_not_committed() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        legacy(&root, Kind::Pixel);
        let result = transact(&root, Some(replacement()), |boundary, transaction| {
            if boundary == "after-pixel-retirement" {
                fs::write(
                    transaction.join("previous-pixel/PixelBeacon.lua"),
                    b"changed during retirement",
                )?;
            }
            Ok(())
        });
        assert!(result.is_err());
        assert_eq!(
            fs::read(root.join("PixelBeacon/PixelBeacon.lua")).unwrap(),
            b"changed during retirement"
        );
        assert!(!inspect_presence(&root).unwrap().eligible());
    }

    fn interrupted(root: &Path, commit: bool) {
        let pixel = legacy(root, Kind::Pixel);
        let data = legacy(root, Kind::LegacyData);
        let replacement = replacement();
        let transaction = tempfile::Builder::new()
            .prefix(TRANSACTION_PREFIX)
            .tempdir_in(root)
            .unwrap()
            .keep();
        let receipt = Receipt {
            identity: "PixelBeacon transaction 1".into(),
            root: root.to_owned(),
            pixel: Some(pixel.receipt()),
            data: Some(data.receipt()),
            replacement: Some(replacement.receipt()),
        };
        write_synced(
            &transaction.join(RECEIPT_FILE),
            &serde_json::to_vec(&receipt).unwrap(),
        )
        .unwrap();
        fs::create_dir(transaction.join("replacement")).unwrap();
        for (name, bytes) in replacement.files {
            write_synced(&transaction.join("replacement").join(name), &bytes).unwrap();
        }
        rename_no_replace(
            &root.join(Kind::Pixel.directory()),
            &transaction.join(Kind::Pixel.previous()),
        )
        .unwrap();
        rename_no_replace(
            &root.join(Kind::LegacyData.directory()),
            &transaction.join(Kind::LegacyData.previous()),
        )
        .unwrap();
        if commit {
            rename_no_replace(
                &transaction.join("replacement"),
                &root.join(super::super::SUBFOLDER),
            )
            .unwrap();
        }
        recover(root).unwrap();
        if commit {
            assert!(inspect_presence(root).unwrap().eligible());
            assert!(!root.join(LEGACY_DATA_SUBFOLDER).exists());
            assert_eq!(fs::read_dir(root).unwrap().count(), 1);
        } else {
            assert_eq!(
                snapshot(
                    root,
                    &root.join(Kind::Pixel.directory()),
                    Kind::Pixel,
                    false
                )
                .unwrap()
                .unwrap(),
                pixel
            );
            assert_eq!(
                snapshot(
                    root,
                    &root.join(Kind::LegacyData.directory()),
                    Kind::LegacyData,
                    false
                )
                .unwrap()
                .unwrap(),
                data
            );
            assert_eq!(fs::read_dir(root).unwrap().count(), 2);
        }
    }

    #[test]
    fn interrupted_before_commit_rolls_back_and_after_commit_finishes_exact_cleanup() {
        for commit in [false, true] {
            let sandbox = tempfile::tempdir().unwrap();
            interrupted(&fs::canonicalize(sandbox.path()).unwrap(), commit);
        }
    }

    #[test]
    fn cleanup_refuses_new_or_changed_recovery_files_after_committing() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        legacy(&root, Kind::Pixel);
        let result = transact(&root, Some(replacement()), |boundary, transaction| {
            if boundary == "before-cleanup" {
                fs::write(
                    transaction.join("previous-pixel/user-file"),
                    b"retain unknown bytes",
                )?;
            }
            Ok(())
        });
        let recovery = match result {
            Err(LifecycleError::CleanupPending { recovery, .. }) => recovery,
            other => panic!("unexpected {other:?}"),
        };
        assert_eq!(
            fs::read(recovery.join("previous-pixel/user-file")).unwrap(),
            b"retain unknown bytes"
        );
        assert!(recover(&root).is_err());
        assert!(inspect_presence(&root).unwrap().eligible());
        assert_eq!(
            fs::read(recovery.join("previous-pixel/user-file")).unwrap(),
            b"retain unknown bytes"
        );
    }

    #[test]
    fn arbitrary_prefix_directory_and_forged_receipt_never_authorize_cleanup() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        let unknown = root.join(format!("{TRANSACTION_PREFIX}foreign"));
        fs::create_dir(&unknown).unwrap();
        fs::write(unknown.join("keep"), b"foreign").unwrap();
        recover(&root).unwrap();
        assert_eq!(fs::read(unknown.join("keep")).unwrap(), b"foreign");
        fs::write(unknown.join(RECEIPT_FILE), b"{}").unwrap();
        assert!(recover(&root).is_err());
        assert_eq!(fs::read(unknown.join("keep")).unwrap(), b"foreign");
    }

    #[cfg(unix)]
    #[test]
    fn linked_root_package_file_and_recovery_entries_are_never_followed() {
        use std::os::unix::fs::symlink;
        let sandbox = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let root = fs::canonicalize(sandbox.path()).unwrap();
        symlink(outside.path(), root.join(super::super::SUBFOLDER)).unwrap();
        assert_eq!(inspect(&root).unwrap(), BeaconStatus::Unmanaged);
        assert!(!inspect_presence(&root).unwrap().eligible());
        assert!(install_configured(
            &root,
            RunningState::Unknown,
            101051,
            3,
            false,
            Environment::Live
        )
        .is_err());
        fs::remove_file(root.join(super::super::SUBFOLDER)).unwrap();
        legacy(&root, Kind::Pixel);
        fs::write(outside.path().join("keep"), b"unchanged").unwrap();
        fs::remove_file(root.join("PixelBeacon/PixelBeacon.lua")).unwrap();
        symlink(
            outside.path().join("keep"),
            root.join("PixelBeacon/PixelBeacon.lua"),
        )
        .unwrap();
        assert!(uninstall(&root, RunningState::Unknown).is_err());
        assert_eq!(fs::read(outside.path().join("keep")).unwrap(), b"unchanged");
    }
}
