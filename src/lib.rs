//! ESO Weave foundations library.
//!
//! This crate carries the project foundations: the Config Store
//! ([`config`]) and the Logging subsystem ([`logging`]), plus a per-platform
//! path seam ([`platform`]) that later input and sampling backends extend. The
//! [`beacon`] module manages the on-disk lifecycle of the embedded PixelBeacon
//! addon.

pub mod app;
mod atomic_file;
pub mod beacon;
mod bounded_file;
pub mod catalog;
pub mod catalog_pipeline;
pub mod catalog_update;
pub mod collector;
pub mod config;
pub mod documentation;
pub mod fishing;
pub mod game;
pub mod icon_cache;
pub mod input;
pub mod logging;
pub mod pixelbus;
pub mod platform;
pub mod potion;
pub mod weave;

/// Returns the crate version, single-sourced from `Cargo.toml`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
