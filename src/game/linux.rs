//! Linux Steam Proton installation, process, and focus observations.

use std::path::PathBuf;

use super::{
    steam_candidates_from_roots, FocusObservation, InstallationCandidate, InstallationProvider,
    Presence, ProcessObservation,
};

fn steam_roots() -> Vec<PathBuf> {
    let Some(home) = dirs::home_dir() else {
        return Vec::new();
    };
    [
        home.join(".steam/steam"),
        home.join(".local/share/Steam"),
        home.join(".steam/root"),
        home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
    ]
    .into_iter()
    .filter(|root| root.join("steamapps/libraryfolders.vdf").is_file())
    .collect()
}

pub fn discover_installations() -> (Vec<InstallationCandidate>, bool) {
    let roots = steam_roots();
    if roots.is_empty() {
        return (Vec::new(), false);
    }
    steam_candidates_from_roots(roots, InstallationProvider::SteamProton)
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
