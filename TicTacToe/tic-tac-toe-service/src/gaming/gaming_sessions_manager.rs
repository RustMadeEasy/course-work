// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::errors::GameError;
use crate::gaming::automatic_player::AutomaticPlayer;
use crate::gaming::game_trait::GameTrait;
use crate::gaming::game_updates_publisher::GameUpdatesPublisher;
use crate::gaming::gaming_session::GamingSession;
use crate::gaming::gaming_session_observer_trait::GamingSessionObserverTrait;
use crate::gaming::gaming_session_state_changes::GamingSessionStateChanges;
use crate::models::automatic_player_skill_level::AutomaticPlayerSkillLevel;
use crate::models::game_mode::GameMode;
use crate::models::game_state::GameState;
use crate::models::play_status::PlayStatus;
use crate::models::player_info::PlayerInfo;
use crate::models::requests::GameTurnParams;
use crate::models::responses::{GamingSessionCreationResponse, TurnResponse};
use chrono::Utc;
use function_name::named;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// The length of time after which to consider an inactive Game as abandoned
const ABANDONED_GAME_TTL_MS: i64 = (SECONDS_IN_AN_HOUR * 1000) as i64;
/// The interval on which to run the background Game cleanup task
const CLEANUP_INTERVAL: Duration = Duration::from_secs(SECONDS_IN_AN_HOUR / 2);
/// The number of seconds in an hour
const SECONDS_IN_AN_HOUR: u64 = 60 * 60;

/// The MQTT broker address
const MQTT_BROKER_ADDRESS: &str = "test.mosquitto.org";
/// The MQTT port 
const MQTT_PORT: u16 = 1883;

/// Manages all the Game Sessions.
///
/// NOTE: This is sample code.
///
/// NOTE: Production-grade code would persist the gaming info to a mem cache or database so that
/// multiple instances of the service can be run.
pub(crate) struct GamingSessionsManager<T: GameTrait + Clone + Send + Sync + 'static> {
    observers: Vec<Box<dyn GamingSessionObserverTrait<T> + Send>>,
    sessions: Arc<tokio::sync::Mutex<HashMap<String, Box<GamingSession<T>>>>>,
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionsManager<T> {
    //

    /// Creates a new GamesManager instance.
    #[named]
    pub(crate) fn new() -> Self {
        //

        debug!("{} called", function_name!());

        let mut instance = Self {
            sessions: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
            observers: vec![],
        };

        let publisher = Box::new(GameUpdatesPublisher::new(MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT));
        instance.observers.push(publisher.clone());

        Self::auto_cleanup(instance.sessions.clone(), ABANDONED_GAME_TTL_MS, CLEANUP_INTERVAL);

        instance
    }
}

// Gaming Session Management
impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionsManager<T> {
    //

    /// Adds a Player to the Gaming Session.
    #[named]
    pub(crate) async fn join_session(&mut self,
                                     game_invitation_code: &str,
                                     player_display_name: &str) -> Result<GamingSessionCreationResponse, GameError> {
        //

        debug!("{} called", function_name!());

        let mut session = match self.get_session_by_invitation_code(game_invitation_code).await {
            None => return Err(GameError::InvitationCodeNotFound),
            Some(session) => session,
        };

        // Make the new Player if not already part of the Gaming Session.
        let other_player = match session.participants.iter().find(|p| p.display_name.to_lowercase() == player_display_name.to_lowercase()) {
            None => {
                // Create and add the new Player.
                let other_player = PlayerInfo::new(player_display_name, false);
                session.add_participant(&other_player);
                self.upsert_session(&session).await;
                other_player
            }
            Some(other_player) => other_player.clone(),
        };

        Ok(GamingSessionCreationResponse {
            event_plane_config: session.event_plane_config,
            initiating_player: session.session_owner,
            invitation_code: session.invitation_code,
            other_player: Some(other_player),
            session_id: session.session_id,
        })
    }

    /// Background task that regularly cleans up abandoned Sessions.
    #[named]
    fn auto_cleanup(sessions: Arc<tokio::sync::Mutex<HashMap<String, Box<GamingSession<T>>>>>, ttl: i64, interval: Duration) {
        //

        debug!("{} started", function_name!());

        tokio::spawn(async move {
            //

            loop {
                //

                debug!("{} - waking for cleanup", function_name!());

                let mut expired_sessions: Vec<GamingSession<T>> = vec!();
                let mut gaming_sessions = sessions.lock().await;

                // Note any Game that is abandoned or has ended.
                let now = Utc::now().timestamp_millis();
                for session in gaming_sessions.values().clone() {
                    if let Some(game) = session.current_game.clone() {
                        match game.get_time_of_latest_move() {
                            None => {}
                            Some(time_last_move) => {
                                let game_age = now - time_last_move.timestamp_millis();
                                if game_age > ttl || game.get_current_game_state().has_ended() {
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
                    debug!("{} - Cleanup complete. Removed {} expired games. Going back to sleep.", function_name!(), expired_sessions.len());
                } else {
                    debug!("{} - Cleanup complete. Going back to sleep.", function_name!());
                }

                drop(gaming_sessions);

                // Sleep until the next cleanup.
                tokio::time::sleep(interval).await;

                // TODO: JD: exit when service is shutting down
            }
        });
    }

    /// Creates a new Gaming Session.
    #[named]
    pub(crate) async fn create_new_session(&mut self, session_owner_display_name: &str) -> Result<GamingSession<T>, GameError> {
        //

        debug!("{} called", function_name!());

        let player_one = PlayerInfo::new(session_owner_display_name, false);
        let session = GamingSession::new(player_one, MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT);
        self.upsert_session(&session).await;

        Ok(session.clone())
    }

    /// Retrieves the Gaming Session by Invitation Code.
    async fn get_session_by_invitation_code(&self, invitation_code: &str) -> Option<Box<GamingSession<T>>> {
        self.sessions.lock().await.get(invitation_code).cloned()
    }

    /// Retrieves the Gaming Session by ID.
    pub(crate) async fn get_session_by_id(&self, session_id: &str) -> Option<Box<GamingSession<T>>> {
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
    #[named]
    pub(crate) async fn create_new_single_player_game(&mut self,
                                                      session_id: &str,
                                                      computer_skill_level: &AutomaticPlayerSkillLevel) -> Result<T, GameError> {
        //

        debug!("{} - Session ID: {:?}, Skill Level: {:?}", function_name!(), session_id, computer_skill_level);

        let mut session = match self.get_session_by_id(session_id).await {
            None => return Err(GameError::GamingSessionNotFound),
            Some(session) => session,
        };

        let computer_player = PlayerInfo::new(AutomaticPlayer::<T>::get_name().as_str(), true);

        let mut game = T::new(GameMode::SinglePlayer, &session.session_id)?;

        // Create an AutomaticPlayer to play against Player One.
        let auto_player = AutomaticPlayer::<T>::new(&game.get_id(), &computer_player, computer_skill_level);

        // Make sure the AutomaticPlayer can follow the Game.
        self.observers.push(Box::new(auto_player));

        game.add_player(&computer_player)?;

        self.upsert_game(&game).await;

        session.participants.push(computer_player.clone());
        session.current_game = Some(game.clone());
        self.upsert_session(&session).await;

        Ok(game.clone())
    }

    /// Creates a new Two-Player Game. Returns the new Game as well as a list of Players.
    #[named]
    pub(crate) async fn create_new_two_player_game(&mut self, session_id: &str) -> Result<(T, Vec<PlayerInfo>), GameError> {
        //

        debug!("{} called for Session ID: {}.", function_name!(), session_id);

        let mut session = match self.get_session_by_id(session_id).await {
            Some(session) => *session,
            None => {
                return Err(GameError::GamingSessionNotFound);
            }
        };

        let game = T::new(GameMode::TwoPlayers, &session.session_id)?;

        self.upsert_game(&game).await;

        session.current_game = Some(game.clone());
        self.upsert_session(&session).await;

        Ok((game.clone(), session.participants))
    }

    /// Closes down the specified Game instance.
    #[named]
    pub(crate) async fn end_game(&mut self, game_id: &str, _player_id: &str, session_id: &str) -> Result<(), GameError> {
        //

        debug!("{} called for game: {:?}.", function_name!(), game_id);

        let session = match self.get_session_by_id(session_id).await {
            Some(session) => session,
            None => return Err(GameError::GamingSessionNotFound),
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
    #[named]
    pub(crate) async fn end_gaming_session(&mut self, _player_id: &str, session_id: &str) -> Result<(), GameError> {
        //

        debug!("{} called for session: {:?}.", function_name!(), session_id);

        match self.get_session_by_id(session_id).await {
            None => Err(GameError::GamingSessionNotFound),
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

                self.notify_observers_of_session_change(GamingSessionStateChanges::GamingSessionDeleted, &session).await;

                Ok(())
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

    /// Retrieves the Game being played in the specified Gaming Session.
    pub(crate) async fn get_game_in_session(&self, session_id: impl Into<String>) -> Result<(GamingSession<T>, T), GameError> {
        match self.get_session_by_id(&session_id.into()).await {
            None => Err(GameError::GamingSessionNotFound),
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

    #[named]
    pub(crate) async fn join_current_game(&mut self, session_id: &str, player_id: &str) -> Result<(T, Vec<PlayerInfo>), GameError> {
        //

        debug!("{} called", function_name!());

        let session = match self.get_session_by_id(session_id).await {
            None => return Err(GameError::GamingSessionNotFound),
            Some(session) => session,
        };

        // The Player must have already joined the Gaming Session
        let player = match session.participants.iter().find(|it| it.player_id == player_id) {
            None => return Err(GameError::PlayerNotFound),
            Some(player) => player.clone(),
        };

        let mut game = match session.current_game {
            None => return Err(GameError::GameNotFound),
            Some(ref game) => game.clone(),
        };

        game.add_player(&player)?;

        self.upsert_game(&game).await;

        if game.get_player_count() == 2 {
            self.notify_observers_of_session_change(GamingSessionStateChanges::AllPlayersReady, &session).await;
        }

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

    /// Takes a turn for the specified Player.
    #[named]
    pub(crate) async fn take_turn(
        &mut self,
        game_id: &str,
        game_turn_info: &GameTurnParams,
    ) -> Result<TurnResponse, GameError> {
        //

        debug!("{} called for game: {:?}.", function_name!(), game_id);

        let session = match self.get_session_by_id(game_turn_info.session_id.as_str()).await {
            Some(session) => session,
            None => { return Err(GameError::GamingSessionNotFound); }
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

    #[named]
    async fn notify_observers_of_game_change(&self, game_state_change: GamingSessionStateChanges, session: &GamingSession<T>, game: &T) {
        debug!("{} - notifying observers of Game stage change: {:?}.", function_name!(), game_state_change);
        for observer in self.observers.iter() {
            let _ = observer.session_updated(&game_state_change, session, Some(game.clone())).await;
        }
    }

    #[named]
    async fn notify_observers_of_session_change(&self, state_change: GamingSessionStateChanges, game_session: &GamingSession<T>) {
        debug!("{} - notifying observers of Gaming Session change: {:?}.", function_name!(), state_change);
        for observer in self.observers.iter() {
            let _ = observer.session_updated(&state_change, game_session, None).await;
        }
    }

    #[named]
    fn remove_auto_player_observer(&mut self, game_id: &str) {
        debug!("{} called for game: {:?}.", function_name!(), game_id);
        if let Some(index) = self.observers.iter().position(|it| it.unique_id().as_str() == game_id) {
            self.observers.remove(index);
        }
    }
}
