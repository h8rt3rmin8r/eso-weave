use std::collections::BTreeSet;

use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    assess_replay, canonical_bytes, import_capture_set, import_encounter, list_encounters,
    load_encounter, parse_capture, parse_capture_set, ImportRequest, ReplayAssessment,
    MAX_ESTIMATED_BYTES, MAX_EVENTS,
};
use mlua::{Lua, Table, Value as LuaValue};
use serde_json::Value;

const MANIFEST: &str = include_str!("../addon/EsoWeaveData/EsoWeaveData.txt");
const BOOTSTRAP: &str = include_str!("../addon/EsoWeaveData/EsoWeaveData.lua");
const ADDON: &str = include_str!("../addon/EsoWeaveData/Encounter.lua");
const FIXTURE: &str =
    include_str!("../specs/075-encounter-capture/fixtures/representative-capture.json");

const HARNESS: &str = r#"
EVENT_ADD_ON_LOADED = 1
EVENT_PLAYER_COMBAT_STATE = 2
EVENT_COMBAT_EVENT = 3
EVENT_EFFECT_CHANGED = 4
EVENT_POWER_UPDATE = 5
EVENT_ACTION_SLOT_ABILITY_USED = 6
EVENT_ACTIVE_WEAPON_PAIR_CHANGED = 7
EVENT_PLAYER_DEAD = 8
EVENT_PLAYER_ALIVE = 9
EVENT_BOSSES_CHANGED = 10
EVENT_ACTIVE_QUICKSLOT_CHANGED = 11
EVENT_PLAYER_DEACTIVATED = 12

ACTION_RESULT_DAMAGE = 100
ACTION_RESULT_CRITICAL_DAMAGE = 101
ACTION_RESULT_DOT_TICK = 102
ACTION_RESULT_DOT_TICK_CRITICAL = 103
ACTION_RESULT_BLOCKED_DAMAGE = 104
ACTION_RESULT_DAMAGE_SHIELDED = 105
ACTION_RESULT_HEAL = 200
ACTION_RESULT_CRITICAL_HEAL = 201
ACTION_RESULT_HOT_TICK = 202
ACTION_RESULT_HOT_TICK_CRITICAL = 203
ACTION_RESULT_DIED = 300
ACTION_RESULT_DIED_XP = 301
ACTION_RESULT_RESURRECT = 302

POWERTYPE_HEALTH = 0
HOTBAR_CATEGORY_PRIMARY = 1
HOTBAR_CATEGORY_QUICKSLOT_WHEEL = 9
COMBAT_UNIT_TYPE_PLAYER = 1

__now = 1000
__timestamp = 1788912000
__messages = {}
__slot_ids = { [3] = 7003, [4] = 7004 }
__quickslot_ids = { [4] = 9004 }
__boss_exists = true
__boss_value = 5000
__boss_max = 10000
__fps = 60.4
__latency = 45
__in_combat = false

EVENT_MANAGER = { events = {}, updates = {} }
function EVENT_MANAGER:RegisterForEvent(namespace, event, callback)
    self.events[event] = { namespace = namespace, callback = callback }
end
function EVENT_MANAGER:UnregisterForEvent(namespace, event)
    local entry = self.events[event]
    if entry and entry.namespace == namespace then self.events[event] = nil end
end
function EVENT_MANAGER:RegisterForUpdate(namespace, interval, callback)
    self.updates[namespace] = { interval = interval, callback = callback }
end
function EVENT_MANAGER:UnregisterForUpdate(namespace)
    self.updates[namespace] = nil
end

SLASH_COMMANDS = {}
function d(message) table.insert(__messages, message) end
function GetGameTimeMilliseconds() return __now end
function GetTimeStamp() return __timestamp end
function GetAPIVersion() return 101050 end
function GetESOVersionString() return "12.0.7" end
function GetCVar(name) if name == "language.2" then return "en" end return "" end
function GetPlatformServiceType() return 1 end
function IsUnitInCombat(tag) return __in_combat end
function GetSlotBoundId(slot, hotbar)
    if hotbar == HOTBAR_CATEGORY_QUICKSLOT_WHEEL then
        return __quickslot_ids[slot] or 0
    end
    return __slot_ids[slot] or 0
end
function GetCurrentQuickslot() return 4 end
function DoesUnitExist(tag) return tag == "boss1" and __boss_exists end
function GetUnitPower(tag, powerType) return __boss_value, __boss_max, __boss_max end
function GetFramerate() return __fps end
function GetLatency() return __latency end

function __fire(event, ...)
    local entry = EVENT_MANAGER.events[event]
    if entry then return entry.callback(event, ...) end
end
function __advance(milliseconds) __now = __now + milliseconds end
function __run_updates()
    local callbacks = {}
    for _, entry in pairs(EVENT_MANAGER.updates) do table.insert(callbacks, entry.callback) end
    for _, callback in ipairs(callbacks) do callback() end
end
function __contains_saved_string(needle)
    local seen = {}
    local function visit(value)
        if type(value) == "string" then return string.find(value, needle, 1, true) ~= nil end
        if type(value) ~= "table" or seen[value] then return false end
        seen[value] = true
        for key, child in pairs(value) do
            if visit(key) or visit(child) then return true end
        end
        return false
    end
    return visit(EsoWeaveDataSaved.encounter)
end
function __capture()
    local saved = EsoWeaveDataSaved.encounter
    if saved.state_schema_version ~= 1 then return saved end
    if saved.current then return saved.current end
    if saved.session and saved.session.completed_encounter_count > 0 then
        local key = string.format("%010d", saved.session.completed_encounter_count)
        return saved.records[key].capture
    end
    return nil
end
function __raw_observation(sourceId, occurrence)
    occurrence = occurrence or 1
    local observations = __capture().raw_observations
    assert(type(observations) == "table", "raw_observations is missing")
    local found = 0
    for _, observation in ipairs(observations) do
        if observation.source_id == sourceId then
            found = found + 1
            if found == occurrence then return observation end
        end
    end
    error("missing raw observation " .. sourceId .. " #" .. tostring(occurrence))
end
function __assert_raw_callback(observation, sourceId, sourceCode, argumentCount)
    assert(observation.source_kind == "callback")
    assert(observation.source_id == sourceId)
    assert(observation.source_code == sourceCode)
    assert(observation.source_version == 1)
    assert(observation.api_version == GetAPIVersion())
    assert(observation.argument_count == argumentCount)
    assert(observation.return_count == 0)
    assert(#observation.values == argumentCount)
end
function __assert_tagged_nil(observation, position)
    local value = observation.values[position]
    assert(value.position == position)
    assert(value.value_type == "nil")
    assert(value.boolean == nil and value.string == nil)
    assert(value.sign == nil and value.significand == nil and value.exponent == nil)
end
function __assert_tagged_boolean(observation, position, expected)
    local value = observation.values[position]
    assert(value.position == position)
    assert(value.value_type == "boolean")
    assert(value.boolean == expected)
end
function __assert_tagged_string(observation, position, expected)
    local value = observation.values[position]
    assert(value.position == position)
    assert(value.value_type == "string")
    assert(value.string == expected)
end
function __tagged_number(observation, position)
    local value = observation.values[position]
    assert(value.position == position)
    assert(value.value_type == "number")
    assert(value.sign == 1 or value.sign == -1)
    assert(type(value.significand) == "string")
    assert(string.match(value.significand, "^%d+$") ~= nil)
    assert(type(value.exponent) == "number" and value.exponent == math.floor(value.exponent))
    local magnitude = tonumber(value.significand) * (2 ^ value.exponent)
    return value.sign == -1 and -magnitude or magnitude, value
end
function __assert_tagged_number(observation, position, expected)
    local actual = __tagged_number(observation, position)
    assert(actual == expected, tostring(actual) .. " ~= " .. tostring(expected))
end
function __assert_tagged_negative_zero(observation, position)
    local actual, value = __tagged_number(observation, position)
    assert(actual == 0)
    assert(value.sign == -1)
    assert(value.significand == "0")
    assert(value.exponent == 0)
end
function __projection_count(sourceSequence)
    local count = 0
    for _, event in ipairs(__capture().events) do
        if event.source_sequence == sourceSequence then count = count + 1 end
    end
    return count
end
function __assert_raw_loss(reason)
    local saved = __capture()
    local loss = saved.raw_loss
    assert(saved.status == "partial")
    assert(type(loss) == "table")
    assert(loss.reason == reason)
    assert(loss.missing_sequence_from >= 1)
    assert(loss.missing_sequence_to >= loss.missing_sequence_from)
    assert(loss.missing_sequence_to - loss.missing_sequence_from + 1
        == saved.raw_omitted_observation_count)
    assert(__raw_observation("capture-finish").source_kind == "lifecycle")
    assert(saved.events[#saved.events].kind == "encounter-end")
end
"#;

fn harness(prelude: &str) -> Lua {
    let lua = Lua::new();
    lua.load(HARNESS).exec().expect("install ESO API harness");
    lua.load(BOOTSTRAP)
        .exec()
        .expect("load data addon bootstrap");
    lua.load("__fire(EVENT_ADD_ON_LOADED, 'EsoWeaveData')")
        .exec()
        .expect("activate data addon bootstrap");
    lua.load(prelude).exec().expect("install test overrides");
    lua.load(ADDON).exec().expect("load encounter addon");
    lua.load("__fire(EVENT_ADD_ON_LOADED, 'EsoWeaveData')")
        .exec()
        .expect("activate encounter addon");
    lua
}

fn run(lua: &Lua, script: &str) {
    lua.load(script).exec().expect("run encounter scenario");
}

fn saved_variables_value(value: LuaValue) -> String {
    match value {
        LuaValue::Nil => "nil".into(),
        LuaValue::Boolean(value) => value.to_string(),
        LuaValue::Integer(value) => value.to_string(),
        LuaValue::Number(value) if value.is_finite() && value.fract() == 0.0 => {
            format!("{value:.0}")
        }
        LuaValue::String(value) => {
            let text = value.to_str().unwrap();
            serde_json::to_string(text.as_ref()).unwrap()
        }
        LuaValue::Table(table) => saved_variables_table(table),
        _ => panic!("production SavedVariables contains an unsupported Lua value"),
    }
}

fn saved_variables_table(table: Table) -> String {
    let mut entries = table
        .pairs::<LuaValue, LuaValue>()
        .map(|entry| {
            let (key, value) = entry.unwrap();
            let key = match key {
                LuaValue::Integer(value) => (0_u8, value.to_string(), format!("[{value}]")),
                LuaValue::Number(value) if value.fract() == 0.0 => {
                    let text = format!("{value:.0}");
                    (0, text.clone(), format!("[{text}]"))
                }
                LuaValue::String(value) => {
                    let text = value.to_str().unwrap().to_owned();
                    (
                        1,
                        text.clone(),
                        format!("[{}]", serde_json::to_string(&text).unwrap()),
                    )
                }
                _ => panic!("production SavedVariables contains an unsupported Lua key"),
            };
            (key, saved_variables_value(value))
        })
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    format!(
        "{{{}}}",
        entries
            .into_iter()
            .map(|((_, _, key), value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn serialized_saved_variables(lua: &Lua) -> String {
    let root: Table = lua.globals().get("EsoWeaveDataSaved").unwrap();
    format!("EsoWeaveDataSaved = {}", saved_variables_table(root))
}

fn assert_selected_sources_round_trip_through_sqlite(lua: &Lua) {
    let source = serialized_saved_variables(lua);
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    assert_eq!(capture.addon_version, 3);
    assert_eq!(
        assess_replay(&capture).unwrap(),
        ReplayAssessment::Indeterminate
    );
    let source_ids = capture
        .raw_observations
        .iter()
        .map(|observation| observation.source_id.as_str())
        .collect::<BTreeSet<_>>();
    for required in [
        "EVENT_PLAYER_COMBAT_STATE",
        "EVENT_COMBAT_EVENT",
        "EVENT_EFFECT_CHANGED",
        "EVENT_POWER_UPDATE",
        "EVENT_ACTION_SLOT_ABILITY_USED",
        "EVENT_ACTIVE_WEAPON_PAIR_CHANGED",
        "EVENT_PLAYER_DEAD",
        "EVENT_PLAYER_ALIVE",
        "EVENT_BOSSES_CHANGED",
        "EVENT_ACTIVE_QUICKSLOT_CHANGED",
        "GetSlotBoundId",
        "GetCurrentQuickslot",
        "DoesUnitExist",
        "GetUnitPower",
        "GetFramerate",
        "GetLatency",
        "clock-reset",
        "capture-finish",
    ] {
        assert!(source_ids.contains(required), "missing {required}");
    }
    let expected = canonical_bytes(&capture).unwrap();
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("capture.lua");
    let store = sandbox.path().join("encounters.sqlite");
    std::fs::write(&input, source).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    let loaded = load_encounter(&store, &receipt.session_id, &receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(canonical_bytes(&loaded).unwrap(), expected);
}

#[test]
fn complete_current_capture_is_verified_from_raw_observations() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let source = serialized_saved_variables(&lua);
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    assert_eq!(capture.addon_version, 3);
    assert_eq!(assess_replay(&capture).unwrap(), ReplayAssessment::Verified);
}

#[test]
fn complete_current_capture_differentially_replays_selected_projection_families() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __advance(10)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "DamageCanary",
            0, 0, "SourceCanary", 1, "TargetCanary", 2, 1200, 3, 4,
            true, 9001, 9002, 7001, 50)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_HEAL, false, "HealCanary",
            0, 0, "SourceCanary", 1, "TargetCanary", 2, 300, 3, 0,
            true, 9001, 9001, 7002, 25)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DIED, false, "DeathCanary",
            0, 0, "SourceCanary", 1, "TargetCanary", 2, 0, 0, 0,
            true, 9001, 9002, 7003, 0)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_RESURRECT, false, "RezCanary",
            0, 0, "SourceCanary", 1, "TargetCanary", 2, 0, 0, 0,
            true, 9001, 9002, 7004, 0)
        __fire(EVENT_EFFECT_CHANGED, 1, 2, "EffectCanary", "group1", 1.1,
            2.2, 3, "icon", "deprecated", 4, 5, 6, "UnitCanary",
            9001, 7100, 1)
        __fire(EVENT_POWER_UPDATE, "group1", 1, 3, 800, 1000, 1000)
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 3)
        __fire(EVENT_ACTIVE_WEAPON_PAIR_CHANGED, 2, false)
        __fire(EVENT_PLAYER_DEAD)
        __fire(EVENT_PLAYER_ALIVE)
        __fire(EVENT_BOSSES_CHANGED, false)
        __fire(EVENT_ACTIVE_QUICKSLOT_CHANGED, 4)
        __advance(1000)
        __run_updates()
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 4)
        __boss_value = 4000
        __advance(1000)
        __run_updates()
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(__capture().status == "complete")
        "#,
    );
    let source = serialized_saved_variables(&lua);
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    let profile = capture.normalization_profile.as_ref().unwrap();
    assert_eq!(profile.version, 1);
    assert_eq!(profile.api_version, capture.source.api_version);
    assert_eq!(profile.damage_results, [100, 101, 102, 103, 104, 105]);
    assert_eq!(assess_replay(&capture).unwrap(), ReplayAssessment::Verified);

    let mut unsupported_profile = capture.clone();
    unsupported_profile
        .normalization_profile
        .as_mut()
        .unwrap()
        .version = 2;
    assert!(assess_replay(&unsupported_profile).is_err());

    let mut overlapping_profile = capture.clone();
    overlapping_profile
        .normalization_profile
        .as_mut()
        .unwrap()
        .healing_results[0] = 100;
    assert!(assess_replay(&overlapping_profile).is_err());

    let mut missing_profile = capture.clone();
    missing_profile.normalization_profile = None;
    assert!(assess_replay(&missing_profile).is_err());

    let mut malformed_batch = capture.clone();
    malformed_batch
        .raw_observations
        .iter_mut()
        .find(|observation| observation.source_id == "GetLatency")
        .unwrap()
        .source_id = "GetFramerate".into();
    assert_eq!(
        assess_replay(&malformed_batch).unwrap(),
        ReplayAssessment::Indeterminate
    );
    assert!(canonical_bytes(&malformed_batch)
        .unwrap_err()
        .to_string()
        .contains("indeterminate"));

    let assert_validation_rejected = |name: &str, candidate| {
        assert!(assess_replay(&candidate).is_err(), "{name}");
        assert!(canonical_bytes(&candidate).is_err(), "{name}");
    };
    let assert_divergent = |name: &str, candidate| {
        let assessment =
            assess_replay(&candidate).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(assessment, ReplayAssessment::Divergent, "{name}");
        assert!(canonical_bytes(&candidate).is_err(), "{name}");
    };
    let assert_indeterminate = |name: &str, candidate| {
        assert_eq!(
            assess_replay(&candidate).unwrap(),
            ReplayAssessment::Indeterminate,
            "{name}"
        );
        assert!(canonical_bytes(&candidate).is_err(), "{name}");
    };

    let damage_index = capture
        .events
        .iter()
        .position(|event| event.kind == "damage")
        .unwrap();
    let mut changed_source = capture.clone();
    changed_source.events[damage_index].source_sequence = Some(999_999);
    assert_validation_rejected("source sequence", changed_source);

    let mut changed_ordinal = capture.clone();
    changed_ordinal.events[damage_index].projection_ordinal = Some(1);
    assert_validation_rejected("projection ordinal", changed_ordinal);

    let mut changed_time = capture.clone();
    changed_time.events[damage_index].monotonic_ms += 1;
    assert_validation_rejected("monotonic time", changed_time);

    let mut changed_kind = capture.clone();
    changed_kind.events[damage_index].kind = "healing".into();
    assert_divergent("kind", changed_kind);

    let mut changed_payload = capture.clone();
    changed_payload.events[damage_index].payload.insert(
        "amount".into(),
        eso_weave::encounter::PayloadValue::Integer(991_337),
    );
    assert_divergent("payload", changed_payload);

    let mut omitted = capture.clone();
    omitted.events.remove(damage_index);
    omitted.stored_event_count -= 1;
    omitted.last_sequence -= 1;
    for (index, event) in omitted.events.iter_mut().enumerate() {
        event.sequence = index as u64 + 1;
    }
    assert_divergent("omitted projection", omitted);

    let mut inserted = capture.clone();
    let mut extra = inserted.events[damage_index].clone();
    extra.projection_ordinal = Some(1);
    inserted.events.insert(damage_index + 1, extra);
    inserted.stored_event_count += 1;
    inserted.last_sequence += 1;
    for (index, event) in inserted.events.iter_mut().enumerate() {
        event.sequence = index as u64 + 1;
    }
    assert_divergent("extra projection", inserted);

    let mut reordered = capture.clone();
    let reorder_index = reordered
        .events
        .windows(2)
        .position(|events| {
            events[0].source_sequence == events[1].source_sequence
                && events[0].projection_ordinal == Some(0)
                && events[1].projection_ordinal == Some(1)
                && events[0].kind != events[1].kind
        })
        .unwrap();
    reordered.events.swap(reorder_index, reorder_index + 1);
    reordered.events[reorder_index].projection_ordinal = Some(0);
    reordered.events[reorder_index + 1].projection_ordinal = Some(1);
    for (index, event) in reordered.events.iter_mut().enumerate() {
        event.sequence = index as u64 + 1;
    }
    assert_divergent("projection order", reordered);

    let stop_index = capture
        .raw_observations
        .iter()
        .rposition(|observation| observation.source_id == "EVENT_PLAYER_COMBAT_STATE")
        .unwrap();
    let mut deactivated_complete = capture.clone();
    deactivated_complete.raw_observations[stop_index].source_id = "EVENT_PLAYER_DEACTIVATED".into();
    deactivated_complete.raw_observations[stop_index].argument_count = 1;
    deactivated_complete.raw_observations[stop_index]
        .values
        .truncate(1);
    assert_indeterminate("complete capture after deactivation", deactivated_complete);

    let mut continued_after_stop = capture.clone();
    continued_after_stop.raw_observations[stop_index].values[1].boolean = Some(true);
    assert_indeterminate("finish without a stopping transition", continued_after_stop);

    let mut duplicate_finish = capture.clone();
    let mut extra_finish = duplicate_finish.raw_observations.last().unwrap().clone();
    extra_finish.sequence += 1;
    duplicate_finish.raw_last_sequence = Some(extra_finish.sequence);
    duplicate_finish.raw_observation_count = duplicate_finish
        .raw_observation_count
        .map(|count| count + 1);
    duplicate_finish.raw_observations.push(extra_finish);
    assert_indeterminate("duplicate finish", duplicate_finish);

    let mut divergent = capture;
    let damage = divergent
        .events
        .iter_mut()
        .find(|event| event.kind == "damage")
        .unwrap();
    damage.payload.insert(
        "amount".into(),
        eso_weave::encounter::PayloadValue::Integer(991_337),
    );
    assert_eq!(
        assess_replay(&divergent).unwrap(),
        ReplayAssessment::Divergent
    );
    let error = canonical_bytes(&divergent).unwrap_err().to_string();
    assert!(error.contains("replay diverged"));
    for canary in ["991337", "DamageCanary", "SourceCanary", "TargetCanary"] {
        assert!(!error.contains(canary));
    }

    run(
        &lua,
        r#"
        for _, event in ipairs(__capture().events) do
            if event.kind == "damage" then event.payload.amount = 991337 break end
        end
        "#,
    );
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("divergent.lua");
    let store = sandbox.path().join("encounters.sqlite");
    std::fs::write(&input, serialized_saved_variables(&lua)).unwrap();
    let error = import_encounter(&ImportRequest::new(&input, &store, Channel::Live))
        .unwrap_err()
        .to_string();
    assert!(error.contains("replay diverged"));
    assert!(!store.exists());
}

#[test]
fn replay_matches_lua_actor_type_guards_for_nil_and_non_numeric_ids() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "DamageCanary",
            0, 0, "SourceCanary", 0, "TargetCanary", 0, 100, 0, 0,
            false, nil, "not-a-number", 7001, 0)
        __fire(EVENT_EFFECT_CHANGED, 1, 2, "EffectCanary", false, 1.1,
            2.2, 3, "icon", "deprecated", 4, 5, 6, "UnitCanary",
            nil, 7100, 1)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let capture =
        parse_capture(serialized_saved_variables(&lua).as_bytes(), Channel::Live).unwrap();
    assert_eq!(assess_replay(&capture).unwrap(), ReplayAssessment::Verified);
    let damage = capture
        .events
        .iter()
        .find(|event| event.kind == "damage")
        .unwrap();
    assert_eq!(
        damage.payload["source_actor"],
        eso_weave::encounter::PayloadValue::Integer(0)
    );
    assert_eq!(
        damage.payload["target_actor"],
        eso_weave::encounter::PayloadValue::Integer(0)
    );
    let effect = capture
        .events
        .iter()
        .find(|event| event.kind == "effect")
        .unwrap();
    assert_eq!(
        effect.payload["target_actor"],
        eso_weave::encounter::PayloadValue::Integer(0)
    );
}

#[test]
fn replay_constrains_nonintegral_unit_ids_before_actor_keying() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "First",
            0, 0, "Source", 0, "Target", 0, 100, 0, 0,
            false, 1.0000000000000002, nil, 7001, 0)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "Second",
            0, 0, "Source", 0, "Target", 0, 100, 0, 0,
            false, 1.0000000000000004, nil, 7002, 0)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let capture =
        parse_capture(serialized_saved_variables(&lua).as_bytes(), Channel::Live).unwrap();
    assert_eq!(assess_replay(&capture).unwrap(), ReplayAssessment::Verified);
    let actors = capture
        .events
        .iter()
        .filter(|event| event.kind == "damage")
        .map(|event| event.payload["source_actor"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        actors,
        vec![eso_weave::encounter::PayloadValue::Integer(0); 2]
    );
}

#[test]
fn replay_uses_exact_decimal_keys_for_adjacent_large_unit_ids() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "First",
            0, 0, "Source", 0, "Target", 0, 100, 0, 0,
            false, 1000000000000000, nil, 7001, 0)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "Second",
            0, 0, "Source", 0, "Target", 0, 100, 0, 0,
            false, 1000000000000001, nil, 7002, 0)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let capture =
        parse_capture(serialized_saved_variables(&lua).as_bytes(), Channel::Live).unwrap();
    assert_eq!(assess_replay(&capture).unwrap(), ReplayAssessment::Verified);
    let actors = capture
        .events
        .iter()
        .filter(|event| event.kind == "damage")
        .map(|event| event.payload["source_actor"].clone())
        .collect::<Vec<_>>();
    assert_eq!(
        actors,
        vec![
            eso_weave::encounter::PayloadValue::Integer(1),
            eso_weave::encounter::PayloadValue::Integer(2),
        ]
    );
}

#[test]
fn replay_matches_nil_quickslot_and_boss_power_defaults() {
    let lua = harness(
        r#"
        GetCurrentQuickslot = function() return nil end
        GetUnitPower = function() return nil, nil, nil end
        "#,
    );
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 3)
        __fire(EVENT_BOSSES_CHANGED, false)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let capture =
        parse_capture(serialized_saved_variables(&lua).as_bytes(), Channel::Live).unwrap();
    assert_eq!(assess_replay(&capture).unwrap(), ReplayAssessment::Verified);
}

#[test]
fn replay_is_bounded_at_the_raw_observation_ceiling() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let mut capture =
        parse_capture(serialized_saved_variables(&lua).as_bytes(), Channel::Live).unwrap();
    let start = capture.raw_observations[0].clone();
    let mut intermediate = start.clone();
    let mut stop = capture.raw_observations[1].clone();
    let mut finish = capture.raw_observations[2].clone();
    capture.raw_observations.clear();
    capture.raw_observations.push(start);
    for sequence in 2..=(MAX_EVENTS as u64 - 2) {
        intermediate.sequence = sequence;
        capture.raw_observations.push(intermediate.clone());
    }
    stop.sequence = MAX_EVENTS as u64 - 1;
    finish.sequence = MAX_EVENTS as u64;
    capture.raw_observations.push(stop);
    capture.raw_observations.push(finish);
    capture.raw_last_sequence = Some(MAX_EVENTS as u64);
    capture.raw_observation_count = Some(MAX_EVENTS);
    capture.estimated_bytes = MAX_ESTIMATED_BYTES;
    capture.events[1].source_sequence = Some(MAX_EVENTS as u64);

    assert_eq!(assess_replay(&capture).unwrap(), ReplayAssessment::Verified);
}

fn assert_saved_variables_round_trip_through_sqlite(lua: &Lua, name: &str) {
    let source = serialized_saved_variables(lua);
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    let expected = canonical_bytes(&capture).unwrap();
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join(format!("{name}.lua"));
    let store = sandbox.path().join(format!("{name}.sqlite"));
    std::fs::write(&input, source).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    let loaded = load_encounter(&store, &receipt.session_id, &receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(canonical_bytes(&loaded).unwrap(), expected);
}

#[test]
fn addon_exposes_exactly_two_modes_and_one_toggle_transition() {
    let lua = harness("");
    run(
        &lua,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state_schema_version == 1)
        assert(saved.addon_version == 4)
        assert(saved.selected_mode == "single")
        assert(saved.state == "stopped")

        SLASH_COMMANDS["/ewencounter"]("mode automatic")
        assert(saved.selected_mode == "single")
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        assert(saved.requested_mode == "continuous")
        assert(saved.active_mode == "continuous")
        assert(saved.state == "waiting")

        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(saved.state == "capturing")
        local first = saved.current
        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(saved.state == "waiting")
        assert(saved.session.completed_encounter_count == 1)
        assert(saved.records["0000000001"].capture == first)

        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(saved.session.completed_encounter_count == 2)
        assert(saved.records["0000000002"].ordinal == 2)
        assert(saved.records["0000000001"].capture.session_id
            == saved.records["0000000002"].capture.session_id)
        assert(saved.records["0000000001"].capture.first_sequence == 1)
        assert(saved.records["0000000002"].capture.first_sequence == 1)

        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(saved.session.completed_encounter_count == 3)
        assert(saved.session.next_encounter_ordinal == 4)
        assert(saved.records["0000000003"].ordinal == 3)
        assert(saved.records["0000000003"].capture.first_sequence == 1)
        assert(saved.records["0000000003"].capture.encounter_id
            ~= saved.records["0000000002"].capture.encounter_id)

        SLASH_COMMANDS["/ewencounter"]("toggle")
        assert(saved.state == "stopped")
        assert(saved.requested_mode == nil and saved.active_mode == nil)
        "#,
    );
}

#[test]
fn enablement_warns_about_persisted_values_before_authority_is_committed() {
    for in_combat in [false, true] {
        let lua = harness("");
        run(
            &lua,
            &format!(
                r#"
                SLASH_COMMANDS["/ewencounter"]("channel live")
                __in_combat = {in_combat}
                local original = d
                local authorityAtWarning = "not-seen"
                function d(text)
                    if string.find(text, "names and identifiers", 1, true) then
                        authorityAtWarning = EsoWeaveDataSaved.encounter.requested_mode
                    end
                    original(text)
                end
                SLASH_COMMANDS["/ewencounter"]("toggle")
                assert(authorityAtWarning == nil)
                assert(EsoWeaveDataSaved.encounter.requested_mode == "single")
                assert(EsoWeaveDataSaved.encounter.state
                    == ({in_combat} and "capturing" or "waiting"))
                "#
            ),
        );
    }
}

#[test]
fn in_game_status_names_requested_effective_and_failure_authority() {
    let lua = harness("EsoWeaveEncounterTestLimits = { max_interruptions = 0 }");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        SLASH_COMMANDS["/ewencounter"]("status")
        local waiting = __messages[#__messages]
        assert(string.find(waiting, "Selected mode: continuous", 1, true))
        assert(string.find(waiting, "requested mode: continuous", 1, true))
        assert(string.find(waiting, "active mode: continuous", 1, true))
        assert(string.find(waiting, "channel: live", 1, true))
        assert(string.find(waiting, "state: waiting", 1, true))
        assert(string.find(waiting, "current encounter: none", 1, true))
        assert(string.find(waiting, "session: active", 1, true))

        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        SLASH_COMMANDS["/ewencounter"]("status")
        local capturing = __messages[#__messages]
        assert(string.find(capturing, "state: capturing", 1, true))
        assert(not string.find(capturing, "current encounter: none", 1, true))

        __fire(EVENT_PLAYER_DEACTIVATED)
        SLASH_COMMANDS["/ewencounter"]("status")
        local failed = __messages[#__messages]
        assert(string.find(failed, "state: failed", 1, true))
        assert(string.find(failed, "last interruption: none", 1, true))
        assert(string.find(failed, "failure: interruption-limit", 1, true))
        "#,
    );
}

#[test]
fn single_mode_started_mid_combat_is_truthfully_partial() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("channel live")
        __in_combat = true
        SLASH_COMMANDS["/ewencounter"]("toggle")
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "capturing")
        assert(saved.current.pending_partial_reason == "started-mid-combat")
        local start = saved.current.raw_observations[1]
        assert(start.source_kind == "api-sample")
        assert(start.source_id == "IsUnitInCombat")
        assert(start.argument_count == 1 and start.return_count == 1)
        __assert_tagged_string(start, 1, "player")
        __assert_tagged_boolean(start, 2, true)

        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        local capture = saved.records["0000000001"].capture
        assert(saved.state == "stopped")
        assert(capture.status == "partial")
        assert(capture.partial_reason == "started-mid-combat")
        assert(capture.events[1].payload.reason == "started-mid-combat")
        assert(capture.events[#capture.events].payload.complete == false)
        "#,
    );
}

#[test]
fn mid_combat_toggle_off_remains_importable_without_fabricating_an_exit() {
    let lua = harness("");
    run(
        &lua,
        r#"
        __in_combat = true
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "stopped")
        assert(saved.records["0000000001"].capture.partial_reason
            == "started-mid-combat")
        "#,
    );
    let capture =
        parse_capture(serialized_saved_variables(&lua).as_bytes(), Channel::Live).unwrap();
    assert_eq!(capture.status, eso_weave::encounter::CaptureStatus::Partial);
    assert_eq!(
        assess_replay(&capture).unwrap(),
        ReplayAssessment::Indeterminate
    );

    let deactivated = harness("");
    run(
        &deactivated,
        r#"
        __in_combat = true
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        __fire(EVENT_PLAYER_DEACTIVATED)
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "stopped")
        assert(saved.stop_reason == "single-interrupted")
        assert(saved.records["0000000001"].capture.partial_reason
            == "started-mid-combat")
        assert(saved.interruptions[1].reason == "player-deactivated")
        "#,
    );
    let capture = parse_capture(
        serialized_saved_variables(&deactivated).as_bytes(),
        Channel::Live,
    )
    .unwrap();
    assert_eq!(capture.status, eso_weave::encounter::CaptureStatus::Partial);
    assert_eq!(
        assess_replay(&capture).unwrap(),
        ReplayAssessment::Indeterminate
    );
}

#[test]
fn reload_recovers_active_authority_without_silently_losing_the_prefix() {
    let waiting = harness("");
    run(
        &waiting,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        "#,
    );
    let waiting_source = serialized_saved_variables(&waiting);
    let waiting_revision: i64 = waiting
        .load("return EsoWeaveDataSaved.encounter.revision")
        .eval()
        .unwrap();
    let waiting_recovered = harness(&waiting_source);
    run(
        &waiting_recovered,
        &format!(
            r#"
            local saved = EsoWeaveDataSaved.encounter
            assert(saved.state == "waiting")
            assert(saved.revision == {waiting_revision})
            assert(saved.session.completed_encounter_count == 0)
            assert(saved.session.interruption_count == 0)
            assert(saved.current == nil)
            "#
        ),
    );

    let waiting_single = harness("");
    run(
        &waiting_single,
        r#"
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        assert(EsoWeaveDataSaved.encounter.state == "waiting")
        "#,
    );
    let waiting_single_recovered = harness(&serialized_saved_variables(&waiting_single));
    run(
        &waiting_single_recovered,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "stopped")
        assert(saved.stop_reason == "single-interrupted")
        assert(saved.requested_mode == nil and saved.active_mode == nil)
        assert(saved.session.completed_encounter_count == 0)
        assert(saved.interruptions[1].reason == "runtime-interrupted")
        "#,
    );

    let continuous = harness("");
    run(
        &continuous,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "ability",
            0, 0, "source", 1, "target", 2, 10, 3, 4, true,
            100, 200, 300, 0)
        "#,
    );
    let active_source = serialized_saved_variables(&continuous);
    let recovered = harness(&format!("{active_source}; __in_combat = true"));
    run(
        &recovered,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "capturing")
        assert(saved.requested_mode == "continuous")
        assert(saved.active_mode == "continuous")
        assert(saved.session.completed_encounter_count == 1)
        assert(saved.session.degraded_encounter_count == 1)
        assert(saved.session.next_encounter_ordinal == 3)
        assert(saved.records["0000000001"].capture.partial_reason
            == "runtime-interrupted")
        assert(saved.records["0000000001"].capture.warnings.recovered_interruption == 1)
        assert(saved.interruptions[1].reason == "runtime-interrupted")
        assert(saved.current.pending_partial_reason == "started-mid-combat")
        assert(saved.current.encounter_id ~= saved.records["0000000001"].capture.encounter_id)
        "#,
    );

    let single = harness("");
    run(
        &single,
        r#"
        SLASH_COMMANDS["/ewencounter"]("channel pts")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        "#,
    );
    let single_source = serialized_saved_variables(&single);
    let single_recovered = harness(&single_source);
    run(
        &single_recovered,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "stopped")
        assert(saved.requested_mode == nil and saved.active_mode == nil)
        assert(saved.stop_reason == "single-interrupted")
        assert(saved.session.completed_encounter_count == 1)
        assert(saved.records["0000000001"].capture.partial_reason
            == "runtime-interrupted")
        assert(saved.interruptions[1].reason == "runtime-interrupted")
        "#,
    );

    for (name, lua, channel) in [
        ("continuous-recovery", &recovered, Channel::Live),
        ("single-recovery", &single_recovered, Channel::Pts),
    ] {
        let source = serialized_saved_variables(lua);
        assert_eq!(
            parse_capture_set(source.as_bytes(), channel)
                .unwrap()
                .records
                .len(),
            1
        );
        let sandbox = tempfile::tempdir().unwrap();
        let input = sandbox.path().join(format!("{name}.lua"));
        let store = sandbox.path().join(format!("{name}.sqlite"));
        std::fs::write(&input, source).unwrap();
        let report = import_capture_set(&ImportRequest::new(&input, &store, channel)).unwrap();
        assert_eq!(report.imported_count, 1);
        assert_eq!(list_encounters(&store).unwrap().len(), 1);
    }
}

#[test]
fn toggle_off_while_capturing_retains_a_non_destructive_partial_record() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        local current = EsoWeaveDataSaved.encounter.current
        SLASH_COMMANDS["/ewencounter"]("toggle")
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "stopped")
        assert(saved.stop_reason == "user-disabled")
        assert(saved.requested_mode == nil and saved.active_mode == nil)
        assert(saved.current == nil)
        assert(saved.session.completed_encounter_count == 1)
        assert(saved.records["0000000001"].capture == current)
        assert(current.status == "partial")
        assert(current.partial_reason == "user-stopped")
        assert(current.events[#current.events].kind == "encounter-end")
        assert(EVENT_MANAGER.events[EVENT_COMBAT_EVENT] == nil)
        assert(next(EVENT_MANAGER.updates) == nil)
        "#,
    );
}

#[test]
fn aggregate_encounter_and_interruption_bounds_fail_closed_without_eviction() {
    let encounters = harness("EsoWeaveEncounterTestLimits = { max_session_encounters = 2 }");
    run(
        &encounters,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        for ordinal = 1, 2 do
            __in_combat = true
            __fire(EVENT_PLAYER_COMBAT_STATE, true)
            __in_combat = false
            __fire(EVENT_PLAYER_COMBAT_STATE, false)
        end
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "failed")
        assert(saved.failure.reason == "storage-pressure")
        assert(saved.session.completed_encounter_count == 2)
        assert(saved.records["0000000001"].capture.status == "complete")
        assert(saved.records["0000000002"].capture.status == "complete")
        assert(saved.records["0000000003"] == nil)
        "#,
    );

    let first = harness("EsoWeaveEncounterTestLimits = { max_interruptions = 1 }");
    run(
        &first,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        __fire(EVENT_PLAYER_DEACTIVATED)
        assert(EsoWeaveDataSaved.encounter.state == "interrupted")
        "#,
    );
    let interrupted_source = serialized_saved_variables(&first);
    let second = harness(&format!(
        "EsoWeaveEncounterTestLimits = {{ max_interruptions = 1 }}; {interrupted_source}"
    ));
    run(
        &second,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "waiting")
        __fire(EVENT_PLAYER_DEACTIVATED)
        assert(saved.state == "failed")
        assert(saved.failure.reason == "interruption-limit")
        assert(#saved.interruptions == 1)
        assert(saved.interruptions[1].sequence == 1)
        "#,
    );
}

#[test]
fn exact_next_encounter_reserves_produce_an_importable_terminal_prefix() {
    let lua = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 6, max_raw_observations = 7, max_estimated_bytes = 1048576 }",
    );
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        for _ = 1, 2 do
            __in_combat = true
            __fire(EVENT_PLAYER_COMBAT_STATE, true)
            __in_combat = false
            __fire(EVENT_PLAYER_COMBAT_STATE, false)
        end
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.session.completed_encounter_count == 2)
        assert(saved.records["0000000001"].capture.events[1].kind == "encounter-start")
        assert(saved.records["0000000002"].capture.events[1].kind == "encounter-start")
        assert(saved.state == "failed")
        assert(saved.failure.reason == "storage-pressure")
        "#,
    );
    let parsed =
        parse_capture_set(serialized_saved_variables(&lua).as_bytes(), Channel::Live).unwrap();
    assert_eq!(parsed.records.len(), 2);
}

#[test]
fn failed_and_unknown_controller_states_never_auto_retry_or_rewrite_evidence() {
    let invalid = harness(
        r#"__invalid_controller = {
            state_schema_version = 1,
            addon_version = 4,
            selected_mode = "continuous",
            selected_channel = "live",
            requested_mode = "continuous",
            active_mode = "continuous",
            state = "capturing",
            current_encounter_id = "encounter-invalid",
            records = {},
            interruptions = {},
            revision = 7,
        }
        EsoWeaveDataSaved.encounter = __invalid_controller"#,
    );
    run(
        &invalid,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved == __invalid_controller)
        assert(saved.state == "capturing")
        assert(saved.requested_mode == "continuous")
        assert(saved.revision == 7)
        SLASH_COMMANDS["/ewencounter"]("toggle")
        assert(saved == __invalid_controller)
        assert(saved.revision == 7)
        "#,
    );
    let invalid_source = serialized_saved_variables(&invalid);
    let reloaded = harness(&invalid_source);
    run(
        &reloaded,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "capturing")
        assert(saved.revision == 7)
        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(EsoWeaveDataSaved.encounter.stop_reason == "cleared")
        "#,
    );

    let unknown = harness(
        r#"__unknown = {
            state_schema_version = 77,
            addon_version = 81,
            sentinel = "preserve-unknown-controller",
        }
        EsoWeaveDataSaved.encounter = __unknown"#,
    );
    run(
        &unknown,
        r#"
        assert(EsoWeaveDataSaved.encounter == __unknown)
        SLASH_COMMANDS["/ewencounter"]("toggle")
        assert(EsoWeaveDataSaved.encounter == __unknown)
        assert(EsoWeaveDataSaved.encounter.sentinel
            == "preserve-unknown-controller")
        "#,
    );
}

#[test]
fn malformed_recovery_arithmetic_is_preserved_inactive_without_crashing_load() {
    let active = harness("");
    run(
        &active,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        "#,
    );
    let active_source = serialized_saved_variables(&active);
    let corrupted = harness(&format!(
        "{active_source}; EsoWeaveDataSaved.encounter.current.raw_last_sequence = 'not-a-number'"
    ));
    run(
        &corrupted,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        assert(saved.state == "capturing")
        assert(saved.current.raw_last_sequence == "not-a-number")
        assert(EVENT_MANAGER.events[EVENT_PLAYER_COMBAT_STATE] ~= nil)
        SLASH_COMMANDS["/ewencounter"]("status")
        assert(saved.current.raw_last_sequence == "not-a-number")
        "#,
    );

    for mutation in [
        "current.last_sequence = 0",
        "current.raw_first_sequence = 0; current.raw_last_sequence = 0",
        "current.source = 'not-a-table'",
        "current.raw_observations[1] = 1",
        "current.raw_observations[1] = {}",
        "current.events.extra = current.events[1]",
        "current.raw_observations.extra = current.raw_observations[1]",
        "current.raw_observations[1].values.extra = current.raw_observations[1].values[1]",
        "current.normalization_profile.damage_results.extra = 1",
        "current.estimated_bytes = 33554432; EsoWeaveDataSaved.encounter.session.aggregate_estimated_bytes = 33554432",
        "current.events[1].session_id = 'session-corrupt-1'",
        "current.raw_omitted_observation_count = 1; current.raw_last_sequence = current.raw_last_sequence + 1; current.raw_loss = { missing_sequence_from = current.raw_last_sequence, missing_sequence_to = current.raw_last_sequence, reason = 'unknown-loss' }",
    ] {
        let corrupted = harness(&format!(
            "{active_source}; local current = EsoWeaveDataSaved.encounter.current; {mutation}; __corrupted = EsoWeaveDataSaved.encounter"
        ));
        run(
            &corrupted,
            r#"
            local saved = EsoWeaveDataSaved.encounter
            assert(saved == __corrupted)
            local revision = saved.revision
            SLASH_COMMANDS["/ewencounter"]("toggle")
            assert(saved == __corrupted and saved.revision == revision)
            SLASH_COMMANDS["/ewencounter"]("status")
            assert(string.find(__messages[#__messages], "state-invalid", 1, true))
            "#,
        );
    }

    let terminal = harness("");
    run(
        &terminal,
        r#"
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel live")
        SLASH_COMMANDS["/ewencounter"]("toggle")
        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let terminal_source = serialized_saved_variables(&terminal);
    for mutation in [
        "capture.events[1].session_id = 'session-corrupt-1'",
        "capture.started_at = '9007199254740993'; capture.finished_at = '9007199254740992'",
        "capture.events.extra = capture.events[1]",
    ] {
        let corrupted_terminal = harness(&format!(
            "{terminal_source}; local capture = EsoWeaveDataSaved.encounter.records['0000000001'].capture; {mutation}; __corrupted = EsoWeaveDataSaved.encounter"
        ));
        run(
            &corrupted_terminal,
            r#"
            local saved = EsoWeaveDataSaved.encounter
            local revision = saved.revision
            assert(saved == __corrupted and saved.state == "waiting")
            __in_combat = true
            __fire(EVENT_PLAYER_COMBAT_STATE, true)
            assert(saved.current == nil and saved.revision == revision)
            SLASH_COMMANDS["/ewencounter"]("status")
            assert(string.find(__messages[#__messages], "state-invalid", 1, true))
            "#,
        );
    }
}

#[test]
fn inconsistent_controller_combinations_remain_inactive_and_unchanged() {
    for setup in [
        "EsoWeaveDataSaved.encounter.stop_reason = nil",
        "SLASH_COMMANDS['/ewencounter']('channel live'); SLASH_COMMANDS['/ewencounter']('toggle'); EsoWeaveDataSaved.encounter.stop_reason = 'cleared'",
        "SLASH_COMMANDS['/ewencounter']('channel live'); __in_combat = true; SLASH_COMMANDS['/ewencounter']('toggle'); EsoWeaveDataSaved.encounter.failure = { reason = 'state-invalid', occurred_at = '1788912000' }",
        "SLASH_COMMANDS['/ewencounter']('mode continuous'); SLASH_COMMANDS['/ewencounter']('channel live'); SLASH_COMMANDS['/ewencounter']('toggle'); __fire(EVENT_PLAYER_DEACTIVATED); EsoWeaveDataSaved.encounter.interruptions = {}; EsoWeaveDataSaved.encounter.session.interruption_count = 0",
        "SLASH_COMMANDS['/ewencounter']('mode continuous'); SLASH_COMMANDS['/ewencounter']('channel live'); SLASH_COMMANDS['/ewencounter']('toggle'); __fire(EVENT_PLAYER_DEACTIVATED); EsoWeaveDataSaved.encounter.interruptions[1].occurred_at = '99999999999999999999'",
        "EsoWeaveDataSaved.encounter.revision = 9007199254740992",
        "SLASH_COMMANDS['/ewencounter']('mode continuous'); SLASH_COMMANDS['/ewencounter']('channel live'); SLASH_COMMANDS['/ewencounter']('toggle'); __fire(EVENT_PLAYER_DEACTIVATED); EsoWeaveDataSaved.encounter.interruptions.extra = EsoWeaveDataSaved.encounter.interruptions[1]",
        "SLASH_COMMANDS['/ewencounter']('mode continuous'); SLASH_COMMANDS['/ewencounter']('channel live'); SLASH_COMMANDS['/ewencounter']('toggle'); for _ = 1, 2 do __in_combat = true; __fire(EVENT_PLAYER_COMBAT_STATE, true); __in_combat = false; __fire(EVENT_PLAYER_COMBAT_STATE, false) end; SLASH_COMMANDS['/ewencounter']('toggle'); EsoWeaveDataSaved.encounter.selected_mode = 'single'; EsoWeaveDataSaved.encounter.session.mode = 'single'",
    ] {
        let base = harness("");
        run(&base, setup);
        let source = serialized_saved_variables(&base);
        let reloaded = harness(&format!("{source}; __corrupted = EsoWeaveDataSaved.encounter"));
        run(
            &reloaded,
            r#"
            local saved = EsoWeaveDataSaved.encounter
            local revision = saved.revision
            SLASH_COMMANDS["/ewencounter"]("toggle")
            assert(saved == __corrupted and saved.revision == revision)
            SLASH_COMMANDS["/ewencounter"]("status")
            assert(string.find(__messages[#__messages], "state-invalid", 1, true))
            "#,
        );
    }
}

#[test]
fn malformed_legacy_current_is_preserved_inactive_and_clearable() {
    let lua = harness(
        r#"__legacy = {
            schema_version = 1, addon_version = 1, status = "capturing",
            channel = "live", privacy_profile = "anonymous-local-v1",
            source = { api_version = 101050, game_version = "12.0.7", locale = "en", platform = "1" },
            session_id = "session-1788912000-1000",
            encounter_id = "encounter-1788912000-1000",
            started_at = "1788912000", finished_at = "",
            started_monotonic_ms = 1000, ended_monotonic_ms = 0,
            first_sequence = 1, last_sequence = 1,
            stored_event_count = 1, omitted_event_count = 0,
            estimated_bytes = 900,
            warnings = { recovered_interruption = "not-a-number" },
            events = {{
                session_id = "session-1788912000-1000",
                encounter_id = "encounter-1788912000-1000",
                sequence = 1, monotonic_ms = 0, kind = "encounter-start",
                payload = { reason = "combat-started" },
            }},
        }
        EsoWeaveDataSaved.encounter = __legacy"#,
    );
    run(
        &lua,
        r#"
        assert(EsoWeaveDataSaved.encounter == __legacy)
        assert(__legacy.warnings.recovered_interruption == "not-a-number")
        SLASH_COMMANDS["/ewencounter"]("status")
        assert(string.find(__messages[#__messages], "state-invalid", 1, true))
        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(EsoWeaveDataSaved.encounter.stop_reason == "cleared")
        "#,
    );
}

#[test]
fn addon_is_dormant_until_one_explicit_channel_arm() {
    let lua = harness("");
    run(
        &lua,
        r#"
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveDataSaved.encounter.state == "stopped")

        SLASH_COMMANDS["/ewencounter"]("arm live")
        assert(EsoWeaveDataSaved.encounter.state == "waiting")
        assert(EsoWeaveDataSaved.encounter.selected_channel == "live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveDataSaved.encounter.state == "capturing")
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(__capture().status == "complete")
        assert(__capture().events[1].kind == "encounter-start")
        assert(__capture().events[#__capture().events].kind == "encounter-end")

        local retainedSession = EsoWeaveDataSaved.encounter.session.session_id
        SLASH_COMMANDS["/ewencounter"]("mode continuous")
        SLASH_COMMANDS["/ewencounter"]("channel pts")
        SLASH_COMMANDS["/ewencounter"]("arm pts")
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(EsoWeaveDataSaved.encounter.selected_mode == "single")
        assert(EsoWeaveDataSaved.encounter.selected_channel == "live")
        assert(EsoWeaveDataSaved.encounter.session.session_id == retainedSession)
        assert(__capture().status == "complete")
        SLASH_COMMANDS["/ewencounter"]("clear")
        assert(EsoWeaveDataSaved.encounter.session ~= nil)
        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        SLASH_COMMANDS["/ewencounter"]("arm pts")
        assert(EsoWeaveDataSaved.encounter.selected_channel == "pts")
        SLASH_COMMANDS["/ewencounter"]("disarm")
        assert(EsoWeaveDataSaved.encounter.state == "stopped")

        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        __in_combat = true
        SLASH_COMMANDS["/ewencounter"]("arm live")
        assert(EsoWeaveDataSaved.encounter.state == "capturing")
        assert(EsoWeaveDataSaved.encounter.current.pending_partial_reason
            == "started-mid-combat")
        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(__capture().status == "partial")
        "#,
    );
}

#[test]
fn schema_v1_terminal_capture_is_preserved_without_fabricated_raw_evidence() {
    let lua = harness(
        r#"
        __legacy_encounter = {
            schema_version = 1,
            addon_version = 1,
            status = "complete",
            channel = "live",
            privacy_profile = "anonymous-local-v1",
            source = {
                api_version = 101050,
                game_version = "12.0.8",
                locale = "en",
                platform = "1",
            },
            session_id = "session-1788912000-1000",
            encounter_id = "encounter-1788912000-1000",
            started_at = "1788912000",
            finished_at = "1788912010",
            started_monotonic_ms = 1000,
            ended_monotonic_ms = 10000,
            first_sequence = 1,
            last_sequence = 2,
            stored_event_count = 2,
            omitted_event_count = 0,
            estimated_bytes = 1024,
            warnings = {},
            events = {
                {
                    session_id = "session-1788912000-1000",
                    encounter_id = "encounter-1788912000-1000",
                    sequence = 1,
                    monotonic_ms = 0,
                    kind = "encounter-start",
                    payload = { reason = "combat-started" },
                },
                {
                    session_id = "session-1788912000-1000",
                    encounter_id = "encounter-1788912000-1000",
                    sequence = 2,
                    monotonic_ms = 10000,
                    kind = "encounter-end",
                    payload = { reason = "combat-ended", complete = true },
                },
            },
        }
        EsoWeaveDataSaved.encounter = __legacy_encounter
        "#,
    );
    run(
        &lua,
        r#"
        assert(EsoWeaveDataSaved.encounter.state_schema_version == 1)
        assert(__capture() == __legacy_encounter)
        assert(__capture().schema_version == 1)
        assert(__capture().status == "complete")
        assert(__capture().raw_observations == nil)
        assert(__capture().raw_observation_count == nil)
        assert(__capture().events[2].payload.complete == true)
        SLASH_COMMANDS["/ewencounter"]("arm live")
        assert(__capture() == __legacy_encounter)
        "#,
    );

    let wrapped = serialized_saved_variables(&lua);
    let invalid_continuous = harness(&format!(
        "{wrapped}; local saved = EsoWeaveDataSaved.encounter; saved.selected_mode = 'continuous'; saved.session.mode = 'continuous'; saved.requested_mode = 'continuous'; saved.active_mode = 'continuous'; saved.state = 'waiting'; saved.session.status = 'active'; saved.session.finished_at = nil; saved.stop_reason = nil; __corrupted = saved"
    ));
    run(
        &invalid_continuous,
        r#"
        local saved = EsoWeaveDataSaved.encounter
        local revision = saved.revision
        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(saved == __corrupted and saved.revision == revision and saved.current == nil)
        SLASH_COMMANDS["/ewencounter"]("status")
        assert(string.find(__messages[#__messages], "state-invalid", 1, true))
        "#,
    );
}

#[test]
fn pre_profile_v2_terminal_is_preserved_and_idle_state_advances_safely() {
    let terminal = harness(
        r#"
        __legacy_v2 = { schema_version = 2, addon_version = 2, status = "complete" }
        EsoWeaveDataSaved.encounter = __legacy_v2
        "#,
    );
    run(
        &terminal,
        r#"
        assert(__capture() == __legacy_v2)
        assert(__capture().addon_version == 2)
        "#,
    );

    let idle = harness(
        r#"
        EsoWeaveDataSaved.encounter = {
            schema_version = 2, addon_version = 2, status = "idle"
        }
        "#,
    );
    run(
        &idle,
        r#"
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(EsoWeaveDataSaved.encounter.addon_version == 4)
        assert(EsoWeaveDataSaved.encounter.state_schema_version == 1)
        "#,
    );
}

#[test]
fn representative_capture_retains_every_selected_source_and_links_projections() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __advance(10)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "AbilitySecret",
            0, 0, "AccountSecret", 1, "CharacterSecret", 2, 1200, 3, 4,
            true, 9001, 9002, 7001, 50, "damage-extra")
        __advance(10)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_HEAL, false, "HealSecret",
            0, 0, "AccountSecret", 1, "CharacterSecret", 2, 300, 3, 0,
            true, 9001, 9001, 7002, 25)
        __advance(10)
        __fire(EVENT_EFFECT_CHANGED, 1, 2, "EffectSecret", "group1", 1.1,
            2.2, 3, "IconSecret", "DeprecatedSecret", 4, 5, 6,
            "UnitSecret", 9001, 7100, 1, "effect-extra")
        __advance(10)
        __fire(EVENT_POWER_UPDATE, "group1", 1, 3, 800, 1000, 1000,
            "power-extra")
        __advance(10)
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 3, "slot-extra")
        __advance(10)
        __fire(EVENT_ACTIVE_WEAPON_PAIR_CHANGED, 2, false, "bar-extra")
        __advance(10)
        __fire(EVENT_PLAYER_DEAD, "dead-extra")
        __advance(10)
        __fire(EVENT_PLAYER_ALIVE, "alive-extra")
        __advance(10)
        __fire(EVENT_BOSSES_CHANGED, false, "boss-extra")
        __advance(10)
        __fire(EVENT_ACTIVE_QUICKSLOT_CHANGED, 4, "quickslot-extra")
        __advance(1000)
        __run_updates()

        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 4)
        __boss_value = 4000
        __advance(1000)
        __run_updates()
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __now = 500
        __fire(EVENT_POWER_UPDATE, "group1", 1, 3, 700, 1000, 1000)

        local capture = __capture()
        assert(EsoWeaveDataSaved.encounter.state == "failed")
        assert(capture.status == "partial")
        assert(capture.partial_reason == "clock-reset")
        assert(capture.schema_version == 2)
        local required = {
            ["encounter-start"] = false, ["encounter-end"] = false,
            damage = false, healing = false, effect = false, resource = false,
            cast = false, ["bar-change"] = false, death = false,
            resurrection = false, ["boss-health"] = false,
            performance = false, quickslot = false, discontinuity = false,
        }
        local previousSequence = 0
        local previousTime = 0
        local bossSamples = 0
        local performanceSamples = 0
        local usedQuickslotAbility = 0
        for _, event in ipairs(capture.events) do
            assert(required[event.kind] ~= nil, event.kind)
            required[event.kind] = true
            assert(event.sequence > previousSequence)
            assert(event.monotonic_ms >= previousTime)
            assert(event.session_id == capture.session_id)
            assert(event.encounter_id == capture.encounter_id)
            if event.kind == "boss-health" then bossSamples = bossSamples + 1 end
            if event.kind == "performance" then
                performanceSamples = performanceSamples + 1
            end
            if event.kind == "quickslot" and event.payload.action == "used" then
                usedQuickslotAbility = event.payload.ability_id
            end
            previousSequence = event.sequence
            previousTime = event.monotonic_ms
        end
        for kind, present in pairs(required) do assert(present, kind) end
        assert(bossSamples == 2, "boss samples: " .. tostring(bossSamples))
        assert(performanceSamples == 2,
            "performance samples: " .. tostring(performanceSamples))
        assert(usedQuickslotAbility == 9004,
            "used quickslot ability: " .. tostring(usedQuickslotAbility))
        assert(capture.stored_event_count == #capture.events)

        local callbacks = {
            { "EVENT_PLAYER_COMBAT_STATE", EVENT_PLAYER_COMBAT_STATE, 2 },
            { "EVENT_COMBAT_EVENT", EVENT_COMBAT_EVENT, 19 },
            { "EVENT_EFFECT_CHANGED", EVENT_EFFECT_CHANGED, 18 },
            { "EVENT_POWER_UPDATE", EVENT_POWER_UPDATE, 8 },
            { "EVENT_ACTION_SLOT_ABILITY_USED", EVENT_ACTION_SLOT_ABILITY_USED, 3 },
            { "EVENT_ACTIVE_WEAPON_PAIR_CHANGED", EVENT_ACTIVE_WEAPON_PAIR_CHANGED, 4 },
            { "EVENT_PLAYER_DEAD", EVENT_PLAYER_DEAD, 2 },
            { "EVENT_PLAYER_ALIVE", EVENT_PLAYER_ALIVE, 2 },
            { "EVENT_BOSSES_CHANGED", EVENT_BOSSES_CHANGED, 3 },
            { "EVENT_ACTIVE_QUICKSLOT_CHANGED", EVENT_ACTIVE_QUICKSLOT_CHANGED, 3 },
        }
        for _, expected in ipairs(callbacks) do
            __assert_raw_callback(
                __raw_observation(expected[1]), expected[1], expected[2], expected[3])
        end
        __assert_raw_callback(
            __raw_observation("EVENT_PLAYER_COMBAT_STATE", 2),
            "EVENT_PLAYER_COMBAT_STATE", EVENT_PLAYER_COMBAT_STATE, 2)

        local damageRaw = __raw_observation("EVENT_COMBAT_EVENT")
        __assert_tagged_string(damageRaw, 4, "AbilitySecret")
        __assert_tagged_string(damageRaw, 7, "AccountSecret")
        __assert_tagged_string(damageRaw, 9, "CharacterSecret")
        __assert_tagged_number(damageRaw, 15, 9001)
        __assert_tagged_number(damageRaw, 16, 9002)
        __assert_tagged_string(damageRaw, 19, "damage-extra")

        local effectRaw = __raw_observation("EVENT_EFFECT_CHANGED")
        __assert_tagged_string(effectRaw, 4, "EffectSecret")
        __assert_tagged_string(effectRaw, 5, "group1")
        __assert_tagged_number(effectRaw, 6, 1.1)
        __assert_tagged_number(effectRaw, 7, 2.2)
        __assert_tagged_string(effectRaw, 9, "IconSecret")
        __assert_tagged_string(effectRaw, 10, "DeprecatedSecret")
        __assert_tagged_string(effectRaw, 14, "UnitSecret")
        __assert_tagged_string(effectRaw, 18, "effect-extra")

        local powerRaw = __raw_observation("EVENT_POWER_UPDATE")
        __assert_tagged_string(powerRaw, 2, "group1")
        __assert_tagged_number(powerRaw, 3, 1)
        __assert_tagged_string(powerRaw, 8, "power-extra")
        __assert_tagged_boolean(
            __raw_observation("EVENT_ACTIVE_WEAPON_PAIR_CHANGED"), 3, false)
        __assert_tagged_boolean(__raw_observation("EVENT_BOSSES_CHANGED"), 2, false)

        local slotSample = __raw_observation("GetSlotBoundId")
        assert(slotSample.source_kind == "api-sample")
        assert(slotSample.source_code == nil)
        assert(slotSample.argument_count == 1 and slotSample.return_count == 1)
        __assert_tagged_number(slotSample, 1, 3)
        __assert_tagged_number(slotSample, 2, 7003)
        local currentQuickslot = __raw_observation("GetCurrentQuickslot")
        assert(currentQuickslot.argument_count == 0 and currentQuickslot.return_count == 1)
        __assert_tagged_number(currentQuickslot, 1, 4)
        local bossExists = __raw_observation("DoesUnitExist")
        assert(bossExists.argument_count == 1 and bossExists.return_count == 1)
        __assert_tagged_string(bossExists, 1, "boss1")
        __assert_tagged_boolean(bossExists, 2, true)
        local bossPower = __raw_observation("GetUnitPower")
        assert(bossPower.argument_count == 2 and bossPower.return_count == 3)
        __assert_tagged_string(bossPower, 1, "boss1")
        __assert_tagged_number(bossPower, 2, POWERTYPE_HEALTH)
        __assert_tagged_number(bossPower, 3, 5000)
        __assert_tagged_number(bossPower, 4, 10000)
        __assert_tagged_number(bossPower, 5, 10000)
        local framerate = __raw_observation("GetFramerate")
        assert(framerate.argument_count == 0 and framerate.return_count == 1)
        __assert_tagged_number(framerate, 1, 60.4)
        local latency = __raw_observation("GetLatency")
        assert(latency.argument_count == 0 and latency.return_count == 1)
        __assert_tagged_number(latency, 1, 45)

        local clockReset = __raw_observation("clock-reset")
        assert(clockReset.source_kind == "lifecycle")
        assert(clockReset.argument_count == 2 and clockReset.return_count == 0)
        __assert_tagged_number(clockReset, 1, 3100)
        __assert_tagged_number(clockReset, 2, 500)
        local finish = __raw_observation("capture-finish")
        assert(finish.source_kind == "lifecycle")
        assert(finish.argument_count == 2 and finish.return_count == 0)
        __assert_tagged_string(finish, 1, "clock-reset")
        __assert_tagged_boolean(finish, 2, false)

        local raw = capture.raw_observations
        assert(capture.raw_observation_count == #raw)
        assert(capture.raw_omitted_observation_count == 0)
        assert(capture.raw_first_sequence == raw[1].sequence)
        assert(capture.raw_last_sequence == raw[#raw].sequence)
        local previousRawSequence = 0
        local previousRawTime = 0
        for _, observation in ipairs(raw) do
            assert(observation.sequence > previousRawSequence)
            assert(observation.monotonic_ms >= previousRawTime)
            assert(observation.session_id == capture.session_id)
            assert(observation.encounter_id == capture.encounter_id)
            previousRawSequence = observation.sequence
            previousRawTime = observation.monotonic_ms
        end
        local nextOrdinal = {}
        for _, event in ipairs(capture.events) do
            assert(type(event.source_sequence) == "number")
            local expectedOrdinal = nextOrdinal[event.source_sequence] or 0
            assert(event.projection_ordinal == expectedOrdinal)
            nextOrdinal[event.source_sequence] = expectedOrdinal + 1
        end
        "#,
    );
    assert_selected_sources_round_trip_through_sqlite(&lua);
}

#[test]
fn unknown_callbacks_retain_future_scalar_arguments_and_exact_tagged_edges() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        local negativeZero = -1 / math.huge
        local minimumSubnormal = 4.9406564584124654e-324
        local largePowerOfTwo = 8.9884656743115795e307
        local maximumFinite = 1.7976931348623157e308
        __fire(EVENT_COMBAT_EVENT, 9999, false, "", 0, 0,
            "Søurce 名", 77, "Tårget λ", 88, 123, 3, 4, true,
            9001, 9002, 7001, 50,
            nil, false, 0, "", "future ✓", 1.25, negativeZero,
            minimumSubnormal, largePowerOfTwo, maximumFinite, nil)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)

        local observation = __raw_observation("EVENT_COMBAT_EVENT")
        __assert_raw_callback(observation, "EVENT_COMBAT_EVENT", EVENT_COMBAT_EVENT, 29)
        __assert_tagged_number(observation, 2, 9999)
        __assert_tagged_boolean(observation, 3, false)
        __assert_tagged_string(observation, 4, "")
        __assert_tagged_number(observation, 5, 0)
        __assert_tagged_string(observation, 7, "Søurce 名")
        __assert_tagged_string(observation, 9, "Tårget λ")
        __assert_tagged_nil(observation, 19)
        __assert_tagged_boolean(observation, 20, false)
        __assert_tagged_number(observation, 21, 0)
        __assert_tagged_string(observation, 22, "")
        __assert_tagged_string(observation, 23, "future ✓")
        __assert_tagged_number(observation, 24, 1.25)
        __assert_tagged_negative_zero(observation, 25)
        __assert_tagged_number(observation, 26, minimumSubnormal)
        assert(observation.values[26].significand == "1")
        assert(observation.values[26].exponent == -1074)
        __assert_tagged_number(observation, 27, largePowerOfTwo)
        assert(observation.values[27].significand == "1")
        assert(observation.values[27].exponent == 1023)
        __assert_tagged_number(observation, 28, maximumFinite)
        assert(observation.values[28].significand == "9007199254740991")
        assert(observation.values[28].exponent == 971)
        __assert_tagged_nil(observation, 29)
        assert(__projection_count(observation.sequence) == 0)
        assert(__capture().status == "complete")
        assert(__capture().raw_omitted_observation_count == 0)
        "#,
    );
}

#[test]
fn production_lua_round_trips_raw_values_through_the_restricted_rust_parser() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        local minimumSubnormal = 4.9406564584124654e-324
        local largePowerOfTwo = 8.9884656743115795e307
        local maximumFinite = 1.7976931348623157e308
        __fire(EVENT_COMBAT_EVENT, 9999, false, "Synthetic Ω", 0, 0,
            "@SyntheticAccount", 77, "Synthetic Character", 88, 123, 3, 4,
            true, 9001, 9002, 7001, 50, nil, false, 1.25,
            minimumSubnormal, largePowerOfTwo, maximumFinite, nil)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        "#,
    );
    let source = serialized_saved_variables(&lua);
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    let raw = serde_json::to_value(&capture).unwrap()["raw_observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| observation["source_id"] == "EVENT_COMBAT_EVENT")
        .cloned()
        .unwrap();
    assert_eq!(raw["argument_count"], 25);
    assert_eq!(raw["values"][3]["string"], "Synthetic Ω");
    assert_eq!(raw["values"][6]["string"], "@SyntheticAccount");
    assert_eq!(raw["values"][8]["string"], "Synthetic Character");
    assert_eq!(raw["values"][18]["value_type"], "nil");
    assert_eq!(raw["values"][19]["boolean"], false);
    assert_eq!(raw["values"][20]["significand"], "5");
    assert_eq!(raw["values"][20]["exponent"], -2);
    assert_eq!(raw["values"][21]["significand"], "1");
    assert_eq!(raw["values"][21]["exponent"], -1074);
    assert_eq!(raw["values"][22]["significand"], "1");
    assert_eq!(raw["values"][22]["exponent"], 1023);
    assert_eq!(raw["values"][23]["significand"], "9007199254740991");
    assert_eq!(raw["values"][23]["exponent"], 971);
    assert_eq!(raw["values"][24]["value_type"], "nil");

    let expected = canonical_bytes(&capture).unwrap();
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("exact-values.lua");
    let store = sandbox.path().join("encounters.sqlite");
    std::fs::write(&input, source).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    let loaded = load_encounter(&store, &receipt.session_id, &receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(canonical_bytes(&loaded).unwrap(), expected);
}

#[test]
fn api_samples_preserve_nil_returns_before_projection_defaults() {
    let lua = harness(
        r#"
        GetSlotBoundId = function() return nil end
        DoesUnitExist = function() return nil end
        GetFramerate = function() return nil end
        GetLatency = function() return nil end
        "#,
    );
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 3)
        __fire(EVENT_BOSSES_CHANGED, false)
        __fire(EVENT_ACTIVE_QUICKSLOT_CHANGED, 4)
        __advance(1000)
        __run_updates()
        __fire(EVENT_PLAYER_COMBAT_STATE, false)

        __assert_tagged_nil(__raw_observation("GetSlotBoundId"), 2)
        __assert_tagged_nil(__raw_observation("DoesUnitExist"), 2)
        __assert_tagged_nil(__raw_observation("GetFramerate"), 1)
        __assert_tagged_nil(__raw_observation("GetLatency"), 1)
        "#,
    );
    assert_saved_variables_round_trip_through_sqlite(&lua, "api-nil");
}

#[test]
fn callback_value_ceiling_declares_whole_observation_loss() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        local extras = {}
        for index = 1, 256 do extras[index] = index end
        __fire(EVENT_PLAYER_DEAD, unpack(extras, 1, 256))
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        __assert_raw_loss("record-limit")
        for _, event in ipairs(__capture().events) do
            assert(event.kind ~= "death")
        end
        "#,
    );
    assert_saved_variables_round_trip_through_sqlite(&lua, "callback-ceiling");
}

#[test]
fn lost_initial_callback_keeps_a_truthful_importable_start_boundary() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        local extras = {}
        for index = 1, 255 do extras[index] = index end
        __fire(EVENT_PLAYER_COMBAT_STATE, true, unpack(extras, 1, 255))
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        __assert_raw_loss("record-limit")
        assert(__capture().raw_loss.missing_sequence_from == 1)
        assert(__capture().events[1].kind == "encounter-start")
        assert(__capture().events[1].source_sequence == 1)
        "#,
    );
    assert_saved_variables_round_trip_through_sqlite(&lua, "lost-initial");
}

#[test]
fn clock_reset_at_normalized_ceiling_remains_importable() {
    let lua = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 4, max_raw_observations = 100, max_estimated_bytes = 1048576 }",
    );
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __advance(10)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "ability",
            0, 0, "source", 1, "target", 2, 10, 3, 4, true,
            100, 200, 300, 0)
        __now = 5
        __fire(EVENT_POWER_UPDATE, "group1", 1, 3, 700, 1000, 1000)
        assert(__capture().status == "partial")
        assert(__capture().partial_reason == "clock-reset")
        assert(__raw_observation("clock-reset").source_kind == "lifecycle")
        assert(__capture().events[3].kind == "discontinuity")
        assert(__capture().events[3].payload.reason == "capture-overflow")
        "#,
    );
    assert_saved_variables_round_trip_through_sqlite(&lua, "clock-ceiling");
}

#[test]
fn overflow_and_actor_limits_preserve_terminal_capacity_and_exact_loss() {
    let lua = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 6, max_raw_observations = 7, max_estimated_bytes = 1048576, max_actors = 1 }",
    );
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        for index = 1, 8 do
            __advance(1)
            __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "private",
                0, 0, "source", 1, "target", 2, index, 3, 4, true,
                10000 + index, 20000 + index, 7001, 0)
        end
        local controller = EsoWeaveDataSaved.encounter
        local capture = __capture()
        assert(controller.state == "failed")
        assert(controller.failure.reason == "storage-pressure")
        assert(capture.status == "partial")
        assert(capture.partial_reason == "capture-overflow")
        assert(#capture.events <= 6)
        assert(capture.estimated_bytes <= 1048576)
        assert(capture.omitted_event_count > 0)
        assert(capture.warnings.actor_limit > 0)
        local marker = capture.events[#capture.events - 1]
        local terminal = capture.events[#capture.events]
        assert(marker.kind == "discontinuity")
        assert(marker.payload.reason == "capture-overflow")
        assert(marker.payload.missing_sequence_to - marker.payload.missing_sequence_from + 1
            == capture.omitted_event_count)
        assert(marker.sequence == marker.payload.missing_sequence_to + 1)
        assert(terminal.kind == "encounter-end")
        assert(terminal.sequence == marker.sequence + 1)
        assert(terminal.payload.complete == false)
        assert(capture.pending_partial_reason == nil)
        assert(capture.pending_loss_from == nil)
        assert(capture.pending_loss_to == nil)
        assert(capture.pending_loss_reason == nil)
        assert(capture.raw_omitted_observation_count == 0)
        assert(capture.raw_last_sequence - capture.raw_first_sequence + 1
            == capture.raw_observation_count)
        "#,
    );
}

#[test]
fn stop_deactivation_and_callback_failure_are_partial_and_torn_down() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        SLASH_COMMANDS["/ewencounter"]("stop")
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(__capture().status == "partial")
        assert(__capture().partial_reason == "user-stopped")
        assert(EVENT_MANAGER.events[EVENT_COMBAT_EVENT] == nil)
        assert(next(EVENT_MANAGER.updates) == nil)

        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_PLAYER_DEACTIVATED)
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(__capture().partial_reason == "player-deactivated")
        assert(EsoWeaveDataSaved.encounter.session.interruption_count == 1)
        assert(EVENT_MANAGER.events[EVENT_EFFECT_CHANGED] == nil)
        local deactivated = __raw_observation("EVENT_PLAYER_DEACTIVATED")
        __assert_raw_callback(
            deactivated, "EVENT_PLAYER_DEACTIVATED", EVENT_PLAYER_DEACTIVATED, 1)
        assert(__raw_observation("capture-finish").source_kind == "lifecycle")

        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        GetSlotBoundId = function() error("injected callback failure") end
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 3)
        assert(EsoWeaveDataSaved.encounter.state == "failed")
        assert(EsoWeaveDataSaved.encounter.failure.reason == "callback-failed")
        assert(__capture().partial_reason == "callback-failed")
        assert(EVENT_MANAGER.events[EVENT_ACTION_SLOT_ABILITY_USED] == nil)
        assert(next(EVENT_MANAGER.updates) == nil)
        assert(not __contains_saved_string("injected callback failure"))
        __assert_raw_loss("callback-failed")
        "#,
    );
}

#[test]
fn player_deactivation_round_trips_through_sqlite() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_PLAYER_DEACTIVATED, "future-deactivation-value")
        "#,
    );
    let source = serialized_saved_variables(&lua);
    let capture = parse_capture(source.as_bytes(), Channel::Live).unwrap();
    assert!(capture.raw_observations.iter().any(|observation| {
        observation.source_id == "EVENT_PLAYER_DEACTIVATED" && observation.argument_count == 2
    }));
    let expected = canonical_bytes(&capture).unwrap();
    let sandbox = tempfile::tempdir().unwrap();
    let input = sandbox.path().join("deactivated.lua");
    let store = sandbox.path().join("encounters.sqlite");
    std::fs::write(&input, source).unwrap();
    let receipt = import_encounter(&ImportRequest::new(&input, &store, Channel::Live)).unwrap();
    let loaded = load_encounter(&store, &receipt.session_id, &receipt.encounter_id)
        .unwrap()
        .unwrap();
    assert_eq!(canonical_bytes(&loaded).unwrap(), expected);
}

#[test]
fn oversized_strings_and_unsupported_values_declare_whole_observation_loss() {
    let oversized = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 100, max_raw_observations = 100, max_estimated_bytes = 8192, max_string_bytes = 8 }",
    );
    run(
        &oversized,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false,
            "oversized-private-canary", 0, 0, "source", 1, "target", 2,
            10, 3, 4, true, 100, 200, 300, 0)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        __assert_raw_loss("string-limit")
        assert(not __contains_saved_string("oversized-private-canary"))
        for _, event in ipairs(__capture().events) do
            assert(event.kind ~= "damage")
        end
        "#,
    );

    let unsupported = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 100, max_raw_observations = 100, max_estimated_bytes = 8192, max_string_bytes = 1024 }",
    );
    run(
        &unsupported,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "ability",
            0, 0, "source", 1, "target", 2, 10, 3, 4, true,
            100, 200, 300, 0, { value = "unsupported-private-canary" })
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        __assert_raw_loss("unsupported-value")
        assert(not __contains_saved_string("unsupported-private-canary"))
        for _, event in ipairs(__capture().events) do
            assert(event.kind ~= "damage")
        end
        "#,
    );

    let nonfinite = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 100, max_raw_observations = 100, max_estimated_bytes = 8192, max_string_bytes = 1024 }",
    );
    run(
        &nonfinite,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_COMBAT_EVENT, 9999, false, "ability", 0, 0,
            "source", 1, "target", 2, 10, 3, 4, true,
            100, 200, 300, 0, math.huge)
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        __assert_raw_loss("unsupported-value")
        "#,
    );
}

#[test]
fn byte_overflow_and_saved_interruption_remain_bounded_and_partial() {
    let lua = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 100, max_raw_observations = 100, max_estimated_bytes = 8192, max_actors = 20 }",
    );
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        for index = 1, 30 do
            __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "private",
                0, 0, "source", 1, "target", 2, index, 3, 4, true,
                10000, 20000, 7001, 0)
        end
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(EsoWeaveDataSaved.encounter.state == "failed")
        assert(EsoWeaveDataSaved.encounter.failure.reason == "storage-pressure")
        assert(__capture().status == "partial")
        assert(__capture().estimated_bytes <= 8192)
        __assert_raw_loss("byte-limit")

        "#,
    );

    let recovery = harness(
        r#"EsoWeaveDataSaved.encounter = {
            schema_version = 1,
            addon_version = 1,
            status = "capturing",
            channel = "live",
            privacy_profile = "anonymous-local-v1",
            source = { api_version = 101050, game_version = "12.0.7", locale = "en", platform = "1" },
            session_id = "session-1788912000-1000",
            encounter_id = "encounter-1788912000-1000",
            started_at = "1788912000",
            finished_at = "",
            started_monotonic_ms = 1000,
            ended_monotonic_ms = 5,
            first_sequence = 1,
            last_sequence = 1,
            stored_event_count = 1,
            omitted_event_count = 0,
            estimated_bytes = 900,
            partial_reason = nil,
            warnings = {},
            events = {{
                session_id = "session-1788912000-1000",
                encounter_id = "encounter-1788912000-1000",
                sequence = 1,
                monotonic_ms = 0,
                kind = "encounter-start",
                payload = { reason = "combat-started" },
            }},
        }"#,
    );
    run(
        &recovery,
        r#"
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        assert(__capture().status == "partial")
        assert(__capture().partial_reason == "player-deactivated")
        assert(__capture().events[#__capture().events].kind
            == "encounter-end")
        assert(__capture().warnings.recovered_interruption == 1)
        assert(EsoWeaveDataSaved.encounter.interruptions[1].reason
            == "runtime-interrupted")
        "#,
    );

    let overflow_recovery = harness(
        r#"EsoWeaveDataSaved.encounter = {
            schema_version = 1,
            addon_version = 1,
            status = "capturing",
            channel = "live",
            privacy_profile = "anonymous-local-v1",
            source = { api_version = 101050, game_version = "12.0.7", locale = "en", platform = "1" },
            session_id = "session-1788912000-1000",
            encounter_id = "encounter-1788912000-1000",
            started_at = "1788912000",
            finished_at = "",
            started_monotonic_ms = 1000,
            ended_monotonic_ms = 5,
            first_sequence = 1,
            last_sequence = 3,
            stored_event_count = 1,
            omitted_event_count = 2,
            estimated_bytes = 900,
            partial_reason = nil,
            pending_partial_reason = "capture-overflow",
            pending_loss_from = 2,
            pending_loss_to = 3,
            pending_loss_reason = "capture-overflow",
            warnings = {},
            events = {{
                session_id = "session-1788912000-1000",
                encounter_id = "encounter-1788912000-1000",
                sequence = 1,
                monotonic_ms = 0,
                kind = "encounter-start",
                payload = { reason = "combat-started" },
            }},
        }"#,
    );
    run(
        &overflow_recovery,
        r#"
        assert(EsoWeaveDataSaved.encounter.state == "stopped")
        local capture = __capture()
        assert(capture.status == "partial")
        assert(capture.partial_reason == "capture-overflow")
        assert(capture.omitted_event_count == 2)
        local marker = capture.events[2]
        local terminal = capture.events[3]
        assert(marker.kind == "discontinuity")
        assert(marker.sequence == 4)
        assert(marker.payload.missing_sequence_from == 2)
        assert(marker.payload.missing_sequence_to == 3)
        assert(marker.payload.reason == "capture-overflow")
        assert(terminal.kind == "encounter-end")
        assert(terminal.sequence == 5)
        assert(capture.pending_partial_reason == nil)
        assert(capture.pending_loss_from == nil)
        assert(capture.pending_loss_to == nil)
        assert(capture.pending_loss_reason == nil)
        "#,
    );
}

#[test]
fn addon_identity_and_source_are_strictly_confined() {
    assert!(MANIFEST.contains("## Title: ESO Weave Data"));
    assert!(MANIFEST.contains("## AddOnVersion: 2"));
    assert!(MANIFEST.contains("## APIVersion: 101051 101050"));
    assert!(MANIFEST.contains("## SavedVariables: EsoWeaveDataSaved"));
    assert!(MANIFEST.contains("## X-ESO-Weave-Data-Managed: true"));
    assert!(MANIFEST.ends_with("Encounter.lua\n"));

    for required in [
        "MAX_EVENTS = 100000",
        "MAX_RAW_OBSERVATIONS = 100000",
        "MAX_ESTIMATED_BYTES = 33554432",
        "MAX_ACTORS = 4096",
        "MAX_SESSION_ENCOUNTERS = 1024",
        "MAX_INTERRUPTION_MARKERS = 1024",
        "EVENT_COMBAT_EVENT",
        "EVENT_EFFECT_CHANGED",
        "EVENT_POWER_UPDATE",
        "EVENT_ACTION_SLOT_ABILITY_USED",
        "EVENT_ACTIVE_WEAPON_PAIR_CHANGED",
        "EVENT_PLAYER_DEAD",
        "EVENT_PLAYER_ALIVE",
        "EVENT_BOSSES_CHANGED",
        "EVENT_ACTIVE_QUICKSLOT_CHANGED",
        "EVENT_PLAYER_DEACTIVATED",
        "GetGameTimeMilliseconds",
        "IsUnitInCombat",
        "GetFramerate",
        "GetLatency",
        "/ewencounter",
    ] {
        assert!(ADDON.contains(required), "missing {required}");
    }

    for forbidden in [
        "PixelBeacon",
        "EsoWeaveCollector",
        "CallSecureProtected",
        "RequestOpenUnsafeURL",
        "SendHTTPRequest",
        "UseItem",
        "EquipItem",
        "loadstring",
        "dofile",
        "require(",
        "os.execute",
        "io.open",
        "mode automatic",
        "ZO_CreateStringId",
        "KEYBIND_STRIP",
    ] {
        assert!(!ADDON.contains(forbidden), "forbidden surface {forbidden}");
    }
}

#[test]
fn normalized_fixture_declares_every_kind_and_its_sequence_gap() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("parse fixture");
    let events = fixture["events"].as_array().expect("event array");
    assert_eq!(
        fixture["stored_event_count"].as_u64(),
        Some(events.len() as u64)
    );
    assert_eq!(fixture["status"], "partial");
    assert_eq!(fixture["omitted_event_count"], 1);

    let kinds: BTreeSet<_> = events
        .iter()
        .map(|event| event["kind"].as_str().expect("event kind"))
        .collect();
    let required: BTreeSet<_> = [
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
    ]
    .into_iter()
    .collect();
    assert_eq!(kinds, required);

    let marker = events
        .iter()
        .find(|event| event["kind"] == "discontinuity")
        .expect("discontinuity");
    assert_eq!(marker["payload"]["missing_sequence_from"], 13);
    assert_eq!(marker["payload"]["missing_sequence_to"], 13);
    assert_eq!(marker["sequence"], 14);
}
