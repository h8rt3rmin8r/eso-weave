//! Linux (Steam Proton) AddOns discovery and the ESO running-game probe.
//!
//! Discovery parses `libraryfolders.vdf` to find the library holding the ESO
//! app id, then resolves the compatdata Documents path. The probe scans `/proc`
//! and returns [`RunningState::Unknown`] on any failure.

use std::path::PathBuf;

use super::{eso_addons_subpath, Environment, ESO_APP_ID};
use crate::game::steam;

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
