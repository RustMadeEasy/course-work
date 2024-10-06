use crate::game_board::{BoardPosition, GameBoard, GamePiece, MAX_BOARD_COLUMNS, MAX_BOARD_ROWS};
use crate::game_observer_trait::{GameObserverTrait, GameStateChange};
use crate::game_trait::GameTrait;
use crate::models::requests::GameTurnInfo;
use crate::models::{AutoPlayerSkillLevel, PlayerInfo};
use crate::play_status::PlayStatus;
use async_trait::async_trait;
use log::error;
use std::marker::PhantomData;
use tokio::time::{sleep, Duration};

// To help make the auto-player feel like more human, we deliberate on the move for anywhere
// between MIN_DELIBERATION_TIME and MAX_DELIBERATION_TIME seconds.
static MAX_DELIBERATION_TIME: f32 = 3_f32;
static MIN_DELIBERATION_TIME: usize = 1;

/// AutoPlayer can play a game of Tic-Tac-Toe at various skill levels.
pub(crate) struct AutoPlayer<T: GameTrait + Clone + Send + Sync> {
    game_id: String,
    phantom_type: PhantomData<T>,
    player_info: PlayerInfo,
    skill_level: AutoPlayerSkillLevel,
}

impl<T: GameTrait + Clone + Send + Sync> AutoPlayer<T> {
    //

    pub(crate) fn get_name() -> String {
        "Reema".to_string()
    }

    pub(crate) fn new(game_id: &String, player_info: PlayerInfo, skill_level: AutoPlayerSkillLevel) -> Self {
        Self {
            game_id: game_id.clone(),
            phantom_type: Default::default(),
            player_info,
            skill_level,
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
        // TODO: JD: consider blocking the opponent from winning
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

    pub(crate) async fn take_turn(&self, game: &T) {
        //

        let game_board = game.get_current_game_state().game_board;

        if let Some(new_board_position) = match self.skill_level {
            AutoPlayerSkillLevel::Beginner => self.take_turn_as_a_beginner(game_board),
            AutoPlayerSkillLevel::Intermediate => self.take_turn_as_an_intermediate(game_board),
            AutoPlayerSkillLevel::Expert => self.take_turn_as_an_expert(game_board),
            AutoPlayerSkillLevel::Master => self.take_turn_as_a_master(game_board),
        } {
            let game_id = game.get_id();
            let player_id = self.player_info.player_id.clone();

            tokio::spawn(async move {

                // Make the service feel like it is deliberating on the move for some time.
                // We wait anywhere between 1 and 4 seconds.
                let wait_time_secs = (rand::random::<f32>() * MAX_DELIBERATION_TIME).floor() as usize + MIN_DELIBERATION_TIME;
                sleep(Duration::from_secs(wait_time_secs as u64)).await;

                // *** Control the service via the API in the same way clients do. ***

                let url = format!("http://127.0.0.1:50020/v1/games/{}/turns", game_id);
                let client = reqwest::Client::new();

                let game_turn_info = GameTurnInfo { destination: new_board_position, player_id };
                let result = client.post(url)
                    .json(&game_turn_info)
                    .send()
                    .await;

                match result {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Failed to send game turn. Error {}", error);
                    }
                }
            });
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

// GameObserverTrait implementation
#[async_trait]
impl<T: GameTrait + Clone + Send + Sync + 'static> GameObserverTrait<T> for AutoPlayer<T> {
    //

    async fn game_updated(&self, game_state_change: &GameStateChange, game: &T) {
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
                                self.take_turn(game).await;
                            }
                        }
                    }
                    PlayStatus::NotStarted => {} // Early return. Nothing to do.
                    _ => {}
                }
            }
        }
    }

    fn unique_id(&self) -> String {
        self.game_id.clone()
    }
}
