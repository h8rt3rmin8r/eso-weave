//! Windows backend: `WH_KEYBOARD_LL` and `WH_MOUSE_LL` hooks for interception,
//! `SendInput` for synthesis, injected-input flagging for recursion breaking,
//! and raised timer resolution for the worker lifetime.
//!
//! This is a thin adapter over the OS. The safety-critical decision is made by
//! [`InputEngine::classify_native`]; this file only translates OS events into
//! native input events, feeds focus, and acts on the returned [`Decision`].

use std::sync::{Arc, OnceLock};

use windows_sys::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::Media::{timeBeginPeriod, timeEndPeriod};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN,
    MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL,
    MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetForegroundWindow, GetMessageW, GetWindowTextW,
    SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, HC_ACTION, KBDLLHOOKSTRUCT,
    LLKHF_EXTENDED, LLKHF_INJECTED, LLMHF_INJECTED, MSG, MSLLHOOKSTRUCT, WH_KEYBOARD_LL,
    WH_MOUSE_LL, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_XBUTTONDOWN,
    WM_XBUTTONUP,
};

use crate::input::{
    Decision, InputBackend, InputEngine, InputError, Key, KeyboardControl, ModifierSide,
    MouseButton, MouseControl, NativeControl, NativeInput, NativeInputEvent, NativeModifier,
    Origin, Transition,
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
        send_keyboard(key_to_vk(key), false, transition)
    }

    fn synthesize_mouse(
        &self,
        button: MouseButton,
        transition: Transition,
    ) -> Result<(), InputError> {
        let control = match button {
            MouseButton::Primary => MouseControl::Left,
            MouseButton::Secondary => MouseControl::Right,
        };
        send_mouse(control, transition)
    }

    fn synthesize_native(
        &self,
        control: NativeControl,
        transition: Transition,
    ) -> Result<(), InputError> {
        match control {
            NativeControl::Keyboard(key) => {
                let (vk, extended) = keyboard_to_vk(key);
                send_keyboard(vk, extended, transition)
            }
            NativeControl::Mouse(button) => send_mouse(button, transition),
        }
    }

    fn synthesize_modifier(
        &self,
        modifier: NativeModifier,
        transition: Transition,
    ) -> Result<(), InputError> {
        let vk = match modifier {
            NativeModifier::Control => 0x11,
            NativeModifier::Alt => 0x12,
            NativeModifier::Shift => 0x10,
            NativeModifier::Command => 0x5B,
        };
        send_keyboard(vk, modifier == NativeModifier::Command, transition)
    }

    fn run(&self, engine: Arc<InputEngine>) -> Result<(), InputError> {
        let _ = ENGINE.set(engine);
        let _ = TITLE.set(self.window_title.clone());

        unsafe { timeBeginPeriod(1) };

        let keyboard =
            unsafe { SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), std::ptr::null_mut(), 0) };
        if keyboard.is_null() {
            unsafe { timeEndPeriod(1) };
            return Err(InputError::Start(
                "SetWindowsHookExW failed to install the keyboard hook".to_string(),
            ));
        }
        let mouse = unsafe {
            SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), std::ptr::null_mut(), 0)
        };
        if mouse.is_null() {
            unsafe {
                UnhookWindowsHookEx(keyboard);
                timeEndPeriod(1);
            }
            return Err(InputError::Start(
                "SetWindowsHookExW failed to install the mouse hook".to_string(),
            ));
        }

        let mut msg: MSG = unsafe { std::mem::zeroed() };
        unsafe {
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            UnhookWindowsHookEx(mouse);
            UnhookWindowsHookEx(keyboard);
            timeEndPeriod(1);
        }
        Ok(())
    }
}

fn send_keyboard(vk: u16, extended: bool, transition: Transition) -> Result<(), InputError> {
    let mut input: INPUT = unsafe { std::mem::zeroed() };
    input.r#type = INPUT_KEYBOARD;
    input.Anonymous.ki = KEYBDINPUT {
        wVk: vk,
        wScan: 0,
        dwFlags: (if transition == Transition::Up {
            KEYEVENTF_KEYUP
        } else {
            0
        }) | if extended { KEYEVENTF_EXTENDEDKEY } else { 0 },
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

fn send_mouse(button: MouseControl, transition: Transition) -> Result<(), InputError> {
    let (flags, data) = match (button, transition) {
        (MouseControl::Left, Transition::Down) => (MOUSEEVENTF_LEFTDOWN, 0),
        (MouseControl::Left, Transition::Up) => (MOUSEEVENTF_LEFTUP, 0),
        (MouseControl::Right, Transition::Down) => (MOUSEEVENTF_RIGHTDOWN, 0),
        (MouseControl::Right, Transition::Up) => (MOUSEEVENTF_RIGHTUP, 0),
        (MouseControl::Middle, Transition::Down) => (MOUSEEVENTF_MIDDLEDOWN, 0),
        (MouseControl::Middle, Transition::Up) => (MOUSEEVENTF_MIDDLEUP, 0),
        (MouseControl::Button4, Transition::Down) => (MOUSEEVENTF_XDOWN, 1),
        (MouseControl::Button4, Transition::Up) => (MOUSEEVENTF_XUP, 1),
        (MouseControl::Button5, Transition::Down) => (MOUSEEVENTF_XDOWN, 2),
        (MouseControl::Button5, Transition::Up) => (MOUSEEVENTF_XUP, 2),
        (MouseControl::WheelUp, Transition::Down) => (MOUSEEVENTF_WHEEL, 120),
        (MouseControl::WheelDown, Transition::Down) => (MOUSEEVENTF_WHEEL, (-120_i32) as u32),
        (MouseControl::WheelUp | MouseControl::WheelDown, Transition::Up) => return Ok(()),
    };
    let mut input: INPUT = unsafe { std::mem::zeroed() };
    input.r#type = INPUT_MOUSE;
    input.Anonymous.mi = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: data,
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

unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let info = &*(lparam as *const KBDLLHOOKSTRUCT);
        if let (Some(input), Some(transition)) = (
            vk_to_input(info.vkCode, info.flags & LLKHF_EXTENDED != 0),
            transition_of(wparam as u32),
        ) {
            if let Some(engine) = ENGINE.get() {
                let origin = if info.flags & LLKHF_INJECTED != 0 {
                    Origin::SelfOriginated
                } else {
                    Origin::Real
                };
                engine.set_focused(foreground_is_target());
                let decision = engine.classify_native(NativeInputEvent {
                    input,
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

unsafe extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        let info = &*(lparam as *const MSLLHOOKSTRUCT);
        if let Some((control, transition)) = mouse_transition(wparam as u32, info.mouseData) {
            if let Some(engine) = ENGINE.get() {
                engine.set_focused(foreground_is_target());
                let origin = if info.flags & LLMHF_INJECTED != 0 {
                    Origin::SelfOriginated
                } else {
                    Origin::Real
                };
                if engine.classify_native(NativeInputEvent {
                    input: NativeInput::Primary(NativeControl::Mouse(control)),
                    transition,
                    origin,
                }) == Decision::Suppress
                {
                    return 1;
                }
            }
        }
    }
    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

fn mouse_transition(message: u32, data: u32) -> Option<(MouseControl, Transition)> {
    let high = (data >> 16) as u16;
    match message {
        WM_LBUTTONDOWN => Some((MouseControl::Left, Transition::Down)),
        WM_LBUTTONUP => Some((MouseControl::Left, Transition::Up)),
        WM_RBUTTONDOWN => Some((MouseControl::Right, Transition::Down)),
        WM_RBUTTONUP => Some((MouseControl::Right, Transition::Up)),
        WM_MBUTTONDOWN => Some((MouseControl::Middle, Transition::Down)),
        WM_MBUTTONUP => Some((MouseControl::Middle, Transition::Up)),
        WM_XBUTTONDOWN if high == 1 => Some((MouseControl::Button4, Transition::Down)),
        WM_XBUTTONUP if high == 1 => Some((MouseControl::Button4, Transition::Up)),
        WM_XBUTTONDOWN if high == 2 => Some((MouseControl::Button5, Transition::Down)),
        WM_XBUTTONUP if high == 2 => Some((MouseControl::Button5, Transition::Up)),
        WM_MOUSEWHEEL if (high as i16) > 0 => Some((MouseControl::WheelUp, Transition::Down)),
        WM_MOUSEWHEEL if (high as i16) < 0 => Some((MouseControl::WheelDown, Transition::Down)),
        _ => None,
    }
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

fn vk_to_input(vk: u32, extended: bool) -> Option<NativeInput> {
    let modifier = match vk {
        0x10 | 0xA0 => Some((NativeModifier::Shift, ModifierSide::Left)),
        0xA1 => Some((NativeModifier::Shift, ModifierSide::Right)),
        0x11 => Some((
            NativeModifier::Control,
            if extended {
                ModifierSide::Right
            } else {
                ModifierSide::Left
            },
        )),
        0xA2 => Some((NativeModifier::Control, ModifierSide::Left)),
        0xA3 => Some((NativeModifier::Control, ModifierSide::Right)),
        0x12 => Some((
            NativeModifier::Alt,
            if extended {
                ModifierSide::Right
            } else {
                ModifierSide::Left
            },
        )),
        0xA4 => Some((NativeModifier::Alt, ModifierSide::Left)),
        0xA5 => Some((NativeModifier::Alt, ModifierSide::Right)),
        0x5B => {
            return Some(NativeInput::ModifierPrimary {
                modifier: NativeModifier::Command,
                side: ModifierSide::Left,
                primary: NativeControl::Keyboard(KeyboardControl::LeftWindows),
            });
        }
        0x5C => {
            return Some(NativeInput::ModifierPrimary {
                modifier: NativeModifier::Command,
                side: ModifierSide::Right,
                primary: NativeControl::Keyboard(KeyboardControl::RightWindows),
            });
        }
        _ => None,
    };
    if let Some((modifier, side)) = modifier {
        return Some(NativeInput::SidedModifier { modifier, side });
    }
    vk_to_keyboard(vk, extended).map(|key| NativeInput::Primary(NativeControl::Keyboard(key)))
}

fn vk_to_keyboard(vk: u32, extended: bool) -> Option<KeyboardControl> {
    if (0x30..=0x39).contains(&vk) {
        return keyboard_from_code(16 + (vk - 0x30) as u8);
    }
    if (0x41..=0x5A).contains(&vk) {
        return keyboard_from_code(26 + (vk - 0x41) as u8);
    }
    if (0x70..=0x87).contains(&vk) {
        return keyboard_from_code(52 + (vk - 0x70) as u8);
    }
    let code = match (vk, extended) {
        (0x0D, true) => 99,
        (0x08, _) => 76,
        (0x14, _) => 77,
        (0x2E, _) => 78,
        (0x28, _) => 79,
        (0x23, _) => 80,
        (0x0D, _) => 81,
        (0x1B, _) => 82,
        (0x24, _) => 83,
        (0x2D, _) => 84,
        (0x25, _) => 85,
        (0x90, _) => 86,
        (0x60..=0x69, _) => 87 + (vk - 0x60) as u8,
        (0x6B, _) => 97,
        (0x6E, _) => 98,
        (0x6D, _) => 100,
        (0x6F, _) => 101,
        (0x6A, _) => 102,
        (0x22, _) => 103,
        (0x21, _) => 104,
        (0x13, _) => 105,
        (0x2C, _) => 106,
        (0x27, _) => 107,
        (0x91, _) => 108,
        (0x20, _) => 109,
        (0x09, _) => 110,
        (0x26, _) => 111,
        (0xE2, _) => 112,
        (0xBA, _) => 113,
        (0xBF, _) => 114,
        (0xC0, _) => 115,
        (0xDB, _) => 116,
        (0xDC, _) => 117,
        (0xDD, _) => 118,
        (0xDE, _) => 119,
        (0xDF, _) => 120,
        (0xBC, _) => 121,
        (0xBD, _) => 122,
        (0xBE, _) => 123,
        (0xBB, _) => 124,
        (0x5B, _) => 125,
        (0x5C, _) => 126,
        _ => return None,
    };
    keyboard_from_code(code)
}

fn keyboard_from_code(code: u8) -> Option<KeyboardControl> {
    match NativeControl::from_wire_code(code) {
        Some(NativeControl::Keyboard(key)) => Some(key),
        _ => None,
    }
}

fn keyboard_to_vk(key: KeyboardControl) -> (u16, bool) {
    let code = key as u8;
    if (16..=25).contains(&code) {
        return (0x30 + u16::from(code - 16), false);
    }
    if (26..=51).contains(&code) {
        return (0x41 + u16::from(code - 26), false);
    }
    if (52..=75).contains(&code) {
        return (0x70 + u16::from(code - 52), false);
    }
    match key {
        KeyboardControl::Backspace => (0x08, false),
        KeyboardControl::CapsLock => (0x14, false),
        KeyboardControl::Delete => (0x2E, true),
        KeyboardControl::DownArrow => (0x28, true),
        KeyboardControl::End => (0x23, true),
        KeyboardControl::Enter => (0x0D, false),
        KeyboardControl::Escape => (0x1B, false),
        KeyboardControl::Home => (0x24, true),
        KeyboardControl::Insert => (0x2D, true),
        KeyboardControl::LeftArrow => (0x25, true),
        KeyboardControl::NumLock => (0x90, true),
        KeyboardControl::Numpad0 => (0x60, false),
        KeyboardControl::Numpad1 => (0x61, false),
        KeyboardControl::Numpad2 => (0x62, false),
        KeyboardControl::Numpad3 => (0x63, false),
        KeyboardControl::Numpad4 => (0x64, false),
        KeyboardControl::Numpad5 => (0x65, false),
        KeyboardControl::Numpad6 => (0x66, false),
        KeyboardControl::Numpad7 => (0x67, false),
        KeyboardControl::Numpad8 => (0x68, false),
        KeyboardControl::Numpad9 => (0x69, false),
        KeyboardControl::NumpadAdd => (0x6B, false),
        KeyboardControl::NumpadDot => (0x6E, false),
        KeyboardControl::NumpadEnter => (0x0D, true),
        KeyboardControl::NumpadMinus => (0x6D, false),
        KeyboardControl::NumpadSlash => (0x6F, true),
        KeyboardControl::NumpadStar => (0x6A, false),
        KeyboardControl::PageDown => (0x22, true),
        KeyboardControl::PageUp => (0x21, true),
        KeyboardControl::Pause => (0x13, false),
        KeyboardControl::PrintScreen => (0x2C, true),
        KeyboardControl::RightArrow => (0x27, true),
        KeyboardControl::ScrollLock => (0x91, false),
        KeyboardControl::Space => (0x20, false),
        KeyboardControl::Tab => (0x09, false),
        KeyboardControl::UpArrow => (0x26, true),
        KeyboardControl::Oem102GermanLessThan => (0xE2, false),
        KeyboardControl::Oem1Semicolon => (0xBA, false),
        KeyboardControl::Oem2ForwardSlash => (0xBF, false),
        KeyboardControl::Oem3Tick => (0xC0, false),
        KeyboardControl::Oem4LeftSquareBracket => (0xDB, false),
        KeyboardControl::Oem5BackSlash => (0xDC, false),
        KeyboardControl::Oem6RightSquareBracket => (0xDD, false),
        KeyboardControl::Oem7SingleQuote => (0xDE, false),
        KeyboardControl::Oem8BackTick => (0xDF, false),
        KeyboardControl::OemComma => (0xBC, false),
        KeyboardControl::OemMinus => (0xBD, false),
        KeyboardControl::OemPeriod => (0xBE, false),
        KeyboardControl::OemPlus => (0xBB, false),
        KeyboardControl::LeftWindows => (0x5B, true),
        KeyboardControl::RightWindows => (0x5C, true),
        _ => unreachable!("contiguous keyboard ranges handled above"),
    }
}

fn key_to_vk(key: Key) -> u16 {
    match key {
        Key::Digit1 => 0x31,
        Key::Digit2 => 0x32,
        Key::Digit3 => 0x33,
        Key::Digit4 => 0x34,
        Key::Digit5 => 0x35,
        Key::E => 0x45,
        Key::R => 0x52,
        Key::X => 0x58,
        Key::Q => 0x51,
        Key::Space => 0x20,
        Key::F1 => 0x70,
        Key::F2 => 0x71,
        Key::F3 => 0x72,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_native_keyboard_primary_round_trips() {
        for key in KeyboardControl::ALL {
            let (vk, extended) = keyboard_to_vk(key);
            assert_eq!(
                vk_to_keyboard(u32::from(vk), extended),
                Some(key),
                "{key:?}"
            );
        }
    }

    #[test]
    fn physical_modifiers_preserve_side_and_windows_primary_identity() {
        assert_eq!(
            vk_to_input(0xA0, false),
            Some(NativeInput::SidedModifier {
                modifier: NativeModifier::Shift,
                side: ModifierSide::Left,
            })
        );
        assert_eq!(
            vk_to_input(0x11, true),
            Some(NativeInput::SidedModifier {
                modifier: NativeModifier::Control,
                side: ModifierSide::Right,
            })
        );
        assert_eq!(
            vk_to_input(0x5B, true),
            Some(NativeInput::ModifierPrimary {
                modifier: NativeModifier::Command,
                side: ModifierSide::Left,
                primary: NativeControl::Keyboard(KeyboardControl::LeftWindows),
            })
        );
    }

    #[test]
    fn every_mouse_primary_has_a_hook_and_synthesis_shape() {
        assert_eq!(MouseControl::ALL.len(), 7);
        assert_eq!(
            mouse_transition(WM_LBUTTONDOWN, 0),
            Some((MouseControl::Left, Transition::Down))
        );
        assert_eq!(
            mouse_transition(WM_XBUTTONDOWN, 1_u32 << 16),
            Some((MouseControl::Button4, Transition::Down))
        );
        assert_eq!(
            mouse_transition(WM_MOUSEWHEEL, (120_u32) << 16),
            Some((MouseControl::WheelUp, Transition::Down))
        );
        assert_eq!(
            mouse_transition(WM_MOUSEWHEEL, ((-120_i16) as u16 as u32) << 16),
            Some((MouseControl::WheelDown, Transition::Down))
        );
    }
}
