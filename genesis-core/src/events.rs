//! Event types for GENESIS simulation
//!
//! This module defines events that are emitted by the simulation
//! and can be listened to by other systems.

use bevy::prelude::Event;

/// Event emitted when the user scrubs the timeline
///
/// This event is used to notify other systems (such as particle rendering)
/// when timeline scrubbing starts and ends, so they can respond appropriately
/// (e.g., pause particle animations during scrubbing).
///
/// # Fields
///
/// * `is_scrubbing` - `true` when the user is actively scrubbing, `false` when scrubbing ends
///
/// # Usage
///
/// Systems can listen to this event by querying `EventReader<ScrubbingEvent>`:
/// ```no_run
/// fn handle_scrubbing(mut events: EventReader<ScrubbingEvent>) {
///     for event in events.read() {
///         if event.is_scrubbing {
///             // Scrubbing started - pause particle animations
///         } else {
///             // Scrubbing ended - resume particle animations
///         }
///     }
/// }
/// ```
#[derive(Event, Debug, Clone)]
pub struct ScrubbingEvent {
    /// `true` when the user is actively scrubbing the timeline, `false` when scrubbing ends
    pub is_scrubbing: bool,
}
