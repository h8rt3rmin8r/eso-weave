use eso_weave::data_addon::{
    catalog_checksum_from_source, embedded_checksum, CATALOG, MANAGED_MARKER, MANIFEST,
};

#[test]
fn catalog_messages_explain_replacement_cancellation_and_hide_callback_errors() {
    let lua = mlua::Lua::new();
    lua.load(r#"
        EVENT_ADD_ON_LOADED = 1
        EVENT_PLAYER_COMBAT_STATE = 2
        EVENT_PLAYER_DEACTIVATED = 3
        EVENT_MANAGER = { events = {}, updates = {} }
        function EVENT_MANAGER:RegisterForEvent(name, event, callback) self.events[event] = callback end
        function EVENT_MANAGER:UnregisterForEvent(name, event) self.events[event] = nil end
        function EVENT_MANAGER:RegisterForUpdate(name, interval, callback) self.updates[name] = callback end
        function EVENT_MANAGER:UnregisterForUpdate(name) self.updates[name] = nil end
        __messages = {}
        SLASH_COMMANDS = {}
        EsoWeaveDataSaved = { encounter = { sentinel = true } }
        function d(text) table.insert(__messages, text) end
        function IsUnitInCombat() return false end
        function GetTimeStamp() return 1 end
        function GetAPIVersion() return 101050 end
        function GetCVar() return 'en' end
        function GetGameTimeMilliseconds() return 1 end
        function GetNumSkillTypes() error('PRIVATE-CALLBACK-VALUE') end
    "#).exec().unwrap();
    lua.load(CATALOG).exec().unwrap();
    lua.load(
        r#"
        EVENT_MANAGER.events[EVENT_ADD_ON_LOADED](1, 'EsoWeaveData')
        SLASH_COMMANDS['/ewcollect']('help')
        assert(string.find(table.concat(__messages, '\n'), 'replaces', 1, true))
        SLASH_COMMANDS['/ewcollect']('start live player-skills')
        SLASH_COMMANDS['/ewcollect']('cancel')
        assert(EsoWeaveDataSaved.catalog.status == 'cancelled')
        assert(string.find(__messages[#__messages], 'incomplete', 1, true))
        assert(string.find(__messages[#__messages], 'Encounter History', 1, true))
        SLASH_COMMANDS['/ewcollect']('start live player-skills')
        EVENT_MANAGER.updates['EsoWeaveDataCatalogUpdate']()
        assert(EsoWeaveDataSaved.catalog.status == 'failed')
        local failed = __messages[#__messages]
        assert(not string.find(failed, 'PRIVATE-CALLBACK-VALUE', 1, true))
        assert(string.find(failed, '/ewcollect start', 1, true))
        SLASH_COMMANDS['/ewcollect']('clear confirm')
        assert(EsoWeaveDataSaved.catalog == nil)
        assert(EsoWeaveDataSaved.encounter.sentinel == true)
    "#,
    )
    .exec()
    .unwrap();
}

#[test]
fn data_addon_has_shared_identity_and_saved_variables_contract() {
    assert!(MANIFEST.contains("## Title: ESO Weave Data"));
    assert!(MANIFEST.contains("## AddOnVersion: 2"));
    assert!(MANIFEST.contains("## APIVersion: 101051 101050"));
    assert!(MANIFEST.contains("## SavedVariables: EsoWeaveDataSaved"));
    assert!(MANIFEST.lines().any(|line| line.trim() == MANAGED_MARKER));
    assert!(MANIFEST.ends_with("Encounter.lua\n"));
    assert!(!MANIFEST.contains("PixelBeacon.lua"));
    assert_eq!(
        catalog_checksum_from_source(CATALOG).unwrap(),
        embedded_checksum()
    );
}

#[test]
fn addon_exposes_explicit_bounded_lifecycle() {
    for command in ["/ewcollect", "start", "status", "resume", "cancel", "help"] {
        assert!(CATALOG.contains(command), "missing {command}");
    }
    for behavior in [
        "IsUnitInCombat(\"player\")",
        "EVENT_PLAYER_COMBAT_STATE",
        "RegisterForUpdate",
        "MAX_RECORDS_PER_TICK",
        "MAX_MILLISECONDS_PER_TICK",
        "MAX_SNAPSHOT_BYTES",
        "MAX_RECORDS",
        "MAX_STRING_BYTES",
        "MAX_CHUNK_BYTES",
        "table.sort",
        "adler32",
        "checkpoint",
        "cancellation_reason",
        "/reloadui",
        "logout",
    ] {
        assert!(CATALOG.contains(behavior), "missing {behavior}");
    }
}

#[test]
fn addon_names_only_approved_iterator_categories_and_stable_id_calls() {
    for category in [
        "player-skills",
        "crafted-abilities",
        "item-sets",
        "champion-skills",
        "companions-races-classes",
    ] {
        assert!(CATALOG.contains(category), "missing {category}");
    }
    for api in [
        "GetSkillLineId",
        "GetSkillAbilityId",
        "GetCraftedAbilityIdAtIndex",
        "GetScriptIdAtSlotIndexForCraftedAbility",
        "GetNextItemSetCollectionId",
        "GetItemSetCollectionPieceInfo",
        "GetChampionDisciplineId",
        "GetChampionSkillId",
        "GetClassInfo",
        "GetUnitRaceId",
    ] {
        assert!(CATALOG.contains(api), "missing stable ID API {api}");
    }
    for forbidden in [
        "PixelBeacon",
        "CallSecureProtected",
        "UseItem",
        "EquipItem",
        "RequestOpenUnsafeURL",
        "SendHTTPRequest",
        "EVENT_COMBAT_EVENT",
        "IsPublicTestServer",
    ] {
        assert!(
            !CATALOG.contains(forbidden),
            "forbidden addon surface {forbidden}"
        );
    }
}
