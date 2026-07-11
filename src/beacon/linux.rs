//! Linux (Steam Proton) AddOns discovery and the ESO running-game probe.
//!
//! Discovery parses `libraryfolders.vdf` to find the library holding the ESO
//! app id, then resolves the compatdata Documents path. The probe scans `/proc`
//! and returns [`RunningState::Unknown`] on any failure.

use std::path::PathBuf;

use super::{eso_addons_subpath, steam, Environment, RunningState, ESO_APP_ID};

/// Resolves the AddOns directory under the ESO Proton prefix.
pub fn addons_dir(env: Environment) -> Option<PathBuf> {
    let steam_root = steam_root()?;
    let vdf = std::fs::read_to_string(steam_root.join("steamapps/libraryfolders.vdf")).ok()?;
    let library = steam::library_paths_for_app(&vdf, ESO_APP_ID)
        .into_iter()
        .next()?;
    let documents = library.join(format!(
        "steamapps/compatdata/{ESO_APP_ID}/pfx/drive_c/users/steamuser/Documents"
    ));
    Some(documents.join(eso_addons_subpath(env)))
}

/// Locates a Steam root that contains a `libraryfolders.vdf`.
fn steam_root() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let candidates = [
        home.join(".steam/steam"),
        home.join(".local/share/Steam"),
        home.join(".steam/root"),
        home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
    ];
    candidates
        .into_iter()
        .find(|root| root.join("steamapps/libraryfolders.vdf").is_file())
}

/// The ESO client executable names to match, lowercased.
const ESO_EXE_NAMES: [&str; 2] = ["eso64.exe", "eso.exe"];

/// Scans `/proc` for the ESO client process.
pub fn probe_game_running() -> RunningState {
    let entries = match std::fs::read_dir("/proc") {
        Ok(entries) => entries,
        Err(_) => return RunningState::Unknown,
    };

    let mut walked_any = false;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.is_empty() || !name.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        walked_any = true;
        if let Ok(comm) = std::fs::read_to_string(entry.path().join("comm")) {
            let comm = comm.trim().to_ascii_lowercase();
            if ESO_EXE_NAMES.contains(&comm.as_str()) || comm.starts_with("eso64") {
                return RunningState::Running;
            }
        }
    }

    if walked_any {
        RunningState::NotRunning
    } else {
        RunningState::Unknown
    }
}
