# Incremental native parser contract

Date: 2026-10-15. S130 supports normal/separate native log version 15. The owner
deferred normal/inline to #296 and verbose combinations to #297. This contract
does not change game settings or claim those deferred serializations work.

## Pure public interface

Expose these types through `eso_weave::encounter`. The implementation may use
private modules. Names and semantics below are the integration contract.

```rust
pub const NATIVE_INCREMENTAL_SCHEMA_VERSION: u32 = 2;
pub const NATIVE_CHECKPOINT_VERSION: u32 = 1;
pub const MAX_NATIVE_FEED_BYTES: usize = 256 * 1024;
pub const MAX_NATIVE_CHECKPOINT_BYTES: usize = 128 * 1024 * 1024;

pub struct IncrementalNativeParser { /* private validated state */ }

impl IncrementalNativeParser {
    pub fn new(channel: Channel) -> Self;
    pub fn feed(&mut self, bytes: &[u8]) -> Result<NativeFeed, NativeParserError>;
    pub fn position(&self) -> NativePosition;
    pub fn checkpoint(&self) -> Result<NativeCheckpoint, NativeParserError>;
    pub fn restore(
        checkpoint: &NativeCheckpoint,
        expected_channel: Channel,
    ) -> Result<Self, NativeParserError>;
    pub fn terminate_generation(
        &mut self,
        reason: NativeTerminationReason,
    ) -> Result<NativeTermination, NativeParserError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativePosition {
    pub accepted_bytes: u64,
    pub complete_bytes: u64,
    pub record_count: u64,
}

pub struct NativeFeed {
    pub accepted: usize,
    pub completed: Option<NativeCompletedFight>,
    pub position: NativePosition,
    pub diagnostics: Vec<NativeDiagnostic>,
    pub status: NativeParserStatus,
}

pub struct NativeCompletedFight {
    pub encounter: NativeEncounter,
    pub logical_identity: NativeLogicalIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeLogicalIdentity {
    pub run_key: String,
    pub fight_key: String,
    pub evidence_sha256: String,
}

pub fn logical_native_identity_v1(
    encounter: &NativeEncounter,
) -> Result<NativeLogicalIdentity, EncounterError>;

pub fn logical_native_identity(
    encounter: &NativeEncounter,
) -> Result<NativeLogicalIdentity, EncounterError>;

pub struct NativeCheckpoint { /* private encoded bytes */ }
impl NativeCheckpoint {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, NativeParserError>;
    pub fn as_bytes(&self) -> &[u8];
    pub fn channel(&self) -> Channel;
    pub fn position(&self) -> NativePosition;
}
```

`NativeParserError` is a controlled enum with no raw source strings, paths or
identifiers. Required variants are `FeedTooLarge`, `PositionOverflow`,
`InvalidCheckpoint`, `UnsupportedCheckpoint`, `CheckpointTooLarge`,
`ChannelMismatch` and `GenerationTerminated`. It implements Error and Debug with
controlled descriptions only. FeedTooLarge and GenerationTerminated do not mutate
state. Raw parser/checkpoint values use metadata-only Debug, as NativeRecord does.

`NativeParserStatus` has `AwaitingRun`, `ReadingRun`, `ReadingCombat`,
`Recovering` and `Terminated`. The checkpoint owns exact underlying state;
the status is an observation, not authority to reconstruct missing context.

## Feed, progress and completion

An offered feed must be at most 256 KiB. It consumes a prefix and returns its
length in `accepted`; the caller resubmits the unaccepted suffix. Stop immediately
after the first validated END_COMBAT and return at most one completed fight.
This prevents a short feed containing many tiny fights from cloning many large
metadata snapshots into a single output. A caller can loop within its independent
turn budget, committing outputs and yielding between turns.

For a nonempty accepted feed, `accepted_bytes` advances by exactly `accepted`.
It includes retained incomplete tail bytes. `complete_bytes` is the offset just
after the last processed LF-terminated record and never exceeds accepted_bytes.
Their difference is the exact unconsumed tail, except bounded overlong-line
discard recovery, where it describes the skipped pending byte span. `record_count` counts consumed
terminated records, including controlled rejected records during recovery.
All additions and differences are checked u64 arithmetic. Original NativeRecord
coordinates are physical source-generation coordinates, not feed-relative.

Only decode a complete LF-terminated record. A preceding CR belongs to the
terminator. Split UTF-8, CRLF, quotes, escapes and groups wait in the exact byte
tail until LF arrives. Do not require the incomplete tail to be valid UTF-8.
An initial BOM is accepted only at generation offset zero, retaining S127's
record-offset behavior. Empty feeds are idempotent: no progress, completion,
new diagnostic, interruption or termination.

A valid END_COMBAT immediately emits its fight, even without END_LOG and while
the source is active. Completed output has the same raw ordered lines, relevant
preceding context, quantities and quality as one-chunk incremental parsing.
No END_LOG or END_COMBAT may be synthesized. BEGIN_LOG resets run context and
combat ordinal; independently complete fights in an unfinished run remain valid.
Clock reversal stays explicit; reversed paired endpoints do not create a fight.

## Schema 1 and schema 2 evidence

Reuse existing NativeRecord, NativeHeader, NativeRun and NativeEncounter structs
and existing native projector. `schema_version = 1` retains exact S127 identity,
canonical bytes, coordinate limits, complete-file parsing and validation behavior.
Do not rewrite old rows, headers, IDs, hashes or physical coordinates.

Incremental output uses `schema_version = 2` for its run and encounter. Preserve
original physical u64 ordinals, offsets and physical run_ordinal as provenance.
Remove only the schema1 lifetime MAX_NATIVE_FILE_BYTES, MAX_NATIVE_RECORDS and
MAX_NATIVE_RUNS coordinate ceilings for validated schema2. Keep all per-line,
context, fight and canonical validation limits. All counters are checked u64;
there is no practical lifetime file-age, file-size, record or run count limit.
Schema2 does not bypass raw line validation, boundary validation, exact counts,
ordered context, checked quantities, channel validation or canonical byte bounds.

`parse_native_log`, its existing schema1 import path and native schema1 fixtures
remain compatible. Canonical encoding of an existing schema1 value is byte-for-
byte unchanged. `canonical_native_bytes`, `validate_native_encounter` and
`project_native_encounter` deliberately dispatch validation/identity by schema.
No parallel native model, metrics implementation or native SQL table family is
introduced solely for schema2.

## Logical identity and compatible aliases

Logical identity is independent of a filename, filesystem ID, physical run
ordinal, absolute line ordinal and absolute byte offset. Define run_key as the
SHA-256 of a canonical serde_json tuple:

```text
("eso-weave/native-logical-run/v1", channel,
 epoch_ms, log_version, realm, language, game_version)
```

Define fight_key as SHA-256 of
`("eso-weave/native-logical-fight/v1", run_key, encounter_ordinal)`.
Schema2 session_id is `native-session-v2-` followed by run_key; encounter_id is
`native-encounter-v2-` followed by fight_key. Physical provenance remains in the
canonical capture, but cannot produce a second logical fight by itself.

The evidence fingerprint is the SHA-256 of a deterministic normalized identity
view. Start with a validated capture; normalize its run/encounter schema markers
and IDs to schema2 and replace physical run_ordinal with the fixed value 1 in
the comparison view. Normalize each ordinal as
`record.ordinal - header.record.ordinal + 1` and each byte offset as
`record.byte_offset - header.record.byte_offset` (header ordinal becomes 1 and
offset 0), including first/last combat ordinals. Preserve exact original lines, context order, combat ordinal, times,
channel, unknown counts and quality counters. The normalization format has its
own versioned domain and fixed struct field order. It never changes stored raw
canonical bytes. Both logical helpers use that same comparison view; the `_v1`
helper first requires schema1 and existing strict schema1 validation.

The store's alias index maps logical fight_key and matching evidence_sha256 to
the retained original session_id, encounter_id and canonical hash. A shifted or
copied schema1/schema2 capture returns already-present against that retained row.
The first accepted canonical evidence remains authoritative for its provenance;
later copies do not overwrite its offsets or hashes. Backfill old logical aliases
only within an admitted successful write transaction. A matching logical key
with divergent evidence is an explicit collision, never an already-present
result, replacement or silent merge. Distinct typed headers retain distinct
run keys. Identical header plus identical recorded evidence cannot establish two
different real-world events without further evidence; do not claim otherwise.

## Bounds and controlled recovery

Retain S127 bounds: 64 KiB per raw line, 4096 tokens, bracket depth 8, 100000
metadata entries, 200000 records/32 MiB raw text per pending fight, 8 MiB relevant
starting context and 64 MiB canonical bytes per completed fight. Additionally,
current retained context is bounded to 8 MiB, exact tail to 64 KiB (with one
additional CR byte permitted only while awaiting its LF), checkpoint
encoding to 128 MiB and per-feed diagnostics to 16 aggregated entries. Starting
context and evolving context are independently accounted and bounded. Reserve
work/bytes before snapshot cloning, scanning or encoding. Empty feed does not
recharge retained context. Context/filter work budgets apply per bounded feed,
not cumulatively over the lifetime file. A nonterminal feed cannot retain a
completed fight beyond the single returned output.

`NativeDiagnostic` contains controlled code, affected start/end physical byte
offsets, last usable position, affected-record count and incomplete-evidence
count. Codes distinguish UnsupportedLogVersion, UnsupportedSerialization,
MalformedKnownRecord, MalformedEncoding, InvalidBoundary, LineLimit,
ContextLimit and FightLimit. Unknown valid record types remain retained/countable
evidence and do not enter malformed recovery.

On a malformed/unsupported/over-limit terminated segment, preserve already
completed output and enter Recovering. Mark any affected pending fight incomplete;
do not publish it as complete. Source bytes remain untouched. Recovery consumes
bounded records until a fresh syntactically valid normal/separate v15 BEGIN_LOG
header. Validate that complete header before resetting context; arbitrary later
BEGIN_COMBAT/END_COMBAT cannot become a fight. Diagnostic ranges and counts report
skipped evidence. For an overlong unterminated line, enter bounded discard-until-
LF state rather than retaining unbounded tail; record its exact skipped length
with checked counters. A partial header cannot recover until its terminating LF.

Feed returns controlled diagnostics with the last usable boundary instead of
throwing away valid earlier completions. Structural input errors and bounds are
different from unknown type evidence. A feed rejected before consumption leaves
the original position/checkpoint available for retry. Unexpected allocation or
internal invariant failure must never acknowledge an uncommitted fight.

## Checkpoint restore and source termination

Checkpoint version1 encodes an integrity SHA-256, channel, position, pending tail/discard state, current
run/header, ordinals, metadata/unknown context, pending fight and its starting
context/counters, recovery accounting and termination receipt. Status and cached
references are derived from validated retained evidence instead of serialized
as authoritative caches. The integrity digest is computed over the canonical
checkpoint with its digest field empty. It detects corruption, not source
authenticity; verified caller source-generation binding is still required.
It contains no uncommitted completed output. `from_bytes` bounds decoding before
allocation; `restore` rejects unknown versions/fields, mismatched channel,
invalid bounds/positions, invalid raw retained lines, inconsistent identities,
context order, counters or boundary state before any output is possible. Rebuild
and compare derived indexes/counters rather than trusting serialized caches.

The parser is filesystem-free. The runtime must verify the persisted source ID,
generation witness, revision and checkpoint position before calling restore.
Successful deserialization alone proves no file identity. Never mix an old tail
with replacement bytes. Corrupt/incompatible/mismatched state requires declared
bounded replay from an independently verified boundary, or fresh generation
recovery; it cannot fabricate missing pre-combat metadata.

NativeTerminationReason has RotatedComplete, Replaced and Truncated.
Shutdown uses checkpoint suspension and preserves unfinished state; it does not
call terminate_generation or count an interrupted active fight. The explicit
terminate_generation call is reserved for confirmed RotatedComplete, Replaced or
Truncated generations. NativeTermination records reason, position, incomplete
fight/run/tail counts. It emits no fake completed
fight, retains bounded unfinished evidence in the terminal checkpoint and is
idempotent. Feed after terminal state returns GenerationTerminated. A new
generation uses a new parser. EOF, a quiet period, logging disabled or desktop
shutdown do not prove a source generation ended.

The runtime/store commits completed fights, aliases, deletion suppression and
the corresponding checkpoint under one transaction/revision and commit-admission
boundary. The runtime advances its durable cursor only after commit. A failed
transaction retries from the old checkpoint and rereads the same source prefix;
the parser itself never opens SQLite or controls eligibility, scheduling, UI,
addon presence, filesystem discovery or gameplay input.

## Decisive validation

Every meaningful byte split of the sanitized S127 fixture must match one-chunk
schema2 output, including original lines, coordinates, logical keys/fingerprints
and supported projection. Existing schema1 fixture canonical bytes remain exact.
Checkpoint round trips inside UTF-8 tail, metadata and pending combat must match
uninterrupted parsing. Corrupt checkpoints produce no completion. Copied and
extracted later runs deduplicate through aliases while physical evidence remains
unchanged. Empty input, quiet periods, unsupported/malformed recovery, checked
counter overflow and logical streams past old whole-file ceilings have explicit
tests. No test asserts deferred inline/verbose producer fidelity.
