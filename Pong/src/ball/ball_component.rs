// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::math::Vec2;
use bevy::prelude::Component;
use rand::Rng;

// Ball direction control levers
const BALL_DIRECTION_VARIABILITY: f32 = 0.17;
const LATERAL_DIRECTION_REDUCTION_FACTOR: f32 = 1.04;
const VERTICAL_PULL: f32 = 1.015;

/// Marker for Ball entities.
#[derive(Component)]
pub(crate) struct BallComponent {
    /// Specifies the Ball's current direction.
    direction: Vec2,
}

impl BallComponent {
    //

    /// Provides the Ball with a bit of downward direction in order to simulate gravity.
    fn add_gravity(ball_direction: &mut Vec2) {
        ball_direction.x /= LATERAL_DIRECTION_REDUCTION_FACTOR;
        ball_direction.y *= VERTICAL_PULL;
    }

    pub(crate) fn get_direction(&self) -> Vec2 {
        self.direction
    }

    /// Creates a new BallComponent instance.
    pub(crate) fn new(direction: Vec2) -> Self {
        Self { direction }
    }

    /// Provides a bit of randomness to the Ball's direction.
    fn randomize_ball_direction(ball_direction: &mut Vec2) {
        let mut rng = rand::thread_rng();
        let direction_variability =
            rng.gen_range(-BALL_DIRECTION_VARIABILITY..=BALL_DIRECTION_VARIABILITY);
        ball_direction.x += direction_variability;
        ball_direction.y += direction_variability;
    }

    /// Modifies and finalizes the Ball's direction, including introducing gravity and a
    /// level of variability. Use this method to set the Ball's direction instead of setting it
    /// directly.
    pub(crate) fn set_direction(&mut self, new_direction: Vec2) {
        let mut new_direction = new_direction;

        // Make the game a little more fun by providing some realism in the form of variability.
        Self::randomize_ball_direction(&mut new_direction);
        Self::add_gravity(&mut new_direction);
        self.direction = new_direction.normalize();
    }
}
