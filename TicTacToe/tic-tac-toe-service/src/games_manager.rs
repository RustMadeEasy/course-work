use std::collections::{HashMap, HashSet};
use std::time::Duration;

use log::warn;
use mqtt_publisher_lib::broker_config::{MqttProtocolVersion, PublisherConfig};
use mqtt_publisher_lib::publisher::Publisher;
use mqtt_publisher_lib::publisher_qos::PublisherQoS;

use crate::errors::GameError;
use crate::game_engine::GameEngine;
use crate::game_state::GameState;
use crate::game_trait::GameTrait;
use crate::models::event_plane::EventPlaneTopicNames;
use crate::models::requests::{AddPlayerParams, GameTurnInfo, NewGameParams};
use crate::play_status::PlayStatus;
use crate::verification_code_generator::VerificationCodeGenerator;

pub(crate) type TicTacToeGamesManager = GamesManager<GameEngine>;

const MQTT_BROKER_ADDRESS: &str = "test.mosquitto.org";
const MQTT_PORT: u16 = 1883;

/**
 * Manages all the Game instances played on this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Joel@RustMadeEasy.com
 */

/// Manages all the Game instances played on this service.
///
/// NOTE: this is sample code.
///
/// NOTE: Production-grade code would persist the gaming info to a mem cache or database so that
/// multiple instances of the service can be run.
#[derive(Clone)]
pub(crate) struct GamesManager<G: GameTrait + Clone> {
    //

    /// Provides MQTT message publishing functionality.
    event_publisher: Publisher,

    /// The games being managed by this instance. They are stored by game ID.
    games: HashMap<String, G>,
}

impl<GenericGame: GameTrait + Clone> GamesManager<GenericGame> {
    //

    /// Adds a Player to the Game.
    pub(crate) async fn add_player(
        &mut self,
        second_player_params: &AddPlayerParams,
    ) -> Result<GenericGame, GameError> {
        //

        // Find the Game via the game_invitation_code
        let mut game_engine =
            match self.get_game_by_invitation_code(&second_player_params.game_invitation_code) {
                None => {
                    return Err(GameError::InvitationCodeNotFound);
                }
                Some(game_engine) => game_engine,
            };

        game_engine.add_player(&second_player_params.player_display_name)?;

        // Update the Game Engine in the list
        self.games.insert(game_engine.get_id(), game_engine.clone());

        // Inform the listening clients that a Player has been added.
        let topic = EventPlaneTopicNames::PlayerAdded.build(game_engine.get_event_channel_id().as_str());
        let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;

        Ok(game_engine)
    }

    /// Creates a new Game Engine.
    pub(crate) fn create_game_engine(
        &mut self,
        params: &NewGameParams,
    ) -> Result<GenericGame, GameError> {
        //

        let invitation_code = self.generate_invitation_code();
        let game_engine = GenericGame::new(params,
                                           MQTT_BROKER_ADDRESS,
                                           MQTT_PORT,
                                           invitation_code)?;

        self.games.insert(game_engine.get_id().clone(), game_engine.clone());

        Ok(game_engine.clone())
    }

    /// Closes down the specified game and returns its final game state.
    pub(crate) fn end_game(&mut self, game_id: &String) -> Result<(), GameError> {
        //

        let game_engine = self.get_game_engine(game_id)?;

        self.games.remove(&game_engine.get_id());

        Ok(())
    }

    /// Retrieves the specified Game Engine.
    pub(crate) fn get_game_engine(&self, game_id: impl Into<String>) -> Result<GenericGame, GameError> {
        match self.games.get(&game_id.into()) {
            None => Err(GameError::GameNotFound),
            Some(game) => Ok(game.clone()),
        }
    }

    /// Retrieves the history of the Game States from the initial creation through to the current
    /// Game State. This can be used, for instance, the client could provide an animation that
    /// shows a time-lapse of the game play.
    pub(crate) fn get_game_history(&self, game_id: &String) -> Result<Vec<GameState>, GameError> {
        let game_engine = self.get_game_engine(game_id)?;
        Ok(game_engine.get_play_history())
    }

    /// Creates a new GamesManager instance.
    pub(crate) fn new() -> Self {
        let mut configs = HashSet::new();
        configs.insert(PublisherConfig::new(MQTT_BROKER_ADDRESS.to_string(), MQTT_PORT, MqttProtocolVersion::V5));
        Self {
            event_publisher: Publisher::new(configs, Duration::from_secs(60), 10),
            games: Default::default(),
        }
    }

    /// Make a game move for the specified Player.
    pub(crate) async fn take_turn(
        &mut self,
        game_id: &String,
        game_turn_info: &GameTurnInfo,
    ) -> Result<GameState, GameError> {
        //

        let mut game_engine = self.get_game_engine(game_id)?;
        let new_game_state = game_engine.take_turn(game_turn_info)?;

        // Update our game engine
        self.games.insert(game_engine.get_id().clone(), game_engine.clone());

        // Inform the listening clients that a Player has taken a new turn.
        let event_channel_id = game_engine.get_event_channel_id();
        let topic = EventPlaneTopicNames::TurnTaken.build(event_channel_id.as_str());
        let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;

        // If the game has ended, let the listening clients know how it ended.
        match new_game_state.get_play_status() {
            PlayStatus::EndedInStalemate => {
                let topic = EventPlaneTopicNames::GameEndedInStalemate.build(event_channel_id.as_str());
                let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
            }
            PlayStatus::EndedInWin => {
                let topic = EventPlaneTopicNames::GameEndedInWin.build(event_channel_id.as_str());
                let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
            }
            _ => {}
        }

        Ok(new_game_state)
    }
}

impl<GenericGame: GameTrait + Clone> GamesManager<GenericGame> {
    //

    /// Retrieves a game by its invitation code.
    fn get_game_by_invitation_code(&self, invitation_code: &String) -> Option<GenericGame> {
        self.games
            .iter()
            .find(|it| it.1.get_invitation_code() == *invitation_code)
            .map(|it| it.1.clone())
    }

    /// Creates a unique, 6-digit code for use as a Game Invitation. We use a 6-digit Game
    /// Invitation instead of performing the game setup handshaking with the Game ID for 2 reasons:
    ///     1) We don't want to expose the Game ID to clients that are not party to the Game.
    ///     2) A 6-digit code is practical for end-users to utilize.
    fn generate_invitation_code(&self) -> String {
        //

        // Place a limit to prevent endless loop.
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
