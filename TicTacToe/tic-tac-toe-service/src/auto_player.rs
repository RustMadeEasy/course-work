use crate::game_board::{BoardPosition, GameBoard, GamePiece, MAX_BOARD_COLUMNS, MAX_BOARD_ROWS};
use crate::models::PlayerInfo;
use serde::Deserialize;
use utoipa::ToSchema;

/// AutoPlayer can play a game of Tic-Tac-Toe at various skill levels.
pub(crate) struct AutoPlayer {
    player_info: PlayerInfo,
    skill_level: SkillLevel,
}

impl AutoPlayer {
    //

    pub(crate) fn new(player_info: PlayerInfo, skill_level: SkillLevel) -> Self {
        Self {
            player_info,
            skill_level,
        }
    }

    pub(crate) fn take_turn(&self, game_board: GameBoard) -> Option<BoardPosition> {
        match self.skill_level {
            SkillLevel::Beginner => self.take_turn_as_a_beginner(game_board),
            SkillLevel::Intermediate => self.take_turn_as_an_intermediate(game_board),
            SkillLevel::Expert => self.take_turn_as_an_expert(game_board),
            SkillLevel::Master => self.take_turn_as_a_master(game_board),
        }
    }
}

impl AutoPlayer {
    //

    fn take_turn_as_a_beginner(&self, game_board: GameBoard) -> Option<BoardPosition> {

        // *** Select any random, open location on the board ***

        match Self::determine_empty_locations(&game_board) {
            None => None,
            Some(empty_locations) => {
                let index = (rand::random::<f32>() * empty_locations.len() as f32).floor() as usize;
                Some(empty_locations[index].clone())
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

impl AutoPlayer {
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

        Some(empty_locations)
    }
}

#[derive(Deserialize, ToSchema)]
pub(crate) enum SkillLevel {
    /// Performs random moves.
    Beginner,
    /// Takes best tactical move.
    Intermediate,
    /// Takes the best strategic moves, looking several moves into the future.
    Expert,
    /// Expands on the expert level by also making moves that can trick the other player into making
    /// wrong moves.
    Master,
}
