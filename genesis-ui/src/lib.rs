//! GENESIS UI - User interface state and components
//!
//! This crate contains UI state resources and data structures.
//! Actual UI rendering using bevy_egui is not yet implemented.

pub mod timeline;
pub mod overlay;

/// Version of the UI library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
