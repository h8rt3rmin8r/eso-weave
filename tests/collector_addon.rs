use eso_weave::data_addon::{
    catalog_checksum_from_source, embedded_checksum, CATALOG, MANAGED_MARKER, MANIFEST,
};

#[test]
fn addon_has_dedicated_identity_and_saved_variables_contract() {
    assert!(MANIFEST.contains("## Title: ESO Weave Data"));
    assert!(MANIFEST.contains("## AddOnVersion: 1"));
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
