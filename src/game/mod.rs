//! Truthful ESO installation, runtime, focus, signal, and surface observations.

#[cfg(target_os = "linux")]
mod linux;
pub mod steam;
#[cfg(windows)]
mod windows;

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::pixelbus::{MenuSurface, WorldState};

/// Steam app id for The Elder Scrolls Online.
pub const ESO_APP_ID: &str = "306130";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstallationProvider {
    EsoStore,
    Steam,
    Epic,
    SteamProton,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateSource {
    GenericUninstall,
    SteamUninstall,
    SteamManifest,
    EpicManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallationCandidate {
    pub provider: InstallationProvider,
    pub root: PathBuf,
    pub source: CandidateSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallationState {
    NotDetected,
    Detected(InstallationCandidate),
    Ambiguous,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    Present,
    Absent,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusObservation {
    Focused,
    Unfocused,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessObservation {
    pub game: Presence,
    pub launcher: Presence,
    pub focus: FocusObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameRuntime {
    Inactive,
    LauncherOpen,
    Active,
    Unknown,
}

impl ProcessObservation {
    pub fn runtime(self) -> GameRuntime {
        match (self.game, self.launcher) {
            (Presence::Present, _) => GameRuntime::Active,
            (Presence::Unknown, _) => GameRuntime::Unknown,
            (Presence::Absent, Presence::Present) => GameRuntime::LauncherOpen,
            (Presence::Absent, Presence::Unknown) => GameRuntime::Unknown,
            (Presence::Absent, Presence::Absent) => GameRuntime::Inactive,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeaconFreshness {
    NeverObserved,
    Fresh,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceObservation {
    Unavailable,
    Observed(MenuSurface),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameContext {
    NotDetected,
    Unfocused,
    Gameplay,
    Surface(MenuSurface),
    SignalUnavailable,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameObservations {
    pub installation: InstallationState,
    pub runtime: GameRuntime,
    pub focus: FocusObservation,
    pub freshness: BeaconFreshness,
    pub surface: SurfaceObservation,
    pub world: WorldState,
}

impl Default for GameObservations {
    fn default() -> Self {
        Self {
            installation: InstallationState::Unknown,
            runtime: GameRuntime::Unknown,
            focus: FocusObservation::Unknown,
            freshness: BeaconFreshness::NeverObserved,
            surface: SurfaceObservation::Unavailable,
            world: WorldState::Unknown,
        }
    }
}

impl GameObservations {
    pub fn context(&self) -> GameContext {
        if self.runtime == GameRuntime::Unknown {
            return GameContext::Unknown;
        }
        if self.runtime != GameRuntime::Active {
            return GameContext::NotDetected;
        }
        match self.focus {
            FocusObservation::Unknown => return GameContext::Unknown,
            FocusObservation::Unfocused => return GameContext::Unfocused,
            FocusObservation::Focused => {}
        }
        if self.freshness != BeaconFreshness::Fresh {
            return GameContext::SignalUnavailable;
        }
        match self.surface {
            SurfaceObservation::Unavailable => GameContext::SignalUnavailable,
            SurfaceObservation::Observed(MenuSurface::None) => GameContext::Gameplay,
            SurfaceObservation::Observed(surface) => GameContext::Surface(surface),
        }
    }
}

#[derive(Clone, Default)]
pub struct GameState(Arc<RwLock<GameObservations>>);

impl GameState {
    pub fn snapshot(&self) -> GameObservations {
        self.0.read().unwrap().clone()
    }

    pub fn update_processes(&self, observation: ProcessObservation) -> bool {
        let mut state = self.0.write().unwrap();
        let runtime = observation.runtime();
        let focus = if runtime == GameRuntime::Active {
            observation.focus
        } else {
            FocusObservation::Unknown
        };
        let changed = state.runtime != runtime || state.focus != focus;
        state.runtime = runtime;
        state.focus = focus;
        if runtime != GameRuntime::Active {
            state.freshness = BeaconFreshness::NeverObserved;
            state.surface = SurfaceObservation::Unavailable;
            state.world = WorldState::Unknown;
        }
        changed
    }

    pub fn update_installation(&self, installation: InstallationState) -> bool {
        let mut state = self.0.write().unwrap();
        let changed = state.installation != installation;
        state.installation = installation;
        changed
    }

    pub fn observe_heartbeat(&self) {
        self.0.write().unwrap().freshness = BeaconFreshness::Fresh;
    }

    pub fn observe_surface(&self, surface: SurfaceObservation) {
        self.0.write().unwrap().surface = surface;
    }

    pub fn observe_world(&self, world: WorldState) {
        self.0.write().unwrap().world = world;
    }

    pub fn signal_lost(&self) {
        let mut state = self.0.write().unwrap();
        state.freshness = BeaconFreshness::Lost;
        state.surface = SurfaceObservation::Unavailable;
        state.world = WorldState::Unknown;
    }
}

fn normalized(path: &Path) -> PathBuf {
    let absolute = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    let mut result = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                result.pop();
            }
            other => result.push(other.as_os_str()),
        }
    }
    result
}

fn strong(provider: InstallationProvider) -> bool {
    provider != InstallationProvider::EsoStore
}

/// Reconciles already validated provider candidates without relying on order.
pub fn reconcile(candidates: Vec<InstallationCandidate>, source_failed: bool) -> InstallationState {
    let mut roots: BTreeMap<String, Vec<InstallationCandidate>> = BTreeMap::new();
    for mut candidate in candidates {
        candidate.root = normalized(&candidate.root);
        let key = candidate.root.to_string_lossy().to_ascii_lowercase();
        roots.entry(key).or_default().push(candidate);
    }
    if roots.is_empty() {
        return if source_failed {
            InstallationState::Unknown
        } else {
            InstallationState::NotDetected
        };
    }
    if roots.len() != 1 {
        return InstallationState::Ambiguous;
    }
    let candidates = roots.into_values().next().unwrap();
    let mut winners: Vec<_> = candidates.iter().filter(|c| strong(c.provider)).collect();
    winners.sort_by_key(|c| c.provider);
    winners.dedup_by_key(|c| c.provider);
    if winners.len() > 1 {
        return InstallationState::Ambiguous;
    }
    InstallationState::Detected(winners.first().copied().unwrap_or(&candidates[0]).clone())
}

/// Validates the stable launcher and client artifacts below an ESO root.
pub fn valid_game_root(root: &Path) -> bool {
    let client = root
        .join("The Elder Scrolls Online")
        .join("game")
        .join("client");
    (client.join("eso64.exe").is_file() || client.join("eso.exe").is_file())
        && root
            .join("Launcher")
            .join("Bethesda.net_Launcher.exe")
            .is_file()
}

/// Discovers validated Steam installations from every supplied Steam root.
/// Source failures are accumulated rather than stopping later roots from being
/// inspected, so native and sandboxed Steam installations can coexist.
pub fn steam_candidates_from_roots(
    roots: impl IntoIterator<Item = PathBuf>,
    provider: InstallationProvider,
) -> (Vec<InstallationCandidate>, bool) {
    let mut candidates = Vec::new();
    let mut failed = false;
    for root in roots {
        let vdf = match std::fs::read_to_string(root.join("steamapps/libraryfolders.vdf")) {
            Ok(vdf) => vdf,
            Err(_) => {
                failed = true;
                continue;
            }
        };
        for library in steam::library_paths_for_app(&vdf, ESO_APP_ID) {
            let manifest = library.join("steamapps/appmanifest_306130.acf");
            match std::fs::read_to_string(manifest)
                .ok()
                .and_then(|text| steam::install_dir_from_manifest(&text))
            {
                Some(directory) => {
                    let game_root = library.join("steamapps/common").join(directory);
                    if valid_game_root(&game_root) {
                        candidates.push(InstallationCandidate {
                            provider,
                            root: game_root,
                            source: CandidateSource::SteamManifest,
                        });
                    }
                }
                None => failed = true,
            }
        }
    }
    (candidates, failed)
}

/// Caps a reader sleep so the next runtime probe is never delayed by a larger
/// configurable sampling interval.
pub fn runtime_probe_delay_ms(reader_interval_ms: u64, now_ms: u64, next_probe_ms: u64) -> u64 {
    reader_interval_ms.min(next_probe_ms.saturating_sub(now_ms))
}

/// Reads an Epic `.item` manifest into a candidate only when it identifies ESO
/// and its recorded root contains the required artifacts.
pub fn epic_candidate(value: &serde_json::Value) -> Option<InstallationCandidate> {
    let display = value
        .get("DisplayName")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let app = value.get("AppName").and_then(|v| v.as_str()).unwrap_or("");
    if !display
        .to_ascii_lowercase()
        .contains("elder scrolls online")
        && !app.to_ascii_lowercase().contains("elder scrolls online")
    {
        return None;
    }
    let root = PathBuf::from(value.get("InstallLocation")?.as_str()?);
    valid_game_root(&root).then_some(InstallationCandidate {
        provider: InstallationProvider::Epic,
        root,
        source: CandidateSource::EpicManifest,
    })
}

pub fn observe_processes() -> ProcessObservation {
    platform_processes()
}

pub fn discover_installation() -> InstallationState {
    let (candidates, failed) = platform_installations();
    reconcile(candidates, failed)
}

#[cfg(windows)]
fn platform_processes() -> ProcessObservation {
    windows::observe_processes()
}
#[cfg(target_os = "linux")]
fn platform_processes() -> ProcessObservation {
    linux::observe_processes()
}
#[cfg(not(any(windows, target_os = "linux")))]
fn platform_processes() -> ProcessObservation {
    ProcessObservation {
        game: Presence::Unknown,
        launcher: Presence::Unknown,
        focus: FocusObservation::Unknown,
    }
}

#[cfg(windows)]
fn platform_installations() -> (Vec<InstallationCandidate>, bool) {
    windows::discover_installations()
}
#[cfg(target_os = "linux")]
fn platform_installations() -> (Vec<InstallationCandidate>, bool) {
    linux::discover_installations()
}
#[cfg(not(any(windows, target_os = "linux")))]
fn platform_installations() -> (Vec<InstallationCandidate>, bool) {
    (Vec::new(), true)
}
