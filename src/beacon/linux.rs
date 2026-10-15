//! Linux (Steam Proton) AddOns discovery and the ESO running-game probe.
//!
//! Discovery parses `libraryfolders.vdf` to find the library holding the ESO
//! app id, then resolves the compatdata Documents path. The probe scans `/proc`
//! and returns [`crate::beacon::RunningState::Unknown`] on any failure.

use std::path::PathBuf;

use super::Environment;

/// Resolves the AddOns directory under the ESO Proton prefix.
pub fn addons_dir(env: Environment) -> Option<PathBuf> {
    addons_dirs(env).into_iter().next()
}

pub fn addons_dirs(env: Environment) -> Vec<PathBuf> {
    let Some(home) = dirs::home_dir() else {
        return Vec::new();
    };
    let candidates = [
        home.join(".steam/steam"),
        home.join(".local/share/Steam"),
        home.join(".steam/root"),
        home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
    ];
    super::addons_dirs_in_steam_roots(&candidates, env)
}
