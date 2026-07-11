//! Centralized user-facing UI strings: section titles, control labels, tooltips,
//! and settings help text.
//!
//! Keeping them in one place lets the render layer stay thin, keeps wording for
//! the same concept consistent across the window and the settings modal, and lets
//! tests assert coverage (every control has a tooltip) and hygiene (no user-facing
//! label contains an underscore).

// Status region titles.
pub const STATUS_TITLE: &str = "Status";
pub const FISHING_TITLE: &str = "Fishing";
pub const BEACON_TITLE: &str = "Pixel Beacon (Addon)";

// Status region tooltips.
pub const STATUS_TOOLTIP: &str =
    "Whether the weave engine is running or suspended. Input is only ever sent while the game window is focused.";
pub const FISHING_TOOLTIP: &str =
    "Whether the fishing routine is active. It reads the Pixel Beacon signal to detect bites.";
pub const BEACON_TOOLTIP: &str =
    "Install state of the bundled PixelBeacon companion addon that renders the pixel signal.";

// Status region toggles.
pub const SUSPEND_LABEL: &str = "Running";
pub const SUSPEND_TOOLTIP: &str = "Suspend or resume the weave engine.";
pub const FISHING_TOGGLE_LABEL: &str = "Fishing";
pub const FISHING_TOGGLE_TOOLTIP: &str = "Start or stop the fishing routine.";
pub const BEACON_INSTALL_TOOLTIP: &str =
    "Install or update the PixelBeacon addon in your AddOns folder.";
pub const BEACON_UNINSTALL_TOOLTIP: &str =
    "Remove the PixelBeacon addon. Only a folder marked as managed by ESO Weave is deleted.";

// Skills section.
pub const SKILLS_TITLE: &str = "Skills";
pub const SKILLS_TOOLTIP: &str =
    "Per-slot weave configuration: which slots are active, their weave type, and any delay override.";

/// The Skills grid columns as (header label, tooltip), left to right.
pub const SKILL_COLUMNS: [(&str, &str); 5] = [
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
        "Delay",
        "The delay in milliseconds in effect for this slot: the override when set, otherwise the global default.",
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
pub const ALL_SETTINGS: [&Setting; 18] = [
    &SET_THEME,
    &SET_ALWAYS_ON_TOP,
    &SET_GLOBAL_COOLDOWN,
    &SET_D_WEAVE,
    &SET_D_HEAVY,
    &SET_D_BASH,
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
    &SET_LOG_LEVEL,
    &SET_FILE_LOGGING,
];

/// Every user-facing label, for the no-underscore hygiene test.
pub fn all_labels() -> Vec<&'static str> {
    let mut labels = vec![
        STATUS_TITLE,
        FISHING_TITLE,
        BEACON_TITLE,
        SUSPEND_LABEL,
        FISHING_TOGGLE_LABEL,
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
        SUSPEND_TOOLTIP,
        FISHING_TOGGLE_TOOLTIP,
        BEACON_INSTALL_TOOLTIP,
        BEACON_UNINSTALL_TOOLTIP,
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
