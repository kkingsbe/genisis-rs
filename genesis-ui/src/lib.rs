//! GENESIS UI - User interface components
//!
//! This crate contains all UI elements using bevy_egui, including
//! timeline scrubbers, time controls, and information overlays.

pub mod timeline;
pub mod overlay;

/// Version of the UI library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
