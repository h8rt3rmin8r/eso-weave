//! The actions the engine can classify and hand off, and their default bindings.

use crate::input::key::Key;

/// A named operation the engine can be triggered to perform. Execution belongs to
/// later slices; this slice only classifies and hands off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action {
    /// Skill slot 1.
    Skill1,
    /// Skill slot 2.
    Skill2,
    /// Skill slot 3.
    Skill3,
    /// Skill slot 4.
    Skill4,
    /// Skill slot 5.
    Skill5,
    /// The ultimate ability.
    Ultimate,
    /// The synergy prompt.
    Synergy,
    /// Toggle the suspend state (suspend-exempt).
    ToggleSuspend,
    /// Toggle fishing (suspend-exempt).
    ToggleFishing,
}

impl Action {
    /// Every action, in a stable order.
    pub const ALL: [Action; 9] = [
        Action::Skill1,
        Action::Skill2,
        Action::Skill3,
        Action::Skill4,
        Action::Skill5,
        Action::Ultimate,
        Action::Synergy,
        Action::ToggleSuspend,
        Action::ToggleFishing,
    ];

    /// The canonical string used as the settings key for this action.
    pub fn as_str(self) -> &'static str {
        match self {
            Action::Skill1 => "skill1",
            Action::Skill2 => "skill2",
            Action::Skill3 => "skill3",
            Action::Skill4 => "skill4",
            Action::Skill5 => "skill5",
            Action::Ultimate => "ultimate",
            Action::Synergy => "synergy",
            Action::ToggleSuspend => "toggle_suspend",
            Action::ToggleFishing => "toggle_fishing",
        }
    }

    /// Parses a canonical action string, returning `None` for an unknown action.
    pub fn parse(value: &str) -> Option<Action> {
        Action::ALL.into_iter().find(|a| a.as_str() == value)
    }

    /// Whether this action remains active while the engine is suspended.
    pub fn suspend_exempt(self) -> bool {
        matches!(self, Action::ToggleSuspend | Action::ToggleFishing)
    }

    /// Whether this action is an application-level toggle (suspend or fishing)
    /// rather than a weave action. Toggle actions are routed to the GUI intent
    /// path instead of the weave worker, so a hotkey and its button reach one
    /// shared state.
    pub fn is_app_toggle(self) -> bool {
        matches!(self, Action::ToggleSuspend | Action::ToggleFishing)
    }

    /// The default physical key for this action (master specification section 6.4).
    pub fn default_key(self) -> Key {
        match self {
            Action::Skill1 => Key::Digit1,
            Action::Skill2 => Key::Digit2,
            Action::Skill3 => Key::Digit3,
            Action::Skill4 => Key::Digit4,
            Action::Skill5 => Key::Digit5,
            Action::Ultimate => Key::R,
            Action::Synergy => Key::X,
            Action::ToggleSuspend => Key::F1,
            Action::ToggleFishing => Key::F2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Action;

    #[test]
    fn is_app_toggle_is_true_only_for_the_two_toggles() {
        for action in Action::ALL {
            let expected = matches!(action, Action::ToggleSuspend | Action::ToggleFishing);
            assert_eq!(
                action.is_app_toggle(),
                expected,
                "{action:?} toggle classification"
            );
        }
    }

    #[test]
    fn skill_actions_are_not_app_toggles() {
        for action in [
            Action::Skill1,
            Action::Skill2,
            Action::Skill3,
            Action::Skill4,
            Action::Skill5,
            Action::Ultimate,
            Action::Synergy,
        ] {
            assert!(!action.is_app_toggle(), "{action:?} must be a weave action");
        }
    }
}
