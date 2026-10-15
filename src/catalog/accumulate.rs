//! Accumulation of bounded observations without treating absence as deletion.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use rusqlite::Connection;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::model::*;
use super::schema::{open_read_only, verify_connection};
use super::CatalogError;

/// Reconstructs normalized input only after validating the immutable artifact.
pub fn read_bundle(path: &Path) -> Result<CatalogBundle, CatalogError> {
    let connection = open_read_only(path)?;
    verify_connection(&connection)?;
    let release = rows(&connection, "SELECT json_object('catalog_version',catalog_version,'channel',channel,'game_version',game_version,'api_version',api_version,'created_at',created_at,'locales',json(locales_json),'tool_version',tool_version) FROM catalog_release")?
        .pop().ok_or_else(|| CatalogError::Validation("catalog has no release".into()))?;
    let source_snapshots = rows(&connection, "SELECT json_object('snapshot_id',snapshot_id,'family',family,'channel',channel,'game_version',game_version,'api_version',api_version,'locale',locale,'revision',revision,'raw_sha256',raw_sha256,'uri',uri,'acquired_at',acquired_at,'license_scope',license_scope,'acquisition_method',acquisition_method,'redistribution',redistribution) FROM source_snapshot")?;
    let source_records = rows(&connection, "SELECT json_object('record_id',record_id,'snapshot_id',snapshot_id,'category',category,'source_key',source_key,'content_sha256',content_sha256,'acquisition_method',acquisition_method,'import_result',import_result) FROM source_record")?;
    let coverage = rows(&connection, "SELECT json_object('category',category,'snapshot_id',snapshot_id,'locale',locale,'scope',scope,'completeness',completeness,'limits',limits_text) FROM coverage")?;
    let entities = rows(&connection, "SELECT json_object('entity',json_object('kind',kind,'stable_id',stable_id),'observed_only',json(CASE observed_only WHEN 1 THEN 'true' ELSE 'false' END),'source_records',json((SELECT json_group_array(record_id) FROM entity_source s WHERE s.kind=e.kind AND s.stable_id=e.stable_id AND s.channel=e.channel AND s.api_version=e.api_version))) FROM entity e")?;
    let attributes = rows(&connection, "SELECT json_object('entity',json_object('kind',kind,'stable_id',stable_id),'name',name,'value',json(CASE value_type WHEN 'integer' THEN CAST(integer_value AS TEXT) WHEN 'boolean' THEN CASE integer_value WHEN 1 THEN 'true' ELSE 'false' END WHEN 'real' THEN CAST(real_value AS TEXT) WHEN 'json' THEN text_value ELSE json_quote(text_value) END),'source_record',source_record_id) FROM entity_attribute")?;
    let relations = rows(&connection, "SELECT json_object('kind',relation_kind,'from',json_object('kind',from_kind,'stable_id',from_stable_id),'to',json_object('kind',to_kind,'stable_id',to_stable_id),'source_record',source_record_id) FROM entity_relation")?;
    let aliases = rows(&connection, "SELECT json_object('kind',alias_kind,'from',json_object('kind',from_kind,'stable_id',from_stable_id),'to',json_object('kind',to_kind,'stable_id',to_stable_id),'source_record',source_record_id) FROM entity_alias")?;
    let localized_text = rows(&connection, "SELECT json_object('entity',json_object('kind',kind,'stable_id',stable_id),'locale',locale,'text_kind',text_kind,'value',value,'normalized_search',normalized_search,'source_version',source_version,'redistribution',redistribution,'source_record',source_record_id) FROM localized_text")?;
    let icon_references = rows(&connection, "SELECT json_object('entity',json_object('kind',kind,'stable_id',stable_id),'virtual_path',virtual_path,'availability',availability,'source_record',source_record_id) FROM icon_reference")?;
    let icon_assets = rows(&connection, "SELECT json_object('content_sha256',content_sha256,'transformation_id',transformation_id,'media_type',media_type,'width',width,'height',height,'origin',origin,'attribution',attribution,'availability',availability,'redistribution',redistribution) FROM icon_asset")?;
    CatalogBundle {
        schema_version: INPUT_SCHEMA_VERSION,
        release,
        source_snapshots,
        source_records,
        coverage,
        entities,
        attributes,
        relations,
        aliases,
        localized_text,
        icon_references,
        icon_assets,
    }
    .normalize_and_validate()
}

fn rows<T: DeserializeOwned>(connection: &Connection, sql: &str) -> Result<Vec<T>, CatalogError> {
    let mut statement = connection.prepare(sql)?;
    let mut cursor = statement.query([])?;
    let mut output = Vec::new();
    while let Some(row) = cursor.next()? {
        output.push(serde_json::from_str(&row.get::<_, String>(0)?)?);
    }
    Ok(output)
}

pub fn same_domain(left: &CatalogBundle, right: &CatalogBundle) -> bool {
    left.release.channel == right.release.channel
        && left.release.api_version == right.release.api_version
        && left.release.game_version == right.release.game_version
}

/// Facts have stable keys; source acquisition identifiers are evidence, not keys.
/// Conflict resolution follows observed time then raw digest and is order independent.
pub fn accumulate(
    previous: CatalogBundle,
    incoming: CatalogBundle,
) -> Result<CatalogBundle, CatalogError> {
    if !same_domain(&previous, &incoming) {
        return Err(CatalogError::Validation(
            "cannot accumulate different catalog version domains".into(),
        ));
    }
    let observations: Vec<_> = previous
        .entities
        .iter()
        .chain(&incoming.entities)
        .cloned()
        .collect();
    let mut result = incoming.clone();
    result
        .release
        .locales
        .extend(previous.release.locales.iter().cloned());
    result.release.locales.sort();
    result.release.locales.dedup();
    result.source_snapshots = merge(
        previous.source_snapshots,
        incoming.source_snapshots,
        |v| v.snapshot_id.clone(),
        |_, _| true,
    );
    let snapshots: BTreeMap<_, _> = result
        .source_snapshots
        .iter()
        .map(|v| (v.snapshot_id.clone(), v))
        .collect();
    result.source_records = merge(
        previous.source_records,
        incoming.source_records,
        |v| v.record_id.clone(),
        |_, _| false,
    );
    let evidence: BTreeMap<_, _> = result
        .source_records
        .iter()
        .map(|record| {
            let snapshot = snapshots[&record.snapshot_id];
            (record.record_id.clone(), observation_order(snapshot))
        })
        .collect();
    let newer = |old: &str, new: &str| evidence.get(new) > evidence.get(old);
    result.entities = merge(
        previous.entities,
        incoming.entities,
        |v| v.entity.clone(),
        |a, b| a.observed_only && !b.observed_only,
    );
    // Keep one observation per source-key/locale, plus all evidence still used by facts.
    let records_by_id: BTreeMap<_, _> = result
        .source_records
        .iter()
        .map(|v| (v.record_id.as_str(), v))
        .collect();
    let mut entity_evidence = BTreeMap::<EntityRef, BTreeMap<(String, String), String>>::new();
    for entity in &observations {
        for record_id in &entity.source_records {
            let Some(record) = records_by_id.get(record_id.as_str()) else {
                continue;
            };
            let snapshot = snapshots[&record.snapshot_id];
            let key = (record.source_key.clone(), snapshot.locale.clone());
            let sources = entity_evidence.entry(entity.entity.clone()).or_default();
            if sources.get(&key).is_none_or(|old| newer(old, record_id)) {
                sources.insert(key, record_id.clone());
            }
        }
    }
    for entity in &mut result.entities {
        entity.source_records = entity_evidence
            .remove(&entity.entity)
            .unwrap_or_default()
            .into_values()
            .collect();
        entity.source_records.sort();
        entity.source_records.dedup();
    }
    result.attributes = merge(
        previous.attributes,
        incoming.attributes,
        |v| (v.entity.clone(), v.name.clone()),
        |a, b| newer(&a.source_record, &b.source_record),
    );
    result.relations = merge(
        previous.relations,
        incoming.relations,
        |v| (v.kind.clone(), v.from.clone(), v.to.clone()),
        |a, b| newer(&a.source_record, &b.source_record),
    );
    result.aliases = merge(
        previous.aliases,
        incoming.aliases,
        |v| (v.kind.clone(), v.from.clone(), v.to.clone()),
        |a, b| newer(&a.source_record, &b.source_record),
    );
    result.localized_text = merge(
        previous.localized_text,
        incoming.localized_text,
        |v| (v.entity.clone(), v.locale.clone(), v.text_kind.clone()),
        |a, b| newer(&a.source_record, &b.source_record),
    );
    result.icon_references = merge(
        previous.icon_references,
        incoming.icon_references,
        |v| (v.entity.clone(), v.virtual_path.clone()),
        |a, b| newer(&a.source_record, &b.source_record),
    );
    result.icon_assets = merge(
        previous.icon_assets,
        incoming.icon_assets,
        |v| (v.content_sha256.clone(), v.transformation_id.clone()),
        |_, _| false,
    );
    result.coverage = merge(
        previous.coverage,
        incoming.coverage,
        |v| (v.category.clone(), v.locale.clone(), v.scope.clone()),
        |a, b| {
            rank(b.completeness) > rank(a.completeness)
                || (b.completeness == a.completeness
                    && observation_order(snapshots[&b.snapshot_id])
                        > observation_order(snapshots[&a.snapshot_id]))
        },
    );
    let mut used: BTreeSet<String> = result
        .entities
        .iter()
        .flat_map(|v| v.source_records.iter().cloned())
        .collect();
    used.extend(result.attributes.iter().map(|v| v.source_record.clone()));
    used.extend(result.relations.iter().map(|v| v.source_record.clone()));
    used.extend(result.aliases.iter().map(|v| v.source_record.clone()));
    used.extend(
        result
            .localized_text
            .iter()
            .map(|v| v.source_record.clone()),
    );
    used.extend(
        result
            .icon_references
            .iter()
            .map(|v| v.source_record.clone()),
    );
    result
        .source_records
        .retain(|v| used.contains(&v.record_id));
    let mut snapshots_used: BTreeSet<_> = result
        .source_records
        .iter()
        .map(|v| v.snapshot_id.clone())
        .collect();
    snapshots_used.extend(result.coverage.iter().map(|v| v.snapshot_id.clone()));
    result
        .source_snapshots
        .retain(|v| snapshots_used.contains(&v.snapshot_id));
    result.normalize_and_validate()
}

fn merge<T, K: Ord>(
    old: Vec<T>,
    new: Vec<T>,
    key: impl Fn(&T) -> K,
    replace: impl Fn(&T, &T) -> bool,
) -> Vec<T> {
    let mut values = BTreeMap::new();
    for value in old.into_iter().chain(new) {
        let identity = key(&value);
        if values.get(&identity).is_none_or(|old| replace(old, &value)) {
            values.insert(identity, value);
        }
    }
    values.into_values().collect()
}

fn rank(value: Completeness) -> u8 {
    match value {
        Completeness::Unknown => 0,
        Completeness::Opportunistic => 1,
        Completeness::Bounded => 2,
        Completeness::Exhaustive => 3,
    }
}

fn observation_order(snapshot: &SourceSnapshotInput) -> (i64, String, String) {
    let raw = &snapshot.acquired_at;
    let timestamp = raw
        .parse::<i64>()
        .ok()
        .or_else(|| {
            if raw.get(10..11)? != "T" {
                return None;
            }
            let date = raw.get(..10)?;
            let clock = raw.get(11..19)?;
            let mut date = date.split('-');
            let year = date.next()?.parse().ok()?;
            let month = time::Month::try_from(date.next()?.parse::<u8>().ok()?).ok()?;
            let day = date.next()?.parse().ok()?;
            let mut clock = clock.split(':');
            let suffix = raw.get(19..)?;
            let offset = if suffix == "Z" || suffix.is_empty() {
                time::UtcOffset::UTC
            } else {
                let sign = if suffix.starts_with('+') {
                    1
                } else if suffix.starts_with('-') {
                    -1
                } else {
                    return None;
                };
                let mut parts = suffix.get(1..)?.split(':');
                let hours = parts.next()?.parse::<i8>().ok()? * sign;
                let minutes = parts.next()?.parse::<i8>().ok()? * sign;
                time::UtcOffset::from_hms(hours, minutes, 0).ok()?
            };
            Some(
                time::Date::from_calendar_date(year, month, day)
                    .ok()?
                    .with_hms(
                        clock.next()?.parse().ok()?,
                        clock.next()?.parse().ok()?,
                        clock.next()?.parse().ok()?,
                    )
                    .ok()?
                    .assume_offset(offset)
                    .unix_timestamp(),
            )
        })
        .unwrap_or(0);
    (timestamp, snapshot.raw_sha256.clone(), raw.clone())
}

pub fn latest_observation(bundle: &CatalogBundle) -> ((i64, String, String), usize) {
    (
        bundle
            .source_snapshots
            .iter()
            .map(observation_order)
            .max()
            .unwrap_or_default(),
        bundle.entities.len()
            + bundle.attributes.len()
            + bundle.relations.len()
            + bundle.localized_text.len(),
    )
}

/// Content identity deliberately excludes release/provenance acquisition metadata.
pub fn content_sha256(bundle: &CatalogBundle) -> Result<String, CatalogError> {
    let mut value = serde_json::to_value(bundle)?;
    value["release"] = json!({"channel":bundle.release.channel,"game_version":bundle.release.game_version,"api_version":bundle.release.api_version,"locales":bundle.release.locales});
    if let Value::Object(object) = &mut value {
        object.remove("source_snapshots");
        object.remove("source_records");
    }
    for section in [
        "entities",
        "attributes",
        "relations",
        "aliases",
        "localized_text",
        "icon_references",
        "coverage",
    ] {
        if let Some(rows) = value[section].as_array_mut() {
            for row in rows.iter_mut() {
                if let Value::Object(row) = row {
                    for name in [
                        "source_records",
                        "source_record",
                        "source_version",
                        "snapshot_id",
                    ] {
                        row.remove(name);
                    }
                }
            }
            rows.sort_by_cached_key(Value::to_string);
            rows.dedup();
        }
    }
    Ok(Sha256::digest(serde_json::to_vec(&value)?)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}
