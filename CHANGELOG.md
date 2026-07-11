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

### Changed

### Fixed

### Decisions

- 2026-07-11: Pin the Rust toolchain to 1.96.0 via `rust-toolchain.toml` (a
  pinned artifact; this dated entry records its creation) and adopt serde,
  serde_json, tracing, tracing-subscriber, dirs, time, and thiserror for the
  foundations slice. Rationale is recorded in
  `specs/001-foundations/research.md`.
