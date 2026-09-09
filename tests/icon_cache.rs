use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use eso_weave::icon_cache::{
    build_generation, open_generation, FallbackReason, IconCacheRequest, IconLookupState,
    MAX_ICON_DIMENSION, MAX_SOURCE_BYTES, TRANSFORMATION_ID,
};
use image::{ImageFormat, Rgba, RgbaImage};
use sha2::{Digest, Sha256};

const CATALOG_HASH: &str = "1111111111111111111111111111111111111111111111111111111111111111";

struct Sandbox {
    root: tempfile::TempDir,
}

impl Sandbox {
    fn new() -> Self {
        Self {
            root: tempfile::tempdir().expect("icon cache sandbox"),
        }
    }

    fn source(&self) -> PathBuf {
        self.root.path().join("source")
    }

    fn cache(&self) -> PathBuf {
        self.root.path().join("cache")
    }

    fn source_path(&self, relative: &str) -> PathBuf {
        self.source()
            .join(relative.replace('/', std::path::MAIN_SEPARATOR_STR))
    }
}

fn write_png(path: &Path, width: u32, height: u32, color: [u8; 4]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let image = RgbaImage::from_pixel(width, height, Rgba(color));
    image.save_with_format(path, ImageFormat::Png).unwrap();
}

fn write_dxt1_dds(path: &Path, color_565: u16) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"DDS ");
    for value in [124_u32, 0x0008_1007, 4, 4, 8, 0, 0] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes.extend_from_slice(&[0_u8; 44]);
    for value in [32_u32, 4] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes.extend_from_slice(b"DXT1");
    bytes.extend_from_slice(&[0_u8; 20]);
    bytes.extend_from_slice(&0x1000_u32.to_le_bytes());
    bytes.extend_from_slice(&[0_u8; 16]);
    assert_eq!(bytes.len(), 128);
    bytes.extend_from_slice(&color_565.to_le_bytes());
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    fs::write(path, bytes).unwrap();
}

fn request(sandbox: &Sandbox, references: &[&str]) -> IconCacheRequest {
    fs::create_dir_all(sandbox.source()).unwrap();
    IconCacheRequest::new(
        sandbox.source(),
        sandbox.cache(),
        CATALOG_HASH,
        references.iter().map(|value| value.to_string()).collect(),
    )
}

#[test]
fn valid_local_and_missing_references_get_explicit_renderable_mappings() {
    let sandbox = Sandbox::new();
    write_png(
        &sandbox.source_path("esoui/art/icons/alpha.png"),
        4,
        4,
        [12, 34, 56, 255],
    );
    let source_path = sandbox.source_path("esoui/art/icons/alpha.png");
    let source_before = fs::read(&source_path).unwrap();
    let receipt = build_generation(&request(
        &sandbox,
        &["/esoui/art/icons/alpha.png", "/esoui/art/icons/missing.dds"],
    ))
    .expect("build generation");

    assert_eq!(receipt.entry_count, 2);
    assert_eq!(receipt.placeholder_count, 1);
    assert_eq!(receipt.transformation_id, TRANSFORMATION_ID);
    assert_eq!(receipt.generation_sha256.len(), 64);

    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    let ready = generation.resolve("/ESOUI/ART/ICONS/ALPHA.PNG").unwrap();
    assert_eq!(ready.state, IconLookupState::Ready);
    assert!(!ready.uses_placeholder);
    assert!(ready.object_path.is_file());

    let missing = generation.resolve("/esoui/art/icons/missing.dds").unwrap();
    assert_eq!(missing.state, IconLookupState::Missing);
    assert!(missing.uses_placeholder);
    assert_eq!(missing.fallback_reason, Some(FallbackReason::Missing));
    assert!(missing.object_path.is_file());

    assert!(!receipt.manifest_path.is_absolute());
    let manifest_bytes = fs::read(sandbox.cache().join(&receipt.manifest_path)).unwrap();
    let manifest_text = String::from_utf8(manifest_bytes).unwrap();
    assert!(!manifest_text.contains(sandbox.source().to_string_lossy().as_ref()));
    assert!(!manifest_text.contains(sandbox.cache().to_string_lossy().as_ref()));
    assert_eq!(source_before, fs::read(source_path).unwrap());
}

#[test]
fn dds_decoding_and_rgba_png_input_deduplicate_by_transformed_content() {
    let sandbox = Sandbox::new();
    write_dxt1_dds(&sandbox.source_path("esoui/art/icons/red.dds"), 0xf800);
    write_png(
        &sandbox.source_path("esoui/art/icons/red.png"),
        4,
        4,
        [255, 0, 0, 255],
    );

    let receipt = build_generation(&request(
        &sandbox,
        &["/esoui/art/icons/red.dds", "/esoui/art/icons/red.png"],
    ))
    .unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    let dds = generation.resolve("/esoui/art/icons/red.dds").unwrap();
    let png = generation.resolve("/esoui/art/icons/red.png").unwrap();

    assert_eq!(dds.object_sha256, png.object_sha256);
    assert_eq!(receipt.object_count, 2, "local object plus placeholder");
}

#[test]
fn repeated_builds_are_byte_stable_and_reuse_immutable_generation() {
    let sandbox = Sandbox::new();
    write_png(
        &sandbox.source_path("esoui/art/icons/stable.png"),
        3,
        2,
        [2, 4, 6, 128],
    );
    let request = request(&sandbox, &["/esoui/art/icons/stable.png"]);
    let first = build_generation(&request).unwrap();
    let first_bytes = fs::read(sandbox.cache().join(&first.manifest_path)).unwrap();
    let second = build_generation(&request).unwrap();

    assert_eq!(first.generation_sha256, second.generation_sha256);
    assert_eq!(first.manifest_path, second.manifest_path);
    assert_eq!(
        first_bytes,
        fs::read(sandbox.cache().join(second.manifest_path)).unwrap()
    );
}

#[test]
fn invalid_virtual_paths_and_root_aliases_are_rejected_before_publication() {
    for path in [
        "",
        "/../secret.png",
        "/esoui//secret.png",
        "C:/secret.png",
        "\\\\server\\share\\secret.png",
        "/esoui/./secret.png",
        "/esoui/art\\secret.png",
        "/esoui/art/nul\0.png",
    ] {
        let sandbox = Sandbox::new();
        let error = build_generation(&request(&sandbox, &[path])).unwrap_err();
        assert!(
            error.to_string().contains("virtual path"),
            "{path:?}: {error}"
        );
        assert!(!sandbox.cache().join("generations").exists());
    }

    let sandbox = Sandbox::new();
    fs::create_dir_all(sandbox.source()).unwrap();
    let alias = IconCacheRequest::new(
        sandbox.source(),
        sandbox.source().join("cache"),
        CATALOG_HASH,
        vec!["/esoui/art/icons/a.png".to_string()],
    );
    let error = build_generation(&alias).unwrap_err();
    assert!(error.to_string().contains("distinct"));
}

#[test]
fn unsupported_corrupt_oversized_and_overdimensioned_inputs_fall_back() {
    let sandbox = Sandbox::new();
    let unsupported = sandbox.source_path("esoui/art/icons/notes.txt");
    let corrupt = sandbox.source_path("esoui/art/icons/corrupt.png");
    let oversized = sandbox.source_path("esoui/art/icons/large.dds");
    fs::create_dir_all(unsupported.parent().unwrap()).unwrap();
    fs::write(&unsupported, b"not an image").unwrap();
    fs::write(&corrupt, b"not a png").unwrap();
    let mut file = fs::File::create(&oversized).unwrap();
    file.write_all(b"DDS ").unwrap();
    file.set_len(MAX_SOURCE_BYTES + 1).unwrap();
    write_png(
        &sandbox.source_path("esoui/art/icons/wide.png"),
        MAX_ICON_DIMENSION + 1,
        1,
        [0, 0, 0, 255],
    );

    let receipt = build_generation(&request(
        &sandbox,
        &[
            "/esoui/art/icons/notes.txt",
            "/esoui/art/icons/corrupt.png",
            "/esoui/art/icons/large.dds",
            "/esoui/art/icons/wide.png",
        ],
    ))
    .unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();

    assert_eq!(receipt.placeholder_count, 4);
    assert_eq!(
        generation
            .resolve("/esoui/art/icons/notes.txt")
            .unwrap()
            .state,
        IconLookupState::Unsupported
    );
    for path in ["corrupt.png", "large.dds", "wide.png"] {
        assert_eq!(
            generation
                .resolve(&format!("/esoui/art/icons/{path}"))
                .unwrap()
                .state,
            IconLookupState::Failed
        );
    }
}

#[test]
fn animated_png_input_falls_back_instead_of_publishing_its_default_frame() {
    let sandbox = Sandbox::new();
    let path = sandbox.source_path("esoui/art/icons/animated.png");
    write_png(&path, 1, 1, [1, 2, 3, 255]);
    let mut bytes = fs::read(&path).unwrap();
    insert_png_chunk(&mut bytes, 33, *b"acTL", &[0, 0, 0, 1, 0, 0, 0, 0]);
    let frame_control = [
        0, 0, 0, 0, // sequence
        0, 0, 0, 1, // width
        0, 0, 0, 1, // height
        0, 0, 0, 0, // x offset
        0, 0, 0, 0, // y offset
        0, 1, 0, 10, // delay numerator and denominator
        0, 0, // dispose and blend
    ];
    insert_png_chunk(&mut bytes, 53, *b"fcTL", &frame_control);
    fs::write(&path, bytes).unwrap();

    let receipt = build_generation(&request(&sandbox, &["/esoui/art/icons/animated.png"])).unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    let resolution = generation.resolve("/esoui/art/icons/animated.png").unwrap();
    assert_eq!(resolution.state, IconLookupState::Failed);
    assert!(resolution.uses_placeholder);
}

#[test]
fn unsupported_dds_volume_cubemap_and_mipmap_shapes_fall_back_before_decode() {
    let sandbox = Sandbox::new();
    for name in ["volume.dds", "cubemap.dds", "mipmaps.dds"] {
        write_dxt1_dds(
            &sandbox.source_path(&format!("esoui/art/icons/{name}")),
            0xf800,
        );
    }

    let volume = sandbox.source_path("esoui/art/icons/volume.dds");
    mutate_u32(&volume, 8, |flags| flags | 0x0080_0000);
    mutate_u32(&volume, 24, |_| 2);
    mutate_u32(&volume, 112, |caps2| caps2 | 0x0020_0000);

    let cubemap = sandbox.source_path("esoui/art/icons/cubemap.dds");
    mutate_u32(&cubemap, 112, |caps2| caps2 | 0x0000_fe00);

    let mipmaps = sandbox.source_path("esoui/art/icons/mipmaps.dds");
    mutate_u32(&mipmaps, 8, |flags| flags | 0x0002_0000);
    mutate_u32(&mipmaps, 28, |_| 17);

    let receipt = build_generation(&request(
        &sandbox,
        &[
            "/esoui/art/icons/volume.dds",
            "/esoui/art/icons/cubemap.dds",
            "/esoui/art/icons/mipmaps.dds",
        ],
    ))
    .unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    assert_eq!(receipt.placeholder_count, 3);
    for name in ["volume.dds", "cubemap.dds", "mipmaps.dds"] {
        assert_eq!(
            generation
                .resolve(&format!("/esoui/art/icons/{name}"))
                .unwrap()
                .state,
            IconLookupState::Failed
        );
    }
}

#[test]
fn source_links_are_never_followed() {
    let sandbox = Sandbox::new();
    let outside = sandbox.root.path().join("outside.png");
    write_png(&outside, 2, 2, [1, 2, 3, 255]);
    let linked = sandbox.source_path("esoui/art/icons/linked.png");
    fs::create_dir_all(linked.parent().unwrap()).unwrap();
    if create_file_symlink(&outside, &linked).is_err() {
        return;
    }

    let receipt = build_generation(&request(&sandbox, &["/esoui/art/icons/linked.png"])).unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    let resolution = generation.resolve("/esoui/art/icons/linked.png").unwrap();
    assert!(resolution.uses_placeholder);
    assert_eq!(resolution.state, IconLookupState::Failed);
}

#[test]
#[cfg(target_os = "linux")]
fn ambiguous_case_matches_are_rejected() {
    let sandbox = Sandbox::new();
    write_png(
        &sandbox.source_path("esoui/art/icons/Icon.png"),
        1,
        1,
        [1, 1, 1, 255],
    );
    write_png(
        &sandbox.source_path("esoui/art/icons/ICON.png"),
        1,
        1,
        [2, 2, 2, 255],
    );
    let receipt = build_generation(&request(&sandbox, &["/esoui/art/icons/icon.png"])).unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    assert!(
        generation
            .resolve("/esoui/art/icons/icon.png")
            .unwrap()
            .uses_placeholder
    );
}

#[test]
fn a_failed_candidate_cannot_damage_an_existing_generation() {
    let sandbox = Sandbox::new();
    write_png(
        &sandbox.source_path("esoui/art/icons/first.png"),
        1,
        1,
        [1, 2, 3, 255],
    );
    let first = build_generation(&request(&sandbox, &["/esoui/art/icons/first.png"])).unwrap();
    let first_manifest = fs::read(sandbox.cache().join(&first.manifest_path)).unwrap();

    let probe = Sandbox::new();
    write_png(
        &probe.source_path("esoui/art/icons/second.png"),
        1,
        1,
        [7, 8, 9, 255],
    );
    let probe_receipt =
        build_generation(&request(&probe, &["/esoui/art/icons/second.png"])).unwrap();
    let probe_generation =
        open_generation(&probe.cache(), &probe_receipt.generation_sha256).unwrap();
    let second_hash = probe_generation
        .resolve("/esoui/art/icons/second.png")
        .unwrap()
        .object_sha256;

    write_png(
        &sandbox.source_path("esoui/art/icons/second.png"),
        1,
        1,
        [7, 8, 9, 255],
    );
    fs::write(
        sandbox
            .cache()
            .join("objects")
            .join(format!("{second_hash}.png")),
        b"corrupt collision",
    )
    .unwrap();
    assert!(build_generation(&request(&sandbox, &["/esoui/art/icons/second.png"],)).is_err());

    assert_eq!(
        first_manifest,
        fs::read(sandbox.cache().join(&first.manifest_path)).unwrap()
    );
    assert_eq!(
        open_generation(&sandbox.cache(), &first.generation_sha256)
            .unwrap()
            .resolve("/esoui/art/icons/first.png")
            .unwrap()
            .state,
        IconLookupState::Ready
    );
}

#[test]
fn generation_loading_rejects_noncanonical_manifest_and_object_tampering() {
    let sandbox = Sandbox::new();
    write_png(
        &sandbox.source_path("esoui/art/icons/a.png"),
        1,
        1,
        [5, 6, 7, 255],
    );
    let receipt = build_generation(&request(&sandbox, &["/esoui/art/icons/a.png"])).unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    let object = generation
        .resolve("/esoui/art/icons/a.png")
        .unwrap()
        .object_path;
    fs::write(&object, b"tampered").unwrap();
    assert!(open_generation(&sandbox.cache(), &receipt.generation_sha256).is_err());

    let sandbox = Sandbox::new();
    write_png(
        &sandbox.source_path("esoui/art/icons/b.png"),
        1,
        1,
        [5, 6, 7, 255],
    );
    let receipt = build_generation(&request(&sandbox, &["/esoui/art/icons/b.png"])).unwrap();
    let manifest_path = sandbox.cache().join(&receipt.manifest_path);
    let mut manifest = fs::read(&manifest_path).unwrap();
    manifest.extend_from_slice(b" \n");
    fs::write(&manifest_path, &manifest).unwrap();
    let noncanonical_hash = format!("{:x}", Sha256::digest(&manifest));
    let generation_root = sandbox.cache().join("generations");
    fs::rename(
        generation_root.join(&receipt.generation_sha256),
        generation_root.join(&noncanonical_hash),
    )
    .unwrap();
    assert!(open_generation(&sandbox.cache(), &noncanonical_hash).is_err());
}

#[test]
fn generation_loading_rejects_linked_cache_objects() {
    let sandbox = Sandbox::new();
    write_png(
        &sandbox.source_path("esoui/art/icons/a.png"),
        1,
        1,
        [5, 6, 7, 255],
    );
    let receipt = build_generation(&request(&sandbox, &["/esoui/art/icons/a.png"])).unwrap();
    let generation = open_generation(&sandbox.cache(), &receipt.generation_sha256).unwrap();
    let object = generation
        .resolve("/esoui/art/icons/a.png")
        .unwrap()
        .object_path;
    let outside = sandbox.root.path().join("outside-cache-object.png");
    fs::copy(&object, &outside).unwrap();
    fs::remove_file(&object).unwrap();
    if create_file_symlink(&outside, &object).is_err() {
        return;
    }

    assert!(open_generation(&sandbox.cache(), &receipt.generation_sha256).is_err());
}

fn mutate_u32(path: &Path, offset: usize, update: impl FnOnce(u32) -> u32) {
    let mut bytes = fs::read(path).unwrap();
    let current = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    bytes[offset..offset + 4].copy_from_slice(&update(current).to_le_bytes());
    fs::write(path, bytes).unwrap();
}

fn insert_png_chunk(bytes: &mut Vec<u8>, offset: usize, kind: [u8; 4], data: &[u8]) {
    let mut chunk = Vec::with_capacity(data.len() + 12);
    chunk.extend_from_slice(&(data.len() as u32).to_be_bytes());
    chunk.extend_from_slice(&kind);
    chunk.extend_from_slice(data);
    chunk.extend_from_slice(&png_crc32(&kind, data).to_be_bytes());
    bytes.splice(offset..offset, chunk);
}

fn png_crc32(kind: &[u8; 4], data: &[u8]) -> u32 {
    kind.iter().chain(data).fold(u32::MAX, |crc, byte| {
        (0..8).fold(crc ^ u32::from(*byte), |value, _| {
            if value & 1 == 0 {
                value >> 1
            } else {
                (value >> 1) ^ 0xedb8_8320
            }
        })
    }) ^ u32::MAX
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}
