# Data Model: Evidence-Scoped Encounter Recommendations

## RecommendationReport

- `schema_version`: recommendation report schema, initially `1`
- `policy_version`: stable recommendation ruleset, `s090-v1`
- `availability`: `Ready`, `Qualified`, or `Suppressed`
- `evidence`: one `RecommendationEvidence`
- `reasons`: stable ordered `RecommendationReason` values
- `advice`: zero to two ordered `AdviceItem` values

Invariants:

- Equal `EncounterProjection` inputs produce equal reports.
- A suppressed report has no advice.
- A qualified report marks every item qualified.
- Advice order is dominant damage, then low uptime.
- No field authorizes or describes an executable action.

## RecommendationEvidence

- `duration_ms`: projection duration
- `cast_count`: length of ordered observed cast IDs
- `first_sequence`, `last_sequence`: inclusive projection span
- `sequence_span`: checked inclusive span represented without overflow
- `missing_sequences`: union count of exact loss ranges clipped to the span
- `projection_compatible`: supported projection schema and algorithm gate
- `loss_ranges_valid`: every declared range is forward and countable
- `metric_evidence_valid`: algorithm, source range, quality, and loss consistency
- `known_ids`, `unknown_ids`: sorted positive aggregate catalog receipt IDs
- `known_ability_ids`, `unknown_ability_ids`: kind-scoped ability resolution evidence
- `known_effect_ids`, `unknown_effect_ids`: kind-scoped effect resolution evidence
- `unknown_damage_share`: sum of valid damage-share values for unknown abilities
- `loss_ranges`: exact sorted projection loss ranges and reasons
- `citation`: shared `RecommendationCitation`

Facts remain values and identifiers. They contain no recommendation prose.

## RecommendationCitation

- `session_id`, `encounter_id`
- `raw_content_sha256`
- `projection_schema_version`
- `metric_algorithm_version`
- `catalog_schema_version`
- `catalog_version`
- `catalog_semantic_sha256`
- `channel`, `api_version`
- `recommendation_schema_version`
- `recommendation_policy_version`

The citation is copied into every advice item from one immutable projection. It
contains no generation time or filesystem path.

## RecommendationAvailability

- `Ready`: sample, loss, and identity gates pass without qualification
- `Qualified`: advice may be present, but exact sub-material loss or catalog
  uncertainty is visible
- `Suppressed`: global evidence is insufficient; advice must be empty

Availability is separate from provisional confidence. Ready advice remains a
provisional review prompt until a later version has stronger evidence.

## RecommendationReason

Stable ordered kinds:

1. `InsufficientDuration`
2. `InsufficientCasts`
3. `UnsupportedProjectionVersion`
4. `InvalidSequenceSpan`
5. `InvalidLossRange`
6. `InvalidMetricEvidence`
7. `MaterialCaptureLoss`
8. `DeclaredCaptureLoss`
9. `UnknownCatalogIds`
10. `MaterialUnknownDamageShare`
11. `UnknownTargetOmitted`

Each reason carries only the bounded values needed to explain the gate. Global
suppression reasons precede qualification and rule-local omission reasons.

## AdviceItem

- `rule`: `DominantDamageShare` or `LowEffectUptime`
- `target_kind`: `Ability` or `Effect`
- `target_id`: positive numeric catalog identity
- `observed_ratio`: finite value in zero through one
- `qualification`: `Provisional` or `ProvisionalQualified`
- `citation`: complete `RecommendationCitation`

Presentation derives fixed project-authored wording from `rule`, `target_id`, and
`observed_ratio`. No untrusted capture or catalog string enters advice prose.

## Deterministic Selection

### Dominant damage

1. Reject invalid ratios and unknown ability targets.
2. Sort by descending share, then ascending numeric ID.
3. Select at most one value at or above `0.40`.
4. Omit the rule when unknown ability damage share is at or above `0.25`.

### Low effect uptime

1. Reject invalid ratios and unknown effect targets.
2. Sort by ascending uptime, then ascending numeric ID.
3. Select at most one value at or below `0.50`.

## Loss Arithmetic

- Inclusive span is `last - first + 1` using widened checked arithmetic.
- Loss ranges are clipped to the span, sorted, and unioned before counting.
- Material loss is tested exactly as `missing * 10 >= span` with widened integers.
- An invalid reversed span fails closed to suppression.
- A reversed individual range fails closed instead of being silently discarded.
- Identical ranges repeated by several metric receipts are counted once.

## Lifecycle

`EncounterProjection -> RecommendationReport -> app-owned EncounterDetail -> UI`

The report is discarded with the selected detail. A catalog replacement clears
and rebuilds both from the same new projection. No recommendation is written to
raw storage, catalog storage, configuration, logs, or a derived file.
