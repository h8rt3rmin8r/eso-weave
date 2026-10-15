use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
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
    read_bounded_stable_snapshot(canonical_root, path, max_bytes).map(|(bytes, _)| bytes)
}

pub(crate) fn read_bounded_stable_snapshot(
    canonical_root: &Path,
    path: &Path,
    max_bytes: u64,
) -> Result<(Vec<u8>, Option<std::time::SystemTime>), StableReadError> {
    read_snapshot_with(canonical_root, path, max_bytes, || {})
}

#[cfg(test)]
fn read_bounded_stable_with(
    canonical_root: &Path,
    path: &Path,
    max_bytes: u64,
    after_read: impl FnOnce(),
) -> Result<Vec<u8>, StableReadError> {
    read_snapshot_with(canonical_root, path, max_bytes, after_read).map(|(bytes, _)| bytes)
}

fn read_snapshot_with(
    canonical_root: &Path,
    path: &Path,
    max_bytes: u64,
    after_read: impl FnOnce(),
) -> Result<(Vec<u8>, Option<std::time::SystemTime>), StableReadError> {
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
    Ok((bytes, original_modified))
}

fn same_file_identity(left: &File, right: &File) -> std::io::Result<bool> {
    Ok(file_identity(left)? == file_identity(right)?)
}

#[cfg(unix)]
fn file_identity(file: &File) -> std::io::Result<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file.metadata()?;
    Ok((metadata.dev(), metadata.ino()))
}

#[cfg(windows)]
fn file_identity(file: &File) -> std::io::Result<(u64, u64)> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::Storage::FileSystem::{
        GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
    };

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
    Ok((u64::from(information.dwVolumeSerialNumber), index))
}

#[cfg(not(any(unix, windows)))]
fn file_identity(_file: &File) -> std::io::Result<(u64, u64)> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "file identity unavailable",
    ))
}

const APPEND_WITNESS_BYTES: usize = 4096;
pub(crate) const MAX_APPEND_READ: usize = 256 * 1024;

/// A verified bounded prefix plus just enough anchored evidence for any consumed subprefix.
/// Witnesses are calculated from captured bytes, never from a later changed source.
pub(crate) struct AppendRange {
    pub bytes: Vec<u8>,
    start: u64,
    prefix: Vec<u8>,
    preceding: Vec<u8>,
    identity: (u64, u64),
}

impl AppendRange {
    pub fn witness(&self, accepted: usize) -> Result<String, StableReadError> {
        use sha2::{Digest, Sha256};
        if accepted > self.bytes.len() {
            return Err(StableReadError::Invalid(
                "append witness exceeds captured bytes",
            ));
        }
        let cursor = self
            .start
            .checked_add(accepted as u64)
            .ok_or(StableReadError::Invalid("append position overflow"))?;
        let prefix_len = cursor.min(APPEND_WITNESS_BYTES as u64) as usize;
        let mut trailing = Vec::with_capacity(APPEND_WITNESS_BYTES * 2);
        if accepted < APPEND_WITNESS_BYTES {
            let wanted = APPEND_WITNESS_BYTES - accepted;
            trailing
                .extend_from_slice(&self.preceding[self.preceding.len().saturating_sub(wanted)..]);
        }
        trailing.extend_from_slice(
            &self.bytes[accepted.saturating_sub(APPEND_WITNESS_BYTES)..accepted],
        );
        let mut hash = Sha256::new();
        hash.update(b"eso-weave/native-source-boundary/v1");
        hash.update(self.identity.0.to_be_bytes());
        hash.update(self.identity.1.to_be_bytes());
        hash.update(cursor.to_be_bytes());
        hash.update((prefix_len as u64).to_be_bytes());
        hash.update(&self.prefix[..prefix_len]);
        hash.update(trailing);
        Ok(hash
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }
}

pub(crate) fn read_append_range(
    root: &Path,
    path: &Path,
    start: u64,
    limit: usize,
) -> Result<AppendRange, StableReadError> {
    if limit > MAX_APPEND_READ {
        return Err(StableReadError::Invalid(
            "append read exceeds its turn budget",
        ));
    }
    let mut file = open_bounded_stable(root, path, u64::MAX)?;
    let length = file.metadata().map_err(StableReadError::Io)?.len();
    if start > length {
        return Err(StableReadError::Invalid("native source was truncated"));
    }
    let count = (length - start).min(limit as u64) as usize;
    let end = start
        .checked_add(count as u64)
        .ok_or(StableReadError::Invalid("append position overflow"))?;
    let identity = file_identity(&file).map_err(StableReadError::Io)?;
    let prefix = read_file_range(&mut file, 0, end.min(APPEND_WITNESS_BYTES as u64) as usize)?;
    let preceding_start = start.saturating_sub(APPEND_WITNESS_BYTES as u64);
    let preceding = read_file_range(
        &mut file,
        preceding_start,
        (start - preceding_start) as usize,
    )?;
    let bytes = read_file_range(&mut file, start, count)?;
    // Re-read only the bounded sampled regions; writers may append past them freely.
    if read_file_range(&mut file, 0, prefix.len())? != prefix
        || read_file_range(&mut file, preceding_start, preceding.len())? != preceding
        || read_file_range(&mut file, start, count)? != bytes
    {
        return Err(StableReadError::Invalid(
            "native source changed during bounded read",
        ));
    }
    let path_file = open_bounded_stable(root, path, u64::MAX)?;
    if !same_file_identity(&file, &path_file).map_err(StableReadError::Io)?
        || path_file.metadata().map_err(StableReadError::Io)?.len() < end
    {
        return Err(StableReadError::Invalid("native source generation changed"));
    }
    Ok(AppendRange {
        bytes,
        start,
        prefix,
        preceding,
        identity,
    })
}

fn read_file_range(file: &mut File, offset: u64, count: usize) -> Result<Vec<u8>, StableReadError> {
    file.seek(SeekFrom::Start(offset))
        .map_err(StableReadError::Io)?;
    let mut bytes = vec![0; count];
    file.read_exact(&mut bytes).map_err(StableReadError::Io)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_ranges_allow_growth_preserve_bytes_and_verify_old_boundary() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("Encounter.log");
        fs::write(&file, b"first\nsecond\n").unwrap();
        let canonical = fs::canonicalize(root.path()).unwrap();
        let first = read_append_range(&canonical, &file, 0, 6).unwrap();
        assert_eq!(first.bytes, b"first\n");
        let witness = first.witness(6).unwrap();
        use std::io::Write;
        OpenOptions::new()
            .append(true)
            .open(&file)
            .unwrap()
            .write_all(b"third\n")
            .unwrap();
        let next = read_append_range(&canonical, &file, 6, 7).unwrap();
        assert_eq!(next.witness(0).unwrap(), witness);
        assert_eq!(next.bytes, b"second\n");
        assert_eq!(fs::read(&file).unwrap(), b"first\nsecond\nthird\n");
        fs::write(&file, b"other\nsecond\nthird\n").unwrap();
        let changed = read_append_range(&canonical, &file, 6, 0).unwrap();
        assert_ne!(changed.witness(0).unwrap(), witness);
    }

    #[test]
    fn append_partial_prefix_witness_matches_reopen_at_that_exact_cursor() {
        let root = tempfile::tempdir().unwrap();
        let file = root.path().join("Encounter.log");
        let bytes = (0..20_000).map(|n| (n % 251) as u8).collect::<Vec<_>>();
        fs::write(&file, &bytes).unwrap();
        let canonical = fs::canonicalize(root.path()).unwrap();
        for start in [0, 9, 4093, 4096, 9000] {
            let part = read_append_range(&canonical, &file, start, 7000).unwrap();
            for accepted in [0, 3, 4096, part.bytes.len()] {
                let cursor = start + accepted as u64;
                let reopened = read_append_range(&canonical, &file, cursor, 0).unwrap();
                assert_eq!(
                    part.witness(accepted).unwrap(),
                    reopened.witness(0).unwrap()
                );
            }
        }
        assert!(read_append_range(&canonical, &file, 20_001, 32).is_err());
        assert!(read_append_range(&canonical, &file, 0, 256 * 1024 + 1).is_err());
    }

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
