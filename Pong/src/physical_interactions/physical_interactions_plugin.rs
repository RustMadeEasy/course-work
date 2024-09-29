// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::app::{App, FixedUpdate, Plugin};
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::math::Vec2;
use bevy::prelude::{in_state, EventWriter, IntoSystemConfigs, Query, Transform, Window, With};
use bevy::window::PrimaryWindow;

use crate::ball::ball_component::BallComponent;
use crate::ball::ball_plugin::BALL_RADIUS;
use crate::game_controller::GamePlayState;
use crate::paddle::paddle_component::PaddleComponent;
use crate::paddle::paddle_plugin::{PADDLE_HEIGHT, PADDLE_WIDTH};
use crate::physical_interactions::collision_event::CollisionEvent;
use crate::physical_interactions::physical_interactions_actor::PhysicalInteractionActor;

/// Manages the movements and interactions between the Ball, Ceiling, Floor, and Walls. This
/// includes controlling the ball direction, providing hit-detection, and invoking collision sounds.
pub(crate) struct PhysicalInteractionsPlugin;

impl Plugin for PhysicalInteractionsPlugin {
    // Constructs the plugin.
    fn build(&self, app: &mut App) {
        app //
            .add_event::<CollisionEvent>()
            .add_systems(
                FixedUpdate,
                Self::ball_and_paddle_interaction.run_if(in_state(GamePlayState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                Self::ball_and_wall_interaction.run_if(in_state(GamePlayState::Playing)),
            );
    }
}

impl PhysicalInteractionsPlugin {
    //

    /// Handles the interaction between the Ball, Ceiling, Floor, and Walls. This includes
    /// hit-detection, and invocation of collision sounds.
    fn ball_and_wall_interaction(
        mut ball_query: Query<(&Transform, &mut BallComponent)>,
        mut event_writer: EventWriter<CollisionEvent>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        if let (Ok(window), Ok((transform, mut ball))) =
            (window_query.get_single(), ball_query.get_single_mut())
        {
            //

            let mut collision_occurred = false;
            let mut new_ball_direction: Vec2 = ball.get_direction();
            let mut target: PhysicalInteractionActor = PhysicalInteractionActor::None;

            // Do we need to change the X direction?
            if DirectionDetector::does_vector_point_left(&new_ball_direction)
                && ((transform.translation.x - BALL_RADIUS) <= 0_f32)
            {
                // If the ball is traveling towards the left wall and has hit it, then reverse the x-direction
                new_ball_direction.x = -new_ball_direction.x;
                collision_occurred = true;
                target = PhysicalInteractionActor::SideWall;
            } else if DirectionDetector::does_vector_point_right(&ball.get_direction())
                && ((transform.translation.x + BALL_RADIUS) >= window.width())
            {
                // If the ball is traveling towards the right wall and has hit it, then reverse the x-direction
                new_ball_direction.x = -new_ball_direction.x;
                collision_occurred = true;
                target = PhysicalInteractionActor::SideWall;
            }

            // Do we need to change the Y direction?
            if DirectionDetector::does_vector_point_up(&new_ball_direction)
                && ((transform.translation.y + BALL_RADIUS) >= window.height())
            {
                // If the ball is traveling towards the ceiling and has hit it, then reverse the y-direction
                new_ball_direction.y = -new_ball_direction.y;
                collision_occurred = true;
                target = crate::physical_interactions::physical_interactions_plugin::PhysicalInteractionActor::Ceiling;
            } else if DirectionDetector::does_vector_point_down(&ball.get_direction())
                && ((transform.translation.y - BALL_RADIUS) <= 0_f32)
            {
                //

                // If the ball is traveling towards the bottom wall and has hit it, then reverse the y-direction
                new_ball_direction.y = -new_ball_direction.y;
                collision_occurred = true;

                target = PhysicalInteractionActor::Floor;
            }

            if collision_occurred {
                //

                ball.set_direction(new_ball_direction);

                // Post an event so that the other areas of the code know that the Ball has
                // hit one the confines of the room.
                event_writer.send(CollisionEvent::new(
                    PhysicalInteractionActor::Ball,
                    target,
                ));
            }
        }
    }

    /// Handles the interaction between the Ball and the Paddle. This includes hit-detection,
    /// and invocation of collision sounds.
    fn ball_and_paddle_interaction(
        mut ball_query: Query<(&Transform, &mut BallComponent)>,
        mut event_writer: EventWriter<CollisionEvent>,
        paddle_query: Query<&Transform, With<PaddleComponent>>,
    ) {
        //

        if let Ok((ball_transform, mut ball)) = ball_query.get_single_mut() {
            //

            if let Ok(paddle_transform) = paddle_query.get_single() {
                //

                // *** Manually test for intersection of the Ball into the bounds of the Paddle. ***

                // NOTE: Unfortunately, we can't use Bevy's built-in translation::distance()
                // function to detect proximity (collision) because we are not working wth two
                // spherical shapes.

                let paddle_position = Vec2::new(
                    paddle_transform.translation.x,
                    paddle_transform.translation.y,
                );
                let paddle_half_size = Vec2::new(PADDLE_WIDTH / 2_f32, PADDLE_HEIGHT / 2_f32);
                let paddle_bounds = Aabb2d::new(paddle_position, paddle_half_size);

                let ball_center =
                    Vec2::new(ball_transform.translation.x, ball_transform.translation.y);
                let ball_bounds = BoundingCircle::new(ball_center, BALL_RADIUS);

                let mut new_ball_direction: Vec2 = ball.get_direction();

                // Are they touching?
                if ball_bounds.intersects(&paddle_bounds) {
                    //

                    // *** Reverse the Ball's vertical direction ***

                    // Prevent Ball jitter by only changing the direction upwards
                    let previous_vertical_direction = new_ball_direction.y;
                    new_ball_direction.y = new_ball_direction.y.abs();

                    if new_ball_direction.y != previous_vertical_direction {
                        //

                        ball.set_direction(new_ball_direction);

                        // Post an event so that the other areas of the code know that the Ball has
                        // hit the Paddle.
                        event_writer.send(CollisionEvent::new(
                            PhysicalInteractionActor::Ball,
                            PhysicalInteractionActor::Paddle,
                        ));
                    }
                }
            }
        }
    }
}

/// Determines the direction of a vector.
pub(crate) struct DirectionDetector;

impl DirectionDetector {
    //

    /// Determines whether the specified vector points Down.
    fn does_vector_point_down(direction: &Vec2) -> bool {
        direction.y < 0_f32
    }

    /// Determines whether the specified vector points Left.
    fn does_vector_point_left(direction: &Vec2) -> bool {
        direction.x < 0_f32
    }

    /// Determines whether the specified vector points Right.
    fn does_vector_point_right(direction: &Vec2) -> bool {
        direction.x > 0_f32
    }

    /// Determines whether the specified vector points Up.
    fn does_vector_point_up(direction: &Vec2) -> bool {
        direction.y > 0_f32
    }
}
