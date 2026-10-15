# S131 additive decoder and numeric projection assessment

Date: 2026-10-15. Design research only, based on the current working checkout
and the pinned producer disassembly. No product edits were made by this task.

## Current compatibility boundaries

`NativeRecord` stores the exact original line. `canonical_native_bytes` validates
then serializes the existing capture structure. The logical evidence hash also
includes those retained lines. A profile decoder should therefore expose typed
facts beside the raw line, without replacing verbose or inline evidence with a
manufactured positional line.

`validate_native_encounter` recomputes unknown record/context counts using
`validate_layout_for`. Broadening the set of recognized record names globally
would invalidate old schema 1/2 captures whose counters correctly recorded those
names as unknown. This is independent of whether the new decoder understands
their grammar.

Keep a frozen canonical classification for schemas 1 and 2. A record can have a
new typed decoder while its historic canonical counter classification remains
unchanged. If newly ingested records must use a revised canonical known/unknown
classification, that classification needs a new version discriminator; it must
not be imposed retroactively on old rows. Numeric support alone does not require
a raw-row migration or rewritten canonical JSON.

## Signed producer facts

COMBAT_EVENT at `0x10064a544` prints hitValue and overflow with `%d`. BEGIN_CAST
at `0x100649582` prints durationMS with `%d`. Their signed textual values must
be accepted and retained as observed, rather than rejected by the current
unsigned validators.

PowerType also uses `%d`, but this Mac producer zero-extends its byte argument
at `0x10064a67a`. Its demonstrated emitted range is therefore decimal 0..255.
Negative powerType values are not established by that instruction path alone.
The new decoder should at least accept those decimal values; treating the field
as an uppercase symbolic enum would incorrectly reject valid output. Existing
schema 1/2 records also admit older word tokens, which should remain readable.

Keep unsigned validation for actual IDs and sequence coordinates. Do not make
every integer signed merely because these particular fields use signed text.

## Minimal additive numeric strategy

1. Give decoded combat facts signed observed hit/overflow values. A single
   `i128` representation can hold both current producer i32 values and every
   historical u64 value accepted by the legacy reader. Preserve the legacy
   lexical/range path when validating existing schema 1/2 evidence; a strict
   i32 conversion must not retroactively reject an old u64 quantity.
2. Promote the six native quantity totals, including the two optional local
   totals, from u128 to i128 and use checked signed addition. Never use absolute
   value, clamp to zero, cast a negative value to unsigned, or round-trip a
   quantity through f64.
3. Continue counting a valid COMBAT_EVENT before deciding whether its action
   result contributes to reviewed damage/healing categories. A signed amount
   on an unrelated action result must not make projection of the fight fail.
4. Preserve signed BEGIN_CAST durationMS in the decoded fact. That field is not
   `NativeProjection.duration_ms`, which is the elapsed time between the paired
   combat boundaries. Negative cast text must not alter encounter duration or
   prevent cast counting.
5. Keep signed totals as algebraic sums of the observed source fields. Do not
   relabel them as effective health change or invent sentinel semantics.
   The existing local rate formula can remain the literal observed-hit total
   divided by elapsed seconds, subject to its existing coverage/time rules.

This preserves all previously accepted nonnegative totals. The maximum fight
contains 200,000 records; even if every historical value is u64::MAX, their sum
is below 2^82, well inside i128. Positive i128 values serialize as the same JSON
integer tokens as the existing u128 values. The existing regression at
`tests/encounter_native.rs:602` explicitly requires
`18446744073709551616`, so an i64 accumulator or JSON-number narrowing would
regress existing behavior.

The shared typed decoder should provide source unit ID and all semantic field
positions to projection. The current projector's fixed `values[9]` source slot
cannot be reused for inline records after four metadata fields are inserted.

## Derivation versioning and old rows

Native detail is computed from retained canonical evidence by
`EncounterHistoryService::native_detail`. Stored derivations are opaque JSON
plus a verified hash, keyed by their algorithm version; canonical raw rows do
not need rewriting for an additive projection version.

Keeping the existing observed-sum algorithm number is defensible if this is
strictly an input-domain extension and every old result remains identical. If
the implementation chooses a new algorithm version, append its derivations
beside existing ones and update the pending-work selector: current
`src/encounter/store/ingest.rs:755` hardcodes `native-v1`. Otherwise a new-version
derivation will never satisfy that selector, causing repeated processing.

## Focused verification recommendations

- Freeze old schema 1/2 canonical bytes, hashes, unknown counters and positive
  projection JSON, including the existing greater-than-u64 total regression.
- Decode the same ordinary and signed event in each required profile and assert
  identical typed quantities, source attribution and event/cast counts.
- Exercise i32::MIN, -1, zero, i32::MAX and legacy u64::MAX hit/overflow values;
  assert signed sums without clamping or unsigned wrapping.
- Exercise negative hit/overflow on a non-damage/non-heal action result and
  negative cast duration text; valid evidence must still project successfully.
- Exercise numeric powerType, retained legacy tokens, and an old unknown record
  that gains a decoder; canonical counter validation must remain compatible.

These are implementation recommendations. They do not add a positivity
restriction, claim game-run verification, or require migration of old evidence.
