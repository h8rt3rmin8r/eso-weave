# Data Model: Deterministic Encounter Replay

## NormalizationProfile

- `version`: replay algorithm profile version, exactly 1 for S095.
- `api_version`: must equal the capture envelope API version.
- `player_combat_unit_type`: finite integer runtime constant.
- `health_power_type`: finite integer runtime constant.
- `quickslot_category`: finite integer runtime constant.
- `damage_results`: exactly six unique integer result constants.
- `healing_results`: exactly four unique integer result constants.
- `death_results`: exactly two unique integer result constants.
- `resurrect_result`: one integer result constant.

All result constants are unique across classifications.

Current addon-version-3 schema-v2 captures require this entity. Older captures
omit it and use explicit legacy dispatch.

## ReplayAssessment

- `Verified`: exact replay and projection comparison succeeded.
- `Divergent`: exact comparison identifies a mismatch internally; import fails
  with one value-free category message.
- `Indeterminate`: declared loss, discontinuity, or incomplete correlation
  prevents a claim; partial import may continue under degraded evidence rules.
- `Unavailable`: schema v1 or pre-profile schema v2 has no replay contract.

No assessment contains raw or normalized payload values.

## Replay State

- Actor interner, bounded to the existing actor ceiling.
- Boss roster signature and existence/power batch state.
- Action-slot and quickslot API correlation state.
- Performance API correlation state.
- Per-source projection ordinal counters.
- Output event vector, bounded to the existing event ceiling.

State exists only for one replay invocation and is never persisted.

## Validation and Import Flow

1. Restricted parsing and structural envelope validation.
2. Profile validation for current captures.
3. Pure replay or explicit legacy/partial assessment.
4. Exact comparison for complete current captures.
5. Canonicalization, hash calculation, and immutable storage only after success.

## Invariants

- Replay never reads compatibility events while deriving projections.
- A complete current capture cannot be stored unless verified.
- Declared raw loss can never be reported as verified or divergent.
- Legacy canonical blobs and content hashes are not rewritten.
- Unknown extra raw scalar values remain preserved.
- Diagnostic cardinality is constant and value-free.
