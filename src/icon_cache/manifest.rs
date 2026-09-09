use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::path::normalize_virtual_path;
use super::{
    valid_sha256, IconCacheError, MAX_ICON_DIMENSION, MAX_ICON_REFERENCES, TRANSFORMATION_ID,
};

pub const MANIFEST_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FallbackReason {
    Missing,
    Unsupported,
    Invalid,
    PermissionDenied,
    IoFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconAvailability {
    Ready,
    Placeholder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconRedistribution {
    UserLocalOnly,
    Allowed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IconCacheEntry {
    pub canonical_path: String,
    pub lookup_key: String,
    pub object_sha256: String,
    pub source_sha256: Option<String>,
    pub width: u32,
    pub height: u32,
    pub media_type: String,
    pub origin: String,
    pub availability: IconAvailability,
    pub redistribution: IconRedistribution,
    pub fallback_reason: Option<FallbackReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IconCacheManifest {
    pub schema_version: u32,
    pub catalog_semantic_sha256: String,
    pub transformation_id: String,
    pub placeholder_sha256: String,
    pub entries: Vec<IconCacheEntry>,
}

impl IconCacheManifest {
    pub fn validate(&self) -> Result<(), IconCacheError> {
        if self.schema_version != MANIFEST_SCHEMA_VERSION {
            return invalid("unsupported icon cache manifest schema");
        }
        if !valid_sha256(&self.catalog_semantic_sha256) || !valid_sha256(&self.placeholder_sha256) {
            return invalid("manifest contains an invalid SHA-256");
        }
        if self.transformation_id != TRANSFORMATION_ID {
            return invalid("manifest transformation is unsupported");
        }
        if self.entries.len() > MAX_ICON_REFERENCES {
            return invalid("manifest icon reference count exceeds the cache limit");
        }

        let mut prior = None;
        let mut keys = BTreeSet::new();
        for entry in &self.entries {
            let normalized = normalize_virtual_path(&entry.canonical_path)?;
            if normalized.lookup_key != entry.lookup_key
                || !keys.insert(entry.lookup_key.as_str())
                || prior.is_some_and(|value: &str| value >= entry.lookup_key.as_str())
            {
                return invalid("manifest icon mappings are not unique and sorted");
            }
            prior = Some(entry.lookup_key.as_str());
            if !valid_sha256(&entry.object_sha256)
                || entry
                    .source_sha256
                    .as_deref()
                    .is_some_and(|value| !valid_sha256(value))
            {
                return invalid("manifest entry contains an invalid SHA-256");
            }
            if entry.width == 0
                || entry.height == 0
                || entry.width > MAX_ICON_DIMENSION
                || entry.height > MAX_ICON_DIMENSION
                || entry.media_type != "image/png"
            {
                return invalid("manifest entry contains invalid image metadata");
            }
            match entry.availability {
                IconAvailability::Ready
                    if entry.origin == "user-supplied"
                        && entry.redistribution == IconRedistribution::UserLocalOnly
                        && entry.source_sha256.is_some()
                        && entry.fallback_reason.is_none() => {}
                IconAvailability::Placeholder
                    if entry.origin == "project-placeholder"
                        && entry.redistribution == IconRedistribution::Allowed
                        && entry.source_sha256.is_none()
                        && entry.fallback_reason.is_some()
                        && entry.object_sha256 == self.placeholder_sha256 => {}
                _ => return invalid("manifest entry has inconsistent provenance"),
            }
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, IconCacheError> {
        self.validate()?;
        let mut bytes = serde_json::to_vec_pretty(self)?;
        bytes.push(b'\n');
        Ok(bytes)
    }
}

fn invalid<T>(message: impl Into<String>) -> Result<T, IconCacheError> {
    Err(IconCacheError::Validation(message.into()))
}
