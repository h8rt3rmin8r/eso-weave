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
    let mut file = open_bounded_stable(canonical_root, path, max_bytes)?;
    let metadata = file.metadata().map_err(StableReadError::Io)?;
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
    Ok(bytes)
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
