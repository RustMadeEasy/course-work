// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::game_board::GameBoard;
use crate::gaming::game_trait::GameTrait;
use crate::gaming::gaming_session::GamingSession;
use crate::gaming::gaming_session_observer_trait::GamingSessionObserverTrait;
use crate::gaming::gaming_session_state_changes::GamingSessionStateChanges;
use crate::models::automatic_player_skill_level::AutomaticPlayerSkillLevel;
use crate::models::board_position::BoardPosition;
use crate::models::game_piece::GamePiece;
use crate::models::play_status::PlayStatus;
use crate::models::player_info::PlayerInfo;
use crate::models::requests::GameTurnParams;
use async_trait::async_trait;
use function_name::named;
use log::{debug, error, info};
use std::marker::PhantomData;
use tokio::time::{sleep, Duration};

// To help make the auto-player feel more human, we deliberate on the move for anywhere
// between MIN_DELIBERATION_TIME and MAX_DELIBERATION_TIME seconds.

/// Maximum automatic turn deliberation time in seconds
static MAX_DELIBERATION_TIME_SECS: f32 = 3_f32;
/// Minimum automatic turn deliberation time in seconds
static MIN_DELIBERATION_TIME_SECS: usize = 1;

/// AutomaticPlayer can play a game of Tic-Tac-Toe at various skill levels.
pub(crate) struct AutomaticPlayer<T: GameTrait + Clone + Send + Sync> {
    /// The ID the Game being played.
    game_id: String,
    /// Needed to mark T as in use
    phantom_type: PhantomData<T>,
    /// The Automatic Player's details
    player_info: PlayerInfo,
    /// The skill level at which the Automatic Player is to play the Game.
    skill_level: AutomaticPlayerSkillLevel,
}

impl<T: GameTrait + Clone + Send + Sync> AutomaticPlayer<T> {
    //

    /// Returns the name of the AutomaticPlayer.
    pub(crate) fn get_name() -> String {
        "Reema".to_string()
    }

    /// Creates a new instance.
    pub(crate) fn new(game_id: &str, player_info: &PlayerInfo, skill_level: &AutomaticPlayerSkillLevel) -> Self {
        info!("Creating AutomaticPlayer {}", game_id);
        Self {
            game_id: game_id.to_string(),
            phantom_type: Default::default(),
            player_info: player_info.clone(),
            skill_level: skill_level.clone(),
        }
    }
}

impl<T: GameTrait + Clone + Send + Sync> AutomaticPlayer<T> {
    //

    /// Takes a turn with the skill level of a beginning player.
    fn take_turn_as_a_beginner(&self, game_board: GameBoard) -> Option<BoardPosition> {
        //

        debug!("Taking AutomaticPlayer turn as a beginner for game {}", self.game_id);

        // *** Select any random, open location on the board ***

        match Self::determine_empty_locations(&game_board) {
            None => None,
            Some(open_locations) => {
                let index = (rand::random::<f32>() * open_locations.len() as f32).floor() as usize;
                Some(open_locations[index].clone())
            }
        }
    }

    /// Takes a turn with the skill level of an intermediate player.
    fn take_turn_as_an_intermediate(&self, _game_board: GameBoard) -> Option<BoardPosition> {
        debug!("Taking AutomaticPlayer turn as an intermediate for game {}", self.game_id);
        // TODO: JD: consider blocking the opponent from winning
        // TODO: JD: finish
        self.take_turn_as_a_beginner(_game_board)
    }

    /// Takes a turn with the skill level of an expert player.
    fn take_turn_as_an_expert(&self, _game_board: GameBoard) -> Option<BoardPosition> {
        debug!("Taking AutomaticPlayer turn as an expert for game {}", self.game_id);
        // TODO: JD: finish
        self.take_turn_as_a_beginner(_game_board)
    }

    /// Takes a turn with the skill level of a master-level player.
    fn take_turn_as_a_master(&self, _game_board: GameBoard) -> Option<BoardPosition> {
        debug!("Taking AutomaticPlayer turn as a master for game {}", self.game_id);
        // TODO: JD: finish
        self.take_turn_as_a_beginner(_game_board)
    }
}

impl<T: GameTrait + Clone + Send + Sync> AutomaticPlayer<T> {
    //

    pub(crate) async fn take_turn(&self, game: &T, session_id: String) {
        //

        info!("Taking AutomaticPlayer turn for Game {}", self.game_id);

        let game_board = game.get_current_game_state().game_board;

        if let Some(new_board_position) = match self.skill_level {
            AutomaticPlayerSkillLevel::Beginner => self.take_turn_as_a_beginner(game_board),
            AutomaticPlayerSkillLevel::Intermediate => self.take_turn_as_an_intermediate(game_board),
            AutomaticPlayerSkillLevel::Expert => self.take_turn_as_an_expert(game_board),
            AutomaticPlayerSkillLevel::Master => self.take_turn_as_a_master(game_board),
        } {
            //

            let game_id = game.get_id();
            let player_id = self.player_info.player_id.clone();

            tokio::spawn(async move {

                // Make the service feel more human by deliberating on the move for some time...
                let deliberation_time_in_secs = (rand::random::<f32>() * MAX_DELIBERATION_TIME_SECS).floor() as usize + MIN_DELIBERATION_TIME_SECS;
                sleep(Duration::from_secs(deliberation_time_in_secs as u64)).await;

                // *** Now, control the service via the API in the same way client apps do. ***

                let url = format!("http://127.0.0.1:50020/v1/games/{}/turns", game_id);
                let client = reqwest::Client::new();

                let game_turn_info = GameTurnParams {
                    destination: new_board_position,
                    player_id,
                    session_id,
                };
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

impl<T: GameTrait + Clone + Send + Sync> AutomaticPlayer<T> {
    //

    /// Determines the empty locations on the specified Game Board.
    #[named]
    pub(crate) fn determine_empty_locations(grid: &GameBoard) -> Option<Vec<BoardPosition>> {
        //

        debug!("{} called", function_name!());

        if grid.is_empty() {
            return None;
        }

        let mut empty_locations: Vec<BoardPosition> = Vec::new();

        // Gather the locations that are marked as Unselected.
        for row in grid.iter().enumerate() {
            for column in row.1.iter().enumerate() {
                let game_piece = column.1;
                if *game_piece == GamePiece::Unselected {
                    empty_locations.push(BoardPosition::new(row.0, column.0));
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
impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionObserverTrait<T> for AutomaticPlayer<T> {
    //

    #[named]
    async fn session_updated(&self, state_change: &GamingSessionStateChanges, session: &GamingSession<T>, game: Option<T>) {
        //

        debug!("{} called", function_name!());

        if let Some(game) = game {
            //

            debug!("AutomaticPlayer: received session_updated() for session {} and game {}", session.session_id, game.get_id());

            match state_change {
                // Is it my turn?
                GamingSessionStateChanges::GameIsReady | GamingSessionStateChanges::GameTurnTaken => {
                    let game_state = game.get_current_game_state();
                    match game_state.play_status {
                        PlayStatus::InProgress => {
                            if let Some(current_player) = game.get_current_player() {
                                if current_player.player_id == self.player_info.player_id {
                                    self.take_turn(&game, session.session_id.clone()).await;
                                }
                            }
                        }
                        PlayStatus::NotStarted => {} // Early return. Nothing to do.
                        _ => {}
                    }
                }
                GamingSessionStateChanges::GameDeleted | GamingSessionStateChanges::GamingSessionDeleted => {}
            }
        }
    }

    /// Returns the unique ID of this instance.
    fn unique_id(&self) -> String {
        self.game_id.clone()
    }
}
