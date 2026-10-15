use eso_weave::collector::parse_capture;
use eso_weave::data_addon::{catalog_checksum_from_source, embedded_checksum, BOOTSTRAP, CATALOG};
use mlua::{Lua, Table, Value};

const HARNESS: &str = r#"
EVENT_ADD_ON_LOADED = 1
EVENT_PLAYER_ACTIVATED = 2
EVENT_PLAYER_DEACTIVATED = 3
EVENT_PLAYER_COMBAT_STATE = 4
EVENT_SKILLS_FULL_UPDATE = 5
EVENT_SKILL_LINE_ADDED = 6
EVENT_SKILL_RANK_UPDATE = 7
EVENT_INVENTORY_SINGLE_SLOT_UPDATE = 8
EVENT_EFFECT_CHANGED = 9
EVENT_CRAFTED_ABILITY_LOCK_STATE_CHANGED = 10
EVENT_CRAFTED_ABILITY_SCRIPT_LOCK_STATE_CHANGED = 11
EVENT_ITEM_SET_COLLECTIONS_UPDATED = 12
EVENT_MANAGER = { events = {}, updates = {} }
function EVENT_MANAGER:RegisterForEvent(name, event, callback)
    self.events[event] = self.events[event] or {}; self.events[event][name] = callback
end
function EVENT_MANAGER:UnregisterForEvent(name, event)
    if self.events[event] then self.events[event][name] = nil end
end
function EVENT_MANAGER:RegisterForUpdate(name, interval, callback) self.updates[name] = callback end
function EVENT_MANAGER:UnregisterForUpdate(name) self.updates[name] = nil end
function __fire(event, ...)
    local handlers = {}; for name, callback in pairs(EVENT_MANAGER.events[event] or {}) do handlers[name] = callback end
    for _, callback in pairs(handlers) do callback(event, ...) end
end
__now, __timestamp, __combat, __ready, __skills, __reads, __saves = 1000, 1800000000, false, true, 3, 0, 0
__character, __locale, __game_version, __api = '123', 'en', '12.0.1', 101051
__messages = {}; SLASH_COMMANDS = {}
CHAT_ROUTER = {}; function CHAT_ROUTER:AddSystemMessage(text) table.insert(__messages, text) end
function d(text) error('Catalog must not write chat') end
function __tick(count)
    for i=1,(count or 1) do
        __now = __now + 10
        local callbacks = {}; for name, callback in pairs(EVENT_MANAGER.updates) do callbacks[name] = callback end
        for _, callback in pairs(callbacks) do callback() end
    end
end
function __finish()
    for i=1,100000 do
        __tick()
        if EsoWeaveDataSaved.catalog_maintenance and EsoWeaveDataSaved.catalog_maintenance.state == 'idle' then return end
    end
    error('automatic collector did not settle')
end
function GetGameTimeMilliseconds() return __now end
function GetTimeStamp() return __timestamp end
function IsUnitInCombat() return __combat end
function AreSkillsInitialized() return __ready end
function GetCurrentCharacterId() return __character end
function GetAPIVersion() return __api end
function GetESOVersionString() return __game_version end
function GetCVar() return __locale end
function GetPlatformServiceType() return 1 end
function GetWorldName() return 'NA Megaserver' end
function GetNumSkillTypes() __reads=__reads+1; if __fail then error('PRIVATE-API-VALUE') end; return 1 end
function GetNumSkillLines() return 1 end
function GetSkillLineId() return 100 end
function GetNumSkillAbilities() return __skills end
function GetSkillAbilityId(_,_,index) return 1000+index end
function GetProgressionSkillProgressionId(_,_,index) return 2000+index end
function GetSkillLineProgressionAbilityRank() return 2 end
function GetSpecificSkillAbilityInfo(_,_,index,morph,rank) return 10000+index*100+morph*10+rank, 1, 1 end
function GetAbilityName(id) return 'Ability '..id end
function GetAbilityDescription(id) return __description or ('Description '..id) end
function GetAbilityIcon(id) return '/icons/'..id..'.dds' end
function GetAbilityDuration() return 1000 end
function GetAbilityCooldown(id,caster) assert(caster=='player','GetAbilityCooldown requires caster as second argument'); return 0 end
function GetAbilityCastInfo() return false, 0 end
function GetAbilityRange() return 0, 2800 end
function GetAbilityRadius() return 0 end
function IsAbilityPassive() return false end
function IsAbilityUltimate() return false end
function GetNumCraftedAbilities() return __crafted or 2 end
function GetCraftedAbilityIdAtIndex(index) return 3000+index end
function GetCraftedAbilityDisplayName(id) return 'Crafted '..id end
function GetCraftedAbilityIcon() return '/icons/crafted.dds' end
SCRIBING_SLOT_PRIMARY, SCRIBING_SLOT_SECONDARY, SCRIBING_SLOT_TERTIARY = 1, 2, 3
function GetNumScriptsInSlotForCraftedAbility(_,slot) return slot == 1 and 1 or 0 end
function GetScriptIdAtSlotIndexForCraftedAbility() return 4001 end
function GetCraftedAbilityScriptDisplayName() return 'Shared script' end
function GetCraftedAbilityScriptGeneralDescription() return 'Shared description' end
function GetCraftedAbilityScriptDescription() return 'Shared description' end
function GetNextItemSetCollectionId(previous) return previous == nil and 5001 or nil end
function GetNumItemSetCollectionPieces() return 1 end
function GetItemSetCollectionPieceInfo() return 5002 end
function GetNumChampionDisciplines() return 1 end
function GetChampionDisciplineId() return 6001 end
function GetNumChampionDisciplineSkills() return 1 end
function GetChampionSkillId() return 6002 end
function GetChampionSkillName() return 'Champion' end
function GetChampionSkillDescription() return 'Champion description' end
function GetNumClasses() return 1 end
function GetClassInfo() return 1 end
function GetClassName() return 'Class' end
function GetUnitRaceId() return 2 end
function GetRaceName() return 'Race' end
function GetActiveCompanionDefId() return 3 end
function GetCompanionName() return 'Companion' end
BAG_WORN, BAG_BACKPACK, LINK_STYLE_DEFAULT = 1, 2, 0
ITEMTYPE_FOOD, ITEMTYPE_DRINK, ITEMTYPE_POTION, ITEMTYPE_POISON = 4, 5, 6, 7
function GetBagSize() return 2 end
function GetItemLink(bag,slot) return 'item-'..bag..'-'..slot end
function GetItemLinkItemId(link) return string.find(link,'-0') and 7001 or 7002 end
function GetItemLinkName() return 'Item' end
function GetItemLinkIcon() return '/icons/item.dds' end
function GetItemLinkItemType(link) return string.find(link,'-0') and ITEMTYPE_POTION or 1, 1 end
function GetItemLinkArmorType() return 1 end
function GetItemLinkWeaponType() return 1 end
function GetItemLinkEquipType() return 1 end
function GetItemLinkTraitInfo() return 1 end
function GetItemLinkDisplayQuality() return 2 end
function GetNumBuffs() return 1 end
function GetUnitBuffInfo() return 'Effect', 0, 0, 1, 1, '/icons/effect.dds', '', 1, 1, 1, 8001, false, true end
function GetUnitActiveMundusStoneBuffIndices() return 1 end
STAT_HEALTH_MAX, STAT_MAGICKA_MAX, STAT_STAMINA_MAX, STAT_ARMOR_RATING = 1,2,3,4
SI_DERIVEDSTATS = 100
function GetString(_,id) return 'Stat '..id end
function GetAddOnManager() return { RequestAddOnSavedVariablesPrioritySave = function(_,name) assert(name=='PixelBeacon'); __saves=__saves+1 end } end
EsoWeaveDataSaved = { schema_version=1, addon_version=1, encounter = { sentinel = 'preserved' } }
ESO_WEAVE_STARTUP_CONFIG = { catalog_disabled = false, channel = 'live' }
"#;

fn harness(before: &str) -> Lua {
    let lua = Lua::new();
    lua.load(HARNESS).exec().unwrap();
    lua.load(before).exec().unwrap();
    lua.load(BOOTSTRAP).exec().unwrap();
    lua.load(CATALOG).exec().unwrap();
    lua.load("__fire(EVENT_ADD_ON_LOADED, 'PixelBeacon')")
        .exec()
        .unwrap();
    lua
}
fn run(lua: &Lua, script: &str) {
    lua.load(script).exec().unwrap();
}
fn serialize_value(value: Value) -> String {
    match value {
        Value::Nil => "nil".into(),
        Value::Boolean(value) => value.to_string(),
        Value::Integer(value) => value.to_string(),
        Value::Number(value) if value.fract() == 0.0 => format!("{value:.0}"),
        Value::String(value) => serde_json::to_string(value.to_str().unwrap().as_ref()).unwrap(),
        Value::Table(value) => serialize_table(value),
        _ => panic!("unsupported SavedVariables value"),
    }
}
fn serialize_table(table: Table) -> String {
    let mut entries = table
        .pairs::<Value, Value>()
        .map(|entry| {
            let (key, value) = entry.unwrap();
            (
                format!("[{}]", serialize_value(key)),
                serialize_value(value),
            )
        })
        .collect::<Vec<_>>();
    entries.sort();
    format!(
        "{{{}}}",
        entries
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn source(lua: &Lua) -> String {
    format!(
        "EsoWeaveDataSaved = {}",
        serialize_table(lua.globals().get("EsoWeaveDataSaved").unwrap())
    )
}

#[test]
fn automatic_load_collects_real_double_facts_and_all_shared_relationships_quietly() {
    let lua = harness("");
    run(&lua,"assert(SLASH_COMMANDS['/ewcollect']==nil); __fire(EVENT_PLAYER_ACTIVATED); __finish(); assert(#__messages==0); assert(__saves==1); assert(EsoWeaveDataSaved.encounter.sentinel=='preserved')");
    let capture = parse_capture(source(&lua).as_bytes()).unwrap();
    assert_eq!(capture.channel.as_str(), "live");
    assert_eq!(capture.scope_key, "character-123");
    assert!(capture
        .records
        .iter()
        .any(|record| record.kind.as_str() == "ability-progression"));
    assert!(capture
        .records
        .iter()
        .any(|record| record.kind.as_str() == "ability-rank"));
    assert!(capture
        .records
        .iter()
        .any(|record| record.kind.as_str() == "ability-morph"));
    for category in [
        "ability-metadata",
        "effects-and-status",
        "items-and-gear",
        "consumables",
        "mundus-effects",
        "combat-statistics",
        "constants",
    ] {
        assert!(
            capture
                .records
                .iter()
                .any(|record| record.category == category),
            "no actual facts for {category}"
        );
    }
    let script = capture
        .records
        .iter()
        .find(|record| record.kind.as_str() == "script")
        .unwrap();
    let json = serde_json::to_value(script).unwrap();
    assert_eq!(json["parents"].as_array().unwrap().len(), 2);
    assert_eq!(
        catalog_checksum_from_source(CATALOG).unwrap(),
        embedded_checksum()
    );
}

#[test]
fn automatic_collector_defers_for_readiness_combat_and_loading_then_resumes_once() {
    let lua = harness("__ready=false");
    run(
        &lua,
        r#"__fire(EVENT_PLAYER_ACTIVATED); __tick(10); assert(__reads==0)
        __ready=true; __combat=true; __fire(EVENT_PLAYER_COMBAT_STATE,true); __tick(10); assert(__reads==0)
        __combat=false; __fire(EVENT_PLAYER_COMBAT_STATE,false); __tick();
        __fire(EVENT_PLAYER_DEACTIVATED); local reads=__reads; __tick(10); assert(__reads==reads)
        __fire(EVENT_PLAYER_ACTIVATED); __fire(EVENT_PLAYER_ACTIVATED); __finish()
        assert(EsoWeaveDataSaved.catalog.status=='complete'); assert(__saves==1); assert(#__messages==0)"#,
    );
}

#[test]
fn automatic_refresh_coalesces_events_and_unchanged_content_keeps_completed_identity() {
    let lua = harness("");
    run(
        &lua,
        r#"__fire(EVENT_PLAYER_ACTIVATED); __finish(); local old=EsoWeaveDataSaved.catalog
        for i=1,100 do __fire(EVENT_SKILLS_FULL_UPDATE) end
        __tick(1000); __finish(); assert(EsoWeaveDataSaved.catalog==old); assert(__saves==1)
        local reads=__reads; __tick(1000); assert(__reads==reads)
        __locale='de'; __character='456'; __fire(EVENT_PLAYER_ACTIVATED); __finish()
        assert(EsoWeaveDataSaved.catalog~=old); assert(EsoWeaveDataSaved.catalog.locale=='de')
        assert(EsoWeaveDataSaved.catalog.scope_key=='character-456')"#,
    );
}

#[test]
fn completed_snapshot_survives_interruption_api_failure_and_automatic_restart() {
    let lua = harness("");
    run(
        &lua,
        r#"__fire(EVENT_PLAYER_ACTIVATED); __finish(); __old=EsoWeaveDataSaved.catalog
        __fail=true; __fire(EVENT_SKILLS_FULL_UPDATE); __tick(1000)
        assert(EsoWeaveDataSaved.catalog==__old)
        assert(not string.find(EsoWeaveDataSaved.catalog_maintenance.reason or '', 'PRIVATE',1,true))
        __fail=false; __skills=4; __tick(4000); __finish(); assert(EsoWeaveDataSaved.catalog~=__old)
        __skills=5; __fire(EVENT_SKILLS_FULL_UPDATE); __tick(); __fire(EVENT_PLAYER_DEACTIVATED)
        assert(EsoWeaveDataSaved.catalog.status=='complete')"#,
    );
    let reloaded = harness(&source(&lua));
    run(&reloaded,"__skills=5; __fire(EVENT_PLAYER_ACTIVATED); __finish(); assert(EsoWeaveDataSaved.catalog.status=='complete'); assert(EsoWeaveDataSaved.encounter.sentinel=='preserved')");
    parse_capture(source(&reloaded).as_bytes()).unwrap();
}

#[test]
fn disabled_startup_is_naturally_applied_without_reading_or_deleting_saved_facts() {
    let lua=harness("ESO_WEAVE_STARTUP_CONFIG.catalog_disabled=true; EsoWeaveDataSaved.catalog={status='complete',sentinel=true}");
    run(&lua,"__fire(EVENT_PLAYER_ACTIVATED); __tick(1000); assert(__reads==0); assert(EsoWeaveDataSaved.catalog.sentinel); assert(EsoWeaveDataSaved.catalog_maintenance.state=='disabled'); ESO_WEAVE_STARTUP_CONFIG.catalog_disabled=false; __tick(1000); assert(__reads==0); assert(#__messages==0)");
    let pts = harness("ESO_WEAVE_STARTUP_CONFIG.channel='pts'");
    run(&pts, "__fire(EVENT_PLAYER_ACTIVATED); __finish()");
    assert_eq!(
        parse_capture(source(&pts).as_bytes())
            .unwrap()
            .channel
            .as_str(),
        "pts"
    );
}

#[test]
fn scan_sort_serialization_and_checksums_yield_and_preserve_last_good_on_encoding_failure() {
    let lua = harness("__skills=700");
    run(
        &lua,
        r#"__fire(EVENT_PLAYER_ACTIVATED); __tick(); assert(EsoWeaveDataSaved.catalog==nil)
        local phases={}; for i=1,100000 do
            __tick(); local state=EsoWeaveDataSaved.catalog_maintenance.state; phases[state]=(phases[state] or 0)+1
            assert((EsoWeaveDataSaved.catalog_maintenance.last_tick_steps or 0)<=64)
            assert((EsoWeaveDataSaved.catalog_maintenance.last_tick_bytes or 0)<=4096)
            if state=='idle' then break end
        end
        assert((phases.sorting or 0)>1); assert((phases.serializing or 0)>1)
        assert(EsoWeaveDataSaved.catalog and EsoWeaveDataSaved.catalog.status=='complete', EsoWeaveDataSaved.catalog_maintenance.state); __old=EsoWeaveDataSaved.catalog
        __skills=2; __description=string.rep(string.char(1),12000); __fire(EVENT_SKILLS_FULL_UPDATE)
        local encoded=false; for i=1,1000 do __tick(); if EsoWeaveDataSaved.catalog_maintenance.state=='serializing' then encoded=true end end
        assert(encoded); assert(EsoWeaveDataSaved.catalog==__old); assert(#__messages==0)"#,
    );
    parse_capture(source(&lua).as_bytes()).unwrap();
}

#[test]
fn unknown_saved_versions_and_malformed_config_remain_preserved_inactive() {
    let lua =
        harness("EsoWeaveDataSaved.schema_version=99; EsoWeaveDataSaved.catalog={sentinel=true}");
    run(&lua,"__fire(EVENT_PLAYER_ACTIVATED); __tick(1000); assert(__reads==0); assert(EsoWeaveDataSaved.schema_version==99); assert(EsoWeaveDataSaved.catalog.sentinel)");
    let invalid = harness("ESO_WEAVE_STARTUP_CONFIG={catalog_disabled='false',channel='invalid'}");
    run(
        &invalid,
        "__fire(EVENT_PLAYER_ACTIVATED); __tick(1000); assert(__reads==0)",
    );
}

#[test]
fn fresh_initialization_native_states_and_catalog_failure_leave_other_modules_running() {
    let fresh = harness("EsoWeaveDataSaved=nil");
    run(
        &fresh,
        "__fire(EVENT_PLAYER_ACTIVATED); __finish(); assert(EsoWeaveDataSaved.schema_version==1)",
    );
    let base = parse_capture(source(&fresh).as_bytes()).unwrap();
    for state in ["off", "on", "processing-disabled"] {
        let lua=harness(&format!("__native_state='{state}'; __signal_ticks=0; EVENT_MANAGER:RegisterForUpdate('PixelBeaconSignals',10,function() __signal_ticks=__signal_ticks+1 end)"));
        run(&lua,"__fire(EVENT_PLAYER_ACTIVATED); __finish(); __good=EsoWeaveDataSaved.catalog; __fail=true; __fire(EVENT_SKILLS_FULL_UPDATE); __tick(1000); assert(__signal_ticks>1000); assert(EsoWeaveDataSaved.catalog==__good); assert(#__messages==0)");
        assert_eq!(
            base.records,
            parse_capture(source(&lua).as_bytes()).unwrap().records
        );
    }
}

#[test]
fn known_item_and_effect_noise_does_not_rescan_or_request_additional_saves() {
    let lua = harness("");
    run(
        &lua,
        r#"__fire(EVENT_PLAYER_ACTIVATED); __finish(); local reads=__reads
        for i=1,100 do
            __fire(EVENT_INVENTORY_SINGLE_SLOT_UPDATE,BAG_BACKPACK,0)
            __fire(EVENT_EFFECT_CHANGED,1,1,'Effect','player',0,0,1,'icon','',1,1,1,'name',1,8001,1)
        end
        __tick(1000); assert(__reads==reads); assert(__saves==1)"#,
    );
}

#[test]
fn passive_ranks_and_crafted_skill_lines_use_their_supported_api_views() {
    let lua = harness(
        r#"function IsSkillAbilityPassive(_,_,index) return index==1 end
        function GetNumPassiveSkillRanks() return 3 end
        function IsCraftedAbilitySkill(_,_,index) return index==2 end
        local original=GetSpecificSkillAbilityInfo
        function GetSpecificSkillAbilityInfo(a,b,index,morph,rank)
            assert(index~=2,'crafted variants are not standard skill variants')
            assert(index~=1 or morph==0,'passives have no morph choices')
            return original(a,b,index,morph,rank)
        end"#,
    );
    run(&lua,"__fire(EVENT_PLAYER_ACTIVATED); __finish(); assert(EsoWeaveDataSaved.catalog.status=='complete')");
    let capture = parse_capture(source(&lua).as_bytes()).unwrap();
    assert!(capture
        .records
        .iter()
        .any(|record| record.kind.as_str() == "ability-rank" && record.stable_id == 10103));
    assert!(!capture
        .records
        .iter()
        .any(|record| record.kind.as_str() == "ability-morph" && record.stable_id == 10101));
}

#[test]
fn documented_unlock_events_schedule_refresh_without_a_player_action() {
    let lua = harness("");
    run(
        &lua,
        r#"__fire(EVENT_PLAYER_ACTIVATED); __finish(); __crafted=3
        __fire(EVENT_CRAFTED_ABILITY_LOCK_STATE_CHANGED,3003,true,false); __tick(200); __finish()
        assert(EsoWeaveDataSaved.catalog.status=='complete'); assert(#__messages==0)"#,
    );
    let capture = parse_capture(source(&lua).as_bytes()).unwrap();
    assert_eq!(
        capture
            .records
            .iter()
            .filter(|record| record.kind.as_str() == "crafted-ability")
            .count(),
        3
    );
}

#[test]
fn missing_actual_provenance_defers_instead_of_publishing_a_guessed_identity() {
    let lua = harness("__game_version=nil");
    run(&lua,"__fire(EVENT_PLAYER_ACTIVATED); __tick(200); assert(__reads==0); assert(EsoWeaveDataSaved.catalog==nil); __game_version='eso.live.12.1.0.1234567'; __tick(4000); __finish()");
    let capture = parse_capture(source(&lua).as_bytes()).unwrap();
    assert_eq!(capture.game_version, "eso.live.12.1.0.1234567");
}

#[test]
fn a_constants_only_scan_reports_its_reason_and_retries_without_publishing_empty_game_data() {
    let lua = harness(
        r#"function GetNumSkillTypes() return 0 end
        function GetNumCraftedAbilities() return 0 end
        function GetNextItemSetCollectionId() return nil end
        function GetNumChampionDisciplines() return 0 end
        function GetNumClasses() return 0 end
        function GetUnitRaceId() return 0 end
        function GetActiveCompanionDefId() return 0 end
        function GetBagSize() return 0 end
        function GetNumBuffs() return 0 end
        function GetUnitActiveMundusStoneBuffIndices() end
        STAT_HEALTH_MAX,STAT_MAGICKA_MAX,STAT_STAMINA_MAX,STAT_ARMOR_RATING=nil,nil,nil,nil"#,
    );
    run(&lua,"__fire(EVENT_PLAYER_ACTIVATED); __tick(1000); assert(EsoWeaveDataSaved.catalog==nil); assert(EsoWeaveDataSaved.catalog_maintenance.reason=='no-useful-definitions'); assert(EsoWeaveDataSaved.catalog_maintenance.state=='retry-wait')");
}

#[test]
fn malformed_previous_metadata_cannot_strand_recovery_or_be_reused_as_a_noop() {
    for mutation in [
        "EsoWeaveDataSaved.catalog.coverage[1]=true",
        "EsoWeaveDataSaved.catalog.chunks[1]=true",
        "EsoWeaveDataSaved.catalog.chunks[1].byte_count=1",
        "EsoWeaveDataSaved.catalog.selected_categories={}",
        "EsoWeaveDataSaved.catalog.warnings={string.rep('x',100000)}",
    ] {
        let lua = harness("");
        run(
            &lua,
            "__fire(EVENT_PLAYER_ACTIVATED); __finish(); __old=EsoWeaveDataSaved.catalog",
        );
        run(&lua, mutation);
        run(&lua,"__fire(EVENT_SKILLS_FULL_UPDATE); __tick(200); __finish(); assert(EsoWeaveDataSaved.catalog~=__old); assert(#__messages==0)");
        parse_capture(source(&lua).as_bytes()).unwrap();
    }
}

#[test]
fn natural_restart_rebuilds_untrusted_prior_array_shapes_before_noop_reuse() {
    let first = harness("");
    run(&first, "__fire(EVENT_PLAYER_ACTIVATED); __finish()");
    let saved = source(&first);
    for mutation in [
        "EsoWeaveDataSaved.catalog.warnings={[2]='unexpected'}",
        "EsoWeaveDataSaved.catalog.unrecognized='field'",
    ] {
        let restarted = harness(&format!("{saved}; {mutation}"));
        run(&restarted, "__fire(EVENT_PLAYER_ACTIVATED); __finish()");
        parse_capture(source(&restarted).as_bytes()).unwrap();
    }
}

#[test]
fn source_identity_is_revalidated_before_publishing_a_completed_generation() {
    let lua =
        harness("function GetAbilityDescription(id) __locale='de'; return 'Description '..id end");
    run(&lua,"__fire(EVENT_PLAYER_ACTIVATED); __finish(); assert(EsoWeaveDataSaved.catalog.locale=='de'); assert(EsoWeaveDataSaved.catalog.status=='complete')");
    assert_eq!(parse_capture(source(&lua).as_bytes()).unwrap().locale, "de");
}
