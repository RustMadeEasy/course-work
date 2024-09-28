// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    in_state, Commands, IntoSystemConfigs, Query, Res, SpriteBundle, Time, Transform, Update,
    Window, With,
};
use bevy::window::PrimaryWindow;
use rand::random;

use crate::ball::ball_component::BallComponent;
use crate::game_controller::GamePlayState;
use crate::physical_interactions::DIRECTION_BACKWARD;

const BALL_DIAMETER: f32 = 64_f32;
pub(crate) const BALL_RADIUS: f32 = 32_f32;
const BALL_SPAWN_LATERAL_RANDOMNESS_FACTOR: f32 = 3_f32;
const BALL_SPEED: f32 = 700_f32;
const BALL_SPRITE: &str = "sprites/ball_blue_large.png";
const BALL_Z_INDEX: f32 = 1_f32;

/// Handles Ball presentation and movement.
pub(crate) struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, Self::spawn_ball)
            .add_systems(
                Update,
                Self::move_ball.run_if(in_state(GamePlayState::Playing)),
            );
    }
}

impl BallPlugin {
    //

    /// Implements frame-by-frame movement of the Ball along the direction specified in the
    /// BallComponent's get_direction() function. See BallComponent.
    fn move_ball(mut ball_query: Query<(&mut Transform, &BallComponent)>, time: Res<Time>) {
        if let Ok((mut transform, ball)) = ball_query.get_single_mut() {
            let translation = ball.get_direction() * BALL_SPEED * time.delta_seconds();
            transform.translation += translation.extend(0_f32);
        }
    }

    /// Spawns the Ball and sets its initial direction.
    fn spawn_ball(
        asset_server: Res<AssetServer>,
        mut commands: Commands,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        if let Ok(window) = window_query.get_single() {
            //

            // Start from the top-middle of the view
            let start_point: Vec3 = Vec3::new(
                window.width() / 2.0,
                window.height() - BALL_DIAMETER,
                BALL_Z_INDEX,
            );

            let sprite_bundle = SpriteBundle {
                transform: Transform::from_translation(start_point),
                texture: asset_server.load(BALL_SPRITE),
                ..Default::default()
            };

            // Aim the Ball in the general direction of the Paddle
            let initial_x_direction = random::<f32>() / BALL_SPAWN_LATERAL_RANDOMNESS_FACTOR;
            let initial_y_direction = DIRECTION_BACKWARD * random::<f32>();
            let initial_direction = Vec2::new(initial_x_direction, initial_y_direction).normalize();

            commands.spawn((sprite_bundle, BallComponent::new(initial_direction)));
        }
    }
}
