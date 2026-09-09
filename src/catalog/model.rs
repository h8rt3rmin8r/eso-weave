//! Strict normalized catalog input and validation.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::CatalogError;

pub const INPUT_SCHEMA_VERSION: u32 = 1;
pub const MAX_INPUT_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_RECORDS: usize = 500_000;
pub const MAX_STRING_BYTES: usize = 64 * 1024;

pub const CATEGORIES: [&str; 15] = [
    "player-skills",
    "crafted-abilities",
    "ability-metadata",
    "effects-and-status",
    "items-and-gear",
    "item-sets",
    "champion-skills",
    "consumables",
    "mundus-effects",
    "companions-races-classes",
    "combat-statistics",
    "constants",
    "localized-text",
    "icon-references",
    "icon-bytes",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Channel {
    Live,
    Pts,
}

impl Channel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Pts => "pts",
        }
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceChannel {
    Live,
    Pts,
    NotApplicable,
}

impl SourceChannel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Pts => "pts",
            Self::NotApplicable => "not-applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Completeness {
    Exhaustive,
    Bounded,
    Opportunistic,
    Unknown,
}

impl Completeness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exhaustive => "exhaustive",
            Self::Bounded => "bounded",
            Self::Opportunistic => "opportunistic",
            Self::Unknown => "unknown",
        }
    }

    fn rank(self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::Opportunistic => 1,
            Self::Bounded => 2,
            Self::Exhaustive => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Redistribution {
    Allowed,
    AttributionRequired,
    UserGeneratedOnly,
    Prohibited,
    Unresolved,
}

impl Redistribution {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::AttributionRequired => "attribution-required",
            Self::UserGeneratedOnly => "user-generated-only",
            Self::Prohibited => "prohibited",
            Self::Unresolved => "unresolved",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntityKind {
    SkillType,
    SkillLine,
    Ability,
    AbilityProgression,
    AbilityRank,
    AbilityMorph,
    CraftedAbility,
    Script,
    Effect,
    Item,
    ItemVariant,
    EquipmentType,
    ArmorType,
    WeaponType,
    Quality,
    Trait,
    Enchantment,
    ItemSet,
    ItemSetPiece,
    ItemSetBonus,
    CombatStat,
    ChampionSkill,
    Mundus,
    FoodDrink,
    Potion,
    Poison,
    Class,
    Race,
    Companion,
    CollectionCategory,
    Constant,
}

impl EntityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SkillType => "skill-type",
            Self::SkillLine => "skill-line",
            Self::Ability => "ability",
            Self::AbilityProgression => "ability-progression",
            Self::AbilityRank => "ability-rank",
            Self::AbilityMorph => "ability-morph",
            Self::CraftedAbility => "crafted-ability",
            Self::Script => "script",
            Self::Effect => "effect",
            Self::Item => "item",
            Self::ItemVariant => "item-variant",
            Self::EquipmentType => "equipment-type",
            Self::ArmorType => "armor-type",
            Self::WeaponType => "weapon-type",
            Self::Quality => "quality",
            Self::Trait => "trait",
            Self::Enchantment => "enchantment",
            Self::ItemSet => "item-set",
            Self::ItemSetPiece => "item-set-piece",
            Self::ItemSetBonus => "item-set-bonus",
            Self::CombatStat => "combat-stat",
            Self::ChampionSkill => "champion-skill",
            Self::Mundus => "mundus",
            Self::FoodDrink => "food-drink",
            Self::Potion => "potion",
            Self::Poison => "poison",
            Self::Class => "class",
            Self::Race => "race",
            Self::Companion => "companion",
            Self::CollectionCategory => "collection-category",
            Self::Constant => "constant",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "skill-type" => Self::SkillType,
            "skill-line" => Self::SkillLine,
            "ability" => Self::Ability,
            "ability-progression" => Self::AbilityProgression,
            "ability-rank" => Self::AbilityRank,
            "ability-morph" => Self::AbilityMorph,
            "crafted-ability" => Self::CraftedAbility,
            "script" => Self::Script,
            "effect" => Self::Effect,
            "item" => Self::Item,
            "item-variant" => Self::ItemVariant,
            "equipment-type" => Self::EquipmentType,
            "armor-type" => Self::ArmorType,
            "weapon-type" => Self::WeaponType,
            "quality" => Self::Quality,
            "trait" => Self::Trait,
            "enchantment" => Self::Enchantment,
            "item-set" => Self::ItemSet,
            "item-set-piece" => Self::ItemSetPiece,
            "item-set-bonus" => Self::ItemSetBonus,
            "combat-stat" => Self::CombatStat,
            "champion-skill" => Self::ChampionSkill,
            "mundus" => Self::Mundus,
            "food-drink" => Self::FoodDrink,
            "potion" => Self::Potion,
            "poison" => Self::Poison,
            "class" => Self::Class,
            "race" => Self::Race,
            "companion" => Self::Companion,
            "collection-category" => Self::CollectionCategory,
            "constant" => Self::Constant,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityRef {
    pub kind: EntityKind,
    pub stable_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogBundle {
    pub schema_version: u32,
    pub release: ReleaseInput,
    pub source_snapshots: Vec<SourceSnapshotInput>,
    pub source_records: Vec<SourceRecordInput>,
    pub coverage: Vec<CoverageInput>,
    pub entities: Vec<EntityInput>,
    pub attributes: Vec<AttributeInput>,
    pub relations: Vec<RelationInput>,
    pub aliases: Vec<AliasInput>,
    pub localized_text: Vec<LocalizedTextInput>,
    pub icon_references: Vec<IconReferenceInput>,
    pub icon_assets: Vec<IconAssetInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseInput {
    pub catalog_version: String,
    pub channel: Channel,
    pub game_version: String,
    pub api_version: u32,
    pub created_at: String,
    pub locales: Vec<String>,
    pub tool_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSnapshotInput {
    pub snapshot_id: String,
    pub family: String,
    pub channel: SourceChannel,
    pub game_version: String,
    pub api_version: u32,
    pub locale: String,
    pub revision: String,
    pub raw_sha256: String,
    pub uri: String,
    pub acquired_at: String,
    pub license_scope: String,
    pub acquisition_method: String,
    pub redistribution: Redistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRecordInput {
    pub record_id: String,
    pub snapshot_id: String,
    pub category: String,
    pub source_key: String,
    pub content_sha256: String,
    pub acquisition_method: String,
    pub import_result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageInput {
    pub category: String,
    pub snapshot_id: String,
    pub locale: String,
    pub scope: String,
    pub completeness: Completeness,
    pub limits: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityInput {
    pub entity: EntityRef,
    pub observed_only: bool,
    pub source_records: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttributeInput {
    pub entity: EntityRef,
    pub name: String,
    pub value: serde_json::Value,
    pub source_record: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationInput {
    pub kind: String,
    pub from: EntityRef,
    pub to: EntityRef,
    pub source_record: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AliasInput {
    pub kind: String,
    pub from: EntityRef,
    pub to: EntityRef,
    pub source_record: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalizedTextInput {
    pub entity: EntityRef,
    pub locale: String,
    pub text_kind: String,
    pub value: String,
    pub normalized_search: Option<String>,
    pub source_version: String,
    pub redistribution: Redistribution,
    pub source_record: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IconReferenceInput {
    pub entity: EntityRef,
    pub virtual_path: String,
    pub availability: String,
    pub source_record: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IconAssetInput {
    pub content_sha256: String,
    pub transformation_id: String,
    pub media_type: String,
    pub width: u32,
    pub height: u32,
    pub origin: String,
    pub attribution: String,
    pub availability: String,
    pub redistribution: Redistribution,
}

impl CatalogBundle {
    pub fn normalize_and_validate(mut self) -> Result<Self, CatalogError> {
        if self.schema_version != INPUT_SCHEMA_VERSION {
            return validation(format!(
                "unsupported input schema {}, expected {INPUT_SCHEMA_VERSION}",
                self.schema_version
            ));
        }
        nonempty("release.catalog_version", &self.release.catalog_version)?;
        nonempty("release.game_version", &self.release.game_version)?;
        nonempty("release.created_at", &self.release.created_at)?;
        nonempty("release.tool_version", &self.release.tool_version)?;
        if self.release.api_version == 0 || self.release.locales.is_empty() {
            return validation("release requires a positive API version and at least one locale");
        }
        validate_strings(&self)?;
        sort_unique_strings("release locales", &mut self.release.locales)?;

        if self.source_snapshots.is_empty() {
            return validation("at least one source snapshot is required");
        }
        self.source_snapshots
            .sort_by(|left, right| left.snapshot_id.cmp(&right.snapshot_id));
        reject_duplicate_by(
            "source snapshot",
            self.source_snapshots
                .iter()
                .map(|value| value.snapshot_id.as_str()),
        )?;
        let snapshots: BTreeSet<_> = self
            .source_snapshots
            .iter()
            .map(|value| value.snapshot_id.as_str())
            .collect();
        for snapshot in &self.source_snapshots {
            nonempty("source snapshot id", &snapshot.snapshot_id)?;
            validate_sha256("source snapshot raw hash", &snapshot.raw_sha256)?;
            match snapshot.channel {
                SourceChannel::Live if self.release.channel != Channel::Live => {
                    return validation("live source snapshot cannot enter a PTS catalog")
                }
                SourceChannel::Pts if self.release.channel != Channel::Pts => {
                    return validation("PTS source snapshot cannot enter a live catalog")
                }
                _ => {}
            }
            if snapshot.channel != SourceChannel::NotApplicable
                && (snapshot.api_version != self.release.api_version
                    || snapshot.game_version != self.release.game_version)
            {
                return validation(format!(
                    "source snapshot {} version does not match the release",
                    snapshot.snapshot_id
                ));
            }
        }

        self.source_records
            .sort_by(|left, right| left.record_id.cmp(&right.record_id));
        reject_duplicate_by(
            "source record",
            self.source_records
                .iter()
                .map(|value| value.record_id.as_str()),
        )?;
        let records: BTreeSet<_> = self
            .source_records
            .iter()
            .map(|value| value.record_id.as_str())
            .collect();
        for record in &self.source_records {
            if !snapshots.contains(record.snapshot_id.as_str()) {
                return validation(format!(
                    "source record {} references missing snapshot {}",
                    record.record_id, record.snapshot_id
                ));
            }
            require_category(&record.category)?;
            validate_sha256("source record content hash", &record.content_sha256)?;
            if !matches!(record.import_result.as_str(), "accepted" | "observed-only") {
                return validation(format!(
                    "source record {} has invalid import result",
                    record.record_id
                ));
            }
        }
        reject_duplicate_by(
            "source record key",
            self.source_records.iter().map(|value| {
                (
                    value.snapshot_id.as_str(),
                    value.category.as_str(),
                    value.source_key.as_str(),
                )
            }),
        )?;

        self.coverage.sort_by(|left, right| {
            (&left.category, &left.snapshot_id, &left.locale, &left.scope).cmp(&(
                &right.category,
                &right.snapshot_id,
                &right.locale,
                &right.scope,
            ))
        });
        for coverage in &self.coverage {
            require_category(&coverage.category)?;
            if !snapshots.contains(coverage.snapshot_id.as_str()) {
                return validation(format!(
                    "coverage {} references missing snapshot {}",
                    coverage.category, coverage.snapshot_id
                ));
            }
            if coverage.completeness != Completeness::Exhaustive && coverage.limits.is_empty() {
                return validation(format!(
                    "coverage {} must declare its limits",
                    coverage.category
                ));
            }
            if coverage.completeness.rank() > maximum_completeness(&coverage.category).rank() {
                return validation(format!(
                    "coverage {} claims {} beyond the approved source contract",
                    coverage.category,
                    coverage.completeness.as_str()
                ));
            }
        }
        reject_duplicate_by(
            "coverage",
            self.coverage.iter().map(|value| {
                (
                    value.category.as_str(),
                    value.snapshot_id.as_str(),
                    value.locale.as_str(),
                    value.scope.as_str(),
                )
            }),
        )?;
        let covered: BTreeSet<String> = self
            .coverage
            .iter()
            .map(|value| value.category.clone())
            .collect();
        let default_snapshot = self.source_snapshots[0].snapshot_id.clone();
        for category in CATEGORIES {
            if !covered.contains(category) {
                self.coverage.push(CoverageInput {
                    category: category.to_string(),
                    snapshot_id: default_snapshot.clone(),
                    locale: "all".to_string(),
                    scope: "not-provided".to_string(),
                    completeness: Completeness::Unknown,
                    limits: "category absent from normalized input".to_string(),
                });
            }
        }
        self.coverage.sort_by(|left, right| {
            (&left.category, &left.snapshot_id, &left.locale, &left.scope).cmp(&(
                &right.category,
                &right.snapshot_id,
                &right.locale,
                &right.scope,
            ))
        });

        self.entities.sort_by_key(|value| value.entity.clone());
        reject_duplicate_by("entity", self.entities.iter().map(|value| &value.entity))?;
        let entities: BTreeSet<_> = self
            .entities
            .iter()
            .map(|value| value.entity.clone())
            .collect();
        for entity in &mut self.entities {
            if entity.entity.stable_id <= 0 || entity.source_records.is_empty() {
                return validation("entity requires a positive stable ID and provenance");
            }
            sort_unique_strings("entity source records", &mut entity.source_records)?;
            for source in &entity.source_records {
                require_record(source, &records)?;
            }
        }

        self.attributes
            .sort_by(|left, right| (&left.entity, &left.name).cmp(&(&right.entity, &right.name)));
        reject_duplicate_by(
            "attribute",
            self.attributes
                .iter()
                .map(|value| (&value.entity, value.name.as_str())),
        )?;
        for attribute in &self.attributes {
            require_entity(&attribute.entity, &entities, "attribute")?;
            require_record(&attribute.source_record, &records)?;
            nonempty("attribute name", &attribute.name)?;
            if matches!(
                attribute.name.as_str(),
                "index" | "array-index" | "lua-index"
            ) {
                return validation(
                    "iterator positions must use the explicit version-scoped-order attribute",
                );
            }
            if attribute.value.is_null() {
                return validation("attribute values cannot be null");
            }
        }

        self.relations.sort_by(|left, right| {
            (&left.kind, &left.from, &left.to).cmp(&(&right.kind, &right.from, &right.to))
        });
        reject_duplicate_by(
            "relationship",
            self.relations
                .iter()
                .map(|value| (value.kind.as_str(), &value.from, &value.to)),
        )?;
        for relation in &self.relations {
            nonempty("relationship kind", &relation.kind)?;
            require_entity(&relation.from, &entities, "relationship source")?;
            require_entity(&relation.to, &entities, "relationship target")?;
            require_record(&relation.source_record, &records)?;
        }

        self.aliases.sort_by(|left, right| {
            (&left.kind, &left.from, &left.to).cmp(&(&right.kind, &right.from, &right.to))
        });
        reject_duplicate_by(
            "alias",
            self.aliases
                .iter()
                .map(|value| (value.kind.as_str(), &value.from, &value.to)),
        )?;
        for alias in &self.aliases {
            if !matches!(
                alias.kind.as_str(),
                "renamed" | "morph" | "perfected" | "retired" | "superseded"
            ) {
                return validation(format!("unsupported alias kind {}", alias.kind));
            }
            require_entity(&alias.from, &entities, "alias source")?;
            require_entity(&alias.to, &entities, "alias target")?;
            require_record(&alias.source_record, &records)?;
        }

        self.localized_text.sort_by(|left, right| {
            (&left.entity, &left.locale, &left.text_kind).cmp(&(
                &right.entity,
                &right.locale,
                &right.text_kind,
            ))
        });
        reject_duplicate_by(
            "localized text",
            self.localized_text.iter().map(|value| {
                (
                    &value.entity,
                    value.locale.as_str(),
                    value.text_kind.as_str(),
                )
            }),
        )?;
        for text in &self.localized_text {
            require_entity(&text.entity, &entities, "localized text")?;
            require_record(&text.source_record, &records)?;
            if !self.release.locales.contains(&text.locale) {
                return validation(format!(
                    "localized text locale {} is absent from release locales",
                    text.locale
                ));
            }
        }

        self.icon_references.sort_by(|left, right| {
            (&left.entity, &left.virtual_path).cmp(&(&right.entity, &right.virtual_path))
        });
        reject_duplicate_by(
            "icon reference",
            self.icon_references
                .iter()
                .map(|value| (&value.entity, value.virtual_path.as_str())),
        )?;
        for icon in &self.icon_references {
            require_entity(&icon.entity, &entities, "icon reference")?;
            require_record(&icon.source_record, &records)?;
            if !matches!(
                icon.availability.as_str(),
                "reference-only" | "local" | "missing" | "placeholder"
            ) {
                return validation("invalid icon reference availability");
            }
        }

        self.icon_assets.sort_by(|left, right| {
            (&left.content_sha256, &left.transformation_id)
                .cmp(&(&right.content_sha256, &right.transformation_id))
        });
        reject_duplicate_by(
            "icon asset",
            self.icon_assets.iter().map(|value| {
                (
                    value.content_sha256.as_str(),
                    value.transformation_id.as_str(),
                )
            }),
        )?;
        for asset in &self.icon_assets {
            validate_sha256("icon asset content hash", &asset.content_sha256)?;
            if asset.width == 0 || asset.height == 0 {
                return validation("icon asset dimensions must be positive");
            }
        }

        let total = self.source_snapshots.len()
            + self.source_records.len()
            + self.coverage.len()
            + self.entities.len()
            + self.attributes.len()
            + self.relations.len()
            + self.aliases.len()
            + self.localized_text.len()
            + self.icon_references.len()
            + self.icon_assets.len();
        if total > MAX_RECORDS {
            return validation(format!(
                "catalog has {total} records, limit is {MAX_RECORDS}"
            ));
        }
        Ok(self)
    }
}

fn validate_strings(bundle: &CatalogBundle) -> Result<(), CatalogError> {
    let value = serde_json::to_value(bundle)?;
    let mut pending = vec![value];
    while let Some(value) = pending.pop() {
        match value {
            serde_json::Value::String(value) if value.len() > MAX_STRING_BYTES => {
                return validation(format!(
                    "string has {} bytes, limit is {MAX_STRING_BYTES}",
                    value.len()
                ));
            }
            serde_json::Value::Array(values) => pending.extend(values),
            serde_json::Value::Object(values) => pending.extend(values.into_values()),
            _ => {}
        }
    }
    Ok(())
}

fn maximum_completeness(category: &str) -> Completeness {
    match category {
        "constants" => Completeness::Exhaustive,
        "player-skills"
        | "crafted-abilities"
        | "item-sets"
        | "champion-skills"
        | "companions-races-classes"
        | "combat-statistics"
        | "localized-text" => Completeness::Bounded,
        "ability-metadata" | "effects-and-status" | "items-and-gear" | "consumables"
        | "mundus-effects" | "icon-references" => Completeness::Opportunistic,
        _ => Completeness::Unknown,
    }
}

fn require_category(category: &str) -> Result<(), CatalogError> {
    if CATEGORIES.contains(&category) {
        Ok(())
    } else {
        validation(format!("unknown catalog category {category}"))
    }
}

fn require_entity(
    entity: &EntityRef,
    entities: &BTreeSet<EntityRef>,
    label: &str,
) -> Result<(), CatalogError> {
    if entities.contains(entity) {
        Ok(())
    } else {
        validation(format!(
            "{label} relationship references missing {} {}",
            entity.kind.as_str(),
            entity.stable_id
        ))
    }
}

fn require_record(record: &str, records: &BTreeSet<&str>) -> Result<(), CatalogError> {
    if records.contains(record) {
        Ok(())
    } else {
        validation(format!("missing source record {record}"))
    }
}

fn nonempty(label: &str, value: &str) -> Result<(), CatalogError> {
    if value.is_empty() {
        validation(format!("{label} cannot be empty"))
    } else {
        Ok(())
    }
}

fn validate_sha256(label: &str, value: &str) -> Result<(), CatalogError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        validation(format!("{label} must be a lowercase SHA-256"))
    }
}

fn sort_unique_strings(label: &str, values: &mut [String]) -> Result<(), CatalogError> {
    values.sort();
    for pair in values.windows(2) {
        if pair[0] == pair[1] {
            return validation(format!("duplicate {label}: {}", pair[0]));
        }
    }
    Ok(())
}

fn reject_duplicate_by<T: Ord>(
    label: &str,
    values: impl IntoIterator<Item = T>,
) -> Result<(), CatalogError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return validation(format!("duplicate {label}"));
        }
    }
    Ok(())
}

fn validation<T>(message: impl Into<String>) -> Result<T, CatalogError> {
    Err(CatalogError::Validation(message.into()))
}

pub fn canonical_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let sorted: BTreeMap<_, _> = map.iter().collect();
            let body = sorted
                .into_iter()
                .map(|(key, value)| {
                    format!(
                        "{}:{}",
                        serde_json::to_string(key).expect("JSON key"),
                        canonical_json(value)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            format!("{{{body}}}")
        }
        serde_json::Value::Array(values) => {
            let body = values
                .iter()
                .map(canonical_json)
                .collect::<Vec<_>>()
                .join(",");
            format!("[{body}]")
        }
        _ => serde_json::to_string(value).expect("serializable JSON value"),
    }
}
