//! Deterministic catalog construction, verification, diffing, and publication.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use rusqlite::{params, Connection, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::model::{canonical_json, CatalogBundle, Channel, MAX_INPUT_BYTES};
use super::schema::{
    artifact_sha256, canonical_row, configure_new_database, create_schema, open_read_only,
    semantic_sha256, verify_connection, DatabaseDiagnostics, IdRange, SCHEMA_VERSION,
};
use super::CatalogError;

#[derive(Debug, Clone)]
pub struct BuildRequest {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub report_path: Option<PathBuf>,
    pub expected_channel: Channel,
}

impl BuildRequest {
    pub fn new(
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        expected_channel: Channel,
    ) -> Self {
        Self {
            input_path: input_path.as_ref().to_path_buf(),
            output_path: output_path.as_ref().to_path_buf(),
            report_path: None,
            expected_channel,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub integrity_check: String,
    pub foreign_key_violations: usize,
    pub read_only_reopen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyReport {
    pub schema_version: u32,
    pub catalog_version: String,
    pub channel: Channel,
    pub game_version: String,
    pub api_version: u32,
    pub semantic_sha256: String,
    pub artifact_sha256: String,
    pub integrity_check: String,
    pub foreign_key_violations: usize,
    pub entity_counts: BTreeMap<String, usize>,
    pub coverage_counts: BTreeMap<String, usize>,
    pub id_ranges: BTreeMap<String, IdRange>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub changed: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogDiff {
    pub entities: SurfaceDiff,
    pub localized_text: SurfaceDiff,
    #[serde(default)]
    pub localized_text_redistribution_changes: Vec<String>,
    pub relations: SurfaceDiff,
    pub coverage: SurfaceDiff,
    #[serde(default)]
    pub coverage_regressions: Vec<String>,
    pub icon_references: SurfaceDiff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildReport {
    pub status: String,
    pub schema_version: u32,
    pub catalog_version: String,
    pub channel: Channel,
    pub game_version: String,
    pub api_version: u32,
    pub input_sha256: String,
    pub source_set_sha256: String,
    pub semantic_sha256: String,
    pub artifact_sha256: String,
    pub validation: ValidationReport,
    pub entity_counts: BTreeMap<String, usize>,
    pub coverage_counts: BTreeMap<String, usize>,
    pub id_ranges: BTreeMap<String, IdRange>,
    pub diff: CatalogDiff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FailureReport {
    status: &'static str,
    input: String,
    output: String,
    errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RollbackManifest {
    schema_version: u32,
    destination: String,
    rollback: String,
    artifact_sha256: String,
    semantic_sha256: String,
    catalog_version: String,
}

pub fn build_catalog(request: &BuildRequest) -> Result<BuildReport, CatalogError> {
    match build_catalog_inner(request) {
        Ok(report) => Ok(report),
        Err(error) => {
            if failure_report_path_is_safe(request) {
                if let Some(path) = &request.report_path {
                    let report = FailureReport {
                        status: "failed",
                        input: request.input_path.display().to_string(),
                        output: request.output_path.display().to_string(),
                        errors: vec![error.to_string()],
                    };
                    let _ = write_json_atomic(path, &report);
                }
            }
            Err(error)
        }
    }
}

fn build_catalog_inner(request: &BuildRequest) -> Result<BuildReport, CatalogError> {
    validate_artifact_paths(request, None)?;
    let metadata = fs::metadata(&request.input_path)?;
    if metadata.len() > MAX_INPUT_BYTES {
        return Err(CatalogError::Validation(format!(
            "catalog input is {} bytes, limit is {MAX_INPUT_BYTES}",
            metadata.len()
        )));
    }
    let raw_input = fs::read(&request.input_path)?;
    let bundle: CatalogBundle = serde_json::from_slice(&raw_input)?;
    let bundle = bundle.normalize_and_validate()?;
    if bundle.release.channel != request.expected_channel {
        return Err(CatalogError::Validation(format!(
            "input channel {} does not match requested channel {}",
            bundle.release.channel, request.expected_channel
        )));
    }
    let input_sha256 = sha256_json(&bundle)?;
    let source_set_sha256 = sha256_json(&bundle.source_snapshots)?;

    let existing = if request.output_path.is_file() {
        let existing = verify_catalog(&request.output_path)?;
        if existing.channel != bundle.release.channel {
            return Err(CatalogError::Validation(format!(
                "refusing to replace {} catalog with {} catalog at {}",
                existing.channel,
                bundle.release.channel,
                request.output_path.display()
            )));
        }
        validate_artifact_paths(request, Some(&existing.artifact_sha256))?;
        Some(existing)
    } else {
        None
    };

    let output_parent = request
        .output_path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(output_parent)?;
    let candidate = tempfile::Builder::new()
        .prefix(".catalog-candidate-")
        .suffix(".sqlite")
        .tempfile_in(output_parent)?
        .into_temp_path();
    let mut connection = Connection::open(&candidate)?;
    configure_new_database(&connection)?;
    let transaction = connection.transaction()?;
    create_schema(&transaction)?;
    insert_bundle(&transaction, &bundle, &input_sha256, &source_set_sha256)?;
    let semantic_hash = semantic_sha256(&transaction)?;
    transaction.execute(
        "UPDATE catalog_release SET semantic_sha256 = ?1 WHERE singleton = 1",
        [&semantic_hash],
    )?;
    transaction.commit()?;
    connection.close().map_err(|(_, error)| error)?;

    OpenOptions::new()
        .read(true)
        .write(true)
        .open(&candidate)?
        .sync_all()?;
    let candidate_connection = open_read_only(&candidate)?;
    let diagnostics = verify_connection(&candidate_connection)?;
    drop(candidate_connection);
    let candidate_artifact_sha256 = artifact_sha256(&candidate)?;

    let diff = if let Some(existing) = &existing {
        let diff = diff_catalogs(&request.output_path, &candidate)?;
        preserve_rollback(&request.output_path, existing)?;
        diff
    } else {
        CatalogDiff::default()
    };

    let report = build_report(
        &bundle,
        input_sha256,
        source_set_sha256,
        candidate_artifact_sha256,
        diagnostics,
        diff,
    );
    if let Some(path) = &request.report_path {
        write_json_atomic(path, &report)?;
    }
    persist_candidate(candidate, &request.output_path)?;
    Ok(report)
}

fn build_report(
    bundle: &CatalogBundle,
    input_sha256: String,
    source_set_sha256: String,
    artifact_sha256: String,
    diagnostics: DatabaseDiagnostics,
    diff: CatalogDiff,
) -> BuildReport {
    BuildReport {
        status: "published".to_string(),
        schema_version: diagnostics.schema_version,
        catalog_version: bundle.release.catalog_version.clone(),
        channel: bundle.release.channel,
        game_version: bundle.release.game_version.clone(),
        api_version: bundle.release.api_version,
        input_sha256,
        source_set_sha256,
        semantic_sha256: diagnostics.semantic_sha256,
        artifact_sha256,
        validation: ValidationReport {
            integrity_check: diagnostics.integrity_check,
            foreign_key_violations: diagnostics.foreign_key_violations,
            read_only_reopen: true,
        },
        entity_counts: diagnostics.entity_counts,
        coverage_counts: diagnostics.coverage_counts,
        id_ranges: diagnostics.id_ranges,
        diff,
    }
}

fn insert_bundle(
    transaction: &Transaction<'_>,
    bundle: &CatalogBundle,
    input_sha256: &str,
    source_set_sha256: &str,
) -> Result<(), CatalogError> {
    let release = &bundle.release;
    transaction.execute(
        "INSERT INTO catalog_release(
            singleton, schema_version, catalog_version, channel, game_version,
            api_version, created_at, locales_json, tool_version, input_sha256,
            source_set_sha256, semantic_sha256
        ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, '')",
        params![
            SCHEMA_VERSION,
            release.catalog_version,
            release.channel.as_str(),
            release.game_version,
            release.api_version,
            release.created_at,
            serde_json::to_string(&release.locales)?,
            release.tool_version,
            input_sha256,
            source_set_sha256,
        ],
    )?;

    {
        let mut statement = transaction.prepare(
            "INSERT INTO source_snapshot VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13
            )",
        )?;
        for source in &bundle.source_snapshots {
            statement.execute(params![
                source.snapshot_id,
                source.family,
                source.channel.as_str(),
                source.game_version,
                source.api_version,
                source.locale,
                source.revision,
                source.raw_sha256,
                source.uri,
                source.acquired_at,
                source.license_scope,
                source.acquisition_method,
                source.redistribution.as_str(),
            ])?;
        }
    }
    {
        let mut statement =
            transaction.prepare("INSERT INTO source_record VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;
        for record in &bundle.source_records {
            statement.execute(params![
                record.record_id,
                record.snapshot_id,
                record.category,
                record.source_key,
                record.content_sha256,
                record.acquisition_method,
                record.import_result,
            ])?;
        }
    }
    {
        let mut statement =
            transaction.prepare("INSERT INTO coverage VALUES (?1, ?2, ?3, ?4, ?5, ?6)")?;
        for coverage in &bundle.coverage {
            statement.execute(params![
                coverage.category,
                coverage.snapshot_id,
                coverage.locale,
                coverage.scope,
                coverage.completeness.as_str(),
                coverage.limits,
            ])?;
        }
    }
    {
        let mut entity_statement =
            transaction.prepare("INSERT INTO entity VALUES (?1, ?2, ?3, ?4, ?5)")?;
        let mut source_statement =
            transaction.prepare("INSERT INTO entity_source VALUES (?1, ?2, ?3, ?4, ?5)")?;
        for entity in &bundle.entities {
            entity_statement.execute(params![
                entity.entity.kind.as_str(),
                entity.entity.stable_id,
                release.channel.as_str(),
                release.api_version,
                entity.observed_only,
            ])?;
            for record in &entity.source_records {
                source_statement.execute(params![
                    entity.entity.kind.as_str(),
                    entity.entity.stable_id,
                    release.channel.as_str(),
                    release.api_version,
                    record,
                ])?;
            }
        }
    }
    {
        let mut statement = transaction.prepare(
            "INSERT INTO entity_attribute VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10
            )",
        )?;
        for attribute in &bundle.attributes {
            let (value_type, integer, real, text) = attribute_columns(&attribute.value)?;
            statement.execute(params![
                attribute.entity.kind.as_str(),
                attribute.entity.stable_id,
                release.channel.as_str(),
                release.api_version,
                attribute.name,
                value_type,
                integer,
                real,
                text,
                attribute.source_record,
            ])?;
        }
    }
    {
        let mut statement = transaction
            .prepare("INSERT INTO entity_relation VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?;
        for relation in &bundle.relations {
            statement.execute(params![
                relation.kind,
                relation.from.kind.as_str(),
                relation.from.stable_id,
                relation.to.kind.as_str(),
                relation.to.stable_id,
                release.channel.as_str(),
                release.api_version,
                relation.source_record,
            ])?;
        }
    }
    {
        let mut statement = transaction
            .prepare("INSERT INTO entity_alias VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")?;
        for alias in &bundle.aliases {
            statement.execute(params![
                alias.kind,
                alias.from.kind.as_str(),
                alias.from.stable_id,
                alias.to.kind.as_str(),
                alias.to.stable_id,
                release.channel.as_str(),
                release.api_version,
                alias.source_record,
            ])?;
        }
    }
    {
        let mut statement = transaction.prepare(
            "INSERT INTO localized_text VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11
            )",
        )?;
        for text in &bundle.localized_text {
            statement.execute(params![
                text.entity.kind.as_str(),
                text.entity.stable_id,
                release.channel.as_str(),
                release.api_version,
                text.locale,
                text.text_kind,
                text.value,
                text.normalized_search,
                text.source_version,
                text.redistribution.as_str(),
                text.source_record,
            ])?;
        }
    }
    {
        let mut statement = transaction
            .prepare("INSERT INTO icon_reference VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;
        for icon in &bundle.icon_references {
            statement.execute(params![
                icon.entity.kind.as_str(),
                icon.entity.stable_id,
                release.channel.as_str(),
                release.api_version,
                icon.virtual_path,
                icon.availability,
                icon.source_record,
            ])?;
        }
    }
    {
        let mut statement = transaction
            .prepare("INSERT INTO icon_asset VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?;
        for asset in &bundle.icon_assets {
            statement.execute(params![
                asset.content_sha256,
                asset.transformation_id,
                asset.media_type,
                asset.width,
                asset.height,
                asset.origin,
                asset.attribution,
                asset.availability,
                asset.redistribution.as_str(),
            ])?;
        }
    }
    Ok(())
}

type AttributeColumns = (&'static str, Option<i64>, Option<f64>, Option<String>);

fn attribute_columns(value: &serde_json::Value) -> Result<AttributeColumns, CatalogError> {
    match value {
        serde_json::Value::Bool(value) => Ok(("boolean", Some(i64::from(*value)), None, None)),
        serde_json::Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                Ok(("integer", Some(value), None, None))
            } else if let Some(value) = value.as_u64() {
                let value = i64::try_from(value).map_err(|_| {
                    CatalogError::Validation("attribute integer exceeds SQLite range".to_string())
                })?;
                Ok(("integer", Some(value), None, None))
            } else {
                Ok(("real", None, value.as_f64(), None))
            }
        }
        serde_json::Value::String(value) => Ok(("text", None, None, Some(value.clone()))),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            Ok(("json", None, None, Some(canonical_json(value))))
        }
        serde_json::Value::Null => Err(CatalogError::Validation(
            "attribute value cannot be null".to_string(),
        )),
    }
}

fn preserve_rollback(destination: &Path, existing: &VerifyReport) -> Result<(), CatalogError> {
    let rollback = rollback_generation_path(destination, &existing.artifact_sha256);
    let manifest = rollback_manifest_path(destination);
    if rollback.exists() {
        let rollback_hash = artifact_sha256(&rollback)?;
        if rollback_hash != existing.artifact_sha256 {
            return Err(CatalogError::Validation(format!(
                "rollback generation {} does not match the last known-good catalog",
                rollback.display()
            )));
        }
    } else {
        let parent = rollback
            .parent()
            .filter(|value| !value.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let temporary = tempfile::Builder::new()
            .prefix(".catalog-rollback-")
            .tempfile_in(parent)?
            .into_temp_path();
        fs::copy(destination, &temporary)?;
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(&temporary)?
            .sync_all()?;
        let rollback_hash = artifact_sha256(&temporary)?;
        if rollback_hash != existing.artifact_sha256 {
            return Err(CatalogError::Validation(
                "rollback copy hash differs from the last known-good catalog".to_string(),
            ));
        }
        persist_candidate(temporary, &rollback)?;
    }
    write_json_atomic(
        &manifest,
        &RollbackManifest {
            schema_version: existing.schema_version,
            destination: destination.display().to_string(),
            rollback: rollback.display().to_string(),
            artifact_sha256: existing.artifact_sha256.clone(),
            semantic_sha256: existing.semantic_sha256.clone(),
            catalog_version: existing.catalog_version.clone(),
        },
    )
}

fn rollback_manifest_path(destination: &Path) -> PathBuf {
    suffixed_path(destination, ".rollback.json")
}

fn rollback_generation_path(destination: &Path, artifact_sha256: &str) -> PathBuf {
    suffixed_path(destination, &format!(".rollback.{artifact_sha256}.sqlite"))
}

fn suffixed_path(path: &Path, suffix: &str) -> PathBuf {
    let mut file_name = path
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("catalog.sqlite"))
        .to_os_string();
    file_name.push(suffix);
    path.with_file_name(file_name)
}

fn validate_artifact_paths(
    request: &BuildRequest,
    rollback_sha256: Option<&str>,
) -> Result<(), CatalogError> {
    let input = comparable_path(&request.input_path)?;
    let output = comparable_path(&request.output_path)?;
    if same_path(&input, &output) {
        return Err(CatalogError::Validation(
            "catalog input and output paths must be distinct".to_string(),
        ));
    }
    let Some(report_path) = &request.report_path else {
        return Ok(());
    };
    let report = comparable_path(report_path)?;
    let manifest_path = rollback_manifest_path(&request.output_path);
    let mut reserved = vec![
        ("input", input),
        ("output", output.clone()),
        ("rollback manifest", comparable_path(&manifest_path)?),
    ];
    if let Some(artifact_sha256) = rollback_sha256 {
        reserved.push((
            "rollback generation",
            comparable_path(&rollback_generation_path(
                &request.output_path,
                artifact_sha256,
            ))?,
        ));
    }
    if manifest_path.is_file() {
        if let Ok(bytes) = fs::read(&manifest_path) {
            if let Ok(manifest) = serde_json::from_slice::<RollbackManifest>(&bytes) {
                reserved.push((
                    "active rollback generation",
                    comparable_path(Path::new(&manifest.rollback))?,
                ));
            }
        }
    }
    for (label, path) in reserved {
        if same_path(&report, &path) {
            return Err(CatalogError::Validation(format!(
                "report path must not alias the catalog {label}"
            )));
        }
    }
    let artifact_parent = comparable_path(
        request
            .output_path
            .parent()
            .filter(|value| !value.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new(".")),
    )?;
    if same_path(
        report.parent().unwrap_or_else(|| Path::new(".")),
        &artifact_parent,
    ) {
        let output_name = comparable_name(
            request
                .output_path
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("catalog.sqlite")),
        );
        let report_name = comparable_name(
            report
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("")),
        );
        if report_name.starts_with(&format!("{output_name}.rollback")) {
            return Err(CatalogError::Validation(
                "report path must not use a catalog rollback artifact path".to_string(),
            ));
        }
    }
    Ok(())
}

fn failure_report_path_is_safe(request: &BuildRequest) -> bool {
    let existing = request
        .output_path
        .is_file()
        .then(|| verify_catalog(&request.output_path).ok())
        .flatten();
    validate_artifact_paths(
        request,
        existing
            .as_ref()
            .map(|report| report.artifact_sha256.as_str()),
    )
    .is_ok()
}

fn comparable_path(path: &Path) -> Result<PathBuf, CatalogError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    if absolute.exists() {
        return Ok(fs::canonicalize(absolute)?);
    }
    let mut ancestor = absolute.as_path();
    let mut suffix = Vec::<OsString>::new();
    while !ancestor.exists() {
        let name = ancestor.file_name().ok_or_else(|| {
            CatalogError::Validation(format!("path has no existing ancestor: {}", path.display()))
        })?;
        suffix.push(name.to_os_string());
        ancestor = ancestor.parent().ok_or_else(|| {
            CatalogError::Validation(format!("path has no existing ancestor: {}", path.display()))
        })?;
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

fn comparable_name(value: &std::ffi::OsStr) -> String {
    let value = value.to_string_lossy();
    #[cfg(windows)]
    {
        value.to_lowercase()
    }
    #[cfg(not(windows))]
    {
        value.into_owned()
    }
}

pub fn verify_catalog(path: impl AsRef<Path>) -> Result<VerifyReport, CatalogError> {
    let path = path.as_ref();
    let connection = open_read_only(path)?;
    let diagnostics = verify_connection(&connection)?;
    let (catalog_version, channel, game_version, api_version): (String, String, String, u32) =
        connection.query_row(
            "SELECT catalog_version, channel, game_version, api_version
             FROM catalog_release WHERE singleton = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )?;
    drop(connection);
    Ok(VerifyReport {
        schema_version: diagnostics.schema_version,
        catalog_version,
        channel: parse_channel(&channel)?,
        game_version,
        api_version,
        semantic_sha256: diagnostics.semantic_sha256,
        artifact_sha256: artifact_sha256(path)?,
        integrity_check: diagnostics.integrity_check,
        foreign_key_violations: diagnostics.foreign_key_violations,
        entity_counts: diagnostics.entity_counts,
        coverage_counts: diagnostics.coverage_counts,
        id_ranges: diagnostics.id_ranges,
    })
}

pub fn diff_catalogs(
    old_path: impl AsRef<Path>,
    new_path: impl AsRef<Path>,
) -> Result<CatalogDiff, CatalogError> {
    let old = open_read_only(old_path.as_ref())?;
    let new = open_read_only(new_path.as_ref())?;
    verify_connection(&old)?;
    verify_connection(&new)?;
    let localized_text_redistribution_changes =
        value_changes(text_redistribution(&old)?, text_redistribution(&new)?);
    let coverage_regressions = coverage_regressions(&old, &new)?;
    Ok(CatalogDiff {
        entities: compare_surfaces(entity_surface(&old)?, entity_surface(&new)?),
        localized_text: compare_surfaces(
            surface(
                &old,
                "SELECT kind, stable_id, channel, api_version, locale, text_kind,
                        value, normalized_search, source_version, redistribution, source_record_id
                 FROM localized_text
                 ORDER BY kind, stable_id, channel, api_version, locale, text_kind",
                6,
            )?,
            surface(
                &new,
                "SELECT kind, stable_id, channel, api_version, locale, text_kind,
                        value, normalized_search, source_version, redistribution, source_record_id
                 FROM localized_text
                 ORDER BY kind, stable_id, channel, api_version, locale, text_kind",
                6,
            )?,
        ),
        localized_text_redistribution_changes,
        relations: compare_surfaces(
            surface(
                &old,
                "SELECT relation_kind, from_kind, from_stable_id, to_kind, to_stable_id,
                        channel, api_version, source_record_id
                 FROM entity_relation
                 ORDER BY relation_kind, from_kind, from_stable_id, to_kind, to_stable_id,
                          channel, api_version",
                7,
            )?,
            surface(
                &new,
                "SELECT relation_kind, from_kind, from_stable_id, to_kind, to_stable_id,
                        channel, api_version, source_record_id
                 FROM entity_relation
                 ORDER BY relation_kind, from_kind, from_stable_id, to_kind, to_stable_id,
                          channel, api_version",
                7,
            )?,
        ),
        coverage: compare_surfaces(
            surface(
                &old,
                "SELECT category, snapshot_id, locale, scope, completeness, limits_text
                 FROM coverage ORDER BY category, snapshot_id, locale, scope",
                4,
            )?,
            surface(
                &new,
                "SELECT category, snapshot_id, locale, scope, completeness, limits_text
                 FROM coverage ORDER BY category, snapshot_id, locale, scope",
                4,
            )?,
        ),
        coverage_regressions,
        icon_references: compare_surfaces(
            surface(
                &old,
                "SELECT kind, stable_id, channel, api_version, virtual_path,
                        availability, source_record_id
                 FROM icon_reference
                 ORDER BY kind, stable_id, channel, api_version, virtual_path",
                5,
            )?,
            surface(
                &new,
                "SELECT kind, stable_id, channel, api_version, virtual_path,
                        availability, source_record_id
                 FROM icon_reference
                 ORDER BY kind, stable_id, channel, api_version, virtual_path",
                5,
            )?,
        ),
    })
}

fn text_redistribution(connection: &Connection) -> Result<BTreeMap<String, String>, CatalogError> {
    let mut statement = connection.prepare(
        "SELECT kind, stable_id, channel, api_version, locale, text_kind, redistribution
         FROM localized_text ORDER BY kind, stable_id, channel, api_version, locale, text_kind",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            format!(
                "{}|{}|{}|{}|{}|{}",
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?
            ),
            row.get::<_, String>(6)?,
        ))
    })?;
    rows.collect::<Result<BTreeMap<_, _>, _>>()
        .map_err(CatalogError::from)
}

fn value_changes(old: BTreeMap<String, String>, new: BTreeMap<String, String>) -> Vec<String> {
    old.into_iter()
        .filter_map(|(key, old_value)| {
            new.get(&key)
                .is_some_and(|new_value| new_value != &old_value)
                .then_some(key)
        })
        .collect()
}

fn coverage_regressions(old: &Connection, new: &Connection) -> Result<Vec<String>, CatalogError> {
    let old = coverage_completeness(old)?;
    let new = coverage_completeness(new)?;
    let mut regressions = Vec::new();
    for (key, old_value) in old {
        let Some(new_value) = new.get(&key) else {
            continue;
        };
        if completeness_rank(new_value)? < completeness_rank(&old_value)? {
            regressions.push(key);
        }
    }
    Ok(regressions)
}

fn coverage_completeness(
    connection: &Connection,
) -> Result<BTreeMap<String, String>, CatalogError> {
    let mut statement = connection.prepare(
        "SELECT category, snapshot_id, locale, scope, completeness
         FROM coverage ORDER BY category, snapshot_id, locale, scope",
    )?;
    let rows = statement.query_map([], |row| {
        Ok((
            format!(
                "{}|{}|{}|{}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?
            ),
            row.get::<_, String>(4)?,
        ))
    })?;
    rows.collect::<Result<BTreeMap<_, _>, _>>()
        .map_err(CatalogError::from)
}

fn completeness_rank(value: &str) -> Result<u8, CatalogError> {
    match value {
        "unknown" => Ok(0),
        "opportunistic" => Ok(1),
        "bounded" => Ok(2),
        "exhaustive" => Ok(3),
        _ => Err(CatalogError::Validation(format!(
            "catalog contains invalid coverage completeness {value}"
        ))),
    }
}

fn entity_surface(connection: &Connection) -> Result<BTreeMap<String, String>, CatalogError> {
    let mut values: BTreeMap<String, Vec<String>> = BTreeMap::new();
    append_surface(
        connection,
        &mut values,
        "SELECT kind, stable_id, channel, api_version, observed_only
         FROM entity ORDER BY kind, stable_id, channel, api_version",
        4,
        "entity",
    )?;
    append_surface(
        connection,
        &mut values,
        "SELECT kind, stable_id, channel, api_version, record_id
         FROM entity_source ORDER BY kind, stable_id, channel, api_version, record_id",
        4,
        "source",
    )?;
    append_surface(
        connection,
        &mut values,
        "SELECT kind, stable_id, channel, api_version, name, value_type,
                integer_value, real_value, text_value, source_record_id
         FROM entity_attribute ORDER BY kind, stable_id, channel, api_version, name",
        4,
        "attribute",
    )?;
    Ok(values
        .into_iter()
        .map(|(key, mut rows)| {
            rows.sort();
            (key, rows.join("\n"))
        })
        .collect())
}

fn append_surface(
    connection: &Connection,
    destination: &mut BTreeMap<String, Vec<String>>,
    sql: &str,
    key_columns: usize,
    label: &str,
) -> Result<(), CatalogError> {
    let mut statement = connection.prepare(sql)?;
    let columns = statement.column_count();
    let mut rows = statement.query([])?;
    while let Some(row) = rows.next()? {
        let key = (0..key_columns)
            .map(|index| row.get_ref(index).map(canonical_row))
            .collect::<Result<Vec<_>, _>>()?
            .join("|");
        let value = (key_columns..columns)
            .map(|index| row.get_ref(index).map(canonical_row))
            .collect::<Result<Vec<_>, _>>()?
            .join("|");
        destination
            .entry(key)
            .or_default()
            .push(format!("{label}|{value}"));
    }
    Ok(())
}

fn surface(
    connection: &Connection,
    sql: &str,
    key_columns: usize,
) -> Result<BTreeMap<String, String>, CatalogError> {
    let mut values = BTreeMap::<String, Vec<String>>::new();
    append_surface(connection, &mut values, sql, key_columns, "row")?;
    Ok(values
        .into_iter()
        .map(|(key, rows)| (key, rows.join("\n")))
        .collect())
}

fn compare_surfaces(old: BTreeMap<String, String>, new: BTreeMap<String, String>) -> SurfaceDiff {
    let mut diff = SurfaceDiff::default();
    for (key, old_value) in &old {
        match new.get(key) {
            None => diff.removed.push(key.clone()),
            Some(new_value) if new_value != old_value => diff.changed.push(key.clone()),
            Some(_) => {}
        }
    }
    for key in new.keys() {
        if !old.contains_key(key) {
            diff.added.push(key.clone());
        }
    }
    diff
}

fn parse_channel(value: &str) -> Result<Channel, CatalogError> {
    match value {
        "live" => Ok(Channel::Live),
        "pts" => Ok(Channel::Pts),
        _ => Err(CatalogError::Validation(format!(
            "catalog contains invalid channel {value}"
        ))),
    }
}

fn sha256_json(value: &impl Serialize) -> Result<String, CatalogError> {
    Ok(sha256_bytes(&serde_json::to_vec(value)?))
}

fn sha256_bytes(value: &[u8]) -> String {
    let digest = Sha256::digest(value);
    let mut result = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut result, "{byte:02x}").expect("write to String");
    }
    result
}

fn write_json_atomic(path: &Path, value: &impl Serialize) -> Result<(), CatalogError> {
    let parent = path
        .parent()
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::Builder::new()
        .prefix(".catalog-report-")
        .tempfile_in(parent)?;
    serde_json::to_writer_pretty(temporary.as_file_mut(), value)?;
    temporary.as_file_mut().write_all(b"\n")?;
    temporary.as_file().sync_all()?;
    temporary.persist(path).map_err(|error| error.error)?;
    Ok(())
}

#[cfg(windows)]
fn persist_candidate(
    candidate: tempfile::TempPath,
    destination: &Path,
) -> Result<(), CatalogError> {
    if !destination.exists() {
        candidate
            .persist(destination)
            .map_err(|error| error.error)?;
        return Ok(());
    }
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{ReplaceFileW, REPLACEFILE_WRITE_THROUGH};

    let destination_wide: Vec<u16> = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let candidate_wide: Vec<u16> = candidate
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    // SAFETY: both paths are owned, NUL-terminated UTF-16 buffers that remain
    // alive for the call. Null optional pointers request no OS-created backup;
    // the verified rollback copy was already synced separately.
    let replaced = unsafe {
        ReplaceFileW(
            destination_wide.as_ptr(),
            candidate_wide.as_ptr(),
            std::ptr::null(),
            REPLACEFILE_WRITE_THROUGH,
            std::ptr::null(),
            std::ptr::null(),
        )
    };
    if replaced == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(())
}

#[cfg(not(windows))]
fn persist_candidate(
    candidate: tempfile::TempPath,
    destination: &Path,
) -> Result<(), CatalogError> {
    candidate
        .persist(destination)
        .map_err(|error| error.error)?;
    Ok(())
}
