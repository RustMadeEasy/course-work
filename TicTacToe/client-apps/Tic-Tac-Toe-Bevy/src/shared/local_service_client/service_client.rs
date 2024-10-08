//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use std::time::Duration;

use crate::shared::local_models::local_game_state::LocalGameStateResource;
use crate::shared::local_models::local_grid_position::LocalGridPosition;
use crate::shared::local_models::local_player_info::LocalPlayerInfo;
use crate::shared::local_service_client::helper_functions::remote_players_to_local_players;
use crate::shared::local_service_client::service_client::helpers::{GameInfoResult, AUTO_UPDATE_INFO, SDK_CONFIG};
use tic_tac_toe_rust_client_sdk::apis::tic_tac_toe_api::{
    AddPlayerError, CreateGameError, GetGameInfoError, TakeTurnError,
};
use tic_tac_toe_rust_client_sdk::apis::{tic_tac_toe_api, Error};
use tic_tac_toe_rust_client_sdk::models::{AddPlayerParams, BoardPosition, GameMode, GameTurnInfo, NewGameParams, SkillLevel};

/// Serves as a local client to the Tic-Tac-Toe service. This struct also caches the Game Info so
/// that it is accessed directly from memory. This prevents networking-induced lag in the game frame
/// updates.
pub(crate) struct LocalServiceClient;

impl LocalServiceClient {
    //

    // TODO: JD: REFACTOR: make sure that none of the methods return errors from tic_tac_toe_api
    // SDK. We want a separation between the app code and the client SDK code.

    /// Starts a new Tic-Tac-Toe Game and returns the initial Game state. Returns a tuple containing
    /// the game state, player info, and invitation code.
    pub(crate) fn create_game(
        local_player_display_name: &str,
        is_two_player: bool,
    ) -> Result<(LocalGameStateResource, LocalPlayerInfo, String), Error<CreateGameError>> {
        //

        let params = NewGameParams {
            game_mode: if is_two_player { GameMode::TwoPlayers } else { GameMode::SinglePlayer },
            player_one_display_name: local_player_display_name.to_string(),
            single_player_skill_level: Some(Some(SkillLevel::Beginner)),
        };

        let creation_result = match tic_tac_toe_api::create_game(&SDK_CONFIG, params) {
            Ok(game_info) => game_info,
            Err(error) => return Err(error),
        };

        let game_state: LocalGameStateResource = creation_result.game_info.clone().into();

        Ok((
            game_state,
            creation_result.game_info.players.first().unwrap().clone().into(),
            creation_result.game_invitation_code,
        ))
    }

    /// Returns Game state of a specified Game. NOTE: This info cached for immediate access if
    /// setup_auto_update() is called whenever a new Game is started or joined.
    pub(crate) fn get_game_info(game_id: &str) -> GameInfoResult {
        //

        // Try the mem cache
        let info_mutex = AUTO_UPDATE_INFO.lock().unwrap();
        let result = info_mutex.latest_results.clone();
        drop(info_mutex);

        match result {
            None => {
                // Cache missed. Call the server directly.
                Self::internal_get_game_info(game_id)
            }
            Some(result) => {
                // Cache hit, wooohooo!
                result
            }
        }
    }

    /// Returns Game state of a specified Game.
    fn internal_get_game_info(game_id: &str) -> GameInfoResult {
        match tic_tac_toe_api::get_game_info(&SDK_CONFIG, game_id) {
            Ok(game_info) => Ok((
                game_info.clone().into(),
                remote_players_to_local_players(&game_info.players),
            )),
            Err(error) => match error {
                Error::ResponseError(error) => match error.entity {
                    None => Err(GetGameInfoError::UnknownValue(Default::default())),
                    Some(error) => Err(error),
                },
                _ => Err(GetGameInfoError::UnknownValue(Default::default())),
            },
        }
    }

    /// Joins a Game via the Game Invitation Code.
    pub(crate) fn join_game(
        game_invitation_code: &str,
        player_display_name: &str,
    ) -> Result<(LocalGameStateResource, Vec<LocalPlayerInfo>), Error<AddPlayerError>> {
        //

        let params = AddPlayerParams {
            game_invitation_code: game_invitation_code.to_string(),
            player_display_name: player_display_name.to_string(),
        };

        let game_creation_result = match tic_tac_toe_api::add_player(&SDK_CONFIG, params) {
            Ok(game_info) => game_info,
            Err(error) => return Err(error),
        };

        Ok((
            game_creation_result.game_info.clone().into(),
            remote_players_to_local_players(&game_creation_result.game_info.players),
        ))
    }

    /// Sets Game ID to use for Game Info auto-updating. The interval parameter should be set to
    /// 1/2 the interval at which the system calls get_game_info(), i.e. twice as frequent.
    pub(crate) fn setup_auto_update(new_game_id: &str, interval: &Duration) {
        //

        let mut info_mutex = AUTO_UPDATE_INFO.lock().unwrap();

        info_mutex.game_id = new_game_id.to_string();
        info_mutex.interval = *interval;
        let is_already_running = info_mutex.is_running;
        drop(info_mutex);

        // If the thread is not already running then let's get it started.
        if !is_already_running {
            Self::start_auto_game_info_update();
        }
    }

    /// Begins the background thread that frequently retrieves and caches the Game Info.
    fn start_auto_game_info_update() {
        //

        let mut info_mutex = AUTO_UPDATE_INFO.lock().unwrap();

        // Exit if we are already running.
        if info_mutex.is_running {
            drop(info_mutex);
            return;
        }

        // Indicate that the thread is running
        info_mutex.is_running = true;

        drop(info_mutex);

        std::thread::spawn(|| {
            loop {
                // Get a local snapshot of the info to work from. We do this per loop so that
                // these settings can be changed dynamically. NOTE: We don't want to keep the mutex
                // locked for the duration of this call.
                let local_info_mutex = AUTO_UPDATE_INFO.lock().unwrap();
                let local_info = local_info_mutex.clone();
                drop(local_info_mutex);

                // Call the server for the latest state of the specified Game.
                let result = LocalServiceClient::internal_get_game_info(&local_info.game_id);

                // Cache the result. We lock the mutex for only as long as it takes to write to it.
                let mut latest_results = AUTO_UPDATE_INFO.lock().unwrap();
                latest_results.latest_results = Some(result);
                drop(latest_results);

                // Sleep for the interval.
                std::thread::sleep(local_info.interval);
            }
        });
    }

    /// Takes a turn on behalf of the local Player.
    pub(crate) fn take_turn(
        game_id: &str,
        grid_position: LocalGridPosition,
        local_player_id: &str,
    ) -> Result<(), Error<TakeTurnError>> {
        let params = GameTurnInfo {
            destination: BoardPosition {
                column: grid_position.column as i32,
                row: grid_position.row as i32,
            },
            player_id: local_player_id.to_string(),
        };
        tic_tac_toe_api::take_turn(&SDK_CONFIG, game_id, params)
    }
}

mod helpers {
    use crate::shared::local_models::local_game_state::LocalGameStateResource;
    use crate::shared::local_models::local_player_info::LocalPlayerInfo;
    use std::sync::{LazyLock, Mutex};
    use std::time::Duration;
    use tic_tac_toe_rust_client_sdk::apis::configuration::Configuration;
    use tic_tac_toe_rust_client_sdk::apis::tic_tac_toe_api::GetGameInfoError;

    pub(crate) static SDK_CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
        base_path: "http://127.0.0.1:50020".to_string(), // TODO: JD: set this to the address of the load balancer.
        user_agent: Some("Tic-Tac-Toe Rust Client".to_string()),
        ..Default::default()
    });

    pub(crate) static AUTO_UPDATE_INFO: LazyLock<Mutex<AutoUpdateInfo>> = LazyLock::new(|| { Mutex::new(AutoUpdateInfo::default()) });

    pub(crate) type GameInfoResult = Result<(LocalGameStateResource, Vec<LocalPlayerInfo>), GetGameInfoError>;

    /// Models the data need to perform and cache background updates for Game Info.
    #[derive(Clone, Default)]
    pub(crate) struct AutoUpdateInfo {
        pub(crate) game_id: String,
        pub(crate) interval: Duration,
        pub(crate) is_running: bool,
        pub(crate) latest_results: Option<GameInfoResult>,
    }
}