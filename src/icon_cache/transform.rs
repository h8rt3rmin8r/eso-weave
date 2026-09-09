use std::fs::{File, OpenOptions};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use image::codecs::png::{CompressionType, FilterType, PngDecoder, PngEncoder};
use image::{DynamicImage, ImageDecoder, ImageEncoder, Rgba, RgbaImage};
use sha2::{Digest, Sha256};

#[cfg(windows)]
use std::ffi::OsString;
#[cfg(target_os = "linux")]
use std::fs;

use super::path::{is_link_like, is_same_or_nested};
use super::{FallbackReason, MAX_ICON_DIMENSION, MAX_ICON_PIXELS, MAX_SOURCE_BYTES};

pub struct TransformedIcon {
    pub png: Vec<u8>,
    pub source_sha256: String,
    pub width: u32,
    pub height: u32,
}

pub fn transform_source(
    canonical_root: &Path,
    path: &Path,
) -> Result<TransformedIcon, FallbackReason> {
    let mut file = open_no_follow(path).map_err(map_io)?;
    let metadata = file.metadata().map_err(map_io)?;
    if !metadata.is_file() || is_link_like(&metadata) || metadata.len() > MAX_SOURCE_BYTES {
        return Err(FallbackReason::Invalid);
    }
    let opened_path = opened_file_path(&file).map_err(map_io)?;
    if !is_same_or_nested(&opened_path, canonical_root) {
        return Err(FallbackReason::Invalid);
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.by_ref()
        .take(MAX_SOURCE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(map_io)?;
    if bytes.len() as u64 > MAX_SOURCE_BYTES {
        return Err(FallbackReason::Invalid);
    }
    let source_sha256 = sha256(&bytes);
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or(FallbackReason::Unsupported)?;
    let image = match extension.as_str() {
        "png" => decode_png(&bytes)?,
        "dds" => decode_dds(&bytes)?,
        _ => return Err(FallbackReason::Unsupported),
    };
    validate_dimensions(image.width(), image.height())?;
    Ok(TransformedIcon {
        png: encode_png(&image)?,
        source_sha256,
        width: image.width(),
        height: image.height(),
    })
}

pub fn placeholder_png() -> (Vec<u8>, u32, u32) {
    const SIZE: u32 = 64;
    let mut image = RgbaImage::new(SIZE, SIZE);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let border = x < 3 || y < 3 || x >= SIZE - 3 || y >= SIZE - 3;
        let diagonal = x.abs_diff(y) <= 2 || x + y >= SIZE - 3 && x + y <= SIZE + 1;
        *pixel = if border || diagonal {
            Rgba([82, 104, 111, 255])
        } else if (x / 8 + y / 8) % 2 == 0 {
            Rgba([38, 44, 48, 255])
        } else {
            Rgba([47, 54, 59, 255])
        };
    }
    (
        encode_png(&image).expect("project placeholder dimensions are valid"),
        SIZE,
        SIZE,
    )
}

pub fn verify_png(bytes: &[u8], expected: (u32, u32)) -> Result<(), FallbackReason> {
    let image = decode_png(bytes)?;
    if image.dimensions() != expected {
        return Err(FallbackReason::Invalid);
    }
    Ok(())
}

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn decode_png(bytes: &[u8]) -> Result<RgbaImage, FallbackReason> {
    let decoder = PngDecoder::new(Cursor::new(bytes)).map_err(|_| FallbackReason::Invalid)?;
    if decoder.is_apng().map_err(|_| FallbackReason::Invalid)? {
        return Err(FallbackReason::Invalid);
    }
    validate_dimensions(decoder.dimensions().0, decoder.dimensions().1)?;
    DynamicImage::from_decoder(decoder)
        .map(DynamicImage::into_rgba8)
        .map_err(|_| FallbackReason::Invalid)
}

fn decode_dds(bytes: &[u8]) -> Result<RgbaImage, FallbackReason> {
    let dds =
        image_dds::ddsfile::Dds::read(Cursor::new(bytes)).map_err(|_| FallbackReason::Invalid)?;
    validate_dimensions(dds.get_width(), dds.get_height())?;
    if dds.get_depth() != 1 || dds.get_num_array_layers() != 1 || dds.get_num_mipmap_levels() > 16 {
        return Err(FallbackReason::Invalid);
    }
    image_dds::image_from_dds(&dds, 0).map_err(|_| FallbackReason::Invalid)
}

fn validate_dimensions(width: u32, height: u32) -> Result<(), FallbackReason> {
    let pixels = width.checked_mul(height).ok_or(FallbackReason::Invalid)?;
    if width == 0
        || height == 0
        || width > MAX_ICON_DIMENSION
        || height > MAX_ICON_DIMENSION
        || pixels > MAX_ICON_PIXELS
    {
        return Err(FallbackReason::Invalid);
    }
    Ok(())
}

fn encode_png(image: &RgbaImage) -> Result<Vec<u8>, FallbackReason> {
    let mut bytes = Vec::new();
    PngEncoder::new_with_quality(&mut bytes, CompressionType::Best, FilterType::Adaptive)
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|_| FallbackReason::Invalid)?;
    Ok(bytes)
}

fn map_io(error: std::io::Error) -> FallbackReason {
    if error.kind() == std::io::ErrorKind::PermissionDenied {
        FallbackReason::PermissionDenied
    } else {
        FallbackReason::IoFailed
    }
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
        "icon cache source handles support only Windows and Linux",
    ))
}

#[cfg(not(any(target_os = "linux", windows)))]
fn opened_file_path(_file: &File) -> std::io::Result<PathBuf> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "icon cache source handles support only Windows and Linux",
    ))
}
