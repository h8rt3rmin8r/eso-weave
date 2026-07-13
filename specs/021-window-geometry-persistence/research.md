# Phase 0 Research: Window Geometry Persistence

## Questions and findings

### How does the app currently open its window, and why is nothing restored?

`src/main.rs` builds `eframe::egui::ViewportBuilder::default().with_inner_size([600.0, 720.0]).with_min_inner_size([480.0, 420.0])` and runs with `NativeOptions { viewport, ..Default::default() }`. Nothing sets position, size, or maximized from persisted data. `Cargo.toml` builds eframe with `default-features = false` (glow, default_fonts, x11, wayland), so the eframe `persistence` feature is off and egui's built-in window persistence backend does not exist; there is also no `eframe::App::save` override. Conclusion: persistence must be implemented explicitly.

### Where should geometry live: config.json, state.json, or eframe ron storage?

The constitution states configuration "stores user settings only. No session, runtime, or derived state is ever written to the config file." Window geometry is automatically captured runtime state (where the window last happened to be), so it belongs with the existing session state in `state.json` (`src/config/state.rs`, `SessionState`), alongside the suspend and fishing intents and the derived api-version cache. eframe ron `persist_window` is rejected: it needs the persistence feature (intentionally disabled), writes a second store in a different location, and duplicates the JSON-with-schema-version machinery the project already standardizes on.

### What geometry does egui expose, and in what units?

egui 0.35 exposes `ViewportInfo` via `ctx.input(|i| i.viewport())`, with `outer_rect: Option<Rect>` (window outer top-left and extent, screen coordinates), `inner_rect: Option<Rect>` (client area), and `maximized: Option<bool>`. All rects are in egui points (logical units). `ViewportBuilder::with_position`, `with_inner_size`, and `with_maximized` also take points/logical values. Capturing and restoring entirely in points is therefore a consistent round trip independent of the DPI scale in effect. Position is taken from `outer_rect.min` and size from `inner_rect.size`, matching what `with_position` (outer top-left) and `with_inner_size` (client size) consume.

### How is the same monitor reselected on a multi-monitor desktop?

egui reports positions in absolute desktop (virtual-screen) coordinates that include each monitor's offset. Restoring the saved outer top-left position places the window back on the monitor whose region contains that point, when the layout is unchanged. A separate monitor identifier is not required for the common case.

### How is an off-screen (disconnected/changed monitor) position detected safely?

On Windows, the desktop virtual-screen rectangle is `GetSystemMetrics(SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN)` (physical pixels; `windows-sys` feature `Win32_UI_WindowsAndMessaging`, already enabled). To compare against point-space stored geometry, divide by the system DPI scale from `GetDpiForSystem()/96.0` (`windows-sys` feature `Win32_UI_HiDpi`, to be added). The pure helper then requires the recorded window rectangle to overlap the point-space desktop by a visible margin (a strip of the title bar and a minimum width); if it does not, the position is dropped and the window opens centered at the restored size. On Linux the accessor returns `None`: Wayland cannot report or set absolute window position, and X11 placement is left to the window manager, so only size and maximized restore there.

### How are writes coalesced, and how is a last-moment change not lost?

`AppModel` already owns a `SaveScheduler` that coalesces a continuous edit into a single settle-delayed write (400 ms) and a `maybe_flush` called each frame. Geometry changes reuse this: a change marks the session store dirty and writes once after motion settles. Because a resize immediately before quitting could settle after the window is gone, a forced session write is added on window close (the OS close request) and on the Exit menu item, capturing the final geometry (FR-008).

### Why integers, and does SessionState stay Copy/Eq?

`SessionState` derives `Copy`, `PartialEq`, and `Eq`, and `apply_api_check` compares instances. Storing geometry as `f32` would forfeit `Eq`. Rounding points to integers (`i32` position which may be negative on secondary monitors, `u32` size) preserves `Copy` and `Eq`, avoids sub-pixel write churn, and is exact for window placement.

### What is the migration story?

`CURRENT_STATE_VERSION` is 2. Adding the optional `window` field is additive and backward compatible: an old `state.json` without it deserializes with `window: None` via `#[serde(default)]`, meaning no restore (open at default). The version is bumped to 3 for provenance, consistent with how the api-version cache was added at the 1 -> 2 bump. A malformed state file already falls back to defaults with a notice, so a malformed geometry section degrades to "no recorded geometry".

## Alternatives considered

- **eframe `persist_window` (ron storage)**: rejected (feature disabled, second store, constitution's JSON-with-schema-version standard).
- **config.json `ui` section**: rejected (constitution forbids runtime state in the settings file).
- **Storing a monitor identifier**: unnecessary for the unchanged-layout common case; absolute coordinates already select the monitor, and off-screen recovery covers the changed-layout case.
- **Physical-pixel storage**: rejected; eframe's `ViewportBuilder` consumes points, so points-in/points-out is the consistent representation and avoids a physical-to-logical conversion on the restore path.
