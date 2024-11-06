//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::log::error;
use function_name::named;
use std::sync::{LazyLock, Mutex};
use std::time::Duration;
use tic_tac_toe_rust_client_sdk::apis::configuration::Configuration;
use tic_tac_toe_rust_client_sdk::apis::tic_tac_toe_api::GetLatestGameTurnError;
use tic_tac_toe_rust_client_sdk::apis::{tic_tac_toe_api, Error};
use tic_tac_toe_rust_client_sdk::models::{GamePiece, TurnResponse};

/// An auto-refreshing Game State info cache.
pub(crate) struct GameStateCache;

impl GameStateCache {
    //

    /// Returns Game state of a specified Game. NOTE: This info is cached for immediate access if
    /// setup_auto_update() is called whenever a new Game is started or joined.
    pub(crate) fn get_latest_game_turn(game_id: &str) -> Result<TurnResponse, Error<GetLatestGameTurnError>> {

        // Try the mem cache
        let info_mutex = AUTO_UPDATE_INFO.lock().unwrap();
        let result = info_mutex.latest_results.clone();
        drop(info_mutex);

        match result {
            None => {
                match Self::load_and_cache_latest_game_turn(game_id) {
                    Ok(response) => Ok(response),
                    Err(error) => Err(error),
                }
            }
            Some(result) => {
                // Cache hit, wooohooo!
                Ok(result)
            }
        }
    }

    fn load_and_cache_latest_game_turn(game_id: &str) -> Result<TurnResponse, Error<GetLatestGameTurnError>> {
        match tic_tac_toe_api::get_latest_game_turn(&SDK_CONFIG, game_id) {
            Ok(response) => {
                // Cache the result. We lock the mutex for only as long as it takes to write to it.
                let mut info_mutex = AUTO_UPDATE_INFO.lock().unwrap();
                info_mutex.latest_results = Some(response.clone());
                drop(info_mutex);
                Ok(response)
            }
            Err(error) => Err(error),
        }
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
    #[named]
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
                match GameStateCache::load_and_cache_latest_game_turn(&local_info.game_id) {
                    Ok(_) => {}
                    Err(error) => {
                        error!("{} - Error encountered: {:?}", function_name!(), error);
                    }
                }

                // Sleep for the interval.
                std::thread::sleep(local_info.interval);
            }
        });
    }
}

pub(crate) static SDK_CONFIG: LazyLock<Configuration> = LazyLock::new(|| Configuration {
    base_path: "http://127.0.0.1:50020".to_string(), // TODO: JD: set this to the address of the load balancer.
    user_agent: Some("Tic-Tac-Toe Rust Client".to_string()),
    ..Default::default()
});

pub(crate) static AUTO_UPDATE_INFO: LazyLock<Mutex<AutoUpdateInfo>> = LazyLock::new(|| { Mutex::new(AutoUpdateInfo::default()) });

/// Models the data need to perform and cache background updates for Game Info.
#[derive(Clone, Default)]
pub(crate) struct AutoUpdateInfo {
    pub(crate) game_id: String,
    pub(crate) interval: Duration,
    pub(crate) is_running: bool,
    pub(crate) latest_results: Option<TurnResponse>,
}

pub(crate) struct GamePieceHelper;
impl GamePieceHelper {
    pub(crate) fn display_name(game_piece: GamePiece) -> String {
        match game_piece {
            GamePiece::Unselected => "".to_string(),
            GamePiece::X => "(X)".to_string(),
            GamePiece::O => "(O)".to_string(),
        }
    }
}
