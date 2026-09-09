use std::fs;
use std::path::{Component, Path, PathBuf};

use super::{FallbackReason, IconCacheError};

const MAX_COMPONENTS: usize = 64;
const MAX_COMPONENT_BYTES: usize = 255;
const MAX_DIRECTORY_ENTRIES: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedIconPath {
    pub canonical: String,
    pub lookup_key: String,
    pub components: Vec<String>,
}

pub enum LocalResolution {
    Found(PathBuf),
    Fallback(FallbackReason),
}

pub fn normalize_virtual_path(value: &str) -> Result<NormalizedIconPath, IconCacheError> {
    if value.is_empty()
        || value.len() > 64 * 1024
        || value.contains(['\0', '\\', ':'])
        || !value.starts_with('/')
        || value.starts_with("//")
    {
        return invalid("virtual path is not a safe rooted game path");
    }
    let components: Vec<_> = value[1..].split('/').map(str::to_string).collect();
    if components.is_empty()
        || components.len() > MAX_COMPONENTS
        || components.iter().any(|component| {
            component.is_empty()
                || component == "."
                || component == ".."
                || component.len() > MAX_COMPONENT_BYTES
        })
    {
        return invalid("virtual path contains an unsafe component");
    }
    let lookup_key = components
        .iter()
        .map(|component| component.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("/");
    Ok(NormalizedIconPath {
        canonical: value.to_string(),
        lookup_key,
        components,
    })
}

pub fn validate_distinct_roots(source: &Path, cache: &Path) -> Result<PathBuf, IconCacheError> {
    let source_metadata = fs::symlink_metadata(source)?;
    if !source_metadata.is_dir() || is_link_like(&source_metadata) {
        return invalid("source root must be a real directory, not a link");
    }
    let source = fs::canonicalize(source)?;
    if cache.exists() && is_link_like(&fs::symlink_metadata(cache)?) {
        return invalid("cache root must be a real path, not a link");
    }
    let cache = resolve_for_alias(cache)?;
    if is_same_or_nested(&cache, &source) || is_same_or_nested(&source, &cache) {
        return invalid("source and cache roots must be distinct and unnested");
    }
    Ok(source)
}

pub fn resolve_local(
    canonical_root: &Path,
    icon: &NormalizedIconPath,
) -> Result<LocalResolution, IconCacheError> {
    let mut current = canonical_root.to_path_buf();
    for (index, component) in icon.components.iter().enumerate() {
        let entries = match fs::read_dir(&current) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(LocalResolution::Fallback(FallbackReason::Missing));
            }
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                return Ok(LocalResolution::Fallback(FallbackReason::PermissionDenied));
            }
            Err(_) => return Ok(LocalResolution::Fallback(FallbackReason::IoFailed)),
        };
        let mut matched = None;
        for (count, entry) in entries.enumerate() {
            if count >= MAX_DIRECTORY_ENTRIES {
                return Ok(LocalResolution::Fallback(FallbackReason::Invalid));
            }
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                    return Ok(LocalResolution::Fallback(FallbackReason::PermissionDenied));
                }
                Err(_) => return Ok(LocalResolution::Fallback(FallbackReason::IoFailed)),
            };
            let Some(name) = entry.file_name().to_str().map(str::to_string) else {
                continue;
            };
            if name.eq_ignore_ascii_case(component) {
                if matched.is_some() {
                    return Ok(LocalResolution::Fallback(FallbackReason::Invalid));
                }
                matched = Some(entry.path());
            }
        }
        let Some(next) = matched else {
            return Ok(LocalResolution::Fallback(FallbackReason::Missing));
        };
        let metadata = match fs::symlink_metadata(&next) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                return Ok(LocalResolution::Fallback(FallbackReason::PermissionDenied));
            }
            Err(_) => return Ok(LocalResolution::Fallback(FallbackReason::IoFailed)),
        };
        if is_link_like(&metadata) {
            return Ok(LocalResolution::Fallback(FallbackReason::Invalid));
        }
        let final_component = index + 1 == icon.components.len();
        if (!final_component && !metadata.is_dir()) || (final_component && !metadata.is_file()) {
            return Ok(LocalResolution::Fallback(FallbackReason::Invalid));
        }
        current = next;
    }
    let resolved = match fs::canonicalize(&current) {
        Ok(path) => path,
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            return Ok(LocalResolution::Fallback(FallbackReason::PermissionDenied));
        }
        Err(_) => return Ok(LocalResolution::Fallback(FallbackReason::IoFailed)),
    };
    if !resolved.starts_with(canonical_root) {
        return Ok(LocalResolution::Fallback(FallbackReason::Invalid));
    }
    Ok(LocalResolution::Found(resolved))
}

fn resolve_for_alias(path: &Path) -> Result<PathBuf, IconCacheError> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    if absolute.exists() {
        return Ok(fs::canonicalize(absolute)?);
    }
    let mut missing = Vec::new();
    let mut ancestor = absolute.as_path();
    while !ancestor.exists() {
        let Some(name) = ancestor.file_name() else {
            return invalid("cache root has no existing ancestor");
        };
        missing.push(name.to_os_string());
        ancestor = ancestor
            .parent()
            .ok_or_else(|| IconCacheError::Validation("cache root has no parent".into()))?;
    }
    let mut resolved = fs::canonicalize(ancestor)?;
    for component in missing.iter().rev() {
        resolved.push(component);
    }
    Ok(normalize_lexically(resolved))
}

fn normalize_lexically(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

pub(super) fn is_same_or_nested(candidate: &Path, root: &Path) -> bool {
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
pub(super) fn is_link_like(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

#[cfg(windows)]
pub(super) fn is_link_like(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

fn invalid<T>(message: impl Into<String>) -> Result<T, IconCacheError> {
    Err(IconCacheError::Validation(message.into()))
}
