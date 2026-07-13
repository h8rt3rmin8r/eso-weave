//! Platform-neutral key identifier.
//!
//! The core and the binding table use [`Key`]; each backend maps between a `Key`
//! and its native scan or key code.

use std::fmt;

/// A platform-neutral physical key identifier.
///
/// This covers the keys the default bindings need. Additional keys are added as
/// later slices require them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Key {
    /// The `1` key.
    Digit1,
    /// The `2` key.
    Digit2,
    /// The `3` key.
    Digit3,
    /// The `4` key.
    Digit4,
    /// The `5` key.
    Digit5,
    /// The `E` key.
    E,
    /// The `R` key.
    R,
    /// The `X` key.
    X,
    /// The `Q` key.
    Q,
    /// The space bar.
    Space,
    /// The `F1` key.
    F1,
    /// The `F2` key.
    F2,
}

impl Key {
    /// The canonical lowercase string used in settings.
    pub fn as_str(self) -> &'static str {
        match self {
            Key::Digit1 => "digit1",
            Key::Digit2 => "digit2",
            Key::Digit3 => "digit3",
            Key::Digit4 => "digit4",
            Key::Digit5 => "digit5",
            Key::E => "e",
            Key::R => "r",
            Key::X => "x",
            Key::Q => "q",
            Key::Space => "space",
            Key::F1 => "f1",
            Key::F2 => "f2",
        }
    }

    /// A friendly, human-readable name for the key, for display in the GUI. This
    /// is presentation only; the canonical `as_str` used for storage and `parse`
    /// are unchanged.
    pub fn display_name(self) -> &'static str {
        match self {
            Key::Digit1 => "Number 1",
            Key::Digit2 => "Number 2",
            Key::Digit3 => "Number 3",
            Key::Digit4 => "Number 4",
            Key::Digit5 => "Number 5",
            Key::E => "E",
            Key::R => "R",
            Key::X => "X",
            Key::Q => "Q",
            Key::Space => "Space",
            Key::F1 => "F1",
            Key::F2 => "F2",
        }
    }

    /// Parses a canonical key string, returning `None` for an unknown key.
    pub fn parse(value: &str) -> Option<Key> {
        match value {
            "digit1" => Some(Key::Digit1),
            "digit2" => Some(Key::Digit2),
            "digit3" => Some(Key::Digit3),
            "digit4" => Some(Key::Digit4),
            "digit5" => Some(Key::Digit5),
            "e" => Some(Key::E),
            "r" => Some(Key::R),
            "x" => Some(Key::X),
            "q" => Some(Key::Q),
            "space" => Some(Key::Space),
            "f1" => Some(Key::F1),
            "f2" => Some(Key::F2),
            _ => None,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Key; 12] = [
        Key::Digit1,
        Key::Digit2,
        Key::Digit3,
        Key::Digit4,
        Key::Digit5,
        Key::E,
        Key::R,
        Key::X,
        Key::Q,
        Key::Space,
        Key::F1,
        Key::F2,
    ];

    #[test]
    fn display_name_is_present_and_clean_for_every_key() {
        for key in ALL {
            let name = key.display_name();
            assert!(!name.trim().is_empty(), "display name empty for {key:?}");
            assert!(
                !name.contains('_'),
                "display name has an underscore for {key:?}"
            );
        }
    }

    #[test]
    fn display_name_maps_expected_values() {
        assert_eq!(Key::Digit1.display_name(), "Number 1");
        assert_eq!(Key::Digit5.display_name(), "Number 5");
        assert_eq!(Key::Space.display_name(), "Space");
        assert_eq!(Key::F1.display_name(), "F1");
        assert_eq!(Key::F2.display_name(), "F2");
        assert_eq!(Key::E.display_name(), "E");
    }

    #[test]
    fn canonical_round_trip_is_unchanged() {
        for key in ALL {
            assert_eq!(Key::parse(key.as_str()), Some(key));
        }
    }
}
