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

    observers: Vec<Box<dyn GameObserverTrait<T> + Send>>,

    sessions: Arc<Mutex<HashMap<String, Box<GamingSession<T>>>>>,
}

// Session Management
impl<T: GameTrait + Clone + Send + Sync + 'static> GamesManager<T> {
    //

    /// Adds a Player to the Gaming Session.
    pub(crate) async fn add_player_to_session(&mut self, params: &JoinSessionParams) -> Result<GamingSessionCreationResult, GameError> {
        //

        debug!("GamesManager: add_player() called. Params: {:?}", params);

        let mut session = match self.get_session(&params.game_invitation_code) {
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

    /// Creates a new Gaming Session.
    pub(crate) async fn create_new_session(&mut self, params: &NewGamingSessionParams) -> Result<GamingSession<T>, GameError> {
        //

        debug!("GamesManager: create_new_session() called. Params: {:?}", params);

        let player_one = PlayerInfo::new(&params.session_owner_display_name, &GamePiece::X, false);
        let session = GamingSession::new(player_one, MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT);
        self.upsert_session(&session);

        Ok(session.clone())
    }

    fn get_session(&self, invitation_code: &String) -> Option<Box<GamingSession<T>>> {
        self.sessions.lock().unwrap().get(invitation_code).cloned()
    }

    fn get_session_containing_game(&self, game_id: &String) -> Option<Box<GamingSession<T>>> {
        for session in self.sessions.lock().unwrap().iter() {
            if let Some(game) = session.1.current_game.clone() {
                if game.get_id().as_str() == game_id {
                    return Some(session.1.clone());
                }
            }
        }
        None
    }

    /// Removes a new Gaming Session.
    fn _remove_session(&mut self, invitation_code: &String) {
        self.sessions.lock().unwrap().remove(invitation_code);
    }

    /// Upserts a Gaming Session.
    fn upsert_session(&mut self, session: &GamingSession<T>) {
        self.sessions.lock().unwrap().insert(session.invitation_code.clone(), Box::new(session.clone()));
    }
}

// Game Management
impl<T: GameTrait + Clone + Send + Sync + 'static> GamesManager<T> {

    //

    /// Adds a Player to the Game.
    pub(crate) async fn add_player_to_game(&mut self, params: &JoinGameParams) -> Result<(), GameError> {
        //

        debug!("GamesManager: add_player() called. Params: {:?}", params);

        match self.get_session_and_game_by_game_id(&params.game_id) {
            Ok(session_and_game) => {
                self.notify_observers_of_game_change(StateChanges::PlayerAddedToGame, &session_and_game.0, &session_and_game.1).await;
                Ok(())
            }
            Err(error) => {
                Err(error)
            }
        }
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

        self.upsert_game(&game);

        Ok(game.clone())
    }

    /// Closes down the specified Game instance.
    pub(crate) fn end_game(&mut self, game_id: &String) -> Result<(), GameError> {
        //

        debug!("GamesManager: end_game() called for game: {:?}.", game_id);

        let game = self.get_game_by_id(game_id)?;

        if game.get_game_mode() == GameMode::SinglePlayer {
            self.remove_auto_player_observer(game_id);
        }

        self.remove_game(game_id);

        Ok(())
    }

    /// Retrieves the specified Session and Game pair.
    pub(crate) fn get_game_by_id(&self, game_id: impl Into<String>) -> Result<T, GameError> {
        match self.get_session_containing_game(&game_id.into()) {
            None => Err(GameError::GameNotFound),
            Some(session) => {
                match session.current_game {
                    None => Err(GameError::GameNotFound),
                    Some(game) => { Ok(game.clone()) }
                }
            }
        }
    }

    /// Retrieves the history of the Game States from the initial creation through to the current
    /// Game State. This can be used, for instance, the client could provide an animation that
    /// shows a time-lapse of the Game play.
    pub(crate) fn get_game_history(&self, game_id: &String) -> Result<Vec<GameState>, GameError> {
        let game = self.get_game_by_id(game_id)?;
        Ok(game.get_play_history())
    }

    /// Retrieves the specified Session and Game pair.
    pub(crate) fn get_session_and_game_by_game_id(&self, game_id: impl Into<String>) -> Result<(GamingSession<T>, T), GameError> {
        let session = match self.get_session_containing_game(&game_id.into()) {
            None => None,
            Some(session) => {
                session.current_game.as_ref().map(|game| ((*session).clone(), game.clone()))
            }
        };

        match session {
            Some(session_and_game) => Ok(session_and_game),
            None => Err(GameError::GameNotFound),
        }
    }

    fn remove_game(&mut self, game_id: &String) -> bool {
        match self.get_session_containing_game(game_id) {
            None => false,
            Some(mut session) => {
                session.clear_game();
                self.upsert_session(&session);
                true
            }
        }
    }

    fn upsert_game(&self, game: &T) -> bool {
        match self.get_session_containing_game(&game.get_id()) {
            Some(mut session) => {
                session.set_game(game);
                true
            }
            None => false,
        }
    }
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamesManager<T> {
    //

    /// Background task that regularly cleans up abandoned Sessions.
    fn auto_cleanup(sessions: Arc<Mutex<HashMap<String, Box<GamingSession<T>>>>>, ttl: i64, interval: Duration) {
        //

        debug!("GamesManager: auto_cleanup() started.");

        thread::spawn(move || {
            //

            loop {
                //

                debug!("GamesManager: Cleanup thread: Waking.");

                let mut expired_sessions: Vec<GamingSession<T>> = vec!();
                let mut gaming_sessions = sessions.lock().unwrap();

                // Remove any Game that is abandoned or has not been updated in a long time.
                let now = Utc::now().timestamp_millis();
                for session in gaming_sessions.values().clone() {
                    if let Some(game) = session.current_game.clone() {
                    match game.get_time_of_latest_move() {
                            None => {}
                            Some(time_last_move) => {
                                let game_age = now - time_last_move.timestamp_millis();
                                if game_age < ttl {
                                    // Keep this Game.
                                    // expired_sessions.insert(game.get_id(), game.clone());
                                } else {
                                    if !game.get_current_game_state().has_ended() {
                                        // TODO: JD: properly end each abandoned game
                                    }
                                    expired_sessions.push(*session.clone());
                                }
                            }
                        }
                    }
                }
                
                for session in expired_sessions.iter_mut() {
                    session.current_game = None;
                    gaming_sessions.insert(session.invitation_code.clone(), Box::new(session.clone()));
                }

                if !expired_sessions.is_empty() {
                    debug!("GamesManager: Cleanup thread: Complete. Removed {} expired games. Going back to sleep.", expired_sessions.iter().count());
                } else {
                    debug!("GamesManager: Cleanup thread: Complete. Going back to sleep.");
                }

                drop(gaming_sessions);

                // Sleep until the next cleanup.
                thread::sleep(interval);

                // TODO: JD: exit when service is shutting down
            }
        });
    }

    /// Creates a new GamesManager instance.
    pub(crate) fn new() -> Self {
        //

        debug!("GamesManager: new()");

        let mut instance = Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            observers: vec![],
        };

        let publisher = GameUpdatesPublisher::new(MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT);

        instance.observers.push(Box::new(publisher));

        Self::auto_cleanup(instance.sessions.clone(), ABANDONED_SESSION_TTL_MS, CLEANUP_INTERVAL);

        instance
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
        self.upsert_game(&session_and_game.1);

        self.notify_observers_of_game_change(StateChanges::GameTurnTaken, &session_and_game.0, &session_and_game.1).await;

        Ok(new_game_state)
    }
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamesManager<T> {
    //

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
}
