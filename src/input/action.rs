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
    /// Toggle auto-potion (suspend-exempt).
    ///
    /// Suspend-exempt like the two toggles above it, so the operator can reach it
    /// while suspended. That is separate from, and must not be confused with,
    /// whether the feature *acts* while suspended: it does not. See
    /// `specs/039-auto-potion/spec.md` FR-010 and FR-015.
    ToggleAutoPotion,
}

impl Action {
    /// Every action, in a stable order.
    pub const ALL: [Action; 10] = [
        Action::Skill1,
        Action::Skill2,
        Action::Skill3,
        Action::Skill4,
        Action::Skill5,
        Action::Ultimate,
        Action::Synergy,
        Action::ToggleSuspend,
        Action::ToggleFishing,
        Action::ToggleAutoPotion,
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
            Action::ToggleAutoPotion => "toggle_auto_potion",
        }
    }

    /// Parses a canonical action string, returning `None` for an unknown action.
    pub fn parse(value: &str) -> Option<Action> {
        Action::ALL.into_iter().find(|a| a.as_str() == value)
    }

    /// Whether this action remains active while the engine is suspended.
    ///
    /// A toggle the operator cannot reach while suspended is not a toggle, so all
    /// three application toggles are exempt. Note what this does NOT mean: the
    /// auto-potion feature itself does nothing while suspended, which is a
    /// separate condition checked in its controller.
    pub fn suspend_exempt(self) -> bool {
        matches!(
            self,
            Action::ToggleSuspend | Action::ToggleFishing | Action::ToggleAutoPotion
        )
    }

    /// Whether this action is an application-level toggle (suspend, fishing, or
    /// auto-potion) rather than a weave action. Toggle actions are routed to the
    /// GUI intent path instead of the weave worker, so a hotkey and its button
    /// reach one shared state. An action missing from here would be handed to the
    /// weave worker, which would try to run a weave sequence for it.
    pub fn is_app_toggle(self) -> bool {
        matches!(
            self,
            Action::ToggleSuspend | Action::ToggleFishing | Action::ToggleAutoPotion
        )
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
            Action::ToggleAutoPotion => Key::F3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Action;

    #[test]
    fn is_app_toggle_is_true_only_for_the_three_toggles() {
        for action in Action::ALL {
            let expected = matches!(
                action,
                Action::ToggleSuspend | Action::ToggleFishing | Action::ToggleAutoPotion
            );
            assert_eq!(
                action.is_app_toggle(),
                expected,
                "{action:?} toggle classification"
            );
        }
    }

    /// The two predicates must agree on exactly the same set. They are separate
    /// functions for separate reasons (one governs suspend, one governs routing),
    /// and a variant added to one and missed in the other is the specific mistake
    /// this asserts against: auto-potion missing from `suspend_exempt` would kill
    /// the hotkey exactly when the operator reaches for it, and missing from
    /// `is_app_toggle` would route it to the weave worker.
    #[test]
    fn the_toggle_predicates_agree_on_the_same_set() {
        for action in Action::ALL {
            assert_eq!(
                action.suspend_exempt(),
                action.is_app_toggle(),
                "{action:?} is classified inconsistently by the two toggle predicates"
            );
        }
    }

    #[test]
    fn every_toggle_has_a_distinct_default_function_key() {
        let toggles: Vec<_> = Action::ALL
            .into_iter()
            .filter(|a| a.is_app_toggle())
            .collect();
        assert_eq!(toggles.len(), 3);
        let mut keys: Vec<_> = toggles.iter().map(|a| a.default_key()).collect();
        keys.sort_by_key(|k| k.as_str());
        keys.dedup();
        assert_eq!(keys.len(), 3, "the toggles must not share a default key");
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
