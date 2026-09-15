//! Linux backend: evdev grabs of one physical keyboard and pointer for
//! interception, plus one uinput virtual device for synthesis and pass-through.
//!
//! Recursion breaking is structural: the backend reads only the grabbed physical
//! devices and writes only the separate virtual device, so it never reads its own
//! output. Focus comes from the X11 active window (X11 and XWayland). On a
//! pure-Wayland session where the active window cannot be read, focus is not
//! confirmed and interception stays off, a documented limitation aligned with the
//! master specification's Wayland best-effort posture.
//!
//! This is a thin adapter over the OS. The safety-critical decision is made by
//! [`InputEngine::classify_native`].

use std::io;
use std::os::fd::AsRawFd;
use std::sync::{Arc, Mutex};

use evdev::uinput::VirtualDevice;
use evdev::{
    AttributeSet, AttributeSetRef, Device, EventType, InputEvent, KeyCode as EvKey,
    RelativeAxisCode,
};

use crate::input::{
    Decision, InputBackend, InputEngine, InputError, Key, KeyboardControl, MouseButton,
    MouseControl, NativeControl, NativeInput, NativeInputEvent, NativeModifier, Origin, Transition,
};

/// The default ESO window title fragment used for focus matching.
pub const DEFAULT_WINDOW_TITLE: &str = "Elder Scrolls Online";

#[derive(Default)]
struct VirtualDeviceState {
    device: Option<VirtualDevice>,
    physical_capabilities_applied: bool,
}

/// The Linux interception and synthesis backend.
pub struct LinuxBackend {
    window_title: String,
    virtual_device: Mutex<VirtualDeviceState>,
}

impl Default for LinuxBackend {
    fn default() -> Self {
        Self {
            window_title: DEFAULT_WINDOW_TITLE.to_string(),
            virtual_device: Mutex::new(VirtualDeviceState::default()),
        }
    }
}

impl LinuxBackend {
    /// Creates a backend matching the given window title fragment.
    pub fn new(window_title: impl Into<String>) -> Self {
        Self {
            window_title: window_title.into(),
            virtual_device: Mutex::new(VirtualDeviceState::default()),
        }
    }

    fn ensure_virtual_device(
        &self,
        physical_keys: Option<&AttributeSetRef<EvKey>>,
        physical_axes: Option<&AttributeSetRef<RelativeAxisCode>>,
    ) -> Result<(), InputError> {
        let mut guard = self.virtual_device.lock().unwrap();
        let physical_capabilities_requested = physical_keys.is_some();
        if virtual_device_needs_rebuild(
            guard.device.is_some(),
            guard.physical_capabilities_applied,
            physical_capabilities_requested,
        ) {
            let keys = advertised_keys(physical_keys);
            let axes = advertised_axes(physical_axes);
            let device = VirtualDevice::builder()
                .map_err(|e| InputError::Start(format!("uinput unavailable: {e}")))?
                .name("eso-weave")
                .with_keys(&keys)
                .map_err(|e| InputError::Start(format!("uinput key setup failed: {e}")))?
                .with_relative_axes(&axes)
                .map_err(|e| InputError::Start(format!("uinput axis setup failed: {e}")))?
                .build()
                .map_err(|e| InputError::Start(format!("uinput build failed: {e}")))?;
            guard.device = Some(device);
            guard.physical_capabilities_applied = physical_capabilities_requested;
        }
        Ok(())
    }
}

fn virtual_device_needs_rebuild(
    device_exists: bool,
    physical_capabilities_applied: bool,
    physical_capabilities_requested: bool,
) -> bool {
    !device_exists || (physical_capabilities_requested && !physical_capabilities_applied)
}

impl InputBackend for LinuxBackend {
    fn synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError> {
        self.ensure_virtual_device(None, None)?;
        let value = match transition {
            Transition::Down => 1,
            Transition::Up => 0,
        };
        let event = InputEvent::new(EventType::KEY.0, to_ev_key(key).code(), value);
        let mut guard = self.virtual_device.lock().unwrap();
        let device = guard
            .device
            .as_mut()
            .ok_or_else(|| InputError::Synth("virtual device missing".to_string()))?;
        device
            .emit(&[event])
            .map_err(|e| InputError::Synth(format!("emit failed: {e}")))
    }

    fn synthesize_mouse(
        &self,
        button: MouseButton,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.ensure_virtual_device(None, None)?;
        let code = match button {
            MouseButton::Primary => EvKey::BTN_LEFT,
            MouseButton::Secondary => EvKey::BTN_RIGHT,
        };
        let value = match transition {
            Transition::Down => 1,
            Transition::Up => 0,
        };
        let event = InputEvent::new(EventType::KEY.0, code.code(), value);
        let mut guard = self.virtual_device.lock().unwrap();
        let device = guard
            .device
            .as_mut()
            .ok_or_else(|| InputError::Synth("virtual device missing".to_string()))?;
        device
            .emit(&[event])
            .map_err(|e| InputError::Synth(format!("mouse emit failed: {e}")))
    }

    fn synthesize_native(
        &self,
        control: NativeControl,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.ensure_virtual_device(None, None)?;
        let event = match control {
            NativeControl::Keyboard(key) => InputEvent::new(
                EventType::KEY.0,
                keyboard_to_ev(key).code(),
                transition_value(transition),
            ),
            NativeControl::Mouse(MouseControl::WheelUp) if transition == Transition::Down => {
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_WHEEL.0, 1)
            }
            NativeControl::Mouse(MouseControl::WheelDown) if transition == Transition::Down => {
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_WHEEL.0, -1)
            }
            NativeControl::Mouse(MouseControl::WheelUp | MouseControl::WheelDown) => return Ok(()),
            NativeControl::Mouse(button) => InputEvent::new(
                EventType::KEY.0,
                mouse_to_ev(button).code(),
                transition_value(transition),
            ),
        };
        self.emit_native(event)
    }

    fn synthesize_modifier(
        &self,
        modifier: NativeModifier,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.ensure_virtual_device(None, None)?;
        let key = match modifier {
            NativeModifier::Control => EvKey::KEY_LEFTCTRL,
            NativeModifier::Alt => EvKey::KEY_LEFTALT,
            NativeModifier::Shift => EvKey::KEY_LEFTSHIFT,
            NativeModifier::Command => EvKey::KEY_LEFTMETA,
        };
        self.emit_native(InputEvent::new(
            EventType::KEY.0,
            key.code(),
            transition_value(transition),
        ))
    }

    fn run(&self, engine: Arc<InputEngine>) -> Result<(), InputError> {
        let (mut keyboard, mut pointer) = open_devices()?;
        let physical_keys = keyboard
            .supported_keys()
            .ok_or_else(|| InputError::Start("keyboard reports no key capabilities".to_string()))?
            .iter()
            .chain(pointer.supported_keys().into_iter().flatten())
            .collect::<AttributeSet<EvKey>>();
        let physical_axes = pointer
            .supported_relative_axes()
            .map(|axes| axes.iter().collect::<AttributeSet<RelativeAxisCode>>())
            .unwrap_or_default();
        self.ensure_virtual_device(Some(&physical_keys), Some(&physical_axes))?;
        keyboard.grab().map_err(|e| {
            InputError::Start(format!(
                "could not grab keyboard (input group membership or a udev rule is required): {e}"
            ))
        })?;
        pointer.grab().map_err(|e| {
            InputError::Start(format!(
                "could not grab pointer (input group membership or a udev rule is required): {e}"
            ))
        })?;

        loop {
            let mut descriptors = [
                libc::pollfd {
                    fd: keyboard.as_raw_fd(),
                    events: libc::POLLIN,
                    revents: 0,
                },
                libc::pollfd {
                    fd: pointer.as_raw_fd(),
                    events: libc::POLLIN,
                    revents: 0,
                },
            ];
            let ready = unsafe { libc::poll(descriptors.as_mut_ptr(), descriptors.len() as _, -1) };
            if ready < 0 {
                return Err(InputError::Start(format!(
                    "polling input devices failed: {}",
                    io::Error::last_os_error()
                )));
            }
            if descriptors[0].revents & libc::POLLIN != 0 {
                self.process_device(&mut keyboard, &engine)?;
            }
            if descriptors[1].revents & libc::POLLIN != 0 {
                self.process_device(&mut pointer, &engine)?;
            }
        }
    }
}

impl LinuxBackend {
    fn emit_native(&self, event: InputEvent) -> Result<(), InputError> {
        let mut guard = self.virtual_device.lock().unwrap();
        guard
            .device
            .as_mut()
            .ok_or_else(|| InputError::Synth("virtual device missing".to_string()))?
            .emit(&[event])
            .map_err(|error| InputError::Synth(format!("native emit failed: {error}")))
    }

    fn process_device(&self, device: &mut Device, engine: &InputEngine) -> Result<(), InputError> {
        let events = device
            .fetch_events()
            .map_err(|error| InputError::Start(format!("reading input events failed: {error}")))?;
        for raw in events {
            let mut forward = true;
            let classified = if raw.event_type() == EventType::KEY {
                transition_of(raw.value()).and_then(|transition| {
                    ev_to_native_input(EvKey::new(raw.code())).map(|input| (input, transition))
                })
            } else if raw.event_type() == EventType::RELATIVE
                && raw.code() == RelativeAxisCode::REL_WHEEL.0
                && raw.value() != 0
            {
                Some((
                    NativeInput::Primary(NativeControl::Mouse(if raw.value() > 0 {
                        MouseControl::WheelUp
                    } else {
                        MouseControl::WheelDown
                    })),
                    Transition::Down,
                ))
            } else {
                None
            };
            if let Some((input, transition)) = classified {
                engine.set_focused(active_window_matches(&self.window_title));
                forward = engine.classify_native(NativeInputEvent {
                    input,
                    transition,
                    origin: Origin::Real,
                }) == Decision::Pass;
            }
            if forward {
                let mut guard = self.virtual_device.lock().unwrap();
                let virt = guard.device.as_mut().ok_or_else(|| {
                    InputError::Synth("virtual device missing during pass-through".to_string())
                })?;
                emit_forwarded_event(raw, |events| virt.emit(events))?;
            }
        }
        Ok(())
    }
}

fn emit_forwarded_event(
    event: InputEvent,
    emit: impl FnOnce(&[InputEvent]) -> io::Result<()>,
) -> Result<(), InputError> {
    if event.event_type() != EventType::KEY && event.event_type() != EventType::RELATIVE {
        return Ok(());
    }
    emit(&[event]).map_err(|error| InputError::Synth(format!("pass-through emit failed: {error}")))
}

fn application_keys() -> AttributeSet<EvKey> {
    KeyboardControl::ALL
        .into_iter()
        .map(keyboard_to_ev)
        .chain([
            EvKey::BTN_LEFT,
            EvKey::BTN_RIGHT,
            EvKey::BTN_MIDDLE,
            EvKey::BTN_SIDE,
            EvKey::BTN_EXTRA,
            EvKey::KEY_LEFTCTRL,
            EvKey::KEY_LEFTALT,
            EvKey::KEY_LEFTSHIFT,
            EvKey::KEY_LEFTMETA,
        ])
        .collect()
}

fn advertised_keys(physical_keys: Option<&AttributeSetRef<EvKey>>) -> AttributeSet<EvKey> {
    let mut keys = application_keys();
    if let Some(physical_keys) = physical_keys {
        for key in physical_keys {
            keys.insert(key);
        }
    }
    keys
}

fn advertised_axes(
    physical_axes: Option<&AttributeSetRef<RelativeAxisCode>>,
) -> AttributeSet<RelativeAxisCode> {
    let mut axes: AttributeSet<RelativeAxisCode> =
        [RelativeAxisCode::REL_WHEEL].into_iter().collect();
    if let Some(physical_axes) = physical_axes {
        for axis in physical_axes {
            axes.insert(axis);
        }
    }
    axes
}

fn transition_value(transition: Transition) -> i32 {
    match transition {
        Transition::Down => 1,
        Transition::Up => 0,
    }
}

fn transition_of(value: i32) -> Option<Transition> {
    match value {
        1 | 2 => Some(Transition::Down),
        0 => Some(Transition::Up),
        _ => None,
    }
}

fn open_devices() -> Result<(Device, Device), InputError> {
    let mut keyboard = None;
    let mut pointer = None;
    for (_path, device) in evdev::enumerate() {
        if device.name() == Some("eso-weave") {
            continue;
        }
        let is_keyboard = device
            .supported_keys()
            .is_some_and(|keys| keys.contains(EvKey::KEY_1) && keys.contains(EvKey::KEY_R));
        if is_keyboard {
            if keyboard.is_none() {
                keyboard = Some(device);
            }
            continue;
        }
        let is_pointer = device
            .supported_keys()
            .is_some_and(|keys| keys.contains(EvKey::BTN_LEFT))
            && device.supported_relative_axes().is_some_and(|axes| {
                axes.contains(RelativeAxisCode::REL_X) && axes.contains(RelativeAxisCode::REL_Y)
            });
        if is_pointer && pointer.is_none() {
            pointer = Some(device);
        }
    }
    match (keyboard, pointer) {
        (Some(keyboard), Some(pointer)) => Ok((keyboard, pointer)),
        (None, _) => Err(InputError::Start(
            "no keyboard device found to intercept".to_string(),
        )),
        (_, None) => Err(InputError::Start(
            "no pointer device found to intercept".to_string(),
        )),
    }
}

fn to_ev_key(key: Key) -> EvKey {
    match key {
        Key::Digit1 => EvKey::KEY_1,
        Key::Digit2 => EvKey::KEY_2,
        Key::Digit3 => EvKey::KEY_3,
        Key::Digit4 => EvKey::KEY_4,
        Key::Digit5 => EvKey::KEY_5,
        Key::E => EvKey::KEY_E,
        Key::R => EvKey::KEY_R,
        Key::X => EvKey::KEY_X,
        Key::Q => EvKey::KEY_Q,
        Key::Space => EvKey::KEY_SPACE,
        Key::F1 => EvKey::KEY_F1,
        Key::F2 => EvKey::KEY_F2,
        Key::F3 => EvKey::KEY_F3,
    }
}

fn from_ev_code(code: u16) -> Option<Key> {
    match EvKey::new(code) {
        EvKey::KEY_1 => Some(Key::Digit1),
        EvKey::KEY_2 => Some(Key::Digit2),
        EvKey::KEY_3 => Some(Key::Digit3),
        EvKey::KEY_4 => Some(Key::Digit4),
        EvKey::KEY_5 => Some(Key::Digit5),
        EvKey::KEY_E => Some(Key::E),
        EvKey::KEY_R => Some(Key::R),
        EvKey::KEY_X => Some(Key::X),
        EvKey::KEY_Q => Some(Key::Q),
        EvKey::KEY_SPACE => Some(Key::Space),
        EvKey::KEY_F1 => Some(Key::F1),
        EvKey::KEY_F2 => Some(Key::F2),
        EvKey::KEY_F3 => Some(Key::F3),
        _ => None,
    }
}

fn mouse_to_ev(mouse: MouseControl) -> EvKey {
    match mouse {
        MouseControl::Left => EvKey::BTN_LEFT,
        MouseControl::Right => EvKey::BTN_RIGHT,
        MouseControl::Middle => EvKey::BTN_MIDDLE,
        MouseControl::Button4 => EvKey::BTN_SIDE,
        MouseControl::Button5 => EvKey::BTN_EXTRA,
        MouseControl::WheelUp | MouseControl::WheelDown => {
            unreachable!("wheel controls use REL_WHEEL")
        }
    }
}

fn ev_to_native_input(key: EvKey) -> Option<NativeInput> {
    let modifier = match key {
        EvKey::KEY_LEFTCTRL | EvKey::KEY_RIGHTCTRL => Some(NativeModifier::Control),
        EvKey::KEY_LEFTALT | EvKey::KEY_RIGHTALT => Some(NativeModifier::Alt),
        EvKey::KEY_LEFTSHIFT | EvKey::KEY_RIGHTSHIFT => Some(NativeModifier::Shift),
        EvKey::KEY_LEFTMETA | EvKey::KEY_RIGHTMETA => Some(NativeModifier::Command),
        _ => None,
    };
    if let Some(modifier) = modifier {
        return Some(NativeInput::Modifier(modifier));
    }
    let mouse = match key {
        EvKey::BTN_LEFT => Some(MouseControl::Left),
        EvKey::BTN_RIGHT => Some(MouseControl::Right),
        EvKey::BTN_MIDDLE => Some(MouseControl::Middle),
        EvKey::BTN_SIDE => Some(MouseControl::Button4),
        EvKey::BTN_EXTRA => Some(MouseControl::Button5),
        _ => None,
    };
    if let Some(mouse) = mouse {
        return Some(NativeInput::Primary(NativeControl::Mouse(mouse)));
    }
    ev_to_keyboard(key).map(|key| NativeInput::Primary(NativeControl::Keyboard(key)))
}

fn keyboard_to_ev(key: KeyboardControl) -> EvKey {
    match key {
        KeyboardControl::Digit0 => EvKey::KEY_0,
        KeyboardControl::Digit1 => EvKey::KEY_1,
        KeyboardControl::Digit2 => EvKey::KEY_2,
        KeyboardControl::Digit3 => EvKey::KEY_3,
        KeyboardControl::Digit4 => EvKey::KEY_4,
        KeyboardControl::Digit5 => EvKey::KEY_5,
        KeyboardControl::Digit6 => EvKey::KEY_6,
        KeyboardControl::Digit7 => EvKey::KEY_7,
        KeyboardControl::Digit8 => EvKey::KEY_8,
        KeyboardControl::Digit9 => EvKey::KEY_9,
        KeyboardControl::A => EvKey::KEY_A,
        KeyboardControl::B => EvKey::KEY_B,
        KeyboardControl::C => EvKey::KEY_C,
        KeyboardControl::D => EvKey::KEY_D,
        KeyboardControl::E => EvKey::KEY_E,
        KeyboardControl::F => EvKey::KEY_F,
        KeyboardControl::G => EvKey::KEY_G,
        KeyboardControl::H => EvKey::KEY_H,
        KeyboardControl::I => EvKey::KEY_I,
        KeyboardControl::J => EvKey::KEY_J,
        KeyboardControl::K => EvKey::KEY_K,
        KeyboardControl::L => EvKey::KEY_L,
        KeyboardControl::M => EvKey::KEY_M,
        KeyboardControl::N => EvKey::KEY_N,
        KeyboardControl::O => EvKey::KEY_O,
        KeyboardControl::P => EvKey::KEY_P,
        KeyboardControl::Q => EvKey::KEY_Q,
        KeyboardControl::R => EvKey::KEY_R,
        KeyboardControl::S => EvKey::KEY_S,
        KeyboardControl::T => EvKey::KEY_T,
        KeyboardControl::U => EvKey::KEY_U,
        KeyboardControl::V => EvKey::KEY_V,
        KeyboardControl::W => EvKey::KEY_W,
        KeyboardControl::X => EvKey::KEY_X,
        KeyboardControl::Y => EvKey::KEY_Y,
        KeyboardControl::Z => EvKey::KEY_Z,
        KeyboardControl::F1 => EvKey::KEY_F1,
        KeyboardControl::F2 => EvKey::KEY_F2,
        KeyboardControl::F3 => EvKey::KEY_F3,
        KeyboardControl::F4 => EvKey::KEY_F4,
        KeyboardControl::F5 => EvKey::KEY_F5,
        KeyboardControl::F6 => EvKey::KEY_F6,
        KeyboardControl::F7 => EvKey::KEY_F7,
        KeyboardControl::F8 => EvKey::KEY_F8,
        KeyboardControl::F9 => EvKey::KEY_F9,
        KeyboardControl::F10 => EvKey::KEY_F10,
        KeyboardControl::F11 => EvKey::KEY_F11,
        KeyboardControl::F12 => EvKey::KEY_F12,
        KeyboardControl::F13 => EvKey::KEY_F13,
        KeyboardControl::F14 => EvKey::KEY_F14,
        KeyboardControl::F15 => EvKey::KEY_F15,
        KeyboardControl::F16 => EvKey::KEY_F16,
        KeyboardControl::F17 => EvKey::KEY_F17,
        KeyboardControl::F18 => EvKey::KEY_F18,
        KeyboardControl::F19 => EvKey::KEY_F19,
        KeyboardControl::F20 => EvKey::KEY_F20,
        KeyboardControl::F21 => EvKey::KEY_F21,
        KeyboardControl::F22 => EvKey::KEY_F22,
        KeyboardControl::F23 => EvKey::KEY_F23,
        KeyboardControl::F24 => EvKey::KEY_F24,
        KeyboardControl::Backspace => EvKey::KEY_BACKSPACE,
        KeyboardControl::CapsLock => EvKey::KEY_CAPSLOCK,
        KeyboardControl::Delete => EvKey::KEY_DELETE,
        KeyboardControl::DownArrow => EvKey::KEY_DOWN,
        KeyboardControl::End => EvKey::KEY_END,
        KeyboardControl::Enter => EvKey::KEY_ENTER,
        KeyboardControl::Escape => EvKey::KEY_ESC,
        KeyboardControl::Home => EvKey::KEY_HOME,
        KeyboardControl::Insert => EvKey::KEY_INSERT,
        KeyboardControl::LeftArrow => EvKey::KEY_LEFT,
        KeyboardControl::NumLock => EvKey::KEY_NUMLOCK,
        KeyboardControl::Numpad0 => EvKey::KEY_KP0,
        KeyboardControl::Numpad1 => EvKey::KEY_KP1,
        KeyboardControl::Numpad2 => EvKey::KEY_KP2,
        KeyboardControl::Numpad3 => EvKey::KEY_KP3,
        KeyboardControl::Numpad4 => EvKey::KEY_KP4,
        KeyboardControl::Numpad5 => EvKey::KEY_KP5,
        KeyboardControl::Numpad6 => EvKey::KEY_KP6,
        KeyboardControl::Numpad7 => EvKey::KEY_KP7,
        KeyboardControl::Numpad8 => EvKey::KEY_KP8,
        KeyboardControl::Numpad9 => EvKey::KEY_KP9,
        KeyboardControl::NumpadAdd => EvKey::KEY_KPPLUS,
        KeyboardControl::NumpadDot => EvKey::KEY_KPDOT,
        KeyboardControl::NumpadEnter => EvKey::KEY_KPENTER,
        KeyboardControl::NumpadMinus => EvKey::KEY_KPMINUS,
        KeyboardControl::NumpadSlash => EvKey::KEY_KPSLASH,
        KeyboardControl::NumpadStar => EvKey::KEY_KPASTERISK,
        KeyboardControl::PageDown => EvKey::KEY_PAGEDOWN,
        KeyboardControl::PageUp => EvKey::KEY_PAGEUP,
        KeyboardControl::Pause => EvKey::KEY_PAUSE,
        KeyboardControl::PrintScreen => EvKey::KEY_SYSRQ,
        KeyboardControl::RightArrow => EvKey::KEY_RIGHT,
        KeyboardControl::ScrollLock => EvKey::KEY_SCROLLLOCK,
        KeyboardControl::Space => EvKey::KEY_SPACE,
        KeyboardControl::Tab => EvKey::KEY_TAB,
        KeyboardControl::UpArrow => EvKey::KEY_UP,
        KeyboardControl::Oem102GermanLessThan => EvKey::KEY_102ND,
        KeyboardControl::Oem1Semicolon => EvKey::KEY_SEMICOLON,
        KeyboardControl::Oem2ForwardSlash => EvKey::KEY_SLASH,
        KeyboardControl::Oem3Tick => EvKey::KEY_GRAVE,
        KeyboardControl::Oem4LeftSquareBracket => EvKey::KEY_LEFTBRACE,
        KeyboardControl::Oem5BackSlash => EvKey::KEY_BACKSLASH,
        KeyboardControl::Oem6RightSquareBracket => EvKey::KEY_RIGHTBRACE,
        KeyboardControl::Oem7SingleQuote => EvKey::KEY_APOSTROPHE,
        KeyboardControl::Oem8BackTick => EvKey::KEY_RO,
        KeyboardControl::OemComma => EvKey::KEY_COMMA,
        KeyboardControl::OemMinus => EvKey::KEY_MINUS,
        KeyboardControl::OemPeriod => EvKey::KEY_DOT,
        KeyboardControl::OemPlus => EvKey::KEY_EQUAL,
        KeyboardControl::LeftWindows => EvKey::KEY_LEFTMETA,
        KeyboardControl::RightWindows => EvKey::KEY_RIGHTMETA,
    }
}

fn ev_to_keyboard(key: EvKey) -> Option<KeyboardControl> {
    KeyboardControl::ALL
        .into_iter()
        .find(|candidate| keyboard_to_ev(*candidate) == key)
}

fn active_window_matches(title: &str) -> bool {
    crate::platform::active_window_title().is_some_and(|name| name.contains(title))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_application_key_round_trips_and_is_advertised() {
        let capabilities = application_keys();
        for key in Key::ALL {
            let native = to_ev_key(key);
            assert!(
                capabilities.contains(native),
                "missing capability for {key:?}"
            );
            assert_eq!(from_ev_code(native.code()), Some(key));
        }
        assert!(capabilities.contains(EvKey::BTN_LEFT));
        assert!(capabilities.contains(EvKey::BTN_RIGHT));
    }

    #[test]
    fn every_native_keyboard_primary_is_advertised_and_round_trips() {
        let capabilities = application_keys();
        for key in KeyboardControl::ALL {
            let native = keyboard_to_ev(key);
            assert!(capabilities.contains(native), "missing {key:?}");
            if !matches!(
                key,
                KeyboardControl::LeftWindows | KeyboardControl::RightWindows
            ) {
                assert_eq!(
                    ev_to_native_input(native),
                    Some(NativeInput::Primary(NativeControl::Keyboard(key)))
                );
            }
        }
        assert!(advertised_axes(None).contains(RelativeAxisCode::REL_WHEEL));
    }

    #[test]
    fn every_shipped_action_and_fishing_default_is_advertised() {
        let capabilities = application_keys();
        for action in crate::input::Action::ALL {
            let key = action.default_key();
            assert!(
                capabilities.contains(to_ev_key(key)),
                "missing Linux capability for {action:?} default {key:?}"
            );
        }

        let fishing_key = crate::fishing::FishingConfig::default().interact_key;
        assert_eq!(fishing_key, Key::E);
        assert!(capabilities.contains(to_ev_key(fishing_key)));
        assert_eq!(
            crate::input::Action::ToggleAutoPotion.default_key(),
            Key::F3
        );
    }

    #[test]
    fn physical_only_keys_are_preserved_in_the_advertised_union() {
        let physical: AttributeSet<EvKey> = [EvKey::KEY_A, EvKey::KEY_Z].into_iter().collect();
        let capabilities = advertised_keys(Some(&physical));
        assert!(capabilities.contains(EvKey::KEY_A));
        assert!(capabilities.contains(EvKey::KEY_Z));
        assert!(capabilities.contains(EvKey::KEY_E));
        assert!(capabilities.contains(EvKey::KEY_F3));
    }

    #[test]
    fn an_early_application_only_device_is_rebuilt_before_grab() {
        assert!(virtual_device_needs_rebuild(false, false, false));
        assert!(!virtual_device_needs_rebuild(true, false, false));
        assert!(virtual_device_needs_rebuild(true, false, true));
        assert!(!virtual_device_needs_rebuild(true, true, true));
    }

    #[test]
    fn unknown_physical_keys_stay_outside_the_application_domain() {
        assert_eq!(from_ev_code(EvKey::KEY_A.code()), None);
    }

    #[test]
    fn forwarded_key_errors_are_explicit_and_metadata_is_ignored() {
        let key = InputEvent::new(EventType::KEY.0, EvKey::KEY_A.code(), 1);
        let error = emit_forwarded_event(key, |_| Err(io::Error::other("blocked"))).unwrap_err();
        assert!(error.to_string().contains("pass-through emit failed"));

        let metadata = InputEvent::new(EventType::MISC.0, 4, 30);
        emit_forwarded_event(metadata, |_| panic!("metadata must not be emitted")).unwrap();
    }
}
