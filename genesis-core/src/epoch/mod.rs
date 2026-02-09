//! Epoch markers for cosmic evolution stages
//!
//! Defines epoch markers representing different phases of cosmological evolution.
//! In Phase 1, only the Singularity epoch exists.

pub mod camera_config;
pub mod singularity;

pub use camera_config::{CameraMode, EpochCameraConfig};
pub use singularity::SingularityEpoch;
