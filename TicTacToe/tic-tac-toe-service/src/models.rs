// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/**
 * Models used in API requests and responses and in MQTT notifications.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

use crate::errors::GameError;
use crate::game_board::GamePiece;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;


const INVITATION_CODE_LENGTH: u64 = 6;

#[derive(Clone, Debug, Default, Deserialize, ToSchema)]
pub(crate) enum AutomaticPlayerSkillLevel {
    /// Performs random moves.
    #[default]
    Beginner,
    /// Takes best tactical move.
    Intermediate,
    /// Takes the best strategic moves, looking several moves into the future.
    Expert,
    /// Expands on the expert level by also making moves that can trick the other player into making
    /// wrong moves.
    Master,
}

/// Specifies the type of Game - single player or two players.
#[derive(Debug, Deserialize, PartialEq, Serialize, ToSchema, Clone)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayers,
}

/// Models a Tic-Tac-Toe Game Player.
#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema, Validate)]
pub(crate) struct PlayerInfo {
    /// Name of the Player.
    pub(crate) display_name: String,
    /// The Game Piece with which the Tic-Tac-Toe Game is played.
    pub(crate) game_piece: GamePiece,
    /// Indicates that this Player's moves are automated, i.e., guided by this service.
    pub(crate) is_automated: bool,
    /// Unique ID of the Player.
    pub(crate) player_id: String,
}

impl PlayerInfo {
    //

    /// Returns the Player other than the specified Player.
    pub(crate) fn get_other_player_info_by_id(
        player_id: impl Into<String>,
        players: &[PlayerInfo],
    ) -> Result<PlayerInfo, GameError> {
        if players.len() < 2 {
            return Err(GameError::PlayerNotFound);
        }

        let player_id = player_id.into();
        match players.iter().find(|it| it.player_id != player_id) {
            None => Err(GameError::PlayerNotFound),
            Some(player) => Ok(player.clone()),
        }
    }

    /// Creates a new PlayerInfo instance.
    pub(crate) fn new(display_name: impl Into<String>,
                      is_automated: bool) -> Self {
        Self {
            display_name: display_name.into(),
            game_piece: GamePiece::None,
            is_automated,
            player_id: Uuid::new_v4().to_string(),
        }
    }
}

pub mod event_plane {
    use serde::{Deserialize, Serialize};
    use strum::Display;
    use utoipa::ToSchema;

    const DOMAIN_NAME: &str = "RustMadeEasy.com";

    /// Models the configuration required for clients to subscribe to real-time Game state updates.
    #[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
    pub struct EventPlaneConfig {
        //

        /// Address of the real-time messaging broker.
        pub(crate) broker_address: String,

        /// Channel used to namespace the messaging.
        pub(crate) channel_id: String,

        /// Broker port number of the real-time messaging broker.
        pub(crate) broker_port: u16,

        /// The topic prefix that allows the clients to subscribe to real-time Game state updates.
        pub(crate) topic_prefix: String,
    }

    impl EventPlaneConfig {
        /// Creates a new EventPlaneConfig instance.
        pub(crate) fn new(broker_address: String, broker_port: u16, channel_id: String) -> Self {
            Self {
                broker_address,
                channel_id: channel_id.clone(),
                broker_port,
                topic_prefix: EventPlaneTopicNames::build_topic_prefix(&channel_id),
            }
        }
    }

    /// Defines the names of the subscription topics used in the real-time messaging event plane.
    ///
    /// A full topic takes the form:
    ///
    /// `[topic_prefix]/[event topic name]`
    ///
    /// NOTE: The topic_prefix can be obtained from the event_plane_config field of the
    /// GameCreationResult model that is returned when creating a new Game or when adding a new
    /// Player to a Game.
    #[derive(Deserialize, Display, Serialize, ToSchema)]
    pub enum EventPlaneTopicNames {
        /// Called when the Game has been deleted from the platform.
        GameDeleted,
        /// Called when the Game has ended in a stalemate.
        GameEndedInStalemate,
        /// Called when the Game has ended in a win.
        GameEndedInWin,
        /// Published when the Game has started.
        GameStarted,
        /// Published when a new Player has been added to the Gaming Session.
        PlayerAddedToSession,
        /// Published when a new Player is ready to begin the Game.
        PlayerReady,
        /// Called when the Gaming Session has been deleted from the platform.
        SessionDeleted,
        /// Published when a Player has taken a new turn.
        TurnTaken,
    }

    impl EventPlaneTopicNames {
        //

        /// Constructs a topic specific to the Session ID.
        pub(crate) fn build(&self, topic_prefix: &str) -> String {
            match self {
                EventPlaneTopicNames::GameDeleted => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameDeleted),
                EventPlaneTopicNames::GameEndedInStalemate => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameEndedInStalemate),
                EventPlaneTopicNames::GameEndedInWin => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameEndedInWin),
                EventPlaneTopicNames::GameStarted => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameStarted),
                EventPlaneTopicNames::PlayerAddedToSession => format!("{topic_prefix}/{}", EventPlaneTopicNames::PlayerAddedToSession),
                EventPlaneTopicNames::PlayerReady => format!("{topic_prefix}/{}", EventPlaneTopicNames::PlayerReady),
                EventPlaneTopicNames::TurnTaken => format!("{topic_prefix}/{}", EventPlaneTopicNames::TurnTaken),
                EventPlaneTopicNames::SessionDeleted => format!("{topic_prefix}/{}", EventPlaneTopicNames::SessionDeleted),
            }
        }

        /// Constructs a topic prefix specific to the Channel ID.
        pub(crate) fn build_topic_prefix(event_channel_id: &str) -> String {
            format!("{DOMAIN_NAME}/Channels/{event_channel_id}")
        }
    }
}

pub mod requests {
    use crate::game_board::BoardPosition;
    use crate::models::AutomaticPlayerSkillLevel;
    use crate::models::INVITATION_CODE_LENGTH;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    pub(crate) const ID_LENGTH_MAX: u64 = 36;
    const ID_LENGTH_MIN: u64 = 1;
    const NAME_LENGTH_MAX: u64 = 40;
    const NAME_LENGTH_MIN: u64 = 1;

    /// Models info needed to end a Game.
    #[derive(Debug, Deserialize, ToSchema, Validate)]
    pub struct EndGameParams {
        #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
        pub player_id: String,
        #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
        pub session_id: String,
    }

    /// Models info needed to end a Gaming Session.
    #[derive(Debug, Deserialize, ToSchema, Validate)]
    pub struct EndGamingSessionParams {
        #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
        pub player_id: String,
    }

    /// Models info needed to perform a Game turn.
    #[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
    pub struct GameTurnInfo {
        pub destination: BoardPosition,
        #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
        pub player_id: String,
        #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
        pub session_id: String,
    }

    /// Models info needed to join a Gaming Session.
    #[derive(Debug, Deserialize, ToSchema, Validate)]
    pub struct JoinSessionParams {
        #[validate(length(min = "INVITATION_CODE_LENGTH", max = "INVITATION_CODE_LENGTH"))]
        pub game_invitation_code: String,
        #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
        pub player_display_name: String,
    }

    /// Models info needed to start a new Gaming Session.
    #[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
    pub struct NewGamingSessionParams {
        #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
        pub session_owner_display_name: String,
    }

    /// Models info needed to start a new Single-Player Game.
    #[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
    pub struct NewSinglePlayerGameParams {
        pub computer_skill_level: AutomaticPlayerSkillLevel,
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    use crate::game_state::GameState;
    use crate::game_trait::GameTrait;
    use crate::models::event_plane::EventPlaneConfig;
    use crate::models::PlayerInfo;
    use crate::tic_tac_toe_game::TicTacToeGame;

    /// Models the current view of a Game.
    #[derive(Deserialize, Serialize, ToSchema)]
    pub struct GameInfo {
        //

        /// Player who has an open turn
        pub(crate) current_player: Option<PlayerInfo>,

        /// The current state the Game
        pub game_state: GameState,

        /// Unique ID of the Game instance
        pub(crate) id: String,

        /// List of Players
        pub(crate) players: Vec<PlayerInfo>,
    }

    impl From<TicTacToeGame> for GameInfo {
        fn from(game: TicTacToeGame) -> GameInfo {
            let game_state = game.get_current_game_state();
            GameInfo {
                current_player: game.current_player,
                game_state,
                id: game.id.clone(),
                players: game.players,
            }
        }
    }

    /// Models the results of a call to the Create Gaming Session endpoint.
    #[derive(Deserialize, Serialize, ToSchema)]
    pub struct GamingSessionCreationResult {
        /// Specifies the configuration required for clients to subscribe to real-time Game state updates.
        pub(crate) event_plane_config: EventPlaneConfig,
        /// Unique Code that is used to invite others to the Gaming Session.
        pub(crate) invitation_code: String,
        /// ID of the Player added to the Gaming Session.
        pub(crate) player_id: String,
        /// Identifies the Gaming Session. This also serves as the communication channel for MQTT notifications.
        pub(crate) session_id: String,
    }

    /// Models the results of a call to the Create Game endpoint.
    #[derive(Deserialize, Serialize, ToSchema)]
    pub struct GameCreationResult {
        /// The initial Game state.
        pub(crate) game_info: GameInfo,
        /// ID of the Gaming Session.
        pub(crate) session_id: String,
    }
}
