// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::prelude::{in_state, IntoSystemConfigs};
use bevy::prelude::{App, EventReader, FixedUpdate, Plugin, ResMut, Startup};
use bevy::prelude::{
    Color, Commands, DetectChanges, PositionType, Query, Res, TextBundle, TextSection, Window, With,
};
use bevy::text::{Text, TextStyle};
use bevy::ui::{Style, Val};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use lazy_static::lazy_static;

use crate::game_controller::GamePlayState;
use crate::physical_interactions::collision_evaluator::CollisionEvaluator;
use crate::physical_interactions::collision_event::CollisionEvent;
use crate::physical_interactions::physical_interactions_actor::PhysicalInteractionActor::{
    Ball, Floor, Paddle,
};
use crate::scoreboard::scoreboard_resource::ScoreboardResource;
use crate::scoreboard::scoreboard_ui_component::ScoreBoardUiComponent;

// Scoring (current scheme):
// Points are granted for each Ball return.
// Points are taken away for missing the Ball.

// TODO: JD: Future scoring ideas:
// The closer to the paddle center the ball was returned, the more the points granted.
// The further the ball when missed, the more the demerits.
// Increase the Ball speed and direction variability as the points increase.
// Decrease the Paddle speed over time.
// Decrease the Paddle width over time.

// Scoring parameters
const MAX_DEMERITS_FOR_MISSING_BALL: i64 = 5;
const MAX_POINTS_TO_GRANT_FOR_RETURNING_BALL: i64 = 5;

const SCOREBOARD_FONT_SIZE: f32 = 35.0;

lazy_static! {
    static ref SCOREBOARD_TEXT_COLOR: Color = Color::hex("2f2f2f").unwrap();
}

pub(crate) struct ScoreboardPlugin;

/// Displays the game score in realtime.
impl Plugin for ScoreboardPlugin {
    /// Constructs the plugin.
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(ScoreboardResource::default())
            .add_systems(Startup, Self::spawn_scoreboard_ui)
            .add_systems(
                FixedUpdate,
                (
                    Self::handle_physical_interaction_events,
                    Self::update_scoreboard,
                )
                    .chain()
                    .run_if(in_state(GamePlayState::Playing)),
            );
    }
}

impl ScoreboardPlugin {
    //

    /// Listens for collision event and updates the score resource based on how the Paddle and Ball
    /// are interacting with the environment.
    fn handle_physical_interaction_events(
        mut event_reader: EventReader<CollisionEvent>,
        mut scoreboard_resource: ResMut<ScoreboardResource>,
    ) {
        //

        // Look for the following situations:
        // a) The Ball hitting the Floor.
        // b) The Paddle returning the Ball.
        for collision_event in event_reader.read() {
            //

            let mut evaluator = CollisionEvaluator::new(collision_event);

            if evaluator.did(Ball).collide_with(Floor).evaluate() {
                //

                // Give demerits for hitting the floor because it means the Player
                // missed the Ball.
                let new_score: i64 = scoreboard_resource.score - MAX_DEMERITS_FOR_MISSING_BALL;
                scoreboard_resource.score = new_score.clamp(0, i64::MAX);
            } else if evaluator.did(Ball).collide_with(Paddle).evaluate() {
                //

                // Grant points for the Ball being returned by the Paddle.
                scoreboard_resource.score += MAX_POINTS_TO_GRANT_FOR_RETURNING_BALL;
            }
        }
    }

    /// Spawns the scoreboard UI.
    fn spawn_scoreboard_ui(
        mut commands: Commands,
        _window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        let sections = [
            TextSection::new(
                "Score: ",
                TextStyle {
                    color: *SCOREBOARD_TEXT_COLOR,
                    font: Default::default(),
                    font_size: SCOREBOARD_FONT_SIZE,
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    color: *SCOREBOARD_TEXT_COLOR,
                    font: Default::default(),
                    font_size: SCOREBOARD_FONT_SIZE,
                },
            ),
        ];

        let style = Style {
            display: Default::default(),
            position_type: PositionType::Absolute,
            left: Val::Px(30.0),
            top: Val::Px(30.0),
            ..default()
        };

        let text_bundle = TextBundle::from_sections(sections).with_style(style);

        commands.spawn((text_bundle, ScoreBoardUiComponent {}));
    }

    /// Keeps the Scoreboard UI updated with the latest score.
    fn update_scoreboard(
        scoreboard_resource: Res<ScoreboardResource>,
        mut text_query: Query<&mut Text, With<ScoreBoardUiComponent>>,
    ) {
        if scoreboard_resource.is_changed() {
            if let Ok(mut text_sections) = text_query.get_single_mut() {
                text_sections.sections[1].value = scoreboard_resource.score.to_string();
            }
        }
    }
}
