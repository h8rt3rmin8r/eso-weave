//! Record-local v15 serialization, pinned in S131's grammar contract.
//!
//! This semantic view never replaces NativeRecord.line. Schema 1 and existing
//! normal/separate schema 2 input keep the historical compatibility lexer.
use super::*;
use std::borrow::Cow;

pub(super) struct DecodedRecord<'a> {
    fields: Vec<Cow<'a, str>>,
}

impl DecodedRecord<'_> {
    pub(super) fn values(&self) -> Vec<&str> {
        self.fields.iter().map(AsRef::as_ref).collect()
    }
}

pub(super) fn decode(line: &str, schema: u32) -> Result<DecodedRecord<'_>, EncounterError> {
    if schema != NATIVE_INCREMENTAL_SCHEMA_VERSION {
        return legacy(line);
    }
    let verbose = line.starts_with("offsetMS: ");
    if !verbose {
        // Arity identifies added inline tails without reinterpreting accepted
        // historical quoted escapes in normal/separate records.
        let old = fields(line);
        let added = old.as_ref().is_ok_and(|v| match v.get(1).copied() {
            Some("COMBAT_EVENT") => !matches!(v.len(), 20 | 29),
            Some("BEGIN_CAST") => !matches!(v.len(), 17 | 26),
            Some("END_CAST") => !matches!(v.len(), 5 | 7),
            Some("EFFECT_CHANGED") => {
                !matches!(v.len(), 17 | 18 | 26 | 27)
                    || v.get(6).is_some_and(|value| value.starts_with('"'))
            }
            _ => false,
        });
        if !added {
            if let Ok(values) = old {
                if validate_layout_for(&values, schema).is_ok() {
                    return Ok(DecodedRecord {
                        fields: values.into_iter().map(Cow::Borrowed).collect(),
                    });
                }
            }
        }
    }
    let parts = split(line, if verbose { b'\t' } else { b',' })?;
    let mut cursor = Cursor {
        parts,
        at: 0,
        verbose,
        output: Vec::new(),
    };
    cursor.scalar("offsetMS")?;
    let kind = cursor.raw("lineType")?;
    word(kind)?;
    cursor.output.push(Cow::Borrowed(kind));
    match kind {
        "BEGIN_LOG" => cursor.scalars(&[
            "timeSinceEpochMS",
            "logVersion",
            "realmName",
            "language",
            "gameVersion",
        ])?,
        "END_LOG" | "BEGIN_COMBAT" | "END_COMBAT" => {}
        "COMBAT_EVENT" => {
            cursor.scalars(&[
                "actionResult",
                "damageType",
                "powerType",
                "hitValue",
                "overflow",
                "castTrackId",
                "abilityId",
            ])?;
            if cursor.has_ability() {
                cursor.ability()?;
            }
            cursor.states()?;
        }
        "BEGIN_CAST" => {
            cursor.scalars(&["durationMS", "channeled", "castTrackId", "abilityId"])?;
            if cursor.has_ability() {
                cursor.ability()?;
            }
            cursor.states()?;
        }
        "END_CAST" => {
            let reason = cursor.scalar("endReason")?;
            cursor.scalars(&["castTrackId", "abilityId"])?;
            let inline = cursor.has_ability();
            if inline {
                cursor.ability()?;
            }
            if reason == "INTERRUPTED" {
                cursor.scalar("interruptingAbilityId")?;
                if inline {
                    cursor.ability()?;
                }
                cursor.scalar("interruptingUnitId")?;
            }
        }
        "EFFECT_CHANGED" => {
            cursor.scalars(&["changeType", "stackCount", "castTrackId", "abilityId"])?;
            if cursor.has_ability() {
                cursor.ability()?;
                cursor.effect(false)?;
            }
            cursor.states()?;
            if cursor.at < cursor.parts.len() {
                cursor.scalar("playerInitiatedRemoveCastTrackId")?;
            }
        }
        "HEALTH_REGEN" => {
            cursor.scalar("effectiveRegen")?;
            cursor.unit()?;
        }
        "UNIT_ADDED" => cursor.scalars(&[
            "unitId",
            "unitType",
            "isLocalPlayer",
            "playerPerSessionId",
            "monsterId",
            "isBoss",
            "classId",
            "raceId",
            "name",
            "displayName",
            "characterId",
            "level",
            "championPoints",
            "ownerUnitId",
            "reaction",
            "isGroupedWithLocalPlayer",
        ])?,
        "UNIT_CHANGED" => cursor.scalars(&[
            "unitId",
            "classId",
            "raceId",
            "name",
            "displayName",
            "characterId",
            "level",
            "championPoints",
            "ownerUnitId",
            "reaction",
            "isGroupedWithLocalPlayer",
        ])?,
        "UNIT_REMOVED" => {
            cursor.scalar("unitId")?;
        }
        "ABILITY_INFO" => {
            cursor.scalars(&[
                "abilityId",
                "name",
                "iconPath",
                "interruptible",
                "blockable",
            ])?;
            if cursor.at < cursor.parts.len() {
                cursor.scalars(&["primaryScript", "secondaryScript", "tertiaryScript"])?;
            }
        }
        "EFFECT_INFO" => {
            cursor.scalar("abilityId")?;
            cursor.effect(true)?;
        }
        "PLAYER_INFO" => {
            cursor.scalar("unitId")?;
            for key in [
                "longTermEffectAbilityIds",
                "longTermEffectStackCounts",
                "equipment",
                "primaryAbilities",
                "backupAbilities",
            ] {
                let value = cursor.raw(key)?;
                cursor.output.push(Cow::Owned(player_array(
                    value,
                    verbose,
                    key == "equipment",
                )?));
            }
        }
        "MAP_CHANGED" | "MAP_INFO" => cursor.scalars(&["id", "name", "texturePath"])?,
        "ZONE_CHANGED" | "ZONE_INFO" => cursor.scalars(&["id", "name", "dungeonDifficulty"])?,
        "TRIAL_INIT" => {
            cursor.scalars(&[
                "id",
                "inProgress",
                "completed",
                "startTimeMS",
                "durationMS",
                "success",
                "finalScore",
            ])?;
            for i in [3, 4, 7] {
                flag(&cursor.output[i])?;
            }
            for i in [2, 5, 6, 8] {
                uint(&cursor.output[i])?;
            }
        }
        "BEGIN_TRIAL" => {
            cursor.scalars(&["id", "startTimeMS"])?;
            for v in &cursor.output[2..] {
                uint(v)?;
            }
        }
        "END_TRIAL" => {
            cursor.scalars(&[
                "id",
                "durationMS",
                "success",
                "finalScore",
                "finalVitalityBonus",
            ])?;
            flag(&cursor.output[4])?;
            for i in [2, 3, 5, 6] {
                uint(&cursor.output[i])?;
            }
        }
        "ENDLESS_DUNGEON_INIT" | "ENDLESS_DUNGEON_BEGIN" => {
            cursor.scalars(&["id", "startTimeMS", "solo"])?
        }
        "ENDLESS_DUNGEON_END" => cursor.scalars(&["id", "durationMS", "finalScore", "solo"])?,
        "ENDLESS_DUNGEON_STAGE_END" => cursor.scalars(&["id", "durationMS"])?,
        "ENDLESS_DUNGEON_BUFF_ADDED" | "ENDLESS_DUNGEON_BUFF_REMOVED" => {
            cursor.scalars(&["id", "abilityId"])?
        }
        _ => {
            // Unknown payloads are retained, not assigned invented field names.
            cursor
                .output
                .extend(cursor.parts[cursor.at..].iter().map(|v| Cow::Borrowed(*v)));
            cursor.at = cursor.parts.len();
        }
    }
    if cursor.at != cursor.parts.len() {
        return rejected();
    }
    Ok(DecodedRecord {
        fields: cursor.output,
    })
}

fn legacy(line: &str) -> Result<DecodedRecord<'_>, EncounterError> {
    Ok(DecodedRecord {
        fields: fields(line)?.into_iter().map(Cow::Borrowed).collect(),
    })
}

/// Count nested separators before allocating semantic arrays. Backslashes have
/// no special role in the producer's literal quoted strings.
fn split(line: &str, delimiter: u8) -> Result<Vec<&str>, EncounterError> {
    if line.is_empty()
        || line.len() > MAX_NATIVE_LINE_BYTES
        || line.bytes().any(|b| b < 32 && b != b'\t')
    {
        return rejected();
    }
    let mut result = Vec::new();
    let (mut start, mut depth, mut quoted, mut tokens) = (0, 0_usize, false, 1);
    for (i, byte) in line.bytes().enumerate() {
        if byte == b'"' {
            quoted = !quoted;
        } else if !quoted {
            match byte {
                b'[' => {
                    depth += 1;
                    if depth > MAX_NATIVE_DEPTH {
                        return rejected();
                    }
                }
                b']' => {
                    depth = depth.checked_sub(1).ok_or_else(|| {
                        EncounterError::Validation("native group is unbalanced".into())
                    })?;
                }
                _ if byte == delimiter => {
                    tokens += 1;
                    if tokens > MAX_NATIVE_TOKENS {
                        return rejected();
                    }
                    if depth == 0 {
                        result.push(&line[start..i]);
                        start = i + 1;
                    }
                }
                _ => {}
            }
        }
    }
    if quoted || depth != 0 {
        return rejected();
    }
    result.push(&line[start..]);
    Ok(result)
}

fn literal(value: &str) -> Result<Cow<'_, str>, EncounterError> {
    if value.starts_with('"') {
        let inner = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .ok_or_else(|| {
                EncounterError::Validation("native quoted value is incomplete".into())
            })?;
        if inner.contains('"') {
            return rejected();
        }
        // Encode only the ephemeral semantic value for the historical shared
        // string validator. This is never persisted as source evidence.
        Ok(Cow::Owned(format!("\"{}\"", inner.replace('\\', "\\\\"))))
    } else {
        Ok(Cow::Borrowed(value))
    }
}

struct Cursor<'a> {
    parts: Vec<&'a str>,
    at: usize,
    verbose: bool,
    output: Vec<Cow<'a, str>>,
}

impl<'a> Cursor<'a> {
    fn raw(&mut self, label: &str) -> Result<&'a str, EncounterError> {
        let raw = *self
            .parts
            .get(self.at)
            .ok_or_else(|| EncounterError::Validation("native field is missing".into()))?;
        self.at += 1;
        if self.verbose {
            raw.strip_prefix(label)
                .and_then(|v| v.strip_prefix(": "))
                .ok_or_else(|| EncounterError::Validation("native field label differs".into()))
        } else {
            Ok(raw)
        }
    }
    fn scalar(&mut self, label: &str) -> Result<&'a str, EncounterError> {
        let value = self.raw(label)?;
        self.output.push(literal(value)?);
        Ok(value)
    }
    fn scalars(&mut self, labels: &[&str]) -> Result<(), EncounterError> {
        for label in labels {
            self.scalar(label)?;
        }
        Ok(())
    }
    fn has_ability(&self) -> bool {
        self.parts.get(self.at).is_some_and(|v| {
            if self.verbose {
                v.starts_with("name: ")
            } else {
                v.starts_with('"')
            }
        })
    }
    fn ability(&mut self) -> Result<(), EncounterError> {
        for label in ["name", "iconPath"] {
            quoted_value(&literal(self.raw(label)?)?)?;
        }
        flag(self.raw("interruptible")?)?;
        flag(self.raw("blockable")?)?;
        Ok(())
    }
    fn effect(&mut self, retain: bool) -> Result<(), EncounterError> {
        let mut empty = 0;
        for label in [
            "effectType",
            "statusEffectType",
            "effectBarDisplayBehaviour",
        ] {
            let value = self.raw(label)?;
            if value == "\"\"" {
                empty += 1;
            } else {
                word(value)?;
            }
            if retain {
                self.output.push(Cow::Borrowed(value));
            }
        }
        if empty != 0 && empty != 3 {
            return rejected();
        }
        let synergy = self.parts.get(self.at).is_some_and(|v| {
            if self.verbose {
                v.starts_with("grantsSyngeryAbilityId: ")
            } else if retain {
                true
            } else {
                // Without a synergy, the second unit field is a current/max pair.
                self.parts
                    .get(self.at + 1)
                    .is_some_and(|v| !v.contains('/'))
            }
        });
        if synergy {
            let value = self.raw("grantsSyngeryAbilityId")?;
            uint(value)?;
            if retain {
                self.output.push(Cow::Borrowed(value));
            }
        }
        Ok(())
    }
    fn unit(&mut self) -> Result<(), EncounterError> {
        self.scalars(&[
            "unitId", "health", "magicka", "stamina", "ultimate", "werewolf", "shield", "mapNX",
            "mapNY", "heading",
        ])
    }
    fn states(&mut self) -> Result<(), EncounterError> {
        self.unit()?;
        if self
            .parts
            .get(self.at)
            .is_some_and(|v| *v == if self.verbose { "target: *" } else { "*" })
        {
            self.scalar("target")?;
        } else {
            self.unit()?;
        }
        Ok(())
    }
}

fn player_array(value: &str, verbose: bool, equipment: bool) -> Result<String, EncounterError> {
    let inner = value
        .strip_prefix('[')
        .and_then(|v| v.strip_suffix(']'))
        .ok_or_else(|| EncounterError::Validation("native array is invalid".into()))?;
    if inner.is_empty() {
        return Ok("[]".into());
    }
    let parts = split(inner, if verbose { b'\t' } else { b',' })?;
    let mut normalized = Vec::new();
    for value in parts {
        if equipment {
            let inner = value
                .strip_prefix('[')
                .and_then(|v| v.strip_suffix(']'))
                .ok_or_else(|| {
                    EncounterError::Validation("native equipment group is invalid".into())
                })?;
            let mut item = Cursor {
                parts: split(inner, if verbose { b'\t' } else { b',' })?,
                at: 0,
                verbose,
                output: Vec::new(),
            };
            item.scalars(&[
                "slot",
                "id",
                "isCP",
                "level",
                "trait",
                "quality",
                "setId",
                "enchantType",
            ])?;
            let missing = verbose
                && item
                    .parts
                    .get(item.at)
                    .is_some_and(|v| v.starts_with("enchantCP: "));
            item.scalar(if missing { "enchantCP" } else { "isEnchantCP" })?;
            item.scalars(&["enchantLevel", "enchantQuality"])?;
            if item.at != item.parts.len() {
                return rejected();
            }
            if missing
                && (item.output[7] != "INVALID"
                    || item.output[8] != "F"
                    || item.output[9] != "0"
                    || item.output[10] != "NORMAL")
            {
                return rejected();
            }
            normalized.push(format!(
                "[{}]",
                item.output
                    .iter()
                    .map(AsRef::as_ref)
                    .collect::<Vec<&str>>()
                    .join(",")
            ));
        } else {
            uint(value)?;
            normalized.push(value.to_string());
        }
    }
    Ok(format!("[{}]", normalized.join(",")))
}
