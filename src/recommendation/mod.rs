//! Deterministic, evidence-scoped encounter review prompts.
//!
//! This module consumes one immutable encounter projection and has no I/O,
//! persistence, network, logging, or gameplay action surface.

use std::collections::BTreeSet;

use crate::catalog::Channel;
use crate::encounter::{
    EncounterProjection, LossRange, MetricQuality, MetricResult, ALGORITHM_VERSION,
    PROJECTION_SCHEMA_VERSION,
};

pub const RECOMMENDATION_SCHEMA_VERSION: u32 = 1;
pub const RECOMMENDATION_POLICY_VERSION: &str = "s090-v1";

const MIN_DURATION_MS: u64 = 10_000;
const MIN_CASTS: usize = 3;
const DOMINANT_DAMAGE_SHARE: f64 = 0.40;
const LOW_EFFECT_UPTIME: f64 = 0.50;
const MATERIAL_UNKNOWN_DAMAGE_SHARE: f64 = 0.25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecommendationAvailability {
    Ready,
    Qualified,
    Suppressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdviceQualification {
    Provisional,
    ProvisionalQualified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdviceRule {
    DominantDamageShare,
    LowEffectUptime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdviceTargetKind {
    Ability,
    Effect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecommendationReasonKind {
    InsufficientDuration,
    InsufficientCasts,
    UnsupportedProjectionVersion,
    InvalidSequenceSpan,
    InvalidLossRange,
    InvalidMetricEvidence,
    MaterialCaptureLoss,
    DeclaredCaptureLoss,
    UnknownCatalogIds,
    MaterialUnknownDamageShare,
    UnknownTargetOmitted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationReason {
    pub kind: RecommendationReasonKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationCitation {
    pub session_id: String,
    pub encounter_id: String,
    pub raw_content_sha256: String,
    pub projection_schema_version: u32,
    pub metric_algorithm_version: String,
    pub catalog_schema_version: u32,
    pub catalog_version: String,
    pub catalog_semantic_sha256: String,
    pub channel: Channel,
    pub api_version: u32,
    pub recommendation_schema_version: u32,
    pub recommendation_policy_version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecommendationEvidence {
    pub duration_ms: u64,
    pub cast_count: usize,
    pub first_sequence: u64,
    pub last_sequence: u64,
    pub sequence_span: u128,
    pub missing_sequences: u128,
    pub projection_compatible: bool,
    pub loss_ranges_valid: bool,
    pub metric_evidence_valid: bool,
    pub known_ids: Vec<i64>,
    pub unknown_ids: Vec<i64>,
    pub unknown_damage_share: f64,
    pub loss_ranges: Vec<LossRange>,
    pub citation: RecommendationCitation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdviceItem {
    pub rule: AdviceRule,
    pub target_kind: AdviceTargetKind,
    pub target_id: i64,
    pub observed_ratio: f64,
    pub qualification: AdviceQualification,
    pub citation: RecommendationCitation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecommendationReport {
    pub schema_version: u32,
    pub policy_version: String,
    pub availability: RecommendationAvailability,
    pub evidence: RecommendationEvidence,
    pub reasons: Vec<RecommendationReason>,
    pub advice: Vec<AdviceItem>,
}

pub fn generate_recommendations(projection: &EncounterProjection) -> RecommendationReport {
    let citation = citation(projection);
    let known_ids = positive_ids(&projection.catalog_join.known_ids);
    let known_set = known_ids.iter().copied().collect::<BTreeSet<_>>();
    let unknown_ids = positive_ids(&projection.catalog_join.unknown_ids)
        .into_iter()
        .filter(|id| !known_set.contains(id))
        .collect::<Vec<_>>();
    let unknown_set = unknown_ids.iter().copied().collect::<BTreeSet<_>>();
    let loss_ranges = declared_loss_ranges(projection);
    let loss_ranges_valid = loss_ranges
        .iter()
        .all(|range| range.missing_sequence_to >= range.missing_sequence_from);
    let projection_compatible = projection.schema_version == PROJECTION_SCHEMA_VERSION
        && projection.algorithm_version == ALGORITHM_VERSION;
    let metric_evidence_valid = recommendation_metric_evidence_valid(projection);
    let (sequence_span, missing_sequences, valid_span) = sequence_loss(
        projection.first_sequence,
        projection.last_sequence,
        &loss_ranges,
    );
    let unknown_damage_share = projection
        .ability_damage_share
        .iter()
        .filter(|share| unknown_set.contains(&share.ability_id))
        .filter_map(|share| valid_ratio(share.result.value))
        .sum::<f64>();
    let material_loss = valid_span
        && loss_ranges_valid
        && missing_sequences > 0
        && missing_sequences.saturating_mul(10) >= sequence_span;
    let declared_loss = valid_span && loss_ranges_valid && missing_sequences > 0 && !material_loss;
    let material_unknown_damage = unknown_damage_share >= MATERIAL_UNKNOWN_DAMAGE_SHARE;

    let mut reasons = Vec::new();
    if projection.duration_ms < MIN_DURATION_MS {
        reasons.push(reason(
            RecommendationReasonKind::InsufficientDuration,
            format!(
                "Observed duration is {} ms; provisional advice requires at least {} ms.",
                projection.duration_ms, MIN_DURATION_MS
            ),
        ));
    }
    let cast_count = projection.ordered_cast_sequence.ability_ids.len();
    if cast_count < MIN_CASTS {
        reasons.push(reason(
            RecommendationReasonKind::InsufficientCasts,
            format!(
                "Observed cast count is {cast_count}; provisional advice requires at least {MIN_CASTS}."
            ),
        ));
    }
    if !projection_compatible {
        reasons.push(reason(
            RecommendationReasonKind::UnsupportedProjectionVersion,
            format!(
                "Projection schema {} and algorithm {} are unsupported; s090-v1 requires schema {} and algorithm {}.",
                projection.schema_version,
                projection.algorithm_version,
                PROJECTION_SCHEMA_VERSION,
                ALGORITHM_VERSION
            ),
        ));
    }
    if !valid_span {
        reasons.push(reason(
            RecommendationReasonKind::InvalidSequenceSpan,
            "The observed sequence span is invalid, so advice is suppressed.".into(),
        ));
    }
    if !loss_ranges_valid {
        reasons.push(reason(
            RecommendationReasonKind::InvalidLossRange,
            "A declared loss range is reversed, so advice is suppressed.".into(),
        ));
    }
    if !metric_evidence_valid {
        reasons.push(reason(
            RecommendationReasonKind::InvalidMetricEvidence,
            "Required metric quality, coverage, or algorithm evidence is inconsistent, so advice is suppressed."
                .into(),
        ));
    }
    if valid_span && loss_ranges_valid && material_loss {
        reasons.push(reason(
            RecommendationReasonKind::MaterialCaptureLoss,
            format!(
                "Declared loss covers {missing_sequences} of {sequence_span} source sequences, meeting the 10% suppression threshold."
            ),
        ));
    } else if valid_span && loss_ranges_valid && declared_loss {
        reasons.push(reason(
            RecommendationReasonKind::DeclaredCaptureLoss,
            format!(
                "Declared loss covers {missing_sequences} of {sequence_span} source sequences; retained prompts are qualified."
            ),
        ));
    }
    if !unknown_ids.is_empty() {
        reasons.push(reason(
            RecommendationReasonKind::UnknownCatalogIds,
            format!(
                "{} numeric catalog ID{} remain unresolved; known-target prompts are qualified.",
                unknown_ids.len(),
                if unknown_ids.len() == 1 { "" } else { "s" }
            ),
        ));
    }
    if material_unknown_damage {
        reasons.push(reason(
            RecommendationReasonKind::MaterialUnknownDamageShare,
            format!(
                "Unknown abilities account for {:.1}% of observed damage, so damage-concentration advice is omitted.",
                unknown_damage_share * 100.0
            ),
        ));
    }

    let unknown_target_omitted = projection.ability_damage_share.iter().any(|share| {
        unknown_set.contains(&share.ability_id)
            && valid_ratio(share.result.value).is_some_and(|value| value >= DOMINANT_DAMAGE_SHARE)
    }) || projection.effect_uptime.iter().any(|uptime| {
        unknown_set.contains(&uptime.ability_id)
            && valid_ratio(uptime.result.value).is_some_and(|value| value <= LOW_EFFECT_UPTIME)
    });
    if unknown_target_omitted {
        reasons.push(reason(
            RecommendationReasonKind::UnknownTargetOmitted,
            "A threshold-crossing unknown target was omitted because its meaning is unresolved."
                .into(),
        ));
    }

    let globally_suppressed = projection.duration_ms < MIN_DURATION_MS
        || cast_count < MIN_CASTS
        || !projection_compatible
        || !valid_span
        || !loss_ranges_valid
        || !metric_evidence_valid
        || material_loss;
    let qualified = declared_loss || !unknown_ids.is_empty();
    let availability = if globally_suppressed {
        RecommendationAvailability::Suppressed
    } else if qualified {
        RecommendationAvailability::Qualified
    } else {
        RecommendationAvailability::Ready
    };

    let mut advice = Vec::with_capacity(2);
    if !globally_suppressed {
        let qualification = if qualified {
            AdviceQualification::ProvisionalQualified
        } else {
            AdviceQualification::Provisional
        };

        if !material_unknown_damage {
            if let Some((target_id, observed_ratio)) = dominant_damage(projection, &known_set) {
                advice.push(AdviceItem {
                    rule: AdviceRule::DominantDamageShare,
                    target_kind: AdviceTargetKind::Ability,
                    target_id,
                    observed_ratio,
                    qualification,
                    citation: citation.clone(),
                });
            }
        }
        if let Some((target_id, observed_ratio)) = low_uptime(projection, &known_set) {
            advice.push(AdviceItem {
                rule: AdviceRule::LowEffectUptime,
                target_kind: AdviceTargetKind::Effect,
                target_id,
                observed_ratio,
                qualification,
                citation: citation.clone(),
            });
        }
    }

    RecommendationReport {
        schema_version: RECOMMENDATION_SCHEMA_VERSION,
        policy_version: RECOMMENDATION_POLICY_VERSION.into(),
        availability,
        evidence: RecommendationEvidence {
            duration_ms: projection.duration_ms,
            cast_count,
            first_sequence: projection.first_sequence,
            last_sequence: projection.last_sequence,
            sequence_span,
            missing_sequences,
            projection_compatible,
            loss_ranges_valid,
            metric_evidence_valid,
            known_ids,
            unknown_ids,
            unknown_damage_share,
            loss_ranges,
            citation,
        },
        reasons,
        advice,
    }
}

fn citation(projection: &EncounterProjection) -> RecommendationCitation {
    RecommendationCitation {
        session_id: projection.session_id.clone(),
        encounter_id: projection.encounter_id.clone(),
        raw_content_sha256: projection.raw_content_sha256.clone(),
        projection_schema_version: projection.schema_version,
        metric_algorithm_version: projection.algorithm_version.clone(),
        catalog_schema_version: projection.catalog_join.catalog_schema_version,
        catalog_version: projection.catalog_join.catalog_version.clone(),
        catalog_semantic_sha256: projection.catalog_join.catalog_semantic_sha256.clone(),
        channel: projection.catalog_join.channel,
        api_version: projection.catalog_join.api_version,
        recommendation_schema_version: RECOMMENDATION_SCHEMA_VERSION,
        recommendation_policy_version: RECOMMENDATION_POLICY_VERSION.into(),
    }
}

fn positive_ids(ids: &[i64]) -> Vec<i64> {
    ids.iter()
        .copied()
        .filter(|id| *id > 0)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn sorted_loss_ranges(ranges: &[LossRange]) -> Vec<LossRange> {
    let mut ranges = ranges.to_vec();
    ranges.sort_by(|left, right| {
        left.missing_sequence_from
            .cmp(&right.missing_sequence_from)
            .then(left.missing_sequence_to.cmp(&right.missing_sequence_to))
            .then(left.reason.cmp(&right.reason))
    });
    ranges
}

fn declared_loss_ranges(projection: &EncounterProjection) -> Vec<LossRange> {
    let mut ranges = projection
        .observed_dps
        .loss_ranges
        .iter()
        .chain(projection.effective_hps.loss_ranges.iter())
        .chain(
            projection
                .ability_damage_share
                .iter()
                .flat_map(|share| share.result.loss_ranges.iter()),
        )
        .chain(
            projection
                .effect_uptime
                .iter()
                .flat_map(|uptime| uptime.result.loss_ranges.iter()),
        )
        .chain(projection.ordered_cast_sequence.loss_ranges.iter())
        .cloned()
        .collect::<Vec<_>>();
    ranges = sorted_loss_ranges(&ranges);
    ranges.dedup_by(|left, right| {
        left.missing_sequence_from == right.missing_sequence_from
            && left.missing_sequence_to == right.missing_sequence_to
            && left.reason == right.reason
    });
    ranges
}

fn recommendation_metric_evidence_valid(projection: &EncounterProjection) -> bool {
    metric_evidence_valid(&projection.observed_dps, projection)
        && metric_evidence_valid(&projection.effective_hps, projection)
        && projection
            .ability_damage_share
            .iter()
            .all(|share| metric_evidence_valid(&share.result, projection))
        && projection
            .effect_uptime
            .iter()
            .all(|uptime| metric_evidence_valid(&uptime.result, projection))
        && projection.ordered_cast_sequence.algorithm_version == ALGORITHM_VERSION
        && projection.ordered_cast_sequence.first_sequence == projection.first_sequence
        && projection.ordered_cast_sequence.last_sequence == projection.last_sequence
        && quality_matches_loss(
            projection.ordered_cast_sequence.quality,
            &projection.ordered_cast_sequence.loss_ranges,
        )
}

fn metric_evidence_valid(result: &MetricResult, projection: &EncounterProjection) -> bool {
    result.algorithm_version == ALGORITHM_VERSION
        && result.first_sequence == projection.first_sequence
        && result.last_sequence == projection.last_sequence
        && quality_matches_loss(result.quality, &result.loss_ranges)
}

fn quality_matches_loss(quality: MetricQuality, ranges: &[LossRange]) -> bool {
    matches!(
        (quality, ranges.is_empty()),
        (MetricQuality::Complete, true) | (MetricQuality::Degraded, false)
    )
}

fn sequence_loss(first: u64, last: u64, ranges: &[LossRange]) -> (u128, u128, bool) {
    if last < first {
        return (0, 0, false);
    }
    let span = u128::from(last) - u128::from(first) + 1;
    let mut intervals = ranges
        .iter()
        .filter(|range| range.missing_sequence_to >= range.missing_sequence_from)
        .filter_map(|range| {
            let start = range.missing_sequence_from.max(first);
            let end = range.missing_sequence_to.min(last);
            (start <= end).then_some((start, end))
        })
        .collect::<Vec<_>>();
    intervals.sort_unstable();

    let mut missing = 0_u128;
    let mut current: Option<(u64, u64)> = None;
    for (start, end) in intervals {
        match current {
            None => current = Some((start, end)),
            Some((current_start, current_end)) if start <= current_end.saturating_add(1) => {
                current = Some((current_start, current_end.max(end)));
            }
            Some((current_start, current_end)) => {
                missing += u128::from(current_end) - u128::from(current_start) + 1;
                current = Some((start, end));
            }
        }
    }
    if let Some((start, end)) = current {
        missing += u128::from(end) - u128::from(start) + 1;
    }
    (span, missing, true)
}

fn dominant_damage(
    projection: &EncounterProjection,
    known_ids: &BTreeSet<i64>,
) -> Option<(i64, f64)> {
    projection
        .ability_damage_share
        .iter()
        .filter(|share| share.ability_id > 0 && known_ids.contains(&share.ability_id))
        .filter_map(|share| valid_ratio(share.result.value).map(|value| (share.ability_id, value)))
        .filter(|(_, value)| *value >= DOMINANT_DAMAGE_SHARE)
        .min_by(|left, right| right.1.total_cmp(&left.1).then(left.0.cmp(&right.0)))
}

fn low_uptime(projection: &EncounterProjection, known_ids: &BTreeSet<i64>) -> Option<(i64, f64)> {
    projection
        .effect_uptime
        .iter()
        .filter(|uptime| uptime.ability_id > 0 && known_ids.contains(&uptime.ability_id))
        .filter_map(|uptime| {
            valid_ratio(uptime.result.value).map(|value| (uptime.ability_id, value))
        })
        .filter(|(_, value)| *value <= LOW_EFFECT_UPTIME)
        .min_by(|left, right| left.1.total_cmp(&right.1).then(left.0.cmp(&right.0)))
}

fn valid_ratio(value: Option<f64>) -> Option<f64> {
    value.filter(|value| value.is_finite() && (0.0..=1.0).contains(value))
}

fn reason(kind: RecommendationReasonKind, message: String) -> RecommendationReason {
    RecommendationReason { kind, message }
}
