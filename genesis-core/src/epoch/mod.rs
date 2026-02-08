//! Epoch management for cosmic evolution stages
//!
//! Defines the EpochPlugin trait and EpochManager resource for tracking
//! different epochs of cosmological evolution. Includes update_epoch_transition
//! system for automatic epoch transitions based on cosmic time.

use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use crate::time::TimeAccumulator;

/// Trait that all epoch plugins must implement
///
/// An epoch represents a distinct phase of cosmic evolution with unique
/// physical laws and phenomena.
pub trait EpochPlugin: Send + Sync {
    /// Returns the name of this epoch
    fn name(&self) -> &'static str;

    /// The cosmic year when this epoch begins
    fn start_year(&self) -> f64;

    /// The cosmic year when this epoch ends
    fn end_year(&self) -> f64;

    /// Allows the epoch to register its systems with the Bevy app
    ///
    /// This is called when the epoch plugin is registered, enabling
    /// epoch-specific systems to be added to the simulation.
    fn build(&self, app: &mut App);
}

/// Resource that manages all registered epoch plugins
///
/// The EpochManager tracks available epochs and maintains the currently
/// active epoch during simulation.
#[derive(Resource)]
pub struct EpochManager {
    /// Map of epoch name to epoch plugin
    epochs: HashMap<String, Arc<dyn EpochPlugin>>,
    /// The currently active epoch name, if any
    current_epoch: Option<String>,
}

impl EpochManager {
    /// Creates a new empty epoch manager
    pub fn new() -> Self {
        Self {
            epochs: HashMap::new(),
            current_epoch: None,
        }
    }

    /// Registers an epoch plugin with the manager
    ///
    /// # Arguments
    /// * `plugin` - The epoch plugin to register
    ///
    /// # Panics
    /// Panics if an epoch with the same name is already registered
    pub fn register_plugin(&mut self, plugin: Arc<dyn EpochPlugin>) {
        let name = plugin.name().to_string();
        if self.epochs.contains_key(&name) {
            panic!("Epoch '{}' is already registered", name);
        }
        self.epochs.insert(name, plugin);
    }

    /// Registers an epoch plugin and builds its systems
    ///
    /// This is a convenience method that both registers the plugin with the
    /// manager and calls its `build` method to register epoch-specific systems
    /// with the Bevy app.
    ///
    /// # Arguments
    /// * `plugin` - The epoch plugin to register
    /// * `app` - The Bevy app to build systems into
    ///
    /// # Panics
    /// Panics if an epoch with the same name is already registered
    pub fn register_and_build_plugin(&mut self, plugin: Arc<dyn EpochPlugin>, app: &mut App) {
        self.register_plugin(plugin.clone());
        plugin.build(app);
    }

    /// Returns a reference to the currently active epoch plugin, if any
    ///
    /// # Returns
    /// * `Some(Arc<dyn EpochPlugin>)` - If an epoch is currently active
    /// * `None` - If no epoch is currently active
    pub fn get_current_epoch(&self) -> Option<Arc<dyn EpochPlugin>> {
        self.current_epoch
            .as_ref()
            .and_then(|name| self.epochs.get(name).cloned())
    }

    /// Sets the currently active epoch by name
    ///
    /// # Arguments
    /// * `name` - The name of the epoch to activate
    ///
    /// # Returns
    /// * `true` - If the epoch was found and activated
    /// * `false` - If no epoch with that name exists
    #[doc(hidden)]
    pub fn set_current_epoch(&mut self, name: &str) -> bool {
        if self.epochs.contains_key(name) {
            self.current_epoch = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Returns an iterator over all registered epoch names
    ///
    /// # Returns
    /// An iterator yielding references to the names of all registered epochs
    pub fn epoch_names(&self) -> impl Iterator<Item = &String> {
        self.epochs.keys()
    }

    /// Gets the number of registered epochs
    pub fn epoch_count(&self) -> usize {
        self.epochs.len()
    }
}

impl Default for EpochManager {
    fn default() -> Self {
        Self::new()
    }
}

/// System that transitions between epochs based on cosmic time
///
/// This system queries the current cosmic time from `TimeAccumulator`
/// and determines which epoch should be active based on the time range
/// of each registered epoch. If the epoch has changed, it updates the
/// `EpochManager` to reflect the new active epoch.
fn update_epoch_transition(
    time: Res<TimeAccumulator>,
    mut manager: ResMut<EpochManager>,
) {
    let current_year = time.years;

    // Find the epoch whose time range contains the current year
    // Collect epoch names to avoid borrow checker issues
    let epoch_names: Vec<String> = manager.epoch_names().cloned().collect();

    for epoch_name in &epoch_names {
        if let Some(epoch) = manager.epochs.get(epoch_name) {
            let start_year = epoch.start_year();
            let end_year = epoch.end_year();

            // Check if current time falls within this epoch's range
            if current_year >= start_year && current_year < end_year {
                // Only update if the epoch has changed
                if manager.current_epoch.as_ref() != Some(epoch_name) {
                    manager.set_current_epoch(epoch_name);
                }
                break;
            }
        }
    }
}

/// Bevy plugin that initializes the epoch manager resource
///
/// This plugin inserts the `EpochManager` resource into the Bevy app,
/// enabling epoch management functionality throughout the simulation.
pub struct EpochManagerPlugin;

impl Plugin for EpochManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EpochManager::default())
            .add_systems(Update, update_epoch_transition);
    }
}
