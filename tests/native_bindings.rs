//! Portable native ESO action-binding model contract tests.

use eso_weave::input::native::{
    KeyboardControl, ModifierSet, MouseControl, NativeAction, NativeBindingSet, NativeBindingState,
    NativeChord, NativeControl,
};

#[test]
fn action_order_is_the_fixed_pixel_contract() {
    assert_eq!(NativeAction::ALL.len(), 11);
    for (index, action) in NativeAction::ALL.into_iter().enumerate() {
        assert_eq!(action.index(), index);
    }
    assert_eq!(NativeAction::Skill1.eso_name(), "ACTION_BUTTON_3");
    assert_eq!(NativeAction::Skill5.eso_name(), "ACTION_BUTTON_7");
    assert_eq!(NativeAction::Ultimate.eso_name(), "ACTION_BUTTON_8");
    assert_eq!(NativeAction::Synergy.eso_name(), "USE_SYNERGY");
    assert_eq!(NativeAction::Attack.eso_name(), "SPECIAL_MOVE_ATTACK");
    assert_eq!(NativeAction::Block.eso_name(), "SPECIAL_MOVE_BLOCK");
    assert_eq!(NativeAction::Interact.eso_name(), "GAME_CAMERA_INTERACT");
    assert_eq!(NativeAction::Quickslot.eso_name(), "ACTION_BUTTON_9");
}

#[test]
fn every_portable_control_has_one_stable_wire_code() {
    let mut codes = std::collections::BTreeSet::new();
    for key in KeyboardControl::ALL {
        let control = NativeControl::Keyboard(key);
        assert!(control.wire_code() >= 0x10);
        assert_eq!(
            NativeControl::from_wire_code(control.wire_code()),
            Some(control)
        );
        assert!(codes.insert(control.wire_code()));
    }
    for button in MouseControl::ALL {
        let control = NativeControl::Mouse(button);
        assert_eq!(
            NativeControl::from_wire_code(control.wire_code()),
            Some(control)
        );
        assert!(codes.insert(control.wire_code()));
    }
    assert_eq!(
        codes.len(),
        KeyboardControl::ALL.len() + MouseControl::ALL.len()
    );
}

#[test]
fn reserved_and_unknown_codes_are_not_controls() {
    for code in 0x00..0x10 {
        assert_eq!(NativeControl::from_wire_code(code), None);
    }
    for code in 207..=u8::MAX {
        assert_eq!(NativeControl::from_wire_code(code), None);
    }
}

#[test]
fn every_modifier_combination_is_normalized() {
    for bits in 0..=0x0F {
        let modifiers = ModifierSet::from_bits(bits).expect("four-bit mask");
        assert_eq!(modifiers.bits(), bits);
        assert_eq!(modifiers.contains(ModifierSet::SHIFT), bits & 1 != 0);
        assert_eq!(modifiers.contains(ModifierSet::CONTROL), bits & 2 != 0);
        assert_eq!(modifiers.contains(ModifierSet::ALT), bits & 4 != 0);
        assert_eq!(modifiers.contains(ModifierSet::COMMAND), bits & 8 != 0);
    }
    assert_eq!(ModifierSet::from_bits(0x10), None);
}

#[test]
fn binding_set_defaults_unavailable_and_indexes_by_action() {
    let mut set = NativeBindingSet::new_unavailable();
    for action in NativeAction::ALL {
        assert_eq!(set.get(action), NativeBindingState::Unavailable);
    }

    let chord = NativeChord {
        primary: NativeControl::Mouse(MouseControl::Right),
        modifiers: ModifierSet::SHIFT | ModifierSet::CONTROL,
    };
    set.set(NativeAction::Block, NativeBindingState::Valid(chord));
    assert_eq!(
        set.get(NativeAction::Block),
        NativeBindingState::Valid(chord)
    );
    assert_eq!(
        set.get(NativeAction::Attack),
        NativeBindingState::Unavailable
    );
}
