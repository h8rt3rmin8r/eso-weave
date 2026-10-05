//! Bounded, revision-pinned published game UI evidence for the selected channel.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::api_check::{ApiCheckError, GameVersion, GameVersionSource, VersionObservation};
use super::Environment;

const MAX_AGE_SECONDS: i64 = 30 * 24 * 60 * 60;
const HISTORY_LIMIT: u64 = 256 * 1024;
const DOCUMENT_LIMIT: u64 = 2 * 1024 * 1024;

/// An observation, never a declaration that an addon supports this API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionEvidence {
    pub environment: Environment,
    pub game_version: GameVersion,
    pub api_version: u32,
    pub source_revision: [u8; 20],
    pub source_updated_at: i64,
    pub checked_at: i64,
}

impl VersionEvidence {
    /// A cached or mismatched observation cannot establish current support.
    pub fn is_current(self, environment: Environment, now: i64) -> bool {
        self.environment == environment
            && self.api_version >= 100_000
            && self.api_version <= 999_999
            && self.source_revision != [0; 20]
            && self.source_updated_at <= self.checked_at
            && self.checked_at <= now
            && now.saturating_sub(self.source_updated_at) <= MAX_AGE_SECONDS
    }
}

/// Published UI source; merge heads need not begin with a numeric release.
pub struct GithubLiveSource {
    environment: Environment,
}

impl Default for GithubLiveSource {
    fn default() -> Self {
        Self::for_environment(Environment::Live)
    }
}

impl GithubLiveSource {
    pub fn for_environment(environment: Environment) -> Self {
        Self { environment }
    }

    fn read(&self, url: &str, limit: u64) -> Result<String, ApiCheckError> {
        ureq::get(url)
            .config()
            .timeout_global(Some(Duration::from_secs(5)))
            .build()
            .header(
                "User-Agent",
                concat!("eso-weave/", env!("CARGO_PKG_VERSION")),
            )
            .header("Accept", "application/vnd.github+json")
            .call()
            .map_err(|err| ApiCheckError::Http(err.to_string()))?
            .body_mut()
            .with_config()
            .limit(limit)
            .lossy_utf8(false)
            .read_to_string()
            .map_err(|err| ApiCheckError::Body(err.to_string()))
    }
}

impl GameVersionSource for GithubLiveSource {
    fn fetch(&self) -> Result<GameVersion, ApiCheckError> {
        self.fetch_observation()
            .map(|observation| observation.game_version)
    }

    fn fetch_observation(&self) -> Result<VersionObservation, ApiCheckError> {
        let history = self.read(
            &format!(
                "https://api.github.com/repos/esoui/esoui/commits?sha={}&per_page=8",
                self.environment.segment()
            ),
            HISTORY_LIMIT,
        )?;
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        // Validate the revision before placing it in the fixed upstream URL.
        let (revision, _, _) = parse_history(&history, now)?;
        let documentation = self.read(
            &format!(
                "https://raw.githubusercontent.com/esoui/esoui/{revision}/ESOUIDocumentation.txt"
            ),
            DOCUMENT_LIMIT,
        )?;
        let evidence = parse_evidence(&history, &documentation, self.environment, now)?;
        Ok(VersionObservation {
            game_version: evidence.game_version,
            evidence: Some(evidence),
        })
    }
}

#[derive(Deserialize)]
struct Commit {
    sha: String,
    commit: CommitBody,
}

#[derive(Deserialize)]
struct CommitBody {
    message: String,
    committer: CommitDate,
}

#[derive(Deserialize)]
struct CommitDate {
    date: String,
}

fn parse_timestamp(value: &str) -> Option<i64> {
    if value.len() != 20
        || !value.is_ascii()
        || &value[4..5] != "-"
        || &value[7..8] != "-"
        || &value[10..11] != "T"
        || &value[13..14] != ":"
        || &value[16..17] != ":"
        || &value[19..20] != "Z"
    {
        return None;
    }
    let date = time::Date::from_calendar_date(
        value[..4].parse().ok()?,
        time::Month::try_from(value[5..7].parse::<u8>().ok()?).ok()?,
        value[8..10].parse().ok()?,
    )
    .ok()?;
    let clock = time::Time::from_hms(
        value[11..13].parse().ok()?,
        value[14..16].parse().ok()?,
        value[17..19].parse().ok()?,
    )
    .ok()?;
    Some(
        time::PrimitiveDateTime::new(date, clock)
            .assume_utc()
            .unix_timestamp(),
    )
}

fn parse_history(body: &str, now: i64) -> Result<(String, i64, GameVersion), ApiCheckError> {
    let invalid = || ApiCheckError::Parse("missing, malformed, or stale channel history".into());
    if body.len() as u64 > HISTORY_LIMIT {
        return Err(invalid());
    }
    let commits: Vec<Commit> = serde_json::from_str(body).map_err(|_| invalid())?;
    if commits.is_empty() || commits.len() > 8 {
        return Err(invalid());
    }
    let head = &commits[0];
    if head.sha.len() != 40 || !head.sha.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(invalid());
    }
    let updated = parse_timestamp(&head.commit.committer.date).ok_or_else(invalid)?;
    if updated > now || now.saturating_sub(updated) > MAX_AGE_SECONDS {
        return Err(invalid());
    }
    let (entry, version) = commits
        .iter()
        .find_map(|entry| {
            super::api_check::parse_commit_message_version(&entry.commit.message)
                .map(|version| (entry, version))
        })
        .ok_or_else(invalid)?;
    let version_date = parse_timestamp(&entry.commit.committer.date).ok_or_else(invalid)?;
    if version_date > updated || now.saturating_sub(version_date) > MAX_AGE_SECONDS {
        return Err(invalid());
    }
    Ok((head.sha.clone(), updated, version))
}

pub(super) fn parse_evidence(
    history: &str,
    documentation: &str,
    environment: Environment,
    now: i64,
) -> Result<VersionEvidence, ApiCheckError> {
    let (revision, updated, game_version) = parse_history(history, now)?;
    let invalid = || ApiCheckError::Parse("missing or malformed documented numeric API".into());
    if documentation.len() as u64 > DOCUMENT_LIMIT {
        return Err(invalid());
    }
    let mut api_headers = documentation
        .lines()
        .take(8)
        .filter_map(|line| line.strip_prefix("h1. ESO UI Documentation for API Version "));
    let token = api_headers.next().ok_or_else(invalid)?;
    if token.len() != 6
        || !token.bytes().all(|byte| byte.is_ascii_digit())
        || api_headers.next().is_some()
    {
        return Err(invalid());
    }
    let api_version = token.parse::<u32>().map_err(|_| invalid())?;
    if !(100_000..=999_999).contains(&api_version) {
        return Err(invalid());
    }
    let mut source_revision = [0; 20];
    for (index, byte) in source_revision.iter_mut().enumerate() {
        *byte =
            u8::from_str_radix(&revision[index * 2..index * 2 + 2], 16).map_err(|_| invalid())?;
    }
    Ok(VersionEvidence {
        environment,
        game_version,
        api_version,
        source_revision,
        source_updated_at: updated,
        checked_at: now,
    })
}
