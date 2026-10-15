//! Bounded source-independent native version-15 ingestion.
use super::*;
use std::io::Write;

pub const NATIVE_INCREMENTAL_SCHEMA_VERSION: u32 = 2;
pub const NATIVE_CHECKPOINT_VERSION: u32 = 1;
pub const MAX_NATIVE_FEED_BYTES: usize = 256 * 1024;
pub const MAX_NATIVE_CHECKPOINT_BYTES: usize = 128 * 1024 * 1024;
const MAX_DIAGNOSTICS: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativePosition {
    pub accepted_bytes: u64,
    pub complete_bytes: u64,
    pub record_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativeParserStatus {
    AwaitingRun,
    ReadingRun,
    ReadingCombat,
    Recovering,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum NativeParserError {
    #[error("native feed exceeds its byte limit")]
    FeedTooLarge,
    #[error("native source position overflowed")]
    PositionOverflow,
    #[error("native checkpoint is invalid")]
    InvalidCheckpoint,
    #[error("native checkpoint version is unsupported")]
    UnsupportedCheckpoint,
    #[error("native checkpoint exceeds its byte limit")]
    CheckpointTooLarge,
    #[error("native checkpoint channel differs from its source")]
    ChannelMismatch,
    #[error("native source generation has terminated")]
    GenerationTerminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativeDiagnosticCode {
    UnsupportedLogVersion,
    UnsupportedSerialization,
    MalformedKnownRecord,
    MalformedEncoding,
    InvalidBoundary,
    LineLimit,
    ContextLimit,
    FightLimit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeDiagnostic {
    pub code: NativeDiagnosticCode,
    pub start_offset: u64,
    pub end_offset: u64,
    pub last_usable_position: NativePosition,
    pub affected_record_count: u64,
    pub incomplete_evidence_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeCompletedFight {
    pub encounter: NativeEncounter,
    pub logical_identity: NativeLogicalIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeFeed {
    pub accepted: usize,
    pub completed: Option<NativeCompletedFight>,
    pub position: NativePosition,
    pub diagnostics: Vec<NativeDiagnostic>,
    pub status: NativeParserStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NativeTerminationReason {
    RotatedComplete,
    Replaced,
    Truncated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeTermination {
    pub reason: NativeTerminationReason,
    pub position: NativePosition,
    pub incomplete_combat_count: u64,
    pub incomplete_run_count: u64,
    pub unterminated_line_count: u64,
}

#[derive(Clone)]
pub struct NativeCheckpoint {
    bytes: Vec<u8>,
    channel: Channel,
    position: NativePosition,
}
impl fmt::Debug for NativeCheckpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeCheckpoint")
            .field("bytes", &self.bytes.len())
            .field("channel", &self.channel)
            .field("position", &self.position)
            .finish()
    }
}
impl NativeCheckpoint {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, NativeParserError> {
        let parser = decode_checkpoint(bytes)?;
        Ok(Self {
            bytes: bytes.to_vec(),
            channel: parser.channel,
            position: parser.position,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn channel(&self) -> Channel {
        self.channel
    }

    pub fn position(&self) -> NativePosition {
        self.position
    }
}

#[derive(Clone)]
pub struct IncrementalNativeParser {
    channel: Channel,
    position: NativePosition,
    run_count: u64,
    combat_ordinal: u64,
    current: Option<NativeRun>,
    context: Context,
    pending: Option<Pending>,
    tail: Vec<u8>,
    discard_start: Option<u64>,
    recovery: Option<NativeDiagnostic>,
    last_usable_position: NativePosition,
    incomplete_combat_count: u64,
    incomplete_run_count: u64,
    termination: Option<NativeTermination>,
}
impl fmt::Debug for IncrementalNativeParser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncrementalNativeParser")
            .field("channel", &self.channel)
            .field("position", &self.position)
            .field("status", &self.status())
            .finish()
    }
}

impl IncrementalNativeParser {
    pub fn new(channel: Channel) -> Self {
        Self {
            channel,
            position: NativePosition::default(),
            run_count: 0,
            combat_ordinal: 0,
            current: None,
            context: Context::default(),
            pending: None,
            tail: Vec::new(),
            discard_start: None,
            recovery: None,
            last_usable_position: NativePosition::default(),
            incomplete_combat_count: 0,
            incomplete_run_count: 0,
            termination: None,
        }
    }

    pub fn position(&self) -> NativePosition {
        self.position
    }

    fn status(&self) -> NativeParserStatus {
        if self.termination.is_some() {
            NativeParserStatus::Terminated
        } else if self.recovery.is_some() {
            NativeParserStatus::Recovering
        } else if self.pending.is_some() {
            NativeParserStatus::ReadingCombat
        } else if self.current.is_some() {
            NativeParserStatus::ReadingRun
        } else {
            NativeParserStatus::AwaitingRun
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<NativeFeed, NativeParserError> {
        if self.termination.is_some() {
            return Err(NativeParserError::GenerationTerminated);
        }
        if bytes.len() > MAX_NATIVE_FEED_BYTES {
            return Err(NativeParserError::FeedTooLarge);
        }
        // Reserve all potential counter additions before changing parser state.
        for value in [
            self.position.accepted_bytes,
            self.position.record_count,
            self.run_count,
            self.combat_ordinal,
            self.incomplete_combat_count,
            self.incomplete_run_count,
        ] {
            value
                .checked_add(bytes.len() as u64)
                .ok_or(NativeParserError::PositionOverflow)?;
        }
        let mut output = NativeFeed {
            accepted: 0,
            completed: None,
            position: self.position,
            diagnostics: Vec::new(),
            status: self.status(),
        };
        let mut work = ContextWork::default();
        while output.accepted < bytes.len() {
            let rest = &bytes[output.accepted..];
            let newline = rest.iter().position(|byte| *byte == b'\n');
            let take = newline.map_or(rest.len(), |at| at + 1);
            let content = &rest[..take - usize::from(newline.is_some())];
            let start = self.position.complete_bytes;
            self.position.accepted_bytes += take as u64;
            output.accepted += take;
            if self.discard_start.is_some() {
                if newline.is_some() {
                    self.position.record_count += 1;
                    self.position.complete_bytes = self.position.accepted_bytes;
                    self.discard_start = None;
                    self.advance_recovery();
                    push_diagnostic(&mut output.diagnostics, self.recovery.as_ref().unwrap());
                }
                continue;
            }
            // One extra CR is permitted only as a split CRLF terminator.
            let combined = self.tail.len() + content.len();
            if combined > MAX_NATIVE_LINE_BYTES + 1
                || (combined > MAX_NATIVE_LINE_BYTES
                    && content.last().or_else(|| self.tail.last()) != Some(&b'\r'))
            {
                self.tail.clear();
                self.enter_recovery(NativeDiagnosticCode::LineLimit, start);
                if newline.is_some() {
                    self.position.record_count += 1;
                    self.position.complete_bytes = self.position.accepted_bytes;
                    self.advance_recovery();
                    push_diagnostic(&mut output.diagnostics, self.recovery.as_ref().unwrap());
                } else {
                    self.discard_start = Some(start);
                }
                continue;
            }
            self.tail.extend_from_slice(content);
            if newline.is_none() {
                break;
            }
            self.position.record_count += 1;
            self.position.complete_bytes = self.position.accepted_bytes;
            let mut raw = std::mem::take(&mut self.tail);
            if raw.last() == Some(&b'\r') {
                raw.pop();
            }
            let bom = start == 0 && raw.starts_with(&[0xef, 0xbb, 0xbf]);
            let raw = &raw[usize::from(bom) * 3..];
            let text = match std::str::from_utf8(raw) {
                Ok(text) => text,
                Err(_) => {
                    self.enter_recovery(NativeDiagnosticCode::MalformedEncoding, start);
                    self.advance_recovery();
                    push_diagnostic(&mut output.diagnostics, self.recovery.as_ref().unwrap());
                    continue;
                }
            };
            let record = NativeRecord {
                ordinal: self.position.record_count,
                byte_offset: start + u64::from(bom) * 3,
                line: text.to_owned(),
            };
            match self.consume_record(record, &mut work) {
                Ok(completed) => {
                    if self.recovery.is_none() {
                        self.last_usable_position = self.position;
                    }
                    output.completed = completed;
                }
                Err(code) => self.enter_recovery(code, start),
            }
            if self.recovery.is_some() {
                self.advance_recovery();
                push_diagnostic(&mut output.diagnostics, self.recovery.as_ref().unwrap());
            }
            if output.completed.is_some() {
                break;
            }
        }
        output.position = self.position;
        output.status = self.status();
        Ok(output)
    }

    fn enter_recovery(&mut self, code: NativeDiagnosticCode, start: u64) {
        if self.recovery.is_none() {
            let incomplete = u64::from(self.pending.is_some());
            self.incomplete_combat_count += incomplete;
            self.incomplete_run_count += u64::from(self.current.is_some());
            self.recovery = Some(NativeDiagnostic {
                code,
                start_offset: start,
                end_offset: self.position.accepted_bytes,
                last_usable_position: self.last_usable_position,
                affected_record_count: 0,
                incomplete_evidence_count: incomplete,
            });
        }
        self.current = None;
        self.pending = None;
        self.context = Context::default();
        self.combat_ordinal = 0;
    }

    fn advance_recovery(&mut self) {
        if let Some(diagnostic) = &mut self.recovery {
            diagnostic.end_offset = self.position.accepted_bytes;
            diagnostic.affected_record_count += 1;
        }
    }

    fn consume_record(
        &mut self,
        record: NativeRecord,
        work: &mut ContextWork,
    ) -> Result<Option<NativeCompletedFight>, NativeDiagnosticCode> {
        let decoded = grammar::decode(&record.line, NATIVE_INCREMENTAL_SCHEMA_VERSION)
            .map_err(|_| NativeDiagnosticCode::MalformedKnownRecord)?;
        let values = decoded.values();
        if self.recovery.is_some() && values.get(1) != Some(&"BEGIN_LOG") {
            return Ok(None);
        }
        let known = validate_layout_for(&values, NATIVE_INCREMENTAL_SCHEMA_VERSION)
            .map_err(|error| layout_diagnostic(&values, error))?;
        let time = uint(values[0]).map_err(|_| NativeDiagnosticCode::MalformedKnownRecord)?;
        match values[1] {
            "BEGIN_LOG" => {
                self.incomplete_combat_count += u64::from(self.pending.take().is_some());
                self.incomplete_run_count += u64::from(self.current.is_some());
                let header = NativeHeader {
                    epoch_ms: uint(values[2]).unwrap(),
                    log_version: 15,
                    realm: quoted_value(values[4]).unwrap(),
                    language: quoted_value(values[5]).unwrap(),
                    game_version: quoted_value(values[6]).unwrap(),
                    record: record.clone(),
                };
                self.run_count += 1;
                self.current = Some(NativeRun {
                    schema_version: NATIVE_INCREMENTAL_SCHEMA_VERSION,
                    session_id: run_id_for(
                        &header,
                        self.run_count,
                        self.channel,
                        NATIVE_INCREMENTAL_SCHEMA_VERSION,
                    )
                    .map_err(|_| NativeDiagnosticCode::MalformedKnownRecord)?,
                    run_ordinal: self.run_count,
                    channel: self.channel,
                    header,
                });
                self.context = Context::default();
                self.combat_ordinal = 0;
                self.recovery = None;
            }
            "BEGIN_COMBAT" => {
                if self.current.is_none() || self.pending.is_some() {
                    return Err(NativeDiagnosticCode::InvalidBoundary);
                }
                work.charge(
                    self.context.metadata.len() + self.context.unknown.len(),
                    self.context.bytes,
                )
                .map_err(|_| NativeDiagnosticCode::ContextLimit)?;
                self.combat_ordinal += 1;
                let mut pending = Pending {
                    ordinal: self.combat_ordinal,
                    start_ms: time,
                    first: record.ordinal,
                    records: Vec::new(),
                    context: self.context.snapshot(),
                    units: BTreeSet::new(),
                    abilities: BTreeSet::new(),
                    bytes: 0,
                    unknown: 0,
                    previous_ms: time,
                    reversals: 0,
                };
                pending
                    .add(record.clone(), &values, known)
                    .map_err(|_| NativeDiagnosticCode::FightLimit)?;
                self.pending = Some(pending);
            }
            "END_COMBAT" => {
                let mut pending = self
                    .pending
                    .take()
                    .ok_or(NativeDiagnosticCode::InvalidBoundary)?;
                let completion = (|| {
                    pending
                        .add(record.clone(), &values, known)
                        .map_err(|_| NativeDiagnosticCode::FightLimit)?;
                    let run = self
                        .current
                        .clone()
                        .ok_or(NativeDiagnosticCode::InvalidBoundary)?;
                    let encounter = pending
                        .finish(run, time)
                        .map_err(|_| NativeDiagnosticCode::InvalidBoundary)?;
                    canonical_native_bytes(&encounter)
                        .map_err(|_| NativeDiagnosticCode::FightLimit)?;
                    let logical_identity = logical_native_identity(&encounter)
                        .map_err(|_| NativeDiagnosticCode::MalformedKnownRecord)?;
                    Ok(NativeCompletedFight {
                        encounter,
                        logical_identity,
                    })
                })();
                match completion {
                    Ok(completed) => return Ok(Some(completed)),
                    Err(code) => {
                        // Final validation consumed the pending evidence.
                        self.incomplete_combat_count += 1;
                        self.enter_recovery(code, record.byte_offset);
                        self.recovery.as_mut().unwrap().incomplete_evidence_count += 1;
                    }
                }
            }
            "END_LOG" => {
                if self.current.take().is_none() {
                    return Err(NativeDiagnosticCode::InvalidBoundary);
                }
                self.incomplete_combat_count += u64::from(self.pending.take().is_some());
                self.context = Context::default();
                self.combat_ordinal = 0;
            }
            _ => {
                if self.current.is_none() {
                    return Err(NativeDiagnosticCode::InvalidBoundary);
                }
                if let Some(pending) = &mut self.pending {
                    pending
                        .add(record.clone(), &values, known)
                        .map_err(|_| NativeDiagnosticCode::FightLimit)?;
                }
                self.context
                    .observe(&record, &values, known)
                    .map_err(|_| NativeDiagnosticCode::ContextLimit)?;
                if self.context.bytes > MAX_NATIVE_CONTEXT_BYTES {
                    return Err(NativeDiagnosticCode::ContextLimit);
                }
            }
        }
        Ok(None)
    }

    pub fn checkpoint(&self) -> Result<NativeCheckpoint, NativeParserError> {
        let mut state = CheckpointState::from_parser(self);
        let mut writer = CheckpointWriter(Vec::new());
        serde_json::to_writer(&mut writer, &state)
            .map_err(|_| NativeParserError::CheckpointTooLarge)?;
        state.integrity_sha256 = super::super::sha256(&writer.0);
        writer.0.clear();
        serde_json::to_writer(&mut writer, &state)
            .map_err(|_| NativeParserError::CheckpointTooLarge)?;
        Ok(NativeCheckpoint {
            bytes: writer.0,
            channel: self.channel,
            position: self.position,
        })
    }

    pub fn restore(
        checkpoint: &NativeCheckpoint,
        expected_channel: Channel,
    ) -> Result<Self, NativeParserError> {
        if checkpoint.channel != expected_channel {
            return Err(NativeParserError::ChannelMismatch);
        }
        let parser = decode_checkpoint(checkpoint.as_bytes())?;
        if parser.channel != expected_channel {
            return Err(NativeParserError::ChannelMismatch);
        }
        Ok(parser)
    }

    pub fn terminate_generation(
        &mut self,
        reason: NativeTerminationReason,
    ) -> Result<NativeTermination, NativeParserError> {
        if let Some(receipt) = &self.termination {
            return Ok(receipt.clone());
        }
        let receipt = NativeTermination {
            reason,
            position: self.position,
            incomplete_combat_count: self
                .incomplete_combat_count
                .checked_add(u64::from(self.pending.is_some()))
                .ok_or(NativeParserError::PositionOverflow)?,
            incomplete_run_count: self
                .incomplete_run_count
                .checked_add(u64::from(self.current.is_some()))
                .ok_or(NativeParserError::PositionOverflow)?,
            unterminated_line_count: u64::from(
                !self.tail.is_empty() || self.discard_start.is_some(),
            ),
        };
        self.termination = Some(receipt.clone());
        Ok(receipt)
    }
}

fn push_diagnostic(diagnostics: &mut Vec<NativeDiagnostic>, next: &NativeDiagnostic) {
    if let Some(previous) = diagnostics
        .iter_mut()
        .find(|previous| previous.code == next.code && previous.start_offset == next.start_offset)
    {
        *previous = next.clone();
    } else if diagnostics.len() < MAX_DIAGNOSTICS {
        diagnostics.push(next.clone());
    }
}

fn layout_diagnostic(values: &[&str], error: EncounterError) -> NativeDiagnosticCode {
    if matches!(error, EncounterError::UnsupportedVersion) {
        return NativeDiagnosticCode::UnsupportedLogVersion;
    }
    if values.iter().skip(2).any(|value| {
        !value.starts_with('"') && (value.contains('=') || value.starts_with("timeSinceEpochMS:"))
    }) {
        return NativeDiagnosticCode::UnsupportedSerialization;
    }
    NativeDiagnosticCode::MalformedKnownRecord
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointState {
    version: u32,
    integrity_sha256: String,
    channel: Channel,
    position: NativePosition,
    run_count: u64,
    combat_ordinal: u64,
    current: Option<NativeRun>,
    #[serde(deserialize_with = "deserialize_context")]
    context: Vec<NativeRecord>,
    pending: Option<PendingState>,
    #[serde(deserialize_with = "deserialize_tail")]
    tail: Vec<u8>,
    discard_start: Option<u64>,
    recovery: Option<NativeDiagnostic>,
    last_usable_position: NativePosition,
    incomplete_combat_count: u64,
    incomplete_run_count: u64,
    termination: Option<NativeTermination>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PendingState {
    ordinal: u64,
    start_ms: u64,
    first: u64,
    #[serde(deserialize_with = "deserialize_fight_records")]
    records: Vec<NativeRecord>,
    #[serde(deserialize_with = "deserialize_context")]
    context: Vec<NativeRecord>,
}

fn context_records(context: &[Arc<NativeRecord>]) -> Vec<NativeRecord> {
    let mut result: Vec<_> = context.iter().map(|record| (**record).clone()).collect();
    result.sort_by_key(|record| record.ordinal);
    result
}

impl CheckpointState {
    fn from_parser(parser: &IncrementalNativeParser) -> Self {
        Self {
            version: NATIVE_CHECKPOINT_VERSION,
            integrity_sha256: String::new(),
            channel: parser.channel,
            position: parser.position,
            run_count: parser.run_count,
            combat_ordinal: parser.combat_ordinal,
            current: parser.current.clone(),
            context: context_records(&parser.context.snapshot()),
            pending: parser.pending.as_ref().map(|pending| PendingState {
                ordinal: pending.ordinal,
                start_ms: pending.start_ms,
                first: pending.first,
                records: pending.records.clone(),
                context: context_records(&pending.context),
            }),
            tail: parser.tail.clone(),
            discard_start: parser.discard_start,
            recovery: parser.recovery.clone(),
            last_usable_position: parser.last_usable_position,
            incomplete_combat_count: parser.incomplete_combat_count,
            incomplete_run_count: parser.incomplete_run_count,
            termination: parser.termination.clone(),
        }
    }
}

struct CheckpointWriter(Vec<u8>);
impl Write for CheckpointWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > MAX_NATIVE_CHECKPOINT_BYTES.saturating_sub(self.0.len()) {
            return Err(std::io::Error::other("checkpoint byte limit"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn decode_checkpoint(bytes: &[u8]) -> Result<IncrementalNativeParser, NativeParserError> {
    if bytes.len() > MAX_NATIVE_CHECKPOINT_BYTES {
        return Err(NativeParserError::CheckpointTooLarge);
    }
    checkpoint_shape_budget(bytes)?;
    let mut state: CheckpointState =
        serde_json::from_slice(bytes).map_err(|_| NativeParserError::InvalidCheckpoint)?;
    if state.version != NATIVE_CHECKPOINT_VERSION {
        return Err(NativeParserError::UnsupportedCheckpoint);
    }
    let expected = std::mem::take(&mut state.integrity_sha256);
    if expected.len() != 64
        || !expected
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(NativeParserError::InvalidCheckpoint);
    }
    let mut writer = CheckpointWriter(Vec::new());
    serde_json::to_writer(&mut writer, &state)
        .map_err(|_| NativeParserError::CheckpointTooLarge)?;
    if super::super::sha256(&writer.0) != expected {
        return Err(NativeParserError::InvalidCheckpoint);
    }
    restore_state(state).map_err(|_| NativeParserError::InvalidCheckpoint)
}

// Scan resource bounds without allocating decoded strings or trusting lengths
// advertised by JSON. Serde subsequently validates actual JSON/UTF-8 syntax.
fn checkpoint_shape_budget(bytes: &[u8]) -> Result<(), NativeParserError> {
    let mut index = 0;
    let mut depth = 0_usize;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => {
                index += 1;
                let mut decoded_upper_bound = 0;
                loop {
                    let byte = *bytes
                        .get(index)
                        .ok_or(NativeParserError::InvalidCheckpoint)?;
                    if byte == b'"' {
                        break;
                    }
                    if byte == b'\\' {
                        index += 1;
                        let escape = *bytes
                            .get(index)
                            .ok_or(NativeParserError::InvalidCheckpoint)?;
                        if escape == b'u' {
                            index += 4;
                            decoded_upper_bound += 4;
                        } else {
                            decoded_upper_bound += 1;
                        }
                    } else {
                        decoded_upper_bound += 1;
                    }
                    if decoded_upper_bound > MAX_NATIVE_LINE_BYTES {
                        return Err(NativeParserError::InvalidCheckpoint);
                    }
                    index += 1;
                }
            }
            b'{' | b'[' => {
                depth += 1;
                if depth > 16 {
                    return Err(NativeParserError::InvalidCheckpoint);
                }
            }
            b'}' | b']' => {
                depth = depth
                    .checked_sub(1)
                    .ok_or(NativeParserError::InvalidCheckpoint)?;
            }
            b'-' | b'0'..=b'9' => {
                let start = index;
                while bytes.get(index).is_some_and(|byte| {
                    matches!(byte, b'0'..=b'9' | b'+' | b'-' | b'.' | b'e' | b'E')
                }) {
                    index += 1;
                    if index - start > 128 {
                        return Err(NativeParserError::InvalidCheckpoint);
                    }
                }
                continue;
            }
            _ => {}
        }
        index += 1;
    }
    if depth != 0 {
        return Err(NativeParserError::InvalidCheckpoint);
    }
    Ok(())
}

fn deserialize_context<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<NativeRecord>, D::Error> {
    deserialize_records(deserializer, MAX_NATIVE_METADATA, MAX_NATIVE_CONTEXT_BYTES)
}

fn deserialize_fight_records<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<NativeRecord>, D::Error> {
    deserialize_records(
        deserializer,
        MAX_NATIVE_COMBAT_RECORDS,
        MAX_NATIVE_COMBAT_BYTES,
    )
}

fn deserialize_records<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
    max_records: usize,
    max_bytes: usize,
) -> Result<Vec<NativeRecord>, D::Error> {
    struct RecordsVisitor {
        max_records: usize,
        max_bytes: usize,
    }
    impl<'de> serde::de::Visitor<'de> for RecordsVisitor {
        type Value = Vec<NativeRecord>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("bounded native records")
        }
        fn visit_seq<A: serde::de::SeqAccess<'de>>(
            self,
            mut sequence: A,
        ) -> Result<Self::Value, A::Error> {
            let mut records = Vec::new();
            let mut bytes = 0_usize;
            while let Some(record) = sequence.next_element::<NativeRecord>()? {
                if records.len() >= self.max_records
                    || record.line.len() > self.max_bytes.saturating_sub(bytes)
                {
                    return Err(serde::de::Error::custom("native checkpoint record limit"));
                }
                bytes += record.line.len();
                records.push(record);
            }
            Ok(records)
        }
    }
    deserializer.deserialize_seq(RecordsVisitor {
        max_records,
        max_bytes,
    })
}

fn deserialize_tail<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<u8>, D::Error> {
    struct TailVisitor;
    impl<'de> serde::de::Visitor<'de> for TailVisitor {
        type Value = Vec<u8>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("bounded native tail")
        }
        fn visit_seq<A: serde::de::SeqAccess<'de>>(
            self,
            mut sequence: A,
        ) -> Result<Self::Value, A::Error> {
            let mut tail = Vec::new();
            while let Some(byte) = sequence.next_element::<u8>()? {
                if tail.len() > MAX_NATIVE_LINE_BYTES {
                    return Err(serde::de::Error::custom("native checkpoint tail limit"));
                }
                tail.push(byte);
            }
            Ok(tail)
        }
    }
    deserializer.deserialize_seq(TailVisitor)
}

fn restore_state(state: CheckpointState) -> Result<IncrementalNativeParser, EncounterError> {
    let invalid = || EncounterError::Validation("native checkpoint state is inconsistent".into());
    let position = state.position;
    if position.complete_bytes > position.accepted_bytes
        || state.last_usable_position.accepted_bytes > position.complete_bytes
        || state.last_usable_position.complete_bytes != state.last_usable_position.accepted_bytes
        || state.last_usable_position.record_count > position.record_count
        || state.run_count > position.record_count
        || state.combat_ordinal > position.record_count
        || state.incomplete_combat_count > position.record_count
        || state.incomplete_run_count > position.record_count
        || state.tail.len() > MAX_NATIVE_LINE_BYTES + 1
        || state.tail.contains(&b'\n')
    {
        return Err(invalid());
    }
    if let Some(start) = state.discard_start {
        if !state.tail.is_empty()
            || state.recovery.is_none()
            || start != position.complete_bytes
            || position.accepted_bytes - start <= MAX_NATIVE_LINE_BYTES as u64
        {
            return Err(invalid());
        }
    } else if position.accepted_bytes - position.complete_bytes != state.tail.len() as u64 {
        return Err(invalid());
    }
    if let Some(recovery) = &state.recovery {
        if state.current.is_some()
            || state.pending.is_some()
            || !state.context.is_empty()
            || state.combat_ordinal != 0
            || recovery.start_offset > recovery.end_offset
            || recovery.end_offset > position.accepted_bytes
            || recovery.last_usable_position != state.last_usable_position
            || recovery.affected_record_count > position.record_count
            || recovery.incomplete_evidence_count > state.incomplete_combat_count
        {
            return Err(invalid());
        }
    }
    let Some(run) = state.current.as_ref() else {
        if !state.context.is_empty() || state.pending.is_some() || state.combat_ordinal != 0 {
            return Err(invalid());
        }
        return finish_restore(state, Context::default(), None);
    };
    validate_native_run(run)?;
    if run.schema_version != NATIVE_INCREMENTAL_SCHEMA_VERSION
        || run.channel != state.channel
        || run.run_ordinal != state.run_count
        || run.header.record.ordinal > position.record_count
        || run.header.record.byte_offset >= position.complete_bytes
    {
        return Err(invalid());
    }
    let context = restore_context(
        &state.context,
        run,
        position.record_count,
        position.complete_bytes,
    )?;
    let pending = if let Some(pending) = &state.pending {
        if pending.records.is_empty()
            || pending.records.len() > MAX_NATIVE_COMBAT_RECORDS
            || pending.ordinal != state.combat_ordinal
            || pending.ordinal == 0
            || pending.first <= run.header.record.ordinal
            || pending.records.last().unwrap().ordinal != position.record_count
        {
            return Err(invalid());
        }
        let mut evolving = restore_context(
            &pending.context,
            run,
            pending.first - 1,
            position.complete_bytes,
        )?;
        let mut rebuilt = Pending {
            ordinal: pending.ordinal,
            start_ms: pending.start_ms,
            first: pending.first,
            records: Vec::new(),
            context: evolving.snapshot(),
            units: BTreeSet::new(),
            abilities: BTreeSet::new(),
            bytes: 0,
            unknown: 0,
            previous_ms: pending.start_ms,
            reversals: 0,
        };
        let mut previous: Option<&NativeRecord> = None;
        for (index, record) in pending.records.iter().enumerate() {
            validate_native_record(record, NATIVE_INCREMENTAL_SCHEMA_VERSION)?;
            let decoded = grammar::decode(&record.line, NATIVE_INCREMENTAL_SCHEMA_VERSION)?;
            let values = decoded.values();
            let known = validate_layout_for(&values, NATIVE_INCREMENTAL_SCHEMA_VERSION)?;
            if Some(record.ordinal) != pending.first.checked_add(index as u64)
                || record.byte_offset < run.header.record.byte_offset
                || record.byte_offset >= position.complete_bytes
                || previous.is_some_and(|prior| {
                    let distance = record.byte_offset.checked_sub(prior.byte_offset);
                    distance != Some(prior.line.len() as u64 + 1)
                        && distance != Some(prior.line.len() as u64 + 2)
                })
            {
                return Err(invalid());
            }
            if index == 0 {
                if values[1] != "BEGIN_COMBAT" || uint(values[0])? != pending.start_ms {
                    return Err(invalid());
                }
            } else if matches!(
                values[1],
                "BEGIN_COMBAT" | "END_COMBAT" | "BEGIN_LOG" | "END_LOG"
            ) {
                return Err(invalid());
            }
            rebuilt.add(record.clone(), &values, known)?;
            evolving.observe(record, &values, known)?;
            previous = Some(record);
        }
        let last = pending.records.last().unwrap();
        let end_distance = position.complete_bytes - last.byte_offset;
        if (end_distance != last.line.len() as u64 + 1
            && end_distance != last.line.len() as u64 + 2)
            || context_records(&evolving.snapshot()) != state.context
        {
            return Err(invalid());
        }
        Some(rebuilt)
    } else {
        None
    };
    finish_restore(state, context, pending)
}

fn restore_context(
    records: &[NativeRecord],
    run: &NativeRun,
    last_ordinal: u64,
    complete_bytes: u64,
) -> Result<Context, EncounterError> {
    if records.len() > MAX_NATIVE_METADATA {
        return rejected();
    }
    let mut context = Context::default();
    let mut prior = run.header.record.ordinal;
    for record in records {
        validate_native_record(record, NATIVE_INCREMENTAL_SCHEMA_VERSION)?;
        let decoded = grammar::decode(&record.line, NATIVE_INCREMENTAL_SCHEMA_VERSION)?;
        let values = decoded.values();
        let known = validate_layout_for(&values, NATIVE_INCREMENTAL_SCHEMA_VERSION)?;
        if record.ordinal <= prior
            || record.ordinal > last_ordinal
            || record.byte_offset <= run.header.record.byte_offset
            || record.byte_offset >= complete_bytes
            || (known
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
                ))
        {
            return rejected();
        }
        prior = record.ordinal;
        context.observe(record, &values, known)?;
        if context.bytes > MAX_NATIVE_CONTEXT_BYTES {
            return Err(EncounterError::NativeWorkLimit);
        }
    }
    if context_records(&context.snapshot()) != records {
        return rejected();
    }
    Ok(context)
}

fn finish_restore(
    state: CheckpointState,
    context: Context,
    pending: Option<Pending>,
) -> Result<IncrementalNativeParser, EncounterError> {
    let mut parser = IncrementalNativeParser {
        channel: state.channel,
        position: state.position,
        run_count: state.run_count,
        combat_ordinal: state.combat_ordinal,
        current: state.current,
        context,
        pending,
        tail: state.tail,
        discard_start: state.discard_start,
        recovery: state.recovery,
        last_usable_position: state.last_usable_position,
        incomplete_combat_count: state.incomplete_combat_count,
        incomplete_run_count: state.incomplete_run_count,
        termination: None,
    };
    if let Some(expected) = state.termination {
        let actual = parser.terminate_generation(expected.reason).map_err(|_| {
            EncounterError::Validation("native termination state is inconsistent".into())
        })?;
        if actual != expected {
            return rejected();
        }
    }
    Ok(parser)
}
