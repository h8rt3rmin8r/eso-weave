use std::collections::{BTreeMap, BTreeSet};

use super::model::{CaptureStatus, EncounterCapture, EncounterEvent, PayloadValue};
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
    if capture.schema_version != 1 || capture.addon_version != 1 {
        return invalid("unsupported encounter capture version");
    }
    if capture.privacy_profile != "anonymous-local-v1" {
        return invalid("unsupported encounter privacy profile");
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
            }
        }
        previous_sequence = event.sequence;
        previous_time = event.monotonic_ms;
    }
    if declared_omissions != capture.omitted_event_count {
        return invalid("encounter omission count does not match discontinuities");
    }

    let terminal_reason = string(&last.payload, "reason")?;
    let terminal_complete = boolean(&last.payload, "complete")?;
    match capture.status {
        CaptureStatus::Complete => {
            if capture.partial_reason.is_some()
                || capture.omitted_event_count != 0
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
        }
    }
    Ok(())
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
            terminal_reason(string(&event.payload, "reason")?)?;
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

fn terminal_reason(reason: &str) -> Result<(), EncounterError> {
    if matches!(
        reason,
        "combat-ended"
            | "capture-overflow"
            | "clock-reset"
            | "user-stopped"
            | "player-deactivated"
            | "callback-failed"
    ) {
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
