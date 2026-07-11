//! ESO Weave foundations library.
//!
//! This crate carries the project foundations: the Config Store
//! ([`config`]) and the Logging subsystem ([`logging`]), plus a per-platform
//! path seam ([`platform`]) that later input and sampling backends extend.

pub mod config;
pub mod input;
pub mod logging;
pub mod platform;
pub mod weave;

/// Returns the crate version, single-sourced from `Cargo.toml`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
