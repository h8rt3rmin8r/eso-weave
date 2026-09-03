//! Linux Steam Proton installation, process, and focus observations.

use std::path::PathBuf;

use super::{
    steam, valid_game_root, CandidateSource, FocusObservation, InstallationCandidate,
    InstallationProvider, Presence, ProcessObservation, ESO_APP_ID,
};

fn steam_root() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    [
        home.join(".steam/steam"),
        home.join(".local/share/Steam"),
        home.join(".steam/root"),
        home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
    ]
    .into_iter()
    .find(|root| root.join("steamapps/libraryfolders.vdf").is_file())
}

pub fn discover_installations() -> (Vec<InstallationCandidate>, bool) {
    let Some(root) = steam_root() else {
        return (Vec::new(), false);
    };
    let vdf = match std::fs::read_to_string(root.join("steamapps/libraryfolders.vdf")) {
        Ok(vdf) => vdf,
        Err(_) => return (Vec::new(), true),
    };
    let mut candidates = Vec::new();
    let mut failed = false;
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
                        provider: InstallationProvider::SteamProton,
                        root: game_root,
                        source: CandidateSource::SteamManifest,
                    });
                }
            }
            None => failed = true,
        }
    }
    (candidates, failed)
}

pub fn observe_processes() -> ProcessObservation {
    let entries = match std::fs::read_dir("/proc") {
        Ok(entries) => entries,
        Err(_) => {
            return ProcessObservation {
                game: Presence::Unknown,
                launcher: Presence::Unknown,
                focus: FocusObservation::Unknown,
            }
        }
    };
    let mut walked = false;
    let mut game = false;
    let mut launcher = false;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.is_empty() || !name.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        walked = true;
        if let Ok(comm) = std::fs::read_to_string(entry.path().join("comm")) {
            let comm = comm.trim().to_ascii_lowercase();
            game |= matches!(comm.as_str(), "eso64.exe" | "eso.exe") || comm.starts_with("eso64");
            launcher |= comm.starts_with("bethesda.net_l");
        }
    }
    if !walked {
        return ProcessObservation {
            game: Presence::Unknown,
            launcher: Presence::Unknown,
            focus: FocusObservation::Unknown,
        };
    }
    ProcessObservation {
        game: if game {
            Presence::Present
        } else {
            Presence::Absent
        },
        launcher: if launcher {
            Presence::Present
        } else {
            Presence::Absent
        },
        focus: if game {
            crate::platform::active_window_title()
                .map(|title| {
                    if title.contains("Elder Scrolls Online") {
                        FocusObservation::Focused
                    } else {
                        FocusObservation::Unfocused
                    }
                })
                .unwrap_or(FocusObservation::Unknown)
        } else {
            FocusObservation::Unknown
        },
    }
}
