use std::collections::{BTreeMap, BTreeSet};

use super::model::{
    CaptureStatus, EncounterCapture, EncounterEvent, PayloadValue, RawObservation, RawSourceKind,
    RawValue, RawValueType,
};
use super::{invalid, EncounterError, MAX_ACTORS, MAX_ESTIMATED_BYTES, MAX_EVENTS};

const EVENT_KINDS: [&str; 14] = [
    "encounter-start",
    "encounter-end",
    "damage",
    "healing",
    "effect",
    "resource",
    "cast",
    "bar-change",
    "death",
    "resurrection",
    "boss-health",
    "performance",
    "quickslot",
    "discontinuity",
];

pub(crate) fn validate(capture: &EncounterCapture) -> Result<(), EncounterError> {
    match capture.schema_version {
        1 if capture.addon_version == 1 => {
            if capture.privacy_profile.as_deref() != Some("anonymous-local-v1") {
                return invalid("unsupported encounter privacy profile");
            }
            validate_legacy_raw_absence(capture)?;
        }
        2 if capture.addon_version == 2 => {
            if capture.privacy_profile.is_some() {
                return invalid("lossless encounter capture cannot declare a privacy profile");
            }
            validate_raw_capture(capture)?;
        }
        _ => return invalid("unsupported encounter capture version"),
    }
    if capture.source.api_version == 0
        || capture.source.game_version.is_empty()
        || capture.source.game_version.len() > 128
        || !safe_source_token(&capture.source.game_version)
        || capture.source.locale.is_empty()
        || capture.source.locale.len() > 16
        || !safe_source_token(&capture.source.locale)
        || capture.source.platform.is_empty()
        || capture.source.platform.len() > 32
        || !safe_source_token(&capture.source.platform)
    {
        return invalid("encounter source provenance is invalid");
    }
    validate_opaque_id(&capture.session_id, "session")?;
    validate_opaque_id(&capture.encounter_id, "encounter")?;
    let started_at = validate_decimal_time(&capture.started_at)?;
    let finished_at = validate_decimal_time(&capture.finished_at)?;
    if finished_at < started_at {
        return invalid("encounter finished before it started");
    }
    if capture.stored_event_count != capture.events.len()
        || !(2..=MAX_EVENTS).contains(&capture.stored_event_count)
        || capture.estimated_bytes > MAX_ESTIMATED_BYTES
    {
        return invalid("encounter count or byte metadata is invalid");
    }
    if capture.first_sequence != 1
        || capture.last_sequence < capture.first_sequence
        || capture
            .last_sequence
            .checked_sub(capture.first_sequence)
            .and_then(|value| value.checked_add(1))
            .and_then(|value| value.checked_sub(capture.omitted_event_count))
            != Some(capture.stored_event_count as u64)
    {
        return invalid("encounter sequence span does not reconcile with counts");
    }
    for (name, count) in &capture.warnings {
        if !matches!(
            name.as_str(),
            "actor_limit" | "terminal_reserve_exhausted" | "recovered_interruption"
        ) || *count == 0
        {
            return invalid("encounter warning is not a supported counted token");
        }
    }

    let first = capture.events.first().expect("validated minimum length");
    let last = capture.events.last().expect("validated minimum length");
    if first.kind != "encounter-start"
        || first.sequence != capture.first_sequence
        || first.monotonic_ms != 0
        || last.kind != "encounter-end"
        || last.sequence != capture.last_sequence
        || last.monotonic_ms != capture.ended_monotonic_ms
    {
        return invalid("encounter boundary events do not match terminal metadata");
    }

    let mut previous_sequence = 0_u64;
    let mut previous_time = 0_u64;
    let mut declared_omissions = 0_u64;
    let mut discontinuity_reasons = BTreeSet::new();
    for event in &capture.events {
        validate_event(capture, event)?;
        if event.sequence <= previous_sequence || event.monotonic_ms < previous_time {
            return invalid("encounter event ordering is invalid");
        }
        if previous_sequence > 0 {
            let expected = previous_sequence
                .checked_add(1)
                .ok_or_else(|| EncounterError::Validation("encounter sequence overflow".into()))?;
            if event.sequence == expected {
                if event.kind == "discontinuity" {
                    return invalid("encounter discontinuity does not follow an omitted range");
                }
            } else {
                if event.kind != "discontinuity" {
                    return invalid("encounter sequence gap lacks a discontinuity");
                }
                let from = integer(&event.payload, "missing_sequence_from")? as u64;
                let to = integer(&event.payload, "missing_sequence_to")? as u64;
                if from != expected || to.checked_add(1) != Some(event.sequence) || to < from {
                    return invalid("encounter discontinuity does not declare its exact gap");
                }
                declared_omissions = declared_omissions
                    .checked_add(to - from + 1)
                    .ok_or_else(|| EncounterError::Validation("omission count overflow".into()))?;
                discontinuity_reasons.insert(string(&event.payload, "reason")?);
            }
        }
        previous_sequence = event.sequence;
        previous_time = event.monotonic_ms;
    }
    if declared_omissions != capture.omitted_event_count {
        return invalid("encounter omission count does not match discontinuities");
    }

    validate_projection_links(capture)?;

    let terminal_reason = string(&last.payload, "reason")?;
    let terminal_complete = boolean(&last.payload, "complete")?;
    let has_raw_clock_reset = capture.raw_observations.iter().any(|observation| {
        observation.source_kind == RawSourceKind::Lifecycle
            && observation.source_id == "clock-reset"
    });
    match capture.status {
        CaptureStatus::Complete => {
            if capture.partial_reason.is_some()
                || capture.omitted_event_count != 0
                || capture
                    .raw_omitted_observation_count
                    .is_some_and(|count| count != 0)
                || capture.raw_loss.is_some()
                || has_raw_clock_reset
                || !terminal_complete
                || terminal_reason != "combat-ended"
            {
                return invalid("complete encounter has inconsistent terminal state");
            }
        }
        CaptureStatus::Partial => {
            let Some(reason) = capture.partial_reason else {
                return invalid("partial encounter lacks a partial reason");
            };
            if terminal_complete || terminal_reason != reason.as_str() {
                return invalid("partial encounter has inconsistent terminal state");
            }
            let reason_has_evidence = matches!(
                reason,
                super::model::PartialReason::UserStopped
                    | super::model::PartialReason::PlayerDeactivated
            ) || discontinuity_reasons.contains(reason.as_str())
                || (reason == super::model::PartialReason::ClockReset && has_raw_clock_reset)
                || capture
                    .raw_loss
                    .as_ref()
                    .is_some_and(|loss| loss.reason.as_str() == reason.as_str());
            if capture.schema_version == 2 && !reason_has_evidence {
                return invalid("partial encounter reason lacks matching loss evidence");
            }
        }
    }
    Ok(())
}

fn validate_legacy_raw_absence(capture: &EncounterCapture) -> Result<(), EncounterError> {
    if capture.raw_first_sequence.is_some()
        || capture.raw_last_sequence.is_some()
        || capture.raw_observation_count.is_some()
        || capture.raw_omitted_observation_count.is_some()
        || capture.raw_loss.is_some()
        || !capture.raw_observations.is_empty()
        || capture
            .events
            .iter()
            .any(|event| event.source_sequence.is_some() || event.projection_ordinal.is_some())
    {
        return invalid("legacy encounter capture contains v2 raw fields");
    }
    Ok(())
}

fn validate_raw_capture(capture: &EncounterCapture) -> Result<(), EncounterError> {
    let Some(first_sequence) = capture.raw_first_sequence else {
        return invalid("lossless encounter capture lacks a first raw sequence");
    };
    let Some(last_sequence) = capture.raw_last_sequence else {
        return invalid("lossless encounter capture lacks a last raw sequence");
    };
    let Some(stored_count) = capture.raw_observation_count else {
        return invalid("lossless encounter capture lacks a raw observation count");
    };
    let Some(omitted_count) = capture.raw_omitted_observation_count else {
        return invalid("lossless encounter capture lacks a raw omission count");
    };
    if first_sequence != 1
        || capture.raw_observations.len() != stored_count
        || !(1..=MAX_EVENTS).contains(&stored_count)
        || last_sequence < first_sequence
        || last_sequence
            .checked_sub(first_sequence)
            .and_then(|value| value.checked_add(1))
            .and_then(|value| value.checked_sub(omitted_count))
            != Some(stored_count as u64)
    {
        return invalid("lossless encounter raw sequence metadata is invalid");
    }

    match (&capture.raw_loss, omitted_count) {
        (None, 0) => {}
        (Some(loss), count) if count > 0 => {
            if loss.missing_sequence_from == 0
                || loss.missing_sequence_to < loss.missing_sequence_from
                || loss
                    .missing_sequence_to
                    .checked_sub(loss.missing_sequence_from)
                    .and_then(|value| value.checked_add(1))
                    != Some(count)
            {
                return invalid("lossless encounter raw loss range is invalid");
            }
        }
        _ => return invalid("lossless encounter raw loss does not match omissions"),
    }

    let mut expected = first_sequence;
    let mut previous_time = 0;
    let mut crossed_loss = false;
    for observation in &capture.raw_observations {
        if observation.sequence != expected {
            let Some(loss) = &capture.raw_loss else {
                return invalid("lossless encounter raw sequence gap is undeclared");
            };
            if crossed_loss
                || expected != loss.missing_sequence_from
                || loss
                    .missing_sequence_to
                    .checked_add(1)
                    .is_none_or(|after_loss| observation.sequence != after_loss)
            {
                return invalid("lossless encounter raw loss does not declare its exact gap");
            }
            crossed_loss = true;
        }
        validate_raw_observation(capture, observation)?;
        if observation.monotonic_ms < previous_time {
            return invalid("lossless encounter raw time moves backward");
        }
        expected = observation
            .sequence
            .checked_add(1)
            .ok_or_else(|| EncounterError::Validation("raw sequence overflow".into()))?;
        previous_time = observation.monotonic_ms;
    }
    if last_sequence
        .checked_add(1)
        .is_none_or(|after_last| expected != after_last)
        || (capture.raw_loss.is_some() && !crossed_loss)
    {
        return invalid("lossless encounter raw terminal sequence is invalid");
    }
    let last = capture
        .raw_observations
        .last()
        .expect("validated raw minimum length");
    if last.source_kind != RawSourceKind::Lifecycle || last.source_id != "capture-finish" {
        return invalid("lossless encounter lacks its terminal raw lifecycle observation");
    }
    Ok(())
}

fn validate_raw_observation(
    capture: &EncounterCapture,
    observation: &RawObservation,
) -> Result<(), EncounterError> {
    if observation.session_id != capture.session_id
        || observation.encounter_id != capture.encounter_id
        || observation.api_version != capture.source.api_version
        || observation.source_version != 1
        || observation.source_id.is_empty()
        || observation.source_id.len() > 128
        || !observation
            .source_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        || observation
            .argument_count
            .checked_add(observation.return_count)
            != Some(observation.values.len())
        || observation.values.len() > 256
    {
        return invalid("lossless encounter raw observation metadata is invalid");
    }
    validate_raw_signature(observation)?;
    for (index, value) in observation.values.iter().enumerate() {
        if value.position != index + 1 {
            return invalid("lossless encounter raw value positions are not contiguous");
        }
        validate_raw_value(value)?;
    }
    Ok(())
}

fn validate_raw_value(value: &RawValue) -> Result<(), EncounterError> {
    let valid = match value.value_type {
        RawValueType::Nil => {
            value.boolean.is_none()
                && value.string.is_none()
                && value.sign.is_none()
                && value.significand.is_none()
                && value.exponent.is_none()
        }
        RawValueType::Boolean => {
            value.boolean.is_some()
                && value.string.is_none()
                && value.sign.is_none()
                && value.significand.is_none()
                && value.exponent.is_none()
        }
        RawValueType::String => {
            value.boolean.is_none()
                && value
                    .string
                    .as_ref()
                    .is_some_and(|text| text.len() <= super::MAX_STRING_BYTES)
                && value.sign.is_none()
                && value.significand.is_none()
                && value.exponent.is_none()
        }
        RawValueType::Number => {
            value.boolean.is_none()
                && value.string.is_none()
                && matches!(value.sign, Some(-1 | 1))
                && value.significand.as_ref().is_some_and(|significand| {
                    !significand.is_empty()
                        && significand.bytes().all(|byte| byte.is_ascii_digit())
                        && (significand == "0" || !significand.starts_with('0'))
                        && significand
                            .parse::<u64>()
                            .is_ok_and(|number| number <= 9_007_199_254_740_991)
                })
                && finite_binary_descriptor(value)
                && (value.significand.as_deref() != Some("0") || value.exponent == Some(0))
        }
    };
    if valid {
        Ok(())
    } else {
        invalid("lossless encounter raw value shape is invalid")
    }
}

fn validate_raw_signature(observation: &RawObservation) -> Result<(), EncounterError> {
    let counts = (observation.argument_count, observation.return_count);
    let valid = match (observation.source_kind, observation.source_id.as_str()) {
        (RawSourceKind::Callback, "EVENT_PLAYER_COMBAT_STATE") => counts.0 >= 2 && counts.1 == 0,
        (RawSourceKind::Callback, "EVENT_PLAYER_DEACTIVATED") => counts.0 >= 1 && counts.1 == 0,
        (RawSourceKind::Callback, "EVENT_COMBAT_EVENT") => counts.0 >= 18 && counts.1 == 0,
        (RawSourceKind::Callback, "EVENT_EFFECT_CHANGED") => counts.0 >= 17 && counts.1 == 0,
        (RawSourceKind::Callback, "EVENT_POWER_UPDATE") => counts.0 >= 7 && counts.1 == 0,
        (RawSourceKind::Callback, "EVENT_ACTION_SLOT_ABILITY_USED")
        | (RawSourceKind::Callback, "EVENT_ACTIVE_QUICKSLOT_CHANGED") => {
            counts.0 >= 2 && counts.1 == 0
        }
        (RawSourceKind::Callback, "EVENT_ACTIVE_WEAPON_PAIR_CHANGED") => {
            counts.0 >= 3 && counts.1 == 0
        }
        (RawSourceKind::Callback, "EVENT_PLAYER_DEAD")
        | (RawSourceKind::Callback, "EVENT_PLAYER_ALIVE") => counts.0 >= 1 && counts.1 == 0,
        (RawSourceKind::Callback, "EVENT_BOSSES_CHANGED") => counts.0 >= 2 && counts.1 == 0,
        (RawSourceKind::ApiSample, "GetSlotBoundId") => {
            matches!(counts, (1, 1) | (2, 1))
        }
        (RawSourceKind::ApiSample, "GetCurrentQuickslot")
        | (RawSourceKind::ApiSample, "GetFramerate")
        | (RawSourceKind::ApiSample, "GetLatency") => counts == (0, 1),
        (RawSourceKind::ApiSample, "DoesUnitExist") => counts == (1, 1),
        (RawSourceKind::ApiSample, "GetUnitPower") => counts == (2, 3),
        (RawSourceKind::Lifecycle, "clock-reset")
        | (RawSourceKind::Lifecycle, "capture-finish") => counts == (2, 0),
        _ => false,
    };
    if !valid {
        return invalid("lossless encounter raw source signature is invalid");
    }
    match observation.source_kind {
        RawSourceKind::Callback => {
            let Some(source_code) = observation.source_code else {
                return invalid("raw callback lacks its event code");
            };
            if observation
                .values
                .first()
                .is_none_or(|value| !raw_number_matches_i64(value, source_code))
            {
                return invalid("raw callback event code does not match its first value");
            }
        }
        RawSourceKind::ApiSample | RawSourceKind::Lifecycle => {
            if observation.source_code.is_some() {
                return invalid("non-callback raw observation has an event code");
            }
        }
    }
    Ok(())
}

fn finite_binary_descriptor(value: &RawValue) -> bool {
    let (Some(significand), Some(exponent)) = (&value.significand, value.exponent) else {
        return false;
    };
    let Ok(significand) = significand.parse::<u64>() else {
        return false;
    };
    if significand == 0 {
        return exponent == 0;
    }
    if !(-1074..=1023).contains(&exponent) {
        return false;
    }
    let highest_bit = 63_i32 - significand.leading_zeros() as i32 + i32::from(exponent);
    if highest_bit < 1023 {
        return true;
    }
    if highest_bit > 1023 || exponent < 971 {
        return false;
    }
    let shift = u32::try_from(exponent - 971).expect("nonnegative bounded shift");
    significand
        .checked_shl(shift)
        .is_some_and(|scaled| scaled <= 9_007_199_254_740_991)
}

fn raw_number_matches_i64(value: &RawValue, expected: i64) -> bool {
    if value.value_type != RawValueType::Number {
        return false;
    }
    let (Some(sign), Some(significand), Some(exponent)) =
        (value.sign, value.significand.as_ref(), value.exponent)
    else {
        return false;
    };
    let Ok(mut magnitude) = significand.parse::<u64>() else {
        return false;
    };
    if exponent >= 0 {
        let Ok(shift) = u32::try_from(exponent) else {
            return false;
        };
        let Some(shifted) = magnitude.checked_shl(shift) else {
            return false;
        };
        magnitude = shifted;
    } else {
        let shift = u32::from(exponent.unsigned_abs());
        if shift >= u64::BITS || magnitude % (1_u64 << shift) != 0 {
            return false;
        }
        magnitude >>= shift;
    }
    let expected_sign = if expected < 0 { -1 } else { 1 };
    sign == expected_sign && magnitude == expected.unsigned_abs()
}

fn validate_projection_links(capture: &EncounterCapture) -> Result<(), EncounterError> {
    if capture.schema_version == 1 {
        return Ok(());
    }
    let raw_sequences = capture
        .raw_observations
        .iter()
        .map(|observation| (observation.sequence, observation))
        .collect::<BTreeMap<_, _>>();
    let mut ordinals = BTreeMap::<u64, u32>::new();
    let mut previous_source_sequence = 0;
    for event in &capture.events {
        let Some(source_sequence) = event.source_sequence else {
            return invalid("v2 normalized event lacks its raw source sequence");
        };
        let Some(ordinal) = event.projection_ordinal else {
            return invalid("v2 normalized event lacks its projection ordinal");
        };
        if let Some(source) = raw_sequences.get(&source_sequence).copied() {
            if source.monotonic_ms != event.monotonic_ms
                || !projection_source_matches(event.kind.as_str(), source.source_id.as_str())
            {
                return invalid("v2 normalized event does not follow its raw source");
            }
        } else {
            let is_declared_lost_start = event.kind == "encounter-start"
                && event.monotonic_ms == 0
                && source_sequence == 1
                && capture.raw_loss.as_ref().is_some_and(|loss| {
                    loss.missing_sequence_from == 1 && loss.missing_sequence_to >= 1
                });
            if !is_declared_lost_start {
                return invalid("v2 normalized event references a missing raw source");
            }
        }
        if source_sequence < previous_source_sequence {
            return invalid("v2 normalized event source sequence moves backward");
        }
        let expected = ordinals.entry(source_sequence).or_default();
        if ordinal != *expected {
            return invalid("v2 normalized projection ordinals are not contiguous");
        }
        *expected = expected
            .checked_add(1)
            .ok_or_else(|| EncounterError::Validation("projection ordinal overflow".into()))?;
        previous_source_sequence = source_sequence;
    }
    Ok(())
}

fn projection_source_matches(event_kind: &str, source_id: &str) -> bool {
    match event_kind {
        "encounter-start" => source_id == "EVENT_PLAYER_COMBAT_STATE",
        "encounter-end" => source_id == "capture-finish",
        "damage" | "healing" => source_id == "EVENT_COMBAT_EVENT",
        "effect" => source_id == "EVENT_EFFECT_CHANGED",
        "resource" => source_id == "EVENT_POWER_UPDATE",
        "cast" => source_id == "EVENT_ACTION_SLOT_ABILITY_USED",
        "bar-change" => source_id == "EVENT_ACTIVE_WEAPON_PAIR_CHANGED",
        "death" => matches!(source_id, "EVENT_COMBAT_EVENT" | "EVENT_PLAYER_DEAD"),
        "resurrection" => {
            matches!(source_id, "EVENT_COMBAT_EVENT" | "EVENT_PLAYER_ALIVE")
        }
        "boss-health" => matches!(source_id, "EVENT_BOSSES_CHANGED" | "GetUnitPower"),
        "performance" => source_id == "GetFramerate",
        "quickslot" => matches!(
            source_id,
            "EVENT_ACTION_SLOT_ABILITY_USED" | "EVENT_ACTIVE_QUICKSLOT_CHANGED"
        ),
        "discontinuity" => matches!(source_id, "clock-reset" | "capture-finish"),
        _ => false,
    }
}

fn validate_event(
    capture: &EncounterCapture,
    event: &EncounterEvent,
) -> Result<(), EncounterError> {
    if event.session_id != capture.session_id || event.encounter_id != capture.encounter_id {
        return invalid("encounter event identity does not match its envelope");
    }
    if !EVENT_KINDS.contains(&event.kind.as_str()) {
        return invalid("encounter event kind is unsupported");
    }
    match event.kind.as_str() {
        "encounter-start" => {
            exact_keys(&event.payload, &["reason"])?;
            if string(&event.payload, "reason")? != "combat-started" {
                return invalid("encounter start reason is invalid");
            }
        }
        "encounter-end" => {
            exact_keys(&event.payload, &["complete", "reason"])?;
            boolean(&event.payload, "complete")?;
            terminal_reason(string(&event.payload, "reason")?, capture.schema_version)?;
        }
        "damage" | "healing" => numeric_payload_shape(
            &event.payload,
            &[
                "ability_id",
                "amount",
                "result",
                "source_actor",
                "target_actor",
            ],
            &[
                "damage_type",
                "overflow",
                "power_type",
                "source_type",
                "target_type",
            ],
        )?,
        "effect" => numeric_payload_shape(
            &event.payload,
            &["ability_id", "change_type", "stack_count", "target_actor"],
            &[
                "ability_type",
                "begin_ms",
                "effect_type",
                "end_ms",
                "source_type",
                "status_effect_type",
            ],
        )?,
        "resource" => numeric_payload(
            &event.payload,
            &[
                "actor",
                "effective_maximum",
                "maximum",
                "power_type",
                "value",
            ],
        )?,
        "cast" => numeric_payload(&event.payload, &["ability_id", "slot"])?,
        "bar-change" => {
            exact_keys(&event.payload, &["active_pair", "locked"])?;
            nonnegative(&event.payload, "active_pair")?;
            boolean(&event.payload, "locked")?;
        }
        "death" | "resurrection" => validate_life_payload(&event.payload)?,
        "boss-health" => {
            numeric_payload(
                &event.payload,
                &[
                    "actor",
                    "boss_index",
                    "effective_maximum",
                    "maximum",
                    "value",
                ],
            )?;
            if !(1..=6).contains(&integer(&event.payload, "boss_index")?) {
                return invalid("boss index is outside the supported range");
            }
        }
        "performance" => numeric_payload(&event.payload, &["frames_per_second", "latency_ms"])?,
        "quickslot" => {
            exact_keys(&event.payload, &["ability_id", "action", "slot"])?;
            numeric_payload_subset(&event.payload, &["ability_id", "slot"])?;
            if !matches!(string(&event.payload, "action")?, "selected" | "used") {
                return invalid("quickslot action is invalid");
            }
        }
        "discontinuity" => {
            exact_keys(
                &event.payload,
                &["missing_sequence_from", "missing_sequence_to", "reason"],
            )?;
            numeric_payload_subset(
                &event.payload,
                &["missing_sequence_from", "missing_sequence_to"],
            )?;
            if !matches!(
                string(&event.payload, "reason")?,
                "capture-overflow" | "clock-reset"
            ) {
                return invalid("discontinuity reason is invalid");
            }
        }
        _ => unreachable!(),
    }
    for actor_key in ["actor", "source_actor", "target_actor"] {
        if event.payload.contains_key(actor_key)
            && integer(&event.payload, actor_key)? as u64 > MAX_ACTORS
        {
            return invalid("encounter actor ID exceeds the capture bound");
        }
    }
    Ok(())
}

fn validate_life_payload(payload: &BTreeMap<String, PayloadValue>) -> Result<(), EncounterError> {
    match payload.get("source") {
        Some(PayloadValue::String(value)) if value == "player-event" => {
            exact_keys(payload, &["actor", "source"])?;
            nonnegative(payload, "actor")?;
        }
        Some(PayloadValue::String(value)) if value == "combat-result" => {
            exact_keys(
                payload,
                &["ability_id", "actor", "result", "source_actor", "source"],
            )?;
            numeric_payload_subset(payload, &["ability_id", "actor", "result", "source_actor"])?;
        }
        Some(PayloadValue::Integer(value)) if (0..=MAX_ACTORS as i64).contains(value) => {
            exact_keys(payload, &["actor", "source"])?;
            nonnegative(payload, "actor")?;
        }
        _ => return invalid("life-event source is invalid"),
    }
    Ok(())
}

fn numeric_payload(
    payload: &BTreeMap<String, PayloadValue>,
    keys: &[&str],
) -> Result<(), EncounterError> {
    exact_keys(payload, keys)?;
    numeric_payload_subset(payload, keys)
}

fn numeric_payload_shape(
    payload: &BTreeMap<String, PayloadValue>,
    required: &[&str],
    optional: &[&str],
) -> Result<(), EncounterError> {
    let actual = payload.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let required = required.iter().copied().collect::<BTreeSet<_>>();
    let allowed = required
        .iter()
        .copied()
        .chain(optional.iter().copied())
        .collect::<BTreeSet<_>>();
    if !required.is_subset(&actual) || !actual.is_subset(&allowed) {
        return invalid("encounter payload keys do not match the event contract");
    }
    for key in actual {
        nonnegative(payload, key)?;
    }
    Ok(())
}

fn numeric_payload_subset(
    payload: &BTreeMap<String, PayloadValue>,
    keys: &[&str],
) -> Result<(), EncounterError> {
    for key in keys {
        nonnegative(payload, key)?;
    }
    Ok(())
}

fn exact_keys(
    payload: &BTreeMap<String, PayloadValue>,
    expected: &[&str],
) -> Result<(), EncounterError> {
    let actual = payload.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        return invalid("encounter payload keys do not match the event contract");
    }
    Ok(())
}

fn nonnegative(payload: &BTreeMap<String, PayloadValue>, key: &str) -> Result<(), EncounterError> {
    if integer(payload, key)? < 0 {
        return invalid("encounter numeric payload value is negative");
    }
    Ok(())
}

fn integer(payload: &BTreeMap<String, PayloadValue>, key: &str) -> Result<i64, EncounterError> {
    match payload.get(key) {
        Some(PayloadValue::Integer(value)) => Ok(*value),
        _ => invalid("encounter payload value has the wrong type"),
    }
}

fn boolean(payload: &BTreeMap<String, PayloadValue>, key: &str) -> Result<bool, EncounterError> {
    match payload.get(key) {
        Some(PayloadValue::Bool(value)) => Ok(*value),
        _ => invalid("encounter payload value has the wrong type"),
    }
}

fn string<'a>(
    payload: &'a BTreeMap<String, PayloadValue>,
    key: &str,
) -> Result<&'a str, EncounterError> {
    match payload.get(key) {
        Some(PayloadValue::String(value)) => Ok(value),
        _ => invalid("encounter payload value has the wrong type"),
    }
}

fn terminal_reason(reason: &str, schema_version: u32) -> Result<(), EncounterError> {
    let legacy = matches!(
        reason,
        "combat-ended"
            | "capture-overflow"
            | "clock-reset"
            | "user-stopped"
            | "player-deactivated"
            | "callback-failed"
    );
    let raw = schema_version == 2
        && matches!(
            reason,
            "unsupported-value" | "record-limit" | "byte-limit" | "string-limit"
        );
    if legacy || raw {
        Ok(())
    } else {
        invalid("encounter terminal reason is invalid")
    }
}

fn validate_opaque_id(value: &str, prefix: &str) -> Result<(), EncounterError> {
    let Some(remainder) = value
        .strip_prefix(prefix)
        .and_then(|value| value.strip_prefix('-'))
    else {
        return invalid("encounter opaque identity is invalid");
    };
    let Some((left, right)) = remainder.split_once('-') else {
        return invalid("encounter opaque identity is invalid");
    };
    if left.is_empty()
        || right.is_empty()
        || right.contains('-')
        || !left.bytes().all(|byte| byte.is_ascii_digit())
        || !right.bytes().all(|byte| byte.is_ascii_digit())
    {
        return invalid("encounter opaque identity is invalid");
    }
    Ok(())
}

fn validate_decimal_time(value: &str) -> Result<u64, EncounterError> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return invalid("encounter wall-clock value is invalid");
    }
    value.parse::<u64>().map_err(|_| {
        EncounterError::Validation("encounter wall-clock value is out of range".into())
    })
}

fn safe_source_token(value: &str) -> bool {
    value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'+'))
}
