use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    AbilityDamageShare, CatalogJoinReceipt, EffectUptime, EncounterProjection, LossRange,
    MetricQuality, MetricResult, OrderedCastSequence,
};
use eso_weave::recommendation::{
    generate_recommendations, AdviceQualification, AdviceRule, AdviceTargetKind,
    RecommendationAvailability, RecommendationReasonKind, RECOMMENDATION_POLICY_VERSION,
    RECOMMENDATION_SCHEMA_VERSION,
};

fn metric(id: &str, value: Option<f64>) -> MetricResult {
    MetricResult {
        metric_id: id.into(),
        value,
        unit: "ratio".into(),
        algorithm_version: "s069-v1".into(),
        first_sequence: 1,
        last_sequence: 100,
        quality: MetricQuality::Complete,
        loss_ranges: Vec::new(),
    }
}

fn projection() -> EncounterProjection {
    EncounterProjection {
        schema_version: 1,
        algorithm_version: "s069-v1".into(),
        session_id: "session-090".into(),
        encounter_id: "encounter-090".into(),
        channel: Channel::Live,
        duration_ms: 30_000,
        first_sequence: 1,
        last_sequence: 100,
        raw_content_sha256: "raw090".into(),
        catalog_join: CatalogJoinReceipt {
            catalog_schema_version: 1,
            catalog_version: "live-101070-090".into(),
            catalog_semantic_sha256: "catalog090".into(),
            channel: Channel::Live,
            api_version: 101070,
            raw_content_sha256: "raw090".into(),
            known_ability_ids: vec![10],
            unknown_ability_ids: Vec::new(),
            known_effect_ids: vec![20],
            unknown_effect_ids: Vec::new(),
            known_ids: vec![10, 20],
            unknown_ids: Vec::new(),
        },
        observed_dps: MetricResult {
            metric_id: "observed-dps".into(),
            value: Some(1_000.0),
            unit: "damage-per-second".into(),
            algorithm_version: "s069-v1".into(),
            first_sequence: 1,
            last_sequence: 100,
            quality: MetricQuality::Complete,
            loss_ranges: Vec::new(),
        },
        effective_hps: MetricResult {
            metric_id: "observed-hps".into(),
            value: Some(100.0),
            unit: "effective-healing-per-second".into(),
            algorithm_version: "s069-v1".into(),
            first_sequence: 1,
            last_sequence: 100,
            quality: MetricQuality::Complete,
            loss_ranges: Vec::new(),
        },
        ability_damage_share: vec![AbilityDamageShare {
            ability_id: 10,
            result: metric("ability-damage-share", Some(0.60)),
        }],
        effect_uptime: vec![EffectUptime {
            ability_id: 20,
            result: metric("effect-uptime", Some(0.40)),
        }],
        ordered_cast_sequence: OrderedCastSequence {
            metric_id: "ordered-cast-sequence".into(),
            ability_ids: vec![10, 10, 20],
            unit: "ability-id-sequence".into(),
            algorithm_version: "s069-v1".into(),
            first_sequence: 1,
            last_sequence: 100,
            quality: MetricQuality::Complete,
            loss_ranges: Vec::new(),
        },
    }
}

fn reason_kinds(
    report: &eso_weave::recommendation::RecommendationReport,
) -> Vec<RecommendationReasonKind> {
    report.reasons.iter().map(|reason| reason.kind).collect()
}

fn set_sequence_span(projection: &mut EncounterProjection, first: u64, last: u64) {
    projection.first_sequence = first;
    projection.last_sequence = last;
    projection.observed_dps.first_sequence = first;
    projection.observed_dps.last_sequence = last;
    projection.effective_hps.first_sequence = first;
    projection.effective_hps.last_sequence = last;
    for share in &mut projection.ability_damage_share {
        share.result.first_sequence = first;
        share.result.last_sequence = last;
    }
    for uptime in &mut projection.effect_uptime {
        uptime.result.first_sequence = first;
        uptime.result.last_sequence = last;
    }
    projection.ordered_cast_sequence.first_sequence = first;
    projection.ordered_cast_sequence.last_sequence = last;
}

#[test]
fn complete_evidence_produces_stable_bounded_fully_cited_prompts() {
    let projection = projection();
    let expected = generate_recommendations(&projection);

    assert_eq!(expected.schema_version, RECOMMENDATION_SCHEMA_VERSION);
    assert_eq!(expected.policy_version, RECOMMENDATION_POLICY_VERSION);
    assert_eq!(expected.availability, RecommendationAvailability::Ready);
    assert!(expected.evidence.projection_compatible);
    assert!(expected.evidence.loss_ranges_valid);
    assert!(expected.evidence.metric_evidence_valid);
    assert!(expected.reasons.is_empty());
    assert_eq!(expected.advice.len(), 2);
    assert_eq!(expected.advice[0].rule, AdviceRule::DominantDamageShare);
    assert_eq!(expected.advice[1].rule, AdviceRule::LowEffectUptime);
    assert!(expected.advice.len() <= 2);

    for item in &expected.advice {
        assert_eq!(item.qualification, AdviceQualification::Provisional);
        assert_eq!(item.citation.session_id, projection.session_id);
        assert_eq!(item.citation.encounter_id, projection.encounter_id);
        assert_eq!(item.citation.raw_content_sha256, "raw090");
        assert_eq!(item.citation.projection_schema_version, 1);
        assert_eq!(item.citation.metric_algorithm_version, "s069-v1");
        assert_eq!(item.citation.catalog_schema_version, 1);
        assert_eq!(item.citation.catalog_version, "live-101070-090");
        assert_eq!(item.citation.catalog_semantic_sha256, "catalog090");
        assert_eq!(item.citation.channel, Channel::Live);
        assert_eq!(item.citation.api_version, 101070);
        assert_eq!(item.citation.recommendation_schema_version, 1);
        assert_eq!(item.citation.recommendation_policy_version, "s090-v1");
    }

    for _ in 0..100 {
        assert_eq!(generate_recommendations(&projection), expected);
    }
}

#[test]
fn duration_and_cast_boundaries_fail_closed() {
    let mut short = projection();
    short.duration_ms = 9_999;
    let report = generate_recommendations(&short);
    assert_eq!(report.availability, RecommendationAvailability::Suppressed);
    assert!(report.advice.is_empty());
    assert!(reason_kinds(&report).contains(&RecommendationReasonKind::InsufficientDuration));

    short.duration_ms = 10_000;
    short.ordered_cast_sequence.ability_ids = vec![10, 20];
    let report = generate_recommendations(&short);
    assert_eq!(report.availability, RecommendationAvailability::Suppressed);
    assert!(reason_kinds(&report).contains(&RecommendationReasonKind::InsufficientCasts));

    short.ordered_cast_sequence.ability_ids.push(10);
    let report = generate_recommendations(&short);
    assert_eq!(report.availability, RecommendationAvailability::Ready);
    assert_eq!(report.advice.len(), 2);
}

#[test]
fn exact_loss_threshold_qualifies_below_ten_percent_and_suppresses_at_it() {
    let mut input = projection();
    set_sequence_span(&mut input, 1, 11);
    input.observed_dps.loss_ranges = vec![LossRange {
        missing_sequence_from: 5,
        missing_sequence_to: 5,
        reason: "bounded-gap".into(),
    }];
    input.observed_dps.quality = MetricQuality::Degraded;
    let qualified = generate_recommendations(&input);
    assert_eq!(qualified.evidence.sequence_span, 11);
    assert_eq!(qualified.evidence.missing_sequences, 1);
    assert_eq!(
        qualified.availability,
        RecommendationAvailability::Qualified
    );
    assert_eq!(
        reason_kinds(&qualified),
        vec![RecommendationReasonKind::DeclaredCaptureLoss]
    );
    assert!(qualified
        .advice
        .iter()
        .all(|item| item.qualification == AdviceQualification::ProvisionalQualified));

    set_sequence_span(&mut input, 1, 10);
    let suppressed = generate_recommendations(&input);
    assert_eq!(suppressed.evidence.sequence_span, 10);
    assert_eq!(suppressed.evidence.missing_sequences, 1);
    assert_eq!(
        suppressed.availability,
        RecommendationAvailability::Suppressed
    );
    assert!(suppressed.advice.is_empty());
    assert_eq!(
        reason_kinds(&suppressed),
        vec![RecommendationReasonKind::MaterialCaptureLoss]
    );
}

#[test]
fn loss_ranges_are_clipped_unioned_and_reversed_spans_suppress() {
    let mut input = projection();
    set_sequence_span(&mut input, u64::MAX - 9, u64::MAX);
    input.observed_dps.loss_ranges = vec![
        LossRange {
            missing_sequence_from: u64::MAX - 8,
            missing_sequence_to: u64::MAX - 5,
            reason: "first".into(),
        },
        LossRange {
            missing_sequence_from: u64::MAX - 6,
            missing_sequence_to: u64::MAX,
            reason: "overlap".into(),
        },
    ];
    input.observed_dps.quality = MetricQuality::Degraded;
    let report = generate_recommendations(&input);
    assert_eq!(report.evidence.sequence_span, 10);
    assert_eq!(report.evidence.missing_sequences, 9);
    assert_eq!(report.availability, RecommendationAvailability::Suppressed);

    set_sequence_span(&mut input, 10, 9);
    let report = generate_recommendations(&input);
    assert_eq!(report.availability, RecommendationAvailability::Suppressed);
    assert!(report.advice.is_empty());
    assert!(reason_kinds(&report).contains(&RecommendationReasonKind::InvalidSequenceSpan));

    let mut full_u64_span = projection();
    set_sequence_span(&mut full_u64_span, 0, u64::MAX);
    let report = generate_recommendations(&full_u64_span);
    assert_eq!(report.evidence.sequence_span, u128::from(u64::MAX) + 1);
    assert_eq!(report.availability, RecommendationAvailability::Ready);
}

#[test]
fn reversed_loss_ranges_and_inconsistent_metric_evidence_fail_closed() {
    let mut input = projection();
    input.effect_uptime[0].result.quality = MetricQuality::Degraded;
    input.effect_uptime[0].result.loss_ranges = vec![LossRange {
        missing_sequence_from: 8,
        missing_sequence_to: 7,
        reason: "reversed".into(),
    }];
    let reversed = generate_recommendations(&input);
    assert_eq!(
        reversed.availability,
        RecommendationAvailability::Suppressed
    );
    assert!(reversed.advice.is_empty());
    assert!(!reversed.evidence.loss_ranges_valid);
    assert!(reason_kinds(&reversed).contains(&RecommendationReasonKind::InvalidLossRange));

    input.effect_uptime[0].result.loss_ranges.clear();
    let inconsistent = generate_recommendations(&input);
    assert_eq!(
        inconsistent.availability,
        RecommendationAvailability::Suppressed
    );
    assert!(!inconsistent.evidence.metric_evidence_valid);
    assert!(reason_kinds(&inconsistent).contains(&RecommendationReasonKind::InvalidMetricEvidence));
}

#[test]
fn invalid_effective_hps_evidence_fails_closed() {
    let mut input = projection();
    input.effective_hps.quality = MetricQuality::Degraded;
    input.effective_hps.loss_ranges = vec![LossRange {
        missing_sequence_from: 8,
        missing_sequence_to: 7,
        reason: "reversed-hps-evidence".into(),
    }];
    let reversed = generate_recommendations(&input);
    assert_eq!(
        reversed.availability,
        RecommendationAvailability::Suppressed
    );
    assert!(reversed.advice.is_empty());
    assert!(!reversed.evidence.loss_ranges_valid);
    assert!(reason_kinds(&reversed).contains(&RecommendationReasonKind::InvalidLossRange));

    input.effective_hps.loss_ranges.clear();
    let inconsistent_quality = generate_recommendations(&input);
    assert_eq!(
        inconsistent_quality.availability,
        RecommendationAvailability::Suppressed
    );
    assert!(!inconsistent_quality.evidence.metric_evidence_valid);
    assert!(reason_kinds(&inconsistent_quality)
        .contains(&RecommendationReasonKind::InvalidMetricEvidence));

    input.effective_hps.quality = MetricQuality::Complete;
    input.effective_hps.algorithm_version = "unexpected-hps-version".into();
    let inconsistent_algorithm = generate_recommendations(&input);
    assert_eq!(
        inconsistent_algorithm.availability,
        RecommendationAvailability::Suppressed
    );
    assert!(!inconsistent_algorithm.evidence.metric_evidence_valid);
}

#[test]
fn valid_degraded_candidate_evidence_qualifies_instead_of_blocking() {
    let mut input = projection();
    input.effect_uptime[0].result.quality = MetricQuality::Degraded;
    input.effect_uptime[0].result.loss_ranges = vec![LossRange {
        missing_sequence_from: 50,
        missing_sequence_to: 50,
        reason: "bounded-gap".into(),
    }];
    let report = generate_recommendations(&input);
    assert_eq!(report.availability, RecommendationAvailability::Qualified);
    assert_eq!(report.evidence.missing_sequences, 1);
    assert_eq!(report.advice.len(), 2);
    assert!(report
        .advice
        .iter()
        .all(|item| item.qualification == AdviceQualification::ProvisionalQualified));
}

#[test]
fn unsupported_projection_versions_fail_closed() {
    for (schema, algorithm) in [(0, "s069-v1"), (1, ""), (99, "future-v1")] {
        let mut input = projection();
        input.schema_version = schema;
        input.algorithm_version = algorithm.into();
        let report = generate_recommendations(&input);
        assert_eq!(report.availability, RecommendationAvailability::Suppressed);
        assert!(report.advice.is_empty());
        assert!(!report.evidence.projection_compatible);
        assert!(
            reason_kinds(&report).contains(&RecommendationReasonKind::UnsupportedProjectionVersion)
        );
    }
}

#[test]
fn unknown_ids_qualify_and_material_unknown_damage_is_rule_local() {
    let mut input = projection();
    input.catalog_join.unknown_ids = vec![999];
    input.catalog_join.unknown_ability_ids = vec![999];
    input.ability_damage_share = vec![
        AbilityDamageShare {
            ability_id: 10,
            result: metric("ability-damage-share", Some(0.751)),
        },
        AbilityDamageShare {
            ability_id: 999,
            result: metric("ability-damage-share", Some(0.249)),
        },
    ];
    let below = generate_recommendations(&input);
    assert_eq!(below.availability, RecommendationAvailability::Qualified);
    assert_eq!(below.advice.len(), 2);
    assert!((below.evidence.unknown_damage_share - 0.249).abs() < f64::EPSILON);
    assert!(reason_kinds(&below).contains(&RecommendationReasonKind::UnknownCatalogIds));

    input.ability_damage_share[0].result.value = Some(0.75);
    input.ability_damage_share[1].result.value = Some(0.25);
    let at = generate_recommendations(&input);
    assert_eq!(at.availability, RecommendationAvailability::Qualified);
    assert!((at.evidence.unknown_damage_share - 0.25).abs() < f64::EPSILON);
    assert_eq!(at.advice.len(), 1);
    assert_eq!(at.advice[0].rule, AdviceRule::LowEffectUptime);
    assert!(reason_kinds(&at).contains(&RecommendationReasonKind::MaterialUnknownDamageShare));
}

#[test]
fn unknown_targets_are_omitted_without_blocking_known_candidates() {
    let mut input = projection();
    input.catalog_join.unknown_ids = vec![7, 8];
    input.catalog_join.unknown_ability_ids = vec![7];
    input.catalog_join.unknown_effect_ids = vec![8];
    input.ability_damage_share.push(AbilityDamageShare {
        ability_id: 7,
        result: metric("ability-damage-share", Some(0.10)),
    });
    input.effect_uptime.insert(
        0,
        EffectUptime {
            ability_id: 8,
            result: metric("effect-uptime", Some(0.10)),
        },
    );

    let report = generate_recommendations(&input);
    assert_eq!(report.availability, RecommendationAvailability::Qualified);
    assert_eq!(report.advice.len(), 2);
    assert_eq!(report.advice[0].target_id, 10);
    assert_eq!(report.advice[1].target_id, 20);
    assert!(reason_kinds(&report).contains(&RecommendationReasonKind::UnknownTargetOmitted));
}

#[test]
fn catalog_entity_kinds_prevent_cross_kind_target_confusion() {
    let mut input = projection();
    input.catalog_join.known_ids = vec![10, 20];
    input.catalog_join.unknown_ids = vec![10, 20];
    input.catalog_join.known_ability_ids = vec![10];
    input.catalog_join.unknown_ability_ids = vec![20];
    input.catalog_join.known_effect_ids = vec![20];
    input.catalog_join.unknown_effect_ids = vec![10];
    input.ability_damage_share = vec![
        AbilityDamageShare {
            ability_id: 10,
            result: metric("ability-damage-share", Some(0.40)),
        },
        AbilityDamageShare {
            ability_id: 20,
            result: metric("ability-damage-share", Some(0.10)),
        },
    ];
    input.effect_uptime = vec![
        EffectUptime {
            ability_id: 10,
            result: metric("effect-uptime", Some(0.10)),
        },
        EffectUptime {
            ability_id: 20,
            result: metric("effect-uptime", Some(0.40)),
        },
    ];

    let report = generate_recommendations(&input);
    assert_eq!(report.availability, RecommendationAvailability::Qualified);
    assert_eq!(
        report
            .advice
            .iter()
            .map(|item| (item.target_kind, item.target_id))
            .collect::<Vec<_>>(),
        vec![
            (AdviceTargetKind::Ability, 10),
            (AdviceTargetKind::Effect, 20),
        ]
    );
    assert_eq!(report.evidence.known_ids, vec![10, 20]);
    assert_eq!(report.evidence.unknown_ids, vec![10, 20]);
    assert!(reason_kinds(&report).contains(&RecommendationReasonKind::UnknownTargetOmitted));
}

#[test]
fn selection_uses_fixed_rule_order_and_numeric_tie_breakers() {
    let mut input = projection();
    input.catalog_join.known_ids = vec![10, 11, 20, 21];
    input.catalog_join.known_ability_ids = vec![10, 11];
    input.catalog_join.known_effect_ids = vec![20, 21];
    input.ability_damage_share.push(AbilityDamageShare {
        ability_id: 11,
        result: metric("ability-damage-share", Some(0.60)),
    });
    input.effect_uptime.push(EffectUptime {
        ability_id: 21,
        result: metric("effect-uptime", Some(0.40)),
    });

    let report = generate_recommendations(&input);
    assert_eq!(report.advice[0].rule, AdviceRule::DominantDamageShare);
    assert_eq!(report.advice[0].target_kind, AdviceTargetKind::Ability);
    assert_eq!(report.advice[0].target_id, 10);
    assert_eq!(report.advice[1].rule, AdviceRule::LowEffectUptime);
    assert_eq!(report.advice[1].target_kind, AdviceTargetKind::Effect);
    assert_eq!(report.advice[1].target_id, 20);
}

#[test]
fn exact_rule_thresholds_are_inclusive_and_evidence_ids_are_normalized() {
    let mut input = projection();
    input.catalog_join.known_ids = vec![20, 10, 20, 0, -1];
    input.catalog_join.unknown_ids = vec![30, 30, 0, -2, 10];
    input.catalog_join.known_ability_ids = vec![10, 10, 0, -1];
    input.catalog_join.known_effect_ids = vec![20, 20, 0];
    input.catalog_join.unknown_ability_ids = vec![30, 30, 0, -2, 10];
    input.ability_damage_share[0].result.value = Some(0.40);
    input.effect_uptime[0].result.value = Some(0.50);

    let report = generate_recommendations(&input);
    assert_eq!(report.evidence.known_ids, vec![10, 20]);
    assert_eq!(report.evidence.unknown_ids, vec![30]);
    assert_eq!(report.advice.len(), 2);
    assert_eq!(report.advice[0].observed_ratio, 0.40);
    assert_eq!(report.advice[1].observed_ratio, 0.50);

    input.catalog_join.known_ids.clear();
    input.catalog_join.unknown_ids.clear();
    input.catalog_join.known_ability_ids.clear();
    input.catalog_join.unknown_ability_ids.clear();
    input.catalog_join.known_effect_ids.clear();
    input.catalog_join.unknown_effect_ids.clear();
    input.ability_damage_share.clear();
    input.effect_uptime.clear();
    let empty_join = generate_recommendations(&input);
    assert_eq!(empty_join.evidence.unknown_damage_share, 0.0);
    assert!(empty_join.advice.is_empty());
}

#[test]
fn unavailable_non_finite_out_of_range_and_non_crossing_values_produce_no_advice() {
    let mut input = projection();
    input.ability_damage_share = vec![
        AbilityDamageShare {
            ability_id: 10,
            result: metric("ability-damage-share", None),
        },
        AbilityDamageShare {
            ability_id: 10,
            result: metric("ability-damage-share", Some(f64::NAN)),
        },
        AbilityDamageShare {
            ability_id: 10,
            result: metric("ability-damage-share", Some(1.1)),
        },
        AbilityDamageShare {
            ability_id: 10,
            result: metric("ability-damage-share", Some(0.39)),
        },
    ];
    input.effect_uptime = vec![
        EffectUptime {
            ability_id: 20,
            result: metric("effect-uptime", Some(f64::INFINITY)),
        },
        EffectUptime {
            ability_id: 20,
            result: metric("effect-uptime", Some(-0.1)),
        },
        EffectUptime {
            ability_id: 20,
            result: metric("effect-uptime", Some(0.51)),
        },
    ];

    let report = generate_recommendations(&input);
    assert_eq!(report.availability, RecommendationAvailability::Ready);
    assert!(report.advice.is_empty());
}

#[test]
fn recommendation_domain_has_no_input_action_or_external_service_dependency() {
    let source = include_str!("../src/recommendation/mod.rs");
    for forbidden in [
        "crate::input",
        "crate::weave",
        "crate::fishing",
        "crate::potion",
        "crate::pixelbus",
        "crate::game",
        "reqwest",
        "rusqlite",
        "std::fs",
        "std::net",
    ] {
        assert!(
            !source.contains(forbidden),
            "recommendation domain unexpectedly depends on {forbidden}"
        );
    }
}
