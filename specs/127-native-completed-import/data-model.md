# S127 Data Model

## NativeRecord

Original UTF-8 line without line terminator, source ordinal and byte offset.
Time and type are validated from the line; quoted commas and nested groups stay exact.
Raw record Debug output is metadata-only. Record/tokens/depth/bytes are bounded.

## NativeHeader / NativeRun

Native canonical schema1 and normal version15 profile; actual epoch milliseconds,
realm/language/game strings and original header line, run ordinal, selected channel
with user-selection provenance. Stable native-session namespace ID is a hash of
the canonical header identity and run ordinal/channel. API/addon version fields do
not exist. Context resets at each BEGIN_LOG; source order never comes from sorting time.

## NativeEncounter

Native encounter namespace ID derived from run identity and combat ordinal.
Paired source ordinals and elapsed milliseconds, exact ordered combat records,
relevant pre-combat metadata context, unknown/clock-discontinuity facts and channel.
Original context lines are ordered before BEGIN_COMBAT and immutable. Unit declarations
and ability/effect/player context are scoped to their run and applied in source order.
Canonical bytes and SHA256 are source-specific and separate from whole-file SHA256.

## NativeSourceReport / NativeImportReport

Read-only source identity/hash/mtime, controlled supported profile, run and completed
combat counts, total/known/unknown/outside-combat records, unfinished runs/combats and
unterminated-tail count. Import adds new/already-present counts and receipts with
stable native identities/content hashes. No original payload strings appear.

## NativeProjection

Own algorithm identifier, source provenance, paired duration and observed native
record/cast/effect/quantity facts, actor-scope information and capability reasons.
Missing API/addon/interval/replay facts stay absent. Quantities and overflow remain
separate; unavailable calculations are not zero or fabricated addon projections.

## EncounterSummary / EncounterDetail

Common summary adds source_kind, optional addon capture_mode and unknown-record count.
Native source has no invented capture mode. Native status describes completed combat
boundaries; quality/capability limitations remain visible. Detail uses an explicit
addon/native variant, keeping addon projections/recommendations unchanged.

## SQLite schema6

Existing raw_encounters and encounter_session_snapshots remain byte-compatible.
Native runs store immutable header identity/canonical metadata; native_encounters store
immutable source-aware canonical evidence, source/content hashes, native identities,
ordinal/channel/times/counts. Both have no-update triggers. Shared encounter_session_order
retains first-import order even after deletion. Exact schema/row/canonical validation
includes native tables; schema1-5 reads remain supported. Migration, collision checks,
ordering and all inserts share one immediate transaction.

## Lifecycle

Unselected -> explicit native path/environment -> worker inspection -> ready/no-completed/error.
Import rereads a stable snapshot, parses/validates/preflights completely, commits
atomically and refreshes mixed history. Changing source key invalidates old inspection
and its pending presentation. Failed import leaves source/store unchanged.
Deletion removes selected history only; reimport preserves retained session ordering.
