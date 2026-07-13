# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

### Added

- ESO API version check automation. On startup, off the GUI thread, the app
  fetches the live ESO game client version from the official esoui/esoui GitHub
  live branch as a bump-detection signal, keeps the on-disk PixelBeacon manifest
  `## APIVersion:` current (marker-gated, never downgrading, preserving all other
  lines), and warns in the live log when the client has moved ahead of this build
  so the player knows to update. A compiled default API version guarantees a valid
  manifest with no network and no stored value; the last known API version and
  last seen game version persist in `state.json`.

### Documentation

- The master specification is rewritten as `docs/ESO-Weave-Specification-v0.2.0.md`,
  documenting the system as built in a declarative voice with expanded mermaid
  diagrams (system architecture, concurrency and ownership, input interception,
  weave sequence, fishing state machine, pixel-bus pipeline, beacon lifecycle, API
  version check, GUI layout, and config and state persistence). Every repository
  reference is repointed to v0.2.0 and the superseded v0.1.0 file is removed.
- The README fishing usage now includes bait selection: without bait selected in
  game the F2 cast fails and fishing never starts. Bait is added as a
  prerequisite, as an explicit step in "Using it", and as a troubleshooting check.

### Decisions

- 2026-07-12: The master specification is superseded by
  `docs/ESO-Weave-Specification-v0.2.0.md`. Per the constitution, a new master-spec
  version lapses the standing Build-Phase Autopilot authorization; this rewrite was
  produced under an explicit operator kickoff, and the standing autopilot
  authorization is re-affirmed against v0.2.0. The version is bumped (not a v0.1.0
  in-place edit) so the document maturity matches its filename, at the cost of
  repointing the path references, which are updated in the same change.
- 2026-07-12: Added a networked dependency, `ureq` (blocking, rustls TLS), to
  support the startup ESO API version check. No async runtime is introduced; the
  check runs on a `std::thread`. `Cargo.toml` is not a pinned artifact; this entry
  records the added networked dependency per the constitution. The chosen version
  source is the esoui/esoui GitHub `live` branch head commit, because the exact
  numeric API version is only published behind bot challenges a plain client
  cannot pass, whereas GitHub reliably reports the live game version string as the
  bump-detection signal. The numeric value written to the manifest resolves as the
  maximum of the stored last known value and the compiled default.

## [0.4.3] - 2026-07-13

### Documentation

- The README now has dedicated Fishing and Weaving usage sections. Fishing
  documents the interaction model (the F2 hotkey casts for you; do not cast
  first), the PixelBeacon prerequisites (installed, enabled, not out of date,
  beacon visible, window focused), the status progression, the interact key and
  timings, and troubleshooting for the early-stop symptom. Weaving documents the
  single-bar overview, the skill slots and defaults, the weave types, and the
  default timings (global cooldown 500 ms, light 50 ms, heavy 1000 ms, bash
  125 ms); multi-bar weaving is noted as out of scope. The Disclaimer is moved to
  be the next-to-last section, immediately before the License.

### Fixed

- Fishing would start (status "active") and then revert to Idle within a few
  seconds, and a caught fish was never reeled in. Three causes are addressed.
  First, the embedded PixelBeacon manifest declared a stale API version
  (`101044`), so the game flagged the addon out of date and, unless the player had
  enabled out-of-date addons, did not load it at all; with no beacon rendered the
  app never saw the cast or bite signal. The manifest now declares
  `## APIVersion: 101050 101054` (the current live value plus a future value, the
  game's supported two-value form) and bumps `## Version`/`## AddOnVersion` to 3 so
  an existing on-disk install is classified as outdated and refreshed. Second, the
  pixel-bus worker loop always slept at the idle interval (1000 ms) and never used
  the fishing interval (100 ms), so it sampled the beacon and ticked the fishing
  state machine only once per second and missed the transient cast and bite pulses
  and the reel window; the loop now polls at the fishing cadence while a session is
  active. Third, fishing deadlines were stamped on the GUI clock but evaluated on a
  separate worker clock; both now share one monotonic origin. The default arm
  timeout is raised from 5000 ms to 8000 ms for margin. The safety behaviors (no
  blocking on the hook thread, focus-scoped suppression, SignalLost cancels any
  pending interact, and managed-marker-gated uninstall) are unchanged and stay
  tested.

### Added

- The Fishing status now reads in plain language (Casting, Fishing (waiting for a
  bite), Reeling in, Recasting) instead of internal state names, and when the
  routine returns to idle it explains why: Idle (no cast detected) after an arm
  timeout, Idle (signal lost) on signal loss, or a plain Idle when the player
  stopped it. The reason persists until fishing is next started and colors a
  fault-stop as a warning, so an early stop is diagnosable at a glance.

### Decisions

- 2026-07-12: The PixelBeacon manifest `## APIVersion` is set to `101050 101054`,
  closing open item R4 (the live API version could not be confirmed offline when
  the value was last left at 101044). The current live value (game Update 50) is
  declared so the game loads the addon, and a future value is declared using the
  supported two-value form to keep the addon current across several future updates.
  `## Version`/`## AddOnVersion` are bumped to 3 so existing installs refresh; the
  managed-marker line is unchanged so marker-gated safe uninstall is unaffected.
- 2026-07-12: The pixel-bus worker selects its poll interval from the live fishing
  state (fishing interval while active, idle interval otherwise) through a pure,
  unit-tested helper, rather than always polling fast (wasteful) or redesigning the
  loop to be event-driven (out of scope). The GUI and worker share one monotonic
  clock origin so fishing deadlines are stamped and evaluated on one timeline. The
  arm-timeout default rises to 8000 ms as a provisional value pending in-game
  validation.

## [0.4.2] - 2026-07-12

### Fixed

- The default F1 (suspend) and F2 (fishing) hotkeys had no effect in-game. The
  input engine hands their actions off on the action channel, which is drained
  only by the weave worker, and the weave engine maps both toggle actions to no
  operation; the real suspend and fishing state is owned by the GUI intent path
  (`AppModel::apply_intent`), so the hotkeys never reached it. The weave worker
  now forwards the two application-level toggle actions to the GUI over a
  dedicated channel, and the GUI drains them each frame and applies them through
  the same intent path as the Status and Fishing buttons. A hotkey and its button
  now share one state, one persistence mark, and one display update. The
  safety-critical `InputEngine::classify` path (recursion breaking, focus-scoped
  suppression, non-blocking hand-off) is unchanged, and toggles still only take
  effect while the game window is focused.

### Added

- Pixel-bus reader diagnostics for weapon-bar detection. The reader now logs a
  DEBUG line when a weapon bar is first detected (with the decoded bar and
  classes) and when it is cleared on signal loss, and a TRACE line with the raw
  sampled block bytes on every observation. This lets the operator confirm
  in-game whether the weapon-bar signal is present and decoding, and tell a
  present heartbeat with a non-decoding B3 (a stale or misrendered addon) apart
  from no heartbeat at all, without a debugger. Nothing weapon-related logs at the
  default level on an idle sample. The decode path and the B3 encoding are
  unchanged; this slice wires up detection visibility only, with no effect on
  weave timing or skill weaving. The live pixel signal is validated in-game (an
  explicit operator follow-up).

### Decisions

- 2026-07-11: Hotkey suspend and fishing toggles are routed from the weave worker
  to the GUI intent path over a dedicated `std::sync::mpsc` channel, rather than
  mutating the shared state from the worker or splitting the action channel inside
  the safety-critical `InputEngine`. Reusing `AppModel::apply_intent` makes a
  hotkey and its button provably identical in state, persistence, and display, and
  leaves the most-tested core untouched. Worst-case reflection latency is the
  existing 250 ms idle repaint cadence.
- 2026-07-11: Weapon-bar reader diagnostics are layered by log level (DEBUG for
  detected and cleared transitions, TRACE for raw per-sample bytes) so detection
  is diagnosable in-game without emitting per-sample log lines at the default
  level, and without changing decode behavior.

## [0.4.1] - 2026-07-11

### Fixed

- Main-window emphasis labels (the Status, Fishing, Pixel Beacon (Addon), and
  Weapon Bar titles, and the Skills column headers Skill, Enabled, Weave,
  Override, Delay) were nearly unreadable on the dark base: they rendered in the
  dark ink used for text on a gold button. egui derives bold (`.strong()`) text
  color from the active-widget text color, which the brand theme sets to
  `gold_text`, so every strong label inherited it. Emphasis labels now go through
  a new `widgets::label_strong` helper that draws them in the primary text color
  (Inter SemiBold at body size), which the palette legibility test already
  guarantees is readable in both themes. Presentation layer only; no
  safety-critical surface changed.

## [0.4.0] - 2026-07-11

### Added

- Weapon-Bar-Aware Adaptive Timing (S014): the app now detects which weapon bar is
  active and each bar's weapon class and applies per-bar skill-delay timing. The
  PixelBeacon addon gains a fourth pixel block (B3) that encodes the active bar and
  a normalized weapon-class code computed in Lua from the game weapon-type
  constants (so the reader never needs the raw enum integers), edge-detected so
  per-attack redraws do not churn and re-baselined after loading screens. The
  pixel-bus reader decodes it, the weave engine keeps a front and back timing
  profile selected by the active bar, and an "auto timing from weapon" preference
  fills each bar's heavy-attack delay from weapon-class presets (dual wield fastest
  through staves and bow slowest). The main window shows the detected bar and
  weapon classes, and the settings expose the auto-timing toggle and a back-bar
  timing group. Closes research item R1 with a new timing appendix
  (`docs/ESO-Weave-Specification-v0.2.0.md` Appendix A). The exact preset values
  and the pixel signal require in-game validation (an explicit follow-up).
- GUI Ergonomics, Information Design, and Auto-Save (S013): a substantial rework
  of the main window. Two-state controls (suspend and resume, fishing, per-skill
  enabled and override, and every boolean setting) are now colorized toggle
  switches; sections use real headings from the bundled Inter SemiBold weight; the
  status region is renamed (Status, Fishing, Pixel Beacon (Addon)), spread across a
  full-width grid, and shows a normalized, color-coded state field; the Skills grid
  has labeled columns (Skill, Enabled, Weave, Override, Delay) and shows the
  inherited default (muted) instead of a literal zero when no override is set, with
  the override targeting the delay for the row's weave type. The Settings screen is
  now a full-frame modal over a dimmed backdrop that closes on an outside click,
  Escape, or the close control, reorganized into labeled clusters (Appearance,
  Combat timing, Fishing, Pixel Beacon and bus, Logging, Keybindings) with no
  underscores in any label and a short inline help line under every option; the
  previously hidden beacon AddOns-folder override and game environment are now
  surfaced. The live log moved into a resizable bottom panel with a darker
  terminal-like fill and a monospace font. All persistence is now automatic and
  coalesced (no Apply or Save control anywhere): main-window skill edits, the live
  suspend and fishing intents, and the log-panel height are all persisted and
  restored across restarts, with a gentle bottom-right save confirmation. Hover
  tooltips and inline help cover the controls, section titles, and Skills columns.

### Decisions

- 2026-07-11: Weapon-Bar-Aware Adaptive Timing (S014) changes pinned contract
  surfaces. The pixel-bus contract (`specs/004-pixelbeacon-addon/contracts/pixel-bus.md`)
  gains the B3 weapon-bar block at x=48 (sample (56,8)): green `0x5A` marker
  (distinct from the latency marker `0xA5`), red packing the front and back
  weapon-class nibbles, blue the active-bar code; the marker is matched within
  tolerance while the data channels are read exactly. The reader contract
  (`specs/005-pixel-bus-reader/contracts/reader.md`) gains `decode_weapon_bar`, the
  `ActiveBar`/`WeaponClass`/`WeaponBarSignal` types, the `WeaponBar` event (emitted
  only on change and only with a heartbeat), and the fourth sample point, and
  `observe` takes a `b3` argument. The PixelBeacon manifest
  (`addon/PixelBeacon/PixelBeacon.txt`) bumps `## Version` and `## AddOnVersion` to
  2 (single-sourced into the app's embedded-version check); the managed-marker line
  is unchanged so safe uninstall still verifies it, and `## APIVersion` is left at
  101044 because the weapon-bar API predates it and the live value cannot be
  confirmed offline (tracked under R4). The weapon-class codes are shared
  byte-for-byte between the addon Lua and the reader. The R1 appendix in the master
  specification records the evidence-based defaults and marks R1 closed. Session
  state is unaffected. In-game validation of the pixel signal and the exact preset
  timings (including one-hand-and-shield) is owed.
- 2026-07-11: GUI Ergonomics and Auto-Save (S013) persists live session state
  (the suspend and fishing on/off intents) so the app restores the state it was
  closed in. Because the constitution requires the configuration file to hold user
  settings only, with no session, runtime, or derived state, this session state is
  written to a separate `state.json` in the config directory, never to
  `config.json`. Folding it into `config.json` was rejected as a constitution
  violation; not persisting it was rejected because the operator requested the
  restore. Restoring under the focus-scoped input invariant is safe: a restored
  running or fishing-on state performs no input until the game window is focused,
  and the fishing intent restores as a clean re-arm rather than a transient
  sub-state. The log-panel height is a user layout preference and is kept in the
  config UI section. No pinned artifact is changed by this slice.

## [0.3.0] - 2026-07-11

### Added

- Brand and UX Polish (S012): a documented "Arcane gold on ink" brand standard
  (`docs/brand/ESO-Weave-Brand-v1.md`) applied across the app and installers. A new
  woven-caret brand mark (gold and teal on an ink badge) replaces the antique
  two-fish gold mark and is regenerated at every size from SVG masters under
  `assets/brand/` by `assets/brand/generate.sh`. The application window and the
  Windows executable now carry the mark (a `build.rs` embeds the exe icon on
  Windows), and the app is themed for both dark (default) and light modes with the
  bundled Inter typeface, aligned skill columns, and a pointer cursor on every
  clickable control. The installer license page is rendered as clean proportional
  text, the wizard uses branded artwork, and the desktop shortcut is now an opt-in
  Custom Setup feature that is off by default. Adds a GitHub social-share image
  (`assets/eso-weave-social.png`).

### Decisions

- 2026-07-11: Brand and UX Polish (S012) changes the pinned packaging artifacts.
  `wix/main.wxs` switches the wizard from `WixUI_InstallDir` to `WixUI_FeatureTree`,
  adds the `WixUIBannerBmp` and `WixUIDialogBmp` branded-artwork variables, and
  moves the desktop shortcut into its own `Level="2"` (off-by-default) `Feature`,
  nested under the application feature, so it is opt-in via the Custom Setup step,
  while the application feature is `Absent="disallow"` and configurable for the
  install location. The shortcut `Target` values use the resolved path
  `[APPLICATIONFOLDER]eso-weave.exe` instead of the `[#EsoWeaveExe]` file key so
  the opt-in shortcut in a child feature does not trip ICE69 (a cross-feature file
  reference); this was confirmed by building the MSI locally with WiX 3.11. A single
  checkbox on the install page was rejected because it requires replacing the entire
  built-in WixUI dialog set, which cannot be validated without a local WiX build.
  `packaging/windows/License.rtf` is regenerated from `LICENSE` as proportional
  (Segoe UI) RTF with headings and spacing, text preserved verbatim. New pinned
  wizard bitmaps `packaging/windows/banner.bmp` (493x58) and
  `packaging/windows/dialog.bmp` (493x312) are added, and the pinned Linux and
  AppImage icons (`packaging/linux/eso-weave.png`,
  `packaging/appimage/AppDir/eso-weave.png`) are regenerated from the new mark. The
  pinned `.gitattributes` adds `*.bmp binary` so the wizard bitmaps are never line
  normalized. All packaging rasters are reproduced by `assets/brand/generate.sh`
  (ImageMagick 7). Rationale is in `specs/012-brand-ux-polish/`.

## [0.2.0] - 2026-07-11

### Added

- Installer and First-Run Experience (S011): the Windows MSI now presents a guided
  WixUI wizard (welcome, license, install location, progress, finish) with a
  license acceptance gate, adds a desktop shortcut alongside the Start Menu entry,
  and offers a de-elevated "Launch ESO Weave" checkbox on the finish page that
  never launches on a silent install. The application is built for the Windows
  subsystem on release, so it no longer flashes a console window, and a startup
  panic hook shows a native message box and writes a log line so a first-run
  failure is never silent. Adds `packaging/windows/License.rtf` and a bin-local
  `startup` module behind a testable `Notifier` seam; the README documents the
  shortcut and log locations.
- README: the `assets/eso-weave-banner.png` banner now heads the README, and the
  static version badge is bumped automatically by the release rollover so it no
  longer drifts from the released version.

### Decisions

- 2026-07-11: Installer and First-Run Experience (S011) changes the pinned
  packaging artifact `wix/main.wxs` (adding the WixUI_InstallDir wizard, the
  `WixUILicenseRtf` variable, a desktop shortcut component, and the ExitDialog
  launch custom action) and adds `packaging/windows/License.rtf` (the repository
  Apache-2.0 license rendered as RTF for the wizard license page). The
  launch-on-finish uses the WixUI ExitDialog with `WixShellExec` and
  `Impersonate="yes"`, which runs in the InstallUISequence as the invoking user for
  a de-elevated launch; a silent install has no UI sequence and never launches.
  cargo-wix links WixUIExtension and WixUtilExtension by default (verified in the
  cargo-wix linker source), so the pinned `.github/workflows/release.yml` is
  unchanged. The WixShellExec custom action takes no `Return` attribute (WiX
  CNDL0038 forbids `Return` without `ExeCommand`). Rationale is in
  `specs/011-installer-first-run/research.md`.
- 2026-07-11: Automate the README version badge in the pinned `release.toml` with a
  `[[pre-release-replacements]]` entry that rewrites the static shields.io badge
  version on every `cargo release`, and correct `docs/releasing.md` (both pinned
  artifacts) which had described the badge as dynamically read from the latest
  GitHub Release. The badge is static, so it needs the rollover to stay in sync.

## [0.1.1] - 2026-07-11

### Added

- Foundations (S001): a single Rust crate with the Config Store (settings-only
  JSON, corruption fallback with `.invalid` preservation, forward migration) and
  the Logging subsystem (runtime-selectable level, always-on ring buffer,
  optional monthly file sink, input-privacy guarantee).
- Input Engine (S002): a platform-agnostic engine core with focused-window-only
  interception, injected-input recursion breaking, a non-blocking bounded
  hand-off, suspend with suspend-exempt toggles, and a conflict-rejecting
  keybinding model persisted as an additive settings section, behind an
  `InputBackend` seam with a mock plus Windows (low-level hook, SendInput) and
  Linux (evdev grab, uinput) backends.
- Weave Engine (S003): seven skill slots with four weave types, a pure
  sequence builder, global timing with per-slot overrides, monotonic-clock
  cooldown gating, inactive-slot pass-through fed to the Input Engine, and
  additive `skills` and `timing` settings sections, executed through a testable
  `WeaveSink` seam. Adds mouse synthesis (primary and secondary) to the input
  backends.
- PixelBeacon addon (S004): the embedded in-game Lua companion under
  `addon/PixelBeacon/`, rendering the three pixel-bus blocks (status heartbeat,
  fishing state, latency with marker and checksum) at constant physical-pixel
  geometry and detecting a bite from bait consumption, with the managed marker
  line in its manifest. No Rust changes.
- Pixel Bus Reader (S005): pure decoders (status heartbeat, fishing signal,
  checksum-validated latency) with per-channel tolerance and a `PixelBusReader`
  state machine that emits typed events and raises SignalLost on heartbeat
  timeout against an injected clock, behind a `SurfaceSampler` seam with a mock
  plus thin GDI (Windows) and X11 (Linux) samplers.
- Beacon Manager (S006): on-disk lifecycle of the embedded PixelBeacon addon
  (embedded manifest and Lua, single-sourced embedded version), pure four-state
  classification, install confined to the `PixelBeacon` subtree of an injected
  AddOns root, and a marker-gated uninstall that deletes only when the managed
  marker line is verified present in the on-disk manifest. AddOns discovery sits
  behind thin backends (Windows Documents known folder; Linux Steam
  `libraryfolders.vdf` plus Proton app id 306130 compatdata), with a manual path
  override and a selectable `live`/`pts` environment persisted as an additive
  `beacon` settings section, plus a best-effort running-game probe feeding the
  `/reloadui` reminder. No new crates.
- Fishing Controller (S007): a pure, non-blocking fishing state machine (Disabled,
  Armed, Waiting, Reeling, Recast) driven by detector events and an injected clock,
  with configurable arm/reel/recast timing persisted as an additive `fishing`
  settings section. On SignalLost it disables fishing and cancels any pending
  interact rather than blind-firing. A `BiteDetector` trait (with a stub) and a v1
  `PixelBusDetector` adapt the Pixel Bus Reader events (dropping Latency), and the
  interact key is synthesized through a `FishingSink` seam over the input backend
  (mock plus real), with `Key::E` added as the default interact key. No new crates.
- Latency-Adaptive Delays (S008): an opt-in weave enhancement that scales the
  `d_weave` and `d_bash` delays by server latency using
  `effective_delay = base + clamp(round(k * latency), 0, 300)` (k default 0.25),
  leaving `d_heavy` and `global_cooldown` untouched. The computation lives in the
  pure weave sequence builder; `sequence_for` delegates to the adapted builder with
  the feature disabled, so existing weave timing is byte-for-byte unchanged unless
  the feature is enabled with live latency. The engine takes latency in via
  `set_latency(Option<u16>)` (clearing on signal loss reverts to base delays), and
  the enabled flag and `k` persist as an additive `latency` settings section. Off by
  default. No new crates.
- Graphical User Interface (S009): an eframe/egui main window that integrates and
  controls every subsystem, built around a testable application view-model (status
  and beacon-light derivation, UI-intent handling, the settings-to-config mapping
  for all of section 10.3, and the reader-event routing) separated from the egui
  rendering. Status region (Suspend/Resume, Go Fish/Stop, a PixelBeacon status light
  with exact-condition tooltip, Install, confirm-gated Uninstall), skills region
  (per-slot active, weave type, and delay override), a colorized live log panel over
  the ring buffer with pause-scroll and a level filter, and an in-app settings
  surface for every section-10.3 category. A worker loop pumps the pixel bus reader
  and routes its events (latency to the weave engine, signal loss to weave and
  fishing, fishing events to the controller) without blocking the UI thread. Adds the
  `eframe`/`egui` dependency (glow backend) and additive `pixelbus` and `ui` settings
  sections.
- Packaging and Distribution (S010): the artifacts that complete the pinned release
  pipeline, a WiX MSI source (`wix/main.wxs`) and `assets/icon.ico`, cargo-deb
  metadata in `Cargo.toml` with a desktop entry, icon, and a packaged `/dev/uinput`
  udev rule, an AppImage `AppDir`, the `scripts/changelog-section.sh` and
  `scripts/linux-build-deps.sh` scripts, `release.toml` for cargo-release, and a
  Linux evdev-permission section in the README. The MSI installs only the
  application and never writes to game or Documents directories; the version stays
  single-sourced from `Cargo.toml`.

### Changed

### Fixed

### Decisions

- 2026-07-11: Pin the Rust toolchain to 1.96.0 via `rust-toolchain.toml` (a
  pinned artifact; this dated entry records its creation) and adopt serde,
  serde_json, tracing, tracing-subscriber, dirs, time, and thiserror for the
  foundations slice. Rationale is recorded in
  `specs/001-foundations/research.md`.
- 2026-07-11: Adopt target-specific dependencies for the Input Engine backends:
  `windows-sys` (Windows) for the low-level hook, SendInput, and timer
  resolution, and `evdev` plus `x11rb` (Linux) for the keyboard grab, uinput
  synthesis, and X11 focus. The Linux backend is type-checked and clippy-clean on
  the linux target; its runtime is validated on a Linux host. Rationale is in
  `specs/002-input-engine/research.md`.
- 2026-07-11: Beacon Manager (S006) single-sources the embedded addon version by
  parsing the embedded `PixelBeacon.txt` manifest at runtime rather than
  declaring a separate version constant, so the file written on install and the
  version verify compares can never drift. Beacon settings (AddOns path override
  and `live`/`pts` environment) reuse the additive opaque config-section pattern
  (like `timing` and `skills`), requiring no config `schema_version` bump. Enable
  the `windows-sys` `Win32_System_Diagnostics_ToolHelp` feature for the
  best-effort running-game process probe; no new crates. Rationale is in
  `specs/006-beacon-manager/research.md`.
- 2026-07-11: Fishing Controller (S007) is a non-blocking, event-and-tick-driven
  state machine with all delays and timeouts modeled as deadlines against an
  injected clock, so it is pure and fully unit-tested. Its interact sink is a
  dedicated key-only `FishingSink` over the input engine's `InputBackend` (not the
  weave engine's `WeaveSink`), keeping the fishing module dependent only on the
  input engine and the reader. Fishing settings reuse the additive opaque
  config-section pattern (no `schema_version` bump), and a `Key::E` variant was
  added to the input engine as the default interact key (its Windows and Linux
  scan-code mappings included). Rationale is in
  `specs/007-fishing-controller/research.md`.
- 2026-07-11: Latency-Adaptive Delays (S008) computes the effective delay in the
  pure weave sequence builder, exactly where the per-slot-resolved delays are
  consumed, so the scaling respects per-slot overrides and stays unit-testable.
  `sequence_for` delegates to `sequence_for_adapted` with the feature disabled,
  structurally guaranteeing no regression to existing weave timing. The adaptation
  config (enabled flag and `k`, valid finite in `[0.0, 4.0]`) persists as a new
  additive `latency` settings section (no `schema_version` bump); the transient
  current latency is runtime state fed via `set_latency` and never written to the
  config file. Rationale is in
  `specs/008-latency-adaptive-delays/research.md`.
- 2026-07-11: Packaging (S010) creates the pinned artifacts the release pipeline
  references for the first time: `scripts/changelog-section.sh` and
  `scripts/linux-build-deps.sh` (the shared changelog extractor and Linux build
  dependency source), `release.toml` (cargo-release: version bump, CHANGELOG roll,
  `release: vX.Y.Z` commit, tag, push), the WiX MSI source `wix/main.wxs`, the
  AppImage `AppDir` under `packaging/appimage/`, and the udev rule and desktop entry
  under `packaging/linux/`. It also adds `[package.metadata.deb]` to `Cargo.toml`
  and generates `assets/icon.ico` from the logo art with ImageMagick. The pinned
  `.github/workflows/release.yml`, `docs/releasing.md`, and `rust-toolchain.toml`
  are not modified, and no release tag is cut. The MSI never writes to game or
  Documents directories. Rationale is in `specs/010-packaging-and-ci/research.md`.
- 2026-07-11: The GUI (S009) adds the `eframe`/`egui` 0.35 dependency with the glow
  backend (`default-features = false`, features `glow`, `default_fonts`, `x11`,
  `wayland`), the spec-named GUI framework; the glow backend is lighter than wgpu and
  builds on both targets. The correctness-bearing logic lives in a testable
  `app` view-model separated from the egui rendering, which is validated with a
  manual checklist because a native window cannot be exercised in the automated
  environment. The input hook thread keeps its own message pump (the S002 contract)
  while eframe owns the main thread; the subsystems are shared across the
  interception, weave-worker, and pixel-bus worker threads via a `SharedBackend`
  adapter so synthesis stays self-originated. Theme and always-on-top, and pixel bus
  sampling tolerance and intervals, persist as additive `ui` and `pixelbus` settings
  sections (no `schema_version` bump). Rationale is in `specs/009-gui/research.md`.
