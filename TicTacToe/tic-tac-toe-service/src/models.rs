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
    use uuid::Uuid;

    const DOMAIN_NAME: &str = "RustMadeEasy.com";

    /// Models the configuration required for clients subscribe to real-time game state updates.
    #[derive(Clone, Deserialize, Serialize, ToSchema)]
    pub struct EventPlaneConfig {
        //

        /// Address of the real-time messaging broker.
        pub(crate) broker_address: String,

        /// Broker port number of the real-time messaging broker.
        pub(crate) broker_port: u16,

        /// The topic prefix that allows the clients to subscribe to real-time game state updates.
        pub(crate) topic_prefix: String,
    }

    impl EventPlaneConfig {
        pub(crate) fn new(broker_address: String, broker_port: u16) -> Self {
            Self {
                broker_address,
                broker_port,
                topic_prefix: EventPlaneTopicNames::build_topic_prefix(Uuid::new_v4().to_string()),
            }
        }
    }

    /// Names of the subscription topics used in the real-time messaging event plane. A full topic
    /// takes the form:
    ///
    /// `[topic_prefix]/[event topic name]`
    ///
    /// NOTE: The topic_prefix can be obtained from the event_plane_config field of the
    /// GameCreationResult model that is returned when creating a new game or when adding a new
    /// player to a Game.
    #[derive(Deserialize, Serialize, ToSchema)]
    pub enum EventPlaneTopicNames {
        /// Called when the game has ended in a stalemate. The client can call Get Game Info to retrieve the details.
        GameEndedInStalemate,
        /// Called when the game has ended in a win. The client can call Get Game Info to retrieve the details.
        GameEndedInWin,
        /// Published when a new player has been added the game. The client can call Get Game Info to retrieve the details.
        PlayerAdded,
        /// Published when a player has taken a new turn. The client can call Get Game Info to retrieve the new board state.
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
        pub(crate) fn build_topic_prefix(event_channel_id: String) -> String {
            format!("{DOMAIN_NAME}/Channels/{event_channel_id}")
        }
    }
}

/// Models a Tic-Tac-Toe game Player.
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct PlayerInfo {
    pub(crate) display_name: String,
    pub(crate) game_piece: GamePiece,
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
    pub(crate) fn new(display_name: impl Into<String>, game_piece: &GamePiece) -> Self {
        Self {
            display_name: display_name.into(),
            player_id: Uuid::new_v4().to_string(),
            game_piece: game_piece.clone(),
        }
    }
}

pub mod requests {
    use serde::Deserialize;
    use utoipa::ToSchema;
    use validator::Validate;

    use crate::game_board::BoardPosition;

    pub(crate) const ID_LENGTH_MAX: u64 = 36;
    const ID_LENGTH_MIN: u64 = 1;
    const INVITATION_CODE_LENGTH: u64 = 6;
    const NAME_LENGTH_MAX: u64 = 40;
    const NAME_LENGTH_MIN: u64 = 1;

    /// Models info needed to add a Player to a game.
    #[derive(Deserialize, ToSchema, Validate)]
    pub struct AddPlayerParams {
        #[validate(length(min = "INVITATION_CODE_LENGTH", max = "INVITATION_CODE_LENGTH"))]
        pub game_invitation_code: String,
        #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
        pub player_display_name: String,
    }

    /// Models info needed to perform a game turn.
    #[derive(Deserialize, ToSchema, Validate)]
    pub struct GameTurnInfo {
        pub destination: BoardPosition,
        #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
        pub player_id: String,
    }

    /// Models info needed to start a new Game.
    #[derive(Deserialize, ToSchema, Validate)]
    pub struct NewGameParams {
        #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
        pub player_one_display_name: String,
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    use crate::errors::GameError;
    use crate::game_state::GameState;
    use crate::game_trait::GameTrait;
    use crate::models::event_plane::EventPlaneConfig;
    use crate::models::PlayerInfo;
    use crate::tic_tac_toe_game::TicTacToeGame;

    /// Models the view of a Game.
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

    impl TryFrom<TicTacToeGame> for GameInfo {
        type Error = GameError;

        fn try_from(game: TicTacToeGame) -> Result<Self, Self::Error> {
            let game_state = game.get_current_game_state();
            Ok(GameInfo {
                current_player: game.current_player,
                game_state,
                id: game.id.clone(),
                players: game.players,
            })
        }
    }

    /// Models the results of a call to the Create Game and Add Player endpoints.
    #[derive(Deserialize, Serialize, ToSchema)]
    pub struct GameCreationResult {
        //

        /// The initial game state.
        pub(crate) game_info: GameInfo,

        /// Configuration required for clients subscribe to real-time game state updates.
        pub(crate) event_plane_config: EventPlaneConfig,

        /// Code used to invite the second player to the game
        pub(crate) game_invitation_code: String,
    }

    impl TryFrom<TicTacToeGame> for GameCreationResult {
        type Error = GameError;

        fn try_from(game: TicTacToeGame) -> Result<Self, Self::Error> {
            let game_invitation_code = game.game_invitation_code.clone();
            Ok(GameCreationResult {
                game_info: GameInfo::try_from(game.clone()).unwrap(),
                event_plane_config: game.event_plane_config,
                game_invitation_code,
            })
        }
    }
}
