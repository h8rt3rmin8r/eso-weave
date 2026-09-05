//! Application view-model: the testable core of the GUI.
//!
//! Everything correctness-bearing (display derivation, UI-intent handling, the
//! settings mapping in [`settings_form`], the reader-event routing in
//! [`routing`], and the log view in [`log_view`]) lives here and is unit-tested
//! against the project's in-memory subsystems. The egui rendering in [`ui`] reads
//! this view-model and raises intents; its sizing behavior is covered by
//! `tests/app_ui_sizing.rs` through a headless egui harness, which supersedes the
//! former claim that a window could not be exercised headlessly (slice 030).

pub mod beacon_light;
pub mod log_view;
pub mod routing;
pub mod settings_form;
pub mod strings;
pub mod theme;
pub mod ui;
pub mod widgets;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::beacon::api_check::ApiCheckOutcome;
use crate::beacon::{self, BeaconPrefs, BeaconStatus};
use crate::config::state::{ApiVersionCache, SessionState, WindowGeometry, CURRENT_STATE_VERSION};
use crate::config::{self, LevelName, Notice, Settings};
use crate::fishing::{FishingController, FishingSink, FishingState, StopReason};
use crate::game::{
    BeaconFreshness, GameContext, GameRuntime, GameState, InstallationProvider, InstallationState,
};
use crate::input::InputEngine;
use crate::logging::LogHandle;
use crate::pixelbus::{
    ActiveBar, CombatSignal, CooldownSet, LifeState, MenuSurface, MovementSignal,
    QuickslotClassification, QuickslotNonPotionKind, QuickslotPotionAvailability, QuickslotState,
    QuickslotUnavailableReason, ResourceLevel, ResourceSet, SlotCooldown, WeaponClass,
};
use crate::potion::{
    AutoPotionConfig, AutoPotionController, AutoPotionResource, AutoPotionState, BlockReason,
    DormantReason, ResourceWatch,
};
use crate::weave::{WeaveConfig, WeaveEngine, WeaveType};

pub use beacon_light::{beacon_light, uninstall_enabled, BeaconCondition, BeaconLight};
pub use log_view::{build_log_view, level_color, LogColor, LogRow};
pub use routing::{app_toggle_intent, route_game_observation, route_reader_event};
pub use settings_form::{SettingsForm, UiPrefs};

/// The application-state indicator and its toggle button label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppStateLabel {
    /// The state indicator text.
    pub indicator: &'static str,
    /// The toggle button label (the action it performs).
    pub button: &'static str,
}

/// Derives the app-state label from the suspend state.
pub fn app_state_label(suspended: bool) -> AppStateLabel {
    if suspended {
        AppStateLabel {
            indicator: "Suspended",
            button: "Resume",
        }
    } else {
        AppStateLabel {
            indicator: "Running",
            button: "Suspend",
        }
    }
}

/// The fishing-state indicator and its toggle button label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FishingLabel {
    /// The fishing state indicator text.
    pub indicator: String,
    /// The toggle button label (the action it performs).
    pub button: &'static str,
}

/// The plain-language fishing indicator for a state and, when idle, the reason it
/// last stopped. An internal state name is never shown.
pub fn fishing_indicator(state: FishingState, reason: Option<StopReason>) -> &'static str {
    match state {
        FishingState::Armed => strings::FISHING_CASTING,
        FishingState::Waiting => strings::FISHING_WAITING,
        FishingState::Reeling => strings::FISHING_REELING,
        FishingState::Recast => strings::FISHING_RECASTING,
        FishingState::Disabled => match reason {
            Some(StopReason::NoCastDetected) => strings::FISHING_IDLE_NO_CAST,
            Some(StopReason::SignalLost) => strings::FISHING_IDLE_SIGNAL_LOST,
            Some(StopReason::GameInactive) => strings::FISHING_IDLE_GAME_INACTIVE,
            Some(StopReason::Unfocused) => strings::FISHING_IDLE_UNFOCUSED,
            Some(StopReason::PlayerUnavailable) => strings::FISHING_IDLE_PLAYER_UNAVAILABLE,
            None | Some(StopReason::UserStop) => strings::FISHING_IDLE,
        },
    }
}

/// Derives the fishing label from the controller state and stop reason.
pub fn fishing_label(state: FishingState, reason: Option<StopReason>) -> FishingLabel {
    FishingLabel {
        indicator: fishing_indicator(state, reason).to_string(),
        button: if state == FishingState::Disabled {
            "Go Fish"
        } else {
            "Stop Fishing"
        },
    }
}

/// A brand status role, mapping a state to a palette color from one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusRole {
    /// Running and healthy (ok color).
    Healthy,
    /// A warning condition (warn color).
    Warning,
    /// An active operation in progress (accent color).
    Active,
    /// Idle or absent (muted color).
    Muted,
    /// An error or lost signal (error color).
    Error,
}

/// The responsive arrangement used by the pre-Skills dashboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardLayout {
    /// Live HUD followed by System and State in one vertical flow.
    Narrow,
    /// Live HUD and System and State in side-by-side columns.
    Wide,
}

/// Minimum available width in egui points for the two-column dashboard.
pub const DASHBOARD_WIDE_MIN: f32 = 880.0;

/// Selects the dashboard arrangement from logical point width.
pub fn dashboard_layout(available_width: f32) -> DashboardLayout {
    if available_width.is_finite() && available_width >= DASHBOARD_WIDE_MIN {
        DashboardLayout::Wide
    } else {
        DashboardLayout::Narrow
    }
}

/// A normalized status line for the top region: a title, a colorized state
/// field, and a tooltip. Derived each frame from the subsystem state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusLine {
    /// The section title, shown first on the line.
    pub title: &'static str,
    /// The normalized state text.
    pub state_text: String,
    /// The palette role that colors the state field.
    pub role: StatusRole,
    /// The hover tooltip for the line.
    pub tooltip: &'static str,
}

/// Derives the Status line from the suspend state.
pub fn status_line_app(suspended: bool) -> StatusLine {
    if suspended {
        StatusLine {
            title: strings::STATUS_TITLE,
            state_text: "Suspended".to_string(),
            role: StatusRole::Warning,
            tooltip: strings::STATUS_TOOLTIP,
        }
    } else {
        StatusLine {
            title: strings::STATUS_TITLE,
            state_text: "Active".to_string(),
            role: StatusRole::Healthy,
            tooltip: strings::STATUS_TOOLTIP,
        }
    }
}

/// Derives the Fishing line from the controller state and stop reason. An idle
/// state that stopped on a fault (no cast detected or signal lost) is colored as
/// a warning so it draws the eye; a clean idle stays muted.
pub fn status_line_fishing(state: FishingState, reason: Option<StopReason>) -> StatusLine {
    let role = match state {
        FishingState::Disabled => match reason {
            Some(StopReason::NoCastDetected)
            | Some(StopReason::SignalLost)
            | Some(StopReason::GameInactive)
            | Some(StopReason::Unfocused)
            | Some(StopReason::PlayerUnavailable) => StatusRole::Warning,
            None | Some(StopReason::UserStop) => StatusRole::Muted,
        },
        _ => StatusRole::Active,
    };
    StatusLine {
        title: strings::FISHING_TITLE,
        state_text: fishing_indicator(state, reason).to_string(),
        role,
        tooltip: strings::FISHING_TOOLTIP,
    }
}

/// Derives the Pixel Beacon line from the beacon condition.
pub fn status_line_beacon(condition: BeaconCondition) -> StatusLine {
    let (state_text, role) = match condition {
        BeaconCondition::InstalledCurrent => {
            ("Installed (current)".to_string(), StatusRole::Healthy)
        }
        BeaconCondition::InstalledOutdated => {
            ("Installed (outdated)".to_string(), StatusRole::Warning)
        }
        BeaconCondition::NotInstalled => ("Not installed".to_string(), StatusRole::Muted),
        BeaconCondition::AddonsNotFound => {
            ("AddOns folder not found".to_string(), StatusRole::Error)
        }
    };
    StatusLine {
        title: strings::BEACON_TITLE,
        state_text,
        role,
        tooltip: strings::BEACON_TOOLTIP,
    }
}

/// The one addon lifecycle action that deserves primary emphasis right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeaconPrimaryAction {
    /// Install the managed addon.
    Install,
    /// Replace an outdated managed addon.
    Update,
}

/// Chooses the next useful addon action without promoting destructive removal.
pub fn beacon_primary_action(condition: BeaconCondition) -> Option<BeaconPrimaryAction> {
    match condition {
        BeaconCondition::NotInstalled | BeaconCondition::AddonsNotFound => {
            Some(BeaconPrimaryAction::Install)
        }
        BeaconCondition::InstalledOutdated => Some(BeaconPrimaryAction::Update),
        BeaconCondition::InstalledCurrent => None,
    }
}

/// The normalized effective auto-potion state shown beside its request switch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoPotionView {
    /// The concise state or current blocker.
    pub text: String,
    /// The palette role that colors the state.
    pub role: StatusRole,
}

/// Maps the controller-owned effective state to user-facing text without
/// reproducing any trigger logic in the UI.
pub fn auto_potion_view(state: AutoPotionState) -> AutoPotionView {
    let (text, role) = match state {
        AutoPotionState::Off => (strings::AUTO_POTION_OFF.to_string(), StatusRole::Muted),
        AutoPotionState::Dormant(DormantReason::GameInactive) => (
            strings::AUTO_POTION_DORMANT_GAME.to_string(),
            StatusRole::Muted,
        ),
        AutoPotionState::Dormant(DormantReason::Unfocused) => (
            strings::AUTO_POTION_DORMANT_UNFOCUSED.to_string(),
            StatusRole::Muted,
        ),
        AutoPotionState::Blocked(reason) => {
            let text = match reason {
                BlockReason::BeaconUnavailable => strings::AUTO_POTION_BLOCKED_BEACON,
                BlockReason::Suspended => strings::AUTO_POTION_BLOCKED_SUSPENDED,
                BlockReason::GameContext => strings::AUTO_POTION_BLOCKED_CONTEXT,
                BlockReason::PlayerUnavailable(LifeState::Unknown | LifeState::Alive) => {
                    strings::AUTO_POTION_BLOCKED_PLAYER_UNKNOWN
                }
                BlockReason::PlayerUnavailable(LifeState::Dead) => {
                    strings::AUTO_POTION_BLOCKED_PLAYER_DEAD
                }
                BlockReason::PlayerUnavailable(LifeState::Reincarnating) => {
                    strings::AUTO_POTION_BLOCKED_PLAYER_REINCARNATING
                }
                BlockReason::NoWatchedResource => strings::AUTO_POTION_BLOCKED_NO_WATCH,
                BlockReason::ResourcesUnavailable => strings::AUTO_POTION_BLOCKED_RESOURCES,
                BlockReason::QuickslotUnavailable => strings::AUTO_POTION_BLOCKED_QUICKSLOT,
                BlockReason::NoPotion => strings::AUTO_POTION_BLOCKED_NO_POTION,
                BlockReason::PotionUnavailable => strings::AUTO_POTION_BLOCKED_POTION,
                BlockReason::PotionCooldown => strings::AUTO_POTION_BLOCKED_COOLDOWN,
                BlockReason::RetryInterval => strings::AUTO_POTION_BLOCKED_RETRY,
            };
            (text.to_string(), StatusRole::Warning)
        }
        AutoPotionState::Ready => (strings::AUTO_POTION_READY.to_string(), StatusRole::Healthy),
        AutoPotionState::Triggered(cause) => {
            let resource = match cause.resource {
                AutoPotionResource::Health => "Health",
                AutoPotionResource::Magicka => "Magicka",
                AutoPotionResource::Stamina => "Stamina",
            };
            (
                format!(
                    "Triggered: {resource} at {}% (threshold {}%)",
                    cause.observed_percent, cause.threshold_percent
                ),
                StatusRole::Active,
            )
        }
    };
    AutoPotionView { text, role }
}

/// The display name for a weapon class.
pub fn weapon_class_name(class: WeaponClass) -> &'static str {
    match class {
        WeaponClass::Unknown => "Unknown",
        WeaponClass::DualWield => "Dual Wield",
        WeaponClass::TwoHanded => "Two Handed",
        WeaponClass::SwordAndShield => "Sword and Shield",
        WeaponClass::Bow => "Bow",
        WeaponClass::DestructionStaff => "Destruction Staff",
        WeaponClass::RestorationStaff => "Restoration Staff",
    }
}

/// The display name for an active bar.
pub fn active_bar_name(bar: ActiveBar) -> &'static str {
    match bar {
        ActiveBar::Unknown => "Unknown",
        ActiveBar::Front => "Front",
        ActiveBar::Back => "Back",
    }
}

/// A normalized view of the detected weapon-bar state for the status region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaponBarView {
    /// Whether any weapon-bar signal has been detected.
    pub detected: bool,
    /// The active bar display name.
    pub active_bar: &'static str,
    /// The front bar weapon class display name.
    pub front: &'static str,
    /// The back bar weapon class display name.
    pub back: &'static str,
    /// The palette role for the state field.
    pub role: StatusRole,
}

/// Derives the weapon-bar view from the decoded bar and classes.
pub fn weapon_bar_view(bar: ActiveBar, front: WeaponClass, back: WeaponClass) -> WeaponBarView {
    let detected =
        bar != ActiveBar::Unknown || front != WeaponClass::Unknown || back != WeaponClass::Unknown;
    WeaponBarView {
        detected,
        active_bar: if detected {
            active_bar_name(bar)
        } else {
            "Not detected"
        },
        front: weapon_class_name(front),
        back: weapon_class_name(back),
        role: if detected {
            StatusRole::Active
        } else {
            StatusRole::Muted
        },
    }
}

/// A normalized view of the detected combat state for the status region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatView {
    /// Whether any combat signal has been decoded.
    pub detected: bool,
    /// The combat state display name.
    pub state: &'static str,
    /// The palette role for the state field.
    pub role: StatusRole,
}

/// Derives the combat view from the decoded signal.
///
/// The unavailable case renders exactly as the weapon-bar readout renders its
/// own, so the operator reads two adjacent fields with one convention.
pub fn combat_view(signal: CombatSignal) -> CombatView {
    let detected = signal != CombatSignal::Unknown;
    CombatView {
        detected,
        state: match signal {
            CombatSignal::InCombat => "In combat",
            CombatSignal::OutOfCombat => "Out of combat",
            CombatSignal::Unknown => "Not detected",
        },
        role: if detected {
            StatusRole::Active
        } else {
            StatusRole::Muted
        },
    }
}

/// A normalized view of the detected movement state for the status region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovementView {
    /// Whether any movement signal has been decoded.
    pub detected: bool,
    /// The movement state display name.
    pub state: &'static str,
    /// The palette role for the state field.
    pub role: StatusRole,
}

/// Derives the movement view from the decoded signal.
///
/// The wording and the role mapping follow [`combat_view`] rather than inventing
/// a second convention, so the operator reads the three adjacent player-state
/// fields the same way.
pub fn movement_view(signal: MovementSignal) -> MovementView {
    let detected = signal != MovementSignal::Unknown;
    MovementView {
        detected,
        state: match signal {
            MovementSignal::Mounted => "Mounted",
            MovementSignal::OnFoot => "On foot",
            MovementSignal::Unknown => "Not detected",
        },
        role: if detected {
            StatusRole::Active
        } else {
            StatusRole::Muted
        },
    }
}

/// A normalized player life-state view for Live HUD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifeStateView {
    pub state: &'static str,
    pub role: StatusRole,
}

pub fn life_state_view(state: LifeState) -> LifeStateView {
    LifeStateView {
        state: match state {
            LifeState::Unknown => "Not detected",
            LifeState::Alive => "Alive",
            LifeState::Dead => "Dead",
            LifeState::Reincarnating => "Reincarnating",
        },
        role: match state {
            LifeState::Alive => StatusRole::Healthy,
            LifeState::Dead | LifeState::Reincarnating => StatusRole::Warning,
            LifeState::Unknown => StatusRole::Muted,
        },
    }
}

/// A normalized view of the detected game UI surface for the status region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuView {
    /// Whether the application currently believes a surface is gating input.
    pub gating: bool,
    /// The surface display name.
    pub state: &'static str,
    /// The palette role for the state field.
    pub role: StatusRole,
}

/// Derives the menu view from the decoded surface.
///
/// Gameplay is shown in the muted role because it is the resting state; an active
/// surface is highlighted, since it means the application is deliberately not
/// intercepting and the operator should be able to see why.
pub fn menu_view(surface: MenuSurface) -> MenuView {
    MenuView {
        gating: surface.gates(),
        state: match surface {
            MenuSurface::None => "Gameplay",
            MenuSurface::SystemMenu => "System menu",
            MenuSurface::Map => "Map",
            MenuSurface::Inventory => "Inventory",
            MenuSurface::Mail => "Mail",
            MenuSurface::Character => "Character",
            MenuSurface::GuildStore => "Guild store",
            MenuSurface::CrownStore => "Crown store",
            MenuSurface::Journal => "Journal",
            MenuSurface::ChatEntry => "Chat entry",
            MenuSurface::Other => "Other menu",
        },
        role: if surface.gates() {
            StatusRole::Active
        } else {
            StatusRole::Muted
        },
    }
}

/// Derives the truthful Game Context projection from all authoritative axes.
pub fn game_context_view(context: GameContext) -> MenuView {
    let (gating, state, role) = match context {
        GameContext::NotDetected => (false, "Not detected", StatusRole::Muted),
        GameContext::Unfocused => (false, "Unfocused", StatusRole::Warning),
        GameContext::Gameplay => (false, "Gameplay", StatusRole::Muted),
        GameContext::SignalUnavailable => (false, "Signal unavailable", StatusRole::Warning),
        GameContext::Unknown => (false, "Unknown", StatusRole::Warning),
        GameContext::Surface(surface) => {
            let view = menu_view(surface);
            return view;
        }
    };
    MenuView {
        gating,
        state,
        role,
    }
}

fn installation_line(state: &InstallationState) -> StatusLine {
    let (text, role) = match state {
        InstallationState::NotDetected => ("Not detected", StatusRole::Muted),
        InstallationState::Ambiguous => ("Multiple installs detected", StatusRole::Warning),
        InstallationState::Unknown => ("Unknown", StatusRole::Warning),
        InstallationState::Detected(candidate) => (
            match candidate.provider {
                InstallationProvider::EsoStore => "Detected (ESO Store)",
                InstallationProvider::Steam => "Detected (Steam)",
                InstallationProvider::Epic => "Detected (Epic Games)",
                InstallationProvider::SteamProton => "Detected (Steam Proton)",
            },
            StatusRole::Healthy,
        ),
    };
    StatusLine {
        title: strings::GAME_INSTALLATION_TITLE,
        state_text: text.to_string(),
        role,
        tooltip: strings::GAME_INSTALLATION_TOOLTIP,
    }
}

fn runtime_line(runtime: GameRuntime) -> StatusLine {
    let (text, role) = match runtime {
        GameRuntime::Inactive => ("Inactive", StatusRole::Muted),
        GameRuntime::LauncherOpen => ("Launcher open", StatusRole::Active),
        GameRuntime::Active => ("Active", StatusRole::Healthy),
        GameRuntime::Unknown => ("Unknown", StatusRole::Warning),
    };
    StatusLine {
        title: strings::GAME_RUNTIME_TITLE,
        state_text: text.to_string(),
        role,
        tooltip: strings::GAME_RUNTIME_TOOLTIP,
    }
}

/// Derives live PixelBeacon signal health independently from addon installation.
pub fn beacon_signal_line(runtime: GameRuntime, freshness: BeaconFreshness) -> StatusLine {
    let (text, role) = match runtime {
        GameRuntime::Inactive | GameRuntime::LauncherOpen => ("Game not active", StatusRole::Muted),
        GameRuntime::Unknown => ("Unknown", StatusRole::Warning),
        GameRuntime::Active => match freshness {
            BeaconFreshness::NeverObserved => ("Not detected", StatusRole::Warning),
            BeaconFreshness::Fresh => ("Signal detected", StatusRole::Healthy),
            BeaconFreshness::Lost => ("Signal lost", StatusRole::Error),
        },
    };
    StatusLine {
        title: strings::BEACON_SIGNAL_TITLE,
        state_text: text.to_string(),
        role,
        tooltip: strings::BEACON_SIGNAL_TOOLTIP,
    }
}

/// Exhaustive presentation state for one resource meter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcePresentation {
    /// A fresh percentage that is above any enabled configured warning threshold.
    Observed(u8),
    /// A fresh percentage at or below an enabled configured warning threshold.
    Low(u8),
    /// ESO is not active, so no live observation is applicable.
    Dormant,
    /// ESO is active but the resource signal is unavailable.
    Unavailable,
}

/// Semantic color family for a resource meter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceTheme {
    /// Health, conventionally red.
    Health,
    /// Stamina, conventionally green.
    Stamina,
    /// Magicka, conventionally blue.
    Magicka,
}

/// A normalized view of one resource pool for the status region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceView {
    /// The typed state used by the meter renderer and accessibility metadata.
    pub presentation: ResourcePresentation,
    /// The exact percentage or explicit non-numeric state.
    pub text: String,
    /// The palette role for the state field.
    pub role: StatusRole,
}

impl ResourceView {
    /// The fresh numeric percentage, absent for dormant or unavailable states.
    pub fn percent(&self) -> Option<u8> {
        match self.presentation {
            ResourcePresentation::Observed(percent) | ResourcePresentation::Low(percent) => {
                Some(percent)
            }
            ResourcePresentation::Dormant | ResourcePresentation::Unavailable => None,
        }
    }

    /// The progress fraction used by the meter fill.
    pub fn fraction(&self) -> Option<f32> {
        self.percent().map(|percent| f32::from(percent) / 100.0)
    }

    /// A coherent non-numeric state for a game that is not active.
    pub fn dormant() -> Self {
        Self {
            presentation: ResourcePresentation::Dormant,
            text: "Game not active".to_string(),
            role: StatusRole::Muted,
        }
    }
}

/// Derives one resource view from its decoded level.
pub fn resource_view(level: ResourceLevel) -> ResourceView {
    resource_view_with_watch(level, ResourceWatch::default())
}

/// Derives one resource view with an explicit user-configured warning threshold.
pub fn resource_view_with_watch(level: ResourceLevel, watch: ResourceWatch) -> ResourceView {
    match level {
        ResourceLevel::Percent(percent) => {
            let low = watch.enabled && percent <= watch.threshold;
            ResourceView {
                presentation: if low {
                    ResourcePresentation::Low(percent)
                } else {
                    ResourcePresentation::Observed(percent)
                },
                text: if low {
                    format!("Low: {percent}%")
                } else {
                    format!("{percent}%")
                },
                role: if low {
                    StatusRole::Warning
                } else {
                    StatusRole::Active
                },
            }
        }
        ResourceLevel::Unknown => ResourceView {
            presentation: ResourcePresentation::Unavailable,
            text: "Signal unavailable".to_string(),
            role: StatusRole::Warning,
        },
    }
}

/// The three resource views, derived together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcesView {
    /// Health.
    pub health: ResourceView,
    /// Stamina.
    pub stamina: ResourceView,
    /// Magicka.
    pub magicka: ResourceView,
}

/// Derives the resource views from the decoded set.
pub fn resources_view(set: ResourceSet) -> ResourcesView {
    ResourcesView {
        health: resource_view(set.health),
        stamina: resource_view(set.stamina),
        magicka: resource_view(set.magicka),
    }
}

/// Derives all resource views with their corresponding configured watches.
pub fn resources_view_with_config(set: ResourceSet, config: AutoPotionConfig) -> ResourcesView {
    ResourcesView {
        health: resource_view_with_watch(set.health, config.health),
        stamina: resource_view_with_watch(set.stamina, config.stamina),
        magicka: resource_view_with_watch(set.magicka, config.magicka),
    }
}

/// A normalized view of one slot's cooldown for the skills region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CooldownView {
    /// The cooldown display text.
    pub text: String,
    /// The palette role for the field.
    pub role: StatusRole,
}

/// Derives the cooldown view from one decoded slot value.
///
/// A running cooldown is shown to a tenth of a second, which is finer than the
/// operator can act on but coarse enough to read at a glance; the encoded
/// resolution is finer still, so nothing is rounded that a later consumer needs.
/// The unknown case renders as the same muted placeholder the other decoded
/// readouts use, and covers both an absent block and a slot the game reports no
/// cooldown for, which is why the Synergy row shows it permanently.
pub fn cooldown_view(cooldown: SlotCooldown) -> CooldownView {
    match cooldown {
        SlotCooldown::Ready => CooldownView {
            text: "Ready".to_string(),
            role: StatusRole::Active,
        },
        SlotCooldown::RemainingMs(ms) => CooldownView {
            text: format!("{:.1}s", f32::from(ms) / 1000.0),
            role: StatusRole::Warning,
        },
        SlotCooldown::Unknown => CooldownView {
            text: "-".to_string(),
            role: StatusRole::Muted,
        },
    }
}

/// A one-line description of the beacon overlay's footprint at a given square
/// size: how many squares, how they are arranged, and the physical pixels they
/// cover.
///
/// This exists because slice 038 doubled the overlay's height, taking it onto a
/// second row for the first time, and an operator should be able to learn what it
/// covers from the application rather than by measuring their own screen. It is
/// shown beside the square-size setting, which is where someone stands when they
/// want the overlay smaller, and it follows the value being edited rather than the
/// value in effect.
///
/// Pure, so the arithmetic is testable without a frame. Derived from the same
/// [`crate::pixelbus::grid_extent`] the reader and the addon use, so it cannot describe a
/// grid other than the one actually drawn.
pub fn grid_footprint_caption(block_px: u32, state: crate::pixelbus::LayoutState) -> String {
    use crate::pixelbus::{LayoutFailure, LayoutMode, LayoutState};
    match state {
        LayoutState::Ready(layout) => {
            let extent = layout.extent(block_px);
            let mode = match layout.mode {
                LayoutMode::Legacy => "Legacy overlay",
                LayoutMode::Negotiated { .. } => "Live overlay",
            };
            let rows = layout.rows();
            let row_label = if rows == 1 { "row" } else { "rows" };
            format!(
                "{mode}: {} data cells, {} columns, {} {row_label}, {} by {} pixels.",
                crate::pixelbus::NUM_BLOCKS,
                layout.columns,
                rows,
                extent.width,
                extent.height
            )
        }
        LayoutState::Unknown => "Live overlay: waiting for PixelBeacon geometry.".to_string(),
        LayoutState::Unavailable(reason) => {
            let reason = match reason {
                LayoutFailure::Missing => "layout header not detected",
                LayoutFailure::InvalidBlockSize => "block size is invalid",
                LayoutFailure::CorruptMagic => "layout magic is corrupt",
                LayoutFailure::UnsupportedVersion { .. } => "layout version is unsupported",
                LayoutFailure::CorruptHighByte | LayoutFailure::CorruptLowByte => {
                    "layout checksum is corrupt"
                }
                LayoutFailure::ColumnsOutOfRange { .. } => "layout columns are invalid",
                LayoutFailure::ExceedsSurface { .. }
                | LayoutFailure::ExtentExceedsSurface { .. } => "layout exceeds the game surface",
            };
            format!("Live overlay unavailable: {reason}.")
        }
    }
}

/// A normalized view of the decoded quickslot for the status region.
/// A normalized view of the explicitly classified quickslot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickslotView {
    /// The selected entry's class or unavailable reason.
    pub state: CooldownView,
    /// Potion availability, or not applicable for other classes.
    pub availability: CooldownView,
    /// The independently decoded cooldown.
    pub cooldown: CooldownView,
}

/// Derives the quickslot view from the decoded state.
///
pub fn quickslot_view(state: QuickslotState) -> QuickslotView {
    let muted = || CooldownView {
        text: "Not applicable".to_string(),
        role: StatusRole::Muted,
    };
    let (classification, availability) = match state.classification {
        QuickslotClassification::Unavailable(reason) => {
            let text = match reason {
                QuickslotUnavailableReason::NoSignal => "Not detected",
                QuickslotUnavailableReason::LegacyProtocol => "Addon update required",
                QuickslotUnavailableReason::CorruptProtocol => "Unreadable signal",
                QuickslotUnavailableReason::UnsupportedApi => "Unsupported game API",
                QuickslotUnavailableReason::InvalidSelection => "Invalid selection",
                QuickslotUnavailableReason::InconsistentFacts => "Inconsistent game data",
            };
            (
                CooldownView {
                    text: text.to_string(),
                    role: StatusRole::Muted,
                },
                muted(),
            )
        }
        QuickslotClassification::Empty => (
            CooldownView {
                text: "Empty".to_string(),
                role: StatusRole::Muted,
            },
            muted(),
        ),
        QuickslotClassification::NonPotion(kind) => {
            let kind = match kind {
                QuickslotNonPotionKind::Item => "Item",
                QuickslotNonPotionKind::Collectible => "Collectible",
                QuickslotNonPotionKind::QuestItem => "Quest item",
                QuickslotNonPotionKind::Emote => "Emote",
                QuickslotNonPotionKind::QuickChat => "Quick chat",
                QuickslotNonPotionKind::Other => "Other",
            };
            (
                CooldownView {
                    text: format!("Non-potion ({kind})"),
                    role: StatusRole::Warning,
                },
                muted(),
            )
        }
        QuickslotClassification::Potion(availability) => {
            let (text, role) = match availability {
                QuickslotPotionAvailability::Depleted => ("Depleted", StatusRole::Warning),
                QuickslotPotionAvailability::Blocked => ("Blocked", StatusRole::Warning),
                QuickslotPotionAvailability::Usable => ("Usable", StatusRole::Active),
            };
            (
                CooldownView {
                    text: "Potion".to_string(),
                    role,
                },
                CooldownView {
                    text: text.to_string(),
                    role,
                },
            )
        }
    };
    QuickslotView {
        state: classification,
        availability,
        cooldown: if state.is_potion() {
            cooldown_view(state.cooldown)
        } else {
            muted()
        },
    }
}

/// The decoded cooldown for one application slot index, or unknown for a slot the
/// game exposes none for.
///
/// Slot 7 is Synergy, which is a contextual prompt rather than an action slot, so
/// the game reports no cooldown for it in any state and it has no beacon block.
fn cooldown_for_slot(cooldowns: CooldownSet, index: u8) -> SlotCooldown {
    match index {
        1 => cooldowns.skill_1,
        2 => cooldowns.skill_2,
        3 => cooldowns.skill_3,
        4 => cooldowns.skill_4,
        5 => cooldowns.skill_5,
        6 => cooldowns.ultimate,
        _ => SlotCooldown::Unknown,
    }
}

/// A view of one skill slot for the skills region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillRow {
    /// The slot index (1 through 7).
    pub index: u8,
    /// The display label (e.g. "Skill 1", "Ultimate (R)", "Synergy (X)").
    pub label: String,
    /// Whether the slot is active.
    pub active: bool,
    /// The slot's weave type.
    pub weave_type: WeaveType,
    /// The per-slot `d_weave` override, if any.
    pub override_d_weave: Option<u32>,
    /// The per-slot `d_heavy` override, if any.
    pub override_d_heavy: Option<u32>,
    /// The per-slot `d_bash` override, if any.
    pub override_d_bash: Option<u32>,
    /// Whether the delay for this row's weave type is overridden.
    pub is_override: bool,
    /// The decoded cooldown for this slot.
    pub cooldown: CooldownView,
    /// The delay in effect for this row's weave type: the override when set,
    /// otherwise the global default for that weave type.
    pub effective_delay: u32,
}

/// The per-slot override (if any) for the delay matching a weave type.
pub fn override_for(overrides: &crate::weave::SlotOverrides, weave_type: WeaveType) -> Option<u32> {
    match weave_type {
        WeaveType::HeavyAttack => overrides.d_heavy,
        WeaveType::BashAttack => overrides.d_bash,
        WeaveType::LightAttack | WeaveType::BlockCasting => overrides.d_weave,
    }
}

/// The global default delay for a weave type.
pub fn default_delay_for(timing: &crate::weave::TimingConfig, weave_type: WeaveType) -> u32 {
    match weave_type {
        WeaveType::HeavyAttack => timing.d_heavy,
        WeaveType::BashAttack => timing.d_bash,
        WeaveType::LightAttack | WeaveType::BlockCasting => timing.d_weave,
    }
}

/// The [`SkillEdit`] that sets or clears the override matching a weave type.
pub fn override_edit_for(weave_type: WeaveType, value: Option<u32>) -> SkillEdit {
    match weave_type {
        WeaveType::HeavyAttack => SkillEdit::OverrideDHeavy(value),
        WeaveType::BashAttack => SkillEdit::OverrideDBash(value),
        WeaveType::LightAttack | WeaveType::BlockCasting => SkillEdit::OverrideDWeave(value),
    }
}

/// Derives the skill rows from the weave configuration.
pub fn skill_rows(config: &WeaveConfig, cooldowns: CooldownSet) -> Vec<SkillRow> {
    config
        .slots
        .iter()
        .map(|slot| {
            let over = override_for(&slot.overrides, slot.weave_type);
            SkillRow {
                index: slot.index,
                cooldown: cooldown_view(cooldown_for_slot(cooldowns, slot.index)),
                label: slot_label(slot.index),
                active: slot.active,
                weave_type: slot.weave_type,
                override_d_weave: slot.overrides.d_weave,
                override_d_heavy: slot.overrides.d_heavy,
                override_d_bash: slot.overrides.d_bash,
                is_override: over.is_some(),
                effective_delay: over
                    .unwrap_or_else(|| default_delay_for(&config.timing, slot.weave_type)),
            }
        })
        .collect()
}

fn slot_label(index: u8) -> String {
    match index {
        6 => "Ultimate (R)".to_string(),
        7 => "Synergy (X)".to_string(),
        n => format!("Skill {n}"),
    }
}

/// An edit to a skill slot from the skills region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillEdit {
    /// Set the slot's active flag.
    Active(bool),
    /// Set the slot's weave type.
    WeaveType(WeaveType),
    /// Set or clear the `d_weave` override.
    OverrideDWeave(Option<u32>),
    /// Set or clear the `d_heavy` override.
    OverrideDHeavy(Option<u32>),
    /// Set or clear the `d_bash` override.
    OverrideDBash(Option<u32>),
}

/// A user action the [`AppModel`] applies.
pub enum UiIntent {
    /// Toggle the input engine suspend state.
    ToggleSuspend,
    /// Enable or disable the fishing controller.
    SetFishing(bool),
    /// Enable or disable auto-potion.
    ///
    /// Deliberately not persisted to the session, unlike suspend and fishing. A
    /// restored fishing session does nothing until the operator stands at a
    /// fishing hole; a restored auto-potion waits silently to press a key days
    /// later. See `specs/039-auto-potion/research.md` R7.
    SetAutoPotion(bool),
    /// Install or update the beacon addon.
    InstallBeacon,
    /// Uninstall the beacon addon (the UI has already confirmed).
    UninstallBeacon,
    /// Update the beacon addon: uninstall the managed copy then install the
    /// current one.
    UpdateBeacon,
    /// Edit a skill slot.
    EditSkill(u8, SkillEdit),
    /// Apply and persist the settings form.
    ApplySettings(Box<SettingsForm>),
    /// Attach or detach the live log panel.
    ToggleLogPanel(bool),
    /// Set the panel-local minimum log level.
    SetLogFilter(LevelName),
    /// Persist the live-log panel height (a user layout preference), in points.
    SetLogHeight(u32),
    /// Persist the System and State disclosure preference.
    SetSystemStateExpanded(bool),
    /// Record the latest window geometry (session state), restored on next launch.
    SetWindowGeometry(WindowGeometry),
}

/// The number of log lines the live-log panel must show at its minimum height,
/// so the panel is readable rather than a one-line sliver.
pub const LOG_MIN_LINES: f32 = 6.0;

/// The live-log panel frame's inner margin (points) on each edge. Shared with the
/// egui bottom-panel frame in [`ui`], so the six-line minimum and the drawn frame
/// agree on the space reserved around the text.
pub const LOG_FRAME_MARGIN: f32 = 6.0;

/// The fraction to scale interactive control heights by, a reduction of about
/// twenty percent (see issue #7). Applied once in [`theme::apply`] so buttons,
/// toggles, and dropdowns shrink consistently.
pub const CONTROL_HEIGHT_SCALE: f32 = 0.8;

/// The minimum interior padding (points) kept above and below a control's text
/// line, so a reduced control height never clips its label.
const CONTROL_MIN_TEXT_PADDING: f32 = 3.0;

/// The reduced interactive-control height (points) for a base height and the
/// control font's line height: about [`CONTROL_HEIGHT_SCALE`] of the base, but
/// never so short that the text line plus its minimum padding would be clipped.
/// Pure and deterministic. See issue #7 (FR-011, FR-012).
pub fn reduced_interact_height(base: f32, font_line_height: f32) -> f32 {
    (base * CONTROL_HEIGHT_SCALE).max(font_line_height + CONTROL_MIN_TEXT_PADDING)
}

/// The minimum live-log panel height (points) that shows [`LOG_MIN_LINES`] lines
/// of log text at the given row height, plus the frame's top and bottom margins.
/// Strictly increasing in `row_height`. Pure. See issue #5 (FR-005).
pub fn log_min_height(row_height: f32) -> f32 {
    LOG_MIN_LINES * row_height + 2.0 * LOG_FRAME_MARGIN
}

/// The minimum window content extent `(width, height)` in points. Before the
/// content has been measured and is stable, a safe boot floor applies so nothing is
/// clipped; once `stable`, the measured extent sets the minimum per dimension and
/// the boot floor no longer applies, so the minimum hugs the real content and can
/// shrink when the content shrinks (issue #8, FR-001/FR-002/FR-004). Pure.
pub fn content_min_size(measured: (f32, f32), boot_floor: (f32, f32), stable: bool) -> (f32, f32) {
    if stable {
        measured
    } else {
        boot_floor
    }
}

/// The padding (points) added around the measured content to form the intrinsic
/// extent: the central panel's frame margins plus a hairline, so the enforced
/// minimum shows the content without clipping its outermost pixel.
pub const CONTENT_PADDING: f32 = 16.0;

/// The intrinsic content extent `(width, height)` in points, from the widest
/// content-sized block and the height the laid-out content actually occupied.
///
/// The distinction this encodes is the root fix of issue #12. Full-width chrome (a
/// separator, the menu bar's background) must contribute height only: including its
/// width would make the extent a function of the window, which is what pinned the
/// enforced minimum to the window's own current size and produced the ratcheted
/// shrink. Pure. See FR-001, FR-007.
pub fn intrinsic_extent(content_width: f32, content_height: f32) -> (f32, f32) {
    (
        content_width + CONTENT_PADDING,
        content_height + CONTENT_PADDING,
    )
}

/// Caps an enforced minimum at the display work area, per axis. A minimum larger
/// than the work area would leave the window unpositionable or unresizable on a
/// small display at a high scale factor. Pure. See FR-008.
pub fn cap_to_work_area(minimum: (f32, f32), work_area: (f32, f32)) -> (f32, f32) {
    (minimum.0.min(work_area.0), minimum.1.min(work_area.1))
}

/// The window size to request when the intrinsic content no longer fits, or `None`
/// when it already does. The window grows to fit its content but never shrinks
/// back: taking away a size the user chose would be its own defect. Pure. See
/// FR-009.
pub fn window_growth_request(intrinsic: (f32, f32), window: (f32, f32)) -> Option<(f32, f32)> {
    let grow_x = intrinsic.0 > window.0 + 0.5;
    let grow_y = intrinsic.1 > window.1 + 0.5;
    if grow_x || grow_y {
        Some((intrinsic.0.max(window.0), intrinsic.1.max(window.1)))
    } else {
        None
    }
}

/// Whether a content measurement is stable: true only when the previous frame's
/// measurement exists and both dimensions are within `epsilon` of the current one.
/// Two consecutive close measurements gate the switch from the boot floor to the
/// measured extent, so a transient first-frame layout never latches the minimum
/// (issue #8, FR-003). Pure.
pub fn measurement_stable(prev: Option<(f32, f32)>, current: (f32, f32), epsilon: f32) -> bool {
    match prev {
        Some((px, py)) => (px - current.0).abs() <= epsilon && (py - current.1).abs() <= epsilon,
        None => false,
    }
}

/// The log height reserved in the enforced minimum open-window height: the six-line
/// minimum plus one extra row. This keeps the pane resizable at the minimum (its
/// maximum is one row above its minimum, so it is never frozen) while still letting
/// the window shrink with the log compressing back toward six lines (issue #8,
/// FR-006). Pure.
pub fn open_log_reserve(row_height: f32) -> f32 {
    log_min_height(row_height) + row_height
}

/// The new live-log panel height (points) after a window-height change, splitting
/// the change proportionally between the central pane and the log pane. The log
/// moves by its live fraction of the usable height (`log_h / prev_window_h`) applied
/// to the delta, then is clamped to `[log_min, max(window_h - content_h, log_min)]`
/// so the log never drops below six lines and the central pane never drops below its
/// content; the central pane is the remainder, absorbing any rounding (issue #8,
/// FR-007/FR-008). Pure. A non-positive `prev_window_h` falls back to clamping the
/// current height.
pub fn split_log_height(
    prev_window_h: f32,
    window_h: f32,
    log_h: f32,
    content_h: f32,
    log_min: f32,
) -> f32 {
    let hi = (window_h - content_h).max(log_min);
    if prev_window_h <= 0.0 {
        return log_h.clamp(log_min, hi);
    }
    let fraction = log_h / prev_window_h;
    let target = log_h + fraction * (window_h - prev_window_h);
    target.clamp(log_min, hi)
}

/// Clamps a live-log panel height (points) into its valid range for the given
/// window height: at least the six-line minimum ([`log_min_height`] at
/// `row_height`), at most `window_height - content_min_height` so the pane top can
/// never cross into the interactive (Skills) area. When the window is too short
/// for both, the minimum wins and the pane collapses to its readable floor rather
/// than covering the controls. See issue #5 (FR-005, FR-007).
pub fn clamp_log_height(
    height: f32,
    window_height: f32,
    row_height: f32,
    content_min_height: f32,
) -> f32 {
    let min = log_min_height(row_height);
    let max = (window_height - content_min_height).max(min);
    height.clamp(min, max)
}

/// The greatest live-log panel height (points) that cannot overlap the central
/// content, which is simply the space left above it. Never negative.
///
/// This is the unconditional half of the log-pane boundary. [`clamp_log_height`]
/// resolves a window too short for both the content and a six-line log in favor of
/// the log's readable floor, which lets the pane cover controls in that degenerate
/// case; issue #13 states that covering an interactive control is always a hard
/// fail, so the rendered height takes the minimum of the two. In any window large
/// enough to satisfy the enforced minimum the two agree and this bound is inert.
/// Pure. See FR-010.
pub fn log_max_height_no_overlap(window_height: f32, content_height: f32) -> f32 {
    (window_height - content_height).max(0.0)
}

/// Computes a settings-modal dimension (points) for a window dimension. The modal
/// grows sub-linearly with the window, so it takes a progressively smaller fraction
/// of a larger window while its absolute size keeps increasing, is clamped to
/// `[min_px, max_px]`, and never exceeds `max_frac` of the window (so it always
/// fits, even a very small window). Pure and deterministic.
pub fn modal_extent(window: f32, min_px: f32, max_px: f32, max_frac: f32) -> f32 {
    // Grow at a fraction of the window's growth past the minimum, so the occupied
    // fraction decreases as the window enlarges.
    const GROWTH: f32 = 0.55;
    let grown = min_px + (window - min_px).max(0.0) * GROWTH;
    grown.clamp(min_px, max_px).min(window * max_frac)
}

/// The derived display state for one frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppView {
    /// The app-state indicator and button.
    pub app_state: AppStateLabel,
    /// The fishing indicator and button.
    pub fishing: FishingLabel,
    /// The normalized Status line (title, colorized state, tooltip).
    pub status_line: StatusLine,
    /// The normalized Fishing line.
    pub fishing_line: StatusLine,
    /// The normalized Pixel Beacon line.
    pub beacon_line: StatusLine,
    /// Live PixelBeacon freshness, independent from addon installation.
    pub beacon_signal_line: StatusLine,
    /// Distribution-platform installation evidence.
    pub installation_line: StatusLine,
    /// Process-derived ESO lifecycle state.
    pub runtime_line: StatusLine,
    /// Whether the engine is currently suspended (for the Status toggle).
    pub suspended: bool,
    /// Whether fishing is currently active (for the Fishing toggle).
    pub fishing_active: bool,
    /// The beacon status light.
    pub beacon: BeaconLight,
    /// The derived beacon condition.
    pub beacon_condition: BeaconCondition,
    /// Whether the Uninstall control is enabled.
    pub uninstall_enabled: bool,
    /// One row per skill slot.
    pub skills: Vec<SkillRow>,
    /// The detected weapon-bar state.
    pub weapon_bar: WeaponBarView,
    /// The detected combat state.
    pub combat: CombatView,
    /// The detected movement state.
    pub movement: MovementView,
    /// The authoritative player life state.
    pub life: LifeStateView,
    /// The detected game UI surface, and whether it is gating input.
    pub menu: MenuView,
    /// The detected resource levels.
    pub resources: ResourcesView,
    /// The detected quickslot state.
    pub quickslot: QuickslotView,
    /// Whether the operator currently requests auto-potion.
    pub auto_potion_requested: bool,
    /// The effective auto-potion state or current blocker.
    pub auto_potion: AutoPotionView,
    /// Whether the log panel is attached.
    pub log_panel_open: bool,
    /// The panel-local minimum log level.
    pub log_filter: LevelName,
}

/// Coalesces persistence so a continuous edit results in a single settle-write.
///
/// A change marks the config and/or session store dirty and records the time.
/// [`should_flush`](SaveScheduler::should_flush) becomes true once the store is
/// dirty and the most recent change has settled for the configured interval.
///
/// A change is either meaningful (a settings change: a toggle or a form-field
/// edit) or layout-only (a window move/resize or a log-pane resize). Both persist
/// identically, but only a meaningful change sets `dirty_notify`, which is what
/// gates the "Settings saved" confirmation so pure layout writes stay silent
/// (issue #6, FR-009/FR-010). Invariant: `dirty_notify` implies dirty.
#[derive(Debug)]
pub struct SaveScheduler {
    dirty_config: bool,
    dirty_session: bool,
    dirty_notify: bool,
    last_change: Option<Instant>,
    settle: Duration,
}

impl SaveScheduler {
    /// Creates a scheduler that flushes once a change has settled for `settle`.
    pub fn new(settle: Duration) -> Self {
        Self {
            dirty_config: false,
            dirty_session: false,
            dirty_notify: false,
            last_change: None,
            settle,
        }
    }

    /// Marks the configuration store dirty for a meaningful settings change as of
    /// `now` (raises the save confirmation on flush).
    pub fn mark_config(&mut self, now: Instant) {
        self.dirty_config = true;
        self.dirty_notify = true;
        self.last_change = Some(now);
    }

    /// Marks the session store dirty for a meaningful settings change as of `now`
    /// (raises the save confirmation on flush).
    pub fn mark_session(&mut self, now: Instant) {
        self.dirty_session = true;
        self.dirty_notify = true;
        self.last_change = Some(now);
    }

    /// Marks the configuration store dirty for a layout-only change (log-pane
    /// height) as of `now`. Persists exactly like [`mark_config`](Self::mark_config)
    /// but does not raise the save confirmation.
    pub fn mark_config_layout(&mut self, now: Instant) {
        self.dirty_config = true;
        self.last_change = Some(now);
    }

    /// Marks the session store dirty for a layout-only change (window geometry) as
    /// of `now`. Persists exactly like [`mark_session`](Self::mark_session) but does
    /// not raise the save confirmation.
    pub fn mark_session_layout(&mut self, now: Instant) {
        self.dirty_session = true;
        self.last_change = Some(now);
    }

    /// Whether anything is pending a write.
    pub fn is_dirty(&self) -> bool {
        self.dirty_config || self.dirty_session
    }

    /// Whether a meaningful (confirmation-worthy) change is pending.
    pub fn pending_notify(&self) -> bool {
        self.dirty_notify
    }

    /// Whether a flush is due: dirty and settled for the configured interval.
    pub fn should_flush(&self, now: Instant) -> bool {
        match self.last_change {
            Some(t) => self.is_dirty() && now.duration_since(t) >= self.settle,
            None => false,
        }
    }

    /// Clears the dirty flags and returns which stores need writing
    /// `(config, session)`.
    pub fn take(&mut self) -> (bool, bool) {
        let flags = (self.dirty_config, self.dirty_session);
        self.dirty_config = false;
        self.dirty_session = false;
        self.dirty_notify = false;
        self.last_change = None;
        flags
    }

    /// Clears only the session dirty flag, after a forced session write. A pending
    /// config write is left intact.
    pub fn clear_session(&mut self) {
        self.dirty_session = false;
        if !self.dirty_config {
            self.last_change = None;
        }
    }
}

/// The result of a coalesced flush: whether any store was written, and whether the
/// write included a meaningful settings change worth confirming to the user. A
/// layout-only flush (window geometry or log-pane height) has `wrote == true` and
/// `notify == false`, so it persists silently (issue #6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FlushOutcome {
    /// Whether at least one store was written.
    pub wrote: bool,
    /// Whether a meaningful settings change was in the flushed batch.
    pub notify: bool,
}

/// The application model: holds the shared subsystem handles, derives the view,
/// and applies UI intents.
pub struct AppModel {
    input: Arc<InputEngine>,
    weave: Arc<Mutex<WeaveEngine>>,
    fishing: Arc<Mutex<FishingController>>,
    fishing_sink: Box<dyn FishingSink + Send>,
    potion: Arc<Mutex<AutoPotionController>>,
    game: GameState,
    clock: Instant,
    log: LogHandle,
    settings: Settings,
    config_dir: Option<PathBuf>,
    beacon_prefs: BeaconPrefs,
    runtime_block_px: u32,
    log_panel_open: bool,
    log_filter: LevelName,
    scheduler: SaveScheduler,
    api_version: ApiVersionCache,
    window: Option<WindowGeometry>,
}

impl AppModel {
    /// Creates the model over the given shared subsystems and configuration.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        input: Arc<InputEngine>,
        weave: Arc<Mutex<WeaveEngine>>,
        fishing: Arc<Mutex<FishingController>>,
        fishing_sink: Box<dyn FishingSink + Send>,
        potion: Arc<Mutex<AutoPotionController>>,
        log: LogHandle,
        settings: Settings,
        config_dir: Option<PathBuf>,
        clock: Instant,
    ) -> Self {
        Self::new_with_game(
            input,
            weave,
            fishing,
            fishing_sink,
            potion,
            GameState::default(),
            log,
            settings,
            config_dir,
            clock,
        )
    }

    /// Creates the model over a caller-owned shared game observation handle.
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_game(
        input: Arc<InputEngine>,
        weave: Arc<Mutex<WeaveEngine>>,
        fishing: Arc<Mutex<FishingController>>,
        fishing_sink: Box<dyn FishingSink + Send>,
        potion: Arc<Mutex<AutoPotionController>>,
        game: GameState,
        log: LogHandle,
        settings: Settings,
        config_dir: Option<PathBuf>,
        clock: Instant,
    ) -> Self {
        let beacon_prefs = beacon::prefs_from_value(&settings.beacon);
        let log_filter = settings.logging.level;
        let mut reader_notices = Vec::new();
        let runtime_block_px =
            crate::pixelbus::load_reader_config(&settings.pixelbus, &mut reader_notices).block_px;
        Self {
            input,
            weave,
            fishing,
            fishing_sink,
            potion,
            game,
            clock,
            log,
            settings,
            config_dir,
            beacon_prefs,
            runtime_block_px,
            log_panel_open: false,
            log_filter,
            scheduler: SaveScheduler::new(Duration::from_millis(400)),
            api_version: ApiVersionCache::default(),
            window: None,
        }
    }

    /// A shared handle to the logging facility (for the log panel snapshot).
    pub fn log_handle(&self) -> LogHandle {
        self.log.clone()
    }

    /// Returns the shared game observation handle.
    pub fn game_state(&self) -> GameState {
        self.game.clone()
    }

    /// The current GUI preferences (theme and always-on-top).
    pub fn ui_prefs(&self) -> UiPrefs {
        settings_form::ui_from_value(&self.settings.ui).0
    }

    /// A fresh settings form seeded from the current configuration.
    pub fn settings_form(&self) -> SettingsForm {
        SettingsForm::load(&self.settings).0
    }

    /// Current runtime pixel-bus geometry for the settings footprint caption.
    pub fn layout_state(&self) -> crate::pixelbus::LayoutState {
        self.weave.lock().unwrap().layout()
    }

    /// Block size used by this process's reader until the next application start.
    pub fn runtime_block_px(&self) -> u32 {
        self.runtime_block_px
    }

    /// The current derived display state.
    pub fn view(&self) -> AppView {
        let condition = self.beacon_condition();
        let (fishing_state, fishing_reason, fishing_requested) = {
            let fishing = self.fishing.lock().unwrap();
            (fishing.state(), fishing.stop_reason(), fishing.enabled())
        };
        let (auto_potion_requested, auto_potion_state, auto_potion_config) = {
            let potion = self.potion.lock().unwrap();
            (potion.enabled(), potion.state(), *potion.config())
        };
        let (mut skills, active_bar, classes, combat, movement, life, resources, quickslot) = {
            let cooldowns = self.weave.lock().unwrap().cooldowns();
            let weave = self.weave.lock().unwrap();
            (
                skill_rows(weave.config(), cooldowns),
                weave.active_bar(),
                weave.weapon_classes(),
                weave.combat(),
                weave.movement(),
                weave.life(),
                weave.resources(),
                weave.quickslot(),
            )
        };
        let suspended = self.input.is_suspended();
        let game = self.game.snapshot();
        let active = game.runtime == GameRuntime::Active;
        let mut weapon_bar = weapon_bar_view(active_bar, classes.0, classes.1);
        let mut combat = combat_view(combat);
        let mut movement = movement_view(movement);
        let mut life = life_state_view(life);
        let mut resources = resources_view_with_config(resources, auto_potion_config);
        let mut quickslot = quickslot_view(quickslot);
        if !active {
            weapon_bar = WeaponBarView {
                detected: false,
                active_bar: "Game not active",
                front: "Game not active",
                back: "Game not active",
                role: StatusRole::Muted,
            };
            combat = CombatView {
                detected: false,
                state: "Game not active",
                role: StatusRole::Muted,
            };
            movement = MovementView {
                detected: false,
                state: "Game not active",
                role: StatusRole::Muted,
            };
            life = LifeStateView {
                state: "Game not active",
                role: StatusRole::Muted,
            };
            for value in [
                &mut resources.health,
                &mut resources.stamina,
                &mut resources.magicka,
            ] {
                *value = ResourceView::dormant();
            }
            for value in [
                &mut quickslot.state,
                &mut quickslot.availability,
                &mut quickslot.cooldown,
            ] {
                value.text = "Game not active".to_string();
                value.role = StatusRole::Muted;
            }
            for skill in &mut skills {
                skill.cooldown.text = "Game not active".to_string();
                skill.cooldown.role = StatusRole::Muted;
            }
        }
        let mut fishing_label = fishing_label(fishing_state, fishing_reason);
        if fishing_requested {
            fishing_label.button = "Stop Fishing";
        }
        AppView {
            app_state: app_state_label(suspended),
            fishing: fishing_label,
            status_line: status_line_app(suspended),
            fishing_line: status_line_fishing(fishing_state, fishing_reason),
            beacon_line: status_line_beacon(condition),
            beacon_signal_line: beacon_signal_line(game.runtime, game.freshness),
            installation_line: installation_line(&game.installation),
            runtime_line: runtime_line(game.runtime),
            suspended,
            fishing_active: fishing_requested,
            beacon: beacon_light(condition),
            beacon_condition: condition,
            uninstall_enabled: uninstall_enabled(condition),
            skills,
            weapon_bar,
            combat,
            movement,
            life,
            menu: game_context_view(game.context()),
            resources,
            quickslot,
            auto_potion_requested,
            auto_potion: auto_potion_view(auto_potion_state),
            log_panel_open: self.log_panel_open,
            log_filter: self.log_filter,
        }
    }

    /// Derives the beacon condition from discovery plus the on-disk status.
    pub fn beacon_condition(&self) -> BeaconCondition {
        match beacon::resolve_addons_dir(&self.beacon_prefs) {
            Ok(root) => BeaconCondition::from_status(beacon::status(&root)),
            Err(_) => BeaconCondition::AddonsNotFound,
        }
    }

    /// Applies a UI intent, returning any notices to surface.
    pub fn apply_intent(&mut self, intent: UiIntent) -> Vec<Notice> {
        match intent {
            UiIntent::ToggleSuspend => {
                let now = self.input.is_suspended();
                self.input.set_suspended(!now);
                self.scheduler.mark_session(Instant::now());
                Vec::new()
            }
            UiIntent::SetFishing(enabled) => {
                let now = self.now_ms();
                self.fishing
                    .lock()
                    .unwrap()
                    .set_enabled(enabled, now, self.fishing_sink.as_mut());
                self.scheduler.mark_session(Instant::now());
                Vec::new()
            }
            UiIntent::SetAutoPotion(enabled) => {
                self.potion.lock().unwrap().set_enabled(enabled);
                // No `mark_session`: the enable is deliberately not persisted.
                Vec::new()
            }
            UiIntent::InstallBeacon => {
                self.install_beacon();
                Vec::new()
            }
            UiIntent::UninstallBeacon => {
                self.uninstall_beacon();
                Vec::new()
            }
            UiIntent::UpdateBeacon => {
                // A clean reinstall: remove the managed copy (marker-gated, so an
                // unmanaged folder is never deleted) then install the current one.
                self.uninstall_beacon();
                self.install_beacon();
                Vec::new()
            }
            UiIntent::EditSkill(index, edit) => {
                self.edit_skill(index, edit);
                self.scheduler.mark_config(Instant::now());
                Vec::new()
            }
            UiIntent::ApplySettings(form) => self.apply_settings(*form),
            UiIntent::ToggleLogPanel(open) => {
                self.log_panel_open = open;
                Vec::new()
            }
            UiIntent::SetLogFilter(level) => {
                // The live-log level and the settings Log level are one setting:
                // changing the panel dropdown also changes what is captured and
                // persists it, so the two controls stay in sync.
                self.log_filter = level;
                self.settings.logging.level = level;
                self.log.set_level(level);
                self.scheduler.mark_config(Instant::now());
                Vec::new()
            }
            UiIntent::SetLogHeight(height) => {
                let mut prefs = settings_form::ui_from_value(&self.settings.ui).0;
                prefs.log_panel_height = height;
                self.settings.ui = settings_form::ui_to_value(&prefs);
                // Resizing the log pane is a layout change: it persists but does
                // not raise the save confirmation (issue #6).
                self.scheduler.mark_config_layout(Instant::now());
                Vec::new()
            }
            UiIntent::SetSystemStateExpanded(expanded) => {
                let mut prefs = settings_form::ui_from_value(&self.settings.ui).0;
                prefs.system_state_expanded = expanded;
                self.settings.ui = settings_form::ui_to_value(&prefs);
                self.scheduler.mark_config_layout(Instant::now());
                Vec::new()
            }
            UiIntent::SetWindowGeometry(geometry) => {
                if self.window != Some(geometry) {
                    self.window = Some(geometry);
                    // Moving or resizing the window is a layout change: it persists
                    // but does not raise the save confirmation (issue #6).
                    self.scheduler.mark_session_layout(Instant::now());
                }
                Vec::new()
            }
        }
    }

    /// The current time in milliseconds on the shared monotonic clock the model
    /// was constructed with. Fishing deadlines are stamped against this clock, and
    /// the pixel-bus worker evaluates them against the same origin.
    pub fn now_ms(&self) -> u64 {
        self.clock.elapsed().as_millis() as u64
    }

    fn edit_skill(&mut self, index: u8, edit: SkillEdit) {
        {
            let mut weave = self.weave.lock().unwrap();
            if let Some(slot) = weave
                .config_mut()
                .slots
                .iter_mut()
                .find(|slot| slot.index == index)
            {
                match edit {
                    SkillEdit::Active(active) => slot.active = active,
                    SkillEdit::WeaveType(weave_type) => slot.weave_type = weave_type,
                    SkillEdit::OverrideDWeave(value) => slot.overrides.d_weave = value,
                    SkillEdit::OverrideDHeavy(value) => slot.overrides.d_heavy = value,
                    SkillEdit::OverrideDBash(value) => slot.overrides.d_bash = value,
                }
            }
            weave.apply_activity(&self.input);
        }
    }

    /// The effective, sanitized block size from the current settings (defaulting
    /// to the standard size when unset). Notices from sanitization are surfaced on
    /// the settings load path, so they are discarded here.
    fn configured_block_px(&self) -> u32 {
        let mut discard = Vec::new();
        crate::pixelbus::load_reader_config(&self.settings.pixelbus, &mut discard).block_px
    }

    fn install_beacon(&mut self) {
        let api_version = self.effective_api_version();
        let block_px = self.configured_block_px();
        match beacon::resolve_addons_dir(&self.beacon_prefs) {
            Ok(root) => match beacon::install_sized(
                &root,
                beacon::probe_game_running(),
                api_version,
                block_px,
            ) {
                Ok(outcome) => {
                    if outcome.reload_required {
                        tracing::info!(
                            target: "eso_weave::app",
                            "PixelBeacon installed; run /reloadui or relog for it to take effect"
                        );
                    } else {
                        tracing::info!(target: "eso_weave::app", "PixelBeacon installed");
                    }
                }
                Err(err) => {
                    tracing::warn!(target: "eso_weave::app", "install failed: {err}");
                }
            },
            Err(_) => {
                tracing::warn!(target: "eso_weave::app", "install: AddOns directory not found");
            }
        }
    }

    fn uninstall_beacon(&mut self) {
        match beacon::resolve_addons_dir(&self.beacon_prefs) {
            Ok(root) => match beacon::uninstall(&root, beacon::probe_game_running()) {
                Ok(outcome) => {
                    if outcome.reload_required {
                        tracing::info!(
                            target: "eso_weave::app",
                            "PixelBeacon removed; run /reloadui or relog for it to take effect"
                        );
                    } else {
                        tracing::info!(target: "eso_weave::app", "PixelBeacon removed");
                    }
                }
                Err(err) => {
                    tracing::warn!(target: "eso_weave::app", "uninstall refused: {err}");
                }
            },
            Err(_) => {
                tracing::warn!(target: "eso_weave::app", "uninstall: AddOns directory not found");
            }
        }
    }

    fn apply_settings(&mut self, form: SettingsForm) -> Vec<Notice> {
        let previous_block_px = self.configured_block_px();
        form.apply(&mut self.settings);
        let notices = self.reload_from_settings();
        // A block-size change re-derives the reader geometry (adopted on the next
        // start) and re-deploys the addon so the drawn squares match. The
        // comparison runs both values through the same sanitize path.
        let current_block_px = self.configured_block_px();
        if current_block_px != previous_block_px {
            self.redeploy_for_block_size(previous_block_px, current_block_px);
        }
        // Persistence is coalesced: mark the config store dirty and let the
        // scheduler flush a single settle-write. There is no explicit save.
        self.scheduler.mark_config(Instant::now());
        notices
    }

    /// Drives a managed-only re-deploy after the block size changed. An unmanaged
    /// or absent install is never written; the outcome is surfaced in the live log
    /// (the addon and reader adopt the new size after a `/reloadui` and an app
    /// restart respectively).
    fn redeploy_for_block_size(&self, previous: u32, current: u32) {
        let api_version = self.effective_api_version();
        let root = match beacon::resolve_addons_dir(&self.beacon_prefs) {
            Ok(root) => root,
            Err(_) => {
                tracing::warn!(
                    target: "eso_weave::app",
                    "block size changed but the AddOns directory was not found; PixelBeacon not re-deployed"
                );
                return;
            }
        };
        match beacon::redeploy_for_block_size(
            &root,
            beacon::probe_game_running(),
            api_version,
            current,
        ) {
            Ok(beacon::RedeployOutcome::Redeployed(_)) => {
                tracing::info!(
                    target: "eso_weave::app",
                    previous,
                    current,
                    "PixelBeacon re-deployed at the new block size; run /reloadui and restart ESO Weave for it to take effect"
                );
            }
            Ok(beacon::RedeployOutcome::SkippedUnmanaged) => {
                tracing::warn!(
                    target: "eso_weave::app",
                    "block size changed but PixelBeacon is unmanaged; the folder was not modified. Reinstall to apply the new size"
                );
            }
            Ok(beacon::RedeployOutcome::SkippedNotInstalled) => {
                tracing::info!(
                    target: "eso_weave::app",
                    current,
                    "block size changed; PixelBeacon is not installed. The new size applies on the next install and app restart"
                );
            }
            Err(err) => {
                tracing::warn!(target: "eso_weave::app", "block-size re-deploy failed: {err}");
            }
        }
    }

    /// Restores the persisted session state (suspend and fishing intents) on
    /// launch. Restoring a running or fishing-on state performs no input while
    /// the game window is unfocused, because synthesis and suppression are scoped
    /// to the focused game window by the input backend.
    pub fn restore_session(&mut self, state: SessionState) {
        self.api_version = state.api_version;
        self.window = state.window;
        if state.suspended != self.input.is_suspended() {
            self.input.set_suspended(state.suspended);
        }
        if state.fishing {
            let now = self.now_ms();
            self.fishing
                .lock()
                .unwrap()
                .set_enabled(true, now, self.fishing_sink.as_mut());
        }
    }

    /// The effective numeric API version for rendering a manifest: the higher of
    /// the last known value and the compiled default. Never below the default.
    pub fn effective_api_version(&self) -> u32 {
        self.api_version
            .last_known_api_version
            .unwrap_or(0)
            .max(beacon::DEFAULT_API_VERSION)
    }

    /// Applies a startup version-check outcome: updates the cache and, when it
    /// changed, marks the session store dirty so the value is persisted through the
    /// existing coalesced save path. The bump notice is emitted by the check thread
    /// via tracing and shown in the live log; nothing is surfaced here.
    pub fn apply_api_check(&mut self, outcome: ApiCheckOutcome) {
        let updated = ApiVersionCache {
            last_known_api_version: Some(outcome.last_known_api_version),
            last_seen_game_version: outcome
                .last_seen_game_version
                .or(self.api_version.last_seen_game_version),
        };
        if updated != self.api_version {
            self.api_version = updated;
            self.scheduler.mark_session(Instant::now());
        }
    }

    /// Whether fishing is currently on (enabled), the live on/off intent used to
    /// negate a hotkey fishing toggle so a hotkey and the Fishing button share one
    /// state. Mirrors the check in [`current_session_state`](Self::current_session_state).
    pub fn fishing_on(&self) -> bool {
        self.fishing.lock().unwrap().enabled()
    }

    /// Whether auto-potion is currently switched on.
    pub fn auto_potion_on(&self) -> bool {
        self.potion.lock().unwrap().enabled()
    }

    /// The current session state to persist (suspend flag and fishing on/off
    /// intent, never a transient fishing sub-state).
    pub fn current_session_state(&self) -> SessionState {
        let fishing_on = self.fishing.lock().unwrap().enabled();
        SessionState {
            schema_version: CURRENT_STATE_VERSION,
            suspended: self.input.is_suspended(),
            fishing: fishing_on,
            api_version: self.api_version,
            window: self.window,
        }
    }

    /// Flushes any pending coalesced writes if they have settled. Returns whether a
    /// write occurred and whether that write included a meaningful settings change
    /// (so the caller can show the save confirmation only for real changes, not for
    /// pure layout writes; issue #6).
    pub fn maybe_flush(&mut self, now: Instant) -> FlushOutcome {
        if !self.scheduler.should_flush(now) {
            return FlushOutcome::default();
        }
        let notify = self.scheduler.pending_notify();
        let (write_config, write_session) = self.scheduler.take();
        let Some(dir) = self.config_dir.clone() else {
            return FlushOutcome::default();
        };
        let mut saved = false;
        if write_config {
            self.store_live_into_settings();
            match config::save(&dir, &self.settings) {
                Ok(()) => saved = true,
                Err(err) => {
                    tracing::warn!(target: "eso_weave::config", "could not save settings: {err}");
                }
            }
        }
        if write_session {
            let state = self.current_session_state();
            match config::state::save(&dir, &state) {
                Ok(()) => saved = true,
                Err(err) => {
                    tracing::warn!(target: "eso_weave::config", "could not save session state: {err}");
                }
            }
        }
        FlushOutcome {
            wrote: saved,
            notify: notify && saved,
        }
    }

    /// Forces an immediate write of the current session state, used on window
    /// close so a change made in the final moments before exit (for example a
    /// resize) is not lost to the settle-delayed scheduler. Returns whether a
    /// write occurred.
    pub fn flush_session_now(&mut self) -> bool {
        let Some(dir) = self.config_dir.clone() else {
            return false;
        };
        let state = self.current_session_state();
        match config::state::save(&dir, &state) {
            Ok(()) => {
                self.scheduler.clear_session();
                true
            }
            Err(err) => {
                tracing::warn!(target: "eso_weave::config", "could not save session state on close: {err}");
                false
            }
        }
    }

    /// Syncs the live weave engine configuration back into the settings so
    /// main-window skill edits are persisted.
    fn store_live_into_settings(&mut self) {
        let weave = self.weave.lock().unwrap();
        weave.store(&mut self.settings);
    }

    /// Reloads the live subsystems from the current settings, returning fallback
    /// notices.
    pub fn reload_from_settings(&mut self) -> Vec<Notice> {
        let mut notices = Vec::new();
        notices.extend(self.input.load_bindings(&self.settings));
        notices.extend(self.weave.lock().unwrap().load(&self.settings));
        self.weave.lock().unwrap().apply_activity(&self.input);
        let potion_config = AutoPotionConfig::load(&self.settings.potion, &mut notices);
        self.potion.lock().unwrap().set_config(potion_config);
        self.beacon_prefs = beacon::prefs_from_value(&self.settings.beacon);
        self.log.set_level(self.settings.logging.level);
        self.log
            .set_file_enabled(self.settings.logging.file_enabled);
        // Keep the live-log panel dropdown in sync with the settings Log level, so
        // applying a settings change updates the panel too.
        self.log_filter = self.settings.logging.level;
        notices
    }

    /// Whether the beacon status is currently installed (for enabling controls).
    pub fn beacon_installed(&self) -> bool {
        !matches!(self.beacon_condition(), BeaconCondition::NotInstalled)
            && self.beacon_status().is_some()
    }

    fn beacon_status(&self) -> Option<BeaconStatus> {
        beacon::resolve_addons_dir(&self.beacon_prefs)
            .ok()
            .map(|root| beacon::status(&root))
    }
}
