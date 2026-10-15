//! Source-specific completed native log evidence.
mod incremental;

use crate::catalog::Channel;
pub use incremental::{
    IncrementalNativeParser, NativeCheckpoint, NativeCompletedFight, NativeDiagnostic,
    NativeDiagnosticCode, NativeFeed, NativeParserError, NativeParserStatus, NativePosition,
    NativeTermination, NativeTerminationReason, MAX_NATIVE_CHECKPOINT_BYTES, MAX_NATIVE_FEED_BYTES,
    NATIVE_CHECKPOINT_VERSION, NATIVE_INCREMENTAL_SCHEMA_VERSION,
};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{invalid, EncounterError};

pub const NATIVE_SCHEMA_VERSION: u32 = 1;
pub const MAX_NATIVE_FILE_BYTES: u64 = 256 * 1024 * 1024;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeRecord {
    pub ordinal: u64,
    pub byte_offset: u64,
    pub line: String,
}
impl fmt::Debug for NativeRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeRecord")
            .field("ordinal", &self.ordinal)
            .field("bytes", &self.line.len())
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeHeader {
    pub epoch_ms: u64,
    pub log_version: u32,
    pub realm: String,
    pub language: String,
    pub game_version: String,
    pub record: NativeRecord,
}
impl fmt::Debug for NativeHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeHeader")
            .field("log_version", &self.log_version)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeRun {
    pub schema_version: u32,
    pub session_id: String,
    pub run_ordinal: u64,
    pub channel: Channel,
    pub header: NativeHeader,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeEncounter {
    pub schema_version: u32,
    pub run: NativeRun,
    pub encounter_id: String,
    pub encounter_ordinal: u64,
    pub started_ms: u64,
    pub finished_ms: u64,
    pub first_ordinal: u64,
    pub last_ordinal: u64,
    pub context: Vec<NativeRecord>,
    pub records: Vec<NativeRecord>,
    pub unknown_record_count: usize,
    pub unknown_context_count: usize,
    pub clock_reversal_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct NativeSourceReport {
    pub record_count: usize,
    pub run_count: usize,
    pub completed_encounter_count: usize,
    pub unknown_record_count: usize,
    pub outside_combat_record_count: usize,
    pub incomplete_combat_count: usize,
    pub incomplete_run_count: usize,
    pub unterminated_line_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNativeLog {
    pub runs: Vec<NativeRun>,
    pub encounters: Vec<NativeEncounter>,
    pub report: NativeSourceReport,
}

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

const MAX_NATIVE_RUNS: usize = 10_000;
const MAX_NATIVE_RECORDS: usize = 2_000_000;
const MAX_NATIVE_LINE_BYTES: usize = 64 * 1024;
const MAX_NATIVE_TOKENS: usize = 4_096;
const MAX_NATIVE_DEPTH: usize = 8;
const MAX_NATIVE_METADATA: usize = 100_000;
const MAX_NATIVE_COMBATS: usize = 10_000;
const MAX_NATIVE_COMBAT_RECORDS: usize = 200_000;
const MAX_NATIVE_COMBAT_BYTES: usize = 32 * 1024 * 1024;
const MAX_NATIVE_CONTEXT_BYTES: usize = 8 * 1024 * 1024;
const MAX_NATIVE_BATCH_BYTES: usize = 512 * 1024 * 1024;
const MAX_NATIVE_CONTEXT_WORK_RECORDS: usize = 20_000_000;
const MAX_NATIVE_CONTEXT_WORK_BYTES: usize = 512 * 1024 * 1024;

fn rejected<T>() -> Result<T, EncounterError> {
    invalid("native record does not match the supported normal version15 profile")
}

fn fields(line: &str) -> Result<Vec<&str>, EncounterError> {
    if line.is_empty()
        || line.len() > MAX_NATIVE_LINE_BYTES
        || line.bytes().any(|b| b < 32 && b != b'\t')
    {
        return rejected();
    }
    let bytes = line.as_bytes();
    let mut result = Vec::new();
    let (mut start, mut i, mut depth, mut tokens) = (0, 0, 0_usize, 1_usize);
    let mut quoted = false;
    while i < bytes.len() {
        let b = bytes[i];
        if quoted && b == b'\\' {
            if i + 1 == bytes.len() {
                return rejected();
            }
            i += 2;
            continue;
        }
        if b == b'"' {
            if quoted && bytes.get(i + 1) == Some(&b'"') {
                i += 2;
                continue;
            }
            quoted = !quoted;
        } else if !quoted {
            match b {
                b'[' => {
                    depth += 1;
                    if depth > MAX_NATIVE_DEPTH {
                        return rejected();
                    }
                }
                b']' => {
                    if depth == 0 {
                        return rejected();
                    }
                    depth -= 1;
                }
                b',' => {
                    tokens += 1;
                    if tokens > MAX_NATIVE_TOKENS {
                        return rejected();
                    }
                    if depth == 0 {
                        result.push(line[start..i].trim());
                        start = i + 1;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    if quoted || depth != 0 {
        return rejected();
    }
    result.push(line[start..].trim());
    Ok(result)
}

fn uint(value: &str) -> Result<u64, EncounterError> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return rejected();
    }
    value
        .parse()
        .map_err(|_| EncounterError::Validation("native integer is out of range".into()))
}

fn number(value: &str) -> Result<(), EncounterError> {
    let bytes = value.as_bytes();
    let mut i = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));
    let mut digits = 0;
    while bytes.get(i).is_some_and(u8::is_ascii_digit) {
        i += 1;
        digits += 1;
    }
    if bytes.get(i) == Some(&b'.') {
        i += 1;
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 || value.len() > 128 {
        return rejected();
    }
    if matches!(bytes.get(i), Some(b'e' | b'E')) {
        i += 1;
        if matches!(bytes.get(i), Some(b'+' | b'-')) {
            i += 1;
        }
        let begin = i;
        while bytes.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1;
        }
        if begin == i {
            return rejected();
        }
    }
    if i != bytes.len() {
        return rejected();
    }
    Ok(())
}

fn flag(value: &str) -> Result<(), EncounterError> {
    if matches!(value, "T" | "F") {
        Ok(())
    } else {
        rejected()
    }
}

fn word(value: &str) -> Result<(), EncounterError> {
    if !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
    {
        Ok(())
    } else {
        rejected()
    }
}

fn quoted_value(value: &str) -> Result<String, EncounterError> {
    if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
        return rejected();
    }
    let inner = &value[1..value.len() - 1];
    let mut chars = inner.chars().peekable();
    let mut result = String::new();
    while let Some(c) = chars.next() {
        if c == '"' {
            if chars.next() != Some('"') {
                return rejected();
            }
            result.push('"');
        } else if c == '\\' {
            let Some(next) = chars.next() else {
                return rejected();
            };
            match next {
                '"' | '\\' | ',' => result.push(next),
                _ => return rejected(),
            }
        } else {
            result.push(c);
        }
    }
    Ok(result)
}

fn state(values: &[&str], at: usize) -> Result<(), EncounterError> {
    if values.len() < at + 10 {
        return rejected();
    }
    uint(values[at])?;
    for ratio in &values[at + 1..at + 6] {
        let Some((current, maximum)) = ratio.split_once('/') else {
            return rejected();
        };
        number(current)?;
        number(maximum)?;
    }
    for value in &values[at + 6..at + 10] {
        number(value)?;
    }
    Ok(())
}

fn target(values: &[&str], at: usize) -> Result<usize, EncounterError> {
    if values.get(at) == Some(&"*") {
        return Ok(at + 1);
    }
    state(values, at)?;
    Ok(at + 10)
}

fn array(value: &str) -> Result<Vec<&str>, EncounterError> {
    if !value.starts_with('[') || !value.ends_with(']') {
        return rejected();
    }
    let inner = &value[1..value.len() - 1];
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    fields(inner)
}

fn validate_layout(values: &[&str]) -> Result<bool, EncounterError> {
    if values.len() < 2 {
        return rejected();
    }
    uint(values[0])?;
    word(values[1])?;
    let n = values.len();
    match values[1] {
        "BEGIN_LOG" => {
            if n != 7 {
                return rejected();
            }
            uint(values[2])?;
            if uint(values[3])? != 15 {
                return Err(EncounterError::UnsupportedVersion);
            }
            for value in &values[4..7] {
                quoted_value(value)?;
            }
        }
        "END_LOG" | "BEGIN_COMBAT" | "END_COMBAT" => {
            if n != 2 {
                return rejected();
            }
        }
        "COMBAT_EVENT" => {
            if n != 20 && n != 29 {
                return rejected();
            }
            for value in &values[2..5] {
                word(value)?;
            }
            for value in &values[5..9] {
                uint(value)?;
            }
            state(values, 9)?;
            if target(values, 19)? != n {
                return rejected();
            }
        }
        "BEGIN_CAST" => {
            if n != 17 && n != 26 {
                return rejected();
            }
            uint(values[2])?;
            flag(values[3])?;
            uint(values[4])?;
            uint(values[5])?;
            state(values, 6)?;
            if target(values, 16)? != n {
                return rejected();
            }
        }
        "END_CAST" => {
            if n != 5 && n != 7 {
                return rejected();
            }
            word(values[2])?;
            // The mandatory extra integer is observed but not assigned a meaning.
            for value in &values[3..] {
                uint(value)?;
            }
        }
        "EFFECT_CHANGED" => {
            if !matches!(n, 17 | 18 | 26 | 27) {
                return rejected();
            }
            word(values[2])?;
            for value in &values[3..6] {
                uint(value)?;
            }
            state(values, 6)?;
            let end = target(values, 16)?;
            if n == end + 1 {
                uint(values[end])?;
            } else if n != end {
                return rejected();
            }
        }
        "HEALTH_REGEN" => {
            if n != 13 {
                return rejected();
            }
            uint(values[2])?;
            state(values, 3)?;
        }
        "UNIT_ADDED" => {
            if n != 18 {
                return rejected();
            }
            uint(values[2])?;
            word(values[3])?;
            flag(values[4])?;
            uint(values[5])?;
            uint(values[6])?;
            flag(values[7])?;
            uint(values[8])?;
            uint(values[9])?;
            quoted_value(values[10])?;
            quoted_value(values[11])?;
            for value in &values[12..16] {
                uint(value)?;
            }
            word(values[16])?;
            flag(values[17])?;
        }
        "UNIT_CHANGED" => {
            if n != 13 {
                return rejected();
            }
            for value in &values[2..5] {
                uint(value)?;
            }
            quoted_value(values[5])?;
            quoted_value(values[6])?;
            for value in &values[7..11] {
                uint(value)?;
            }
            word(values[11])?;
            flag(values[12])?;
        }
        "UNIT_REMOVED" => {
            if n != 3 {
                return rejected();
            }
            uint(values[2])?;
        }
        "ABILITY_INFO" => {
            if n != 7 {
                return rejected();
            }
            uint(values[2])?;
            quoted_value(values[3])?;
            quoted_value(values[4])?;
            flag(values[5])?;
            flag(values[6])?;
        }
        "EFFECT_INFO" => {
            if n != 6 && n != 7 {
                return rejected();
            }
            uint(values[2])?;
            for value in &values[3..6] {
                word(value)?;
            }
            if n == 7 {
                uint(values[6])?;
            }
        }
        "PLAYER_INFO" => {
            if n != 8 {
                return rejected();
            }
            uint(values[2])?;
            let effects = array(values[3])?;
            let stacks = array(values[4])?;
            if effects.len() != stacks.len() {
                return rejected();
            }
            for value in effects.iter().chain(&stacks) {
                uint(value)?;
            }
            for equipment in array(values[5])? {
                let item = array(equipment)?;
                if item.len() != 11 {
                    return rejected();
                }
                word(item[0])?;
                uint(item[1])?;
                flag(item[2])?;
                uint(item[3])?;
                word(item[4])?;
                word(item[5])?;
                uint(item[6])?;
                word(item[7])?;
                flag(item[8])?;
                uint(item[9])?;
                word(item[10])?;
            }
            for list in &values[6..8] {
                for value in array(list)? {
                    uint(value)?;
                }
            }
        }
        "MAP_CHANGED" | "MAP_INFO" | "ZONE_CHANGED" | "ZONE_INFO" => {
            if n != 5 {
                return rejected();
            }
            uint(values[2])?;
            quoted_value(values[3])?;
            if values[1].starts_with("MAP_") {
                quoted_value(values[4])?;
            } else {
                word(values[4])?;
            }
        }
        "ENDLESS_DUNGEON_INIT" | "ENDLESS_DUNGEON_BEGIN" => {
            if n != 5 {
                return rejected();
            }
            uint(values[2])?;
            uint(values[3])?;
            flag(values[4])?;
        }
        "ENDLESS_DUNGEON_STAGE_END"
        | "ENDLESS_DUNGEON_BUFF_ADDED"
        | "ENDLESS_DUNGEON_BUFF_REMOVED" => {
            if n != 4 {
                return rejected();
            }
            uint(values[2])?;
            uint(values[3])?;
        }
        "ENDLESS_DUNGEON_END" => {
            if n != 6 {
                return rejected();
            }
            for value in &values[2..5] {
                uint(value)?;
            }
            flag(values[5])?;
        }
        _ => return Ok(false),
    }
    Ok(true)
}

fn validate_layout_for(values: &[&str], schema: u32) -> Result<bool, EncounterError> {
    if schema == NATIVE_INCREMENTAL_SCHEMA_VERSION
        && values.get(1) == Some(&"ABILITY_INFO")
        && values.len() == 10
    {
        validate_layout(&values[..7])?;
        for value in &values[7..] {
            quoted_value(value)?;
        }
        return Ok(true);
    }
    validate_layout(values)
}

fn run_id(header: &NativeHeader, ordinal: u64, channel: Channel) -> Result<String, EncounterError> {
    let bytes = serde_json::to_vec(&(NATIVE_SCHEMA_VERSION, ordinal, channel, &header.record.line))
        .map_err(|_| EncounterError::Validation("native header identity is invalid".into()))?;
    Ok(format!("native-session-{}", super::sha256(&bytes)))
}
fn encounter_id(run: &str, ordinal: u64) -> String {
    format!(
        "native-encounter-{}",
        super::sha256(format!("{run}:{ordinal}").as_bytes())
    )
}

fn logical_run_key(header: &NativeHeader, channel: Channel) -> Result<String, EncounterError> {
    let bytes = serde_json::to_vec(&(
        "eso-weave/native-logical-run/v1",
        channel,
        header.epoch_ms,
        header.log_version,
        &header.realm,
        &header.language,
        &header.game_version,
    ))
    .map_err(|_| EncounterError::Validation("native logical header is invalid".into()))?;
    Ok(super::sha256(&bytes))
}

fn logical_fight_key(run_key: &str, ordinal: u64) -> Result<String, EncounterError> {
    let bytes = serde_json::to_vec(&("eso-weave/native-logical-fight/v1", run_key, ordinal))
        .map_err(|_| EncounterError::Validation("native logical identity is invalid".into()))?;
    Ok(super::sha256(&bytes))
}

fn run_id_for(
    header: &NativeHeader,
    ordinal: u64,
    channel: Channel,
    schema: u32,
) -> Result<String, EncounterError> {
    if schema == NATIVE_INCREMENTAL_SCHEMA_VERSION {
        Ok(format!(
            "native-session-v2-{}",
            logical_run_key(header, channel)?
        ))
    } else {
        run_id(header, ordinal, channel)
    }
}

fn encounter_id_for(run: &NativeRun, ordinal: u64) -> Result<String, EncounterError> {
    if run.schema_version == NATIVE_INCREMENTAL_SCHEMA_VERSION {
        let key = logical_run_key(&run.header, run.channel)?;
        Ok(format!(
            "native-encounter-v2-{}",
            logical_fight_key(&key, ordinal)?
        ))
    } else {
        Ok(encounter_id(&run.session_id, ordinal))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLogicalIdentity {
    pub run_key: String,
    pub fight_key: String,
    pub evidence_sha256: String,
}

pub fn logical_native_identity_v1(
    capture: &NativeEncounter,
) -> Result<NativeLogicalIdentity, EncounterError> {
    if capture.schema_version != NATIVE_SCHEMA_VERSION {
        return rejected();
    }
    logical_native_identity(capture)
}

/// Compares logical evidence without changing retained canonical provenance.
pub fn logical_native_identity(
    capture: &NativeEncounter,
) -> Result<NativeLogicalIdentity, EncounterError> {
    validate_native_encounter(capture)?;
    let run_key = logical_run_key(&capture.run.header, capture.run.channel)?;
    let fight_key = logical_fight_key(&run_key, capture.encounter_ordinal)?;
    let mut normalized = capture.clone();
    normalized.schema_version = NATIVE_INCREMENTAL_SCHEMA_VERSION;
    normalized.run.schema_version = NATIVE_INCREMENTAL_SCHEMA_VERSION;
    normalized.run.run_ordinal = 1;
    normalized.run.session_id = format!("native-session-v2-{run_key}");
    normalized.encounter_id = format!("native-encounter-v2-{fight_key}");
    let origin = &capture.run.header.record;
    let normalize_ordinal = |ordinal: u64| {
        ordinal
            .checked_sub(origin.ordinal)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| EncounterError::Validation("native relative ordinal is invalid".into()))
    };
    normalized.first_ordinal = normalize_ordinal(capture.first_ordinal)?;
    normalized.last_ordinal = normalize_ordinal(capture.last_ordinal)?;
    normalized.run.header.record.ordinal = 1;
    normalized.run.header.record.byte_offset = 0;
    for record in normalized.context.iter_mut().chain(&mut normalized.records) {
        record.ordinal = normalize_ordinal(record.ordinal)?;
        record.byte_offset = record
            .byte_offset
            .checked_sub(origin.byte_offset)
            .ok_or_else(|| {
                EncounterError::Validation("native relative offset is invalid".into())
            })?;
    }
    let bytes = serde_json::to_vec(&("eso-weave/native-logical-evidence/v1", &normalized))
        .map_err(|_| EncounterError::Validation("native logical evidence is invalid".into()))?;
    Ok(NativeLogicalIdentity {
        run_key,
        fight_key,
        evidence_sha256: super::sha256(&bytes),
    })
}

#[derive(Default)]
struct ContextWork {
    records: usize,
    bytes: usize,
}
impl ContextWork {
    fn charge(&mut self, records: usize, bytes: usize) -> Result<(), EncounterError> {
        let records = self
            .records
            .checked_add(records)
            .ok_or(EncounterError::NativeWorkLimit)?;
        let bytes = self
            .bytes
            .checked_add(bytes)
            .ok_or(EncounterError::NativeWorkLimit)?;
        if records > MAX_NATIVE_CONTEXT_WORK_RECORDS || bytes > MAX_NATIVE_CONTEXT_WORK_BYTES {
            return Err(EncounterError::NativeWorkLimit);
        }
        self.records = records;
        self.bytes = bytes;
        Ok(())
    }
}

#[derive(Default, Clone)]
struct Context {
    metadata: BTreeMap<(String, u64), Arc<NativeRecord>>,
    unknown: Vec<Arc<NativeRecord>>,
    bytes: usize,
}
impl Context {
    fn remember(
        &mut self,
        key: (String, u64),
        record: &NativeRecord,
    ) -> Result<(), EncounterError> {
        let previous = self.metadata.insert(key, Arc::new(record.clone()));
        self.bytes -= previous.map_or(0, |old| old.line.len());
        self.bytes = self
            .bytes
            .checked_add(record.line.len())
            .ok_or(EncounterError::NativeWorkLimit)?;
        Ok(())
    }

    fn observe(
        &mut self,
        record: &NativeRecord,
        values: &[&str],
        known: bool,
    ) -> Result<(), EncounterError> {
        if !known {
            self.unknown.push(Arc::new(record.clone()));
            self.bytes = self
                .bytes
                .checked_add(record.line.len())
                .ok_or(EncounterError::NativeWorkLimit)?;
        }
        match values[1] {
            "UNIT_ADDED" | "UNIT_CHANGED" | "PLAYER_INFO" | "ABILITY_INFO" | "EFFECT_INFO" => {
                let id = uint(values[2])?;
                self.remember((values[1].into(), id), record)?;
            }
            "UNIT_REMOVED" => {
                let id = uint(values[2])?;
                for kind in ["UNIT_ADDED", "UNIT_CHANGED", "PLAYER_INFO"] {
                    if let Some(removed) = self.metadata.remove(&(kind.into(), id)) {
                        self.bytes -= removed.line.len();
                    }
                }
            }
            "MAP_CHANGED" | "MAP_INFO" | "ZONE_CHANGED" | "ZONE_INFO" => {
                self.remember((values[1].into(), 0), record)?;
            }
            _ => {}
        }
        if self.metadata.len() + self.unknown.len() > MAX_NATIVE_METADATA {
            return rejected();
        }
        Ok(())
    }
    fn snapshot(&self) -> Vec<Arc<NativeRecord>> {
        self.metadata
            .values()
            .chain(self.unknown.iter())
            .cloned()
            .collect()
    }
}

#[derive(Clone)]
struct Pending {
    ordinal: u64,
    start_ms: u64,
    first: u64,
    records: Vec<NativeRecord>,
    context: Vec<Arc<NativeRecord>>,
    units: BTreeSet<u64>,
    abilities: BTreeSet<u64>,
    bytes: usize,
    unknown: usize,
    previous_ms: u64,
    reversals: usize,
}
impl Pending {
    fn add(
        &mut self,
        record: NativeRecord,
        values: &[&str],
        known: bool,
    ) -> Result<(), EncounterError> {
        let now = uint(values[0])?;
        if now < self.previous_ms {
            self.reversals += 1;
        }
        self.previous_ms = now;
        if !known {
            self.unknown += 1;
        }
        self.bytes = self.bytes.checked_add(record.line.len()).ok_or_else(|| {
            EncounterError::Validation("native combat byte count overflowed".into())
        })?;
        if self.records.len() >= MAX_NATIVE_COMBAT_RECORDS || self.bytes > MAX_NATIVE_COMBAT_BYTES {
            return rejected();
        }
        match values[1] {
            "COMBAT_EVENT" => {
                self.abilities.insert(uint(values[8])?);
                self.references(values, 9, 19)?;
            }
            "BEGIN_CAST" | "EFFECT_CHANGED" => {
                self.abilities.insert(uint(values[5])?);
                self.references(values, 6, 16)?;
            }
            "HEALTH_REGEN" => {
                self.units.insert(uint(values[3])?);
            }
            "UNIT_ADDED" | "UNIT_CHANGED" | "UNIT_REMOVED" | "PLAYER_INFO" => {
                self.units.insert(uint(values[2])?);
            }
            "ABILITY_INFO" | "EFFECT_INFO" => {
                self.abilities.insert(uint(values[2])?);
            }
            _ => {}
        }
        self.records.push(record);
        Ok(())
    }
    fn references(
        &mut self,
        values: &[&str],
        source: usize,
        target: usize,
    ) -> Result<(), EncounterError> {
        self.units.insert(uint(values[source])?);
        if values[target] != "*" {
            self.units.insert(uint(values[target])?);
        }
        Ok(())
    }
    fn finish(self, run: NativeRun, finished_ms: u64) -> Result<NativeEncounter, EncounterError> {
        if finished_ms < self.start_ms {
            return rejected();
        }
        run.header
            .epoch_ms
            .checked_add(finished_ms)
            .ok_or_else(|| EncounterError::Validation("native timestamp overflowed".into()))?;
        let mut context = Vec::new();
        let mut unknown_context_count = 0;
        let mut context_bytes = 0_usize;
        for record in self.context {
            let values = fields(&record.line)?;
            let known = validate_layout_for(&values, run.schema_version)?;
            let relevant = match values[1] {
                "UNIT_ADDED" => self.units.contains(&uint(values[2])?) || values[4] == "T",
                "UNIT_CHANGED" | "PLAYER_INFO" => self.units.contains(&uint(values[2])?),
                "ABILITY_INFO" | "EFFECT_INFO" => self.abilities.contains(&uint(values[2])?),
                _ => true,
            };
            if relevant {
                if !known {
                    unknown_context_count += 1;
                }
                context_bytes = context_bytes
                    .checked_add(record.line.len())
                    .ok_or_else(|| {
                        EncounterError::Validation("native context byte count overflowed".into())
                    })?;
                if context_bytes > MAX_NATIVE_CONTEXT_BYTES {
                    return rejected();
                }
                context.push((*record).clone());
            }
        }
        context.sort_by_key(|record| record.ordinal);
        let last = self
            .records
            .last()
            .ok_or_else(|| {
                EncounterError::Validation("native combat has no terminal record".into())
            })?
            .ordinal;
        Ok(NativeEncounter {
            schema_version: run.schema_version,
            encounter_id: encounter_id_for(&run, self.ordinal)?,
            run,
            encounter_ordinal: self.ordinal,
            started_ms: self.start_ms,
            finished_ms,
            first_ordinal: self.first,
            last_ordinal: last,
            context,
            records: self.records,
            unknown_record_count: self.unknown,
            unknown_context_count,
            clock_reversal_count: self.reversals,
        })
    }
}

pub fn parse_native_log(bytes: &[u8], channel: Channel) -> Result<ParsedNativeLog, EncounterError> {
    if bytes.len() as u64 > MAX_NATIVE_FILE_BYTES {
        return invalid("native file exceeds its byte limit");
    }
    let terminated = bytes.iter().rposition(|b| *b == b'\n').map_or(0, |i| i + 1);
    let tail = &bytes[terminated..];
    if tail.len() > MAX_NATIVE_LINE_BYTES {
        return rejected();
    }
    std::str::from_utf8(bytes)
        .map_err(|_| EncounterError::Validation("native file is not UTF-8".into()))?;
    let text = std::str::from_utf8(&bytes[..terminated])
        .map_err(|_| EncounterError::Validation("native file is not UTF-8".into()))?;
    let mut parsed = ParsedNativeLog {
        runs: Vec::new(),
        encounters: Vec::new(),
        report: NativeSourceReport::default(),
    };
    parsed.report.unterminated_line_count = usize::from(!tail.is_empty());
    let mut current: Option<NativeRun> = None;
    let mut context = Context::default();
    let mut pending: Option<Pending> = None;
    let mut combat_ordinal = 0_u64;
    let mut offset = 0_u64;
    let mut batch_bytes = 0_usize;
    let mut context_work = ContextWork::default();
    for terminated_line in text.split_inclusive('\n') {
        let raw = terminated_line
            .trim_end_matches('\n')
            .strip_suffix('\r')
            .unwrap_or(terminated_line.trim_end_matches('\n'));
        let raw = if offset == 0 {
            raw.strip_prefix('\u{feff}').unwrap_or(raw)
        } else {
            raw
        };
        if raw.is_empty() {
            return rejected();
        }
        parsed.report.record_count += 1;
        if parsed.report.record_count > MAX_NATIVE_RECORDS {
            return rejected();
        }
        let values = fields(raw)?;
        let known = validate_layout(&values)?;
        let time = uint(values[0])?;
        let record = NativeRecord {
            ordinal: parsed.report.record_count as u64,
            byte_offset: offset
                + u64::from(offset == 0 && terminated_line.starts_with('\u{feff}')) * 3,
            line: raw.into(),
        };
        offset += terminated_line.len() as u64;
        if !known {
            parsed.report.unknown_record_count += 1;
        }
        match values[1] {
            "BEGIN_LOG" => {
                if current.is_some() {
                    parsed.report.incomplete_run_count += 1;
                }
                if pending.take().is_some() {
                    parsed.report.incomplete_combat_count += 1;
                }
                let header = NativeHeader {
                    epoch_ms: uint(values[2])?,
                    log_version: 15,
                    realm: quoted_value(values[4])?,
                    language: quoted_value(values[5])?,
                    game_version: quoted_value(values[6])?,
                    record,
                };
                if parsed.report.run_count >= MAX_NATIVE_RUNS {
                    return rejected();
                }
                let ordinal = parsed.report.run_count as u64 + 1;
                let run = NativeRun {
                    schema_version: NATIVE_SCHEMA_VERSION,
                    session_id: run_id(&header, ordinal, channel)?,
                    run_ordinal: ordinal,
                    channel,
                    header,
                };
                parsed.report.run_count += 1;
                parsed.runs.push(run.clone());
                current = Some(run);
                context = Context::default();
                combat_ordinal = 0;
                parsed.report.outside_combat_record_count += 1;
            }
            "BEGIN_COMBAT" => {
                if current.is_none() || pending.is_some() {
                    return rejected();
                }
                // Reserve cumulative work before cloning or revalidating context.
                // This budget spans every recording run in the physical file.
                context_work.charge(
                    context.metadata.len() + context.unknown.len(),
                    context.bytes,
                )?;
                combat_ordinal += 1;
                let mut capture = Pending {
                    ordinal: combat_ordinal,
                    start_ms: time,
                    first: record.ordinal,
                    records: Vec::new(),
                    context: context.snapshot(),
                    units: BTreeSet::new(),
                    abilities: BTreeSet::new(),
                    bytes: 0,
                    unknown: 0,
                    previous_ms: time,
                    reversals: 0,
                };
                capture.add(record, &values, known)?;
                pending = Some(capture);
            }
            "END_COMBAT" => {
                let Some(mut capture) = pending.take() else {
                    return rejected();
                };
                capture.add(record, &values, known)?;
                let encounter = capture.finish(
                    current.clone().ok_or_else(|| {
                        EncounterError::Validation("native run is missing".into())
                    })?,
                    time,
                )?;
                batch_bytes = batch_bytes
                    .checked_add(
                        encounter
                            .records
                            .iter()
                            .chain(&encounter.context)
                            .map(|r| r.line.len())
                            .sum::<usize>(),
                    )
                    .ok_or_else(|| {
                        EncounterError::Validation("native batch byte count overflowed".into())
                    })?;
                if parsed.encounters.len() >= MAX_NATIVE_COMBATS
                    || batch_bytes > MAX_NATIVE_BATCH_BYTES
                {
                    return rejected();
                }
                parsed.encounters.push(encounter);
            }
            "END_LOG" => {
                if current.take().is_none() {
                    return rejected();
                }
                if pending.take().is_some() {
                    parsed.report.incomplete_combat_count += 1;
                }
                context = Context::default();
                parsed.report.outside_combat_record_count += 1;
            }
            _ => {
                if current.is_none() {
                    return rejected();
                }
                if let Some(capture) = &mut pending {
                    capture.add(record.clone(), &values, known)?;
                } else {
                    parsed.report.outside_combat_record_count += 1;
                }
                context.observe(&record, &values, known)?;
            }
        }
    }
    if current.is_some() {
        parsed.report.incomplete_run_count += 1;
    }
    if pending.is_some() {
        parsed.report.incomplete_combat_count += 1;
    }
    if parsed.report.run_count == 0 {
        return invalid("native file has no supported BEGIN_LOG header");
    }
    parsed.report.completed_encounter_count = parsed.encounters.len();
    Ok(parsed)
}

pub const MAX_NATIVE_CANONICAL_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct NativeSourceInspection {
    pub path: std::path::PathBuf,
    pub channel: Channel,
    pub modified: Option<std::time::SystemTime>,
    pub source_sha256: String,
    pub report: NativeSourceReport,
}

pub fn read_native_source(
    path: &std::path::Path,
    channel: Channel,
) -> Result<(String, ParsedNativeLog, Option<std::time::SystemTime>), EncounterError> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(std::path::Path::new("."));
    let root = std::fs::canonicalize(parent)?;
    let (bytes, modified) =
        crate::bounded_file::read_bounded_stable_snapshot(&root, path, MAX_NATIVE_FILE_BYTES)
            .map_err(|error| match error {
                crate::bounded_file::StableReadError::Io(error) => EncounterError::Io(error),
                crate::bounded_file::StableReadError::Invalid(_) => EncounterError::Validation(
                    "native file could not be read as a bounded stable regular file".into(),
                ),
            })?;
    let parsed = parse_native_log(&bytes, channel)?;
    Ok((super::sha256(&bytes), parsed, modified))
}

pub fn inspect_native_log(
    path: impl AsRef<std::path::Path>,
    channel: Channel,
) -> Result<NativeSourceInspection, EncounterError> {
    let path = path.as_ref();
    let (source_sha256, parsed, modified) = read_native_source(path, channel)?;
    Ok(NativeSourceInspection {
        path: path.into(),
        channel,
        modified,
        source_sha256,
        report: parsed.report,
    })
}

pub(crate) fn validate_native_run(run: &NativeRun) -> Result<(), EncounterError> {
    if !matches!(
        run.schema_version,
        NATIVE_SCHEMA_VERSION | NATIVE_INCREMENTAL_SCHEMA_VERSION
    ) || run.run_ordinal == 0
        || (run.schema_version == NATIVE_SCHEMA_VERSION && run.run_ordinal > MAX_NATIVE_RUNS as u64)
        || run.header.log_version != 15
    {
        return rejected();
    }
    validate_native_record(&run.header.record, run.schema_version)?;
    let values = fields(&run.header.record.line)?;
    validate_layout(&values)?;
    if values[1] != "BEGIN_LOG"
        || uint(values[2])? != run.header.epoch_ms
        || quoted_value(values[4])? != run.header.realm
        || quoted_value(values[5])? != run.header.language
        || quoted_value(values[6])? != run.header.game_version
        || run.session_id
            != run_id_for(
                &run.header,
                run.run_ordinal,
                run.channel,
                run.schema_version,
            )?
    {
        return rejected();
    }
    Ok(())
}

fn validate_native_record(record: &NativeRecord, schema: u32) -> Result<(), EncounterError> {
    if record.ordinal == 0
        || (schema == NATIVE_SCHEMA_VERSION
            && (record.ordinal > MAX_NATIVE_RECORDS as u64
                || record.byte_offset > MAX_NATIVE_FILE_BYTES))
    {
        return rejected();
    }
    validate_layout_for(&fields(&record.line)?, schema)?;
    Ok(())
}

pub fn validate_native_encounter(capture: &NativeEncounter) -> Result<(), EncounterError> {
    validate_native_run(&capture.run)?;
    if capture.schema_version != capture.run.schema_version
        || capture.encounter_ordinal == 0
        || (capture.schema_version == NATIVE_SCHEMA_VERSION
            && capture.encounter_ordinal > MAX_NATIVE_RECORDS as u64)
        || capture.encounter_id != encounter_id_for(&capture.run, capture.encounter_ordinal)?
        || capture.records.len() < 2
        || capture.records.len() > MAX_NATIVE_COMBAT_RECORDS
        || capture.context.len() > MAX_NATIVE_METADATA
        || capture.first_ordinal <= capture.run.header.record.ordinal
        || capture.last_ordinal < capture.first_ordinal
        || capture.finished_ms < capture.started_ms
    {
        return rejected();
    }
    capture
        .run
        .header
        .epoch_ms
        .checked_add(capture.finished_ms)
        .ok_or_else(|| EncounterError::Validation("native timestamp overflowed".into()))?;
    let mut bytes = 0_usize;
    let mut unknown = 0;
    let mut previous_time = capture.started_ms;
    let mut reversals = 0;
    let mut previous_offset = 0;
    for (i, record) in capture.records.iter().enumerate() {
        validate_native_record(record, capture.schema_version)?;
        let values = fields(&record.line)?;
        let known = validate_layout_for(&values, capture.schema_version)?;
        if Some(record.ordinal) != capture.first_ordinal.checked_add(i as u64)
            || (i > 0 && record.byte_offset <= previous_offset)
        {
            return rejected();
        }
        previous_offset = record.byte_offset;
        let time = uint(values[0])?;
        if time < previous_time {
            reversals += 1;
        }
        previous_time = time;
        if !known {
            unknown += 1;
        }
        if i == 0 {
            if values[1] != "BEGIN_COMBAT" || time != capture.started_ms {
                return rejected();
            }
        } else if i + 1 == capture.records.len() {
            if values[1] != "END_COMBAT"
                || time != capture.finished_ms
                || record.ordinal != capture.last_ordinal
            {
                return rejected();
            }
        } else if matches!(
            values[1],
            "BEGIN_COMBAT" | "END_COMBAT" | "BEGIN_LOG" | "END_LOG"
        ) {
            return rejected();
        }
        bytes += record.line.len();
    }
    if bytes > MAX_NATIVE_COMBAT_BYTES
        || unknown != capture.unknown_record_count
        || reversals != capture.clock_reversal_count
    {
        return rejected();
    }
    let mut prior = capture.run.header.record.ordinal;
    let mut unknown_context = 0;
    bytes = 0;
    for record in &capture.context {
        validate_native_record(record, capture.schema_version)?;
        if record.ordinal <= prior || record.ordinal >= capture.first_ordinal {
            return rejected();
        }
        prior = record.ordinal;
        let values = fields(&record.line)?;
        let known = validate_layout_for(&values, capture.schema_version)?;
        if known
            && !matches!(
                values[1],
                "UNIT_ADDED"
                    | "UNIT_CHANGED"
                    | "PLAYER_INFO"
                    | "ABILITY_INFO"
                    | "EFFECT_INFO"
                    | "MAP_CHANGED"
                    | "MAP_INFO"
                    | "ZONE_CHANGED"
                    | "ZONE_INFO"
            )
        {
            return rejected();
        }
        if !known {
            unknown_context += 1;
        }
        bytes += record.line.len();
    }
    if bytes > MAX_NATIVE_CONTEXT_BYTES || unknown_context != capture.unknown_context_count {
        return rejected();
    }
    Ok(())
}

pub fn canonical_native_bytes(capture: &NativeEncounter) -> Result<Vec<u8>, EncounterError> {
    validate_native_encounter(capture)?;
    let bytes = serde_json::to_vec(capture)
        .map_err(|_| EncounterError::Validation("native canonical encoding failed".into()))?;
    if bytes.len() > MAX_NATIVE_CANONICAL_BYTES {
        return rejected();
    }
    Ok(bytes)
}

pub(crate) fn canonical_native_run_bytes(run: &NativeRun) -> Result<Vec<u8>, EncounterError> {
    validate_native_run(run)?;
    serde_json::to_vec(run)
        .map_err(|_| EncounterError::Validation("native run encoding failed".into()))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeImportReceipt {
    pub outcome: super::ImportOutcome,
    pub session_id: String,
    pub encounter_id: String,
    pub content_sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NativeImportReport {
    pub source_sha256: String,
    pub imported_count: usize,
    pub already_present_count: usize,
    pub source: NativeSourceReport,
    pub receipts: Vec<NativeImportReceipt>,
}

pub fn import_native_log(
    request: &super::ImportRequest,
) -> Result<NativeImportReport, EncounterError> {
    super::ensure_distinct_paths(&request.input_path, &request.store_path)?;
    let (source_sha256, parsed, _) =
        read_native_source(&request.input_path, request.expected_channel)?;
    if parsed.encounters.is_empty() {
        return Ok(NativeImportReport {
            source_sha256,
            imported_count: 0,
            already_present_count: 0,
            source: parsed.report,
            receipts: Vec::new(),
        });
    }
    let prepared = parsed
        .encounters
        .iter()
        .map(|capture| {
            let canonical = canonical_native_bytes(capture)?;
            let hash = super::sha256(&canonical);
            Ok((canonical, hash))
        })
        .collect::<Result<Vec<_>, EncounterError>>()?;
    super::store::append_native_set(&request.store_path, &parsed, &prepared, source_sha256)
}

/// Counts and reviewed quantities from native evidence, without catalog authority.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NativeProjection {
    pub algorithm_version: u32,
    pub raw_content_sha256: String,
    pub log_version: u32,
    pub channel: Channel,
    pub duration_ms: u64,
    pub record_count: usize,
    pub combat_event_count: usize,
    pub cast_start_count: usize,
    pub effect_change_count: usize,
    pub unknown_record_count: usize,
    pub damage_hit: u128,
    pub damage_overflow: u128,
    pub healing_hit: u128,
    pub healing_overflow: u128,
    pub local_damage_hit: Option<u128>,
    pub local_healing_hit: Option<u128>,
    pub local_damage_per_second: Option<f64>,
    pub quality_reasons: Vec<String>,
    pub unavailable: Vec<String>,
}

/// Uses ordered observed unit relations, never names or an inferred player identity.
pub fn project_native_encounter(
    capture: &NativeEncounter,
) -> Result<NativeProjection, EncounterError> {
    let canonical = canonical_native_bytes(capture)?;
    let mut projection = NativeProjection {
        algorithm_version: 1,
        raw_content_sha256: super::sha256(&canonical),
        log_version: capture.run.header.log_version,
        channel: capture.run.channel,
        duration_ms: capture.finished_ms - capture.started_ms,
        record_count: capture.records.len(),
        combat_event_count: 0,
        cast_start_count: 0,
        effect_change_count: 0,
        unknown_record_count: capture.unknown_record_count + capture.unknown_context_count,
        damage_hit: 0, damage_overflow: 0, healing_hit: 0, healing_overflow: 0,
        local_damage_hit: Some(0), local_healing_hit: Some(0), local_damage_per_second: None,
        quality_reasons: Vec::new(),
        unavailable: vec![
            "Effect uptime is unavailable: native effect-change counts do not establish callback-equivalent duration or complete coverage.".into(),
            "Quickslot actions, death-recap snapshots and addon replay are unavailable: this native profile does not contain their addon callback evidence.".into(),
            "Catalog API joins and recommendations are unavailable: native log version and game-version metadata do not declare a game API version.".into(),
        ],
    };
    let mut local_units = BTreeSet::new();
    let mut local_known = true;
    for record in capture.context.iter().chain(&capture.records) {
        let values = fields(&record.line)?;
        match values[1] {
            "UNIT_ADDED" => {
                let id = uint(values[2])?;
                if values[3] == "PLAYER" && values[4] == "T" {
                    local_units.insert(id);
                } else {
                    local_units.remove(&id);
                }
            }
            "UNIT_REMOVED" => {
                local_units.remove(&uint(values[2])?);
            }
            _ => {}
        }
        if record.ordinal < capture.first_ordinal {
            continue;
        }
        match values[1] {
            "BEGIN_CAST" => projection.cast_start_count += 1,
            "EFFECT_CHANGED" => projection.effect_change_count += 1,
            "COMBAT_EVENT" => {
                projection.combat_event_count += 1;
                let hit = uint(values[5])?;
                let overflow = uint(values[6])?;
                let damage = matches!(
                    values[2],
                    "DAMAGE" | "CRITICAL_DAMAGE" | "DOT_TICK" | "DOT_TICK_CRITICAL"
                );
                let healing = matches!(
                    values[2],
                    "HEAL" | "CRITICAL_HEAL" | "HOT_TICK" | "HOT_TICK_CRITICAL"
                );
                if damage {
                    projection.damage_hit = add_quantity(projection.damage_hit, hit)?;
                    projection.damage_overflow =
                        add_quantity(projection.damage_overflow, overflow)?;
                }
                if healing {
                    projection.healing_hit = add_quantity(projection.healing_hit, hit)?;
                    projection.healing_overflow =
                        add_quantity(projection.healing_overflow, overflow)?;
                }
                if local_units.len() != 1 {
                    local_known = false;
                } else if local_units.contains(&uint(values[9])?) {
                    if damage {
                        projection.local_damage_hit =
                            Some(add_quantity(projection.local_damage_hit.unwrap_or(0), hit)?);
                    }
                    if healing {
                        projection.local_healing_hit = Some(add_quantity(
                            projection.local_healing_hit.unwrap_or(0),
                            hit,
                        )?);
                    }
                }
            }
            _ => {}
        }
    }
    if local_units.len() != 1 {
        local_known = false;
    }
    if !local_known {
        projection.local_damage_hit = None;
        projection.local_healing_hit = None;
        projection.unavailable.push("Local-player quantities are unavailable: a unique observed local PLAYER relation is missing for part of the fight.".into());
    }
    if projection.unknown_record_count != 0 {
        projection.quality_reasons.push("Unknown native records are retained; recognized quantities cover only the interpreted observations.".into());
    }
    if capture.clock_reversal_count != 0 {
        projection
            .quality_reasons
            .push("The native observation clock moved backward inside the fight.".into());
    }
    if projection.duration_ms == 0 {
        projection
            .quality_reasons
            .push("The paired combat boundaries have zero elapsed duration.".into());
    }
    if projection.quality_reasons.is_empty() {
        projection.local_damage_per_second = projection
            .local_damage_hit
            .map(|value| value as f64 / (projection.duration_ms as f64 / 1000.0));
    } else {
        projection.unavailable.push("Local damage rate is unavailable because observation coverage or elapsed-time quality is reduced.".into());
    }
    Ok(projection)
}

fn add_quantity(total: u128, value: u64) -> Result<u128, EncounterError> {
    total.checked_add(u128::from(value)).ok_or_else(|| {
        EncounterError::Validation("native observed quantity exceeds the supported total".into())
    })
}

#[cfg(test)]
mod context_work_tests {
    use super::*;

    #[test]
    fn cumulative_context_work_limits_allow_boundary_and_reject_next_charge() {
        let mut records = ContextWork::default();
        records.charge(MAX_NATIVE_CONTEXT_WORK_RECORDS, 0).unwrap();
        assert!(matches!(
            records.charge(1, 0),
            Err(EncounterError::NativeWorkLimit)
        ));
        let mut bytes = ContextWork::default();
        bytes.charge(0, MAX_NATIVE_CONTEXT_WORK_BYTES).unwrap();
        assert!(matches!(
            bytes.charge(0, 1),
            Err(EncounterError::NativeWorkLimit)
        ));
    }

    #[test]
    fn context_byte_accounting_tracks_replacement_removal_and_unknown_records() {
        let mut context = Context::default();
        for (i, line) in [
            "0,UNIT_ADDED,1,PLAYER,T,0,1,F,1,1,\"Fixture\",\"Fixture\",1,1,1,0,PLAYER_ALLY,F",
            "1,UNIT_CHANGED,1,1,1,\"Fixture, Changed\",\"Fixture\",1,1,1,0,PLAYER_ALLY,F",
            "2,UNIT_CHANGED,1,1,1,\"Fixture\",\"Fixture\",1,1,1,0,PLAYER_ALLY,F",
            "3,FUTURE_CONTEXT,1",
            "4,UNIT_REMOVED,1",
        ]
        .iter()
        .enumerate()
        {
            let record = NativeRecord {
                ordinal: i as u64 + 1,
                byte_offset: i as u64,
                line: (*line).into(),
            };
            let values = fields(line).unwrap();
            let known = validate_layout(&values).unwrap();
            context.observe(&record, &values, known).unwrap();
            assert_eq!(
                context.bytes,
                context
                    .snapshot()
                    .iter()
                    .map(|row| row.line.len())
                    .sum::<usize>()
            );
        }
        assert_eq!(context.metadata.len(), 0);
        assert_eq!(context.bytes, "3,FUTURE_CONTEXT,1".len());
    }
}
