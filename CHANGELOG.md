# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

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
