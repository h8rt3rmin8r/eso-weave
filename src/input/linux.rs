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

use std::sync::{Arc, Mutex};

use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, Device, EventType, InputEvent, Key as EvKey};

use crate::input::{
    Decision, InputBackend, InputEngine, InputError, Key, KeyEvent, Origin, Transition,
};

/// The default ESO window title fragment used for focus matching.
pub const DEFAULT_WINDOW_TITLE: &str = "Elder Scrolls Online";

const ALL_EV_KEYS: [EvKey; 11] = [
    EvKey::KEY_1,
    EvKey::KEY_2,
    EvKey::KEY_3,
    EvKey::KEY_4,
    EvKey::KEY_5,
    EvKey::KEY_R,
    EvKey::KEY_X,
    EvKey::KEY_Q,
    EvKey::KEY_SPACE,
    EvKey::KEY_F1,
    EvKey::KEY_F2,
];

/// The Linux interception and synthesis backend.
pub struct LinuxBackend {
    window_title: String,
    virtual_device: Mutex<Option<VirtualDevice>>,
}

impl Default for LinuxBackend {
    fn default() -> Self {
        Self {
            window_title: DEFAULT_WINDOW_TITLE.to_string(),
            virtual_device: Mutex::new(None),
        }
    }
}

impl LinuxBackend {
    /// Creates a backend matching the given window title fragment.
    pub fn new(window_title: impl Into<String>) -> Self {
        Self {
            window_title: window_title.into(),
            virtual_device: Mutex::new(None),
        }
    }

    fn ensure_virtual_device(&self) -> Result<(), InputError> {
        let mut guard = self.virtual_device.lock().unwrap();
        if guard.is_none() {
            let mut keys = AttributeSet::<EvKey>::new();
            for key in ALL_EV_KEYS {
                keys.insert(key);
            }
            let device = VirtualDeviceBuilder::new()
                .map_err(|e| InputError::Start(format!("uinput unavailable: {e}")))?
                .name("eso-weave")
                .with_keys(&keys)
                .map_err(|e| InputError::Start(format!("uinput key setup failed: {e}")))?
                .build()
                .map_err(|e| InputError::Start(format!("uinput build failed: {e}")))?;
            *guard = Some(device);
        }
        Ok(())
    }
}

impl InputBackend for LinuxBackend {
    fn synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError> {
        self.ensure_virtual_device()?;
        let value = match transition {
            Transition::Down => 1,
            Transition::Up => 0,
        };
        let event = InputEvent::new(EventType::KEY, to_ev_key(key).code(), value);
        let mut guard = self.virtual_device.lock().unwrap();
        let device = guard
            .as_mut()
            .ok_or_else(|| InputError::Synth("virtual device missing".to_string()))?;
        device
            .emit(&[event])
            .map_err(|e| InputError::Synth(format!("emit failed: {e}")))
    }

    fn run(&self, engine: Arc<InputEngine>) -> Result<(), InputError> {
        self.ensure_virtual_device()?;
        let mut device = open_keyboard()?;
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
                    if let Some(virt) = guard.as_mut() {
                        let _ = virt.emit(&[raw]);
                    }
                }
            }
        }
    }
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
        Key::R => EvKey::KEY_R,
        Key::X => EvKey::KEY_X,
        Key::Q => EvKey::KEY_Q,
        Key::Space => EvKey::KEY_SPACE,
        Key::F1 => EvKey::KEY_F1,
        Key::F2 => EvKey::KEY_F2,
    }
}

fn from_ev_code(code: u16) -> Option<Key> {
    match EvKey::new(code) {
        EvKey::KEY_1 => Some(Key::Digit1),
        EvKey::KEY_2 => Some(Key::Digit2),
        EvKey::KEY_3 => Some(Key::Digit3),
        EvKey::KEY_4 => Some(Key::Digit4),
        EvKey::KEY_5 => Some(Key::Digit5),
        EvKey::KEY_R => Some(Key::R),
        EvKey::KEY_X => Some(Key::X),
        EvKey::KEY_Q => Some(Key::Q),
        EvKey::KEY_SPACE => Some(Key::Space),
        EvKey::KEY_F1 => Some(Key::F1),
        EvKey::KEY_F2 => Some(Key::F2),
        _ => None,
    }
}

fn active_window_matches(title: &str) -> bool {
    x11_active_title().is_some_and(|name| name.contains(title))
}

fn x11_active_title() -> Option<String> {
    use x11rb::connection::Connection;
    use x11rb::protocol::xproto::{AtomEnum, ConnectionExt};

    let (conn, screen_num) = x11rb::connect(None).ok()?;
    let root = conn.setup().roots[screen_num].root;

    let active_atom = conn
        .intern_atom(false, b"_NET_ACTIVE_WINDOW")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let active = conn
        .get_property(false, root, active_atom, AtomEnum::WINDOW, 0, 1)
        .ok()?
        .reply()
        .ok()?;
    let window = active.value32()?.next()?;

    let name_atom = conn
        .intern_atom(false, b"_NET_WM_NAME")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let utf8_atom = conn
        .intern_atom(false, b"UTF8_STRING")
        .ok()?
        .reply()
        .ok()?
        .atom;
    let name = conn
        .get_property(false, window, name_atom, utf8_atom, 0, 1024)
        .ok()?
        .reply()
        .ok()?;
    if !name.value.is_empty() {
        return Some(String::from_utf8_lossy(&name.value).into_owned());
    }

    let wm_name = conn
        .get_property(false, window, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 1024)
        .ok()?
        .reply()
        .ok()?;
    Some(String::from_utf8_lossy(&wm_name.value).into_owned())
}
