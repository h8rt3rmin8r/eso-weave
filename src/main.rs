//! ESO Weave binary entry point.
//!
//! Resolves platform directories, loads settings, initializes logging, builds
//! the shared subsystems, spawns the interception, weave-worker, and pixel-bus
//! worker threads, and runs the eframe GUI on the main thread. The GUI and the
//! worker threads share the subsystems; the input backend keeps its own thread
//! and message pump (the S002 contract) while eframe owns the main event loop.

// Release builds target the Windows subsystem so the GUI carries no console
// window; debug builds keep the console so developers see stdout and logs during
// the dev loop.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use eso_weave::app::{route_reader_event, ui::EsoWeaveApp, AppModel};
use eso_weave::config::{self, LoadOutcome};
use eso_weave::fishing::{FishingConfig, FishingController, RealFishingSink};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::{InputBackend, InputEngine, InputError, Key, MouseButton, Transition};
use eso_weave::pixelbus::{self, PixelBusReader, SurfaceSampler};
use eso_weave::weave::{RealSink, WeaveConfig, WeaveEngine};
use eso_weave::{logging, platform, version};

mod startup;

/// The exact ESO window title the backends and samplers resolve.
const WINDOW_TITLE: &str = "Elder Scrolls Online";

fn main() {
    // Install the startup panic hook first, before any fallible work, so a
    // failure is surfaced even though the release build has no console. Panics
    // before logging is ready still raise the dialog; the log line is best effort
    // until logging::init runs a few lines below.
    let gui_started = startup::install_hook(Box::new(startup::DialogNotifier));

    let config_dir = platform::config_dir();
    let outcome = match &config_dir {
        Some(dir) => config::load(dir),
        None => LoadOutcome::default(),
    };
    let settings = outcome.settings;

    let log = logging::init(&settings.logging, platform::log_dir().unwrap_or_default());
    for notice in &outcome.notices {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }
    tracing::info!(
        target: "eso_weave",
        "eso-weave {} started (schema_version={})",
        version(),
        settings.schema_version
    );

    // Input engine and its shared backend.
    let (bindings, binding_notices) = BindingTable::from_settings_map(&settings.bindings);
    for notice in &binding_notices {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }
    let (engine, actions) = InputEngine::new(bindings, 64);
    let input = Arc::new(engine);
    let backend = Arc::new(make_backend());

    // Weave engine, loaded from settings and synced to the bindings.
    let mut weave_engine = WeaveEngine::new(WeaveConfig::default());
    for notice in weave_engine.load(&settings) {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }
    weave_engine.config_mut().sync_keys(&input.bindings());
    weave_engine.apply_activity(&input);
    let weave = Arc::new(Mutex::new(weave_engine));

    // Fishing controller.
    let mut fishing_notices = Vec::new();
    let fishing_config = FishingConfig::load(&settings.fishing, &mut fishing_notices);
    for notice in &fishing_notices {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }
    let fishing = Arc::new(Mutex::new(FishingController::new(fishing_config)));

    // Pixel bus reader configuration.
    let mut reader_notices = Vec::new();
    let reader_config = pixelbus::load_reader_config(&settings.pixelbus, &mut reader_notices);
    for notice in &reader_notices {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }

    // Interception thread: the backend runs its own event loop (S002 contract).
    {
        let backend = backend.clone();
        let engine = input.clone();
        thread::spawn(move || {
            if let Err(err) = backend.run(engine) {
                tracing::warn!(target: "eso_weave::input", "interception ended: {err}");
            }
        });
    }

    // Weave worker: drains handed-off actions and runs sequences through the backend.
    {
        let backend = backend.clone();
        let weave = weave.clone();
        thread::spawn(move || {
            let mut sink = RealSink::new(SharedBackend(backend));
            while let Ok(action) = actions.recv() {
                weave.lock().unwrap().handle(action, &mut sink);
            }
        });
    }

    // Pixel bus worker: samples the reader and routes events to the subsystems.
    {
        let backend = backend.clone();
        let weave = weave.clone();
        let fishing = fishing.clone();
        thread::spawn(move || {
            let mut reader = PixelBusReader::new(reader_config);
            let mut sink = RealFishingSink::new(SharedBackend(backend));
            let mut sampler = resolve_sampler();
            let origin = Instant::now();
            loop {
                thread::sleep(Duration::from_millis(reader_config.interval_idle_ms));
                if sampler.is_none() {
                    sampler = resolve_sampler();
                }
                let Some(active) = sampler.as_ref() else {
                    continue;
                };
                let now = origin.elapsed().as_millis() as u64;
                let events = reader.sample_and_observe(active.as_ref(), now);
                let mut weave = weave.lock().unwrap();
                let mut fishing = fishing.lock().unwrap();
                for event in events {
                    route_reader_event(event, &mut weave, &mut fishing, now, &mut sink);
                }
                fishing.tick(now, &mut sink);
            }
        });
    }

    // GUI on the main thread.
    let gui_sink = Box::new(RealFishingSink::new(SharedBackend(backend.clone())));
    // Load persisted session state before the config directory is moved into the
    // model, so the live suspend and fishing intents can be restored on launch.
    let session = config_dir
        .as_ref()
        .map(|dir| eso_weave::config::state::load(dir));
    let mut model = AppModel::new(
        input.clone(),
        weave.clone(),
        fishing.clone(),
        gui_sink,
        log.clone(),
        settings,
        config_dir,
    );
    if let Some((state, notices)) = session {
        for notice in &notices {
            tracing::warn!(target: "eso_weave::config", "{}", notice.message);
        }
        model.restore_session(state);
    }

    // The GUI is about to take over the main thread; from here on a panic is
    // logged but no longer raises a dialog (a mid-session worker panic should not
    // pop a message box).
    gui_started.store(true, std::sync::atomic::Ordering::SeqCst);

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([600.0, 720.0])
        .with_min_inner_size([480.0, 420.0]);
    if let Some(icon) = window_icon() {
        viewport = viewport.with_icon(icon);
    }
    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    if let Err(err) = eframe::run_native(
        "ESO Weave",
        native_options,
        Box::new(|cc| {
            eso_weave::app::theme::install_fonts(&cc.egui_ctx);
            Ok(Box::new(EsoWeaveApp::new(model)))
        }),
    ) {
        tracing::error!(target: "eso_weave", "GUI exited with error: {err}");
    }
}

/// Decodes the bundled window icon into an egui icon for the title bar and
/// taskbar. Returns `None` if it cannot be decoded, in which case the window
/// simply carries no custom icon.
fn window_icon() -> Option<eframe::egui::IconData> {
    let bytes = include_bytes!("../assets/brand/window-icon-256.png");
    match image::load_from_memory(bytes) {
        Ok(img) => {
            let img = img.to_rgba8();
            let (width, height) = img.dimensions();
            Some(eframe::egui::IconData {
                rgba: img.into_raw(),
                width,
                height,
            })
        }
        Err(err) => {
            tracing::warn!(target: "eso_weave", "window icon decode failed: {err}");
            None
        }
    }
}

/// A shareable adapter over the input backend, so the interception thread, the
/// weave worker, the pixel bus worker, and the GUI all synthesize through the
/// same backend (self-originated marking stays consistent).
struct SharedBackend<B>(Arc<B>);

impl<B: InputBackend> InputBackend for SharedBackend<B> {
    fn synthesize(&self, key: Key, transition: Transition) -> Result<(), InputError> {
        self.0.synthesize(key, transition)
    }

    fn synthesize_mouse(
        &self,
        button: MouseButton,
        transition: Transition,
    ) -> Result<(), InputError> {
        self.0.synthesize_mouse(button, transition)
    }

    fn run(&self, engine: Arc<InputEngine>) -> Result<(), InputError> {
        self.0.run(engine)
    }
}

#[cfg(windows)]
fn make_backend() -> eso_weave::input::WindowsBackend {
    eso_weave::input::WindowsBackend::new(WINDOW_TITLE)
}

#[cfg(target_os = "linux")]
fn make_backend() -> eso_weave::input::LinuxBackend {
    eso_weave::input::LinuxBackend::new(WINDOW_TITLE)
}

#[cfg(windows)]
fn resolve_sampler() -> Option<Box<dyn SurfaceSampler>> {
    eso_weave::pixelbus::GdiSampler::for_window(WINDOW_TITLE)
        .map(|sampler| Box::new(sampler) as Box<dyn SurfaceSampler>)
}

#[cfg(target_os = "linux")]
fn resolve_sampler() -> Option<Box<dyn SurfaceSampler>> {
    Some(Box::new(eso_weave::pixelbus::X11Sampler::for_window(
        WINDOW_TITLE,
    )))
}
