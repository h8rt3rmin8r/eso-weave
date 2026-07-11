//! Windows path specifics: logs live under the roaming app data directory,
//! since Windows has no XDG state directory.

use std::path::PathBuf;

pub fn log_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join(super::APP_DIR).join("logs"))
}
