//! Centralized user-facing UI strings: section titles, control labels, tooltips,
//! and settings help text.
//!
//! Keeping them in one place lets the render layer stay thin, keeps wording for
//! the same concept consistent across the window and the settings modal, and lets
//! tests assert coverage (every control has a tooltip) and hygiene (no user-facing
//! label contains an underscore).

// Status region titles.
pub const LIVE_HUD_TITLE: &str = "Live HUD";
pub const SYSTEM_STATE_TITLE: &str = "System and State";
pub const SYSTEM_STATE_TOOLTIP: &str =
    "Expand or collapse application, addon, and automation state. This layout preference is saved.";
pub const STATUS_TITLE: &str = "ESO Weave";
pub const FISHING_TITLE: &str = "Fishing";
pub const BEACON_TITLE: &str = "PixelBeacon installation";
pub const BEACON_SIGNAL_TITLE: &str = "PixelBeacon signal";
pub const GAME_TITLE: &str = "Game";
pub const GAME_INSTALLATION_TITLE: &str = "Game installation";
pub const GAME_RUNTIME_TITLE: &str = "Game state";
pub const LIFE_TITLE: &str = "Life state";

// Status region tooltips.
pub const STATUS_TOOLTIP: &str =
    "Whether the weave engine is running or suspended. Input is only ever sent while the game window is focused.";
pub const FISHING_TOOLTIP: &str =
    "Whether the fishing routine is active. It reads the Pixel Beacon signal to detect bites.";
pub const BEACON_TOOLTIP: &str =
    "Install state of the bundled PixelBeacon companion addon that renders the pixel signal.";
pub const BEACON_SIGNAL_TOOLTIP: &str =
    "Whether a fresh PixelBeacon signal is currently available from the active ESO client.";
pub const GAME_INSTALLATION_TOOLTIP: &str =
    "Whether ESO is installed and which distribution platform supplied the authoritative evidence.";
pub const GAME_RUNTIME_TOOLTIP: &str =
    "Whether the ESO client is inactive, its launcher is open, or the game client is active.";

// Fishing status indicators: plain language for each fishing routine phase, and
// for an idle state the reason the routine last stopped.
pub const FISHING_CASTING: &str = "Casting";
pub const FISHING_WAITING: &str = "Fishing (waiting for a bite)";
pub const FISHING_REELING: &str = "Reeling in";
pub const FISHING_RECASTING: &str = "Recasting";
pub const FISHING_IDLE: &str = "Idle";
pub const FISHING_IDLE_NO_CAST: &str = "Idle (no cast detected)";
pub const FISHING_IDLE_SIGNAL_LOST: &str = "Idle (signal lost)";
pub const FISHING_IDLE_GAME_INACTIVE: &str = "Idle (game not active)";
pub const FISHING_IDLE_UNFOCUSED: &str = "Idle (game not focused)";
pub const FISHING_IDLE_PLAYER_UNAVAILABLE: &str = "Idle (player unavailable)";

// Status region toggles.
pub const SUSPEND_LABEL: &str = "Running";
pub const SUSPEND_TOOLTIP: &str = "Suspend or resume the weave engine.";
pub const FISHING_TOGGLE_LABEL: &str = "Fishing";
pub const FISHING_TOGGLE_TOOLTIP: &str = "Start or stop the fishing routine.";
pub const BEACON_INSTALL_TOOLTIP: &str =
    "Install or update the PixelBeacon addon in your AddOns folder.";
pub const BEACON_UPDATE_TOOLTIP: &str =
    "Reinstall the PixelBeacon addon: remove the managed copy and install the current one. Enabled only when the addon is installed.";
pub const BEACON_UNINSTALL_TOOLTIP: &str =
    "Remove the PixelBeacon addon. Only a folder marked as managed by ESO Weave is deleted.";

// Weapon-bar section.
pub const WEAPON_BAR_TITLE: &str = "Active weapon bar";
pub const WEAPON_BAR_TOOLTIP: &str =
    "The active weapon bar and the weapon class detected on each bar. Requires the updated Pixel Beacon addon; shows Unknown without a signal.";

// Combat-state section.
pub const COMBAT_TITLE: &str = "Combat";
pub const COMBAT_TOOLTIP: &str =
    "Whether the character is in combat, read from the Pixel Beacon addon. Requires addon version 6 or later; shows Not detected without a signal. Nothing in the application acts on this yet.";

// Movement-state section.
pub const MOVEMENT_TITLE: &str = "Movement";
pub const MOVEMENT_TOOLTIP: &str =
    "Whether the character is mounted, read from the Pixel Beacon addon. Requires addon version 10 or later; shows Not detected without a signal. Sprinting is not shown because the game exposes no sprint state to an addon. Nothing in the application acts on this yet.";
pub const LIFE_TOOLTIP: &str =
    "Whether the character is alive, dead, or reincarnating. Only a fresh Alive signal permits automated input; unavailable evidence blocks safely.";

// Menu-gate section.
pub const MENU_TITLE: &str = "Game Context";
pub const MENU_TOOLTIP: &str =
    "Combines game activity, focus, Pixel Beacon freshness, and the observed game surface. Gameplay requires a fresh valid no-menu observation. Unavailable means the evidence is not authoritative. Menu and text-entry states gate input.";

// Resource section.
pub const HEALTH_TITLE: &str = "Health";
pub const STAMINA_TITLE: &str = "Stamina";
pub const MAGICKA_TITLE: &str = "Magicka";
pub const RESOURCE_TOOLTIP: &str =
    "The pool as a percentage of its current maximum, read from the Pixel Beacon addon. Requires addon version 8 or later; shows Not detected without a signal. Nothing in the application acts on these yet.";

// Quickslot section.
pub const QUICKSLOT_TITLE: &str = "Quickslot";
pub const QUICKSLOT_AVAILABILITY_TITLE: &str = "Potion availability";
pub const QUICKSLOT_COOLDOWN_TITLE: &str = "Potion cooldown";
pub const QUICKSLOT_TOOLTIP: &str =
    "What the selected quickslot contains, read from the Pixel Beacon addon. Requires addon version 13 or later and distinguishes empty, non-potion, unavailable, and potion states.";
pub const QUICKSLOT_AVAILABILITY_TOOLTIP: &str =
    "Whether a selected potion has stock and ESO reports its slot usable. Depleted and Blocked are non-actionable.";
pub const QUICKSLOT_COOLDOWN_TOOLTIP: &str =
    "The selected potion's independent slot cooldown. Ready or remaining time never decides whether the slot contains a potion.";

// Auto-potion section.
pub const AUTO_POTION_TITLE: &str = "Auto-potion";
pub const AUTO_POTION_OFF: &str = "Off";
pub const AUTO_POTION_DORMANT_GAME: &str = "Dormant: game inactive";
pub const AUTO_POTION_DORMANT_UNFOCUSED: &str = "Dormant: game unfocused";
pub const AUTO_POTION_BLOCKED_BEACON: &str = "Blocked: beacon unavailable";
pub const AUTO_POTION_BLOCKED_SUSPENDED: &str = "Blocked: input suspended";
pub const AUTO_POTION_BLOCKED_CONTEXT: &str = "Blocked: game context";
pub const AUTO_POTION_BLOCKED_PLAYER_UNKNOWN: &str = "Blocked: life state unavailable";
pub const AUTO_POTION_BLOCKED_PLAYER_DEAD: &str = "Blocked: player dead";
pub const AUTO_POTION_BLOCKED_PLAYER_REINCARNATING: &str = "Blocked: reincarnating";
pub const AUTO_POTION_BLOCKED_NO_WATCH: &str = "Blocked: no watched resource";
pub const AUTO_POTION_BLOCKED_RESOURCES: &str = "Blocked: resources unavailable";
pub const AUTO_POTION_BLOCKED_QUICKSLOT: &str = "Blocked: quickslot unavailable";
pub const AUTO_POTION_BLOCKED_NO_POTION: &str = "Blocked: no potion selected";
pub const AUTO_POTION_BLOCKED_POTION: &str = "Blocked: potion unavailable";
pub const AUTO_POTION_BLOCKED_COOLDOWN: &str = "Blocked: potion cooldown";
pub const AUTO_POTION_BLOCKED_RETRY: &str = "Blocked: retry interval";
pub const AUTO_POTION_READY: &str = "Ready";
pub const AUTO_POTION_TOGGLE_LABEL: &str = "Auto-potion";
pub const AUTO_POTION_TOOLTIP: &str =
    "Shows whether auto-potion is Off, dormant, blocked, ready, or just triggered. The switch records your request; the state names the current runtime result.";
pub const AUTO_POTION_TOGGLE_TOOLTIP: &str =
    "Turn auto-potion on or off (F3). It also needs at least one resource enabled in Settings; with none enabled it never fires.";
pub const CLUSTER_AUTO_POTION: &str = "Auto-potion";
pub const SET_POTION_HEALTH: Setting = Setting {
    label: "Watch health (threshold %)",
    help: "Fire when health is at or below this percentage. The rule is an OR across the enabled resources, so any one of them being low is enough.",
};
pub const SET_POTION_MAGICKA: Setting = Setting {
    label: "Watch magicka (threshold %)",
    help: "Fire when magicka is at or below this percentage.",
};
pub const SET_POTION_STAMINA: Setting = Setting {
    label: "Watch stamina (threshold %)",
    help: "Fire when stamina is at or below this percentage.",
};
pub const SET_POTION_KEY: Setting = Setting {
    label: "Quickslot key",
    help: "The key pressed to drink. Defaults to Q, the game's default quickslot bind; change it here if you rebound it in game.",
};
pub const SET_POTION_RETRY: Setting = Setting {
    label: "Minimum retry interval (ms)",
    help: "The floor between two attempts. It covers the gap between pressing the key and the game reporting the resulting cooldown, which is at least one sampling interval. Raise it if potions are being spent too quickly.",
};

// Skills section.
pub const SKILLS_TITLE: &str = "Skills";
pub const SKILLS_TOOLTIP: &str =
    "Per-slot weave configuration: which slots are active, their weave type, and any delay override.";

/// The Skills grid columns as (header label, tooltip), left to right.
pub const SKILL_COLUMNS: [(&str, &str); 6] = [
    ("Skill", "The action slot this row configures."),
    ("Enabled", "Whether this slot takes part in the weave."),
    (
        "Weave",
        "The basic attack woven with this skill: light, heavy, bash, or block casting.",
    ),
    (
        "Override",
        "Use a custom delay for this slot instead of the global default for its weave type.",
    ),
    (
        "Delay (ms)",
        "The delay in milliseconds in effect for this slot: the override when set, otherwise the global default.",
    ),
    (
        "Cooldown",
        "How long this slot has left before it can be used again, read from the Pixel Beacon addon. Requires addon version 11 or later; shows a dash without a signal. Synergy always shows a dash because the game exposes no cooldown for it. Nothing in the application acts on this yet.",
    ),
];

// Live log.
pub const LOG_TITLE: &str = "Live Log";
pub const LOG_TOOLTIP: &str = "Recent application events. Drag the divider above to resize.";
pub const LOG_FILTER_TOOLTIP: &str =
    "Show only events at or above this level. Does not change what is captured.";

// Menu.
pub const MENU_FILE: &str = "File";
pub const MENU_VIEW: &str = "View";
pub const MENU_SETTINGS: &str = "Settings";
pub const MENU_SETTINGS_TOOLTIP: &str = "Open settings.";
pub const MENU_EXIT: &str = "Exit";
pub const MENU_LOG_TOGGLE: &str = "Live Log";
pub const MENU_LOG_TOGGLE_TOOLTIP: &str = "Show or hide the live log panel.";

// Save toast.
pub const SAVED_TOAST: &str = "Settings saved";

// Settings cluster titles.
pub const CLUSTER_APPEARANCE: &str = "Appearance";
pub const CLUSTER_COMBAT_TIMING: &str = "Combat timing";
pub const CLUSTER_FISHING: &str = "Fishing";
pub const CLUSTER_BEACON: &str = "Pixel Beacon and bus";
pub const CLUSTER_LOGGING: &str = "Logging";
pub const CLUSTER_KEYBINDINGS: &str = "Keybindings";

/// A single settings option's label and help text.
pub struct Setting {
    /// The human-readable label (no underscore).
    pub label: &'static str,
    /// The one-line inline help shown beneath the control.
    pub help: &'static str,
}

pub const SET_THEME: Setting = Setting {
    label: "Theme",
    help: "The color scheme of the window.",
};
pub const SET_ALWAYS_ON_TOP: Setting = Setting {
    label: "Always on top",
    help: "Keep the ESO Weave window above other windows.",
};
pub const SET_GLOBAL_COOLDOWN: Setting = Setting {
    label: "Global cooldown (ms)",
    help: "Minimum interval between weave executions.",
};
pub const SET_D_WEAVE: Setting = Setting {
    label: "Light attack delay (ms)",
    help: "Base gap between the basic attack and the skill key.",
};
pub const SET_D_HEAVY: Setting = Setting {
    label: "Heavy attack delay (ms)",
    help: "How long a heavy attack is held before the skill key.",
};
pub const SET_D_BASH: Setting = Setting {
    label: "Bash delay (ms)",
    help: "Gap before the bash action in a bash attack.",
};
pub const SET_AUTO_TIMING: Setting = Setting {
    label: "Auto timing from weapon",
    help: "Set each bar's heavy-attack delay automatically from the weapon equipped on that bar.",
};
pub const SET_LATENCY_ENABLED: Setting = Setting {
    label: "Adapt to latency",
    help: "Shorten delays automatically as measured latency rises.",
};
pub const SET_LATENCY_K: Setting = Setting {
    label: "Latency factor",
    help: "How strongly latency shortens the delays (higher adapts more).",
};
pub const SET_ARM_TIMEOUT: Setting = Setting {
    label: "Arm timeout (ms)",
    help: "How long to wait for a bite before recasting.",
};
pub const SET_REEL_DELAY: Setting = Setting {
    label: "Reel delay (ms)",
    help: "Delay between detecting a bite and reeling in.",
};
pub const SET_RECAST_DELAY: Setting = Setting {
    label: "Recast delay (ms)",
    help: "Delay before casting the line again after a catch or timeout.",
};
pub const SET_BEACON_PATH: Setting = Setting {
    label: "AddOns folder override",
    help: "Use this AddOns folder instead of the auto-detected one. Leave blank to auto-detect.",
};
pub const SET_BEACON_ENV: Setting = Setting {
    label: "Game environment",
    help: "Which ESO install to target when detecting the AddOns folder.",
};
pub const SET_BLOCK_PX: Setting = Setting {
    label: "Block size (px)",
    help: "Advanced: the physical-pixel size of each beacon square, and the only way to shrink the on-screen overlay (which cannot be moved). Changing it re-deploys PixelBeacon and takes effect after a /reloadui and an app restart.",
};
pub const SET_TOLERANCE: Setting = Setting {
    label: "Color tolerance",
    help: "How much a sampled pixel may differ from the expected color and still match.",
};
pub const SET_INTERVAL_FISHING: Setting = Setting {
    label: "Sample interval while fishing (ms)",
    help: "How often the pixel signal is read while a cast is active.",
};
pub const SET_INTERVAL_IDLE: Setting = Setting {
    label: "Sample interval while idle (ms)",
    help: "How often the pixel signal is read while idle.",
};
pub const SET_LOG_LEVEL: Setting = Setting {
    label: "Log level",
    help: "The lowest level of event that is captured.",
};
pub const SET_FILE_LOGGING: Setting = Setting {
    label: "Write log to file",
    help: "Also write captured events to a monthly log file.",
};

/// Every settings option, for coverage and hygiene tests.
pub const ALL_SETTINGS: [&Setting; 24] = [
    &SET_THEME,
    &SET_ALWAYS_ON_TOP,
    &SET_GLOBAL_COOLDOWN,
    &SET_D_WEAVE,
    &SET_D_HEAVY,
    &SET_D_BASH,
    &SET_AUTO_TIMING,
    &SET_LATENCY_ENABLED,
    &SET_LATENCY_K,
    &SET_ARM_TIMEOUT,
    &SET_REEL_DELAY,
    &SET_RECAST_DELAY,
    &SET_BEACON_PATH,
    &SET_BEACON_ENV,
    &SET_TOLERANCE,
    &SET_INTERVAL_FISHING,
    &SET_INTERVAL_IDLE,
    &SET_POTION_HEALTH,
    &SET_POTION_MAGICKA,
    &SET_POTION_STAMINA,
    &SET_POTION_KEY,
    &SET_POTION_RETRY,
    &SET_LOG_LEVEL,
    &SET_FILE_LOGGING,
];

/// Every user-facing label, for the no-underscore hygiene test.
pub fn all_labels() -> Vec<&'static str> {
    let mut labels = vec![
        STATUS_TITLE,
        FISHING_TITLE,
        BEACON_TITLE,
        GAME_INSTALLATION_TITLE,
        GAME_RUNTIME_TITLE,
        FISHING_CASTING,
        FISHING_WAITING,
        FISHING_REELING,
        FISHING_RECASTING,
        FISHING_IDLE,
        FISHING_IDLE_NO_CAST,
        FISHING_IDLE_SIGNAL_LOST,
        FISHING_IDLE_GAME_INACTIVE,
        FISHING_IDLE_UNFOCUSED,
        SUSPEND_LABEL,
        FISHING_TOGGLE_LABEL,
        WEAPON_BAR_TITLE,
        COMBAT_TITLE,
        MOVEMENT_TITLE,
        MENU_TITLE,
        HEALTH_TITLE,
        STAMINA_TITLE,
        MAGICKA_TITLE,
        AUTO_POTION_TITLE,
        AUTO_POTION_TOGGLE_LABEL,
        AUTO_POTION_OFF,
        AUTO_POTION_DORMANT_GAME,
        AUTO_POTION_DORMANT_UNFOCUSED,
        AUTO_POTION_BLOCKED_BEACON,
        AUTO_POTION_BLOCKED_SUSPENDED,
        AUTO_POTION_BLOCKED_CONTEXT,
        AUTO_POTION_BLOCKED_NO_WATCH,
        AUTO_POTION_BLOCKED_RESOURCES,
        AUTO_POTION_BLOCKED_QUICKSLOT,
        AUTO_POTION_BLOCKED_NO_POTION,
        AUTO_POTION_BLOCKED_POTION,
        AUTO_POTION_BLOCKED_COOLDOWN,
        AUTO_POTION_BLOCKED_RETRY,
        AUTO_POTION_READY,
        QUICKSLOT_TITLE,
        QUICKSLOT_AVAILABILITY_TITLE,
        QUICKSLOT_COOLDOWN_TITLE,
        SKILLS_TITLE,
        LOG_TITLE,
        MENU_FILE,
        MENU_VIEW,
        MENU_SETTINGS,
        MENU_EXIT,
        MENU_LOG_TOGGLE,
        SAVED_TOAST,
        CLUSTER_APPEARANCE,
        CLUSTER_COMBAT_TIMING,
        CLUSTER_FISHING,
        CLUSTER_BEACON,
        CLUSTER_AUTO_POTION,
        CLUSTER_LOGGING,
        CLUSTER_KEYBINDINGS,
    ];
    for (header, _) in SKILL_COLUMNS {
        labels.push(header);
    }
    for setting in ALL_SETTINGS {
        labels.push(setting.label);
    }
    labels
}

/// Every tooltip and help string, for the coverage (non-empty) test.
pub fn all_tooltips() -> Vec<&'static str> {
    let mut tips = vec![
        STATUS_TOOLTIP,
        FISHING_TOOLTIP,
        BEACON_TOOLTIP,
        GAME_INSTALLATION_TOOLTIP,
        GAME_RUNTIME_TOOLTIP,
        SUSPEND_TOOLTIP,
        FISHING_TOGGLE_TOOLTIP,
        BEACON_INSTALL_TOOLTIP,
        BEACON_UPDATE_TOOLTIP,
        BEACON_UNINSTALL_TOOLTIP,
        WEAPON_BAR_TOOLTIP,
        COMBAT_TOOLTIP,
        MOVEMENT_TOOLTIP,
        MENU_TOOLTIP,
        RESOURCE_TOOLTIP,
        QUICKSLOT_TOOLTIP,
        QUICKSLOT_AVAILABILITY_TOOLTIP,
        QUICKSLOT_COOLDOWN_TOOLTIP,
        AUTO_POTION_TOOLTIP,
        AUTO_POTION_TOGGLE_TOOLTIP,
        SKILLS_TOOLTIP,
        LOG_TOOLTIP,
        LOG_FILTER_TOOLTIP,
        MENU_SETTINGS_TOOLTIP,
        MENU_LOG_TOGGLE_TOOLTIP,
    ];
    for (_, tip) in SKILL_COLUMNS {
        tips.push(tip);
    }
    for setting in ALL_SETTINGS {
        tips.push(setting.help);
    }
    tips
}
