//! Windows backend: a `WH_KEYBOARD_LL` hook for interception, `SendInput` for
//! synthesis, injected-input flagging for recursion breaking, and raised timer
//! resolution for the worker lifetime.
//!
//! This is a thin adapter over the OS. The safety-critical decision is made by
//! [`InputEngine::classify`]; this file only translates OS events into
//! [`KeyEvent`]s, feeds focus, and acts on the returned [`Decision`].

use std::sync::{Arc, OnceLock};

use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Media::{timeBeginPeriod, timeEndPeriod};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_KEYUP,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEINPUT,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetForegroundWindow, GetMessageW, GetWindowTextW,
    SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, HC_ACTION, KBDLLHOOKSTRUCT,
    LLKHF_INJECTED, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use crate::input::{
    Decision, InputBackend, InputEngine, InputError, Key, KeyEvent, MouseButton, Origin, Transition,
};

static ENGINE: OnceLock<Arc<InputEngine>> = OnceLock::new();
static TITLE: OnceLock<String> = OnceLock::new();

/// The default ESO window title fragment used for focus matching.
pub const DEFAULT_WINDOW_TITLE: &str = "Elder Scrolls Online";

/// The Windows interception and synthesis backend.
pub struct WindowsBackend {
    window_title: String,
}

impl Default for WindowsBackend {
    fn default() -> Self {
        Self {
            window_title: DEFAULT_WINDOW_TITLE.to_string(),
        }
    }
}

impl WindowsBackend {
    /// Creates a backend matching the given window title fragment.
    pub fn new(window_title: impl Into<String>) -> Self {
        Self {
            window_title: window_title.into(),
        }
    }
}

impl InputBackend for WindowsBackend {
    fn synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError> {
        let mut input: INPUT = unsafe { std::mem::zeroed() };
        input.r#type = INPUT_KEYBOARD;
        input.Anonymous.ki = KEYBDINPUT {
            wVk: key_to_vk(key),
            wScan: 0,
            dwFlags: if transition == Transition::Up {
                KEYEVENTF_KEYUP
            } else {
                0
            },
            time: 0,
            dwExtraInfo: 0,
        };
        let sent = unsafe { SendInput(1, &input, std::mem::size_of::<INPUT>() as i32) };
        if sent == 1 {
            Ok(())
        } else {
            Err(InputError::Synth(
                "SendInput did not dispatch the event".to_string(),
            ))
        }
    }

    fn synthesize_mouse(
        &self,
        button: MouseButton,
        transition: Transition,
    ) -> Result<(), InputError> {
        let flags = match (button, transition) {
            (MouseButton::Primary, Transition::Down) => MOUSEEVENTF_LEFTDOWN,
            (MouseButton::Primary, Transition::Up) => MOUSEEVENTF_LEFTUP,
            (MouseButton::Secondary, Transition::Down) => MOUSEEVENTF_RIGHTDOWN,
            (MouseButton::Secondary, Transition::Up) => MOUSEEVENTF_RIGHTUP,
        };
        let mut input: INPUT = unsafe { std::mem::zeroed() };
        input.r#type = INPUT_MOUSE;
        input.Anonymous.mi = MOUSEINPUT {
            dx: 0,
            dy: 0,
            mouseData: 0,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0,
        };
        let sent = unsafe { SendInput(1, &input, std::mem::size_of::<INPUT>() as i32) };
        if sent == 1 {
            Ok(())
        } else {
            Err(InputError::Synth(
                "SendInput did not dispatch the mouse event".to_string(),
            ))
        }
    }

    fn run(&self, engine: Arc<InputEngine>) -> Result<(), InputError> {
        let _ = ENGINE.set(engine);
        let _ = TITLE.set(self.window_title.clone());

        unsafe { timeBeginPeriod(1) };

        let hook =
            unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), std::ptr::null_mut(), 0) };
        if hook.is_null() {
            unsafe { timeEndPeriod(1) };
            return Err(InputError::Start(
                "SetWindowsHookExW failed to install the keyboard hook".to_string(),
            ));
        }

        let mut msg: MSG = unsafe { std::mem::zeroed() };
        unsafe {
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            UnhookWindowsHookEx(hook);
            timeEndPeriod(1);
        }
        Ok(())
    }
}

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let info = &*(lparam as *const KBDLLHOOKSTRUCT);
        if let (Some(key), Some(transition)) =
            (vk_to_key(info.vkCode), transition_of(wparam as u32))
        {
            if let Some(engine) = ENGINE.get() {
                let origin = if info.flags & LLKHF_INJECTED != 0 {
                    Origin::SelfOriginated
                } else {
                    Origin::Real
                };
                engine.set_focused(foreground_is_target());
                let decision = engine.classify(KeyEvent {
                    key,
                    transition,
                    origin,
                });
                if decision == Decision::Suppress {
                    return 1;
                }
            }
        }
    }
    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

fn transition_of(message: u32) -> Option<Transition> {
    match message {
        WM_KEYDOWN | WM_SYSKEYDOWN => Some(Transition::Down),
        WM_KEYUP | WM_SYSKEYUP => Some(Transition::Up),
        _ => None,
    }
}

fn foreground_is_target() -> bool {
    let Some(title) = TITLE.get() else {
        return false;
    };
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return false;
        }
        let mut buffer = [0u16; 256];
        let len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
        if len <= 0 {
            return false;
        }
        String::from_utf16_lossy(&buffer[..len as usize]).contains(title.as_str())
    }
}

fn vk_to_key(vk: u32) -> Option<Key> {
    match vk {
        0x31 => Some(Key::Digit1),
        0x32 => Some(Key::Digit2),
        0x33 => Some(Key::Digit3),
        0x34 => Some(Key::Digit4),
        0x35 => Some(Key::Digit5),
        0x52 => Some(Key::R),
        0x58 => Some(Key::X),
        0x51 => Some(Key::Q),
        0x20 => Some(Key::Space),
        0x70 => Some(Key::F1),
        0x71 => Some(Key::F2),
        _ => None,
    }
}

fn key_to_vk(key: Key) -> u16 {
    match key {
        Key::Digit1 => 0x31,
        Key::Digit2 => 0x32,
        Key::Digit3 => 0x33,
        Key::Digit4 => 0x34,
        Key::Digit5 => 0x35,
        Key::R => 0x52,
        Key::X => 0x58,
        Key::Q => 0x51,
        Key::Space => 0x20,
        Key::F1 => 0x70,
        Key::F2 => 0x71,
    }
}
