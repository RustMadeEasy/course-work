// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::errors::GameError;
use crate::gaming::automatic_player::AutomaticPlayer;
use crate::gaming::game_observer_trait::{GamingSessionObserverTrait, GamingSessionStateChanges};
use crate::gaming::game_trait::GameTrait;
use crate::gaming::game_updates_publisher::GameUpdatesPublisher;
use crate::gaming::gaming_session::GamingSession;
use crate::gaming::tic_tac_toe_game::TicTacToeGame;
use crate::models::requests::GameTurnParams;
use crate::models::responses::{GamingSessionCreationResult, TurnResult};
use crate::models::GameState;
use crate::models::PlayStatus;
use crate::models::{AutomaticPlayerSkillLevel, GameMode, PlayerInfo};
use chrono::Utc;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub(crate) type TicTacToeGamesManager = GamingSessionsManager<TicTacToeGame>;

const ABANDONED_SESSION_TTL_MS: i64 = (SECOND_IN_AN_HOUR * 1000) as i64;
const CLEANUP_INTERVAL: Duration = Duration::from_secs(SECOND_IN_AN_HOUR / 2);
const SECOND_IN_AN_HOUR: u64 = 60 * 60;

const MQTT_BROKER_ADDRESS: &str = "test.mosquitto.org";
const MQTT_PORT: u16 = 1883;

/// Manages all the Game Sessions.
///
/// NOTE: This is sample code.
///
/// NOTE: Production-grade code would persist the gaming info to a mem cache or database so that
/// multiple instances of the service can be run.
pub(crate) struct GamingSessionsManager<T: GameTrait + Clone + Send + Sync + 'static> {
    //

    observers: Vec<Box<dyn GamingSessionObserverTrait<T> + Send>>,
    sessions: Arc<tokio::sync::Mutex<HashMap<String, Box<GamingSession<T>>>>>,
}

// Session Management
impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionsManager<T> {
    //

    /// Adds a Player to the Gaming Session.
    pub(crate) async fn add_player_to_session(&mut self,
                                              game_invitation_code: &str,
                                              player_display_name: &str) -> Result<GamingSessionCreationResult, GameError> {
        //

        debug!("GamesManager: add_player_to_session() called.");

        let mut session = match self.get_session_by_invitation_code(game_invitation_code).await {
            None => return Err(GameError::InvitationCodeNotFound),
            Some(session) => session,
        };

        let new_player = PlayerInfo::new(player_display_name, false);
        session.add_participant(&new_player);
        self.upsert_session(&session).await;

        let other_player = PlayerInfo::get_other_player_info_by_id(session.session_owner.player_id.clone(), &session.participants)?;

        Ok(GamingSessionCreationResult {
            event_plane_config: session.event_plane_config,
            initiating_player: session.session_owner,
            invitation_code: session.invitation_code,
            other_player: Some(other_player),
            session_id: session.session_id,
        })
    }

    /// Creates a new Gaming Session.
    pub(crate) async fn create_new_session(&mut self, session_owner_display_name: &str) -> Result<GamingSession<T>, GameError> {
        //

        debug!("GamesManager: create_new_session() called.");

        let player_one = PlayerInfo::new(session_owner_display_name, false);
        let session = GamingSession::new(player_one, MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT);
        self.upsert_session(&session).await;

        Ok(session.clone())
    }

    /// Called to indicate that a Player is ready to Play. This is required as part of the handshaking 
    /// during new Game setup.
    pub(crate) async fn note_player_readiness(&self, session_id: &str, _player_id: &str) -> Result<(), GameError> {
        //

        debug!("GamesManager: note_player_readiness() called.");

        let session = match self.get_session_by_session_id(session_id).await {
            None => return Err(GameError::SessionNotFound),
            Some(session) => session,
        };

        self.notify_observers_of_session_change(GamingSessionStateChanges::PlayerReady, &session).await;

        Ok(())
    }

    /// Retrieves the Gaming Session by Invitation Code.
    async fn get_session_by_invitation_code(&self, invitation_code: &str) -> Option<Box<GamingSession<T>>> {
        self.sessions.lock().await.get(invitation_code).cloned()
    }

    /// Retrieves the Gaming Session by ID.
    pub(crate) async fn get_session_by_session_id(&self, session_id: &str) -> Option<Box<GamingSession<T>>> {
        self.sessions.lock().await.iter().find(|it| it.1.session_id.as_str() == session_id).map(|it| it.1.clone())
    }

    async fn get_session_containing_game(&self, game_id: &str) -> Option<Box<GamingSession<T>>> {
        for session in self.sessions.lock().await.iter() {
            if let Some(game) = session.1.current_game.clone() {
                if game.get_id().as_str() == game_id {
                    return Some(session.1.clone());
                }
            }
        }
        None
    }

    /// Removes a new Gaming Session.
    async fn _remove_session(&mut self, invitation_code: &str) {
        self.sessions.lock().await.remove(invitation_code);
    }

    /// Upserts a Gaming Session.
    async fn upsert_session(&mut self, session: &GamingSession<T>) {
        self.sessions.lock().await.insert(session.invitation_code.clone(), Box::new(session.clone()));
    }
}

// Game Management
impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionsManager<T> {

    //

    /// Creates a new Single-Player Game.
    pub(crate) async fn create_new_single_player_game(&mut self,
                                                      session_id: &str,
                                                      computer_skill_level: &AutomaticPlayerSkillLevel) -> Result<T, GameError> {
        //

        debug!("GamesManager: create_new_single_player_game() called. Session ID: {:?}, Skill Level: {:?}", session_id, computer_skill_level);

        let mut session = match self.get_session_by_session_id(session_id).await {
            None => return Err(GameError::SessionNotFound),
            Some(session) => session,
        };

        let human_player = match session.participants.first() {
            None => return Err(GameError::InvalidSession),
            Some(player) => player,
        };

        let computer_player = PlayerInfo::new(AutomaticPlayer::<T>::get_name().as_str(), true);

        let game = T::new(GameMode::SinglePlayer, human_player, &computer_player, &session.session_id)?;

        // Create an AutomaticPlayer to play against Player One.
        let auto_player = AutomaticPlayer::<T>::new(&game.get_id(), &computer_player, computer_skill_level);

        // Make sure the AutomaticPlayer can follow the Game.
        self.observers.push(Box::new(auto_player));

        self.upsert_game(&game).await;

        session.participants.push(computer_player.clone());
        session.current_game = Some(game.clone());
        self.upsert_session(&session).await;

        self.notify_observers_of_game_change(GamingSessionStateChanges::GameStarted, &session, &game).await;

        Ok(game.clone())
    }

    /// Creates a new Two-Player Game. Returns the new Game as well as a list of Players.
    pub(crate) async fn create_new_two_player_game(&mut self, session_id: &str) -> Result<(T, Vec<PlayerInfo>), GameError> {
        //

        debug!("GamesManager: create_new_two_player_game() called for Session ID: {}", session_id);

        let mut session = match self.get_session_by_session_id(session_id).await {
            Some(session) => *session,
            None => {
                return Err(GameError::SessionNotFound);
            }
        };

        if session.participants.len() < 2 {
            return Err(GameError::SessionHasTooFewPlayers);
        }

        let game = T::new(GameMode::TwoPlayers,
                          session.participants.first().unwrap(),
                          session.participants.last().unwrap(),
                          &session.session_id)?;

        self.upsert_game(&game).await;

        session.current_game = Some(game.clone());
        self.upsert_session(&session).await;

        self.notify_observers_of_game_change(GamingSessionStateChanges::GameStarted, &session, &game).await;

        Ok((game.clone(), session.participants))
    }

    async fn remove_game(&mut self, game_id: &str) -> bool {
        match self.get_session_containing_game(game_id).await {
            None => false,
            Some(mut session) => {
                session.clear_game();
                self.upsert_session(&session).await;
                self.notify_observers_of_session_change(GamingSessionStateChanges::GameDeleted, &session).await;
                true
            }
        }
    }

    /// Closes down the specified Game instance.
    pub(crate) async fn end_game(&mut self, game_id: &str, _player_id: &str, session_id: &str) -> Result<(), GameError> {
        //

        debug!("GamesManager: end_game() called for game: {:?}.", game_id);

        let session = match self.get_session_by_session_id(session_id).await {
            Some(session) => session,
            None => return Err(GameError::SessionNotFound),
        };

        // Only allow Players who are part of the Game's Gaming Session to end the Game.
        if !session.participants.iter().any(|it| it.player_id == _player_id) {
            return Err(GameError::PlayerNotFound);
        }

        let game = self.get_game_by_id(game_id).await?;

        if game.get_game_mode() == GameMode::SinglePlayer {
            self.remove_auto_player_observer(game_id);
        }

        self.remove_game(game_id).await;

        Ok(())
    }

    /// Closes down the specified Game instance.
    pub(crate) async fn end_gaming_session(&mut self, _player_id: &str, session_id: &str) -> Result<(), GameError> {
        //

        debug!("GamesManager: end_gaming_session() called for session: {:?}.", session_id);

        match self.get_session_by_session_id(session_id).await {
            None => Err(GameError::SessionNotFound),
            Some(session) => {
                //

                // Only allow Players who are part of the session to end the session.
                if !session.participants.iter().any(|it| it.player_id == _player_id) {
                    return Err(GameError::PlayerNotFound);
                }

                // Close down the session's game, if any
                if let Some(ref game) = session.current_game {
                    let _ = self.end_game(&game.get_id(), session.session_owner.player_id.as_str(), session_id).await;
                }

                // Remove the Session
                self.sessions.lock().await.remove(session_id);

                self.notify_observers_of_session_change(GamingSessionStateChanges::SessionDeleted, &session).await;

                Ok(())
            }
        }
    }

    /// Retrieves the Game being played in the specified Gaming Session.
    pub(crate) async fn get_game_in_session(&self, session_id: impl Into<String>) -> Result<(GamingSession<T>, T), GameError> {
        match self.get_session_by_session_id(&session_id.into()).await {
            None => Err(GameError::SessionNotFound),
            Some(session) => {
                let session = (*session).clone();
                match session.current_game {
                    None => {
                        Err(GameError::GameNotFound)
                    }
                    Some(ref game) => {
                        Ok((session.clone(), game.clone()))
                    }
                }
            }
        }
    }

    /// Retrieves the specified Session and Game pair.
    pub(crate) async fn get_game_by_id(&self, game_id: impl Into<String>) -> Result<T, GameError> {
        match self.get_session_containing_game(&game_id.into()).await {
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
    pub(crate) async fn get_game_history(&self, game_id: &str) -> Result<Vec<GameState>, GameError> {
        let game = self.get_game_by_id(game_id).await?;
        Ok(game.get_play_history())
    }

    async fn upsert_game(&mut self, game: &T) -> bool {
        match self.get_session_containing_game(&game.get_id()).await {
            Some(mut session) => {
                session.set_game(game);
                self.upsert_session(&session).await;
                true
            }
            None => false,
        }
    }
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionsManager<T> {
    //

    /// Background task that regularly cleans up abandoned Sessions.
    fn auto_cleanup(sessions: Arc<tokio::sync::Mutex<HashMap<String, Box<GamingSession<T>>>>>, ttl: i64, interval: Duration) {
        //

        debug!("GamesManager: auto_cleanup() started.");

        tokio::spawn(async move {
            //

            loop {
                //

                debug!("GamesManager: Cleanup thread: Waking.");

                let mut expired_sessions: Vec<GamingSession<T>> = vec!();
                let mut gaming_sessions = sessions.lock().await;

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
                    debug!("GamesManager: Cleanup thread: Complete. Removed {} expired games. Going back to sleep.", expired_sessions.len());
                } else {
                    debug!("GamesManager: Cleanup thread: Complete. Going back to sleep.");
                }

                drop(gaming_sessions);

                // Sleep until the next cleanup.
                tokio::time::sleep(interval).await;

                // TODO: JD: exit when service is shutting down
            }
        });
    }

    /// Creates a new GamesManager instance.
    pub(crate) fn new() -> Self {
        //

        debug!("GamesManager: new()");

        let mut instance = Self {
            sessions: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            observers: vec![],
        };

        let publisher = Box::new(GameUpdatesPublisher::new(MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT));
        instance.observers.push(publisher.clone());

        Self::auto_cleanup(instance.sessions.clone(), ABANDONED_SESSION_TTL_MS, CLEANUP_INTERVAL);

        instance
    }

    /// Takes a turn for the specified Player.
    pub(crate) async fn take_turn(
        &mut self,
        game_id: &str,
        game_turn_info: &GameTurnParams,
    ) -> Result<TurnResult, GameError> {
        //

        debug!("GamesManager: take_turn() called for game: {:?}.", game_id);

        let session = match self.get_session_by_session_id(game_turn_info.session_id.as_str()).await {
            Some(session) => session,
            None => { return Err(GameError::SessionNotFound); }
        };

        if let Some(ref game) = session.current_game {
            //

            if game.get_id() != game_id {
                return Err(GameError::GameNotFound);
            }

            match game.get_current_game_state().play_status {
                PlayStatus::EndedInStalemate | PlayStatus::EndedInWin => {
                    return Err(GameError::GameHasAlreadyEnded);
                }
                PlayStatus::InProgress => {}
                PlayStatus::NotStarted => {
                    return Err(GameError::GameNotStarted);
                }
            }

            let mut updated_game = game.clone();

            let turn_result = updated_game.take_turn(game_turn_info)?;

            // Update our Game instance.
            self.upsert_game(&updated_game).await;

            self.notify_observers_of_game_change(GamingSessionStateChanges::GameTurnTaken, &session, &updated_game).await;

            Ok(turn_result)
        } else {
            Err(GameError::GameNotFound)
        }
    }
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionsManager<T> {
    //

    async fn notify_observers_of_game_change(&self, game_state_change: GamingSessionStateChanges, session: &GamingSession<T>, game: &T) {
        debug!("GamesManager: notifying observers of game stage change: {:?}.", game_state_change);
        for observer in self.observers.iter() {
            let _ = observer.session_updated(&game_state_change, session, Some(game)).await;
        }
    }

    async fn notify_observers_of_session_change(&self, state_change: GamingSessionStateChanges, game_session: &GamingSession<T>) {
        debug!("GamesManager: notifying observers of Gaming Session change: {:?}.", state_change);
        for observer in self.observers.iter() {
            let _ = observer.session_updated(&state_change, game_session, None).await;
        }
    }

    fn remove_auto_player_observer(&mut self, game_id: &str) {
        debug!("GamesManager: remove_auto_player_observer() called for game: {:?}.", game_id);
        if let Some(index) = self.observers.iter().position(|it| it.unique_id().as_str() == game_id) {
            self.observers.remove(index);
        }
    }
}
