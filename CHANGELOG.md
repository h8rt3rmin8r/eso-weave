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
