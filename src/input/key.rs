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
            Key::R => "r",
            Key::X => "x",
            Key::Q => "q",
            Key::Space => "space",
            Key::F1 => "f1",
            Key::F2 => "f2",
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
