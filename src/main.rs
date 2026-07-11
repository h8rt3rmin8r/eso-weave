//! ESO Weave binary entry point.
//!
//! For the foundations slice this wires the Config Store and Logging together:
//! resolve platform directories, load settings, initialize logging from the
//! loaded preferences, emit any load notices, log a startup line, and exit.

use eso_weave::config::{self, LoadOutcome};
use eso_weave::{logging, platform, version};

fn main() {
    let outcome = match platform::config_dir() {
        Some(dir) => config::load(&dir),
        None => LoadOutcome::default(),
    };

    let log_dir = platform::log_dir().unwrap_or_default();
    let _handle = logging::init(&outcome.settings.logging, log_dir);

    for notice in &outcome.notices {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }

    tracing::info!(
        target: "eso_weave",
        "eso-weave {} started (schema_version={})",
        version(),
        outcome.settings.schema_version
    );
}
