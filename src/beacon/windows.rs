//! Windows AddOns discovery and the ESO running-game probe.
//!
//! Discovery resolves the Documents known folder through the shell API (via the
//! `dirs` crate), never a literal path. The probe is a read-only process
//! snapshot and returns [`crate::beacon::RunningState::Unknown`] on any failure.

use std::path::PathBuf;

use super::{addons_dir_under_documents, Environment};

/// Resolves the AddOns directory under the Documents known folder.
pub fn addons_dir(env: Environment) -> Option<PathBuf> {
    dirs::document_dir().map(|documents| addons_dir_under_documents(&documents, env))
}
