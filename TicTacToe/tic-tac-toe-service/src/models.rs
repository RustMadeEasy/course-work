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

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;


use crate::errors::GameError;
use crate::game_board::GamePiece;

pub mod event_plane {
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    const DOMAIN_NAME: &str = "RustMadeEasy.com";

    /// Models the configuration required for clients to subscribe to real-time Game state updates.
    #[derive(Clone, Deserialize, Serialize, ToSchema)]
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
    #[derive(Deserialize, Serialize, ToSchema)]
    pub enum EventPlaneTopicNames {
        /// Called when the Game has ended in a stalemate. The client can call Get Game Info to
        /// retrieve the details.
        GameEndedInStalemate,
        /// Called when the Game has ended in a win. The client can call Get Game Info to retrieve
        /// the details.
        GameEndedInWin,
        /// Published when a new Player has been added the Game. The client can call Get Game Info
        /// to retrieve the details.
        PlayerAdded,
        /// Published when a Player has taken a new turn. The client can call Get Game Info to
        /// retrieve the new board state.
        TurnTaken,
    }

    impl EventPlaneTopicNames {
        //

        /// Constructs a topic specific to the Session ID.
        pub(crate) fn build(&self, topic_prefix: &str) -> String {
            match self {
                EventPlaneTopicNames::GameEndedInStalemate => format!("{topic_prefix}/GameEndedInStalemate"),
                EventPlaneTopicNames::GameEndedInWin => format!("{topic_prefix}/GameEndedInWin"),
                EventPlaneTopicNames::PlayerAdded => format!("{topic_prefix}/PlayerAdded"),
                EventPlaneTopicNames::TurnTaken => format!("{topic_prefix}/TurnTaken"),
            }
        }

        /// Constructs a topic prefix specific to the Channel ID.
        pub(crate) fn build_topic_prefix(event_channel_id: &String) -> String {
            format!("{DOMAIN_NAME}/Channels/{event_channel_id}")
        }
    }
}

/// Models a Tic-Tac-Toe Game Player.
#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
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
    pub(crate) fn new(display_name: impl Into<String>, game_piece: &GamePiece, is_automated: bool) -> Self {
        Self {
            display_name: display_name.into(),
            game_piece: game_piece.clone(),
            is_automated,
            player_id: Uuid::new_v4().to_string(),
        }
    }
}

pub mod requests {
    use crate::auto_player::SkillLevel;
    use crate::game_board::BoardPosition;
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;
    use validator::Validate;

    pub(crate) const ID_LENGTH_MAX: u64 = 36;
    const ID_LENGTH_MIN: u64 = 1;
    const INVITATION_CODE_LENGTH: u64 = 6;
    const NAME_LENGTH_MAX: u64 = 40;
    const NAME_LENGTH_MIN: u64 = 1;

    /// Models info needed to add a Player to a Game.
    #[derive(Deserialize, ToSchema, Validate)]
    pub struct AddPlayerParams {
        #[validate(length(min = "INVITATION_CODE_LENGTH", max = "INVITATION_CODE_LENGTH"))]
        pub game_invitation_code: String,
        #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
        pub player_display_name: String,
    }

    /// Models info needed to perform a Game turn.
    #[derive(Deserialize, ToSchema, Validate)]
    pub struct GameTurnInfo {
        pub destination: BoardPosition,
        #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
        pub player_id: String,
    }

    /// Specifies the type of Game - single player or two players.
    #[derive(Deserialize, PartialEq, Serialize, ToSchema)]
    pub enum GameMode {
        SinglePlayer,
        TwoPlayers,
    }

    /// Models info needed to start a new Game.
    #[derive(Deserialize, ToSchema, Validate)]
    pub struct NewGameParams {
        pub game_mode: GameMode,
        #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
        pub player_one_display_name: String,
        pub single_player_skill_level: Option<SkillLevel>,
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

    /// Models the results of a call to the Create Game and Add Player endpoints.
    #[derive(Deserialize, Serialize, ToSchema)]
    pub struct GameCreationResult {
        //

        /// The initial Game state.
        pub(crate) game_info: GameInfo,

        /// Configuration required for clients subscribe to real-time Game state updates.
        pub(crate) event_plane_config: EventPlaneConfig,

        /// Code used to invite the second Player to the Game
        pub(crate) game_invitation_code: String,
    }
}
