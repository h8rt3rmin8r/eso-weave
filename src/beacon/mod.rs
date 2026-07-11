//! Beacon Manager: the on-disk lifecycle of the embedded PixelBeacon addon.
//!
//! The addon files are embedded with [`MANIFEST`] and [`LUA`], and the embedded
//! version is single-sourced by parsing the embedded manifest. The correctness
//! and safety logic (manifest parsing, four-state classification, install with
//! subtree confinement, and the marker-gated [`uninstall`]) is pure and fully
//! tested against an injected AddOns root. Discovery of that root and the
//! running-game probe sit behind thin per-platform backends.

#[cfg(target_os = "linux")]
mod linux;
pub mod steam;
#[cfg(windows)]
mod windows;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// The addon subfolder written under the AddOns directory.
pub const SUBFOLDER: &str = "PixelBeacon";
/// The addon manifest file name.
pub const MANIFEST_FILE: &str = "PixelBeacon.txt";
/// The addon Lua file name.
pub const LUA_FILE: &str = "PixelBeacon.lua";
/// The exact managed-marker line that gates uninstall.
pub const MANAGED_MARKER: &str = "## X-ESO-Weave-Managed: true";
/// The Steam app id for The Elder Scrolls Online.
pub const ESO_APP_ID: &str = "306130";

/// The canonical embedded addon manifest, shipped verbatim by install.
pub const MANIFEST: &str = include_str!("../../addon/PixelBeacon/PixelBeacon.txt");
/// The canonical embedded addon Lua, shipped verbatim by install.
pub const LUA: &str = include_str!("../../addon/PixelBeacon/PixelBeacon.lua");

/// The ESO game environment selecting the AddOns subdirectory.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    /// The default `live` environment.
    #[default]
    Live,
    /// The public test server environment.
    Pts,
}

impl Environment {
    /// The directory segment for this environment (`"live"` or `"pts"`).
    pub fn segment(self) -> &'static str {
        match self {
            Environment::Live => "live",
            Environment::Pts => "pts",
        }
    }
}

/// The beacon module's view of its opaque `beacon` settings section.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct BeaconPrefs {
    /// A manual AddOns directory override; when set it takes precedence over
    /// auto-discovery.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_override: Option<PathBuf>,
    /// The selected game environment.
    #[serde(default)]
    pub environment: Environment,
}

/// Reads [`BeaconPrefs`] from the opaque `beacon` settings value. An absent or
/// null value, or a malformed section, yields defaults.
pub fn prefs_from_value(value: &serde_json::Value) -> BeaconPrefs {
    if value.is_null() {
        return BeaconPrefs::default();
    }
    serde_json::from_value(value.clone()).unwrap_or_default()
}

/// Serializes [`BeaconPrefs`] to the opaque `beacon` settings value.
pub fn prefs_to_value(prefs: &BeaconPrefs) -> serde_json::Value {
    serde_json::to_value(prefs).unwrap_or(serde_json::Value::Null)
}

/// The classified installed state of the beacon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeaconStatus {
    /// No `PixelBeacon` folder, or the folder has no readable manifest.
    NotInstalled,
    /// Folder present, marker line present, and the installed version equals the
    /// embedded version.
    ManagedUpToDate,
    /// Folder present and marker line present, but the installed version differs
    /// from (or cannot be read against) the embedded version.
    ManagedVersionMismatch,
    /// Folder present with a readable manifest that lacks the marker line.
    Unmanaged,
}

/// A best-effort signal of whether the ESO client is running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunningState {
    /// The ESO client was detected.
    Running,
    /// The ESO client was not detected.
    NotRunning,
    /// The running state could not be determined.
    Unknown,
}

/// A failure to resolve the AddOns directory.
#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryError {
    /// Auto-discovery failed and no usable override was set.
    #[error("the ESO AddOns directory could not be found")]
    NotFound,
    /// The platform has no discovery backend.
    #[error("no AddOns discovery backend for this platform")]
    Unsupported,
}

/// The successful result of an install or uninstall.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleOutcome {
    /// The beacon status after the operation.
    pub status: BeaconStatus,
    /// Whether a `/reloadui` or relog reminder applies.
    pub reload_required: bool,
}

/// A typed failure from an install or uninstall.
#[derive(thiserror::Error, Debug)]
pub enum LifecycleError {
    /// The resolved AddOns directory is not an existing directory.
    #[error("the resolved AddOns directory does not exist")]
    AddonsDirMissing,
    /// Uninstall refused because the on-disk manifest lacks the marker line.
    #[error("refusing to remove an unmanaged PixelBeacon folder")]
    Unmanaged,
    /// A filesystem operation failed.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Returns true when some line of `manifest`, trimmed, is exactly the
/// managed-marker line.
pub fn has_managed_marker(manifest: &str) -> bool {
    manifest.lines().any(|line| line.trim() == MANAGED_MARKER)
}

/// Parses the value of the first `## Version:` line as a `u32`, or `None` when
/// it is absent or unparsable.
pub fn parse_manifest_version(manifest: &str) -> Option<u32> {
    for line in manifest.lines() {
        if let Some(rest) = line.trim().strip_prefix("## Version:") {
            return rest.trim().parse::<u32>().ok();
        }
    }
    None
}

/// The embedded addon version, single-sourced from the embedded manifest.
pub fn embedded_version() -> u32 {
    parse_manifest_version(MANIFEST).expect("embedded manifest carries a parseable ## Version:")
}

/// The reload reminder rule: true when the game is running or the state is
/// unknown (fail safe toward reminding), false when it is not running.
pub fn reload_reminder(state: RunningState) -> bool {
    matches!(state, RunningState::Running | RunningState::Unknown)
}

/// The ESO AddOns subpath relative to a Documents directory
/// (`Elder Scrolls Online/<env>/AddOns`).
pub fn eso_addons_subpath(env: Environment) -> PathBuf {
    Path::new("Elder Scrolls Online")
        .join(env.segment())
        .join("AddOns")
}

/// Composes the AddOns directory under a resolved Documents directory.
pub fn addons_dir_under_documents(documents: &Path, env: Environment) -> PathBuf {
    documents.join(eso_addons_subpath(env))
}

/// Classifies the installed beacon status under `addons_root`. Reads only.
pub fn status(addons_root: &Path) -> BeaconStatus {
    let manifest_path = addons_root.join(SUBFOLDER).join(MANIFEST_FILE);
    let manifest = match std::fs::read_to_string(&manifest_path) {
        Ok(text) => text,
        Err(_) => return BeaconStatus::NotInstalled,
    };
    if !has_managed_marker(&manifest) {
        return BeaconStatus::Unmanaged;
    }
    match parse_manifest_version(&manifest) {
        Some(version) if version == embedded_version() => BeaconStatus::ManagedUpToDate,
        _ => BeaconStatus::ManagedVersionMismatch,
    }
}

/// Installs (or safely updates) the embedded addon into `addons_root`.
///
/// The AddOns root must already exist; only the `PixelBeacon` subfolder is
/// created and populated. Every write is confined to that subfolder.
pub fn install(
    addons_root: &Path,
    running: RunningState,
) -> Result<LifecycleOutcome, LifecycleError> {
    if !addons_root.is_dir() {
        tracing::warn!(
            target: "beacon",
            path = %addons_root.display(),
            "install refused: AddOns directory does not exist"
        );
        return Err(LifecycleError::AddonsDirMissing);
    }
    let dir = addons_root.join(SUBFOLDER);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(MANIFEST_FILE), MANIFEST)?;
    std::fs::write(dir.join(LUA_FILE), LUA)?;
    tracing::info!(target: "beacon", path = %dir.display(), "installed PixelBeacon");
    Ok(LifecycleOutcome {
        status: BeaconStatus::ManagedUpToDate,
        reload_required: reload_reminder(running),
    })
}

/// Removes the `PixelBeacon` folder under `addons_root`, but only when the
/// managed-marker line is verified present in the on-disk manifest. A folder
/// without the marker line, or with no manifest, is left untouched.
pub fn uninstall(
    addons_root: &Path,
    running: RunningState,
) -> Result<LifecycleOutcome, LifecycleError> {
    let dir = addons_root.join(SUBFOLDER);
    let manifest_path = dir.join(MANIFEST_FILE);
    let manifest = match std::fs::read_to_string(&manifest_path) {
        Ok(text) => text,
        Err(_) => {
            tracing::warn!(
                target: "beacon",
                path = %dir.display(),
                "uninstall refused: no readable manifest"
            );
            return Err(LifecycleError::Unmanaged);
        }
    };
    if !has_managed_marker(&manifest) {
        tracing::warn!(
            target: "beacon",
            path = %dir.display(),
            "uninstall refused: manifest lacks the managed-marker line"
        );
        return Err(LifecycleError::Unmanaged);
    }
    std::fs::remove_dir_all(&dir)?;
    tracing::info!(target: "beacon", path = %dir.display(), "removed PixelBeacon");
    Ok(LifecycleOutcome {
        status: BeaconStatus::NotInstalled,
        reload_required: reload_reminder(running),
    })
}

/// Resolves the AddOns directory for the given preferences.
///
/// A usable `path_override` (an existing directory) wins. Otherwise the
/// per-platform backend resolves it, joining the ESO environment subpath.
/// Returns [`DiscoveryError::NotFound`] when neither yields a path; never
/// creates a directory.
pub fn resolve_addons_dir(prefs: &BeaconPrefs) -> Result<PathBuf, DiscoveryError> {
    if let Some(over) = &prefs.path_override {
        if over.is_dir() {
            return Ok(over.clone());
        }
        tracing::warn!(
            target: "beacon",
            path = %over.display(),
            "AddOns override path is not an existing directory"
        );
        return Err(DiscoveryError::NotFound);
    }
    match platform_addons_dir(prefs.environment) {
        Some(path) => Ok(path),
        None => {
            tracing::warn!(target: "beacon", "AddOns directory could not be auto-discovered");
            Err(DiscoveryError::NotFound)
        }
    }
}

/// Best-effort probe of whether the ESO client is running. Never panics.
pub fn probe_game_running() -> RunningState {
    platform_probe_game_running()
}

#[cfg(windows)]
fn platform_addons_dir(env: Environment) -> Option<PathBuf> {
    windows::addons_dir(env)
}
#[cfg(target_os = "linux")]
fn platform_addons_dir(env: Environment) -> Option<PathBuf> {
    linux::addons_dir(env)
}
#[cfg(not(any(windows, target_os = "linux")))]
fn platform_addons_dir(_env: Environment) -> Option<PathBuf> {
    None
}

#[cfg(windows)]
fn platform_probe_game_running() -> RunningState {
    windows::probe_game_running()
}
#[cfg(target_os = "linux")]
fn platform_probe_game_running() -> RunningState {
    linux::probe_game_running()
}
#[cfg(not(any(windows, target_os = "linux")))]
fn platform_probe_game_running() -> RunningState {
    RunningState::Unknown
}
