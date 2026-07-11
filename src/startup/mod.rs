//! Startup failure surfacing.
//!
//! Once the release binary is built for the Windows subsystem it has no console,
//! so an unhandled panic during startup would otherwise be completely invisible.
//! This module installs a panic hook that always logs the failure and, while the
//! GUI event loop has not yet started, also shows a native dialog. The dialog is
//! suppressed once the GUI is running so that a long-lived worker-thread panic
//! does not pop a message box mid-session; such panics are still logged.
//!
//! The notification path is behind the [`Notifier`] trait so the gating logic is
//! unit-testable with a mock, without opening a real dialog.

use std::panic::PanicHookInfo;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// A sink for a user-visible startup-failure notice.
pub trait Notifier: Send + Sync {
    /// Presents the failure to the user (a native dialog in the real impl).
    fn notify(&self, title: &str, body: &str);
}

/// The title shown on the startup-failure notice.
const FAILURE_TITLE: &str = "ESO Weave failed to start";

/// Formats a panic payload and optional location into a single stable message.
///
/// The output is always non-empty; the location is appended only when present.
pub fn panic_message(payload: &str, location: Option<String>) -> String {
    let payload = if payload.is_empty() {
        "unknown panic"
    } else {
        payload
    };
    match location {
        Some(loc) => format!("{payload} (at {loc})"),
        None => payload.to_string(),
    }
}

/// Logs a startup panic and, when the GUI has not yet started, notifies the user.
///
/// This is the testable core of the panic hook: it takes the already-formatted
/// message, the current gui-started state, and the notifier, so tests can drive
/// it directly without installing a global hook.
fn handle_panic(message: &str, gui_started: bool, notifier: &dyn Notifier) {
    tracing::error!(target: "eso_weave", "startup panic: {message}");
    if !gui_started {
        notifier.notify(FAILURE_TITLE, message);
    }
}

/// Extracts a human-readable payload string from a panic.
fn payload_string(info: &PanicHookInfo<'_>) -> String {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s.clone()
    } else {
        String::new()
    }
}

/// Installs the process-wide panic hook.
///
/// Call once, early in `main`, before spawning worker threads. The returned flag
/// is shared with the hook: set it to `true` immediately before entering the GUI
/// event loop so that later panics are logged but do not raise a dialog.
pub fn install_hook(notifier: Box<dyn Notifier>) -> Arc<AtomicBool> {
    let gui_started = Arc::new(AtomicBool::new(false));
    let hook_flag = gui_started.clone();
    std::panic::set_hook(Box::new(move |info| {
        let message = panic_message(
            &payload_string(info),
            info.location().map(|l| l.to_string()),
        );
        handle_panic(
            &message,
            hook_flag.load(Ordering::SeqCst),
            notifier.as_ref(),
        );
    }));
    gui_started
}

/// The real notifier: a native message box on Windows, a stderr line elsewhere.
pub struct DialogNotifier;

#[cfg(windows)]
impl Notifier for DialogNotifier {
    fn notify(&self, title: &str, body: &str) {
        use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};

        let wide = |s: &str| -> Vec<u16> { s.encode_utf16().chain(std::iter::once(0)).collect() };
        let text = wide(body);
        let caption = wide(title);
        // SAFETY: both strings are null-terminated wide buffers that outlive the
        // call, and a null owner window is valid for a top-level message box.
        unsafe {
            MessageBoxW(
                std::ptr::null_mut(),
                text.as_ptr(),
                caption.as_ptr(),
                MB_OK | MB_ICONERROR,
            );
        }
    }
}

#[cfg(not(windows))]
impl Notifier for DialogNotifier {
    fn notify(&self, title: &str, body: &str) {
        eprintln!("{title}: {body}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MockNotifier {
        calls: Mutex<Vec<(String, String)>>,
    }

    impl Notifier for MockNotifier {
        fn notify(&self, title: &str, body: &str) {
            self.calls
                .lock()
                .unwrap()
                .push((title.to_string(), body.to_string()));
        }
    }

    #[test]
    fn panic_message_includes_payload_and_location() {
        let msg = panic_message("boom", Some("src/main.rs:10:5".to_string()));
        assert_eq!(msg, "boom (at src/main.rs:10:5)");
    }

    #[test]
    fn panic_message_omits_location_when_absent() {
        let msg = panic_message("boom", None);
        assert_eq!(msg, "boom");
    }

    #[test]
    fn panic_message_is_never_empty() {
        let msg = panic_message("", None);
        assert!(!msg.is_empty());
        assert_eq!(msg, "unknown panic");
    }

    #[test]
    fn notifies_when_gui_not_started() {
        let mock = MockNotifier::default();
        handle_panic("boom", false, &mock);
        let calls = mock.calls.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, FAILURE_TITLE);
        assert_eq!(calls[0].1, "boom");
    }

    #[test]
    fn does_not_notify_when_gui_started() {
        let mock = MockNotifier::default();
        handle_panic("boom", true, &mock);
        assert!(mock.calls.lock().unwrap().is_empty());
    }
}
