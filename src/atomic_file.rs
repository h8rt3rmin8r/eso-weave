//! Shared atomic replacement for synced temporary files.

use std::path::Path;

pub(crate) fn persist(candidate: tempfile::TempPath, destination: &Path) -> std::io::Result<()> {
    if !destination.exists() {
        candidate
            .persist(destination)
            .map_err(|error| error.error)?;
        return Ok(());
    }
    persist_existing(candidate, destination)
}

#[cfg(windows)]
fn persist_existing(candidate: tempfile::TempPath, destination: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{ReplaceFileW, REPLACEFILE_WRITE_THROUGH};

    let destination_wide = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let candidate_wide = candidate
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    // SAFETY: both owned buffers are NUL-terminated and live for the call.
    let replaced = unsafe {
        ReplaceFileW(
            destination_wide.as_ptr(),
            candidate_wide.as_ptr(),
            std::ptr::null(),
            REPLACEFILE_WRITE_THROUGH,
            std::ptr::null(),
            std::ptr::null(),
        )
    };
    if replaced == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(not(windows))]
fn persist_existing(candidate: tempfile::TempPath, destination: &Path) -> std::io::Result<()> {
    candidate
        .persist(destination)
        .map_err(|error| error.error)?;
    Ok(())
}
