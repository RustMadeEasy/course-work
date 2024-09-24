use std::marker::PhantomData;

use serde::Serialize;
use uuid::Uuid;

use crate::errors::GameError;
use crate::errors::GameError::BoardLocationAlreadyOccupied;
use crate::game_board::{BoardPosition, GameBoard, GamePiece};
use crate::game_state::GameState;
use crate::game_trait::GameTrait;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::requests::{GameTurnInfo, NewGameParams};
use crate::models::PlayerInfo;
use crate::play_status::PlayStatus;

/**
 * Provides Tic-Tac-Toe game play functionality.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

/// Provides Tic-Tac-Toe game play functionality.
#[derive(Clone, Serialize)]
pub(crate) struct GameEngine {
    //

    /// The Player who can currently make a game move.
    pub(crate) current_player: Option<PlayerInfo>,

    /// Provide the configuration required for clients subscribe to game updates via MQTT.
    pub(crate) event_plane_config: EventPlaneConfig,

    /// Code used to invite the second player to the game
    pub(crate) game_invitation_code: String,

    /// Unique ID of the Game Engine
    pub(crate) id: String,

    /// Helps ensures this struct can only be instantiated via new()
    #[serde(skip)]
    _outside_instantiation_preventor: PhantomData<u8>,

    /// The list of Game States from the very first turn until the latest turn
    pub(super) play_history: Vec<GameState>,

    pub(crate) players: Vec<PlayerInfo>,
}

impl GameEngine {
    //

    /// Determines whether the specified board location is occupied by a game piece.
    fn is_location_occupied(game_board: &GameBoard, position: &BoardPosition) -> bool {
        if !GameState::is_valid_board_position(position) {
            return false;
        }
        game_board[position.row][position.column] != GamePiece::None
    }
}

impl GameTrait for GameEngine {
    //

    /// Adds a Player to the Game.
    fn add_player(&mut self, display_name: impl Into<String> + Copy) -> Result<(), GameError> {
        //

        let game_piece: GamePiece;

        match self.players.len() {
            0 => {
                // Add Player One
                game_piece = GamePiece::X;
            }
            1 => {
                // Add Player Two
                game_piece = GamePiece::O;

                // Makes sure the display name of the Second Player is different from that of the First Player.
                if display_name.into().to_lowercase()
                    == self.players.first().unwrap().display_name.to_lowercase()
                {
                    return Err(GameError::DisplayNameAlreadyInUseInGame);
                }
            }
            _ => {
                // We are already maxed out
                return Err(GameError::MaximumPlayersAlreadyAdded);
            }
        }

        let player_info = PlayerInfo::new(display_name, &game_piece);

        self.players.push(player_info);

        // set the Player One to be the first to take their turn
        self.current_player = Some(self.players.first().unwrap().clone());

        Ok(())
    }

    /// Determines whether the specified Player can take a turn.
    fn _can_player_take_turn(&self, player: &PlayerInfo) -> bool {
        //

        // We can only begin Game Play when both players have been added to the Game.
        if self.players.len() < 2 {
            return false;
        }

        player.player_id == self.current_player.clone().unwrap().player_id
    }

    /// Returns the current state of the Game Board.
    fn get_current_game_state(&self) -> GameState {
        //

        if !self.play_history.is_empty() {
            self.play_history.last().unwrap().clone()
        } else {
            //

            // This is a brand-new game...

            // If we have both Players, then the game has begun!
            if self.players.len() > 1 {
                GameState::new_with_initial_play_status(
                    &self.players.first().unwrap().player_id,
                    &PlayStatus::InProgress,
                )
            } else {
                GameState::new()
            }
        }
    }

    /// Returns the Event Channel ID of this Game.
    fn get_event_channel_id(&self) -> String { self.event_plane_config.topic_prefix.clone() }

    /// Returns the ID of this Game.
    fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Returns the invitation code that can be used to add the second Player to this Game.
    fn get_invitation_code(&self) -> String {
        self.game_invitation_code.clone()
    }

    /// Returns the Game Play History.
    fn get_play_history(&self) -> Vec<GameState> {
        self.play_history.clone()
    }

    /// Returns the specified Player.
    fn get_player_info_by_id(&self, player_id: impl Into<String>) -> Result<PlayerInfo, GameError> {
        let player_id = player_id.into();
        match self.players.iter().find(|it| it.player_id == player_id) {
            None => Err(GameError::PlayerNotFound),
            Some(player) => Ok(player.clone()),
        }
    }

    /// Creates a new GameEngine instance.
    fn new(params: &NewGameParams,
           mqtt_broker_address: impl Into<String>,
           mqtt_port: u16,
           invitation_code: impl Into<String>) -> Result<Self, GameError> {
        //

        let mut engine = Self {
            current_player: None,
            id: Uuid::new_v4().to_string(),
            _outside_instantiation_preventor: Default::default(),
            players: vec![],
            play_history: vec![],
            game_invitation_code: invitation_code.into(),
            event_plane_config: EventPlaneConfig::new(mqtt_broker_address.into(), mqtt_port),
        };

        engine.add_player(&params.player_one_display_name)?;

        Ok(engine)
    }

    /// Make a game move for the specified Player.
    fn take_turn(&mut self, game_turn_info: &GameTurnInfo) -> Result<GameState, GameError> {
        //

        // Do not allow game moves when the game has already been completed
        let board_state = self.get_current_game_state();
        match board_state.get_play_status() {
            PlayStatus::EndedInStalemate | PlayStatus::EndedInWin => {
                return Err(GameError::GameHasAlreadyEnded);
            }
            PlayStatus::InProgress | PlayStatus::NotStarted => {}
        }

        // Get the Players - also validating that the correct IDs have been sent in.
        let player_taking_a_turn = self.get_player_info_by_id(&game_turn_info.player_id)?;
        let other_player =
            PlayerInfo::get_other_player_info_by_id(&game_turn_info.player_id, &self.players)?;

        // Ensure that a Player is not making a move out of turn.
        if player_taking_a_turn.player_id != self.current_player.clone().unwrap().player_id {
            return Err(GameError::WrongPlayerTakingTurn);
        }

        // Make sure that the target location is not already occupied
        if Self::is_location_occupied(&board_state.get_game_board(), &game_turn_info.destination) {
            return Err(BoardLocationAlreadyOccupied);
        }

        // Make a new Board State by adding the specified piece to the board of the current Board State.
        let final_board_state = board_state.place_game_piece(
            &game_turn_info.destination,
            &player_taking_a_turn,
            &other_player,
        )?;

        // Add this move to our Game Play History
        self.play_history.push(final_board_state.clone());

        // change Players
        self.current_player = Some(other_player);

        Ok(final_board_state.clone())
    }
}
