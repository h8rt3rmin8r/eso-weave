//! Windows provider, process, and focus observations.

use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use windows_sys::Win32::Foundation::{CloseHandle, ERROR_SUCCESS, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY, REG_SZ,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use super::{
    epic_candidate, valid_game_root, CandidateSource, FocusObservation, InstallationCandidate,
    InstallationProvider, Presence, ProcessObservation,
};

const GAME_NAMES: [&str; 2] = ["eso64.exe", "eso.exe"];
const LAUNCHER_NAME: &str = "bethesda.net_launcher.exe";

fn wide(value: &str) -> Vec<u16> {
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(Some(0))
        .collect()
}

fn registry_string(root: HKEY, subkey: &str, name: &str, view: u32) -> Result<Option<PathBuf>, ()> {
    unsafe {
        let mut key = std::ptr::null_mut();
        if RegOpenKeyExW(root, wide(subkey).as_ptr(), 0, KEY_READ | view, &mut key) != ERROR_SUCCESS
        {
            return Ok(None);
        }
        let mut kind = 0;
        let mut bytes = 0;
        let result = RegQueryValueExW(
            key,
            wide(name).as_ptr(),
            std::ptr::null_mut(),
            &mut kind,
            std::ptr::null_mut(),
            &mut bytes,
        );
        if result != ERROR_SUCCESS || kind != REG_SZ || bytes < 2 {
            RegCloseKey(key);
            return Ok(None);
        }
        let mut buffer = vec![0u16; bytes as usize / 2];
        let result = RegQueryValueExW(
            key,
            wide(name).as_ptr(),
            std::ptr::null_mut(),
            &mut kind,
            buffer.as_mut_ptr().cast(),
            &mut bytes,
        );
        RegCloseKey(key);
        if result != ERROR_SUCCESS {
            return Err(());
        }
        let len = buffer
            .iter()
            .position(|value| *value == 0)
            .unwrap_or(buffer.len());
        Ok(Some(PathBuf::from(OsString::from_wide(&buffer[..len]))))
    }
}

fn uninstall_candidate(
    key_name: &str,
    provider: InstallationProvider,
    source: CandidateSource,
) -> (Vec<InstallationCandidate>, bool) {
    let key = format!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{key_name}");
    let mut candidates = Vec::new();
    let mut failed = false;
    for root in [HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER] {
        for view in [KEY_WOW64_64KEY, KEY_WOW64_32KEY] {
            match registry_string(root, &key, "InstallLocation", view) {
                Ok(Some(path)) if valid_game_root(&path) => {
                    candidates.push(InstallationCandidate {
                        provider,
                        root: path,
                        source,
                    })
                }
                Ok(_) => {}
                Err(()) => failed = true,
            }
        }
    }
    (candidates, failed)
}

fn epic_candidates() -> (Vec<InstallationCandidate>, bool) {
    let Some(program_data) = std::env::var_os("PROGRAMDATA") else {
        return (Vec::new(), false);
    };
    let manifests = Path::new(&program_data)
        .join("Epic")
        .join("EpicGamesLauncher")
        .join("Data")
        .join("Manifests");
    if !manifests.exists() {
        return (Vec::new(), false);
    }
    let entries = match std::fs::read_dir(manifests) {
        Ok(entries) => entries,
        Err(_) => return (Vec::new(), true),
    };
    let mut candidates = Vec::new();
    let mut failed = false;
    for entry in entries.flatten() {
        if entry.path().extension().is_none_or(|ext| ext != "item") {
            continue;
        }
        match std::fs::read_to_string(entry.path())
            .ok()
            .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        {
            Some(value) => candidates.extend(epic_candidate(&value)),
            None => failed = true,
        }
    }
    (candidates, failed)
}

pub fn discover_installations() -> (Vec<InstallationCandidate>, bool) {
    let (mut candidates, mut failed) = uninstall_candidate(
        "Steam App 306130",
        InstallationProvider::Steam,
        CandidateSource::SteamUninstall,
    );
    let (generic, generic_failed) = uninstall_candidate(
        "The Elder Scrolls Online",
        InstallationProvider::EsoStore,
        CandidateSource::GenericUninstall,
    );
    let (epic, epic_failed) = epic_candidates();
    candidates.extend(generic);
    candidates.extend(epic);
    failed |= generic_failed | epic_failed;
    (candidates, failed)
}

fn exe_name(raw: &[u16]) -> String {
    let len = raw
        .iter()
        .position(|value| *value == 0)
        .unwrap_or(raw.len());
    String::from_utf16_lossy(&raw[..len]).to_ascii_lowercase()
}

pub fn observe_processes() -> ProcessObservation {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return ProcessObservation {
                game: Presence::Unknown,
                launcher: Presence::Unknown,
                focus: FocusObservation::Unknown,
            };
        }
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut game_pids = Vec::new();
        let mut launcher = false;
        let mut walked = false;
        let mut ok = Process32FirstW(snapshot, &mut entry);
        while ok != 0 {
            walked = true;
            let name = exe_name(&entry.szExeFile);
            if GAME_NAMES.contains(&name.as_str()) {
                game_pids.push(entry.th32ProcessID);
            } else if name == LAUNCHER_NAME {
                launcher = true;
            }
            ok = Process32NextW(snapshot, &mut entry);
        }
        CloseHandle(snapshot);
        if !walked {
            return ProcessObservation {
                game: Presence::Unknown,
                launcher: Presence::Unknown,
                focus: FocusObservation::Unknown,
            };
        }
        let focus = if game_pids.is_empty() {
            FocusObservation::Unknown
        } else {
            let hwnd = GetForegroundWindow();
            if hwnd.is_null() {
                FocusObservation::Unknown
            } else {
                let mut pid = 0;
                GetWindowThreadProcessId(hwnd, &mut pid);
                if game_pids.contains(&pid) {
                    FocusObservation::Focused
                } else {
                    FocusObservation::Unfocused
                }
            }
        };
        ProcessObservation {
            game: if game_pids.is_empty() {
                Presence::Absent
            } else {
                Presence::Present
            },
            launcher: if launcher {
                Presence::Present
            } else {
                Presence::Absent
            },
            focus,
        }
    }
}
