use std::fs::{self, File, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};

#[cfg(windows)]
use std::ffi::OsString;

#[derive(Debug)]
pub(crate) enum StableReadError {
    Io(std::io::Error),
    Invalid(&'static str),
}

pub(crate) fn read_bounded_stable(
    canonical_root: &Path,
    path: &Path,
    max_bytes: u64,
) -> Result<Vec<u8>, StableReadError> {
    read_bounded_stable_with(canonical_root, path, max_bytes, || {})
}

fn read_bounded_stable_with(
    canonical_root: &Path,
    path: &Path,
    max_bytes: u64,
    after_read: impl FnOnce(),
) -> Result<Vec<u8>, StableReadError> {
    let mut file = open_bounded_stable(canonical_root, path, max_bytes)?;
    let metadata = file.metadata().map_err(StableReadError::Io)?;
    let original_length = metadata.len();
    let original_modified = metadata.modified().ok();
    let read_limit = max_bytes
        .checked_add(1)
        .ok_or(StableReadError::Invalid("file byte limit is invalid"))?;
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.by_ref()
        .take(read_limit)
        .read_to_end(&mut bytes)
        .map_err(StableReadError::Io)?;
    if bytes.len() as u64 > max_bytes {
        return Err(StableReadError::Invalid("file exceeds its byte limit"));
    }
    after_read();
    let final_metadata = file.metadata().map_err(StableReadError::Io)?;
    if final_metadata.len() != original_length
        || (original_modified.is_some() && final_metadata.modified().ok() != original_modified)
    {
        return Err(StableReadError::Invalid(
            "file changed during its stable read",
        ));
    }
    let path_file = open_no_follow(path).map_err(StableReadError::Io)?;
    let path_metadata = path_file.metadata().map_err(StableReadError::Io)?;
    if !path_metadata.is_file()
        || is_link_like(&path_metadata)
        || !same_file_identity(&file, &path_file).map_err(StableReadError::Io)?
        || path_metadata.len() != final_metadata.len()
        || (final_metadata.modified().is_ok()
            && path_metadata.modified().ok() != final_metadata.modified().ok())
    {
        return Err(StableReadError::Invalid(
            "file path changed during its stable read",
        ));
    }
    Ok(bytes)
}

#[cfg(unix)]
fn same_file_identity(left: &File, right: &File) -> std::io::Result<bool> {
    use std::os::unix::fs::MetadataExt;

    let left = left.metadata()?;
    let right = right.metadata()?;
    Ok(left.dev() == right.dev() && left.ino() == right.ino())
}

#[cfg(windows)]
fn same_file_identity(left: &File, right: &File) -> std::io::Result<bool> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
    };

    fn identity(file: &File) -> std::io::Result<(u32, u64)> {
        let mut information = BY_HANDLE_FILE_INFORMATION::default();
        // SAFETY: the file owns a valid handle for the duration of the call,
        // and information points to writable storage of the required type.
        let succeeded =
            unsafe { GetFileInformationByHandle(file.as_raw_handle() as HANDLE, &mut information) };
        if succeeded == 0 {
            return Err(std::io::Error::last_os_error());
        }
        let index =
            (u64::from(information.nFileIndexHigh) << 32) | u64::from(information.nFileIndexLow);
        Ok((information.dwVolumeSerialNumber, index))
    }

    Ok(identity(left)? == identity(right)?)
}

pub(crate) fn open_bounded_stable(
    canonical_root: &Path,
    path: &Path,
    max_bytes: u64,
) -> Result<File, StableReadError> {
    let file = open_no_follow(path).map_err(StableReadError::Io)?;
    let metadata = file.metadata().map_err(StableReadError::Io)?;
    if !metadata.is_file() || is_link_like(&metadata) {
        return Err(StableReadError::Invalid("file must not be link-like"));
    }
    if metadata.len() > max_bytes {
        return Err(StableReadError::Invalid("file exceeds its byte limit"));
    }
    let opened_path = opened_file_path(&file).map_err(StableReadError::Io)?;
    if !is_same_or_nested(&opened_path, canonical_root) {
        return Err(StableReadError::Invalid(
            "opened file escaped its approved root",
        ));
    }
    Ok(file)
}

pub(crate) fn is_same_or_nested(candidate: &Path, root: &Path) -> bool {
    #[cfg(windows)]
    {
        let candidate = candidate
            .components()
            .map(|value| value.as_os_str().to_string_lossy().to_lowercase())
            .collect::<Vec<_>>();
        let root = root
            .components()
            .map(|value| value.as_os_str().to_string_lossy().to_lowercase())
            .collect::<Vec<_>>();
        candidate.len() >= root.len() && candidate[..root.len()] == root
    }
    #[cfg(not(windows))]
    {
        candidate.starts_with(root)
    }
}

#[cfg(unix)]
pub(crate) fn is_link_like(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(windows)]
pub(crate) fn is_link_like(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(target_os = "linux")]
fn open_no_follow(path: &Path) -> std::io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;

    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
}

#[cfg(target_os = "linux")]
fn opened_file_path(file: &File) -> std::io::Result<PathBuf> {
    use std::os::fd::AsRawFd;

    fs::canonicalize(Path::new("/proc/self/fd").join(file.as_raw_fd().to_string()))
}

#[cfg(windows)]
fn open_no_follow(path: &Path) -> std::io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt;
    use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT;

    OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
}

#[cfg(windows)]
fn opened_file_path(file: &File) -> std::io::Result<PathBuf> {
    use std::os::windows::ffi::OsStringExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFinalPathNameByHandleW, FILE_NAME_NORMALIZED, VOLUME_NAME_DOS,
    };

    let mut buffer = vec![0_u16; 32_768];
    // SAFETY: the file owns a valid handle for the duration of the call, and
    // buffer is writable for the length passed to the Windows API.
    let written = unsafe {
        GetFinalPathNameByHandleW(
            file.as_raw_handle() as HANDLE,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            FILE_NAME_NORMALIZED | VOLUME_NAME_DOS,
        )
    };
    if written == 0 || written as usize >= buffer.len() {
        return Err(std::io::Error::last_os_error());
    }
    buffer.truncate(written as usize);
    Ok(PathBuf::from(OsString::from_wide(&buffer)))
}

#[cfg(not(any(target_os = "linux", windows)))]
fn open_no_follow(_path: &Path) -> std::io::Result<File> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "stable file handles support only Windows and Linux",
    ))
}

#[cfg(not(any(target_os = "linux", windows)))]
fn opened_file_path(_file: &File) -> std::io::Result<PathBuf> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "stable file handles support only Windows and Linux",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_read_rejects_path_replacement_after_reading() {
        let sandbox = tempfile::tempdir().unwrap();
        let selected = sandbox.path().join("selected.txt");
        let replacement = sandbox.path().join("replacement.txt");
        let displaced = sandbox.path().join("displaced.txt");
        fs::write(&selected, b"original").unwrap();
        fs::write(&replacement, b"replaced").unwrap();

        let canonical_root = fs::canonicalize(sandbox.path()).unwrap();
        let result = read_bounded_stable_with(&canonical_root, &selected, 64, || {
            fs::rename(&selected, &displaced).unwrap();
            fs::rename(&replacement, &selected).unwrap();
        });

        assert!(matches!(result, Err(StableReadError::Invalid(_))));
        assert_eq!(fs::read(&selected).unwrap(), b"replaced");
    }
}
