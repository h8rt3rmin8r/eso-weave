use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

use super::manifest::{PipelineRequest, SourceInventory, SourcePin};
use super::{
    canonical_real_directory, read_bounded, resolve_relative, sha256, PipelineError, PipelineRun,
};

pub trait SourceFetcher {
    fn fetch(&self, uri: &str, max_bytes: u64) -> Result<Vec<u8>, PipelineError>;
}

pub(super) struct HttpsFetcher;

impl SourceFetcher for HttpsFetcher {
    fn fetch(&self, uri: &str, max_bytes: u64) -> Result<Vec<u8>, PipelineError> {
        let agent = ureq::Agent::config_builder()
            .https_only(true)
            .max_redirects(0)
            .max_redirects_will_error(true)
            .timeout_global(Some(Duration::from_secs(20)))
            .build()
            .new_agent();
        let mut response = agent
            .get(uri)
            .call()
            .map_err(|error| PipelineError::Acquisition(error.to_string()))?;
        let mut bytes = Vec::new();
        response
            .body_mut()
            .as_reader()
            .take(max_bytes.saturating_add(1))
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > max_bytes {
            return acquisition("download exceeds its declared byte limit");
        }
        Ok(bytes)
    }
}

pub(super) struct AcquiredSource {
    pub(super) path: PathBuf,
    pub(super) inventory: SourceInventory,
}

pub(super) fn acquire_sources(
    request: &PipelineRequest,
    run: &PipelineRun,
    workspace: &std::path::Path,
    fetcher: &dyn SourceFetcher,
) -> Result<BTreeMap<String, AcquiredSource>, PipelineError> {
    fs::create_dir_all(&run.source_cache)?;
    let cache = canonical_real_directory(&run.source_cache, "source cache")?;
    let mut result = BTreeMap::new();
    let mut sources = request.sources.iter().collect::<Vec<_>>();
    sources.sort_by(|left, right| left.id.cmp(&right.id));
    for source in sources {
        let destination = cache.join(format!("{}.bin", source.sha256));
        let cached = if destination.exists() {
            Some(read_bounded(&cache, &destination, source.max_bytes)?)
        } else {
            None
        };
        if let Some(bytes) = &cached {
            verify_source(source, bytes, "cached source")?;
        }
        let (bytes, acquisition) = if let Some(relative) = &source.local_path {
            let path = resolve_relative(workspace, relative, false)?;
            let bytes = read_bounded(workspace, &path, source.max_bytes)?;
            verify_source(source, &bytes, "local source")?;
            (bytes, "local".to_string())
        } else {
            if !immutable_raw_uri(&source.uri, &source.revision) {
                return acquisition("remote URI is not an immutable approved HTTPS source");
            }
            if let Some(bytes) = cached {
                if request.network.refresh {
                    if !(request.network.enabled && run.allow_network) {
                        return acquisition("network refresh requires both network gates");
                    }
                    match fetcher.fetch(&source.uri, source.max_bytes) {
                        Ok(downloaded) => {
                            verify_source(source, &downloaded, "downloaded source")?;
                            (downloaded, "downloaded".to_string())
                        }
                        Err(_) if request.network.allow_stale_cache => {
                            (bytes, "stale-cache".to_string())
                        }
                        Err(error) => return Err(error),
                    }
                } else {
                    (bytes, "cache".to_string())
                }
            } else {
                if !(request.network.enabled && run.allow_network) {
                    return acquisition("remote source requires both network gates");
                }
                let bytes = fetcher.fetch(&source.uri, source.max_bytes)?;
                verify_source(source, &bytes, "downloaded source")?;
                (bytes, "downloaded".to_string())
            }
        };
        if !destination.exists() {
            let temp = tempfile::NamedTempFile::new_in(&cache)?;
            fs::write(temp.path(), &bytes)?;
            match temp.persist_noclobber(&destination) {
                Ok(_) => {}
                Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error.error.into()),
            }
        }
        let published = read_bounded(&cache, &destination, source.max_bytes)?;
        verify_source(source, &published, "published cached source")?;
        result.insert(
            source.id.clone(),
            AcquiredSource {
                path: destination,
                inventory: SourceInventory {
                    id: source.id.clone(),
                    role: source.role,
                    channel: source.channel,
                    game_version: source.game_version.clone(),
                    api_version: source.api_version,
                    locale: source.locale.clone(),
                    revision: source.revision.clone(),
                    sha256: source.sha256.clone(),
                    byte_count: bytes.len() as u64,
                    uri: source.uri.clone(),
                    license_scope: source.license_scope.clone(),
                    redistribution: source.redistribution,
                    acquisition,
                },
            },
        );
    }
    Ok(result)
}

fn verify_source(source: &SourcePin, bytes: &[u8], description: &str) -> Result<(), PipelineError> {
    if bytes.len() as u64 > source.max_bytes || sha256(bytes) != source.sha256 {
        return acquisition(format!("{description} failed size or SHA-256 verification"));
    }
    Ok(())
}

fn immutable_raw_uri(uri: &str, revision: &str) -> bool {
    revision.len() == 40
        && revision.bytes().all(|byte| byte.is_ascii_hexdigit())
        && uri.starts_with("https://raw.githubusercontent.com/esoui/esoui/")
        && uri
            .strip_prefix("https://raw.githubusercontent.com/esoui/esoui/")
            .is_some_and(|tail| tail.starts_with(&format!("{revision}/")))
}

fn acquisition<T>(message: impl Into<String>) -> Result<T, PipelineError> {
    Err(PipelineError::Acquisition(message.into()))
}
