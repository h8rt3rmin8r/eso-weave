//! Canonical immutable player-state snapshots shared by local transports.

use std::sync::{Arc, RwLock};

use serde::Serialize;
use serde_json::{json, Value};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::fishing::{FishingConfig, FishingState, StopReason};
use crate::game::{
    BeaconFreshness, FocusObservation, GameContext, GameObservations, GameRuntime,
    InstallationProvider, InstallationState, SurfaceObservation,
};
use crate::input::{
    NativeAction, NativeBindingSet, NativeBindingState, NativeChord, NativeControl, NativeModifier,
};
use crate::pixelbus::{
    ActiveBar, CombatSignal, CooldownSet, LayoutFailure, LayoutMode, LayoutState, LifeState,
    MovementSignal, QuickslotClassification, QuickslotNonPotionKind, QuickslotPotionAvailability,
    QuickslotState, QuickslotUnavailableReason, ReaderConfig, RecoveryPath, ResourceLevel,
    ResourceSet, RollDodgeState, SlotCooldown, TravelState, UltimateTelemetry, UltimateValue,
    WeaponClass, WorldState,
};
use crate::potion::{
    AutoPotionConfig, AutoPotionResource, AutoPotionState, BlockReason, DormantReason, TriggerCause,
};
use crate::weave::{effective_delay, effective_timing, LatencyConfig, WeaveConfig};

pub const SCHEMA_VERSION: &str = "1.0.0";

pub const PUBLIC_PATHS: &[&str] = &[
    "application.lifecycle",
    "application.version",
    "application.catalog",
    "application.addons.data",
    "game.installation",
    "game.runtime",
    "game.focus",
    "game.context",
    "game.surface",
    "game.world",
    "pixel_bus.layout",
    "pixel_bus.signal",
    "pixel_bus.addon",
    "pixel_bus.fishing_signal",
    "player.weapon.active_bar",
    "player.weapon.front_class",
    "player.weapon.back_class",
    "player.combat",
    "player.movement",
    "player.life",
    "player.roll_dodge",
    "player.travel",
    "player.resources.health",
    "player.resources.stamina",
    "player.resources.magicka",
    "player.network.latency_ms",
    "player.ultimate.current",
    "player.ultimate.maximum",
    "player.ultimate.front_cost",
    "player.ultimate.back_cost",
    "player.ultimate.active_cost",
    "player.ultimate.ready",
    "player.cooldowns.skills",
    "player.cooldowns.ultimate",
    "player.quickslot.classification",
    "player.quickslot.non_potion_kind",
    "player.quickslot.potion_availability",
    "player.quickslot.cooldown",
    "player.quickslot.item_id",
    "player.quickslot.unavailable_reason",
    "player.bindings",
    "player.bindings[].state",
    "player.bindings[].chord",
    "automation.fishing.requested",
    "automation.fishing.effective_state",
    "automation.fishing.stop_reason",
    "automation.auto_potion.requested",
    "automation.auto_potion.effective_state",
    "automation.auto_potion.reason",
    "automation.weave.slots",
    "automation.weave.slots[].effective_delays_ms",
    "interpretation.auto_potion.thresholds",
    "interpretation.auto_potion.retry_interval_ms",
    "interpretation.fishing",
    "interpretation.weave.front_timing",
    "interpretation.weave.back_timing",
    "interpretation.latency",
    "interpretation.pixel_bus",
];

pub const NON_PUBLIC_STATES: &[&str] = &[
    "Rendered status lines, labels, colors, and log filters",
    "Live log entries and raw diagnostics",
    "Filesystem paths, addon staging paths, and database paths",
    "Bearer token and credential storage metadata",
    "Raw pixels, capture buffers, image handles, and window handles",
    "Input-hook handles, injected-event markers, queues, and command channels",
    "Mutable controller internals, timers, debounce counters, and retry counters",
    "PixelBus death-episode latch, prior-heartbeat latch, and configuration generation counters",
    "UI geometry, selected tab, modal state, and transient toast state",
    "Raw SQL connection objects and catalog import bookkeeping",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Capabilities {
    pub schema_version: &'static str,
    pub domains: [&'static str; 6],
    pub http_operations: [&'static str; 4],
    pub database_ids: [&'static str; 2],
    pub query_execution: bool,
    pub mcp_player_state: bool,
    pub pixel_bus: PixelBusCapabilities,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PixelBusCapabilities {
    pub supported_layout_revisions: [u8; 6],
    pub negotiated_layout: bool,
    pub reason: Option<String>,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            domains: [
                "application",
                "game",
                "pixel_bus",
                "player",
                "automation",
                "interpretation",
            ],
            http_operations: [
                "capabilities",
                "player_state",
                "databases",
                "query_database",
            ],
            database_ids: ["catalog", "encounters"],
            query_execution: true,
            mcp_player_state: true,
            pixel_bus: PixelBusCapabilities {
                supported_layout_revisions: [1, 2, 3, 4, 5, 6],
                negotiated_layout: false,
                reason: Some("layout_not_observed".into()),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SnapshotContent {
    pub capabilities: Capabilities,
    pub application: Value,
    pub game: Value,
    pub pixel_bus: Value,
    pub player: Value,
    pub automation: Value,
    pub interpretation: Value,
}

#[derive(Debug, Clone)]
pub struct ProjectionInput {
    pub suspended: bool,
    pub catalog: Value,
    pub data_addon: Value,
    pub pixel_beacon: Value,
    pub game: GameObservations,
    pub layout: LayoutState,
    pub active_bar: ActiveBar,
    pub weapon_classes: (WeaponClass, WeaponClass),
    pub combat: CombatSignal,
    pub movement: MovementSignal,
    pub life: LifeState,
    pub roll_dodge: RollDodgeState,
    pub travel: TravelState,
    pub resources: ResourceSet,
    pub latency_ms: Option<u16>,
    pub ultimate: UltimateTelemetry,
    pub cooldowns: CooldownSet,
    pub quickslot: QuickslotState,
    pub bindings: NativeBindingSet,
    pub fishing_requested: bool,
    pub fishing_state: FishingState,
    pub fishing_stop_reason: Option<StopReason>,
    pub fishing_config: FishingConfig,
    pub auto_potion_requested: bool,
    pub auto_potion_state: AutoPotionState,
    pub auto_potion_config: AutoPotionConfig,
    pub weave_config: WeaveConfig,
    pub latency_config: LatencyConfig,
    pub reader_config: ReaderConfig,
}

#[derive(Debug)]
pub struct PublishedSnapshot {
    revision: u64,
    captured_at: String,
    content: SnapshotContent,
}

impl PublishedSnapshot {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn capabilities(&self) -> Capabilities {
        self.content.capabilities.clone()
    }

    pub fn document(&self, service_generation: u64) -> Value {
        json!({
            "schema_version": SCHEMA_VERSION,
            "snapshot_revision": self.revision,
            "captured_at": self.captured_at,
            "service_generation": service_generation,
            "capabilities": self.content.capabilities,
            "application": self.content.application,
            "game": self.content.game,
            "pixel_bus": self.content.pixel_bus,
            "player": self.content.player,
            "automation": self.content.automation,
            "interpretation": self.content.interpretation,
        })
    }
}

#[derive(Clone)]
pub struct SnapshotPublisher(Arc<RwLock<Arc<PublishedSnapshot>>>);

impl Default for SnapshotPublisher {
    fn default() -> Self {
        let snapshot = PublishedSnapshot {
            revision: 1,
            captured_at: format_timestamp(OffsetDateTime::UNIX_EPOCH),
            content: bootstrap_content(),
        };
        Self(Arc::new(RwLock::new(Arc::new(snapshot))))
    }
}

impl SnapshotPublisher {
    pub fn current(&self) -> Arc<PublishedSnapshot> {
        self.0.read().unwrap().clone()
    }

    pub fn publish(
        &self,
        content: SnapshotContent,
        captured_at: OffsetDateTime,
    ) -> Arc<PublishedSnapshot> {
        let mut current = self.0.write().unwrap();
        if current.content == content {
            return current.clone();
        }
        let next = Arc::new(PublishedSnapshot {
            revision: current.revision.saturating_add(1),
            captured_at: format_timestamp(captured_at),
            content,
        });
        *current = next.clone();
        next
    }
}

pub fn observed(value: Value, source: &'static str) -> Value {
    observation("observed", Some(value), "fresh", source, None)
}

pub fn observed_with_protocol(value: Value, source: &'static str, protocol: Value) -> Value {
    observation("observed", Some(value), "fresh", source, Some(protocol))
}

pub fn unknown(source: &'static str) -> Value {
    observation("unknown", None, "not_applicable", source, None)
}

pub fn unavailable(source: &'static str) -> Value {
    observation("unavailable", None, "not_applicable", source, None)
}

pub fn dormant(source: &'static str) -> Value {
    observation("dormant", None, "not_applicable", source, None)
}

pub fn stale(value: Value, source: &'static str) -> Value {
    observation("observed", Some(value), "stale", source, None)
}

fn observation(
    knowledge: &'static str,
    value: Option<Value>,
    freshness: &'static str,
    source: &'static str,
    protocol: Option<Value>,
) -> Value {
    let mut result = json!({
        "knowledge": knowledge,
        "observed_at": null,
        "age_ms": null,
        "freshness": freshness,
        "source": source,
        "protocol": protocol,
    });
    if let Some(value) = value {
        result["value"] = value;
    }
    result
}

pub fn bootstrap_content() -> SnapshotContent {
    let unknown_game = || unknown("game_process");
    let unknown_pixel = || unknown("pixel_bus");
    let unknown_controller = || unknown("controller");
    let bindings = NativeAction::ALL
        .into_iter()
        .map(|action| {
            json!({
                "action": native_action(action),
                "state": unknown("native_binding_resolver"),
                "chord": unavailable("native_binding_resolver"),
            })
        })
        .collect::<Vec<_>>();
    let skill = unknown_pixel();
    let weave_slots = (1..=7)
        .map(|index| {
            json!({
                "index": index,
                "active": false,
                "weave_type": "light_attack",
                "override": {"d_weave": null, "d_heavy": null, "d_bash": null},
                "effective_delays_ms": {"d_weave": 0, "d_heavy": 0, "d_bash": 0},
            })
        })
        .collect::<Vec<_>>();
    SnapshotContent {
        capabilities: Capabilities::default(),
        application: json!({
            "lifecycle": observed(json!("running"), "application"),
            "version": observed(json!(env!("CARGO_PKG_VERSION")), "build_metadata"),
            "catalog": unavailable("catalog"),
            "addons": {"data": unavailable("data_addon")},
        }),
        game: json!({
            "installation": unknown_game(),
            "runtime": unknown_game(),
            "focus": unknown_game(),
            "context": unknown_game(),
            "surface": unavailable("pixel_bus"),
            "world": unknown_pixel(),
        }),
        pixel_bus: json!({
            "layout": unknown_pixel(),
            "signal": unknown_pixel(),
            "addon": unavailable("pixel_beacon"),
            "fishing_signal": unknown_pixel(),
        }),
        player: json!({
            "weapon": {
                "active_bar": unknown_pixel(),
                "front_class": unknown_pixel(),
                "back_class": unknown_pixel(),
            },
            "combat": unknown_pixel(),
            "movement": unknown_pixel(),
            "life": unknown_pixel(),
            "roll_dodge": unknown_pixel(),
            "travel": unknown_pixel(),
            "resources": {
                "health": unknown_pixel(),
                "stamina": unknown_pixel(),
                "magicka": unknown_pixel(),
            },
            "network": {"latency_ms": unknown_pixel()},
            "ultimate": {
                "current": unknown_pixel(),
                "maximum": unknown_pixel(),
                "front_cost": unknown_pixel(),
                "back_cost": unknown_pixel(),
                "active_cost": unknown("canonical_projection"),
                "ready": unknown("canonical_projection"),
            },
            "cooldowns": {
                "skills": [skill.clone(), skill.clone(), skill.clone(), skill.clone(), skill],
                "ultimate": unknown_pixel(),
            },
            "quickslot": {
                "classification": unknown_pixel(),
                "non_potion_kind": unavailable("pixel_bus"),
                "potion_availability": unavailable("pixel_bus"),
                "cooldown": unknown_pixel(),
                "item_id": unavailable("pixel_bus"),
                "unavailable_reason": unavailable("pixel_bus"),
            },
            "bindings": bindings,
        }),
        automation: json!({
            "fishing": {
                "requested": observed(json!(false), "settings"),
                "effective_state": unknown_controller(),
                "stop_reason": unavailable("controller"),
            },
            "auto_potion": {
                "requested": observed(json!(false), "settings"),
                "effective_state": observed(json!("off"), "controller"),
                "reason": unavailable("controller"),
            },
            "weave": {"slots": weave_slots},
        }),
        interpretation: json!({
            "auto_potion": {
                "thresholds": observed(json!({}), "settings"),
                "retry_interval_ms": observed(json!(0), "settings"),
            },
            "fishing": observed(json!({}), "settings"),
            "weave": {
                "front_timing": observed(json!({}), "settings"),
                "back_timing": observed(json!({}), "settings"),
            },
            "latency": observed(json!({}), "settings"),
            "pixel_bus": observed(json!({}), "settings"),
        }),
    }
}

pub fn project(input: ProjectionInput) -> SnapshotContent {
    let mut capabilities = Capabilities::default();
    match input.layout {
        LayoutState::Ready(layout) => {
            capabilities.pixel_bus.negotiated_layout =
                matches!(layout.mode, LayoutMode::Negotiated { .. });
            capabilities.pixel_bus.reason = None;
        }
        LayoutState::Unknown => {}
        LayoutState::Unavailable(reason) => {
            capabilities.pixel_bus.reason = Some(layout_failure(reason).into());
        }
    }

    let game = &input.game;
    let active_cost = match input.active_bar {
        ActiveBar::Front => ultimate_value(input.ultimate.front_cost),
        ActiveBar::Back => ultimate_value(input.ultimate.back_cost),
        ActiveBar::Unknown => None,
    };
    let ultimate_current = ultimate_value(input.ultimate.current);
    let ultimate_ready = match (ultimate_current, active_cost) {
        (Some(current), Some(cost)) => Some(current >= cost),
        _ => None,
    };
    let timing = effective_timing(
        &input.weave_config.timing,
        &input.weave_config.timing_back,
        input.weave_config.auto_timing,
        input.active_bar,
        input.weapon_classes.0,
        input.weapon_classes.1,
    );
    let weave_slots = input
        .weave_config
        .slots
        .iter()
        .map(|slot| {
            json!({
                "index": slot.index,
                "active": slot.active,
                "weave_type": slot.weave_type.as_str(),
                "override": {
                    "d_weave": slot.overrides.d_weave,
                    "d_heavy": slot.overrides.d_heavy,
                    "d_bash": slot.overrides.d_bash,
                },
                "effective_delays_ms": {
                    "d_weave": effective_delay(slot.d_weave(&timing), input.latency_ms, &input.latency_config),
                    "d_heavy": slot.d_heavy(&timing),
                    "d_bash": effective_delay(slot.d_bash(&timing), input.latency_ms, &input.latency_config),
                },
            })
        })
        .collect::<Vec<_>>();
    let cooldowns = [
        input.cooldowns.skill_1,
        input.cooldowns.skill_2,
        input.cooldowns.skill_3,
        input.cooldowns.skill_4,
        input.cooldowns.skill_5,
    ]
    .into_iter()
    .map(|value| pixel_observation(cooldown_value(value), game))
    .collect::<Vec<_>>();

    SnapshotContent {
        capabilities,
        application: json!({
            "lifecycle": observed(json!(if input.suspended { "suspended" } else { "running" }), "application"),
            "version": observed(json!(env!("CARGO_PKG_VERSION")), "build_metadata"),
            "catalog": input.catalog,
            "addons": {"data": input.data_addon},
        }),
        game: json!({
            "installation": installation_observation(&game.installation),
            "runtime": enum_observation(game_runtime(game.runtime), "game_process"),
            "focus": enum_observation(focus(game.focus), "game_process"),
            "context": enum_observation(game_context(game.context()), "game_observations"),
            "surface": surface_observation(game.surface, game),
            "world": pixel_observation(world_value(game.world), game),
        }),
        pixel_bus: json!({
            "layout": layout_observation(input.layout),
            "signal": enum_observation(signal(game.freshness), "pixel_bus"),
            "addon": input.pixel_beacon,
            // The reader currently emits fishing transitions without retaining
            // the last raw B1 sample. Do not misreport controller state as a
            // direct PixelBus observation.
            "fishing_signal": unknown("pixel_bus"),
        }),
        player: json!({
            "weapon": {
                "active_bar": pixel_observation(active_bar(input.active_bar).map(Value::from), game),
                "front_class": pixel_observation(weapon_class(input.weapon_classes.0).map(Value::from), game),
                "back_class": pixel_observation(weapon_class(input.weapon_classes.1).map(Value::from), game),
            },
            "combat": pixel_observation(combat(input.combat).map(Value::from), game),
            "movement": pixel_observation(movement(input.movement).map(Value::from), game),
            "life": pixel_observation(life(input.life), game),
            "roll_dodge": pixel_observation(roll_dodge(input.roll_dodge).map(Value::from), game),
            "travel": pixel_observation(travel(input.travel).map(Value::from), game),
            "resources": {
                "health": pixel_observation(resource(input.resources.health), game),
                "stamina": pixel_observation(resource(input.resources.stamina), game),
                "magicka": pixel_observation(resource(input.resources.magicka), game),
            },
            "network": {"latency_ms": pixel_observation(input.latency_ms.map(Value::from), game)},
            "ultimate": {
                "current": pixel_observation(ultimate_current.map(Value::from), game),
                "maximum": pixel_observation(ultimate_value(input.ultimate.maximum).map(Value::from), game),
                "front_cost": pixel_observation(ultimate_value(input.ultimate.front_cost).map(Value::from), game),
                "back_cost": pixel_observation(ultimate_value(input.ultimate.back_cost).map(Value::from), game),
                "active_cost": contextual_observation(active_cost.map(Value::from), game, "canonical_projection"),
                "ready": contextual_observation(ultimate_ready.map(Value::from), game, "canonical_projection"),
            },
            "cooldowns": {
                "skills": cooldowns,
                "ultimate": pixel_observation(cooldown_value(input.cooldowns.ultimate), game),
            },
            "quickslot": quickslot(input.quickslot, game),
            "bindings": bindings(input.bindings, game),
        }),
        automation: json!({
            "fishing": {
                "requested": observed(json!(input.fishing_requested), "settings"),
                "effective_state": observed(json!(fishing_state(input.fishing_state)), "controller"),
                "stop_reason": optional_observation(input.fishing_stop_reason.map(stop_reason).map(Value::from), "controller"),
            },
            "auto_potion": {
                "requested": observed(json!(input.auto_potion_requested), "settings"),
                "effective_state": observed(json!(auto_potion_state(input.auto_potion_state)), "controller"),
                "reason": optional_observation(auto_potion_reason(input.auto_potion_state), "controller"),
            },
            "weave": {"slots": weave_slots},
        }),
        interpretation: json!({
            "auto_potion": {
                "thresholds": observed(json!({
                    "health": watch(input.auto_potion_config.health),
                    "stamina": watch(input.auto_potion_config.stamina),
                    "magicka": watch(input.auto_potion_config.magicka),
                    "ultimate": watch(input.auto_potion_config.ultimate),
                }), "settings"),
                "retry_interval_ms": observed(json!(input.auto_potion_config.retry_interval_ms), "settings"),
            },
            "fishing": observed(json!({
                "arm_timeout_ms": input.fishing_config.arm_timeout_ms,
                "reel_delay_ms": input.fishing_config.reel_delay_ms,
                "recast_delay_ms": input.fishing_config.recast_delay_ms,
            }), "settings"),
            "weave": {
                "front_timing": observed(timing_value(input.weave_config.timing, input.weave_config.auto_timing), "settings"),
                "back_timing": observed(timing_value(input.weave_config.timing_back, input.weave_config.auto_timing), "settings"),
            },
            "latency": observed(json!({
                "enabled": input.latency_config.enabled,
                "scale": input.latency_config.k,
                "current_ms": input.latency_ms,
            }), "settings"),
            "pixel_bus": observed(json!({
                "block_px": input.reader_config.block_px,
                "color_tolerance": input.reader_config.tolerance,
                "heartbeat_timeout_ms": input.reader_config.heartbeat_timeout_ms,
                "fishing_interval_ms": input.reader_config.interval_fishing_ms,
                "idle_interval_ms": input.reader_config.interval_idle_ms,
            }), "settings"),
        }),
    }
}

fn optional_observation(value: Option<Value>, source: &'static str) -> Value {
    value.map_or_else(|| unavailable(source), |value| observed(value, source))
}

fn enum_observation(value: Option<&'static str>, source: &'static str) -> Value {
    value.map_or_else(|| unknown(source), |value| observed(json!(value), source))
}

fn pixel_observation(value: Option<Value>, game: &GameObservations) -> Value {
    contextual_observation(value, game, "pixel_bus")
}

fn contextual_observation(
    value: Option<Value>,
    game: &GameObservations,
    source: &'static str,
) -> Value {
    match game.runtime {
        GameRuntime::Unknown => return unknown(source),
        GameRuntime::Active => {}
        GameRuntime::Inactive | GameRuntime::LauncherOpen => return dormant(source),
    }
    let Some(value) = value else {
        return unknown(source);
    };
    if game.focus == FocusObservation::Focused && game.freshness == BeaconFreshness::Fresh {
        observed(value, source)
    } else {
        stale(value, source)
    }
}

fn pixel_optional_observation(value: Option<Value>, game: &GameObservations) -> Value {
    value.map_or_else(
        || unavailable("pixel_bus"),
        |value| pixel_observation(Some(value), game),
    )
}

fn contextual_unavailable(game: &GameObservations, source: &'static str) -> Value {
    match game.runtime {
        GameRuntime::Unknown => unknown(source),
        GameRuntime::Active => unavailable(source),
        GameRuntime::Inactive | GameRuntime::LauncherOpen => dormant(source),
    }
}

fn installation_observation(value: &InstallationState) -> Value {
    match value {
        InstallationState::NotDetected => {
            observed(json!({"availability": "not_detected"}), "game_installation")
        }
        InstallationState::Detected(candidate) => observed(
            json!({
                "availability": "detected",
                "provider": installation_provider(candidate.provider),
                "classification": "validated_root",
            }),
            "game_installation",
        ),
        InstallationState::Ambiguous => unavailable("game_installation"),
        InstallationState::Unknown => unknown("game_installation"),
    }
}

fn installation_provider(value: InstallationProvider) -> &'static str {
    match value {
        InstallationProvider::EsoStore => "eso_store",
        InstallationProvider::Steam => "steam",
        InstallationProvider::Epic => "epic",
        InstallationProvider::SteamProton => "steam_proton",
    }
}

fn game_runtime(value: GameRuntime) -> Option<&'static str> {
    match value {
        GameRuntime::Inactive => Some("inactive"),
        GameRuntime::LauncherOpen => Some("launcher_open"),
        GameRuntime::Active => Some("active"),
        GameRuntime::Unknown => None,
    }
}

fn focus(value: FocusObservation) -> Option<&'static str> {
    match value {
        FocusObservation::Focused => Some("focused"),
        FocusObservation::Unfocused => Some("unfocused"),
        FocusObservation::Unknown => None,
    }
}

fn game_context(value: GameContext) -> Option<&'static str> {
    match value {
        GameContext::NotDetected => Some("dormant"),
        GameContext::Unfocused => Some("unfocused"),
        GameContext::Gameplay => Some("active"),
        GameContext::Surface(_) => Some("surface"),
        GameContext::SignalUnavailable => Some("unavailable"),
        GameContext::Unknown => None,
    }
}

fn surface_observation(value: SurfaceObservation, game: &GameObservations) -> Value {
    match value {
        SurfaceObservation::Unavailable => contextual_unavailable(game, "pixel_bus"),
        SurfaceObservation::Observed(value) => {
            pixel_observation(Some(json!(menu_surface(value))), game)
        }
    }
}

fn menu_surface(value: crate::pixelbus::MenuSurface) -> &'static str {
    use crate::pixelbus::MenuSurface;
    match value {
        MenuSurface::None => "none",
        MenuSurface::SystemMenu => "system_menu",
        MenuSurface::Map => "map",
        MenuSurface::Inventory => "inventory",
        MenuSurface::Mail => "mail",
        MenuSurface::Character => "character",
        MenuSurface::GuildStore => "guild_store",
        MenuSurface::CrownStore => "crown_store",
        MenuSurface::Journal => "journal",
        MenuSurface::ChatEntry => "chat_entry",
        MenuSurface::Other => "other",
    }
}

fn signal(value: BeaconFreshness) -> Option<&'static str> {
    match value {
        BeaconFreshness::NeverObserved => None,
        BeaconFreshness::Fresh => Some("heartbeat"),
        BeaconFreshness::Lost => Some("signal_lost"),
    }
}

fn layout_observation(value: LayoutState) -> Value {
    match value {
        LayoutState::Unknown => unknown("pixel_bus"),
        LayoutState::Unavailable(reason) => {
            unavailable_with_reason("pixel_bus", layout_failure(reason))
        }
        LayoutState::Ready(layout) => observed_with_protocol(
            json!({
                "mode": match layout.mode { LayoutMode::Legacy => "legacy", LayoutMode::Negotiated { .. } => "negotiated" },
                "columns": layout.columns,
                "payload_offset": layout.payload_offset,
                "geometry_valid": true,
                "compatible": true,
            }),
            "pixel_bus",
            json!({
                "name": "pixel_bus",
                "revision": match layout.mode { LayoutMode::Legacy => 1, LayoutMode::Negotiated { version } => version },
                "layout_revision": match layout.mode { LayoutMode::Legacy => 1, LayoutMode::Negotiated { version } => version },
                "capability_reason": null,
            }),
        ),
    }
}

fn unavailable_with_reason(source: &'static str, reason: &'static str) -> Value {
    observation(
        "unavailable",
        None,
        "not_applicable",
        source,
        Some(json!({"capability_reason": reason})),
    )
}

fn layout_failure(value: LayoutFailure) -> &'static str {
    match value {
        LayoutFailure::Missing => "missing",
        LayoutFailure::InvalidBlockSize => "invalid_block_size",
        LayoutFailure::CorruptMagic => "corrupt_magic",
        LayoutFailure::UnsupportedVersion { .. } => "unsupported_version",
        LayoutFailure::CorruptHighByte => "corrupt_high_byte",
        LayoutFailure::CorruptLowByte => "corrupt_low_byte",
        LayoutFailure::ColumnsOutOfRange { .. } => "columns_out_of_range",
        LayoutFailure::ExceedsSurface { .. } => "exceeds_surface",
        LayoutFailure::ExtentExceedsSurface { .. } => "extent_exceeds_surface",
    }
}

fn active_bar(value: ActiveBar) -> Option<&'static str> {
    match value {
        ActiveBar::Front => Some("front"),
        ActiveBar::Back => Some("back"),
        ActiveBar::Unknown => None,
    }
}

fn weapon_class(value: WeaponClass) -> Option<&'static str> {
    match value {
        WeaponClass::DualWield => Some("dual_wield"),
        WeaponClass::TwoHanded => Some("two_handed"),
        WeaponClass::SwordAndShield => Some("sword_and_shield"),
        WeaponClass::Bow => Some("bow"),
        WeaponClass::DestructionStaff => Some("destruction_staff"),
        WeaponClass::RestorationStaff => Some("restoration_staff"),
        WeaponClass::Unknown => None,
    }
}

fn combat(value: CombatSignal) -> Option<&'static str> {
    match value {
        CombatSignal::InCombat => Some("in_combat"),
        CombatSignal::OutOfCombat => Some("out_of_combat"),
        CombatSignal::Unknown => None,
    }
}

fn movement(value: MovementSignal) -> Option<&'static str> {
    match value {
        MovementSignal::OnFoot => Some("on_foot"),
        MovementSignal::Mounted => Some("mounted"),
        MovementSignal::Sprinting => Some("sprinting"),
        MovementSignal::Unknown => None,
    }
}

fn life(value: LifeState) -> Option<Value> {
    match value {
        LifeState::Alive => Some(json!({"state": "alive"})),
        LifeState::Dead => Some(json!({"state": "dead"})),
        LifeState::Recovering(path) => {
            Some(json!({"state": "recovering", "recovery_path": recovery_path(path)}))
        }
        LifeState::Unknown => None,
    }
}

fn recovery_path(value: RecoveryPath) -> &'static str {
    match value {
        RecoveryPath::Ghost => "ghost",
        RecoveryPath::WorldActivation => "world_activation",
        RecoveryPath::NoLoad => "no_load",
    }
}

fn world_value(value: WorldState) -> Option<Value> {
    match value {
        WorldState::Active => Some(json!("active")),
        WorldState::Transitioning => Some(json!("transitioning")),
        WorldState::Unknown => None,
    }
}

fn roll_dodge(value: RollDodgeState) -> Option<&'static str> {
    match value {
        RollDodgeState::Active => Some("active"),
        RollDodgeState::Inactive => Some("inactive"),
        RollDodgeState::Unknown => None,
    }
}

fn travel(value: TravelState) -> Option<&'static str> {
    match value {
        TravelState::Pending => Some("pending"),
        TravelState::Inactive => Some("inactive"),
        TravelState::Unknown => None,
    }
}

fn resource(value: ResourceLevel) -> Option<Value> {
    match value {
        ResourceLevel::Percent(value) => Some(json!(value)),
        ResourceLevel::Unknown => None,
    }
}

fn ultimate_value(value: UltimateValue) -> Option<u16> {
    match value {
        UltimateValue::Points(value) => Some(value),
        UltimateValue::Unknown => None,
    }
}

fn cooldown_value(value: SlotCooldown) -> Option<Value> {
    match value {
        SlotCooldown::Ready => Some(json!({"state": "ready"})),
        SlotCooldown::RemainingMs(value) => {
            Some(json!({"state": "remaining", "remaining_ms": value}))
        }
        SlotCooldown::Unknown => None,
    }
}

fn quickslot(value: QuickslotState, game: &GameObservations) -> Value {
    let (classification, unavailable_classification, kind, availability, reason) =
        match value.classification {
            QuickslotClassification::Unavailable(reason) => {
                (None, true, None, None, Some(quickslot_reason(reason)))
            }
            QuickslotClassification::Empty => (Some("empty"), false, None, None, None),
            QuickslotClassification::NonPotion(kind) => (
                Some("non_potion"),
                false,
                Some(non_potion_kind(kind)),
                None,
                None,
            ),
            QuickslotClassification::Potion(availability) => (
                Some("potion"),
                false,
                None,
                Some(potion_availability(availability)),
                None,
            ),
        };
    let classification = if unavailable_classification {
        contextual_unavailable(game, "pixel_bus")
    } else {
        pixel_observation(classification.map(Value::from), game)
    };
    json!({
        "classification": classification,
        "non_potion_kind": pixel_optional_observation(kind.map(Value::from), game),
        "potion_availability": pixel_optional_observation(availability.map(Value::from), game),
        "cooldown": pixel_observation(cooldown_value(value.cooldown), game),
        "item_id": pixel_optional_observation(value.item_id.map(Value::from), game),
        "unavailable_reason": pixel_optional_observation(reason.map(Value::from), game),
    })
}

fn quickslot_reason(value: QuickslotUnavailableReason) -> &'static str {
    match value {
        QuickslotUnavailableReason::NoSignal => "no_signal",
        QuickslotUnavailableReason::LegacyProtocol => "legacy_protocol",
        QuickslotUnavailableReason::CorruptProtocol => "corrupt_protocol",
        QuickslotUnavailableReason::UnsupportedApi => "unsupported_api",
        QuickslotUnavailableReason::InvalidSelection => "invalid_selection",
        QuickslotUnavailableReason::InconsistentFacts => "inconsistent_facts",
    }
}

fn non_potion_kind(value: QuickslotNonPotionKind) -> &'static str {
    match value {
        QuickslotNonPotionKind::Item => "item",
        QuickslotNonPotionKind::Collectible => "collectible",
        QuickslotNonPotionKind::QuestItem => "quest_item",
        QuickslotNonPotionKind::Emote => "emote",
        QuickslotNonPotionKind::QuickChat => "quick_chat",
        QuickslotNonPotionKind::Other => "other",
    }
}

fn potion_availability(value: QuickslotPotionAvailability) -> &'static str {
    match value {
        QuickslotPotionAvailability::Usable => "usable",
        QuickslotPotionAvailability::Depleted => "depleted",
        QuickslotPotionAvailability::Blocked => "blocked",
    }
}

fn bindings(value: NativeBindingSet, game: &GameObservations) -> Vec<Value> {
    NativeAction::ALL
        .into_iter()
        .map(|action| {
            let (state, unavailable_state, chord) = match value.get(action) {
                NativeBindingState::Unavailable => (None, true, None),
                NativeBindingState::Unbound => (Some("unbound"), false, None),
                NativeBindingState::Conflicting => (Some("conflicting"), false, None),
                NativeBindingState::Unsupported => (Some("unsupported"), false, None),
                NativeBindingState::Valid(chord) => {
                    (Some("valid"), false, Some(chord_value(chord)))
                }
            };
            let state = if unavailable_state {
                contextual_unavailable(game, "native_binding_resolver")
            } else {
                contextual_observation(state.map(Value::from), game, "native_binding_resolver")
            };
            json!({
                "action": native_action(action),
                "state": state,
                "chord": chord.map_or_else(
                    || unavailable("native_binding_resolver"),
                    |value| contextual_observation(Some(value), game, "native_binding_resolver"),
                ),
            })
        })
        .collect()
}

fn native_action(value: NativeAction) -> &'static str {
    match value {
        NativeAction::Skill1 => "skill_1",
        NativeAction::Skill2 => "skill_2",
        NativeAction::Skill3 => "skill_3",
        NativeAction::Skill4 => "skill_4",
        NativeAction::Skill5 => "skill_5",
        NativeAction::Ultimate => "ultimate",
        NativeAction::Synergy => "synergy",
        NativeAction::Attack => "attack",
        NativeAction::Block => "block",
        NativeAction::Interact => "interact",
        NativeAction::Quickslot => "quickslot",
    }
}

fn chord_value(value: NativeChord) -> Value {
    let modifiers = NativeModifier::ORDERED
        .into_iter()
        .filter(|modifier| value.modifiers.contains(modifier.flag()))
        .map(|modifier| match modifier {
            NativeModifier::Control => "control",
            NativeModifier::Alt => "alt",
            NativeModifier::Shift => "shift",
            NativeModifier::Command => "command",
        })
        .collect::<Vec<_>>();
    let (kind, code) = match value.primary {
        NativeControl::Keyboard(key) => ("keyboard", key as u8),
        NativeControl::Mouse(button) => ("mouse", button as u8),
    };
    json!({"control": {"kind": kind, "code": code}, "modifiers": modifiers})
}

fn fishing_state(value: FishingState) -> &'static str {
    match value {
        FishingState::Disabled => "disabled",
        FishingState::Armed => "armed",
        FishingState::Waiting => "waiting",
        FishingState::Reeling => "reeling",
        FishingState::Recast => "recast",
    }
}

fn stop_reason(value: StopReason) -> &'static str {
    match value {
        StopReason::UserStop => "user_stop",
        StopReason::NoCastDetected => "no_cast_detected",
        StopReason::SignalLost => "signal_lost",
        StopReason::GameInactive => "game_inactive",
        StopReason::Unfocused => "unfocused",
        StopReason::Suspended => "suspended",
        StopReason::PlayerUnavailable => "player_unavailable",
        StopReason::WorldUnavailable => "world_unavailable",
        StopReason::TravelPending => "travel_pending",
        StopReason::SettingsChanged => "settings_changed",
        StopReason::InteractUnavailable => "interact_unavailable",
    }
}

fn auto_potion_state(value: AutoPotionState) -> &'static str {
    match value {
        AutoPotionState::Off => "off",
        AutoPotionState::Dormant(_) => "dormant",
        AutoPotionState::Blocked(_) => "blocked",
        AutoPotionState::Ready => "ready",
        AutoPotionState::Triggered(_) => "triggered",
    }
}

fn auto_potion_reason(value: AutoPotionState) -> Option<Value> {
    match value {
        AutoPotionState::Off | AutoPotionState::Ready => None,
        AutoPotionState::Dormant(reason) => Some(
            json!({"type": match reason { DormantReason::GameInactive => "game_inactive", DormantReason::Unfocused => "unfocused" }}),
        ),
        AutoPotionState::Blocked(reason) => Some(json!({"type": block_reason(reason)})),
        AutoPotionState::Triggered(TriggerCause {
            resource,
            observed_percent,
            threshold_percent,
        }) => Some(
            json!({"type": "trigger", "resource": potion_resource(resource), "observed_percent": observed_percent, "threshold_percent": threshold_percent}),
        ),
    }
}

fn block_reason(value: BlockReason) -> &'static str {
    match value {
        BlockReason::QuickslotBindingUnavailable => "quickslot_binding_unavailable",
        BlockReason::BeaconUnavailable => "beacon_unavailable",
        BlockReason::Suspended => "suspended",
        BlockReason::GameContext => "game_context",
        BlockReason::PlayerUnavailable(_) => "player_unavailable",
        BlockReason::WorldUnavailable => "world_unavailable",
        BlockReason::TravelPending => "travel_pending",
        BlockReason::Sprinting => "sprinting",
        BlockReason::NoWatchedResource => "no_watched_resource",
        BlockReason::ResourcesUnavailable => "resources_unavailable",
        BlockReason::QuickslotUnavailable => "quickslot_unavailable",
        BlockReason::NoPotion => "no_potion",
        BlockReason::PotionUnavailable => "potion_unavailable",
        BlockReason::PotionCooldown => "potion_cooldown",
        BlockReason::RetryInterval => "retry_interval",
    }
}

fn potion_resource(value: AutoPotionResource) -> &'static str {
    match value {
        AutoPotionResource::Health => "health",
        AutoPotionResource::Magicka => "magicka",
        AutoPotionResource::Stamina => "stamina",
        AutoPotionResource::Ultimate => "ultimate",
    }
}

fn watch(value: crate::potion::ResourceWatch) -> Value {
    json!({"enabled": value.enabled, "threshold": value.threshold})
}

fn timing_value(value: crate::weave::TimingConfig, auto_timing: bool) -> Value {
    json!({"global_cooldown_ms": value.global_cooldown, "d_weave_ms": value.d_weave, "d_heavy_ms": value.d_heavy, "d_bash_ms": value.d_bash, "auto_timing": auto_timing})
}

fn format_timestamp(value: OffsetDateTime) -> String {
    value
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
}
