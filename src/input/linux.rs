//! Linux backend: an evdev grab of the physical keyboard for interception and a
//! uinput virtual device for synthesis and pass-through.
//!
//! Recursion breaking is structural: the backend reads only the grabbed physical
//! device and writes only the separate virtual device, so it never reads its own
//! output. Focus comes from the X11 active window (X11 and XWayland). On a
//! pure-Wayland session where the active window cannot be read, focus is not
//! confirmed and interception stays off, a documented limitation aligned with the
//! master specification's Wayland best-effort posture.
//!
//! This is a thin adapter over the OS. The safety-critical decision is made by
//! [`InputEngine::classify`].

use std::io;
use std::sync::{Arc, Mutex};

use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, AttributeSetRef, Device, EventType, InputEvent, Key as EvKey};

use crate::input::{
    Decision, InputBackend, InputEngine, InputError, Key, KeyEvent, MouseButton, Origin, Transition,
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
    ) -> Result<(), InputError> {
        let mut guard = self.virtual_device.lock().unwrap();
        let physical_capabilities_requested = physical_keys.is_some();
        if virtual_device_needs_rebuild(
            guard.device.is_some(),
            guard.physical_capabilities_applied,
            physical_capabilities_requested,
        ) {
            let keys = advertised_keys(physical_keys);
            let device = VirtualDeviceBuilder::new()
                .map_err(|e| InputError::Start(format!("uinput unavailable: {e}")))?
                .name("eso-weave")
                .with_keys(&keys)
                .map_err(|e| InputError::Start(format!("uinput key setup failed: {e}")))?
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
        self.ensure_virtual_device(None)?;
        let value = match transition {
            Transition::Down => 1,
            Transition::Up => 0,
        };
        let event = InputEvent::new(EventType::KEY, to_ev_key(key).code(), value);
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
        self.ensure_virtual_device(None)?;
        let code = match button {
            MouseButton::Primary => EvKey::BTN_LEFT,
            MouseButton::Secondary => EvKey::BTN_RIGHT,
        };
        let value = match transition {
            Transition::Down => 1,
            Transition::Up => 0,
        };
        let event = InputEvent::new(EventType::KEY, code.code(), value);
        let mut guard = self.virtual_device.lock().unwrap();
        let device = guard
            .device
            .as_mut()
            .ok_or_else(|| InputError::Synth("virtual device missing".to_string()))?;
        device
            .emit(&[event])
            .map_err(|e| InputError::Synth(format!("mouse emit failed: {e}")))
    }

    fn run(&self, engine: Arc<InputEngine>) -> Result<(), InputError> {
        let mut device = open_keyboard()?;
        let physical_keys = device
            .supported_keys()
            .ok_or_else(|| InputError::Start("keyboard reports no key capabilities".to_string()))?
            .iter()
            .collect::<AttributeSet<EvKey>>();
        self.ensure_virtual_device(Some(&physical_keys))?;
        device.grab().map_err(|e| {
            InputError::Start(format!(
                "could not grab keyboard (input group membership or a udev rule is required): {e}"
            ))
        })?;

        loop {
            let events = device
                .fetch_events()
                .map_err(|e| InputError::Start(format!("reading key events failed: {e}")))?;
            for raw in events {
                let mut forward = true;
                if raw.event_type() == EventType::KEY {
                    if let (Some(key), Some(transition)) =
                        (from_ev_code(raw.code()), transition_of(raw.value()))
                    {
                        engine.set_focused(active_window_matches(&self.window_title));
                        let decision = engine.classify(KeyEvent {
                            key,
                            transition,
                            origin: Origin::Real,
                        });
                        if decision == Decision::Suppress {
                            forward = false;
                        }
                    }
                }
                if forward {
                    let mut guard = self.virtual_device.lock().unwrap();
                    let virt = guard.device.as_mut().ok_or_else(|| {
                        InputError::Synth("virtual device missing during pass-through".to_string())
                    })?;
                    emit_forwarded_key(raw, |events| virt.emit(events))?;
                }
            }
        }
    }
}

fn emit_forwarded_key(
    event: InputEvent,
    emit: impl FnOnce(&[InputEvent]) -> io::Result<()>,
) -> Result<(), InputError> {
    if event.event_type() != EventType::KEY {
        return Ok(());
    }
    emit(&[event]).map_err(|error| InputError::Synth(format!("pass-through emit failed: {error}")))
}

fn application_keys() -> AttributeSet<EvKey> {
    Key::ALL
        .into_iter()
        .map(to_ev_key)
        .chain([EvKey::BTN_LEFT, EvKey::BTN_RIGHT])
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

fn transition_of(value: i32) -> Option<Transition> {
    match value {
        1 | 2 => Some(Transition::Down),
        0 => Some(Transition::Up),
        _ => None,
    }
}

fn open_keyboard() -> Result<Device, InputError> {
    for (_path, device) in evdev::enumerate() {
        let is_keyboard = device
            .supported_keys()
            .is_some_and(|keys| keys.contains(EvKey::KEY_1) && keys.contains(EvKey::KEY_R));
        if is_keyboard {
            return Ok(device);
        }
    }
    Err(InputError::Start(
        "no keyboard device found to intercept".to_string(),
    ))
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
        let key = InputEvent::new(EventType::KEY, EvKey::KEY_A.code(), 1);
        let error = emit_forwarded_key(key, |_| Err(io::Error::other("blocked"))).unwrap_err();
        assert!(error.to_string().contains("pass-through emit failed"));

        let metadata = InputEvent::new(EventType::MISC, 4, 30);
        emit_forwarded_key(metadata, |_| panic!("metadata must not be emitted")).unwrap();
    }
}

fn active_window_matches(title: &str) -> bool {
    crate::platform::active_window_title().is_some_and(|name| name.contains(title))
}
