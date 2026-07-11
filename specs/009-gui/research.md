# Phase 0 Research: Graphical User Interface

All decisions were made under the Build-Phase Autopilot Protocol against the
constitution, the master specification (section 10), and existing code patterns.
The GUI approach (full GUI this slice) was confirmed with the operator; no other
options were escalated.

## Decision: eframe/egui with the glow backend, default features off

- **Decision**: Add `eframe = { version = "0.35", default-features = false,
  features = ["glow", "default_fonts", "x11", "wayland"] }`.
- **Rationale**: The master specification names egui/eframe. The glow (OpenGL)
  backend is lighter than the default wgpu backend, builds cleanly here (verified:
  a probe crate compiled in ~21 s), and is well suited to a simple desktop tool.
  Disabling default features drops wgpu; `default_fonts` gives text out of the box;
  `x11`/`wayland` cover Linux windowing and are inert on Windows. `persistence` and
  `serde` are left off because the app owns its own config store.
- **Alternatives considered**: The wgpu backend (rejected: heavier dependency and
  build for no benefit here). A non-egui toolkit (rejected: contradicts the
  master specification). Bundling custom fonts (rejected: unnecessary for v1).

## Decision: Split a testable view-model from the egui rendering

- **Decision**: Put all correctness-bearing logic (display derivation, UI-intent
  handling, the settings-to-config mapping, the reader-event routing, and the log
  view snapshot/filter/color) in `app::*` modules outside `ui.rs`, tested against
  the project's in-memory subsystem doubles. `ui.rs` only reads the view-model and
  raises intents.
- **Rationale**: A native egui window cannot be exercised in the automated
  environment, so Principle III is satisfied by making everything except the raw
  rendering unit-testable. The rendering is thin and validated with a manual
  checklist.
- **Alternatives considered**: Testing the egui layer with `egui`'s test harness
  (rejected: it cannot validate the real window/threading and adds complexity for
  little coverage; the manual checklist covers rendering). Putting logic inline in
  the egui update loop (rejected: untestable, the anti-pattern this split avoids).

## Decision: Share subsystems across threads with the minimal synchronization each needs

- **Decision**: `InputEngine` is shared as `Arc<InputEngine>` (it is already
  internally synchronized). `WeaveEngine` and `FishingController` (which need `&mut`
  for `handle`/`set_latency` and `on_event`/`tick`/`set_enabled`) are shared as
  `Arc<Mutex<...>>`. `LogHandle` is `Clone` and shared directly. The Beacon Manager
  operations are free functions the UI calls directly. The `PixelBusReader` is owned
  by the pixel-bus worker thread.
- **Rationale**: Each subsystem gets exactly the synchronization it needs; the
  atomically-internal `InputEngine` needs no extra lock, while the `&mut` engines get
  a mutex. The GUI thread and the worker threads then share state safely.
- **Alternatives considered**: A single global mutex around all subsystems (rejected:
  needless contention and coupling). Channels only (rejected: the GUI needs to read
  live status each frame, which snapshots behind a short lock express more directly).

## Decision: Preserve the input hook thread; eframe owns the main thread

- **Decision**: The input backend continues to run on its own thread (its Windows
  low-level hook keeps its dedicated message pump and non-blocking contract from
  S002); the weave worker drains the action receiver on its thread; the pixel-bus
  worker samples and routes on its thread; eframe runs the winit event loop on the
  main thread.
- **Rationale**: This keeps the safety-critical input threading contract intact
  (Principle II) while letting eframe own the main event loop as it requires. The
  threads are independent and already exist as seams from prior slices.
- **Alternatives considered**: Running the hook on the main thread alongside winit
  (rejected: would violate the S002 contract and risk the hook being silently
  removed). Polling input from the UI loop (rejected: not how the interception
  backend works).

## Decision: Reader-event routing reuses `fishing::map_event`

- **Decision**: `route_reader_event(event, weave, fishing, now_ms, sink)` sets the
  weave latency on `Latency(ms)`, clears it on `SignalLost`, and forwards every
  event through `fishing::map_event` to the fishing controller (Latency maps to
  `None` and is dropped for fishing; Heartbeat is a no-op in the controller).
- **Rationale**: Reusing the already-tested `map_event` keeps one source of truth for
  the reader-to-detector mapping and makes the routing a thin, testable wrapper. The
  SignalLost dual action (clear weave latency and disable fishing) is exactly the
  safety behavior the two subsystems require.
- **Alternatives considered**: A bespoke match duplicating `map_event` (rejected:
  duplication). Routing latency into the fishing controller (rejected: the detector
  event set excludes latency).

## Decision: Add `pixelbus` and `ui` additive settings sections to complete 10.3

- **Decision**: Add a `pixelbus` section (sampling interval and per-channel
  tolerance, mapping to `ReaderConfig`) and a `ui` section (a `Theme` enum defaulting
  to dark, and `always_on_top` defaulting to false). Both are additive opaque
  sections like the existing ones; the settings form loads and applies all nine
  section-10.3 categories reusing each subsystem's existing load/store.
- **Rationale**: Section 10.3 requires editing and persisting sampling interval and
  tolerance, theme, and always-on-top, none of which had a settings home yet. Adding
  them as additive sections follows the established pattern and needs no schema bump.
- **Alternatives considered**: Storing theme/always-on-top as runtime state
  (rejected: the constitution forbids runtime state in config, but these are genuine
  user settings, so they belong in config). Folding pixelbus into another section
  (rejected: it is its own concern).

## Decision: Rendering is validated by a documented manual checklist

- **Decision**: `quickstart.md` carries a manual validation checklist (window opens,
  each control drives the expected effect, the log panel colorizes and pause-scrolls,
  settings persist, theme and always-on-top apply). The automated gate is fmt,
  clippy, the view-model tests, and a clean compile of the eframe app.
- **Rationale**: The operator accepted that the native window cannot be verified
  headlessly here; the manual checklist is the honest verification for the rendering,
  while all logic is unit-tested.
- **Alternatives considered**: Claiming the GUI verified by compile alone (rejected:
  dishonest; a clean compile does not prove the window behaves). Screenshot testing
  (rejected: not feasible in this environment).
