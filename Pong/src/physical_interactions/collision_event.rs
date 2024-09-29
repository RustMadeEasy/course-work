// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::prelude::Event;

use crate::physical_interactions::physical_interactions_actor::PhysicalInteractionActor;

/// Models a collision between two physical actors, e.g. the ball and the floor.
#[derive(Clone, Event)]
pub(crate) struct CollisionEvent {
    pub(super) source: PhysicalInteractionActor,
    pub(super) target: PhysicalInteractionActor,
}

impl CollisionEvent {
    //

    /// Constructs a new CollisionEvent instance.
    pub(super) fn new(
        source: PhysicalInteractionActor,
        target: PhysicalInteractionActor,
    ) -> Self {
        Self {
            source,
            target,
        }
    }
}
