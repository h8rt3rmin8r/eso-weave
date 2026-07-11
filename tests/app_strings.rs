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
    for setting in strings::ALL_SETTINGS {
        assert!(!setting.label.trim().is_empty());
        assert!(!setting.label.contains('_'));
        assert!(!setting.help.trim().is_empty());
    }
}

#[test]
fn skill_columns_have_headers_and_tooltips() {
    assert_eq!(strings::SKILL_COLUMNS.len(), 5);
    for (header, tip) in strings::SKILL_COLUMNS {
        assert!(!header.is_empty());
        assert!(!header.contains('_'));
        assert!(!tip.trim().is_empty());
    }
}
