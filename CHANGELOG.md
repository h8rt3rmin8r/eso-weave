# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0/).

## [Unreleased]

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
