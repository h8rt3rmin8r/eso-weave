use std::fs;
use std::io::Cursor;
use std::path::Path;

use image::codecs::png::{CompressionType, FilterType, PngDecoder, PngEncoder};
use image::{DynamicImage, ImageDecoder, ImageEncoder, Rgba, RgbaImage};
use sha2::{Digest, Sha256};

use super::path::is_link_like;
use super::{FallbackReason, MAX_ICON_DIMENSION, MAX_ICON_PIXELS, MAX_SOURCE_BYTES};

pub struct TransformedIcon {
    pub png: Vec<u8>,
    pub source_sha256: String,
    pub width: u32,
    pub height: u32,
}

pub fn transform_source(path: &Path) -> Result<TransformedIcon, FallbackReason> {
    let metadata = fs::symlink_metadata(path).map_err(map_io)?;
    if !metadata.is_file() || is_link_like(&metadata) || metadata.len() > MAX_SOURCE_BYTES {
        return Err(FallbackReason::Invalid);
    }
    let bytes = fs::read(path).map_err(map_io)?;
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
