//! Local-only, placeholder-first icon transformation and immutable caching.

mod manifest;
mod path;
mod transform;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub use manifest::{
    FallbackReason, IconAvailability, IconCacheEntry, IconCacheManifest, IconRedistribution,
    MANIFEST_SCHEMA_VERSION,
};
use path::{
    is_link_like, normalize_virtual_path, resolve_local, validate_distinct_roots, LocalResolution,
};
use transform::{placeholder_png, sha256, transform_source, verify_png};

pub const MAX_SOURCE_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_ICON_DIMENSION: u32 = 1024;
pub const MAX_ICON_PIXELS: u32 = 1024 * 1024;
pub const MAX_ICON_REFERENCES: usize = 500_000;
pub const MAX_MANIFEST_BYTES: u64 = 64 * 1024 * 1024;
pub const TRANSFORMATION_ID: &str = "rgba8-png-v1";

#[derive(thiserror::Error, Debug)]
pub enum IconCacheError {
    #[error("icon cache I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("icon cache JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("icon cache validation failed: {0}")]
    Validation(String),
}

#[derive(Debug, Clone)]
pub struct IconCacheRequest {
    pub source_root: PathBuf,
    pub cache_root: PathBuf,
    pub catalog_semantic_sha256: String,
    pub references: Vec<String>,
}

impl IconCacheRequest {
    pub fn new(
        source_root: impl Into<PathBuf>,
        cache_root: impl Into<PathBuf>,
        catalog_semantic_sha256: impl Into<String>,
        references: Vec<String>,
    ) -> Self {
        Self {
            source_root: source_root.into(),
            cache_root: cache_root.into(),
            catalog_semantic_sha256: catalog_semantic_sha256.into(),
            references,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconCacheReceipt {
    pub generation_sha256: String,
    pub manifest_path: PathBuf,
    pub transformation_id: String,
    pub entry_count: usize,
    pub object_count: usize,
    pub placeholder_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconLookupState {
    Ready,
    Placeholder,
    Missing,
    Unsupported,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconResolution {
    pub state: IconLookupState,
    pub object_path: PathBuf,
    pub object_sha256: String,
    pub uses_placeholder: bool,
    pub fallback_reason: Option<FallbackReason>,
}

pub struct IconCacheGeneration {
    cache_root: PathBuf,
    manifest: IconCacheManifest,
    entries: BTreeMap<String, IconCacheEntry>,
}

impl IconCacheGeneration {
    pub fn manifest(&self) -> &IconCacheManifest {
        &self.manifest
    }

    pub fn resolve(&self, canonical_path: &str) -> Result<IconResolution, IconCacheError> {
        let normalized = normalize_virtual_path(canonical_path)?;
        if let Some(entry) = self.entries.get(&normalized.lookup_key) {
            let state = match (entry.availability, entry.fallback_reason) {
                (IconAvailability::Ready, _) => IconLookupState::Ready,
                (IconAvailability::Placeholder, Some(FallbackReason::Missing)) => {
                    IconLookupState::Missing
                }
                (IconAvailability::Placeholder, Some(FallbackReason::Unsupported)) => {
                    IconLookupState::Unsupported
                }
                (IconAvailability::Placeholder, Some(_)) => IconLookupState::Failed,
                (IconAvailability::Placeholder, None) => IconLookupState::Placeholder,
            };
            return Ok(IconResolution {
                state,
                object_path: object_path(&self.cache_root, &entry.object_sha256),
                object_sha256: entry.object_sha256.clone(),
                uses_placeholder: entry.availability == IconAvailability::Placeholder,
                fallback_reason: entry.fallback_reason,
            });
        }
        Ok(IconResolution {
            state: IconLookupState::Missing,
            object_path: object_path(&self.cache_root, &self.manifest.placeholder_sha256),
            object_sha256: self.manifest.placeholder_sha256.clone(),
            uses_placeholder: true,
            fallback_reason: Some(FallbackReason::Missing),
        })
    }
}

pub fn build_generation(request: &IconCacheRequest) -> Result<IconCacheReceipt, IconCacheError> {
    if !valid_sha256(&request.catalog_semantic_sha256) {
        return invalid("catalog semantic SHA-256 is invalid");
    }
    if request.references.len() > MAX_ICON_REFERENCES {
        return invalid("icon reference count exceeds the cache limit");
    }
    let canonical_source = validate_distinct_roots(&request.source_root, &request.cache_root)?;

    let mut references = request
        .references
        .iter()
        .map(|value| normalize_virtual_path(value))
        .collect::<Result<Vec<_>, _>>()?;
    references.sort_by(|left, right| left.lookup_key.cmp(&right.lookup_key));
    if references
        .windows(2)
        .any(|pair| pair[0].lookup_key == pair[1].lookup_key)
    {
        return invalid("virtual path lookup keys must be unique");
    }

    let objects_directory = request.cache_root.join("objects");
    let generations_directory = request.cache_root.join("generations");
    fs::create_dir_all(&objects_directory)?;
    fs::create_dir_all(&generations_directory)?;
    ensure_real_directory(&request.cache_root)?;
    ensure_real_directory(&objects_directory)?;
    ensure_real_directory(&generations_directory)?;

    let (placeholder_bytes, placeholder_width, placeholder_height) = placeholder_png();
    let placeholder_sha256 = sha256(&placeholder_bytes);
    publish_object(&objects_directory, &placeholder_sha256, &placeholder_bytes)?;

    let mut entries = Vec::with_capacity(references.len());
    let mut object_hashes = BTreeSet::from([placeholder_sha256.clone()]);
    let mut placeholder_count = 0;
    for reference in references {
        let transformed = match resolve_local(&canonical_source, &reference)? {
            LocalResolution::Found(path) => transform_source(&path),
            LocalResolution::Fallback(reason) => Err(reason),
        };
        match transformed {
            Ok(transformed) => {
                let object_sha256 = sha256(&transformed.png);
                publish_object(&objects_directory, &object_sha256, &transformed.png)?;
                object_hashes.insert(object_sha256.clone());
                entries.push(IconCacheEntry {
                    canonical_path: reference.canonical,
                    lookup_key: reference.lookup_key,
                    object_sha256,
                    source_sha256: Some(transformed.source_sha256),
                    width: transformed.width,
                    height: transformed.height,
                    media_type: "image/png".into(),
                    origin: "user-supplied".into(),
                    availability: IconAvailability::Ready,
                    redistribution: IconRedistribution::UserLocalOnly,
                    fallback_reason: None,
                });
            }
            Err(reason) => {
                placeholder_count += 1;
                entries.push(IconCacheEntry {
                    canonical_path: reference.canonical,
                    lookup_key: reference.lookup_key,
                    object_sha256: placeholder_sha256.clone(),
                    source_sha256: None,
                    width: placeholder_width,
                    height: placeholder_height,
                    media_type: "image/png".into(),
                    origin: "project-placeholder".into(),
                    availability: IconAvailability::Placeholder,
                    redistribution: IconRedistribution::Allowed,
                    fallback_reason: Some(reason),
                });
            }
        }
    }

    let manifest = IconCacheManifest {
        schema_version: MANIFEST_SCHEMA_VERSION,
        catalog_semantic_sha256: request.catalog_semantic_sha256.clone(),
        transformation_id: TRANSFORMATION_ID.into(),
        placeholder_sha256,
        entries,
    };
    let manifest_bytes = manifest.canonical_bytes()?;
    if manifest_bytes.len() as u64 > MAX_MANIFEST_BYTES {
        return invalid("icon cache manifest exceeds the byte limit");
    }
    let generation_sha256 = sha256(&manifest_bytes);
    let generation_directory = generations_directory.join(&generation_sha256);
    let manifest_path = generation_directory.join("manifest.json");

    if generation_directory.exists() {
        ensure_real_directory(&generation_directory)?;
        ensure_real_file(&manifest_path)?;
        let existing = fs::read(&manifest_path)?;
        if existing != manifest_bytes {
            return invalid("existing generation does not match its identity");
        }
    } else {
        let candidate = tempfile::Builder::new()
            .prefix(".icon-generation-")
            .tempdir_in(&generations_directory)?;
        let candidate_manifest = candidate.path().join("manifest.json");
        let mut file = fs::File::create(&candidate_manifest)?;
        file.write_all(&manifest_bytes)?;
        file.sync_all()?;
        drop(file);
        ensure_real_file(&candidate_manifest)?;
        let candidate_bytes = fs::read(&candidate_manifest)?;
        if candidate_bytes != manifest_bytes || sha256(&candidate_bytes) != generation_sha256 {
            return invalid("candidate manifest changed before publication");
        }
        let candidate_model: IconCacheManifest = serde_json::from_slice(&candidate_bytes)?;
        if candidate_model != manifest {
            return invalid("candidate manifest does not match the requested generation");
        }
        verify_manifest_objects(&request.cache_root, &manifest)?;
        fs::rename(candidate.path(), &generation_directory)?;
    }

    let generation = open_generation(&request.cache_root, &generation_sha256)?;
    if generation.manifest != manifest {
        return invalid("published generation changed during verification");
    }
    Ok(IconCacheReceipt {
        generation_sha256: generation_sha256.clone(),
        manifest_path: PathBuf::from("generations")
            .join(&generation_sha256)
            .join("manifest.json"),
        transformation_id: TRANSFORMATION_ID.into(),
        entry_count: manifest.entries.len(),
        object_count: object_hashes.len(),
        placeholder_count,
    })
}

pub fn open_generation(
    cache_root: &Path,
    generation_sha256: &str,
) -> Result<IconCacheGeneration, IconCacheError> {
    if !valid_sha256(generation_sha256) {
        return invalid("generation SHA-256 is invalid");
    }
    let manifest_path = cache_root
        .join("generations")
        .join(generation_sha256)
        .join("manifest.json");
    ensure_real_directory(cache_root)?;
    ensure_real_directory(&cache_root.join("objects"))?;
    ensure_real_directory(&cache_root.join("generations"))?;
    ensure_real_directory(&cache_root.join("generations").join(generation_sha256))?;
    let metadata = ensure_real_file(&manifest_path)?;
    if metadata.len() > MAX_MANIFEST_BYTES {
        return invalid("icon cache manifest exceeds the byte limit");
    }
    let bytes = fs::read(&manifest_path)?;
    if sha256(&bytes) != generation_sha256 {
        return invalid("icon cache manifest hash does not match its generation");
    }
    let manifest: IconCacheManifest = serde_json::from_slice(&bytes)?;
    manifest.validate()?;
    if manifest.canonical_bytes()? != bytes {
        return invalid("icon cache manifest is not canonical");
    }
    verify_manifest_objects(cache_root, &manifest)?;
    let entries = manifest
        .entries
        .iter()
        .map(|entry| (entry.lookup_key.clone(), entry.clone()))
        .collect();
    Ok(IconCacheGeneration {
        cache_root: cache_root.to_path_buf(),
        manifest,
        entries,
    })
}

fn verify_manifest_objects(
    cache_root: &Path,
    manifest: &IconCacheManifest,
) -> Result<(), IconCacheError> {
    let mut dimensions = BTreeMap::new();
    dimensions.insert(
        manifest.placeholder_sha256.as_str(),
        manifest
            .entries
            .iter()
            .find(|entry| entry.object_sha256 == manifest.placeholder_sha256)
            .map(|entry| (entry.width, entry.height))
            .unwrap_or((64, 64)),
    );
    for entry in &manifest.entries {
        if dimensions
            .insert(entry.object_sha256.as_str(), (entry.width, entry.height))
            .is_some_and(|prior| prior != (entry.width, entry.height))
        {
            return invalid("one icon object has conflicting dimensions");
        }
    }
    for (hash, expected_dimensions) in dimensions {
        let path = object_path(cache_root, hash);
        let metadata = ensure_real_file(&path)?;
        if metadata.len() > MAX_SOURCE_BYTES {
            return invalid("icon cache object exceeds the byte limit");
        }
        let bytes = fs::read(path)?;
        if sha256(&bytes) != hash || verify_png(&bytes, expected_dimensions).is_err() {
            return invalid("icon cache object failed content verification");
        }
    }
    Ok(())
}

fn publish_object(directory: &Path, hash: &str, bytes: &[u8]) -> Result<(), IconCacheError> {
    let destination = directory.join(format!("{hash}.png"));
    if destination.exists() {
        ensure_real_file(&destination)?;
        if fs::read(&destination)? == bytes {
            return Ok(());
        }
        return invalid("existing icon object does not match its content hash");
    }
    let mut candidate = tempfile::NamedTempFile::new_in(directory)?;
    candidate.write_all(bytes)?;
    candidate.as_file_mut().sync_all()?;
    match candidate.persist_noclobber(&destination) {
        Ok(_) => Ok(()),
        Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
            if fs::read(destination)? == bytes {
                Ok(())
            } else {
                invalid("raced icon object does not match its content hash")
            }
        }
        Err(error) => Err(IconCacheError::Io(error.error)),
    }
}

fn object_path(cache_root: &Path, hash: &str) -> PathBuf {
    cache_root.join("objects").join(format!("{hash}.png"))
}

fn ensure_real_directory(path: &Path) -> Result<fs::Metadata, IconCacheError> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_dir() || is_link_like(&metadata) {
        return invalid("icon cache directory must not be link-like");
    }
    Ok(metadata)
}

fn ensure_real_file(path: &Path) -> Result<fs::Metadata, IconCacheError> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || is_link_like(&metadata) {
        return invalid("icon cache file must not be link-like");
    }
    Ok(metadata)
}

pub(crate) fn valid_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn invalid<T>(message: impl Into<String>) -> Result<T, IconCacheError> {
    Err(IconCacheError::Validation(message.into()))
}
