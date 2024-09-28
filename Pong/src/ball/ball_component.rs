// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::math::Vec2;
use bevy::prelude::Component;
use rand::{thread_rng, Rng};

// Ball movement control levers
const BALL_DIRECTION_VARIABILITY: f32 = 0.19;
const GRAVITY: f32 = 1.015;
const LATERAL_DIRECTION_REDUCTION_FACTOR: f32 = 1.04;

/// Marker for Ball entities.
#[derive(Component)]
pub(crate) struct BallComponent {
    /// Specifies the Ball's current direction.
    direction: Vec2,
}

/// Public contract
impl BallComponent {
    /// Returns the ball's current direction.
    pub(crate) fn get_direction(&self) -> Vec2 {
        self.direction
    }

    /// Creates a new BallComponent instance.
    pub(crate) fn new(direction: Vec2) -> Self {
        Self { direction }
    }

    /// Modifies and finalizes the Ball's direction, including introducing gravity and a
    /// level of variability. Use this method to set the Ball's direction instead of setting it
    /// directly.
    pub(crate) fn set_direction(&mut self, new_direction: Vec2) {
        //

        let mut new_direction = new_direction;

        // Make the game a little more fun by providing some realism in the form of variability.
        self.randomize_ball_direction(&mut new_direction);
        Self::add_gravity(&mut new_direction);
        self.direction = new_direction.normalize();
    }
}

/// Helper functions
impl BallComponent {
    //

    /// Orients the Ball with a bit of downward trajectory in order to simulate gravity.
    fn add_gravity(ball_direction: &mut Vec2) {
        ball_direction.x /= LATERAL_DIRECTION_REDUCTION_FACTOR;
        ball_direction.y *= GRAVITY;
    }

    /// Provides a bit of randomness to the Ball's direction.
    fn randomize_ball_direction(&mut self, ball_direction: &mut Vec2) {
        let direction_variability = thread_rng().gen_range(-BALL_DIRECTION_VARIABILITY..=BALL_DIRECTION_VARIABILITY);
        ball_direction.x += direction_variability;
        ball_direction.y += direction_variability;
    }
}
