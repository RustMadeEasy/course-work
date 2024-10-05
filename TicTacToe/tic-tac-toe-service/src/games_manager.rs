// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::errors::GameError;
use crate::game_observer_trait::{GameObserverTrait, GameStateChange};
use crate::game_state::GameState;
use crate::game_trait::GameTrait;
use crate::game_updates_publisher::GameUpdatesPublisher;
use crate::models::requests::{AddPlayerParams, GameMode, GameTurnInfo, NewGameParams};
use crate::models::PlayerInfo;
use crate::tic_tac_toe_game::TicTacToeGame;
use chrono::Utc;
use log::{debug, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use verification_code_gen::verification_code_generator::VerificationCodeGenerator;

pub(crate) type TicTacToeGamesManager = GamesManager<TicTacToeGame>;

const ABANDONED_GAME_TTL_MS: i64 = (SECOND_IN_AN_HOUR * 1000) as i64;
const CLEANUP_INTERVAL: Duration = Duration::from_secs(SECOND_IN_AN_HOUR / 2);
const SECOND_IN_AN_HOUR: u64 = 60;

const MQTT_BROKER_ADDRESS: &str = "test.mosquitto.org";
const MQTT_PORT: u16 = 1883;

/// Manages all the Game instances played on this service.
///
/// NOTE: This is sample code.
///
/// NOTE: Production-grade code would persist the gaming info to a mem cache or database so that
/// multiple instances of the service can be run.
pub(crate) struct GamesManager<T: GameTrait + Clone + Send + Sync + 'static> {
    //

    /// The Games being managed by this instance. They are stored by Game ID.
    games: Arc<Mutex<HashMap<String, T>>>,

    observers: Vec<Box<dyn GameObserverTrait<T> + Send>>,
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamesManager<T> {
    //

    async fn notify_observers(&self, game_state_change: GameStateChange, game: &T) {
        for observer in self.observers.iter() {
            observer.game_updated(&game_state_change, &game).await;
        }
    }

    /// Adds a Player to the Game.
    pub(crate) async fn add_player(&mut self, second_player_params: &AddPlayerParams) -> Result<T, GameError> {
        //

        // Find the Game instance via the Game_invitation_code.
        let mut game = match self.get_game_by_invitation_code(&second_player_params.game_invitation_code) {
            None => {
                return Err(GameError::InvitationCodeNotFound);
            }
            Some(game) => game,
        };

        game.add_player(&second_player_params.player_display_name, false)?;

        // Update the Game instance in the list.
        self.games.lock().unwrap().insert(game.get_id(), game.clone());

        self.notify_observers(GameStateChange::PlayerAdded, &game).await;

        Ok(game)
    }

    /// Background task that regularly cleans up abandoned and completed Games.
    fn auto_cleanup(games: Arc<Mutex<HashMap<String, T>>>, ttl: i64, interval: Duration) {
        //

        // TODO: JD: test

        thread::spawn(move || {
            //

            loop {
                //

                debug!("Cleanup thread: Waking.");

                let mut non_expired_games: HashMap<String, T> = HashMap::new();
                let mut games = games.lock().unwrap();
                let mut games_expired: u64 = 0;

                // Remove any Game that is abandoned or has not been updated in a long time.
                let now = Utc::now().timestamp_millis();
                for game in games.values().clone() {
                    match game.get_time_of_latest_move() {
                        None => {}
                        Some(time_last_move) => {
                            let game_age = now - time_last_move.timestamp_millis();
                            if game_age < ttl {
                                // Keep this Game.
                                non_expired_games.insert(game.get_id(), game.clone());
                            } else {
                                if !game.get_current_game_state().has_ended() {
                                    // TODO: JD: properly end each abandoned game
                                }
                                games_expired += 1;
                            }
                        }
                    }
                }
                if games_expired > 0 {
                    *games = non_expired_games;
                }

                if games_expired > 0 {
                    debug!("Cleanup thread: Complete. Removed {} expired games. Going back to sleep.", games_expired);
                } else {
                    debug!("Cleanup thread: Complete. Going back to sleep.");
                }

                drop(games);

                // Sleep until the next cleanup.
                thread::sleep(interval);

                // TODO: JD: exit when service is shutting down
            }
        });
    }

    /// Creates a new Game instance.
    pub(crate) fn create_game(&mut self, params: &NewGameParams) -> Result<T, GameError> {
        //

        let invitation_code = if params.game_mode == GameMode::TwoPlayers {
            self.generate_invitation_code()
        } else {
            "".to_string()
        };

        let game = T::new(params, invitation_code, MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT)?;

        self.games.lock().unwrap().insert(game.get_id().clone(), game.clone());

        Ok(game.clone())
    }

    /// Closes down the specified Game instance.
    pub(crate) fn end_game(&mut self, game_id: &String) -> Result<(), GameError> {
        //

        let game = self.get_game_instance(game_id)?;

        self.games.lock().unwrap().remove(&game.get_id());

        Ok(())
    }

    /// Retrieves the specified Game instance.
    pub(crate) fn get_game_instance(&self, game_id: impl Into<String>) -> Result<T, GameError> {
        match self.games.lock().unwrap().get(&game_id.into()) {
            None => Err(GameError::GameNotFound),
            Some(game) => Ok(game.clone()),
        }
    }

    /// Retrieves the history of the Game States from the initial creation through to the current
    /// Game State. This can be used, for instance, the client could provide an animation that
    /// shows a time-lapse of the Game play.
    pub(crate) fn get_game_history(&self, game_id: &String) -> Result<Vec<GameState>, GameError> {
        let game = self.get_game_instance(game_id)?;
        Ok(game.get_play_history())
    }

    /// Creates a new GamesManager instance.
    pub(crate) fn new() -> Self {
        //

        let mut instance = Self {
            observers: vec![],
            games: Default::default(),
        };

        let publisher = GameUpdatesPublisher::new(MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT);

        instance.observers.push(Box::new(publisher));

        Self::auto_cleanup(instance.games.clone(), ABANDONED_GAME_TTL_MS, CLEANUP_INTERVAL);

        instance
    }

    /// Takes a turn for the specified Player.
    pub(crate) async fn take_turn(
        &mut self,
        game_id: &String,
        game_turn_info: &GameTurnInfo,
    ) -> Result<GameState, GameError> {
        //

        let mut game = self.get_game_instance(game_id)?;
        let new_game_state = game.take_turn(game_turn_info)?;

        // Update our Game instance.
        self.games.lock().unwrap().insert(game.get_id().clone(), game.clone());

        self.notify_observers(GameStateChange::TurnTaken, &game).await;

        Ok(new_game_state)
    }
}

// Invitation code handling
impl<T: GameTrait + Clone + Send + Sync + 'static> GamesManager<T> {
    //

    /// Retrieves a Game by its invitation code.
    fn get_game_by_invitation_code(&self, invitation_code: &String) -> Option<T> {
        self.games
            .lock()
            .unwrap()
            .iter()
            .find(|it| it.1.get_invitation_code() == *invitation_code)
            .map(|it| it.1.clone())
    }

    /// Creates a unique, 6-digit code for use as a Game Invitation.
    ///
    /// NOTE: We use a 6-digit Game Invitation instead of performing the Game setup handshaking
    /// with the Game ID for two reasons:
    ///     1) We don't want to expose the Game ID to clients that are not party to the Game.
    ///     2) A 6-digit code is practical for end-users to utilize.
    fn generate_invitation_code(&self) -> String {
        //

        // Place a limit to prevent an endless loop.
        for _ in 0..=1000 {
            let game_invitation_code: String = VerificationCodeGenerator::generate();
            // Ensure uniqueness across all open Games
            if self.get_game_by_invitation_code(&game_invitation_code).is_none() {
                return game_invitation_code;
            }
        }

        // It will be next to impossible to get here. However, we have to cover all cases.
        warn!("Could not create unique game invitation code!");
        "".to_string()
    }
}
