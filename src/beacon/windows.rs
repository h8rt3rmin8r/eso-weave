//! Windows AddOns discovery and the ESO running-game probe.
//!
//! Discovery resolves the Documents known folder through the shell API (via the
//! `dirs` crate), never a literal path. The probe is a read-only process
//! snapshot and returns [`RunningState::Unknown`] on any failure.

use std::path::PathBuf;

use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

use super::{addons_dir_under_documents, Environment, RunningState};

/// Resolves the AddOns directory under the Documents known folder.
pub fn addons_dir(env: Environment) -> Option<PathBuf> {
    dirs::document_dir().map(|documents| addons_dir_under_documents(&documents, env))
}

/// The ESO client executable names to match, lowercased.
const ESO_EXE_NAMES: [&str; 2] = ["eso64.exe", "eso.exe"];

/// Probes running processes for the ESO client.
pub fn probe_game_running() -> RunningState {
    // SAFETY: standard ToolHelp process-snapshot walk. The snapshot handle is
    // closed on every return path, and PROCESSENTRY32W is zero-initialized with
    // its dwSize set as the API requires.
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return RunningState::Unknown;
        }

        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        let mut walked_any = false;
        let mut found = false;
        let mut ok = Process32FirstW(snapshot, &mut entry);
        while ok != 0 {
            walked_any = true;
            let name = exe_name(&entry.szExeFile).to_ascii_lowercase();
            if ESO_EXE_NAMES.contains(&name.as_str()) {
                found = true;
                break;
            }
            ok = Process32NextW(snapshot, &mut entry);
        }

        CloseHandle(snapshot);

        if found {
            RunningState::Running
        } else if walked_any {
            RunningState::NotRunning
        } else {
            RunningState::Unknown
        }
    }
}

/// Reads a null-terminated wide executable name into a `String`.
fn exe_name(raw: &[u16]) -> String {
    let len = raw.iter().position(|&c| c == 0).unwrap_or(raw.len());
    String::from_utf16_lossy(&raw[..len])
}
