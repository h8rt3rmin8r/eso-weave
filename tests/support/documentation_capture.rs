//! Deterministic, non-production documentation capture support.

use std::ffi::OsString;
use std::fs;
use std::iter::once;
use std::mem;
use std::path::{Component, Path, PathBuf};
use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use eframe::egui;
use egui::TexturesDelta;
use egui_kittest::{Harness, TestRenderer};
use egui_wgpu::{wgpu, RenderState, RendererOptions, ScreenDescriptor, WgpuSetup};
use image::RgbaImage;
use serde::Serialize;
use sha2::{Digest, Sha256};

use eso_weave::app::ui::EsoWeaveApp;
use eso_weave::app::{AppModel, AppView, BeaconCondition, ResourcePresentation, StatusRole};
use eso_weave::beacon::{self, BeaconPrefs, Environment};
use eso_weave::catalog::CatalogAccess;
use eso_weave::config::{LoggingPrefs, Settings};
use eso_weave::fishing::{FishingConfig, FishingController, MockFishingSink};
use eso_weave::game::{
    CandidateSource, FocusObservation, InstallationCandidate, InstallationProvider,
    InstallationState, Presence, ProcessObservation, SurfaceObservation,
};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::{ActionReceiver, InputEngine};
use eso_weave::logging;
use eso_weave::pixelbus::{
    ActiveBar, CombatSignal, CooldownSet, LifeState, MenuSurface, MovementSignal,
    QuickslotClassification, QuickslotPotionAvailability, QuickslotState, ResourceLevel,
    ResourceSet, RollDodgeState, SlotCooldown, TravelState, UltimateTelemetry, UltimateValue,
    WeaponBarSignal, WeaponClass, WorldState,
};
use eso_weave::potion::{
    AutoPotionConfig, AutoPotionController, AutoPotionState, BlockReason, MockAutoPotionSink,
    PotionReadings, ResourceWatch,
};
use eso_weave::weave::{WeaveConfig, WeaveEngine, WeaveType};

const MANIFEST_FILE: &str = "capture-manifest.json";
const MANIFEST_GENERATOR: &str = "S083 deterministic screenshot sandbox";
const PIXELS_PER_POINT: f32 = 1.0;
const SETTLE_FRAMES: usize = 6;
const WAIT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scene {
    FirstLaunch,
    HealthySystemState,
    PixelBeaconLost,
    PixelBeaconUnmanaged,
    WeavingConfiguration,
    AutoPotionReady,
    AutoPotionBlocked,
}

impl Scene {
    const ALL: [Self; 7] = [
        Self::FirstLaunch,
        Self::HealthySystemState,
        Self::PixelBeaconLost,
        Self::PixelBeaconUnmanaged,
        Self::WeavingConfiguration,
        Self::AutoPotionReady,
        Self::AutoPotionBlocked,
    ];

    const fn id(self) -> &'static str {
        match self {
            Self::FirstLaunch => "first-launch",
            Self::HealthySystemState => "healthy-system-state",
            Self::PixelBeaconLost => "pixelbeacon-lost",
            Self::PixelBeaconUnmanaged => "pixelbeacon-unmanaged",
            Self::WeavingConfiguration => "weaving-configuration",
            Self::AutoPotionReady => "auto-potion-ready",
            Self::AutoPotionBlocked => "auto-potion-blocked",
        }
    }

    const fn title(self) -> &'static str {
        match self {
            Self::FirstLaunch => "First launch with ESO inactive",
            Self::HealthySystemState => "Healthy System and State baseline",
            Self::PixelBeaconLost => "PixelBeacon signal lost",
            Self::PixelBeaconUnmanaged => "Unmanaged PixelBeacon addon",
            Self::WeavingConfiguration => "Configured weaving slots and delays",
            Self::AutoPotionReady => "Auto Potion ready",
            Self::AutoPotionBlocked => "Auto Potion blocked by missing signal",
        }
    }

    const fn needs_managed_beacon(self) -> bool {
        matches!(
            self,
            Self::HealthySystemState
                | Self::PixelBeaconLost
                | Self::WeavingConfiguration
                | Self::AutoPotionReady
                | Self::AutoPotionBlocked
        )
    }

    const fn active_game(self) -> bool {
        !matches!(self, Self::FirstLaunch | Self::PixelBeaconUnmanaged)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum CaptureTheme {
    Dark,
    Light,
}

impl CaptureTheme {
    const ALL: [Self; 2] = [Self::Dark, Self::Light];

    const fn id(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    const fn egui(self) -> egui::Theme {
        match self {
            Self::Dark => egui::Theme::Dark,
            Self::Light => egui::Theme::Light,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum CaptureViewport {
    Narrow,
    Wide,
}

impl CaptureViewport {
    const ALL: [Self; 2] = [Self::Narrow, Self::Wide];

    const fn id(self) -> &'static str {
        match self {
            Self::Narrow => "narrow",
            Self::Wide => "wide",
        }
    }

    const fn size(self) -> [u32; 2] {
        match self {
            Self::Narrow => [760, 1000],
            Self::Wide => [1280, 900],
        }
    }
}

#[derive(Debug, Serialize)]
struct CaptureReceipt {
    scene: &'static str,
    title: &'static str,
    theme: CaptureTheme,
    viewport: CaptureViewport,
    width: u32,
    height: u32,
    file: String,
    sha256: String,
}

#[derive(Debug, Serialize)]
struct CaptureManifest {
    schema: u32,
    generator: &'static str,
    pixels_per_point: u32,
    captures: Vec<CaptureReceipt>,
}

struct SceneFixture {
    app: EsoWeaveApp,
    action_rx: ActionReceiver,
}

#[derive(Clone, Default)]
struct TextureCollector(Arc<Mutex<TexturesDelta>>);

impl TextureCollector {
    fn take(&self) -> Result<TexturesDelta, String> {
        self.0
            .lock()
            .map(|mut delta| mem::take(&mut *delta))
            .map_err(|_| "texture collector lock was poisoned".to_string())
    }
}

impl TestRenderer for TextureCollector {
    fn handle_delta(&mut self, delta: &mut TexturesDelta) {
        self.0
            .lock()
            .expect("texture collector lock was poisoned")
            .append(mem::take(delta));
    }
}

struct HeadlessRenderer {
    state: RenderState,
}

impl HeadlessRenderer {
    fn new() -> Result<Self, String> {
        let mut setup = egui_wgpu::WgpuSetupCreateNew::without_display_handle();
        setup
            .instance_descriptor
            .backends
            .remove(wgpu::Backends::BROWSER_WEBGPU);
        setup.native_adapter_selector = Some(Arc::new(|adapters, _surface| {
            let mut available = adapters.iter().collect::<Vec<_>>();
            available.sort_by_key(|adapter| match adapter.get_info().backend {
                wgpu::Backend::Metal => 0,
                wgpu::Backend::Vulkan => 1,
                wgpu::Backend::Dx12 => 2,
                wgpu::Backend::Gl => 3,
                wgpu::Backend::BrowserWebGpu => 4,
                wgpu::Backend::Noop => 5,
            });
            available.sort_by_key(|adapter| match adapter.get_info().device_type {
                wgpu::DeviceType::Cpu => 0,
                wgpu::DeviceType::DiscreteGpu => 1,
                wgpu::DeviceType::Other
                | wgpu::DeviceType::IntegratedGpu
                | wgpu::DeviceType::VirtualGpu => 2,
            });
            available
                .first()
                .map(|adapter| (*adapter).clone())
                .ok_or_else(|| "no headless graphics adapter is available".to_string())
        }));

        let setup = WgpuSetup::CreateNew(setup);
        let instance = pollster::block_on(setup.new_instance());
        let state = pollster::block_on(RenderState::create(
            &egui_wgpu::WgpuConfiguration {
                wgpu_setup: setup,
                ..Default::default()
            },
            &instance,
            None,
            RendererOptions::PREDICTABLE,
        ))
        .map_err(|error| format!("could not create the headless render state: {error}"))?;
        Ok(Self { state })
    }

    fn update_textures(&mut self, delta: &mut TexturesDelta) {
        let mut renderer = self.state.renderer.write();
        for (id, images) in delta.set.drain() {
            for image in images {
                renderer.update_texture(&self.state.device, &self.state.queue, id, &image);
            }
        }
        for id in delta.free.drain() {
            renderer.free_texture(&id);
        }
    }

    fn render(
        &mut self,
        context: &egui::Context,
        output: &egui::FullOutput,
    ) -> Result<RgbaImage, String> {
        let mut renderer = self.state.renderer.write();
        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("ESO Weave documentation capture encoder"),
                });
        let logical_size = context.content_rect().size() * context.pixels_per_point();
        let screen = ScreenDescriptor {
            pixels_per_point: context.pixels_per_point(),
            size_in_pixels: [logical_size.x.round() as u32, logical_size.y.round() as u32],
        };
        let tessellated = context.tessellate(output.shapes.clone(), context.pixels_per_point());
        let user_buffers = renderer.update_buffers(
            &self.state.device,
            &self.state.queue,
            &mut encoder,
            &tessellated,
            &screen,
        );
        let texture = self.state.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ESO Weave documentation capture texture"),
            size: wgpu::Extent3d {
                width: screen.size_in_pixels[0],
                height: screen.size_in_pixels[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.state.target_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ESO Weave documentation capture render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                })
                .forget_lifetime();
            renderer.render(&mut pass, &tessellated, &screen);
        }
        self.state
            .queue
            .submit(user_buffers.into_iter().chain(once(encoder.finish())));
        self.state
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(WAIT_TIMEOUT),
            })
            .map_err(|error| format!("headless renderer did not complete: {error}"))?;
        texture_to_image(&self.state.device, &self.state.queue, &texture)
    }
}

struct BufferDimensions {
    height: usize,
    unpadded_bytes_per_row: usize,
    padded_bytes_per_row: usize,
}

impl BufferDimensions {
    fn new(width: usize, height: usize) -> Self {
        let unpadded_bytes_per_row = width * mem::size_of::<u32>();
        let alignment = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padding = (alignment - unpadded_bytes_per_row % alignment) % alignment;
        Self {
            height,
            unpadded_bytes_per_row,
            padded_bytes_per_row: unpadded_bytes_per_row + padding,
        }
    }
}

fn texture_to_image(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
) -> Result<RgbaImage, String> {
    let dimensions = BufferDimensions::new(texture.width() as usize, texture.height() as usize);
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("ESO Weave documentation capture readback"),
        size: (dimensions.padded_bytes_per_row * dimensions.height) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("ESO Weave documentation capture readback encoder"),
    });
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(dimensions.padded_bytes_per_row as u32),
                rows_per_image: None,
            },
        },
        wgpu::Extent3d {
            width: texture.width(),
            height: texture.height(),
            depth_or_array_layers: 1,
        },
    );
    let submission = queue.submit(once(encoder.finish()));
    let slice = buffer.slice(..);
    let (sender, receiver) = mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    device
        .poll(wgpu::PollType::Wait {
            submission_index: Some(submission),
            timeout: Some(WAIT_TIMEOUT),
        })
        .map_err(|error| format!("capture readback did not complete: {error}"))?;
    receiver
        .recv_timeout(WAIT_TIMEOUT)
        .map_err(|_| "capture readback callback timed out".to_string())?
        .map_err(|error| format!("capture buffer mapping failed: {error}"))?;
    let data = slice
        .get_mapped_range()
        .map_err(|error| format!("capture buffer could not be read: {error}"))?;
    let packed = data
        .chunks_exact(dimensions.padded_bytes_per_row)
        .flat_map(|row| row.iter().take(dimensions.unpadded_bytes_per_row))
        .copied()
        .collect::<Vec<_>>();
    drop(data);
    buffer.unmap();
    RgbaImage::from_raw(texture.width(), texture.height(), packed)
        .ok_or_else(|| "capture buffer dimensions were inconsistent".to_string())
}

pub fn run(args: impl Iterator<Item = OsString>) -> Result<(), String> {
    let args = args.collect::<Vec<_>>();
    if args.len() > 1 {
        return Err(
            "usage: cargo test --locked --test documentation_capture -- <output-directory>"
                .to_string(),
        );
    }

    validate_catalog()?;
    validate_production_isolation()?;
    validate_output_rejections()?;
    if args.is_empty() {
        validate_scene_models()?;
        println!("documentation capture catalog and isolation checks passed");
        return Ok(());
    }

    let output = resolve_output_root(Path::new(&args[0]))?;
    generate_captures(&output)
}

fn validate_catalog() -> Result<(), String> {
    let expected = [
        ("first-launch", "First launch with ESO inactive"),
        ("healthy-system-state", "Healthy System and State baseline"),
        ("pixelbeacon-lost", "PixelBeacon signal lost"),
        ("pixelbeacon-unmanaged", "Unmanaged PixelBeacon addon"),
        (
            "weaving-configuration",
            "Configured weaving slots and delays",
        ),
        ("auto-potion-ready", "Auto Potion ready"),
        (
            "auto-potion-blocked",
            "Auto Potion blocked by missing signal",
        ),
    ];
    let actual = Scene::ALL.map(|scene| (scene.id(), scene.title()));
    ensure(
        actual == expected,
        "the seven-scene catalog or its order drifted",
    )?;
    ensure(
        CaptureTheme::ALL.map(CaptureTheme::id) == ["dark", "light"],
        "capture theme order drifted",
    )?;
    ensure(
        CaptureViewport::ALL.map(|viewport| (viewport.id(), viewport.size()))
            == [("narrow", [760, 1000]), ("wide", [1280, 900])],
        "capture viewport contract drifted",
    )?;
    let variants = Scene::ALL.len() * CaptureTheme::ALL.len() * CaptureViewport::ALL.len();
    ensure(
        variants == 28,
        "capture matrix must contain exactly 28 variants",
    )
}

fn validate_production_isolation() -> Result<(), String> {
    let root = repository_root();
    let cargo = read_text(&root.join("Cargo.toml"))?;
    let main = read_text(&root.join("src/main.rs"))?;
    let library = read_text(&root.join("src/lib.rs"))?;
    let support = read_text(&root.join("tests/support/documentation_capture.rs"))?;

    ensure(
        cargo.contains("name = \"documentation_capture\"")
            && cargo.contains("path = \"tests/documentation_capture.rs\"")
            && cargo.contains("harness = false"),
        "documentation capture must remain an explicit no-harness test target",
    )?;
    ensure(
        !cargo.contains("documentation-capture =") && !cargo.contains("documentation_capture ="),
        "documentation capture must not become a Cargo feature",
    )?;
    ensure(
        !main.contains("documentation_capture") && !library.contains("documentation_capture"),
        "production sources must not expose the documentation capture target",
    )?;

    let forbidden = [
        ["beacon::", "install("].concat(),
        ["beacon::", "install_sized("].concat(),
        ["beacon::", "uninstall("].concat(),
        ["beacon::", "redeploy_for_block_size("].concat(),
        ["beacon::", "probe_game_running("].concat(),
        ["Input", "Backend"].concat(),
        ["Gdi", "Sampler"].concat(),
        ["X11", "Sampler"].concat(),
        ["Send", "Input"].concat(),
        ["run_", "native"].concat(),
        ["dirs", "::"].concat(),
    ];
    for pattern in forbidden {
        ensure(
            !support.contains(&pattern),
            &format!("capture support contains forbidden production capability: {pattern}"),
        )?;
    }
    Ok(())
}

fn validate_scene_models() -> Result<(), String> {
    let sandbox = tempfile::tempdir()
        .map_err(|error| format!("could not create validation fixture root: {error}"))?;
    for scene in Scene::ALL {
        let fixture_root = sandbox.path().join(scene.id());
        let fixture = build_scene(scene, CaptureTheme::Dark, &fixture_root)?;
        ensure_action_channel_empty(&fixture.action_rx, scene)?;
    }
    Ok(())
}

fn validate_output_rejections() -> Result<(), String> {
    let repository = repository_root()
        .canonicalize()
        .map_err(|error| format!("could not resolve repository root: {error}"))?;
    ensure(
        resolve_output_root(&repository).is_err(),
        "capture output must reject the repository root",
    )?;
    let outside = repository
        .parent()
        .ok_or_else(|| "repository root has no parent for escape validation".to_string())?
        .join("eso-weave-capture-escape-probe");
    ensure(
        resolve_output_root(&outside).is_err(),
        "capture output must reject repository escape",
    )?;

    let sandbox = tempfile::Builder::new()
        .prefix(".documentation-capture-validation-")
        .tempdir_in(&repository)
        .map_err(|error| format!("could not create output validation root: {error}"))?;
    let collision = sandbox.path().join("file-collision");
    fs::write(&collision, b"not a directory")
        .map_err(|error| format!("could not create output collision fixture: {error}"))?;
    ensure(
        resolve_output_root(&collision).is_err(),
        "capture output must reject a non-directory collision",
    )?;

    let link_target = sandbox.path().join("link-target");
    let link = sandbox.path().join("linked-output");
    fs::create_dir(&link_target)
        .map_err(|error| format!("could not create symlink target fixture: {error}"))?;
    create_directory_symlink(&link_target, &link)
        .map_err(|error| format!("could not create symlink rejection fixture: {error}"))?;
    ensure(
        resolve_output_root(&link).is_err(),
        "capture output must reject a symlinked destination",
    )?;

    let completion_root = sandbox.path().join("completion-marker");
    fs::create_dir(&completion_root)
        .map_err(|error| format!("could not create completion-marker fixture: {error}"))?;
    let prior_manifest = completion_root.join(MANIFEST_FILE);
    fs::write(&prior_manifest, b"stale")
        .map_err(|error| format!("could not create completion-marker fixture: {error}"))?;
    clear_completion_marker(&completion_root)?;
    ensure(
        !prior_manifest.exists(),
        "capture startup must clear an older completion marker",
    )
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

fn generate_captures(output: &Path) -> Result<(), String> {
    clear_completion_marker(output)?;
    let fixture_area = tempfile::Builder::new()
        .prefix(".fixture-data-")
        .tempdir_in(output)
        .map_err(|error| format!("could not create isolated fixture area: {error}"))?;
    let mut renderer = HeadlessRenderer::new()?;
    let mut receipts = Vec::with_capacity(28);
    let mut first_image = None;

    for scene in Scene::ALL {
        for theme in CaptureTheme::ALL {
            for viewport in CaptureViewport::ALL {
                let fixture_root = fixture_area.path().join(scene.id());
                let image = render_variant(&mut renderer, scene, theme, viewport, &fixture_root)?;
                let [width, height] = viewport.size();
                ensure(
                    image.width() == width && image.height() == height,
                    "renderer output dimensions do not match the viewport contract",
                )?;
                let filename = variant_filename(scene, theme, viewport);
                let path = output.join(&filename);
                image
                    .save(&path)
                    .map_err(|error| format!("could not write {}: {error}", path.display()))?;
                let hash =
                    sha256(&fs::read(&path).map_err(|error| {
                        format!("could not verify {}: {error}", path.display())
                    })?);
                if first_image.is_none() {
                    first_image = Some((scene, theme, viewport, image.clone()));
                }
                receipts.push(CaptureReceipt {
                    scene: scene.id(),
                    title: scene.title(),
                    theme,
                    viewport,
                    width,
                    height,
                    file: filename,
                    sha256: hash,
                });
            }
        }
    }

    let (scene, theme, viewport, expected) =
        first_image.ok_or_else(|| "capture matrix unexpectedly produced no images".to_string())?;
    let repeated = render_variant(
        &mut renderer,
        scene,
        theme,
        viewport,
        &fixture_area.path().join(scene.id()),
    )?;
    ensure(
        repeated.as_raw() == expected.as_raw(),
        "same-process representative render was not deterministic",
    )?;

    let manifest = CaptureManifest {
        schema: 1,
        generator: MANIFEST_GENERATOR,
        pixels_per_point: PIXELS_PER_POINT as u32,
        captures: receipts,
    };
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| format!("could not serialize capture manifest: {error}"))?;
    bytes.push(b'\n');
    publish_manifest(output, &bytes)?;
    println!(
        "wrote 28 deterministic documentation captures and {} to {}",
        MANIFEST_FILE,
        output.display()
    );
    Ok(())
}

fn render_variant(
    renderer: &mut HeadlessRenderer,
    scene: Scene,
    theme: CaptureTheme,
    viewport: CaptureViewport,
    fixture_root: &Path,
) -> Result<RgbaImage, String> {
    let fixture = build_scene(scene, theme, fixture_root)?;
    let collector = TextureCollector::default();
    let texture_handle = collector.clone();
    let mut fonts_installed = false;
    let [width, height] = viewport.size();
    let mut harness = Harness::builder()
        .with_size(egui::vec2(width as f32, height as f32))
        .with_pixels_per_point(PIXELS_PER_POINT)
        .with_theme(theme.egui())
        .with_os(egui::os::OperatingSystem::Windows)
        .with_max_steps(12)
        .renderer(collector)
        .build_ui_state(
            move |ui, app: &mut EsoWeaveApp| {
                if !fonts_installed {
                    eso_weave::app::theme::install_fonts(ui.ctx());
                    fonts_installed = true;
                    return;
                }
                app.frame_ui(ui);
            },
            fixture.app,
        );
    harness.run_steps(SETTLE_FRAMES);
    let mut textures = texture_handle.take()?;
    renderer.update_textures(&mut textures);
    let image = renderer.render(&harness.ctx, harness.output())?;
    ensure_action_channel_empty(&fixture.action_rx, scene)?;
    Ok(image)
}

fn build_scene(
    scene: Scene,
    theme: CaptureTheme,
    fixture_root: &Path,
) -> Result<SceneFixture, String> {
    prepare_beacon_fixture(scene, fixture_root)?;
    let mut settings = Settings {
        beacon: beacon::prefs_to_value(&BeaconPrefs {
            path_override: Some(fixture_root.to_path_buf()),
            environment: Environment::Live,
        }),
        ui: serde_json::json!({
            "theme": theme.id(),
            "always_on_top": false,
            "log_panel_height": 160,
            "system_state_expanded": true
        }),
        ..Settings::default()
    };

    let mut weave_config = WeaveConfig::default();
    if scene == Scene::WeavingConfiguration {
        weave_config.timing.d_weave = 75;
        weave_config.timing.d_heavy = 900;
        weave_config.timing.d_bash = 140;
        weave_config.slots[0].weave_type = WeaveType::LightAttack;
        weave_config.slots[0].overrides.d_weave = Some(65);
        weave_config.slots[1].weave_type = WeaveType::HeavyAttack;
        weave_config.slots[1].overrides.d_heavy = Some(850);
        weave_config.slots[2].weave_type = WeaveType::BashAttack;
        weave_config.slots[2].overrides.d_bash = Some(135);
        weave_config.slots[3].weave_type = WeaveType::BlockCasting;
        weave_config.slots[4].active = false;
    }

    let potion_config = AutoPotionConfig {
        health: ResourceWatch {
            enabled: true,
            threshold: 35,
        },
        magicka: ResourceWatch {
            enabled: true,
            threshold: 25,
        },
        stamina: ResourceWatch {
            enabled: false,
            threshold: 30,
        },
        ..AutoPotionConfig::default()
    };
    if matches!(scene, Scene::AutoPotionReady | Scene::AutoPotionBlocked) {
        settings.potion = potion_config.store();
    }

    let (input, action_rx) = InputEngine::new(BindingTable::default(), 16);
    let input = Arc::new(input);
    let mut weave_engine = WeaveEngine::new(weave_config);
    seed_weave_observations(&mut weave_engine, scene.active_game());
    let weave = Arc::new(Mutex::new(weave_engine));
    let fishing = Arc::new(Mutex::new(FishingController::new(FishingConfig::default())));
    let mut potion_controller = AutoPotionController::new(potion_config);
    seed_potion_state(&mut potion_controller, scene)?;
    let potion = Arc::new(Mutex::new(potion_controller));
    let (_dispatch, log) = logging::build(&LoggingPrefs::default(), PathBuf::from("fixture-log"));
    let (reader_update_tx, _reader_update_rx) = mpsc::channel();
    let mut model = AppModel::new(
        input,
        weave,
        fishing,
        Box::new(MockFishingSink::new()),
        potion,
        log,
        reader_update_tx,
        settings,
        None,
        Instant::now(),
    );
    model.set_catalog(CatalogAccess::open_or_empty(
        repository_root().join("assets/catalog/catalog.sqlite"),
    ));
    seed_game_observations(&model, scene);
    let view = model.view();
    validate_scene_view(scene, &view)?;

    let (_toggle_tx, toggle_rx) = mpsc::channel();
    let (_api_tx, api_rx) = mpsc::channel();
    Ok(SceneFixture {
        app: EsoWeaveApp::new(model, toggle_rx, api_rx, None),
        action_rx,
    })
}

fn prepare_beacon_fixture(scene: Scene, root: &Path) -> Result<(), String> {
    fs::create_dir_all(root)
        .map_err(|error| format!("could not create synthetic fixture root: {error}"))?;
    if scene.needs_managed_beacon() {
        let addon = root.join(beacon::SUBFOLDER);
        fs::create_dir_all(&addon)
            .map_err(|error| format!("could not create managed status fixture: {error}"))?;
        fs::write(
            addon.join(beacon::MANIFEST_FILE),
            beacon::MANIFEST.as_bytes(),
        )
        .map_err(|error| format!("could not write managed status fixture: {error}"))?;
    } else if scene == Scene::PixelBeaconUnmanaged {
        let addon = root.join(beacon::SUBFOLDER);
        fs::create_dir_all(&addon)
            .map_err(|error| format!("could not create unmanaged status fixture: {error}"))?;
        fs::write(
            addon.join(beacon::MANIFEST_FILE),
            b"## Title: Community PixelBeacon\n## Version: 1\n",
        )
        .map_err(|error| format!("could not write unmanaged status fixture: {error}"))?;
    }
    Ok(())
}

fn seed_weave_observations(engine: &mut WeaveEngine, active: bool) {
    if !active {
        return;
    }
    engine.set_weapon_bar(WeaponBarSignal {
        bar: ActiveBar::Front,
        front: WeaponClass::DualWield,
        back: WeaponClass::DestructionStaff,
    });
    engine.set_combat(CombatSignal::OutOfCombat);
    engine.set_movement(MovementSignal::OnFoot);
    engine.set_life(LifeState::Alive);
    engine.set_roll_dodge(RollDodgeState::Inactive);
    engine.set_world(WorldState::Active);
    engine.set_travel(TravelState::Inactive);
    engine.set_resources(healthy_resources());
    engine.set_ultimate(UltimateTelemetry {
        current: UltimateValue::Points(185),
        maximum: UltimateValue::Points(500),
        front_cost: UltimateValue::Points(200),
        back_cost: UltimateValue::Points(125),
    });
    engine.set_cooldowns(CooldownSet {
        skill_1: SlotCooldown::Ready,
        skill_2: SlotCooldown::RemainingMs(350),
        skill_3: SlotCooldown::Ready,
        skill_4: SlotCooldown::Ready,
        skill_5: SlotCooldown::RemainingMs(1200),
        ultimate: SlotCooldown::Ready,
    });
    engine.set_quickslot(ready_quickslot());
}

fn seed_potion_state(controller: &mut AutoPotionController, scene: Scene) -> Result<(), String> {
    if !matches!(scene, Scene::AutoPotionReady | Scene::AutoPotionBlocked) {
        return Ok(());
    }
    controller.set_game_active(true);
    controller.set_focused(true);
    controller.set_gated(false);
    controller.set_life_state(LifeState::Alive);
    controller.set_world_state(WorldState::Active);
    controller.set_travel_state(TravelState::Inactive);
    controller.set_movement(MovementSignal::OnFoot);
    if scene == Scene::AutoPotionReady {
        controller.on_heartbeat();
    }
    controller.set_enabled(true);
    let mut sink = MockAutoPotionSink::new();
    let state = controller.tick(
        PotionReadings {
            resources: healthy_resources(),
            quickslot: ready_quickslot(),
        },
        10_000,
        &mut sink,
    );
    let expected = if scene == Scene::AutoPotionReady {
        AutoPotionState::Ready
    } else {
        AutoPotionState::Blocked(BlockReason::BeaconUnavailable)
    };
    ensure(state == expected, "auto-potion fixture state drifted")?;
    ensure(
        sink.ops.is_empty(),
        "fixture evaluation emitted synthesized input",
    )
}

fn seed_game_observations(model: &AppModel, scene: Scene) {
    let game = model.game_state();
    if !scene.active_game() {
        game.update_processes(ProcessObservation {
            game: Presence::Absent,
            launcher: Presence::Absent,
            focus: FocusObservation::Unknown,
        });
        game.update_installation(InstallationState::NotDetected);
        return;
    }
    game.update_installation(InstallationState::Detected(InstallationCandidate {
        provider: InstallationProvider::Steam,
        root: PathBuf::from("fixture-game-install"),
        source: CandidateSource::SteamManifest,
    }));
    game.update_processes(ProcessObservation {
        game: Presence::Present,
        launcher: Presence::Absent,
        focus: FocusObservation::Focused,
    });
    game.observe_heartbeat();
    game.observe_surface(SurfaceObservation::Observed(MenuSurface::None));
    game.observe_world(WorldState::Active);
    if scene == Scene::PixelBeaconLost || scene == Scene::AutoPotionBlocked {
        game.signal_lost();
    }
}

fn validate_scene_view(scene: Scene, view: &AppView) -> Result<(), String> {
    ensure(
        view.status_line.state_text == "Active",
        "all capture scenes must show the companion running",
    )?;
    ensure(
        view.catalog_line.role == StatusRole::Healthy,
        "capture scenes require the checked-in catalog",
    )?;
    match scene {
        Scene::FirstLaunch => {
            ensure(
                view.runtime_line.state_text == "Inactive",
                "first-launch runtime",
            )?;
            ensure(
                view.beacon_condition == BeaconCondition::NotInstalled,
                "first-launch beacon condition",
            )?;
        }
        Scene::HealthySystemState => {
            validate_healthy_view(view)?;
            ensure(
                view.resources.health.presentation == ResourcePresentation::Observed(82),
                "healthy scene resource projection",
            )?;
        }
        Scene::PixelBeaconLost => {
            ensure(
                view.runtime_line.state_text == "Active",
                "lost-signal runtime",
            )?;
            ensure(
                view.beacon_signal_line.state_text == "Signal lost",
                "lost-signal evidence",
            )?;
            ensure(
                view.resources.health.presentation == ResourcePresentation::Observed(82),
                "lost scene retains the deliberately seeded engine observation",
            )?;
        }
        Scene::PixelBeaconUnmanaged => {
            ensure(
                view.beacon_condition == BeaconCondition::Unmanaged,
                "unmanaged beacon condition",
            )?;
            ensure(
                !view.uninstall_enabled,
                "unmanaged addon must not enable removal",
            )?;
        }
        Scene::WeavingConfiguration => {
            validate_healthy_view(view)?;
            ensure(
                view.skills[0].weave_type == WeaveType::LightAttack
                    && view.skills[0].effective_delay == 65
                    && view.skills[0].is_override,
                "weaving scene slot 1 contract",
            )?;
            ensure(
                view.skills[1].weave_type == WeaveType::HeavyAttack
                    && view.skills[1].effective_delay == 850,
                "weaving scene slot 2 contract",
            )?;
            ensure(
                view.skills[2].weave_type == WeaveType::BashAttack
                    && view.skills[2].effective_delay == 135,
                "weaving scene slot 3 contract",
            )?;
        }
        Scene::AutoPotionReady => {
            validate_healthy_view(view)?;
            ensure(view.auto_potion_requested, "ready scene request state")?;
            ensure(
                view.auto_potion.text == "Ready",
                "ready scene effective state",
            )?;
        }
        Scene::AutoPotionBlocked => {
            ensure(view.auto_potion_requested, "blocked scene request state")?;
            ensure(
                view.auto_potion.text == "Blocked: beacon unavailable",
                "blocked scene effective state",
            )?;
            ensure(
                view.beacon_signal_line.state_text == "Signal lost",
                "blocked scene signal evidence",
            )?;
        }
    }
    Ok(())
}

fn validate_healthy_view(view: &AppView) -> Result<(), String> {
    ensure(view.runtime_line.state_text == "Active", "healthy runtime")?;
    ensure(
        view.installation_line.state_text == "Detected (Steam)",
        "healthy installation",
    )?;
    ensure(
        view.beacon_condition == BeaconCondition::InstalledCurrent,
        "healthy addon status",
    )?;
    ensure(
        view.beacon_signal_line.state_text == "Signal detected",
        "healthy signal",
    )?;
    ensure(view.menu.state == "Gameplay", "healthy game context")?;
    ensure(view.world.state == "Active", "healthy world state")
}

fn healthy_resources() -> ResourceSet {
    ResourceSet {
        health: ResourceLevel::Percent(82),
        stamina: ResourceLevel::Percent(64),
        magicka: ResourceLevel::Percent(91),
    }
}

fn ready_quickslot() -> QuickslotState {
    QuickslotState {
        classification: QuickslotClassification::Potion(QuickslotPotionAvailability::Usable),
        cooldown: SlotCooldown::Ready,
        item_id: Some(27036),
    }
}

fn ensure_action_channel_empty(receiver: &ActionReceiver, scene: Scene) -> Result<(), String> {
    match receiver.try_recv() {
        Err(TryRecvError::Empty) => Ok(()),
        Err(TryRecvError::Disconnected) => Err(format!(
            "{} action channel disconnected before isolation was checked",
            scene.id()
        )),
        Ok(action) => Err(format!(
            "{} emitted unexpected input action {action:?}",
            scene.id()
        )),
    }
}

fn variant_filename(scene: Scene, theme: CaptureTheme, viewport: CaptureViewport) -> String {
    format!("{}--{}--{}.png", scene.id(), theme.id(), viewport.id())
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn publish_manifest(output: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut candidate = tempfile::NamedTempFile::new_in(output)
        .map_err(|error| format!("could not stage capture manifest: {error}"))?;
    use std::io::Write as _;
    candidate
        .write_all(bytes)
        .and_then(|()| candidate.as_file().sync_all())
        .map_err(|error| format!("could not stage capture manifest: {error}"))?;
    let destination = output.join(MANIFEST_FILE);
    if destination.exists() {
        fs::remove_file(&destination)
            .map_err(|error| format!("could not replace capture manifest: {error}"))?;
    }
    candidate
        .persist(&destination)
        .map_err(|error| format!("could not publish capture manifest: {}", error.error))?;
    Ok(())
}

fn clear_completion_marker(output: &Path) -> Result<(), String> {
    let destination = output.join(MANIFEST_FILE);
    match fs::remove_file(&destination) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "could not clear prior capture manifest {}: {error}",
            destination.display()
        )),
    }
}

fn resolve_output_root(argument: &Path) -> Result<PathBuf, String> {
    let repository = repository_root()
        .canonicalize()
        .map_err(|error| format!("could not resolve repository root: {error}"))?;
    let candidate = if argument.is_absolute() {
        lexical_normalize(argument)
    } else {
        lexical_normalize(&repository.join(argument))
    };
    ensure(
        candidate.starts_with(&repository) && candidate != repository,
        "capture output must be a subdirectory of the repository",
    )?;
    reject_symlink_chain(&repository, &candidate)?;
    if candidate.exists() && !candidate.is_dir() {
        return Err("capture output exists and is not a directory".to_string());
    }
    fs::create_dir_all(&candidate)
        .map_err(|error| format!("could not create capture output: {error}"))?;
    let resolved = candidate
        .canonicalize()
        .map_err(|error| format!("could not resolve capture output: {error}"))?;
    ensure(
        resolved.starts_with(&repository) && resolved != repository,
        "resolved capture output escaped the repository",
    )?;
    reject_symlink_chain(&repository, &resolved)?;
    Ok(resolved)
}

fn reject_symlink_chain(repository: &Path, candidate: &Path) -> Result<(), String> {
    let relative = candidate
        .strip_prefix(repository)
        .map_err(|_| "capture output escaped the repository".to_string())?;
    let mut current = repository.to_path_buf();
    for component in relative.components() {
        current.push(component.as_os_str());
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(format!(
                    "capture output cannot traverse symlink {}",
                    current.display()
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "could not inspect capture output {}: {error}",
                    current.display()
                ));
            }
        }
    }
    Ok(())
}

fn lexical_normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("could not read {}: {error}", path.display()))
}

fn ensure(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_string())
    }
}
