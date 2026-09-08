//! Hygiene and coverage tests for the centralized UI strings.

use eso_weave::app::strings;

#[test]
fn no_user_facing_label_contains_an_underscore() {
    for label in strings::all_labels() {
        assert!(
            !label.contains('_'),
            "user-facing label contains an underscore: {label:?}"
        );
    }
}

#[test]
fn every_tooltip_and_help_string_is_non_empty() {
    for tip in strings::all_tooltips() {
        assert!(!tip.trim().is_empty(), "tooltip or help string is empty");
    }
}

#[test]
fn settings_labels_and_help_are_present() {
    assert_eq!(strings::ALL_SETTINGS.len(), 26);
    for setting in strings::ALL_SETTINGS {
        assert!(!setting.label.trim().is_empty());
        assert!(!setting.label.contains('_'));
        assert!(!setting.help.trim().is_empty());
    }
}

#[test]
fn s062_settings_copy_distinguishes_live_updates_from_staged_geometry() {
    assert_eq!(
        strings::FISHING_IDLE_SETTINGS_CHANGED,
        "Idle (settings changed)"
    );
    assert_eq!(strings::SET_FISHING_INTERACT_KEY.label, "Interact Key");
    assert!(strings::FISHING_SETTINGS_APPLICATION_HELP.contains("apply while ESO Weave is running"));
    assert!(strings::READER_SETTINGS_APPLICATION_HELP.contains("apply while ESO Weave is running"));

    let block = strings::SET_BLOCK_PX.help;
    assert!(block.contains("re-deploys PixelBeacon"));
    assert!(block.contains("/reloadui or relog"));
    assert!(block.contains("ESO Weave restart"));

    let shipped = strings::all_tooltips().join("\n");
    for obsolete in [
        "all settings apply immediately",
        "live reader fields require restart",
        "Fishing Interact Key is not exposed",
    ] {
        assert!(!shipped.contains(obsolete), "obsolete claim: {obsolete}");
    }
}

#[test]
fn skill_columns_have_headers_and_tooltips() {
    // Six since slice 037 added the Cooldown column. This count is pinned on
    // purpose: the skills grid is the widest content-sized block in the window, so
    // a column added without thinking changes the window's intrinsic width, which
    // is what `tests/app_ui_sizing.rs` then has to account for.
    assert_eq!(strings::SKILL_COLUMNS.len(), 6);
    for (header, tip) in strings::SKILL_COLUMNS {
        assert!(!header.is_empty());
        assert!(!header.contains('_'));
        assert!(!tip.trim().is_empty());
    }
}

#[test]
fn settings_cluster_titles_are_clean() {
    for title in [
        strings::CLUSTER_APPEARANCE,
        strings::CLUSTER_COMBAT_TIMING,
        strings::CLUSTER_FISHING,
        strings::CLUSTER_BEACON,
        strings::CLUSTER_LOGGING,
        strings::CLUSTER_KEYBINDINGS,
    ] {
        assert!(!title.trim().is_empty());
        assert!(!title.contains('_'));
    }
}

#[test]
fn beacon_settings_are_surfaced() {
    // The beacon location override and environment options must be present in the
    // settings surface (they were previously persisted but not shown).
    let labels: Vec<&str> = strings::ALL_SETTINGS.iter().map(|s| s.label).collect();
    assert!(labels.contains(&strings::SET_BEACON_PATH.label));
    assert!(labels.contains(&strings::SET_BEACON_ENV.label));
}

#[test]
fn dashboard_field_labels_use_the_required_concise_title_case() {
    assert_eq!(strings::WEAPON_BAR_TITLE, "Weapon Bar");
    assert_eq!(strings::LIFE_TITLE, "Life State");
    assert_eq!(strings::ROLL_DODGE_TITLE, "Roll Dodge");
    assert_eq!(strings::WORLD_TITLE, "World State");
    assert_eq!(strings::AUTO_POTION_TITLE, "Auto Potion");
    assert_eq!(strings::BEACON_TITLE, "PixelBeacon Status");
    assert_eq!(strings::BEACON_SIGNAL_TITLE, "PixelBeacon Signal");

    let superseded = [
        "Active weapon bar",
        "Life state",
        "Roll dodge",
        "World state",
        "Auto-potion",
        "PixelBeacon installation",
        "PixelBeacon signal",
    ];
    for label in strings::field_labels() {
        assert!(
            !superseded.contains(&label),
            "superseded field label remains registered: {label}"
        );
    }
}

#[test]
fn audited_field_and_settings_labels_match_the_title_case_registry() {
    for label in strings::field_labels() {
        assert!(!label.trim().is_empty());
        assert!(!label.contains('_'));
    }

    for expected in [
        "Game Installation",
        "Game State",
        "Potion Availability",
        "Potion Cooldown",
        "Combat Timing",
        "PixelBeacon and Bus",
        "Always on Top",
        "Global Cooldown (ms)",
        "Light Attack Delay (ms)",
        "Auto Timing from Weapon",
        "Adapt to Latency",
        "Sample Interval While Idle (ms)",
        "Interact Key",
        "Write Log to File",
        "Watch Health (Threshold %)",
        "Toggle Suspend",
        "Toggle Fishing",
        "Toggle Auto Potion",
    ] {
        assert!(
            strings::field_labels().contains(&expected),
            "audited title-case label is missing: {expected}"
        );
    }
}

#[test]
fn behavior_help_matches_latency_and_logging_runtime_contracts() {
    let latency = strings::SET_LATENCY_ENABLED.help;
    assert!(latency.to_ascii_lowercase().contains("add"));
    assert!(latency.contains("Light Attack"));
    assert!(latency.contains("Bash"));
    assert!(latency.contains("300 ms"));
    assert!(!latency.contains("Shorten"));

    let logging = strings::LOG_FILTER_TOOLTIP;
    assert!(logging.contains("Capture"));
    assert!(logging.contains("file logging"));
    assert!(logging.contains("saved"));
    assert!(!logging.contains("Does not change what is captured"));
}
