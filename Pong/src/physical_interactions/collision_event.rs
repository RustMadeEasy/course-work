use bevy::prelude::Event;

use crate::physical_interactions::physical_interactions_actor::PhysicalInteractionActor;

/// Models a collision between two physical actors. 
#[derive(Clone, Event)]
pub(crate) struct CollisionEvent {
    pub(super) _accuracy: f32,
    pub(super) source: PhysicalInteractionActor,
    pub(super) target: PhysicalInteractionActor,
}

impl CollisionEvent {
    //

    /// Constructs a new CollisionEvent instance.
    pub(super) fn new(
        accuracy: f32,
        source: PhysicalInteractionActor,
        target: PhysicalInteractionActor,
    ) -> Self {
        Self {
            _accuracy: accuracy,
            source,
            target,
        }
    }
}
