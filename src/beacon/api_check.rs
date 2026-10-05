//! Startup compatibility observations, independent of managed package support.
#[cfg(test)]
use super::version_source::parse_evidence;
pub use super::version_source::{GithubLiveSource, VersionEvidence};
use super::{DEFAULT_API_VERSION, DEFAULT_GAME_VERSION};
pub use crate::catalog::version::{parse_commit_message_version, GameVersion};
use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum ApiCheckError {
    #[error("http error: {0}")]
    Http(String),
    #[error("body error: {0}")]
    Body(String),
    #[error("parse error: {0}")]
    Parse(String),
}

pub struct VersionObservation {
    pub game_version: GameVersion,
    pub evidence: Option<VersionEvidence>,
}

pub trait GameVersionSource {
    fn fetch(&self) -> Result<GameVersion, ApiCheckError>;
    fn fetch_observation(&self) -> Result<VersionObservation, ApiCheckError> {
        self.fetch().map(|game_version| VersionObservation {
            game_version,
            evidence: None,
        })
    }
}

/// The result of a version check, handed to the GUI for persistence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApiCheckOutcome {
    /// Independently sourced API facts, absent on an unsuccessful source check.
    pub evidence: Option<VersionEvidence>,
    /// Legacy numeric cache, retained as history and never used to grant support.
    pub last_known_api_version: u32,
    /// The newest game version observed, if the fetch succeeded.
    pub last_seen_game_version: Option<GameVersion>,
    /// Whether this run reached and parsed the bounded Live-version source.
    pub fresh: bool,
}

/// Runs the startup compatibility observation without modifying installed addons.
/// Reviewed package updates occur through the existing managed lifecycle.
///
/// Production uses two reads bounded to five seconds each; source errors are
/// returned as unknown with a log record and historical values preserved.
pub fn run_check(
    source: &dyn GameVersionSource,
    _addons_root: Option<&Path>,
    stored_last_known: Option<u32>,
    stored_last_seen_game: Option<GameVersion>,
) -> ApiCheckOutcome {
    let effective = stored_last_known.unwrap_or(0).max(DEFAULT_API_VERSION);

    let mut last_seen = stored_last_seen_game;
    let mut evidence = None;
    let fresh = match source.fetch_observation() {
        Ok(observation) => {
            let fetched = observation.game_version;
            evidence = observation.evidence;
            let baseline = DEFAULT_GAME_VERSION;
            if fetched > baseline {
                tracing::warn!(
                    target: "beacon",
                    "ESO client {fetched} is newer than this build's {DEFAULT_GAME_VERSION}; \
                     check addon API compatibility in Addons"
                );
            }
            last_seen = Some(fetched);
            true
        }
        Err(err) => {
            tracing::warn!(target: "beacon", "Addon API compatibility is unknown: {err}");
            false
        }
    };

    ApiCheckOutcome {
        evidence,
        last_known_api_version: effective,
        last_seen_game_version: last_seen,
        fresh,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beacon::{MANAGED_MARKER, MANIFEST, MANIFEST_FILE, SUBFOLDER};
    use std::fs;
    use tempfile::TempDir;

    fn history() -> String {
        serde_json::json!([
            {"sha": "6639eb2adecc0480557d9068579319919a0c3fe6",
             "commit": {"message": "Merge branch 'pts' into live",
                        "committer": {"date": "2026-09-28T19:44:27Z"}}},
            {"sha": "c6a91c390a9acee843560542923c69d06c438cd3",
             "commit": {"message": "12.1.5",
                        "committer": {"date": "2026-09-28T19:44:27Z"}}}
        ])
        .to_string()
    }

    #[test]
    fn s120_merge_head_uses_numeric_history_and_independent_api() {
        let now = 1_791_244_800; // 2026-10-05 UTC
        let evidence = parse_evidence(
            &history(),
            "{TOC:maxLevel=3}\nh1. ESO UI Documentation for API Version 101051\n",
            super::super::Environment::Live,
            now,
        )
        .unwrap();
        assert_eq!(evidence.game_version, GameVersion::new([12, 1, 5, 0]));
        assert_eq!(evidence.api_version, 101051);
        assert_eq!(evidence.environment, super::super::Environment::Live);
        assert_eq!(evidence.checked_at, now);
    }

    #[test]
    fn s120_missing_malformed_stale_and_future_evidence_is_unknown() {
        let now = 1_791_244_800;
        let doc = "h1. ESO UI Documentation for API Version 101051\n";
        for body in [
            "[]".to_owned(),
            "{}".to_owned(),
            history().replace("6639eb2", "../evil"),
            history().replace("2026-09-28", "2026-08-01"),
            history().replace("2026-09-28", "2026-12-01"),
        ] {
            assert!(parse_evidence(&body, doc, super::super::Environment::Live, now).is_err());
        }
        for bad in [
            "",
            "h1. ESO UI Documentation for API Version 12.1.5",
            "h1. ESO UI Documentation for API Version 101051 extra",
            "h1. ESO UI Documentation for API Version 0",
        ] {
            assert!(parse_evidence(&history(), bad, super::super::Environment::Pts, now).is_err());
        }
    }

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
        let manifest = MANIFEST.replace("## APIVersion: 101051 101050", primary_line);
        assert!(manifest.contains(MANAGED_MARKER));
        fs::write(dir.join(MANIFEST_FILE), manifest).unwrap();
    }

    fn read_manifest(root: &Path) -> String {
        fs::read_to_string(root.join(SUBFOLDER).join(MANIFEST_FILE)).unwrap()
    }

    #[test]
    fn s120_check_does_not_grant_support_to_an_older_installed_package() {
        let root = TempDir::new().unwrap();
        install_managed(root.path(), "## APIVersion: 101040");
        let outcome = run_check(&ok_source([12, 0, 6, 0]), Some(root.path()), None, None);
        assert_eq!(outcome.last_known_api_version, DEFAULT_API_VERSION);
        assert!(read_manifest(root.path()).contains("## APIVersion: 101040"));
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
        assert!(read_manifest(root.path()).contains("## APIVersion: 101040"));
    }

    #[test]
    fn fetch_error_is_swallowed_and_default_used() {
        let outcome = run_check(&err_source(), None, None, None);
        assert_eq!(outcome.last_known_api_version, DEFAULT_API_VERSION);
        assert_eq!(outcome.last_seen_game_version, None);
        assert!(!outcome.fresh);
    }

    #[test]
    fn records_newer_game_version() {
        let outcome = run_check(&ok_source([12, 1, 0, 0]), None, None, None);
        assert_eq!(
            outcome.last_seen_game_version,
            Some(GameVersion::new([12, 1, 0, 0]))
        );
        assert!(outcome.fresh);
    }
}
