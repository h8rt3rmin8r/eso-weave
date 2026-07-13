//! ESO API version check: the startup automation that keeps the PixelBeacon
//! manifest API version current.
//!
//! The exact numeric API version is only published behind bot challenges a plain
//! client cannot pass, so the network fetch reads the live game client version
//! string from the official esoui/esoui GitHub live branch and uses it purely to
//! detect that a client API change shipped. The numeric value written into the
//! manifest resolves locally as the maximum of the stored last known value and the
//! compiled [`super::DEFAULT_API_VERSION`]. The networked source sits behind the
//! [`GameVersionSource`] seam; [`run_check`] and the parser are pure and tested
//! against a mock source and an injected AddOns root, and never panic.

use std::fmt;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::{
    has_managed_marker, parse_api_version_primary, rewrite_api_version, DEFAULT_API_VERSION,
    DEFAULT_GAME_VERSION, MANIFEST_FILE, SUBFOLDER,
};

/// A parsed, comparable ESO game client version, held as four numeric components
/// (left-aligned, zero-padded) so it stays `Copy` for the session-state cache.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GameVersion([u16; 4]);

impl GameVersion {
    /// Constructs a version from its four components.
    pub const fn new(parts: [u16; 4]) -> Self {
        Self(parts)
    }
}

impl fmt::Display for GameVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut end = 3;
        while end > 1 && self.0[end] == 0 {
            end -= 1;
        }
        let parts: Vec<String> = self.0[..=end].iter().map(|p| p.to_string()).collect();
        write!(f, "{}", parts.join("."))
    }
}

/// Parses the leading version token of a commit message (for example `"12.0.6"`
/// from `"12.0.6"` or `"12.0.0 Season Zero Pt.2"`). Returns `None` for an empty
/// token or any non-numeric component.
pub fn parse_commit_message_version(message: &str) -> Option<GameVersion> {
    let token = message.split_whitespace().next()?;
    let mut parts = [0u16; 4];
    let mut count = 0;
    for component in token.split('.') {
        if count >= 4 {
            break;
        }
        parts[count] = component.parse::<u16>().ok()?;
        count += 1;
    }
    if count == 0 {
        return None;
    }
    Some(GameVersion(parts))
}

/// A non-fatal failure of the network version source.
#[derive(thiserror::Error, Debug)]
pub enum ApiCheckError {
    /// The HTTP request failed or returned a non-success status.
    #[error("http error: {0}")]
    Http(String),
    /// The response body could not be read.
    #[error("body error: {0}")]
    Body(String),
    /// The response could not be parsed into a game version.
    #[error("parse error: {0}")]
    Parse(String),
}

/// A source of the current live ESO game client version.
pub trait GameVersionSource {
    /// Fetches the current live game version, or a non-fatal error.
    fn fetch(&self) -> Result<GameVersion, ApiCheckError>;
}

/// The production source: the head commit of the official esoui/esoui `live`
/// branch, whose commit message begins with the live game version string.
pub struct GithubLiveSource {
    url: String,
    user_agent: String,
    timeout: Duration,
}

impl Default for GithubLiveSource {
    fn default() -> Self {
        Self {
            url: "https://api.github.com/repos/esoui/esoui/commits/live".to_string(),
            user_agent: format!(
                "eso-weave/{} (+https://github.com/h8rt3rmin8r/eso-weave)",
                env!("CARGO_PKG_VERSION")
            ),
            timeout: Duration::from_secs(5),
        }
    }
}

impl GameVersionSource for GithubLiveSource {
    fn fetch(&self) -> Result<GameVersion, ApiCheckError> {
        let body = ureq::get(&self.url)
            .timeout(self.timeout)
            .set("User-Agent", &self.user_agent)
            .set("Accept", "application/vnd.github+json")
            .call()
            .map_err(|err| ApiCheckError::Http(err.to_string()))?
            .into_string()
            .map_err(|err| ApiCheckError::Body(err.to_string()))?;
        let json: serde_json::Value =
            serde_json::from_str(&body).map_err(|err| ApiCheckError::Parse(err.to_string()))?;
        let message = json
            .get("commit")
            .and_then(|commit| commit.get("message"))
            .and_then(|message| message.as_str())
            .ok_or_else(|| ApiCheckError::Parse("response had no commit.message".to_string()))?;
        parse_commit_message_version(message)
            .ok_or_else(|| ApiCheckError::Parse(format!("unparseable version in {message:?}")))
    }
}

/// The result of a version check, handed to the GUI for persistence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiCheckOutcome {
    /// The resolved effective numeric API version to persist as last known.
    pub last_known_api_version: u32,
    /// The newest game version observed, if the fetch succeeded.
    pub last_seen_game_version: Option<GameVersion>,
}

/// Runs the startup version check: resolves the effective numeric API version,
/// keeps the on-disk manifest current (marker-gated, never downgrading), fetches
/// the game version for bump detection, and returns the values to persist.
///
/// Never blocks beyond the source timeout and never panics; all filesystem and
/// network errors are swallowed into logs and the returned outcome.
pub fn run_check(
    source: &dyn GameVersionSource,
    addons_root: Option<&Path>,
    stored_last_known: Option<u32>,
    stored_last_seen_game: Option<GameVersion>,
) -> ApiCheckOutcome {
    let effective = stored_last_known.unwrap_or(0).max(DEFAULT_API_VERSION);

    if let Some(root) = addons_root {
        update_installed_manifest(root, effective);
    }

    let mut last_seen = stored_last_seen_game;
    match source.fetch() {
        Ok(fetched) => {
            let baseline = stored_last_seen_game
                .unwrap_or(DEFAULT_GAME_VERSION)
                .max(DEFAULT_GAME_VERSION);
            if fetched > baseline {
                tracing::warn!(
                    target: "beacon",
                    "ESO client {fetched} is newer than this build's {DEFAULT_GAME_VERSION}; \
                     update ESO Weave to refresh the addon API version"
                );
            }
            if last_seen.is_none_or(|seen| fetched > seen) {
                last_seen = Some(fetched);
            }
        }
        Err(err) => {
            tracing::debug!(target: "beacon", "API version check fetch failed: {err}");
        }
    }

    ApiCheckOutcome {
        last_known_api_version: effective,
        last_seen_game_version: last_seen,
    }
}

/// Rewrites the on-disk manifest APIVersion line to `effective` when the addon is
/// installed, the manifest carries the managed marker, and its primary token is
/// older than `effective`. The managed-marker gate governs the write; an unmanaged
/// or unreadable manifest is never written, and an equal-or-newer primary is left
/// untouched (no downgrade, no churn).
fn update_installed_manifest(addons_root: &Path, effective: u32) {
    let manifest_path = addons_root.join(SUBFOLDER).join(MANIFEST_FILE);
    let existing = match std::fs::read_to_string(&manifest_path) {
        Ok(text) => text,
        Err(_) => return,
    };
    if !has_managed_marker(&existing) {
        return;
    }
    let current = parse_api_version_primary(&existing).unwrap_or(0);
    if current >= effective {
        return;
    }
    let updated = rewrite_api_version(&existing, effective);
    match std::fs::write(&manifest_path, updated) {
        Ok(()) => tracing::info!(
            target: "beacon",
            "updated PixelBeacon APIVersion from {current} to {effective}"
        ),
        Err(err) => tracing::warn!(target: "beacon", "APIVersion manifest update failed: {err}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beacon::{MANAGED_MARKER, MANIFEST};
    use std::fs;
    use tempfile::TempDir;

    struct MockSource(Result<GameVersion, ApiCheckError>);

    impl GameVersionSource for MockSource {
        fn fetch(&self) -> Result<GameVersion, ApiCheckError> {
            match &self.0 {
                Ok(version) => Ok(*version),
                Err(err) => Err(ApiCheckError::Parse(err.to_string())),
            }
        }
    }

    fn ok_source(parts: [u16; 4]) -> MockSource {
        MockSource(Ok(GameVersion::new(parts)))
    }

    fn err_source() -> MockSource {
        MockSource(Err(ApiCheckError::Parse("mock".to_string())))
    }

    #[test]
    fn parses_plain_and_suffixed_versions() {
        assert_eq!(
            parse_commit_message_version("12.0.6"),
            Some(GameVersion::new([12, 0, 6, 0]))
        );
        assert_eq!(
            parse_commit_message_version("12.0.0 Season Zero Pt.2"),
            Some(GameVersion::new([12, 0, 0, 0]))
        );
        assert_eq!(
            parse_commit_message_version("12"),
            Some(GameVersion::new([12, 0, 0, 0]))
        );
    }

    #[test]
    fn rejects_empty_and_non_numeric() {
        assert_eq!(parse_commit_message_version(""), None);
        assert_eq!(parse_commit_message_version("   "), None);
        assert_eq!(parse_commit_message_version("Merge branch"), None);
        assert_eq!(parse_commit_message_version("12.x.6"), None);
    }

    #[test]
    fn version_ordering_is_numeric() {
        assert!(GameVersion::new([12, 0, 10, 0]) > GameVersion::new([12, 0, 6, 0]));
        assert!(GameVersion::new([12, 1, 0, 0]) > GameVersion::new([12, 0, 9, 0]));
    }

    #[test]
    fn display_trims_trailing_zero_components() {
        assert_eq!(GameVersion::new([12, 0, 6, 0]).to_string(), "12.0.6");
        assert_eq!(GameVersion::new([12, 0, 0, 0]).to_string(), "12.0");
    }

    fn install_managed(root: &Path, primary_line: &str) {
        let dir = root.join(SUBFOLDER);
        fs::create_dir_all(&dir).unwrap();
        let manifest = MANIFEST.replace("## APIVersion: 101050 101054", primary_line);
        assert!(manifest.contains(MANAGED_MARKER));
        fs::write(dir.join(MANIFEST_FILE), manifest).unwrap();
    }

    fn read_manifest(root: &Path) -> String {
        fs::read_to_string(root.join(SUBFOLDER).join(MANIFEST_FILE)).unwrap()
    }

    #[test]
    fn rewrites_when_primary_older_and_marker_present() {
        let root = TempDir::new().unwrap();
        install_managed(root.path(), "## APIVersion: 101040");
        let outcome = run_check(&ok_source([12, 0, 6, 0]), Some(root.path()), None, None);
        assert_eq!(outcome.last_known_api_version, DEFAULT_API_VERSION);
        assert!(
            read_manifest(root.path()).contains(&format!("## APIVersion: {DEFAULT_API_VERSION}"))
        );
    }

    #[test]
    fn refuses_to_write_unmanaged_manifest() {
        let root = TempDir::new().unwrap();
        let dir = root.path().join(SUBFOLDER);
        fs::create_dir_all(&dir).unwrap();
        let unmanaged = "## Title: PixelBeacon\n## APIVersion: 101040\n";
        fs::write(dir.join(MANIFEST_FILE), unmanaged).unwrap();
        run_check(&ok_source([12, 0, 6, 0]), Some(root.path()), None, None);
        assert_eq!(read_manifest(root.path()), unmanaged);
    }

    #[test]
    fn never_downgrades_or_churns() {
        let root = TempDir::new().unwrap();
        install_managed(root.path(), "## APIVersion: 101060");
        let before = read_manifest(root.path());
        run_check(&ok_source([12, 0, 6, 0]), Some(root.path()), None, None);
        assert_eq!(read_manifest(root.path()), before);
    }

    #[test]
    fn resolution_prefers_stored_then_default() {
        let root = TempDir::new().unwrap();
        install_managed(root.path(), "## APIVersion: 101040");
        let outcome = run_check(&err_source(), Some(root.path()), Some(101070), None);
        assert_eq!(outcome.last_known_api_version, 101070);
        assert!(read_manifest(root.path()).contains("## APIVersion: 101070"));
    }

    #[test]
    fn fetch_error_is_swallowed_and_default_used() {
        let outcome = run_check(&err_source(), None, None, None);
        assert_eq!(outcome.last_known_api_version, DEFAULT_API_VERSION);
        assert_eq!(outcome.last_seen_game_version, None);
    }

    #[test]
    fn records_newer_game_version() {
        let outcome = run_check(&ok_source([12, 1, 0, 0]), None, None, None);
        assert_eq!(
            outcome.last_seen_game_version,
            Some(GameVersion::new([12, 1, 0, 0]))
        );
    }
}
