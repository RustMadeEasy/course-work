// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::auto_player::AutomaticPlayer;
use crate::errors::GameError;
use crate::game_board::GamePiece;
use crate::game_observer_trait::{GameObserverTrait, StateChanges};
use crate::game_session::GamingSession;
use crate::game_session_manager::GamingSessionsManager;
use crate::game_state::GameState;
use crate::game_trait::GameTrait;
use crate::game_updates_publisher::GameUpdatesPublisher;
use crate::models::requests::{GameMode, GameTurnInfo, JoinGameParams, JoinSessionParams, NewGameParams, NewGamingSessionParams};
use crate::models::responses::GamingSessionCreationResult;
use crate::models::PlayerInfo;
use crate::tic_tac_toe_game::TicTacToeGame;
use chrono::Utc;
use log::debug;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub(crate) type TicTacToeGamesManager = GamesManager<TicTacToeGame>;

const ABANDONED_SESSION_TTL_MS: i64 = (SECOND_IN_AN_HOUR * 1000) as i64;
const CLEANUP_INTERVAL: Duration = Duration::from_secs(SECOND_IN_AN_HOUR / 2);
const SECOND_IN_AN_HOUR: u64 = 60 * 60;

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

    game_sessions_manager: GamingSessionsManager<T>,

    observers: Vec<Box<dyn GameObserverTrait<T> + Send>>,
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamesManager<T> {
    //

    /// Adds a Player to the Game.
    pub(crate) async fn add_player_to_game(&mut self, params: &JoinGameParams) -> Result<(), GameError> {
        //

        debug!("GamesManager: add_player() called. Params: {:?}", params);

        let session_and_game = match self.game_sessions_manager.get_session_and_game_by_game_id(&params.game_id) {
            None => return Err(GameError::GameNotFound),
            Some(session) => session,
        };

        self.notify_observers_of_game_change(StateChanges::PlayerAddedToGame, &session_and_game.0, &session_and_game.1).await;

        Ok(())
    }

    /// Adds a Player to the Gaming Session.
    pub(crate) async fn add_player_to_session(&mut self, params: &JoinSessionParams) -> Result<GamingSessionCreationResult, GameError> {
        //

        debug!("GamesManager: add_player() called. Params: {:?}", params);

        let mut session = match self.game_sessions_manager.get_session(&params.game_invitation_code) {
            None => return Err(GameError::InvitationCodeNotFound),
            Some(session) => session,
        };

        let new_player = PlayerInfo::new(&params.player_display_name, &GamePiece::O, false);
        session.add_participant(&new_player);

        self.notify_observers_of_session_change(StateChanges::PlayerAddedToSession, &session).await;

        Ok(GamingSessionCreationResult {
            event_plane_config: session.event_plane_config,
            invitation_code: session.invitation_code,
            session_id: session.session_id,
        })
    }

    /// Background task that regularly cleans up abandoned Sessions.
    fn auto_cleanup(games: Arc<Mutex<HashMap<String, T>>>, ttl: i64, interval: Duration) {
        //

        debug!("GamesManager: auto_cleanup() started.");

        thread::spawn(move || {
            //

            loop {
                //

                debug!("GamesManager: Cleanup thread: Waking.");

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
                    debug!("GamesManager: Cleanup thread: Complete. Removed {} expired games. Going back to sleep.", games_expired);
                } else {
                    debug!("GamesManager: Cleanup thread: Complete. Going back to sleep.");
                }

                drop(games);

                // Sleep until the next cleanup.
                thread::sleep(interval);

                // TODO: JD: exit when service is shutting down
            }
        });
    }

    /// Creates a new Gaming Session and new Game.
    pub(crate) async fn create_new_game(&mut self, params: &NewGameParams) -> Result<T, GameError> {
        //

        debug!("GamesManager: create_new_game() called. Params: {:?}", params);

        let mut game = T::new(params)?;

        // Also, if this is human vs. computer, add the computer opponent now
        if params.game_mode == GameMode::SinglePlayer {
            game.add_player(AutomaticPlayer::<T>::get_name().as_str(), true)?;
            let second_player = game.get_players().last().unwrap().clone();
            let skill_level = params.single_player_skill_level.clone().unwrap_or_default();

            // Create an AutomaticPlayer to play against Play One.
            let auto_player = AutomaticPlayer::<T>::new(&game.get_id(), second_player, skill_level);

            // Make sure the AutomaticPlayer can follow the Game.
            self.observers.push(Box::new(auto_player));
        }

        self.game_sessions_manager.upsert_game(&game);

        Ok(game.clone())
    }

    /// Creates a new Gaming Session.
    pub(crate) async fn create_new_session(&mut self, params: &NewGamingSessionParams) -> Result<GamingSession<T>, GameError> {
        //

        debug!("GamesManager: create_new_session() called. Params: {:?}", params);

        let player_one = PlayerInfo::new(&params.session_owner_display_name, &GamePiece::X, false);
        let session = self.game_sessions_manager.create_session(player_one, MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT);

        Ok(session.clone())
    }

    /// Closes down the specified Game instance.
    pub(crate) fn end_game(&mut self, game_id: &String) -> Result<(), GameError> {
        //

        debug!("GamesManager: end_game() called for game: {:?}.", game_id);

        let game = self.get_game_by_id(game_id)?;

        if game.get_game_mode() == GameMode::SinglePlayer {
            self.remove_auto_player_observer(game_id);
        }

        self.game_sessions_manager.remove_game(game_id);

        Ok(())
    }

    /// Retrieves the history of the Game States from the initial creation through to the current
    /// Game State. This can be used, for instance, the client could provide an animation that
    /// shows a time-lapse of the Game play.
    pub(crate) fn get_game_history(&self, game_id: &String) -> Result<Vec<GameState>, GameError> {
        let game = self.get_game_by_id(game_id)?;
        Ok(game.get_play_history())
    }

    /// Retrieves the specified Session and Game pair.
    pub(crate) fn get_game_by_id(&self, game_id: impl Into<String>) -> Result<T, GameError> {
        match self.game_sessions_manager.get_game_by_id(&game_id.into()) {
            Some(game) => Ok(game),
            None => Err(GameError::GameNotFound),
        }
    }

    /// Retrieves the specified Session and Game pair.
    pub(crate) fn get_session_and_game_by_game_id(&self, game_id: impl Into<String>) -> Result<(GamingSession<T>, T), GameError> {
        match self.game_sessions_manager.get_session_and_game_by_game_id(&game_id.into()) {
            Some(session_and_game) => Ok(session_and_game),
            None => Err(GameError::GameNotFound),
        }
    }

    /// Creates a new GamesManager instance.
    pub(crate) fn new() -> Self {
        //

        debug!("GamesManager: new()");

        let mut instance = Self {
            game_sessions_manager: GamingSessionsManager::<T>::new(),
            observers: vec![],
        };

        let publisher = GameUpdatesPublisher::new(MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT);

        instance.observers.push(Box::new(publisher));

        // TODO: JD: finish
        //Self::auto_cleanup(instance.games.clone(), ABANDONED_SESSION_TTL_MS, CLEANUP_INTERVAL);

        instance
    }

    async fn notify_observers_of_game_change(&mut self, game_state_change: StateChanges, session: &GamingSession<T>, game: &T) {
        debug!("GamesManager: notifying observers of game stage change: {:?}.", game_state_change);
        for observer in self.observers.iter() {
            let _ = observer.game_updated(&game_state_change, session, game).await;
        }
    }

    async fn notify_observers_of_session_change(&mut self, state_change: StateChanges, game_session: &GamingSession<T>) {
        debug!("GamesManager: notifying observers of game stage change: {:?}.", state_change);
        for observer in self.observers.iter() {
            let _ = observer.session_updated(&state_change, game_session).await;
        }
    }

    fn remove_auto_player_observer(&mut self, game_id: &String) {
        debug!("GamesManager: remove_auto_player_observer() called for game: {:?}.", game_id);
        if let Some(index) = self.observers.iter().position(|it| it.unique_id().as_str() == game_id) {
            self.observers.remove(index);
        }
    }

    /// Takes a turn for the specified Player.
    pub(crate) async fn take_turn(
        &mut self,
        game_id: &String,
        game_turn_info: &GameTurnInfo,
    ) -> Result<GameState, GameError> {
        //

        debug!("GamesManager: take_turn() called for game: {:?}.", game_id);

        let mut session_and_game = self.get_session_and_game_by_game_id(game_id)?;
        let new_game_state = session_and_game.1.take_turn(game_turn_info)?;

        // Update our Game instance.
        self.game_sessions_manager.upsert_game(&session_and_game.1);

        self.notify_observers_of_game_change(StateChanges::GameTurnTaken, &session_and_game.0, &session_and_game.1).await;

        Ok(new_game_state)
    }
}

