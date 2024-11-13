//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use std::time::Duration;

use crate::game_play_screen::{OnGamePlayScreen, TilePressedEvent};
use crate::shared::api_helpers::{GameStateCache, SDK_CONFIG};
use crate::shared::app_mode::AppMode;
use crate::shared::app_state_resource::AppStateResource;
use crate::shared::despawn;
use bevy::app::App;
use bevy::log::error;
use bevy::prelude::{in_state, EventReader, EventWriter, FixedUpdate, IntoSystemConfigs, NextState, OnEnter, OnExit, Plugin, ResMut, Update};
use bevy::time::common_conditions::on_timer;
use helpers_for_bevy::status_text::events::SetStatusTextEvent;
use tic_tac_toe_rust_client_sdk::apis::{tic_tac_toe_api, Error};
use tic_tac_toe_rust_client_sdk::models::{AutomaticPlayerSkillLevel, GameCreationResponse, GamePiece, GameTurnParams, GamingSessionCreationResponse, JoinSessionParams, NewGamingSessionParams, NewSinglePlayerGameParams, PlayStatus};

/// Provides the local, client-side logic that works with our TicTacToe Game Service.
pub(super) struct LocalGamePlayPlugin;

pub(super) const STATE_UPDATE_INTERVAL_IN_MS: u64 = 500;

impl Plugin for LocalGamePlayPlugin {
    //

    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(AppMode::GamePlay), Self::join_or_begin_new_game)
            .add_systems(
                Update,
                Self::handle_tile_pressed.run_if(in_state(AppMode::GamePlay)),
            )
            .add_systems(
                FixedUpdate,
                LocalGamePlayPlugin::refresh_game_state
                    .run_if(in_state(AppMode::GamePlay))
                    .run_if(on_timer(Duration::from_millis(STATE_UPDATE_INTERVAL_IN_MS))),
            )
            .add_systems(OnExit(AppMode::GamePlay), despawn::<OnGamePlayScreen>);
    }
}

impl LocalGamePlayPlugin {
    //

    /// Handles the Tile Pressed event.
    fn handle_tile_pressed(
        mut app_state: ResMut<AppStateResource>,
        mut event_reader: EventReader<TilePressedEvent>,
        mut event_writer: EventWriter<SetStatusTextEvent>,
    ) {
        //

        for event in event_reader.read() {
            //

            // Ignore clicks if the Game has yet to begin.
            if !app_state.has_game_started {
                event_writer.send(SetStatusTextEvent::new_with_duration(
                    "Waiting for another player to join game. Please send out the invitation code.",
                    Duration::from_secs(10),
                ));
                return;
            }

            // Ignore clicks if the Game has already ended.
            if app_state.has_game_ended {
                event_writer.send(SetStatusTextEvent::new_with_duration(
                    "Game has already ended.",
                    Duration::from_secs(30),
                ));
                return;
            }

            // Ignore clicks for Tiles on which pieces have already been placed.
            if app_state.get_game_piece_at_placement(&event.grid_position)
                != GamePiece::Unselected
            {
                event_writer.send(SetStatusTextEvent::new_with_duration(
                    "This location is already occupied. Please choose another location.",
                    Duration::from_secs(5),
                ));
                return;
            }

            let not_local_player_turn = format!(
                "Please wait for {} to take their turn.",
                app_state.other_player.clone().unwrap_or_default().display_name
            );

            // Ignore clicks if it is not the local Player's turn.
            if let Some(current_player) = app_state.current_player.clone() {
                if app_state.local_player.player_id != current_player.player_id {
                    event_writer.send(SetStatusTextEvent::new_with_duration(
                        not_local_player_turn,
                        Duration::from_secs(10),
                    ));
                    return;
                }
            }

            // Reflect the move immediately in the UI.
            //
            // NOTE: Because it, necessarily, takes time for the latest game state to be retrieved
            // and to be reflected in the UI, let's be optimistic and immediately update the UI
            // with the proposed game move. The call to the service may fail or even be rejected,
            // e.g. the other Player has abandoned the Game, etc. However, it is better to support
            // the usual case.
            app_state.current_game_state.game_board[event.grid_position.row as usize][event.grid_position.column as usize] =
                app_state.local_player.game_piece;

            // Call our remote Game play server to take the turn.
            let params = GameTurnParams {
                destination: event.grid_position.clone(),
                player_id: app_state.local_player.player_id.clone(),
                session_id: app_state.gaming_session_id.clone(),
            };
            match tic_tac_toe_api::take_turn(
                &SDK_CONFIG,
                &app_state.game_id,
                params) {
                Ok(_) => {}
                Err(error) => {
                    let message = match error {
                        Error::ResponseError(error) => {
                            if error.status == 405 {
                                not_local_player_turn
                            } else {
                                "Problem contacting the TicTacToe server.".to_string()
                            }
                        }
                        _ => "Problem communicating with the TicTacToe server.".to_string(),
                    };
                    event_writer.send(SetStatusTextEvent::new_with_duration(
                        message,
                        Duration::from_secs(10),
                    ));
                }
            }
        }
    }
}

impl LocalGamePlayPlugin {
    //

    fn create_two_player_game(app_state: &AppStateResource) -> Option<GameCreationResponse> {
        //

        let new_game_state = match tic_tac_toe_api::create_two_player_game(&SDK_CONFIG, &app_state.gaming_session_id) {
            Ok(new_game_state) => new_game_state,
            Err(error) => {
                // TODO: JD: finish
                error!("Error joining gaming session: {:?}", error);
                return None;
            }
        };

        Some(new_game_state)
    }

    fn create_single_player_game(app_state: &AppStateResource) -> Option<GameCreationResponse> {
        //

        let params = NewSinglePlayerGameParams { computer_skill_level: AutomaticPlayerSkillLevel::Beginner };
        let new_game_state = match tic_tac_toe_api::create_single_player_game(&SDK_CONFIG, &app_state.gaming_session_id, params) {
            Ok(new_game_state) => new_game_state,
            Err(error) => {
                // TODO: JD: finish
                error!("Error joining gaming session: {:?}", error);
                return None;
            }
        };

        Some(new_game_state)
    }

    /// Starts a new Game or joins an existing Game - depending upon whether the local Player is
    /// initiating the Game.
    fn join_or_begin_new_game(
        mut app_state: ResMut<AppStateResource>,
        mut event_writer: EventWriter<SetStatusTextEvent>,
        mut next_state: ResMut<NextState<AppMode>>,
    ) {
        //

        let gaming_session_info: GamingSessionCreationResponse;

        if app_state.local_player_initiated_gaming_session {
            //

            // *** Create a new Gaming Session ***

            let params = NewGamingSessionParams { session_owner_display_name: app_state.local_player.display_name.clone() };
            gaming_session_info = match tic_tac_toe_api::create_gaming_session(&SDK_CONFIG, params) {
                Ok(result) => {
                    result
                }
                Err(error) => {
                    error!("Error creating gaming session: {:?}", error);
                    return;
                }
            };

            app_state.gaming_session_id = gaming_session_info.session_id.clone();
            if app_state.is_two_player_game {
                app_state.invitation_code = gaming_session_info.invitation_code;
            } else {
                app_state.invitation_code = "".to_string(); // Not needed for Single-Player Game.
            }
            app_state.local_player = gaming_session_info.initiating_player.clone();

            // *** Create a new Game ***

            let game_creation_function = match app_state.is_two_player_game {
                true => {
                    Self::create_two_player_game
                }
                false => Self::create_single_player_game,
            };
            let _ = game_creation_function(&app_state);
        } else {
            //

            // Join an existing Gaming Session via Invitation Code

            app_state.local_player_initiated_gaming_session = false;

            let params = JoinSessionParams {
                game_invitation_code: app_state.invitation_code.clone(),
                player_display_name: app_state.local_player.display_name.clone(),
            };
            gaming_session_info = match tic_tac_toe_api::join_gaming_session(&SDK_CONFIG, params) {
                Ok(gaming_session_info) => gaming_session_info,
                Err(error) => {
                    // TODO: JD: localize the text.
                    error!("Error joining Gaming Session: {:?}", error);
                    let message = match error {
                        Error::ResponseError(error) => {
                            match error.status {
                                reqwest::StatusCode::NOT_FOUND => "Gaming Session not found.",
                                reqwest::StatusCode::INTERNAL_SERVER_ERROR => "Internal server error",
                                _ => "An unexpected error was returned from the TicTacToe server.",
                            }
                        }
                        _ => "An unexpected error was returned from the TicTacToe server.",
                    };
                    event_writer.send(SetStatusTextEvent::new_with_duration(
                        message,
                        Duration::from_secs(5),
                    ));
                    next_state.set(AppMode::StartMenu);
                    return;
                }
            };

            app_state.gaming_session_id = gaming_session_info.session_id.clone();
            app_state.invitation_code = "".to_string(); // Not needed when accepting the invitation.
            app_state.local_player = gaming_session_info.other_player.unwrap_or_default().unwrap_or_default();
            app_state.local_player_initiated_gaming_session = false;

            app_state.is_two_player_game = true;
        };

        // *** Join the Game ***

        let game_creation_response = match tic_tac_toe_api::join_current_game(&SDK_CONFIG, &gaming_session_info.session_id, &app_state.local_player.player_id) {
            Ok(response) => response,
            Err(error) => {
                error!("Error joining Game: {:?}", error);
                // TODO: JD: localize the text.
                let message = match error {
                    Error::ResponseError(error) => {
                        match error.status {
                            reqwest::StatusCode::NOT_FOUND => "Game not found.",
                            reqwest::StatusCode::BAD_REQUEST => "Bad request - Game not started",
                            reqwest::StatusCode::INTERNAL_SERVER_ERROR => "Internal server error",
                            _ => "An unexpected error was returned from the TicTacToe server.",
                        }
                    }
                    _ => "An unexpected error was returned from the TicTacToe server.",
                };
                event_writer.send(SetStatusTextEvent::new_with_duration(
                    message,
                    Duration::from_secs(5),
                ));
                next_state.set(AppMode::StartMenu);
                return;
            }
        };
        // app_state.other_player = game_creation_response.other_player.unwrap_or_default();
        app_state.game_id = game_creation_response.game_info.game_id;
        if app_state.local_player_initiated_gaming_session {
            app_state.local_player = game_creation_response.initiating_player;
            app_state.other_player = game_creation_response.other_player.unwrap_or_default();
        } else {
            if let Some(Some(player)) = game_creation_response.other_player {
                app_state.local_player = player
            }
            app_state.other_player = Some(game_creation_response.initiating_player);
        }
        app_state.current_game_state = game_creation_response.game_info.game_state.clone();
        app_state.has_game_ended = false;

        // *** Begin listening for Game change events ***

        // TODO: JD: when the Game ends, we need to shut down the auto-update Task.

        // Point the background update thread to the new Game ID.
        GameStateCache::setup_auto_update(
            &app_state.game_id,
            &Duration::from_millis(STATE_UPDATE_INTERVAL_IN_MS / 2),
        );
    }

    /// Pulls the latest Game state from the GameStateCache.
    fn refresh_game_state(
        mut app_state: ResMut<AppStateResource>,
        mut event_writer: EventWriter<SetStatusTextEvent>,
    ) {
        //

        // Exit early if the game is no longer in progress.
        if app_state.game_id.is_empty() && app_state.has_game_ended {
            return;
        }

        let game_started_before_call = app_state.has_game_started;

        if app_state.has_game_started {

            // Grab the latest Turn info
            let turn_response = match GameStateCache::get_latest_game_turn(&app_state.game_id) {
                Ok(remote_game_info) => remote_game_info,
                Err(error) => {
                    // TODO: JD: localize the text.
                    let message = match error {
                        Error::ResponseError(error) => {
                            match error.status {
                                reqwest::StatusCode::NOT_FOUND => "Game not found.",
                                reqwest::StatusCode::BAD_REQUEST => "Bad request - Game not started",
                                reqwest::StatusCode::INTERNAL_SERVER_ERROR => "Internal server error",
                                _ => "An unexpected error was returned from the TicTacToe server.",
                            }
                        }
                        _ => "An unexpected error was returned from the TicTacToe server.",
                    };
                    event_writer.send(SetStatusTextEvent::new_with_duration(
                        message,
                        Duration::from_secs(5),
                    ));
                    return;
                }
            };

            app_state.current_game_state = turn_response.new_game_state.clone();
            app_state.current_player = turn_response.current_player.clone().unwrap_or_default();

            // If the Game has ended, let the user know the results.
            match app_state.current_game_state.play_status {
                PlayStatus::EndedInStalemate | PlayStatus::EndedInWin => {
                    let winning_player_name =
                        if app_state.current_game_state.play_status == PlayStatus::EndedInWin {
                            Some(
                                if app_state.local_player.player_id
                                    == app_state.current_game_state.id_of_player_who_made_move.clone()
                                {
                                    app_state.local_player.display_name.clone()
                                } else {
                                    app_state.other_player.clone().unwrap_or_default().display_name
                                },
                            )
                        } else {
                            None
                        };
                    let game_results = app_state.generate_results_text(
                        &turn_response,
                        &app_state.local_player.display_name,
                        &winning_player_name,
                    );
                    if !game_results.is_empty() {
                        event_writer.send(SetStatusTextEvent::new_with_duration(
                            game_results,
                            Duration::from_secs(5),
                        ));
                    }
                }
                _ => {}
            }
        } else {
            //

            if GameStateCache::get_latest_game_readiness(&app_state.game_id).unwrap_or_default() {
                //

                if let Ok(game_creation_response) = tic_tac_toe_api::get_session_current_game(&SDK_CONFIG, &app_state.gaming_session_id) {
                    if app_state.local_player_initiated_gaming_session {
                        app_state.other_player = game_creation_response.other_player.unwrap_or_default();
                    }
                    app_state.current_player = game_creation_response.game_info.current_player.unwrap_or_default();
                    app_state.current_game_state = game_creation_response.game_info.game_state.clone();
                    app_state.has_game_started = true;
                }

                // The other Player has just joined. So, note their info and also inform the local Player.
                if app_state.local_player_initiated_gaming_session
                    && !game_started_before_call
                    && app_state.has_game_started
                {
                    let message = format!(
                        "{} has joined! Let the game begin!",
                        app_state.other_player.clone().unwrap_or_default().display_name
                    );
                    event_writer.send(SetStatusTextEvent::new_with_duration(
                        message,
                        Duration::from_secs(5),
                    ));
                }
            }
        }
    }
}
