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

use eso_weave::app::{
    route_game_observation, route_reader_event, route_reader_safety_gate, ui::EsoWeaveApp, AppModel,
};
use eso_weave::config::state::{sanitize_geometry, RestoreBounds};
use eso_weave::config::{self, LoadOutcome};
use eso_weave::fishing::{FishingConfig, FishingController, FishingState, RealFishingSink};
use eso_weave::game::{FocusObservation, GameRuntime, GameState};
use eso_weave::input::bindings::BindingTable;
use eso_weave::input::{
    Action, InputBackend, InputEngine, InputError, Key, MouseButton, Transition,
};
use eso_weave::pixelbus::{self, poll_interval, PixelBusReader, SurfaceSampler};
use eso_weave::potion::{AutoPotionConfig, AutoPotionController, RealAutoPotionSink};
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
    let fishing = Arc::new(Mutex::new(FishingController::with_life_gate(
        fishing_config,
        input.life_gate(),
    )));

    // Auto-potion controller. Always starts switched off, deliberately, even
    // though suspend and fishing are both restored from the previous session: a
    // restored auto-potion would wait silently to press a key days later, in a
    // fight the operator does not associate with this application.
    let mut potion_notices = Vec::new();
    let potion_config = AutoPotionConfig::load(&settings.potion, &mut potion_notices);
    for notice in &potion_notices {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }
    let potion = Arc::new(Mutex::new(AutoPotionController::new(potion_config)));
    let game = GameState::default();

    // Pixel bus reader configuration.
    let mut reader_notices = Vec::new();
    let reader_config = pixelbus::load_reader_config(&settings.pixelbus, &mut reader_notices);
    for notice in &reader_notices {
        tracing::warn!(target: "eso_weave::config", "{}", notice.message);
    }

    // The game's stored video settings, used by the out-of-band display
    // detection as a cross-check against what the operating system reports and
    // as a pre-launch fallback. Resolved once here rather than per sampling
    // iteration; changing the AddOns override mid-session is therefore picked up
    // on the next launch, which is acceptable because this path is never the
    // authority for a live window. Resolution creates nothing and the file is
    // only ever read.
    let user_settings_path = eso_weave::beacon::resolve_addons_dir(
        &eso_weave::beacon::prefs_from_value(&settings.beacon),
    )
    .ok()
    .and_then(|addons| eso_weave::beacon::user_settings_path(&addons));

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

    // App-toggle channel: the two hotkey toggles (suspend, fishing) are carried
    // from the weave worker to the GUI so a hotkey and its button reach one shared
    // state through the same intent path (feature 015).
    let (toggle_tx, toggle_rx) = std::sync::mpsc::channel::<Action>();

    // Weave worker: drains handed-off actions and runs sequences through the
    // backend. Application-level toggles are not weave actions, so they are
    // forwarded to the GUI intent path instead of the weave engine.
    {
        let backend = backend.clone();
        let weave = weave.clone();
        let input = input.clone();
        thread::spawn(move || {
            let weave_gates = input.weave_gates();
            let mut sink = RealSink::new(SharedBackend(backend), weave_gates.clone());
            while let Ok(action) = actions.recv() {
                if action.is_app_toggle() {
                    // A send error means the GUI receiver is gone (the app is
                    // exiting); dropping the toggle is the correct response.
                    let _ = toggle_tx.send(action);
                } else {
                    if weave_gates.is_gated() {
                        continue;
                    }
                    let mut weave = weave.lock().unwrap();
                    if weave_gates.is_gated() {
                        continue;
                    }
                    weave.handle(action, &mut sink);
                }
            }
        });
    }

    // Shared monotonic clock: the GUI intent path stamps fishing deadlines and
    // the pixel-bus worker evaluates them, so both must read the same origin or a
    // deadline could be judged against a different timeline than it was set on.
    let clock_origin = Instant::now();

    // Pixel bus worker: samples the reader and routes events to the subsystems.
    {
        let backend = backend.clone();
        let weave = weave.clone();
        let fishing = fishing.clone();
        let potion = potion.clone();
        let input = input.clone();
        let game = game.clone();
        thread::spawn(move || {
            let mut reader = PixelBusReader::new(reader_config);
            let mut sink = RealFishingSink::new(SharedBackend(backend.clone()));
            // Auto-potion synthesizes through its own sink over the same backend,
            // preserving recursion flagging. Focus is pushed into the controller
            // explicitly because autonomous synthesis bypasses interception.
            let mut potion_sink = RealAutoPotionSink::new(SharedBackend(backend));
            let mut sampler = None;
            // Display detection rides this loop: no new thread and no new timer.
            // It is change-detected, so a stationary window costs nothing beyond
            // the operating system queries the capture already performs.
            let mut display = pixelbus::DisplayDetector::new();
            let origin = clock_origin;
            let mut next_game_probe_ms = 0;
            loop {
                // Poll fast while a fishing session is active so transient cast
                // and bite signals are sampled and the state machine ticks in
                // time; poll slowly otherwise.
                let fishing_active = fishing.lock().unwrap().state() != FishingState::Disabled;
                // A suspended application intercepts and synthesizes nothing, so
                // it has no menu gate to keep current and can sample slowly.
                let can_intercept = !input.is_suspended() && input.is_game_active();
                let reader_interval_ms =
                    poll_interval(fishing_active, can_intercept, &reader_config);
                let before_sleep = origin.elapsed().as_millis() as u64;
                let sleep_ms = eso_weave::game::runtime_probe_delay_ms(
                    reader_interval_ms,
                    before_sleep,
                    next_game_probe_ms,
                );
                if sleep_ms > 0 {
                    thread::sleep(Duration::from_millis(sleep_ms));
                }
                let now = origin.elapsed().as_millis() as u64;
                if now >= next_game_probe_ms {
                    next_game_probe_ms = now.saturating_add(1000);
                    let before = game.snapshot().runtime;
                    let installation = eso_weave::game::discover_installation();
                    if game.update_installation(installation.clone()) {
                        let (state, provider) = match &installation {
                            eso_weave::game::InstallationState::NotDetected => {
                                ("not-detected", None)
                            }
                            eso_weave::game::InstallationState::Ambiguous => ("ambiguous", None),
                            eso_weave::game::InstallationState::Unknown => ("unknown", None),
                            eso_weave::game::InstallationState::Detected(candidate) => {
                                ("detected", Some(candidate.provider))
                            }
                        };
                        tracing::info!(
                            target: "eso_weave::game",
                            state,
                            provider = ?provider,
                            "game installation observation changed"
                        );
                    }
                    let processes = eso_weave::game::observe_processes();
                    let process_changed = game.update_processes(processes);
                    let after = processes.runtime();
                    let active = after == GameRuntime::Active;
                    let focused = matches!(processes.focus, FocusObservation::Focused);
                    input.set_game_active(active);
                    input.set_focused(focused);
                    fishing
                        .lock()
                        .unwrap()
                        .set_game_environment(active, focused, now, &mut sink);
                    {
                        let mut potion = potion.lock().unwrap();
                        potion.set_game_active(active);
                        potion.set_focused(focused);
                    }
                    if process_changed {
                        tracing::info!(
                            target: "eso_weave::game",
                            runtime = ?after,
                            focus = ?processes.focus,
                            "game runtime observation changed"
                        );
                    }
                    if before == GameRuntime::Active && !active {
                        sampler = None;
                        reader.reset();
                        weave.lock().unwrap().clear_game_observations();
                    } else if before != GameRuntime::Active && active {
                        sampler = None;
                        reader.reset();
                    }
                }
                if game.snapshot().runtime != GameRuntime::Active {
                    continue;
                }
                if sampler.is_none() {
                    sampler = resolve_sampler();
                }
                // Resolved before the sampler check, because the absence of a
                // window is exactly when the stored-settings fallback matters.
                let measured = sampler.as_ref().and_then(|active| active.display());
                let measured_surface = measured.map(|value| value.surface);
                if let Some(update) = display.update(measured, || {
                    let path = user_settings_path.as_ref()?;
                    std::fs::read_to_string(path)
                        .ok()
                        .map(|text| pixelbus::parse_user_settings(&text))
                }) {
                    log_display_update(&update);
                }
                let Some(active) = sampler.as_ref() else {
                    continue;
                };
                let events =
                    reader.sample_and_observe_with_surface(active.as_ref(), now, measured_surface);
                // Close or open the independently shared gate before waiting on
                // a weave that may currently be inside a timed sequence.
                for event in &events {
                    route_reader_safety_gate(*event, &input);
                }
                let mut weave = weave.lock().unwrap();
                let mut fishing = fishing.lock().unwrap();
                let mut potion = potion.lock().unwrap();
                for event in events {
                    route_game_observation(event, &game);
                    route_reader_event(
                        event,
                        &mut weave,
                        &mut fishing,
                        &mut potion,
                        &input,
                        now,
                        &mut sink,
                    );
                }
                fishing.tick(now, &mut sink);
                // Auto-potion rides this same loop: no new thread, no new timer,
                // and nothing on the input hook thread. The suspend state is
                // pushed in rather than read inside the rule, so the rule stays a
                // pure function of its inputs.
                potion.set_suspended(input.is_suspended());
                let _ = potion.tick(
                    eso_weave::potion::PotionReadings {
                        resources: weave.resources(),
                        quickslot: weave.quickslot(),
                    },
                    now,
                    &mut potion_sink,
                );
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

    // Capture what the startup API version check needs before `settings` and
    // `session` are moved into the model: the AddOns preferences and the stored
    // last known API version and last seen game version.
    let api_beacon_prefs = eso_weave::beacon::prefs_from_value(&settings.beacon);
    let (stored_api_version, stored_game_version) = session
        .as_ref()
        .map(|(state, _)| {
            (
                state.api_version.last_known_api_version,
                state.api_version.last_seen_game_version,
            )
        })
        .unwrap_or((None, None));

    // The restored window geometry (if any), captured before `session` is moved
    // into the model, used both to build the viewport and to seed the GUI's
    // change detection so an unchanged restored window is not re-saved.
    let restored_geometry = session.as_ref().and_then(|(state, _)| state.window);
    let (api_tx, api_rx) = std::sync::mpsc::channel();
    let mut model = AppModel::new_with_game(
        input.clone(),
        weave.clone(),
        fishing.clone(),
        gui_sink,
        potion.clone(),
        game,
        log.clone(),
        settings,
        config_dir,
        clock_origin,
    );
    if let Some((state, notices)) = session {
        for notice in &notices {
            tracing::warn!(target: "eso_weave::config", "{}", notice.message);
        }
        model.restore_session(state);
    }

    // Startup ESO API version check: runs off the GUI thread, keeps the on-disk
    // manifest current (marker-gated, never downgrading), detects a client bump,
    // and hands the values to persist back to the GUI. Never blocks the window.
    thread::spawn(move || {
        let addons = eso_weave::beacon::resolve_addons_dir(&api_beacon_prefs).ok();
        let source = eso_weave::beacon::api_check::GithubLiveSource::default();
        let outcome = eso_weave::beacon::api_check::run_check(
            &source,
            addons.as_deref(),
            stored_api_version,
            stored_game_version,
        );
        // A send error means the GUI receiver is gone (the app is exiting); the
        // outcome is simply dropped.
        let _ = api_tx.send(outcome);
    });

    // The GUI is about to take over the main thread; from here on a panic is
    // logged but no longer raises a dialog (a mid-session worker panic should not
    // pop a message box).
    gui_started.store(true, std::sync::atomic::Ordering::SeqCst);

    // Default size the window falls back to when no geometry is restored, and the
    // boot minimum used before the content extent is measured. Once the GUI lays
    // out, the authoritative minimum is derived from the actual content extent and
    // pushed via ViewportCommand::MinInnerSize (see src/app/ui.rs, issue #4), so
    // this constant is only the pre-measurement floor and no longer needs manual
    // re-tuning when a UI row is added. Keep it in sync with BOOT_MIN_SIZE there.
    const DEFAULT_SIZE: [f32; 2] = [600.0, 720.0];
    const MIN_SIZE: [f32; 2] = [480.0, 420.0];
    let mut viewport = eframe::egui::ViewportBuilder::default().with_min_inner_size(MIN_SIZE);
    if let Some(geo) = restored_geometry {
        let virtual_screen = platform::virtual_screen_bounds_points();
        // Cap the restored size at the desktop extent when known, else a generous
        // fallback; the pure helper clamps and validates the recorded values.
        let (max_w, max_h) = virtual_screen
            .map(|(_, _, w, h)| (w as u32, h as u32))
            .unwrap_or((8000, 8000));
        let restore = sanitize_geometry(
            geo,
            RestoreBounds {
                min_w: MIN_SIZE[0] as u32,
                min_h: MIN_SIZE[1] as u32,
                max_w,
                max_h,
                virtual_screen,
            },
        );
        viewport = viewport.with_inner_size([restore.width as f32, restore.height as f32]);
        if let Some((x, y)) = restore.position {
            viewport = viewport.with_position([x as f32, y as f32]);
        }
        if restore.maximized {
            viewport = viewport.with_maximized(true);
        }
    } else {
        viewport = viewport.with_inner_size(DEFAULT_SIZE);
    }
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
            Ok(Box::new(EsoWeaveApp::new(
                model,
                toggle_rx,
                api_rx,
                restored_geometry,
            )))
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

/// Records a display change at debug level.
///
/// Only reached when something actually changed, so this can log
/// unconditionally without flooding: a stationary window produces no lines at
/// all. The reconciliation line is the interesting one. When it reports which
/// stored resolution pair the measured surface matched, alongside the raw
/// window-mode value, that pairing is evidence about what the game's unmapped
/// mode enum means on this install, accumulated from ordinary use. Nothing acts
/// on it.
fn log_display_update(update: &eso_weave::pixelbus::DisplayUpdate) {
    match &update.descriptor {
        Some(descriptor) => tracing::debug!(
            target: "eso_weave::pixelbus",
            surface = format!("{}x{}", descriptor.surface.width, descriptor.surface.height),
            origin = ?descriptor.surface_origin,
            display_origin = ?descriptor.display_origin,
            display_size = ?descriptor.display_size,
            dpi = ?descriptor.dpi,
            source = ?descriptor.source,
            "display resolved"
        ),
        None => tracing::debug!(
            target: "eso_weave::pixelbus",
            "display no longer resolvable"
        ),
    }
    if let Some(reconciliation) = &update.reconciliation {
        tracing::debug!(
            target: "eso_weave::pixelbus",
            outcome = ?reconciliation,
            "display reconciled against stored video settings"
        );
    }
}

#[cfg(windows)]
fn resolve_sampler() -> Option<Box<dyn SurfaceSampler>> {
    eso_weave::pixelbus::GdiSampler::for_window(WINDOW_TITLE)
        .map(|sampler| Box::new(sampler) as Box<dyn SurfaceSampler>)
}

#[cfg(target_os = "linux")]
fn resolve_sampler() -> Option<Box<dyn SurfaceSampler>> {
    // The X11 sampler reads each derived point with a 1x1 request, so it has no
    // capture region to size from the block width.
    Some(Box::new(eso_weave::pixelbus::X11Sampler::for_window(
        WINDOW_TITLE,
    )))
}
