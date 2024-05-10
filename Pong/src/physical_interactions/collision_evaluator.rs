use crate::physical_interactions::collision_event::CollisionEvent;
use crate::physical_interactions::physical_interactions_actor::PhysicalInteractionActor;

/// Provides plain-language evaluation of collision events. It can be used as follows:
///
/// ```
///     let evaluator = CollisionEvaluator::new(collision_event);
///     if evaluator.did(Ball).collide_with(Ceiling).or(SideWall).evaluate() {
///         ... do something here ...
///     }
/// ```
pub(crate) struct CollisionEvaluator {
    collision_source: Option<PhysicalInteractionActor>,
    collision_targets: Vec<PhysicalInteractionActor>,
    event: Option<CollisionEvent>,
}

impl CollisionEvaluator {
    //

    /// Creates a CollisionEvaluator instance.
    pub(crate) fn new(collision: &CollisionEvent) -> Self {
        Self {
            collision_source: None,
            collision_targets: vec![],
            event: Some(collision.clone()),
        }
    }
}

impl CollisionEvaluator {
    //

    /// Sets the Source actor in the collision phrase.
    pub(crate) fn did(&mut self, actor1: PhysicalInteractionActor) -> &mut Self {
        self.collision_source = Some(actor1);
        self
    }

    /// Adds a target actor to the collision phrase.
    pub(crate) fn collide_with(&mut self, potential_target: PhysicalInteractionActor) -> &mut Self {
        self.collision_targets.push(potential_target);
        self
    }

    /// Adds a target actor to the collision phrase. Synonymous with collide_with().
    pub(crate) fn or(&mut self, potential_target: PhysicalInteractionActor) -> &mut Self {
        self.collide_with(potential_target)
    }

    /// Performs final evaluation of the collision phrase and prepares the instance for building
    /// another phrase.
    pub(crate) fn evaluate(&mut self) -> bool {
        //

        let result = if let (Some(actor1), Some(collision)) =
            (self.collision_source.clone(), self.event.clone())
        {
            collision.source == actor1 && self.collision_targets.contains(&collision.target)
        } else {
            // Warn if the function were not called properly.
            println!("WARNING: check() called before requisite call to did(), with() and/or or()!");
            false
        };

        self.collision_source = None;
        self.collision_targets.clear();

        result
    }
}
