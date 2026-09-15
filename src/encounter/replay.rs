use std::collections::BTreeMap;

use serde::Serialize;

use super::model::{
    CaptureStatus, EncounterCapture, EncounterEvent, NormalizationProfile, PayloadValue,
    RawObservation, RawSourceKind, RawValue, RawValueType,
};
use super::{invalid, validate, EncounterError, MAX_ACTORS, MAX_EVENTS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReplayAssessment {
    Verified,
    Divergent,
    Indeterminate,
    Unavailable,
}

pub fn assess_replay(capture: &EncounterCapture) -> Result<ReplayAssessment, EncounterError> {
    validate::validate(capture)?;
    Ok(assess_validated(capture))
}

pub(crate) fn enforce(capture: &EncounterCapture) -> Result<(), EncounterError> {
    if capture.schema_version != 2 || capture.addon_version != 3 {
        return Ok(());
    }
    match (capture.status, assess_validated(capture)) {
        (CaptureStatus::Complete, ReplayAssessment::Verified)
        | (CaptureStatus::Partial, ReplayAssessment::Indeterminate) => Ok(()),
        (CaptureStatus::Complete, ReplayAssessment::Divergent) => {
            invalid("encounter replay diverged from supplied projections")
        }
        (CaptureStatus::Complete, ReplayAssessment::Indeterminate) => {
            invalid("encounter replay is indeterminate for a complete capture")
        }
        _ => invalid("encounter replay assessment is inconsistent with capture version"),
    }
}

fn assess_validated(capture: &EncounterCapture) -> ReplayAssessment {
    if capture.schema_version != 2 || capture.addon_version == 2 {
        return ReplayAssessment::Unavailable;
    }
    if capture.status == CaptureStatus::Partial
        || capture.raw_loss.is_some()
        || capture.raw_omitted_observation_count != Some(0)
        || capture.raw_observations.iter().any(|observation| {
            observation.source_kind == RawSourceKind::Lifecycle
                && observation.source_id == "clock-reset"
        })
    {
        return ReplayAssessment::Indeterminate;
    }
    let Some(profile) = capture.normalization_profile.as_ref() else {
        return ReplayAssessment::Unavailable;
    };
    match replay(capture, profile) {
        Ok(events) if events == capture.events => ReplayAssessment::Verified,
        Ok(_) => ReplayAssessment::Divergent,
        Err(()) => ReplayAssessment::Indeterminate,
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Scalar {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Lifecycle {
    Initial,
    Active,
    AwaitFinish {
        reason: &'static str,
        requested_complete: bool,
    },
    Finished,
}

struct Replayer<'a> {
    capture: &'a EncounterCapture,
    profile: &'a NormalizationProfile,
    events: Vec<EncounterEvent>,
    ordinals: BTreeMap<u64, u32>,
    actors: BTreeMap<String, i64>,
    boss_samples: [Option<(Scalar, Scalar, Scalar)>; 6],
    lifecycle: Lifecycle,
}

fn replay(
    capture: &EncounterCapture,
    profile: &NormalizationProfile,
) -> Result<Vec<EncounterEvent>, ()> {
    let mut state = Replayer {
        capture,
        profile,
        events: Vec::with_capacity(capture.events.len()),
        ordinals: BTreeMap::new(),
        actors: BTreeMap::new(),
        boss_samples: std::array::from_fn(|_| None),
        lifecycle: Lifecycle::Initial,
    };
    let raw = &capture.raw_observations;
    let mut index = 0;
    while index < raw.len() {
        let observation = &raw[index];
        match state.lifecycle {
            Lifecycle::Initial
                if !matches!(
                    (observation.source_kind, observation.source_id.as_str()),
                    (RawSourceKind::Callback, "EVENT_PLAYER_COMBAT_STATE")
                ) =>
            {
                return Err(());
            }
            Lifecycle::AwaitFinish { .. }
                if !matches!(
                    (observation.source_kind, observation.source_id.as_str()),
                    (RawSourceKind::Lifecycle, "capture-finish")
                ) =>
            {
                return Err(());
            }
            Lifecycle::Finished => return Err(()),
            _ => {}
        }
        match (observation.source_kind, observation.source_id.as_str()) {
            (RawSourceKind::Callback, "EVENT_PLAYER_COMBAT_STATE") => {
                let in_combat = truthy(value(observation, 2)?);
                match state.lifecycle {
                    Lifecycle::Initial if in_combat => {
                        state.record(
                            observation,
                            observation,
                            "encounter-start",
                            payload_string("reason", "combat-started"),
                        )?;
                        state.lifecycle = Lifecycle::Active;
                    }
                    Lifecycle::Active if !in_combat => {
                        state.lifecycle = Lifecycle::AwaitFinish {
                            reason: "combat-ended",
                            requested_complete: true,
                        };
                    }
                    Lifecycle::Active => {}
                    _ => return Err(()),
                }
            }
            (RawSourceKind::Callback, "EVENT_COMBAT_EVENT") => {
                state.combat_event(observation)?;
            }
            (RawSourceKind::Callback, "EVENT_EFFECT_CHANGED") => {
                state.effect_event(observation)?;
            }
            (RawSourceKind::Callback, "EVENT_POWER_UPDATE") => {
                state.power_event(observation)?;
            }
            (RawSourceKind::Callback, "EVENT_ACTION_SLOT_ABILITY_USED") => {
                index = state.action_slot(raw, index)?;
            }
            (RawSourceKind::Callback, "EVENT_ACTIVE_WEAPON_PAIR_CHANGED") => {
                let mut payload = BTreeMap::new();
                payload.insert(
                    "active_pair".into(),
                    integer_or_zero(value(observation, 2)?)?,
                );
                payload.insert(
                    "locked".into(),
                    PayloadValue::Bool(matches!(
                        scalar(value(observation, 3)?)?,
                        Scalar::Bool(true)
                    )),
                );
                state.record(observation, observation, "bar-change", payload)?;
            }
            (RawSourceKind::Callback, "EVENT_PLAYER_DEAD") => {
                state.player_life(observation, "death")?;
            }
            (RawSourceKind::Callback, "EVENT_PLAYER_ALIVE") => {
                state.player_life(observation, "resurrection")?;
            }
            (RawSourceKind::Callback, "EVENT_ACTIVE_QUICKSLOT_CHANGED") => {
                index = state.quickslot_changed(raw, index)?;
            }
            (RawSourceKind::Callback, "EVENT_BOSSES_CHANGED") => {
                index = state.boss_batch(raw, index + 1, Some(observation))?;
            }
            (RawSourceKind::Callback, "EVENT_PLAYER_DEACTIVATED") => {
                if state.lifecycle != Lifecycle::Active {
                    return Err(());
                }
                state.lifecycle = Lifecycle::AwaitFinish {
                    reason: "player-deactivated",
                    requested_complete: false,
                };
            }
            (RawSourceKind::ApiSample, "DoesUnitExist") => {
                index = state.boss_batch(raw, index, None)?;
            }
            (RawSourceKind::ApiSample, "GetFramerate") => {
                index = state.performance(raw, index)?;
            }
            (RawSourceKind::Lifecycle, "capture-finish") => {
                state.finish(observation)?;
            }
            (RawSourceKind::Callback, _) => return Err(()),
            (RawSourceKind::ApiSample, _) | (RawSourceKind::Lifecycle, _) => return Err(()),
        }
        index += 1;
    }
    if state.events.len() > MAX_EVENTS
        || state
            .events
            .first()
            .is_none_or(|event| event.kind != "encounter-start")
        || state
            .events
            .last()
            .is_none_or(|event| event.kind != "encounter-end")
        || state.lifecycle != Lifecycle::Finished
    {
        return Err(());
    }
    Ok(state.events)
}

impl Replayer<'_> {
    fn record(
        &mut self,
        timing: &RawObservation,
        projection: &RawObservation,
        kind: &str,
        payload: BTreeMap<String, PayloadValue>,
    ) -> Result<(), ()> {
        if self.events.len() >= MAX_EVENTS {
            return Err(());
        }
        let ordinal = self.ordinals.entry(projection.sequence).or_default();
        let event = EncounterEvent {
            session_id: self.capture.session_id.clone(),
            encounter_id: self.capture.encounter_id.clone(),
            sequence: u64::try_from(self.events.len()).map_err(|_| ())? + 1,
            monotonic_ms: timing.monotonic_ms,
            kind: kind.into(),
            payload,
            source_sequence: Some(projection.sequence),
            projection_ordinal: Some(*ordinal),
        };
        *ordinal = ordinal.checked_add(1).ok_or(())?;
        self.events.push(event);
        Ok(())
    }

    fn actor_for_key(&mut self, key: Option<String>) -> i64 {
        let Some(key) = key else { return 0 };
        if let Some(actor) = self.actors.get(&key) {
            return *actor;
        }
        if self.actors.len() >= MAX_ACTORS as usize {
            return 0;
        }
        let actor = self.actors.len() as i64 + 1;
        self.actors.insert(key, actor);
        actor
    }

    fn actor_for_unit_id(&mut self, raw: &RawValue) -> Result<i64, ()> {
        match scalar(raw)? {
            Scalar::Number(unit_id)
                if unit_id > 0.0
                    && unit_id.fract() == 0.0
                    && exact_integer_range().contains(&unit_id) =>
            {
                Ok(self.actor_for_key(Some(format!("unit:{}", unit_id as i64))))
            }
            Scalar::Nil | Scalar::Bool(_) | Scalar::String(_) | Scalar::Number(_) => Ok(0),
        }
    }

    fn actor_for_unit_tag(&mut self, raw: &RawValue) -> Result<i64, ()> {
        match scalar(raw)? {
            Scalar::String(tag) if tag == "player" => {
                Ok(self.actor_for_key(Some("role:player".into())))
            }
            Scalar::String(tag) if !tag.is_empty() => {
                Ok(self.actor_for_key(Some(format!("tag:{tag}"))))
            }
            Scalar::String(_) | Scalar::Nil | Scalar::Bool(_) | Scalar::Number(_) => Ok(0),
        }
    }

    fn actor_for_combat_unit(
        &mut self,
        unit_id: &RawValue,
        unit_type: &RawValue,
    ) -> Result<i64, ()> {
        if matches!(
            scalar(unit_type)?,
            Scalar::Number(value) if value == self.profile.player_combat_unit_type as f64
        ) {
            Ok(self.actor_for_key(Some("role:player".into())))
        } else {
            self.actor_for_unit_id(unit_id)
        }
    }

    fn combat_event(&mut self, observation: &RawObservation) -> Result<(), ()> {
        let result = integer_or_zero(value(observation, 2)?)?;
        let result_number = payload_integer(&result)?;
        let source_actor =
            self.actor_for_combat_unit(value(observation, 15)?, value(observation, 8)?)?;
        let target_actor =
            self.actor_for_combat_unit(value(observation, 16)?, value(observation, 10)?)?;
        let common = || -> Result<BTreeMap<String, PayloadValue>, ()> {
            Ok(BTreeMap::from([
                (
                    "ability_id".into(),
                    integer_or_zero(value(observation, 17)?)?,
                ),
                ("amount".into(), integer_or_zero(value(observation, 11)?)?),
                (
                    "damage_type".into(),
                    integer_or_zero(value(observation, 13)?)?,
                ),
                ("overflow".into(), integer_or_zero(value(observation, 18)?)?),
                (
                    "power_type".into(),
                    integer_or_zero(value(observation, 12)?)?,
                ),
                ("result".into(), result.clone()),
                ("source_actor".into(), PayloadValue::Integer(source_actor)),
                (
                    "source_type".into(),
                    integer_or_zero(value(observation, 8)?)?,
                ),
                ("target_actor".into(), PayloadValue::Integer(target_actor)),
                (
                    "target_type".into(),
                    integer_or_zero(value(observation, 10)?)?,
                ),
            ]))
        };
        if self.profile.damage_results.contains(&result_number) {
            self.record(observation, observation, "damage", common()?)
        } else if self.profile.healing_results.contains(&result_number) {
            self.record(observation, observation, "healing", common()?)
        } else if self.profile.death_results.contains(&result_number)
            || result_number == self.profile.resurrect_result
        {
            let payload = BTreeMap::from([
                (
                    "ability_id".into(),
                    integer_or_zero(value(observation, 17)?)?,
                ),
                ("actor".into(), PayloadValue::Integer(target_actor)),
                ("result".into(), result),
                (
                    "source".into(),
                    PayloadValue::String("combat-result".into()),
                ),
                ("source_actor".into(), PayloadValue::Integer(source_actor)),
            ]);
            let kind = if result_number == self.profile.resurrect_result {
                "resurrection"
            } else {
                "death"
            };
            self.record(observation, observation, kind, payload)
        } else {
            Ok(())
        }
    }

    fn effect_event(&mut self, observation: &RawObservation) -> Result<(), ()> {
        let actor = self.actor_for_unit_id(value(observation, 15)?)?;
        let actor = if actor == 0 {
            self.actor_for_unit_tag(value(observation, 5)?)?
        } else {
            actor
        };
        let payload = BTreeMap::from([
            (
                "ability_id".into(),
                integer_or_zero(value(observation, 16)?)?,
            ),
            (
                "ability_type".into(),
                integer_or_zero(value(observation, 12)?)?,
            ),
            (
                "begin_ms".into(),
                rounded_milliseconds(value(observation, 6)?)?,
            ),
            (
                "change_type".into(),
                integer_or_zero(value(observation, 2)?)?,
            ),
            (
                "effect_type".into(),
                integer_or_zero(value(observation, 11)?)?,
            ),
            (
                "end_ms".into(),
                rounded_milliseconds(value(observation, 7)?)?,
            ),
            (
                "source_type".into(),
                integer_or_zero(value(observation, 17)?)?,
            ),
            (
                "stack_count".into(),
                integer_or_zero(value(observation, 8)?)?,
            ),
            (
                "status_effect_type".into(),
                integer_or_zero(value(observation, 13)?)?,
            ),
            ("target_actor".into(), PayloadValue::Integer(actor)),
        ]);
        self.record(observation, observation, "effect", payload)
    }

    fn power_event(&mut self, observation: &RawObservation) -> Result<(), ()> {
        let actor = self.actor_for_unit_tag(value(observation, 2)?)?;
        let payload = BTreeMap::from([
            ("actor".into(), PayloadValue::Integer(actor)),
            (
                "effective_maximum".into(),
                integer_or_zero(value(observation, 7)?)?,
            ),
            ("maximum".into(), integer_or_zero(value(observation, 6)?)?),
            (
                "power_type".into(),
                integer_or_zero(value(observation, 4)?)?,
            ),
            ("value".into(), integer_or_zero(value(observation, 5)?)?),
        ]);
        self.record(observation, observation, "resource", payload)
    }

    fn action_slot(&mut self, raw: &[RawObservation], index: usize) -> Result<usize, ()> {
        let callback = &raw[index];
        let slot = value(callback, 2)?;
        let bound = api(raw.get(index + 1).ok_or(())?, "GetSlotBoundId", 1, 1)?;
        if !lua_equal(slot, value(bound, 1)?)? {
            return Err(());
        }
        self.record(
            callback,
            callback,
            "cast",
            BTreeMap::from([
                ("ability_id".into(), integer_or_zero(value(bound, 2)?)?),
                ("slot".into(), integer_or_zero(slot)?),
            ]),
        )?;
        let current = api(raw.get(index + 2).ok_or(())?, "GetCurrentQuickslot", 0, 1)?;
        if lua_equal(slot, value(current, 1)?)? {
            let quickslot = api(raw.get(index + 3).ok_or(())?, "GetSlotBoundId", 2, 1)?;
            if !lua_equal(slot, value(quickslot, 1)?)?
                || integer(value(quickslot, 2)?)? != self.profile.quickslot_category
            {
                return Err(());
            }
            self.record(
                callback,
                callback,
                "quickslot",
                BTreeMap::from([
                    ("ability_id".into(), integer_or_zero(value(quickslot, 3)?)?),
                    ("action".into(), PayloadValue::String("used".into())),
                    ("slot".into(), integer_or_zero(slot)?),
                ]),
            )?;
            Ok(index + 3)
        } else {
            Ok(index + 2)
        }
    }

    fn quickslot_changed(&mut self, raw: &[RawObservation], index: usize) -> Result<usize, ()> {
        let callback = &raw[index];
        let slot = value(callback, 2)?;
        let bound = api(raw.get(index + 1).ok_or(())?, "GetSlotBoundId", 2, 1)?;
        if !lua_equal(slot, value(bound, 1)?)?
            || integer(value(bound, 2)?)? != self.profile.quickslot_category
        {
            return Err(());
        }
        self.record(
            callback,
            callback,
            "quickslot",
            BTreeMap::from([
                ("ability_id".into(), integer_or_zero(value(bound, 3)?)?),
                ("action".into(), PayloadValue::String("selected".into())),
                ("slot".into(), integer_or_zero(slot)?),
            ]),
        )?;
        Ok(index + 1)
    }

    fn boss_batch(
        &mut self,
        raw: &[RawObservation],
        mut index: usize,
        parent: Option<&RawObservation>,
    ) -> Result<usize, ()> {
        for boss_index in 1..=6 {
            let exists = api(raw.get(index).ok_or(())?, "DoesUnitExist", 1, 1)?;
            let tag = format!("boss{boss_index}");
            if string(value(exists, 1)?)? != tag {
                return Err(());
            }
            let exists_value = matches!(scalar(value(exists, 2)?)?, Scalar::Bool(true));
            if exists_value {
                index += 1;
                let power = api(raw.get(index).ok_or(())?, "GetUnitPower", 2, 3)?;
                if string(value(power, 1)?)? != tag
                    || integer(value(power, 2)?)? != self.profile.health_power_type
                {
                    return Err(());
                }
                let sample = (
                    scalar(value(power, 3)?)?,
                    scalar(value(power, 4)?)?,
                    scalar(value(power, 5)?)?,
                );
                let slot = usize::try_from(boss_index - 1).map_err(|_| ())?;
                if self.boss_samples[slot].as_ref() != Some(&sample) {
                    self.boss_samples[slot] = Some(sample);
                    let actor = self.actor_for_key(Some(format!("tag:{tag}")));
                    let projection = parent.unwrap_or(power);
                    self.record(
                        power,
                        projection,
                        "boss-health",
                        BTreeMap::from([
                            ("actor".into(), PayloadValue::Integer(actor)),
                            ("boss_index".into(), PayloadValue::Integer(boss_index)),
                            (
                                "effective_maximum".into(),
                                integer_or_zero(value(power, 5)?)?,
                            ),
                            ("maximum".into(), integer_or_zero(value(power, 4)?)?),
                            ("value".into(), integer_or_zero(value(power, 3)?)?),
                        ]),
                    )?;
                }
            } else {
                self.boss_samples[usize::try_from(boss_index - 1).map_err(|_| ())?] = None;
            }
            if boss_index < 6 {
                index += 1;
            }
        }
        Ok(index)
    }

    fn performance(&mut self, raw: &[RawObservation], index: usize) -> Result<usize, ()> {
        let frames = api(&raw[index], "GetFramerate", 0, 1)?;
        let latency = api(raw.get(index + 1).ok_or(())?, "GetLatency", 0, 1)?;
        self.record(
            frames,
            frames,
            "performance",
            BTreeMap::from([
                (
                    "frames_per_second".into(),
                    rounded_integer(value(frames, 1)?)?,
                ),
                ("latency_ms".into(), integer_or_zero(value(latency, 1)?)?),
            ]),
        )?;
        Ok(index + 1)
    }

    fn player_life(&mut self, observation: &RawObservation, kind: &str) -> Result<(), ()> {
        let actor = self.actor_for_key(Some("role:player".into()));
        self.record(
            observation,
            observation,
            kind,
            BTreeMap::from([
                ("actor".into(), PayloadValue::Integer(actor)),
                ("source".into(), PayloadValue::String("player-event".into())),
            ]),
        )
    }

    fn finish(&mut self, observation: &RawObservation) -> Result<(), ()> {
        let reason = string(value(observation, 1)?)?;
        let complete = match scalar(value(observation, 2)?)? {
            Scalar::Bool(value) => value,
            _ => return Err(()),
        };
        let Lifecycle::AwaitFinish {
            reason: expected_reason,
            requested_complete,
        } = self.lifecycle
        else {
            return Err(());
        };
        if reason != expected_reason || complete != requested_complete {
            return Err(());
        }
        self.record(
            observation,
            observation,
            "encounter-end",
            BTreeMap::from([
                ("complete".into(), PayloadValue::Bool(complete)),
                ("reason".into(), PayloadValue::String(reason)),
            ]),
        )?;
        self.lifecycle = Lifecycle::Finished;
        Ok(())
    }
}

fn api<'a>(
    observation: &'a RawObservation,
    source_id: &str,
    argument_count: usize,
    return_count: usize,
) -> Result<&'a RawObservation, ()> {
    if observation.source_kind == RawSourceKind::ApiSample
        && observation.source_id == source_id
        && observation.argument_count == argument_count
        && observation.return_count == return_count
    {
        Ok(observation)
    } else {
        Err(())
    }
}

fn value(observation: &RawObservation, position: usize) -> Result<&RawValue, ()> {
    observation
        .values
        .get(position.checked_sub(1).ok_or(())?)
        .ok_or(())
}

fn scalar(value: &RawValue) -> Result<Scalar, ()> {
    match value.value_type {
        RawValueType::Nil => Ok(Scalar::Nil),
        RawValueType::Boolean => Ok(Scalar::Bool(value.boolean.ok_or(())?)),
        RawValueType::String => Ok(Scalar::String(value.string.clone().ok_or(())?)),
        RawValueType::Number => {
            let sign = value.sign.ok_or(())?;
            let significand = value
                .significand
                .as_ref()
                .ok_or(())?
                .parse::<u64>()
                .map_err(|_| ())?;
            let exponent = i32::from(value.exponent.ok_or(())?);
            let magnitude = (significand as f64) * 2_f64.powi(exponent);
            let number = if sign < 0 { -magnitude } else { magnitude };
            if number.is_finite() {
                Ok(Scalar::Number(number))
            } else {
                Err(())
            }
        }
    }
}

fn integer(value: &RawValue) -> Result<i64, ()> {
    match scalar(value)? {
        Scalar::Number(number)
            if number.fract() == 0.0 && exact_integer_range().contains(&number) =>
        {
            Ok(number as i64)
        }
        _ => Err(()),
    }
}

fn integer_or_zero(value: &RawValue) -> Result<PayloadValue, ()> {
    match scalar(value)? {
        Scalar::Nil | Scalar::Bool(false) => Ok(PayloadValue::Integer(0)),
        Scalar::Number(number)
            if number.fract() == 0.0 && exact_integer_range().contains(&number) =>
        {
            Ok(PayloadValue::Integer(number as i64))
        }
        _ => Err(()),
    }
}

fn payload_integer(value: &PayloadValue) -> Result<i64, ()> {
    match value {
        PayloadValue::Integer(value) => Ok(*value),
        _ => Err(()),
    }
}

fn rounded_milliseconds(value: &RawValue) -> Result<PayloadValue, ()> {
    match scalar(value)? {
        Scalar::Nil | Scalar::Bool(false) => Ok(PayloadValue::Integer(0)),
        Scalar::Number(number) => checked_floor(number * 1000.0 + 0.5),
        _ => Err(()),
    }
}

fn rounded_integer(value: &RawValue) -> Result<PayloadValue, ()> {
    match scalar(value)? {
        Scalar::Nil | Scalar::Bool(false) => Ok(PayloadValue::Integer(0)),
        Scalar::Number(number) => checked_floor(number + 0.5),
        _ => Err(()),
    }
}

fn checked_floor(number: f64) -> Result<PayloadValue, ()> {
    let number = number.floor();
    if number.is_finite() && exact_integer_range().contains(&number) {
        Ok(PayloadValue::Integer(number as i64))
    } else {
        Err(())
    }
}

fn exact_integer_range() -> std::ops::RangeInclusive<f64> {
    -9_007_199_254_740_991_f64..=9_007_199_254_740_991_f64
}

fn string(value: &RawValue) -> Result<String, ()> {
    match scalar(value)? {
        Scalar::String(value) => Ok(value),
        _ => Err(()),
    }
}

fn truthy(value: &RawValue) -> bool {
    !matches!(scalar(value), Ok(Scalar::Nil | Scalar::Bool(false)))
}

fn lua_equal(left: &RawValue, right: &RawValue) -> Result<bool, ()> {
    Ok(match (scalar(left)?, scalar(right)?) {
        (Scalar::Nil, Scalar::Nil) => true,
        (Scalar::Bool(left), Scalar::Bool(right)) => left == right,
        (Scalar::String(left), Scalar::String(right)) => left == right,
        (Scalar::Number(left), Scalar::Number(right)) => left == right,
        _ => false,
    })
}

fn payload_string(key: &str, value: &str) -> BTreeMap<String, PayloadValue> {
    BTreeMap::from([(key.into(), PayloadValue::String(value.into()))])
}
