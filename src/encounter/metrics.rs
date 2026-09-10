use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::catalog::{CatalogAccess, Channel, EntityKind};

use super::{
    canonical_bytes, ensure_distinct_paths, invalid, load_encounter, sha256, EncounterCapture,
    EncounterError, EncounterEvent, PayloadValue,
};

pub const PROJECTION_SCHEMA_VERSION: u32 = 1;
pub const ALGORITHM_VERSION: &str = "s069-v1";
const PLAYER_SOURCE_TYPE: i64 = 1;

#[derive(Debug, Clone)]
pub struct ProjectionRequest {
    pub store_path: PathBuf,
    pub catalog_path: PathBuf,
    pub output_path: PathBuf,
    pub session_id: String,
    pub encounter_id: String,
}

impl ProjectionRequest {
    pub fn new(
        store_path: impl Into<PathBuf>,
        catalog_path: impl Into<PathBuf>,
        output_path: impl Into<PathBuf>,
        session_id: impl Into<String>,
        encounter_id: impl Into<String>,
    ) -> Self {
        Self {
            store_path: store_path.into(),
            catalog_path: catalog_path.into(),
            output_path: output_path.into(),
            session_id: session_id.into(),
            encounter_id: encounter_id.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetricQuality {
    Complete,
    Degraded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LossRange {
    pub missing_sequence_from: u64,
    pub missing_sequence_to: u64,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MetricResult {
    pub metric_id: String,
    pub value: Option<f64>,
    pub unit: String,
    pub algorithm_version: String,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub quality: MetricQuality,
    pub loss_ranges: Vec<LossRange>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AbilityDamageShare {
    pub ability_id: i64,
    #[serde(flatten)]
    pub result: MetricResult,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EffectUptime {
    pub ability_id: i64,
    #[serde(flatten)]
    pub result: MetricResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderedCastSequence {
    pub metric_id: String,
    pub ability_ids: Vec<i64>,
    pub unit: String,
    pub algorithm_version: String,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub quality: MetricQuality,
    pub loss_ranges: Vec<LossRange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CatalogJoinReceipt {
    pub catalog_schema_version: u32,
    pub catalog_version: String,
    pub catalog_semantic_sha256: String,
    pub channel: Channel,
    pub api_version: u32,
    pub raw_content_sha256: String,
    pub known_ids: Vec<i64>,
    pub unknown_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EncounterProjection {
    pub schema_version: u32,
    pub algorithm_version: String,
    pub session_id: String,
    pub encounter_id: String,
    pub channel: Channel,
    pub duration_ms: u64,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub raw_content_sha256: String,
    pub catalog_join: CatalogJoinReceipt,
    pub observed_dps: MetricResult,
    pub effective_hps: MetricResult,
    pub ability_damage_share: Vec<AbilityDamageShare>,
    pub effect_uptime: Vec<EffectUptime>,
    pub ordered_cast_sequence: OrderedCastSequence,
}

pub fn project_encounter(
    request: &ProjectionRequest,
) -> Result<EncounterProjection, EncounterError> {
    ensure_distinct_paths(&request.store_path, &request.catalog_path)?;
    ensure_distinct_paths(&request.store_path, &request.output_path)?;
    ensure_distinct_paths(&request.catalog_path, &request.output_path)?;
    reject_link_like_output(&request.output_path)?;

    let capture = load_encounter(
        &request.store_path,
        &request.session_id,
        &request.encounter_id,
    )?
    .ok_or_else(|| EncounterError::Validation("encounter identity was not found".into()))?;
    let catalog = CatalogAccess::open_or_empty(&request.catalog_path);
    if let Some(diagnostic) = catalog.diagnostic() {
        return invalid(format!(
            "catalog is unavailable or invalid ({:?})",
            diagnostic.kind
        ));
    }
    let projection = calculate_projection(&capture, &catalog)?;
    let bytes = canonical_projection_bytes(&projection)?;
    publish(&request.output_path, &bytes)?;
    Ok(projection)
}

pub fn calculate_projection(
    capture: &EncounterCapture,
    catalog: &CatalogAccess,
) -> Result<EncounterProjection, EncounterError> {
    let raw_content_sha256 = sha256(&canonical_bytes(capture)?);
    let release = catalog
        .release()
        .map_err(|error| EncounterError::Validation(format!("catalog validation failed: {error}")))?
        .ok_or_else(|| EncounterError::Validation("catalog is unavailable".into()))?;
    if release.channel != capture.channel || release.api_version != capture.source.api_version {
        return invalid(format!(
            "catalog {} API {} does not match capture {} API {}",
            release.channel, release.api_version, capture.channel, capture.source.api_version
        ));
    }

    let mut events = capture.events.iter().collect::<Vec<_>>();
    events.sort_by_key(|event| event.sequence);
    let loss_ranges = collect_loss_ranges(&events)?;
    let quality = if loss_ranges.is_empty() {
        MetricQuality::Complete
    } else {
        MetricQuality::Degraded
    };
    let evidence = Evidence {
        first_sequence: capture.first_sequence,
        last_sequence: capture.last_sequence,
        quality,
        loss_ranges: loss_ranges.clone(),
    };
    let duration_ms = capture.ended_monotonic_ms;
    let duration_seconds = (duration_ms != 0).then(|| duration_ms as f64 / 1000.0);

    let mut total_damage = 0_u64;
    let mut total_healing = 0_u64;
    let mut damage_by_ability = BTreeMap::<i64, u64>::new();
    let mut effect_intervals = BTreeMap::<i64, Vec<(u64, u64)>>::new();
    let mut cast_ids = Vec::new();
    let mut ability_references = BTreeSet::new();
    let mut effect_references = BTreeSet::new();

    for event in &events {
        collect_catalog_reference(event, &mut ability_references, &mut effect_references);
        match event.kind.as_str() {
            "damage" if integer(event, "source_type") == Some(PLAYER_SOURCE_TYPE) => {
                let amount = nonnegative(event, "amount")?;
                total_damage = checked_add(total_damage, amount, "damage total")?;
                if let Some(ability_id) = positive_id(event, "ability_id") {
                    let current = damage_by_ability.entry(ability_id).or_default();
                    *current = checked_add(*current, amount, "ability damage total")?;
                }
            }
            "healing" if integer(event, "source_type") == Some(PLAYER_SOURCE_TYPE) => {
                let amount = nonnegative(event, "amount")?;
                let overflow = optional_nonnegative(event, "overflow")?.unwrap_or(0);
                total_healing = checked_add(
                    total_healing,
                    amount.saturating_sub(overflow),
                    "healing total",
                )?;
            }
            "effect" => collect_effect_interval(event, duration_ms, &mut effect_intervals)?,
            "cast" => {
                if let Some(ability_id) = positive_id(event, "ability_id") {
                    cast_ids.push(ability_id);
                }
            }
            _ => {}
        }
    }

    let observed_dps = metric(
        "observed-dps",
        duration_seconds.map(|seconds| total_damage as f64 / seconds),
        "damage-per-second",
        &evidence,
    );
    let effective_hps = metric(
        "observed-hps",
        duration_seconds.map(|seconds| total_healing as f64 / seconds),
        "effective-healing-per-second",
        &evidence,
    );
    let ability_damage_share = if total_damage == 0 {
        Vec::new()
    } else {
        damage_by_ability
            .into_iter()
            .map(|(ability_id, amount)| AbilityDamageShare {
                ability_id,
                result: metric(
                    "ability-damage-share",
                    Some(amount as f64 / total_damage as f64),
                    "ratio",
                    &evidence,
                ),
            })
            .collect()
    };
    let effect_uptime = effect_intervals
        .into_iter()
        .map(|(ability_id, intervals)| {
            let active_ms = union_length(intervals)?;
            Ok(EffectUptime {
                ability_id,
                result: metric(
                    "effect-uptime",
                    duration_seconds.map(|_| active_ms as f64 / duration_ms as f64),
                    "ratio",
                    &evidence,
                ),
            })
        })
        .collect::<Result<Vec<_>, EncounterError>>()?;
    let catalog_join = join_catalog(
        catalog,
        &release,
        raw_content_sha256.clone(),
        ability_references,
        effect_references,
    )?;

    Ok(EncounterProjection {
        schema_version: PROJECTION_SCHEMA_VERSION,
        algorithm_version: ALGORITHM_VERSION.into(),
        session_id: capture.session_id.clone(),
        encounter_id: capture.encounter_id.clone(),
        channel: capture.channel,
        duration_ms,
        first_sequence: capture.first_sequence,
        last_sequence: capture.last_sequence,
        raw_content_sha256,
        catalog_join,
        observed_dps,
        effective_hps,
        ability_damage_share,
        effect_uptime,
        ordered_cast_sequence: OrderedCastSequence {
            metric_id: "ordered-cast-sequence".into(),
            ability_ids: cast_ids,
            unit: "ability-id-sequence".into(),
            algorithm_version: ALGORITHM_VERSION.into(),
            first_sequence: evidence.first_sequence,
            last_sequence: evidence.last_sequence,
            quality: evidence.quality,
            loss_ranges,
        },
    })
}

pub fn canonical_projection_bytes(
    projection: &EncounterProjection,
) -> Result<Vec<u8>, EncounterError> {
    let mut bytes = serde_json::to_vec(projection).map_err(|error| {
        EncounterError::Validation(format!("projection serialization failed: {error}"))
    })?;
    bytes.push(b'\n');
    Ok(bytes)
}

#[derive(Clone)]
struct Evidence {
    first_sequence: u64,
    last_sequence: u64,
    quality: MetricQuality,
    loss_ranges: Vec<LossRange>,
}

fn metric(id: &str, value: Option<f64>, unit: &str, evidence: &Evidence) -> MetricResult {
    MetricResult {
        metric_id: id.into(),
        value,
        unit: unit.into(),
        algorithm_version: ALGORITHM_VERSION.into(),
        first_sequence: evidence.first_sequence,
        last_sequence: evidence.last_sequence,
        quality: evidence.quality,
        loss_ranges: evidence.loss_ranges.clone(),
    }
}

fn collect_loss_ranges(events: &[&EncounterEvent]) -> Result<Vec<LossRange>, EncounterError> {
    events
        .iter()
        .filter(|event| event.kind == "discontinuity")
        .map(|event| {
            Ok(LossRange {
                missing_sequence_from: nonnegative(event, "missing_sequence_from")?,
                missing_sequence_to: nonnegative(event, "missing_sequence_to")?,
                reason: string(event, "reason")
                    .ok_or_else(|| EncounterError::Validation("loss reason is missing".into()))?
                    .to_string(),
            })
        })
        .collect()
}

fn collect_catalog_reference(
    event: &EncounterEvent,
    abilities: &mut BTreeSet<i64>,
    effects: &mut BTreeSet<i64>,
) {
    let Some(id) = positive_id(event, "ability_id") else {
        return;
    };
    if event.kind == "effect" {
        effects.insert(id);
    } else if matches!(
        event.kind.as_str(),
        "damage" | "healing" | "cast" | "quickslot" | "death" | "resurrection"
    ) {
        abilities.insert(id);
    }
}

fn collect_effect_interval(
    event: &EncounterEvent,
    duration_ms: u64,
    intervals: &mut BTreeMap<i64, Vec<(u64, u64)>>,
) -> Result<(), EncounterError> {
    let Some(ability_id) = positive_id(event, "ability_id") else {
        return Ok(());
    };
    let ability_intervals = intervals.entry(ability_id).or_default();
    let Some(begin_ms) = optional_nonnegative(event, "begin_ms")? else {
        return Ok(());
    };
    let Some(end_ms) = optional_nonnegative(event, "end_ms")? else {
        return Ok(());
    };
    let effect_duration = end_ms.saturating_sub(begin_ms);
    let start = event.monotonic_ms.min(duration_ms);
    let end = start.saturating_add(effect_duration).min(duration_ms);
    if end > start {
        ability_intervals.push((start, end));
    }
    Ok(())
}

fn union_length(mut intervals: Vec<(u64, u64)>) -> Result<u64, EncounterError> {
    intervals.sort_unstable();
    let mut total = 0_u64;
    let mut active: Option<(u64, u64)> = None;
    for (start, end) in intervals {
        match active {
            Some((current_start, current_end)) if start <= current_end => {
                active = Some((current_start, current_end.max(end)));
            }
            Some((current_start, current_end)) => {
                total = checked_add(total, current_end - current_start, "effect uptime")?;
                active = Some((start, end));
            }
            None => active = Some((start, end)),
        }
    }
    if let Some((start, end)) = active {
        total = checked_add(total, end - start, "effect uptime")?;
    }
    Ok(total)
}

fn join_catalog(
    catalog: &CatalogAccess,
    release: &crate::catalog::CatalogRelease,
    raw_content_sha256: String,
    abilities: BTreeSet<i64>,
    effects: BTreeSet<i64>,
) -> Result<CatalogJoinReceipt, EncounterError> {
    let all = abilities.union(&effects).copied().collect::<BTreeSet<_>>();
    let mut known = BTreeSet::new();
    for id in &all {
        let ability = catalog
            .entity(EntityKind::Ability, *id)
            .map_err(catalog_lookup_error)?
            .is_some();
        let effect = effects.contains(id)
            && catalog
                .entity(EntityKind::Effect, *id)
                .map_err(catalog_lookup_error)?
                .is_some();
        if ability || effect {
            known.insert(*id);
        }
    }
    Ok(CatalogJoinReceipt {
        catalog_schema_version: release.schema_version,
        catalog_version: release.catalog_version.clone(),
        catalog_semantic_sha256: release.semantic_sha256.clone(),
        channel: release.channel,
        api_version: release.api_version,
        raw_content_sha256,
        known_ids: known.iter().copied().collect(),
        unknown_ids: all.difference(&known).copied().collect(),
    })
}

fn catalog_lookup_error(error: crate::catalog::CatalogError) -> EncounterError {
    EncounterError::Validation(format!("catalog lookup failed: {error}"))
}

fn integer(event: &EncounterEvent, name: &str) -> Option<i64> {
    match event.payload.get(name) {
        Some(PayloadValue::Integer(value)) => Some(*value),
        _ => None,
    }
}

fn string<'a>(event: &'a EncounterEvent, name: &str) -> Option<&'a str> {
    match event.payload.get(name) {
        Some(PayloadValue::String(value)) => Some(value),
        _ => None,
    }
}

fn positive_id(event: &EncounterEvent, name: &str) -> Option<i64> {
    integer(event, name).filter(|value| *value > 0)
}

fn nonnegative(event: &EncounterEvent, name: &str) -> Result<u64, EncounterError> {
    optional_nonnegative(event, name)?
        .ok_or_else(|| EncounterError::Validation(format!("{} is missing {name}", event.kind)))
}

fn optional_nonnegative(event: &EncounterEvent, name: &str) -> Result<Option<u64>, EncounterError> {
    match integer(event, name) {
        Some(value) => u64::try_from(value)
            .map(Some)
            .map_err(|_| EncounterError::Validation(format!("{} has negative {name}", event.kind))),
        None => Ok(None),
    }
}

fn checked_add(left: u64, right: u64, label: &str) -> Result<u64, EncounterError> {
    left.checked_add(right)
        .ok_or_else(|| EncounterError::Validation(format!("{label} overflow")))
}

fn reject_link_like_output(path: &Path) -> Result<(), EncounterError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if !metadata.is_file() || is_link_like(&metadata) => {
            invalid("encounter projection output must be a regular non-link file")
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn is_link_like(metadata: &fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn publish(path: &Path, bytes: &[u8]) -> Result<(), EncounterError> {
    let parent = path
        .parent()
        .filter(|value| !value.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    let mut temporary = tempfile::Builder::new()
        .prefix(".encounter-projection-")
        .tempfile_in(parent)?;
    temporary.write_all(bytes)?;
    temporary.as_file_mut().sync_all()?;
    crate::atomic_file::persist(temporary.into_temp_path(), path)?;
    Ok(())
}
