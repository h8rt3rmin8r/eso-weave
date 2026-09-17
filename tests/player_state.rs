use std::sync::Arc;

use eso_weave::player_state::{
    bootstrap_content, observed, project, Capabilities, ProjectionInput, SnapshotPublisher,
    NON_PUBLIC_STATES, PUBLIC_PATHS, SCHEMA_VERSION,
};
use serde_json::json;
use time::OffsetDateTime;

use eso_weave::fishing::{FishingConfig, FishingState};
use eso_weave::game::{
    BeaconFreshness, FocusObservation, GameObservations, GameRuntime, InstallationState,
    SurfaceObservation,
};
use eso_weave::input::NativeBindingSet;
use eso_weave::pixelbus::{
    ActiveBar, BusLayout, CombatSignal, CooldownSet, LayoutState, LifeState, MenuSurface,
    MovementSignal, QuickslotState, ReaderConfig, ResourceLevel, ResourceSet, RollDodgeState,
    SlotCooldown, TravelState, UltimateTelemetry, UltimateValue, WeaponClass, WorldState,
};
use eso_weave::potion::{AutoPotionConfig, AutoPotionState};
use eso_weave::weave::{LatencyConfig, WeaveConfig};

#[test]
fn bootstrap_document_has_every_stable_domain_and_public_contract_path() {
    let publisher = SnapshotPublisher::default();
    let document = publisher.current().document(7);
    let value = serde_json::to_value(document).unwrap();

    assert_eq!(value["schema_version"], SCHEMA_VERSION);
    assert_eq!(value["snapshot_revision"], 1);
    assert_eq!(value["service_generation"], 7);
    for domain in [
        "application",
        "game",
        "pixel_bus",
        "player",
        "automation",
        "interpretation",
    ] {
        assert!(value.get(domain).is_some(), "missing domain {domain}");
    }
    for path in PUBLIC_PATHS {
        assert!(
            json_path(&value, path).is_some(),
            "missing public path {path}"
        );
    }
    assert_eq!(value["player"]["bindings"].as_array().unwrap().len(), 11);
    assert_eq!(
        value["automation"]["weave"]["slots"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
}

#[test]
fn unchanged_semantics_preserve_the_revision_and_changed_semantics_advance_once() {
    let publisher = SnapshotPublisher::default();
    let at = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
    let same = bootstrap_content();
    assert_eq!(publisher.publish(same.clone(), at).revision(), 1);
    assert_eq!(publisher.publish(same.clone(), at).revision(), 1);

    let mut changed = same;
    changed.application["lifecycle"] = observed(json!("suspended"), "application");
    assert_eq!(publisher.publish(changed.clone(), at).revision(), 2);
    assert_eq!(publisher.publish(changed, at).revision(), 2);
}

#[test]
fn readers_hold_one_immutable_revision_after_a_replacement() {
    let publisher = SnapshotPublisher::default();
    let old = publisher.current();
    let mut changed = bootstrap_content();
    changed.game["runtime"] = observed(json!("active"), "game_process");
    let new = publisher.publish(changed, OffsetDateTime::UNIX_EPOCH);

    assert_eq!(old.revision(), 1);
    assert_eq!(new.revision(), 2);
    assert_eq!(publisher.current().revision(), 2);
    assert!(!Arc::ptr_eq(&old, &new));
    assert_eq!(
        serde_json::to_value(old.document(1)).unwrap()["game"]["runtime"]["knowledge"],
        "unknown"
    );
    assert_eq!(
        serde_json::to_value(new.document(1)).unwrap()["game"]["runtime"]["value"],
        "active"
    );
}

#[test]
fn capabilities_claim_the_s107_http_and_mcp_state_surfaces() {
    let capabilities = Capabilities::default();
    let value = serde_json::to_value(capabilities).unwrap();
    assert_eq!(value["schema_version"], SCHEMA_VERSION);
    assert_eq!(
        value["http_operations"],
        json!(["capabilities", "player_state"])
    );
    assert_eq!(value["mcp_player_state"], true);
    assert_eq!(value["query_execution"], false);
    assert_eq!(value["database_ids"], json!(["catalog", "encounters"]));
}

#[test]
fn maintained_inventory_matches_the_s104_contract_table() {
    let contract =
        include_str!("../specs/104-local-extension-contract/contracts/player-state-v1.md");
    let mut contract_paths = contract
        .lines()
        .filter(|line| line.starts_with("| `"))
        .filter_map(|line| line.split('`').nth(1))
        .collect::<Vec<_>>();
    contract_paths.sort_unstable();
    let mut maintained = PUBLIC_PATHS.to_vec();
    maintained.sort_unstable();
    assert_eq!(maintained, contract_paths);

    let mut contract_non_public = contract
        .lines()
        .skip_while(|line| *line != "## Deliberately non-public current state")
        .filter(|line| line.starts_with("| ") && !line.starts_with("| ---"))
        .skip(1)
        .filter_map(|line| line.split('|').nth(1))
        .map(str::trim)
        .collect::<Vec<_>>();
    contract_non_public.sort_unstable();
    let mut maintained_non_public = NON_PUBLIC_STATES.to_vec();
    maintained_non_public.sort_unstable();
    assert_eq!(maintained_non_public, contract_non_public);
}

#[test]
fn active_projection_is_complete_and_loss_never_uses_hud_retention() {
    let active = active_input();
    let content = project(active.clone());
    assert_eq!(content.game["runtime"]["value"], "active");
    assert_eq!(content.player["resources"]["health"]["value"], 61);
    assert_eq!(content.player["ultimate"]["ready"]["value"], true);
    assert_eq!(content.player["bindings"].as_array().unwrap().len(), 11);
    assert_eq!(
        content.player["bindings"][0]["state"]["knowledge"],
        "unavailable"
    );
    assert_eq!(
        content.player["quickslot"]["classification"]["knowledge"],
        "unavailable"
    );
    assert_eq!(
        content.automation["weave"]["slots"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert!(!serde_json::to_string(&content)
        .unwrap()
        .contains("hud_freshness"));

    let mut unfocused = active.clone();
    unfocused.game.focus = FocusObservation::Unfocused;
    let content = project(unfocused);
    assert_eq!(content.game["focus"]["value"], "unfocused");
    assert_eq!(content.game["surface"]["freshness"], "stale");
    assert_eq!(content.player["resources"]["health"]["value"], 61);
    assert_eq!(content.player["resources"]["health"]["freshness"], "stale");

    let mut lost = active.clone();
    lost.game.freshness = BeaconFreshness::Lost;
    let content = project(lost);
    assert_eq!(content.pixel_bus["signal"]["value"], "signal_lost");
    assert_eq!(content.player["resources"]["health"]["freshness"], "stale");

    let mut inactive = active;
    inactive.game.runtime = GameRuntime::Inactive;
    let content = project(inactive);
    assert_eq!(
        content.player["resources"]["health"]["knowledge"],
        "dormant"
    );
    assert!(content.player["resources"]["health"].get("value").is_none());

    let mut indeterminate = active_input();
    indeterminate.game.runtime = GameRuntime::Unknown;
    let content = project(indeterminate);
    assert_eq!(
        content.player["resources"]["health"]["knowledge"],
        "unknown"
    );
}

fn active_input() -> ProjectionInput {
    ProjectionInput {
        suspended: false,
        catalog: observed(json!({"catalog_version": "fixture"}), "catalog"),
        data_addon: observed(
            json!({"managed_status": "managed-up-to-date"}),
            "data_addon",
        ),
        pixel_beacon: observed(
            json!({"managed_status": "managed_up_to_date", "compatible": true}),
            "pixel_beacon",
        ),
        game: GameObservations {
            installation: InstallationState::NotDetected,
            runtime: GameRuntime::Active,
            focus: FocusObservation::Focused,
            freshness: BeaconFreshness::Fresh,
            surface: SurfaceObservation::Observed(MenuSurface::None),
            world: WorldState::Active,
        },
        layout: LayoutState::Ready(BusLayout::negotiated(16).unwrap()),
        active_bar: ActiveBar::Front,
        weapon_classes: (WeaponClass::DualWield, WeaponClass::Bow),
        combat: CombatSignal::InCombat,
        movement: MovementSignal::OnFoot,
        life: LifeState::Alive,
        roll_dodge: RollDodgeState::Inactive,
        travel: TravelState::Inactive,
        resources: ResourceSet {
            health: ResourceLevel::Percent(61),
            stamina: ResourceLevel::Percent(42),
            magicka: ResourceLevel::Percent(83),
        },
        latency_ms: Some(72),
        ultimate: UltimateTelemetry {
            current: UltimateValue::Points(200),
            maximum: UltimateValue::Points(500),
            front_cost: UltimateValue::Points(125),
            back_cost: UltimateValue::Points(250),
        },
        cooldowns: CooldownSet {
            skill_1: SlotCooldown::Ready,
            skill_2: SlotCooldown::RemainingMs(250),
            skill_3: SlotCooldown::Ready,
            skill_4: SlotCooldown::Ready,
            skill_5: SlotCooldown::Ready,
            ultimate: SlotCooldown::Ready,
        },
        quickslot: QuickslotState::new_unknown(),
        bindings: NativeBindingSet::new_unavailable(),
        fishing_requested: false,
        fishing_state: FishingState::Disabled,
        fishing_stop_reason: None,
        fishing_config: FishingConfig::default(),
        auto_potion_requested: false,
        auto_potion_state: AutoPotionState::Off,
        auto_potion_config: AutoPotionConfig::default(),
        weave_config: WeaveConfig::default(),
        latency_config: LatencyConfig::default(),
        reader_config: ReaderConfig::default(),
    }
}

fn json_path<'a>(root: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
    let normalized = path.replace("[].", ".").replace("[]", "");
    let mut current = root;
    for part in normalized.split('.') {
        current = if current.is_array() {
            current.get(0)?.get(part)?
        } else {
            current.get(part)?
        };
    }
    Some(current)
}
