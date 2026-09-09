use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::catalog::model::{
    AliasInput, AttributeInput, CatalogBundle, Completeness, CoverageInput, EntityInput, EntityRef,
    IconAssetInput, IconReferenceInput, LocalizedTextInput, Redistribution, RelationInput,
    ReleaseInput, SourceChannel, SourceRecordInput, SourceSnapshotInput,
    CATEGORIES as CATALOG_CATEGORIES, INPUT_SCHEMA_VERSION,
};
use crate::catalog::Channel;

use super::{parse_capture, CollectorEnvelope, CollectorError, ImportRequest};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportReceipt {
    pub status: &'static str,
    pub capture_sha256: String,
    pub staged_sha256: String,
    pub schema_version: u32,
    pub collector_version: u32,
    pub channel: Channel,
    pub game_version: String,
    pub api_version: u32,
    pub locale: String,
    pub platform: String,
    pub megaserver: String,
    pub started_at: String,
    pub finished_at: String,
    pub scope_key_sha256: String,
    pub record_count: usize,
    pub chunk_count: usize,
    pub selected_categories: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn import_capture(request: &ImportRequest) -> Result<ImportReceipt, CollectorError> {
    validate_distinct_paths(&request.input_path, &request.output_path)?;
    let metadata = fs::metadata(&request.input_path)?;
    if !metadata.is_file() {
        return rejected("capture input is not a regular file");
    }
    if metadata.len() > super::MAX_CAPTURE_BYTES {
        return rejected(format!(
            "capture exceeds the {} byte limit",
            super::MAX_CAPTURE_BYTES
        ));
    }
    let bytes = fs::read(&request.input_path)?;
    let capture_sha256 = sha256_bytes(&bytes);
    let envelope = parse_capture(&bytes)?;
    if envelope.channel != request.expected_channel {
        return rejected(format!(
            "capture channel {} does not match expected {}",
            envelope.channel, request.expected_channel
        ));
    }
    if request.catalog_version.trim().is_empty() {
        return rejected("catalog version cannot be empty");
    }

    let bundle = bundle_from_capture(&envelope, &capture_sha256, &request.catalog_version)?
        .normalize_and_validate()?;
    let mut staged = serde_json::to_vec_pretty(&bundle)?;
    staged.push(b'\n');
    let staged_sha256 = sha256_bytes(&staged);
    write_atomic(&request.output_path, &staged)?;

    Ok(ImportReceipt {
        status: "staged",
        capture_sha256,
        staged_sha256,
        schema_version: envelope.schema_version,
        collector_version: envelope.collector_version,
        channel: envelope.channel,
        game_version: envelope.game_version,
        api_version: envelope.api_version,
        locale: envelope.locale,
        platform: envelope.platform,
        megaserver: envelope.megaserver,
        started_at: envelope.started_at,
        finished_at: envelope.finished_at,
        scope_key_sha256: sha256_bytes(envelope.scope_key.as_bytes()),
        record_count: envelope.records.len(),
        chunk_count: envelope.chunks.len(),
        selected_categories: envelope.selected_categories,
        warnings: envelope.warnings,
    })
}

fn bundle_from_capture(
    envelope: &CollectorEnvelope,
    capture_sha256: &str,
    catalog_version: &str,
) -> Result<CatalogBundle, CollectorError> {
    let snapshot_id = format!("collector-{}-{}", envelope.channel, &capture_sha256[..16]);
    let source_channel = match envelope.channel {
        Channel::Live => SourceChannel::Live,
        Channel::Pts => SourceChannel::Pts,
    };
    let mut source_records = Vec::with_capacity(envelope.records.len());
    let mut record_ids = BTreeMap::new();
    for record in &envelope.records {
        let content_sha256 = sha256_bytes(&serde_json::to_vec(record)?);
        let record_id = format!("collector-record-{content_sha256}");
        record_ids.insert(record.source_key.clone(), record_id.clone());
        source_records.push(SourceRecordInput {
            record_id,
            snapshot_id: snapshot_id.clone(),
            category: record.category.clone(),
            source_key: record.source_key.clone(),
            content_sha256,
            acquisition_method: "eso-public-addon-api".to_string(),
            import_result: "accepted".to_string(),
        });
    }

    let mut entity_sources = BTreeMap::<EntityRef, BTreeSet<String>>::new();
    let mut attributes = Vec::new();
    let mut relations = Vec::new();
    let mut localized_text = Vec::new();
    let mut icon_references = Vec::new();
    for record in &envelope.records {
        let record_id = record_ids
            .get(&record.source_key)
            .expect("validated source key has an ID")
            .clone();
        let entity = EntityRef {
            kind: record.kind,
            stable_id: record.stable_id,
        };
        entity_sources
            .entry(entity.clone())
            .or_default()
            .insert(record_id.clone());
        for (name, value) in &record.attributes {
            attributes.push(AttributeInput {
                entity: entity.clone(),
                name: name.clone(),
                value: value.clone(),
                source_record: record_id.clone(),
            });
        }
        if let Some(parent) = &record.parent {
            relations.push(RelationInput {
                kind: parent.relation.clone(),
                from: EntityRef {
                    kind: parent.kind,
                    stable_id: parent.stable_id,
                },
                to: entity.clone(),
                source_record: record_id.clone(),
            });
        }
        for (kind, value) in [("name", &record.name), ("description", &record.description)] {
            if let Some(value) = value {
                localized_text.push(LocalizedTextInput {
                    entity: entity.clone(),
                    locale: envelope.locale.clone(),
                    text_kind: kind.to_string(),
                    value: value.clone(),
                    normalized_search: Some(value.to_lowercase()),
                    source_version: envelope.collector_checksum.clone(),
                    redistribution: Redistribution::UserGeneratedOnly,
                    source_record: record_id.clone(),
                });
            }
        }
        if let Some(virtual_path) = &record.icon_path {
            icon_references.push(IconReferenceInput {
                entity,
                virtual_path: virtual_path.clone(),
                availability: "reference-only".to_string(),
                source_record: record_id,
            });
        }
    }
    let entities = entity_sources
        .into_iter()
        .map(|(entity, source_records)| EntityInput {
            entity,
            observed_only: false,
            source_records: source_records.into_iter().collect(),
        })
        .collect();

    let declared: BTreeMap<_, _> = envelope
        .coverage
        .iter()
        .map(|coverage| (coverage.category.as_str(), coverage))
        .collect();
    let coverage = CATALOG_CATEGORIES
        .into_iter()
        .map(|category| {
            if let Some(declaration) = declared.get(category) {
                CoverageInput {
                    category: category.to_string(),
                    snapshot_id: snapshot_id.clone(),
                    locale: envelope.locale.clone(),
                    scope: declaration.scope.clone(),
                    completeness: Completeness::Bounded,
                    limits: declaration.limitations.clone(),
                }
            } else {
                CoverageInput {
                    category: category.to_string(),
                    snapshot_id: snapshot_id.clone(),
                    locale: "all".to_string(),
                    scope: "not-collected".to_string(),
                    completeness: Completeness::Unknown,
                    limits: "category was not selected for this bounded capture".to_string(),
                }
            }
        })
        .collect();

    Ok(CatalogBundle {
        schema_version: INPUT_SCHEMA_VERSION,
        release: ReleaseInput {
            catalog_version: catalog_version.to_string(),
            channel: envelope.channel,
            game_version: envelope.game_version.clone(),
            api_version: envelope.api_version,
            created_at: envelope.finished_at.clone(),
            locales: vec![envelope.locale.clone()],
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
        },
        source_snapshots: vec![SourceSnapshotInput {
            snapshot_id,
            family: "collector".to_string(),
            channel: source_channel,
            game_version: envelope.game_version.clone(),
            api_version: envelope.api_version,
            locale: envelope.locale.clone(),
            revision: envelope.collector_checksum.clone(),
            raw_sha256: capture_sha256.to_string(),
            uri: "user-local-savedvariables".to_string(),
            acquired_at: envelope.finished_at.clone(),
            license_scope: "user-generated-local-only".to_string(),
            acquisition_method: "eso-public-addon-api".to_string(),
            redistribution: Redistribution::UserGeneratedOnly,
        }],
        source_records,
        coverage,
        entities,
        attributes,
        relations,
        aliases: Vec::<AliasInput>::new(),
        localized_text,
        icon_references,
        icon_assets: Vec::<IconAssetInput>::new(),
    })
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), CollectorError> {
    let parent = path
        .parent()
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::Builder::new()
        .prefix(".collector-stage-")
        .tempfile_in(parent)?;
    temporary.as_file_mut().write_all(bytes)?;
    temporary.as_file().sync_all()?;
    let temporary = temporary.into_temp_path();
    crate::atomic_file::persist(temporary, path).map_err(Into::into)
}

fn validate_distinct_paths(input: &Path, output: &Path) -> Result<(), CollectorError> {
    let input = comparable_path(input)?;
    let output = comparable_path(output)?;
    if same_path(&input, &output) {
        rejected("capture input and staging output paths must be distinct")
    } else {
        Ok(())
    }
}

fn comparable_path(path: &Path) -> Result<PathBuf, CollectorError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    if absolute.exists() {
        return Ok(fs::canonicalize(absolute)?);
    }
    let mut ancestor = absolute.as_path();
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        suffix.push(
            ancestor
                .file_name()
                .ok_or_else(|| CollectorError::Validation("path has no existing ancestor".into()))?
                .to_os_string(),
        );
        ancestor = ancestor
            .parent()
            .ok_or_else(|| CollectorError::Validation("path has no existing ancestor".into()))?;
    }
    let mut resolved = fs::canonicalize(ancestor)?;
    for component in suffix.into_iter().rev() {
        resolved.push(component);
    }
    Ok(normalize_lexically(resolved))
}

fn normalize_lexically(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

fn same_path(left: &Path, right: &Path) -> bool {
    #[cfg(windows)]
    {
        left.to_string_lossy().to_lowercase() == right.to_string_lossy().to_lowercase()
    }
    #[cfg(not(windows))]
    {
        left == right
    }
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut result = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut result, "{byte:02x}").expect("write to String");
    }
    result
}

fn rejected<T>(message: impl Into<String>) -> Result<T, CollectorError> {
    Err(CollectorError::Validation(message.into()))
}
