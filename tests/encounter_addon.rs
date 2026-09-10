use std::collections::BTreeSet;

use mlua::Lua;
use serde_json::Value;

const MANIFEST: &str = include_str!("../addon/EsoWeaveEncounter/EsoWeaveEncounter.txt");
const ADDON: &str = include_str!("../addon/EsoWeaveEncounter/EsoWeaveEncounter.lua");
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
    return visit(EsoWeaveEncounterSaved)
end
"#;

fn harness(prelude: &str) -> Lua {
    let lua = Lua::new();
    lua.load(HARNESS).exec().expect("install ESO API harness");
    lua.load(prelude).exec().expect("install test overrides");
    lua.load(ADDON).exec().expect("load encounter addon");
    lua.load("__fire(EVENT_ADD_ON_LOADED, 'EsoWeaveEncounter')")
        .exec()
        .expect("activate encounter addon");
    lua
}

fn run(lua: &Lua, script: &str) {
    lua.load(script).exec().expect("run encounter scenario");
}

#[test]
fn addon_is_dormant_until_one_explicit_channel_arm() {
    let lua = harness("");
    run(
        &lua,
        r#"
        assert(EsoWeaveEncounterSaved.status == "idle")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveEncounterSaved.status == "idle")

        SLASH_COMMANDS["/ewencounter"]("arm live")
        assert(EsoWeaveEncounterSaved.status == "armed")
        assert(EsoWeaveEncounterSaved.channel == "live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveEncounterSaved.status == "capturing")
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(EsoWeaveEncounterSaved.status == "complete")
        assert(EsoWeaveEncounterSaved.events[1].kind == "encounter-start")
        assert(EsoWeaveEncounterSaved.events[#EsoWeaveEncounterSaved.events].kind == "encounter-end")

        SLASH_COMMANDS["/ewencounter"]("arm pts")
        assert(EsoWeaveEncounterSaved.status == "complete")
        SLASH_COMMANDS["/ewencounter"]("clear")
        assert(EsoWeaveEncounterSaved.status == "complete")
        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        assert(EsoWeaveEncounterSaved.status == "idle")
        SLASH_COMMANDS["/ewencounter"]("arm pts")
        assert(EsoWeaveEncounterSaved.channel == "pts")
        SLASH_COMMANDS["/ewencounter"]("disarm")
        assert(EsoWeaveEncounterSaved.status == "idle")

        __in_combat = true
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveEncounterSaved.status == "armed")
        __in_combat = false
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        __in_combat = true
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        assert(EsoWeaveEncounterSaved.status == "capturing")
        __fire(EVENT_PLAYER_COMBAT_STATE, false)
        assert(EsoWeaveEncounterSaved.status == "complete")
        "#,
    );
}

#[test]
fn representative_capture_has_every_family_and_no_private_callback_text() {
    let lua = harness("");
    run(
        &lua,
        r#"
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __advance(10)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_DAMAGE, false, "AbilitySecret",
            0, 0, "AccountSecret", 1, "CharacterSecret", 2, 1200, 3, 4,
            true, 9001, 9002, 7001, 50)
        __advance(10)
        __fire(EVENT_COMBAT_EVENT, ACTION_RESULT_HEAL, false, "HealSecret",
            0, 0, "AccountSecret", 1, "CharacterSecret", 2, 300, 3, 0,
            true, 9001, 9001, 7002, 25)
        __advance(10)
        __fire(EVENT_EFFECT_CHANGED, 1, 2, "EffectSecret", "group1", 1.1,
            2.2, 3, "IconSecret", 0, 4, 5, 6, "UnitSecret", 9001, 7100, 1)
        __advance(10)
        __fire(EVENT_POWER_UPDATE, "group1", 1, 3, 800, 1000, 1000)
        __advance(10)
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 3)
        __advance(10)
        __fire(EVENT_ACTIVE_WEAPON_PAIR_CHANGED, 2, false)
        __advance(10)
        __fire(EVENT_PLAYER_DEAD)
        __advance(10)
        __fire(EVENT_PLAYER_ALIVE)
        __advance(10)
        __fire(EVENT_ACTIVE_QUICKSLOT_CHANGED, 4)
        __advance(1000)
        __run_updates()

        __now = 500
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 4)
        __boss_value = 4000
        __advance(1000)
        __run_updates()
        __fire(EVENT_PLAYER_COMBAT_STATE, false)

        assert(EsoWeaveEncounterSaved.status == "partial")
        assert(EsoWeaveEncounterSaved.partial_reason == "clock-reset")
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
        for _, event in ipairs(EsoWeaveEncounterSaved.events) do
            assert(required[event.kind] ~= nil, event.kind)
            required[event.kind] = true
            assert(event.sequence > previousSequence)
            assert(event.monotonic_ms >= previousTime)
            assert(event.session_id == EsoWeaveEncounterSaved.session_id)
            assert(event.encounter_id == EsoWeaveEncounterSaved.encounter_id)
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
        assert(EsoWeaveEncounterSaved.stored_event_count == #EsoWeaveEncounterSaved.events)
        assert(not __contains_saved_string("AccountSecret"))
        assert(not __contains_saved_string("CharacterSecret"))
        assert(not __contains_saved_string("AbilitySecret"))
        assert(not __contains_saved_string("EffectSecret"))
        assert(not __contains_saved_string("group1"))
        assert(not __contains_saved_string("UnitSecret"))
        "#,
    );
}

#[test]
fn overflow_and_actor_limits_preserve_terminal_capacity_and_exact_loss() {
    let lua = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 6, max_estimated_bytes = 8192, max_actors = 1 }",
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
        __fire(EVENT_PLAYER_COMBAT_STATE, false)

        assert(EsoWeaveEncounterSaved.status == "partial")
        assert(EsoWeaveEncounterSaved.partial_reason == "capture-overflow")
        assert(#EsoWeaveEncounterSaved.events <= 6)
        assert(EsoWeaveEncounterSaved.estimated_bytes <= 8192)
        assert(EsoWeaveEncounterSaved.omitted_event_count > 0)
        assert(EsoWeaveEncounterSaved.warnings.actor_limit > 0)
        local marker = EsoWeaveEncounterSaved.events[#EsoWeaveEncounterSaved.events - 1]
        local terminal = EsoWeaveEncounterSaved.events[#EsoWeaveEncounterSaved.events]
        assert(marker.kind == "discontinuity")
        assert(marker.payload.reason == "capture-overflow")
        assert(marker.payload.missing_sequence_to - marker.payload.missing_sequence_from + 1
            == EsoWeaveEncounterSaved.omitted_event_count)
        assert(marker.sequence == marker.payload.missing_sequence_to + 1)
        assert(terminal.kind == "encounter-end")
        assert(terminal.sequence == marker.sequence + 1)
        assert(terminal.payload.complete == false)
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
        assert(EsoWeaveEncounterSaved.status == "partial")
        assert(EsoWeaveEncounterSaved.partial_reason == "user-stopped")
        assert(EVENT_MANAGER.events[EVENT_COMBAT_EVENT] == nil)
        assert(next(EVENT_MANAGER.updates) == nil)

        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        __fire(EVENT_PLAYER_DEACTIVATED)
        assert(EsoWeaveEncounterSaved.partial_reason == "player-deactivated")
        assert(EVENT_MANAGER.events[EVENT_EFFECT_CHANGED] == nil)

        SLASH_COMMANDS["/ewencounter"]("clear confirm")
        SLASH_COMMANDS["/ewencounter"]("arm live")
        __fire(EVENT_PLAYER_COMBAT_STATE, true)
        GetSlotBoundId = function() error("injected callback failure") end
        __fire(EVENT_ACTION_SLOT_ABILITY_USED, 3)
        assert(EsoWeaveEncounterSaved.partial_reason == "callback-failed")
        assert(EVENT_MANAGER.events[EVENT_ACTION_SLOT_ABILITY_USED] == nil)
        assert(next(EVENT_MANAGER.updates) == nil)
        assert(not __contains_saved_string("injected callback failure"))
        "#,
    );
}

#[test]
fn byte_overflow_and_saved_interruption_remain_bounded_and_partial() {
    let lua = harness(
        "EsoWeaveEncounterTestLimits = { max_events = 100, max_estimated_bytes = 4096, max_actors = 20 }",
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
        assert(EsoWeaveEncounterSaved.status == "partial")
        assert(EsoWeaveEncounterSaved.estimated_bytes <= 4096)
        assert(EsoWeaveEncounterSaved.events[#EsoWeaveEncounterSaved.events - 1].kind
            == "discontinuity")

        "#,
    );

    let recovery = harness(
        r#"EsoWeaveEncounterSaved = {
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
        assert(EsoWeaveEncounterSaved.status == "partial")
        assert(EsoWeaveEncounterSaved.partial_reason == "player-deactivated")
        assert(EsoWeaveEncounterSaved.events[#EsoWeaveEncounterSaved.events].kind
            == "encounter-end")
        assert(EsoWeaveEncounterSaved.warnings.recovered_interruption == 1)
        "#,
    );
}

#[test]
fn addon_identity_and_source_are_strictly_confined() {
    assert!(MANIFEST.contains("## Title: ESO Weave Encounter"));
    assert!(MANIFEST.contains("## AddOnVersion: 1"));
    assert!(MANIFEST.contains("## APIVersion: 101051 101050"));
    assert!(MANIFEST.contains("## SavedVariables: EsoWeaveEncounterSaved"));
    assert!(MANIFEST.contains("## X-ESO-Weave-Encounter-Managed: true"));
    assert!(MANIFEST.ends_with("EsoWeaveEncounter.lua\n"));

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
