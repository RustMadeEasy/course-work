use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::prelude::{
    in_state, Commands, IntoSystemConfigs, KeyCode, Query, Res, SpriteBundle, Time, Transform,
    Update, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::game_controller::GamePlayState;
use crate::paddle::paddle_component::PaddleComponent;
use crate::physical_interactions::{DIRECTION_BACKWARD, DIRECTION_FORWARD};

const PADDLE_DEPTH: f32 = 1_f32;
pub(crate) const PADDLE_WIDTH: f32 = 310_f32;
pub(crate) const PADDLE_HEIGHT: f32 = 30_f32;
const PADDLE_SPEED: f32 = 700_f32;
const PADDLE_SPRITE: &str = "sprites/paddle_12.png";

pub(crate) struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, Self::spawn_paddle)
            .add_systems(
                Update,
                Self::move_paddle.run_if(in_state(GamePlayState::Playing)),
            );
    }
}

impl PaddlePlugin {
    //

    /// Implements the Paddle's side-to-side movement.
    fn move_paddle(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut paddle_query: Query<&mut Transform, With<PaddleComponent>>,
        time: Res<Time>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        if let (Ok(window), Ok(mut paddle_transform)) =
            (window_query.get_single(), paddle_query.get_single_mut())
        {
            //

            let new_direction = {
                if keyboard_input.pressed(KeyCode::ArrowLeft) {
                    DIRECTION_BACKWARD
                } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                    DIRECTION_FORWARD
                } else {
                    return;
                }
            };

            let left_edge = PADDLE_WIDTH / 2_f32;
            let right_edge = window.width() - PADDLE_WIDTH / 2_f32;

            let new_position_x = paddle_transform.translation.x
                + (new_direction * PADDLE_SPEED * time.delta_seconds());

            // Constrain the Paddle to the window
            let new_position_x = new_position_x.clamp(left_edge, right_edge);

            paddle_transform.translation.x = new_position_x;
        }
    }

    /// Creates the Paddle entity.
    fn spawn_paddle(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        if let Ok(window) = window_query.get_single() {
            //

            let x = window.width() / 2.0;

            let sprite_bundle = SpriteBundle {
                transform: Transform::from_xyz(x, PADDLE_HEIGHT / 2_f32, PADDLE_DEPTH),
                /*.with_scale(scale)*/
                texture: asset_server.load(PADDLE_SPRITE),
                ..Default::default()
            };
            commands.spawn((sprite_bundle, PaddleComponent {}));
        }
    }
}
