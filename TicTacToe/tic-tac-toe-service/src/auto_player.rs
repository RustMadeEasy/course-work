use crate::game_board::{BoardPosition, GameBoard, GamePiece, MAX_BOARD_COLUMNS, MAX_BOARD_ROWS};
use crate::game_observer_trait::{GameObserverTrait, GameStateChange};
use crate::game_trait::GameTrait;
use crate::models::requests::GameTurnInfo;
use crate::models::PlayerInfo;
use crate::play_status::PlayStatus;
use async_trait::async_trait;
use serde::Deserialize;
use std::marker::PhantomData;
use utoipa::ToSchema;

/// AutoPlayer can play a game of Tic-Tac-Toe at various skill levels.
pub(crate) struct AutoPlayer<T: GameTrait + Clone + Send + Sync> {
    player_info: PlayerInfo,
    skill_level: SkillLevel,
    phantom_type: PhantomData<T>,
}

impl<T: GameTrait + Clone + Send + Sync> AutoPlayer<T> {
    //

    pub(crate) fn new(player_info: PlayerInfo, skill_level: SkillLevel) -> Self {
        Self {
            player_info,
            skill_level,
            phantom_type: Default::default(),
        }
    }
}

impl<T: GameTrait + Clone + Send + Sync> AutoPlayer<T> {
    //

    fn take_turn_as_a_beginner(&self, game_board: GameBoard) -> Option<BoardPosition> {

        // *** Select any random, open location on the board ***

        match Self::determine_empty_locations(&game_board) {
            None => None,
            Some(open_locations) => {
                let index = (rand::random::<f32>() * open_locations.len() as f32).floor() as usize;
                Some(open_locations[index].clone())
            }
        }
    }

    fn take_turn_as_an_intermediate(&self, _game_board: GameBoard) -> Option<BoardPosition> {
        // TODO: JD: finish
        None
    }

    fn take_turn_as_an_expert(&self, _game_board: GameBoard) -> Option<BoardPosition> {
        // TODO: JD: finish
        None
    }

    fn take_turn_as_a_master(&self, _game_board: GameBoard) -> Option<BoardPosition> {
        // TODO: JD: finish
        None
    }
}

impl<T: GameTrait + Clone + Send + Sync> AutoPlayer<T> {
    //

    pub(crate) fn take_turn(&self, game: &mut T) {
        //

        let game_board = game.get_current_game_state().game_board;

        if let Some(new_board_position) = match self.skill_level {
            SkillLevel::Beginner => self.take_turn_as_a_beginner(game_board),
            SkillLevel::Intermediate => self.take_turn_as_an_intermediate(game_board),
            SkillLevel::Expert => self.take_turn_as_an_expert(game_board),
            SkillLevel::Master => self.take_turn_as_a_master(game_board),
        } {
            let game_turn_info = GameTurnInfo {
                destination: new_board_position,
                player_id: self.player_info.player_id.clone(),
            };
            let _ = game.take_turn(&game_turn_info);
        }
    }
}

impl<T: GameTrait + Clone + Send + Sync> AutoPlayer<T> {
    //

    /// Determines the empty locations on the specified Game Board.
    pub(crate) fn determine_empty_locations(grid: &GameBoard) -> Option<Vec<BoardPosition>> {
        //

        if grid.is_empty() {
            return None;
        }

        let mut empty_locations: Vec<BoardPosition> = Vec::new();

        for row in 0..MAX_BOARD_ROWS {
            for column in 0..MAX_BOARD_COLUMNS {
                let game_piece = grid[row][column].clone();
                if game_piece == GamePiece::None {
                    empty_locations.push(BoardPosition::new(row, column));
                }
            }
        }

        match empty_locations.is_empty() {
            false => Some(empty_locations),
            true => None,
        }
    }
}

#[async_trait]
impl<T: GameTrait + Clone + Send + Sync> GameObserverTrait<T> for AutoPlayer<T> {
    //

    async fn game_updated(&self, game_state_change: &GameStateChange, game: &mut T) {
        //

        match game_state_change {
            GameStateChange::PlayerAdded => {}
            GameStateChange::TurnTaken => {
                let game_state = game.get_current_game_state();
                match game_state.play_status {
                    PlayStatus::InProgress => {
                        // Is it my turn?
                        if let Some(current_player) = game.get_current_player() {
                            if current_player.player_id == self.player_info.player_id {
                                self.take_turn(game);
                            }
                        }
                    }
                    PlayStatus::NotStarted => {} // Early return. Nothing to do.
                    _ => {}
                }
            }
        }
    }
}

#[derive(Clone, Default, Deserialize, ToSchema)]
pub(crate) enum SkillLevel {
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

