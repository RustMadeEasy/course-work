/**
 * Models used in API requests and responses.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Joel@RustMadeEasy.com
 */

pub mod requests {
    use serde::Deserialize;
    use utoipa::ToSchema;

    use crate::game_board::BoardPosition;

    /// Models info needed to add a player to a game.
    #[derive(Deserialize, ToSchema)]
    pub struct AddPlayerParams {
        pub game_invitation_code: String,
        pub player_display_name: String,
    }

    /// Models info needed to perform a game turn.
    #[derive(Deserialize, ToSchema)]
    pub struct GameTurnInfo {
        pub destination: BoardPosition,
        pub player_id: String,
    }

    /// Models info needed to start a new Game.
    // TODO: HD: implement Validation
    #[derive(Deserialize, ToSchema)]
    pub struct NewGameParams {
        pub player_one_display_name: String,
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

    use crate::errors::GameError;
    use crate::game_engine::GameEngine;
    use crate::game_state::GameState;
    use crate::game_trait::GameTrait;
    use crate::player_info::PlayerInfo;

    /// Models the view of a Game.
    #[derive(Deserialize, Serialize, ToSchema)]
    pub struct GameInfo {
        //
        /// The current state the Game
        pub game_state: GameState,

        // TODO: HD: show the winning combination
        /// Unique ID of the Game Engine
        pub(crate) id: String,

        /// Code used to invite the second player to the game
        pub(crate) game_invitation_code: String,

        /// Player who has an open turn
        pub(crate) current_player: Option<PlayerInfo>,

        /// List of Players
        pub(crate) players: Vec<PlayerInfo>,
    }

    impl TryFrom<GameEngine> for GameInfo {
        type Error = GameError;

        fn try_from(value: GameEngine) -> Result<Self, Self::Error> {
            let game_state = value.get_current_game_state();
            Ok(GameInfo {
                game_state,
                id: value.id.clone(),
                game_invitation_code: value.game_invitation_code,
                current_player: value.current_player,
                players: value.players,
            })
        }
    }
}
