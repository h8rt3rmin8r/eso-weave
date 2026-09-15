use std::collections::BTreeSet;

use eso_weave::catalog::Channel;
use eso_weave::encounter::{
    canonical_bytes, import_encounter, load_encounter, parse_capture, ImportRequest,
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
function __raw_observation(sourceId, occurrence)
    occurrence = occurrence or 1
    local observations = EsoWeaveDataSaved.encounter.raw_observations
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
    for _, event in ipairs(EsoWeaveDataSaved.encounter.events) do
        if event.source_sequence == sourceSequence then count = count + 1 end
    end
    return count
end
function __assert_raw_loss(reason)
    local saved = EsoWeaveDataSaved.encounter
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
fn addon_is_dormant_until_one_explicit_channel_arm() {
    let lua = harness("");
    run(
        &lua,
        r#"
        assert(EsoWeaveDataSaved.encounter.status == "idle")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveDataSaved.encounter.status == "idle")

        SLASH_COMMANDS["/ewencounter"]("arm live")
        assert(EsoWeaveDataSaved.encounter.status == "armed")
        assert(EsoWeaveDataSaved.encounter.channel == "live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveDataSaved.encounter.status == "capturing")
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(EsoWeaveDataSaved.encounter.status == "complete")
        assert(EsoWeaveDataSaved.encounter.events[1].kind == "encounter-start")
        assert(EsoWeaveDataSaved.encounter.events[#EsoWeaveDataSaved.encounter.events].kind == "encounter-end")

        SLASH_COMMANDS["/ewencounter"]("arm pts")
        assert(EsoWeaveDataSaved.encounter.status == "complete")
        SLASH_COMMANDS["/ewencounter"]("clear")
        assert(EsoWeaveDataSaved.encounter.status == "complete")
        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        assert(EsoWeaveDataSaved.encounter.status == "idle")
        SLASH_COMMANDS["/ewencounter"]("arm pts")
        assert(EsoWeaveDataSaved.encounter.channel == "pts")
        SLASH_COMMANDS["/ewencounter"]("disarm")
        assert(EsoWeaveDataSaved.encounter.status == "idle")

        __in_combat = true
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveDataSaved.encounter.status == "armed")
        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveDataSaved.encounter.status == "capturing")
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(EsoWeaveDataSaved.encounter.status == "complete")
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
        assert(EsoWeaveDataSaved.encounter == __legacy_encounter)
        assert(EsoWeaveDataSaved.encounter.schema_version == 1)
        assert(EsoWeaveDataSaved.encounter.status == "complete")
        assert(EsoWeaveDataSaved.encounter.raw_observations == nil)
        assert(EsoWeaveDataSaved.encounter.raw_observation_count == nil)
        assert(EsoWeaveDataSaved.encounter.events[2].payload.complete == true)
        SLASH_COMMANDS["/ewencounter"]("arm live")
        assert(EsoWeaveDataSaved.encounter == __legacy_encounter)
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

        assert(EsoWeaveDataSaved.encounter.status == "partial")
        assert(EsoWeaveDataSaved.encounter.partial_reason == "clock-reset")
        assert(EsoWeaveDataSaved.encounter.schema_version == 2)
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
        for _, event in ipairs(EsoWeaveDataSaved.encounter.events) do
            assert(required[event.kind] ~= nil, event.kind)
            required[event.kind] = true
            assert(event.sequence > previousSequence)
            assert(event.monotonic_ms >= previousTime)
            assert(event.session_id == EsoWeaveDataSaved.encounter.session_id)
            assert(event.encounter_id == EsoWeaveDataSaved.encounter.encounter_id)
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
        assert(EsoWeaveDataSaved.encounter.stored_event_count == #EsoWeaveDataSaved.encounter.events)

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

        local raw = EsoWeaveDataSaved.encounter.raw_observations
        assert(EsoWeaveDataSaved.encounter.raw_observation_count == #raw)
        assert(EsoWeaveDataSaved.encounter.raw_omitted_observation_count == 0)
        assert(EsoWeaveDataSaved.encounter.raw_first_sequence == raw[1].sequence)
        assert(EsoWeaveDataSaved.encounter.raw_last_sequence == raw[#raw].sequence)
        local previousRawSequence = 0
        local previousRawTime = 0
        for _, observation in ipairs(raw) do
            assert(observation.sequence > previousRawSequence)
            assert(observation.monotonic_ms >= previousRawTime)
            assert(observation.session_id == EsoWeaveDataSaved.encounter.session_id)
            assert(observation.encounter_id == EsoWeaveDataSaved.encounter.encounter_id)
            previousRawSequence = observation.sequence
            previousRawTime = observation.monotonic_ms
        end
        local nextOrdinal = {}
        for _, event in ipairs(EsoWeaveDataSaved.encounter.events) do
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
        assert(EsoWeaveDataSaved.encounter.status == "complete")
        assert(EsoWeaveDataSaved.encounter.raw_omitted_observation_count == 0)
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
        for _, event in ipairs(EsoWeaveDataSaved.encounter.events) do
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
        assert(EsoWeaveDataSaved.encounter.raw_loss.missing_sequence_from == 1)
        assert(EsoWeaveDataSaved.encounter.events[1].kind == "encounter-start")
        assert(EsoWeaveDataSaved.encounter.events[1].source_sequence == 1)
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
        assert(EsoWeaveDataSaved.encounter.status == "partial")
        assert(EsoWeaveDataSaved.encounter.partial_reason == "clock-reset")
        assert(__raw_observation("clock-reset").source_kind == "lifecycle")
        assert(EsoWeaveDataSaved.encounter.events[3].kind == "discontinuity")
        assert(EsoWeaveDataSaved.encounter.events[3].payload.reason == "capture-overflow")
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
        assert(EsoWeaveDataSaved.encounter.pending_partial_reason == "capture-overflow")
        assert(EsoWeaveDataSaved.encounter.pending_loss_from ~= nil)
        assert(EsoWeaveDataSaved.encounter.pending_loss_to
            - EsoWeaveDataSaved.encounter.pending_loss_from + 1
            == EsoWeaveDataSaved.encounter.omitted_event_count)
        assert(EsoWeaveDataSaved.encounter.pending_loss_reason == "capture-overflow")
        __fire(EVENT_PLAYER_COMBAT_STATE, false)

        assert(EsoWeaveDataSaved.encounter.status == "partial")
        assert(EsoWeaveDataSaved.encounter.partial_reason == "capture-overflow")
        assert(#EsoWeaveDataSaved.encounter.events <= 6)
        assert(EsoWeaveDataSaved.encounter.estimated_bytes <= 1048576)
        assert(EsoWeaveDataSaved.encounter.omitted_event_count > 0)
        assert(EsoWeaveDataSaved.encounter.warnings.actor_limit > 0)
        local marker = EsoWeaveDataSaved.encounter.events[#EsoWeaveDataSaved.encounter.events - 1]
        local terminal = EsoWeaveDataSaved.encounter.events[#EsoWeaveDataSaved.encounter.events]
        assert(marker.kind == "discontinuity")
        assert(marker.payload.reason == "capture-overflow")
        assert(marker.payload.missing_sequence_to - marker.payload.missing_sequence_from + 1
            == EsoWeaveDataSaved.encounter.omitted_event_count)
        assert(marker.sequence == marker.payload.missing_sequence_to + 1)
        assert(terminal.kind == "encounter-end")
        assert(terminal.sequence == marker.sequence + 1)
        assert(terminal.payload.complete == false)
        assert(EsoWeaveDataSaved.encounter.pending_partial_reason == nil)
        assert(EsoWeaveDataSaved.encounter.pending_loss_from == nil)
        assert(EsoWeaveDataSaved.encounter.pending_loss_to == nil)
        assert(EsoWeaveDataSaved.encounter.pending_loss_reason == nil)
        __assert_raw_loss("record-limit")
        assert(EsoWeaveDataSaved.encounter.raw_last_sequence
            - EsoWeaveDataSaved.encounter.raw_first_sequence + 1
            == EsoWeaveDataSaved.encounter.raw_observation_count
                + EsoWeaveDataSaved.encounter.raw_omitted_observation_count)
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
        assert(EsoWeaveDataSaved.encounter.status == "partial")
        assert(EsoWeaveDataSaved.encounter.partial_reason == "user-stopped")
        assert(EVENT_MANAGER.events[EVENT_COMBAT_EVENT] == nil)
        assert(next(EVENT_MANAGER.updates) == nil)

        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_PLAYER_DEACTIVATED)
        assert(EsoWeaveDataSaved.encounter.partial_reason == "player-deactivated")
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
        assert(EsoWeaveDataSaved.encounter.partial_reason == "callback-failed")
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
        for _, event in ipairs(EsoWeaveDataSaved.encounter.events) do
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
        for _, event in ipairs(EsoWeaveDataSaved.encounter.events) do
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
        assert(EsoWeaveDataSaved.encounter.status == "partial")
        assert(EsoWeaveDataSaved.encounter.estimated_bytes <= 8192)
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
        assert(EsoWeaveDataSaved.encounter.status == "partial")
        assert(EsoWeaveDataSaved.encounter.partial_reason == "player-deactivated")
        assert(EsoWeaveDataSaved.encounter.events[#EsoWeaveDataSaved.encounter.events].kind
            == "encounter-end")
        assert(EsoWeaveDataSaved.encounter.warnings.recovered_interruption == 1)
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
        assert(EsoWeaveDataSaved.encounter.status == "partial")
        assert(EsoWeaveDataSaved.encounter.partial_reason == "capture-overflow")
        assert(EsoWeaveDataSaved.encounter.omitted_event_count == 2)
        local marker = EsoWeaveDataSaved.encounter.events[2]
        local terminal = EsoWeaveDataSaved.encounter.events[3]
        assert(marker.kind == "discontinuity")
        assert(marker.sequence == 4)
        assert(marker.payload.missing_sequence_from == 2)
        assert(marker.payload.missing_sequence_to == 3)
        assert(marker.payload.reason == "capture-overflow")
        assert(terminal.kind == "encounter-end")
        assert(terminal.sequence == 5)
        assert(EsoWeaveDataSaved.encounter.pending_partial_reason == nil)
        assert(EsoWeaveDataSaved.encounter.pending_loss_from == nil)
        assert(EsoWeaveDataSaved.encounter.pending_loss_to == nil)
        assert(EsoWeaveDataSaved.encounter.pending_loss_reason == nil)
        "#,
    );
}

#[test]
fn addon_identity_and_source_are_strictly_confined() {
    assert!(MANIFEST.contains("## Title: ESO Weave Data"));
    assert!(MANIFEST.contains("## AddOnVersion: 1"));
    assert!(MANIFEST.contains("## APIVersion: 101051 101050"));
    assert!(MANIFEST.contains("## SavedVariables: EsoWeaveDataSaved"));
    assert!(MANIFEST.contains("## X-ESO-Weave-Data-Managed: true"));
    assert!(MANIFEST.ends_with("Encounter.lua\n"));

    for required in [
        "MAX_EVENTS = 100000",
        "MAX_ESTIMATED_BYTES = 33554432",
        "MAX_ACTORS = 4096",
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
