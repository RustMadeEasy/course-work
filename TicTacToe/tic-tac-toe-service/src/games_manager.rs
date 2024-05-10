use std::collections::HashMap;
use std::marker::PhantomData;

use log::warn;
use rand::Rng;
use serde::Serialize;

use crate::errors::GameError;
use crate::game_engine::GameEngine;
use crate::game_state::GameState;
use crate::game_trait::GameTrait;
use crate::models::requests::{AddPlayerParams, GameTurnInfo, NewGameParams};

pub(crate) type TicTacToeGamesManager = GamesManager<GameEngine>;

/**
 * Manages all the Game instances played on this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Joel@RustMadeEasy.com
 */

/// Manages all the Game instances played on this service. NOTE: this is sample code.
/// NOTE: Production-grade code would persist the gaming info to a database so that multiple
/// instances of the service can be utilized.
#[derive(Clone, Serialize)]
pub(crate) struct GamesManager<G: GameTrait + Clone> {
    games: HashMap<String, G>,
    #[serde(skip)]
    _outside_instantiation_preventor: PhantomData<u8>,
}

impl<GenericGame: GameTrait + Clone> GamesManager<GenericGame> {
    //

    /// Adds a Player to the Game.
    pub(crate) fn add_player(
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
        self.games
            .insert(game_engine.get_id().clone(), game_engine.clone());

        Ok(game_engine)
    }

    /// Creates a new Game Engine.
    pub(crate) fn create_game_engine(
        &mut self,
        params: &NewGameParams,
    ) -> Result<GenericGame, GameError> {
        //

        let invitation_code = self.generate_invitation_code();
        let game_engine = GenericGame::new(params, invitation_code)?;

        self.games
            .insert(game_engine.get_id().clone(), game_engine.clone());

        Ok(game_engine.clone())
    }

    /// Closes down the specified game and returns its final game state.
    pub(crate) fn end_game(&mut self, game_id: &String) -> Result<(), GameError> {
        //

        let game_engine = self.get_game_engine(game_id)?;

        self.games.remove(&game_engine.get_id());

        Ok(())
    }

    /// Creates a unique, 6-digit code for use as a Game Invitation. We use a 6-digit Game
    /// Invitation instead of handshaking game setup with a Game ID for 2 reasons:
    ///     1) We don't want to expose the Game ID to clients that are not party to the Game.
    ///     2) A 6-digit code is practical for end-users to utilize.
    fn generate_invitation_code(&self) -> String {
        // Place limit to prevent endless loop
        for _ in 0..=1000 {
            let mut rng = rand::thread_rng();
            let game_invitation_code: String = rng.gen_range(100_000..999_999).to_string();
            // Ensure uniqueness across all open Games
            if self
                .get_game_by_invitation_code(&game_invitation_code)
                .is_none()
            {
                return game_invitation_code;
            }
        }

        // It will be next to impossible to get here. However, we have to cover all cases.
        warn!("Could not create unique game invitation code!");
        "".to_string()
    }

    fn get_game_by_invitation_code(&self, invitation_code: &String) -> Option<GenericGame> {
        self.games
            .iter()
            .find(|it| it.1.get_invitation_code() == *invitation_code)
            .map(|it| it.1.clone())
    }

    /// Retrieves the specified Game Engine.
    pub(crate) fn get_game_engine(
        &self,
        game_id: impl Into<String>,
    ) -> Result<GenericGame, GameError> {
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
        Self {
            games: Default::default(),
            _outside_instantiation_preventor: Default::default(),
        }
    }

    /// Make a game move for the specified Player.
    pub(crate) fn take_turn(
        &mut self,
        game_id: &String,
        game_turn_info: &GameTurnInfo,
    ) -> Result<GameState, GameError> {
        //

        let mut game_engine = self.get_game_engine(game_id)?;
        let new_game_state = game_engine.take_turn(game_turn_info)?;

        // Update our game engine
        self.games
            .insert(game_engine.get_id().clone(), game_engine.clone());

        Ok(new_game_state)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::errors::GameError;
    use crate::game_board::GamePiece;
    use crate::game_trait::GameTrait;
    use crate::games_manager::TicTacToeGamesManager;
    use crate::models::requests::{AddPlayerParams, NewGameParams};
    use crate::models::responses::GameInfo;

    #[test]
    fn test_create_new_game() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewGameParams {
            player_one_display_name: display_name.clone(),
        };
        let mut manager = TicTacToeGamesManager::new();

        match manager.create_game_engine(&params) {
            Ok(game_engine) => {
                assert_eq!(
                    game_engine.players.first().unwrap().display_name,
                    display_name
                );
                assert_eq!(game_engine.players.len(), 1);
                assert!(!game_engine.id.is_empty());
                assert_eq!(
                    game_engine.get_current_game_state().get_game_board()[0][0],
                    GamePiece::None
                );
            }
            Err(_) => {
                panic!()
            }
        }
    }

    #[test]
    fn test_ending_game() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewGameParams {
            player_one_display_name: display_name.clone(),
        };
        let mut manager = TicTacToeGamesManager::new();

        match manager.create_game_engine(&params) {
            Ok(game_state) => {
                // End the game and make sure it is no longer accessible
                let _ = manager.end_game(&game_state.id);
                if manager.get_game_engine(&game_state.id).is_ok() {
                    panic!()
                }
            }
            Err(_) => {
                panic!()
            }
        }
    }

    #[test]
    fn test_add_second_player() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewGameParams {
            player_one_display_name: display_name.clone(),
        };
        let mut manager = TicTacToeGamesManager::new();
        let game_engine = match manager.create_game_engine(&params) {
            Ok(game_engine) => game_engine,
            Err(_) => {
                panic!()
            }
        };

        let game_info = GameInfo::try_from(game_engine.clone()).unwrap();

        let second_player_params = AddPlayerParams {
            game_invitation_code: game_info.game_invitation_code,
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player(&second_player_params) {
            Ok(new_game_engine) => {
                match new_game_engine.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the game piece is different from that of Player One
                        assert_ne!(
                            game_engine.players.first().unwrap().game_piece,
                            player_info.game_piece
                        );
                    }
                }
            }
            Err(_) => {
                panic!()
            }
        }
    }

    #[test]
    fn test_add_second_player_with_invalid_invitation_code() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewGameParams {
            player_one_display_name: display_name.clone(),
        };
        let mut manager = TicTacToeGamesManager::new();
        let game_engine = match manager.create_game_engine(&params) {
            Ok(game_engine) => game_engine,
            Err(_) => {
                panic!()
            }
        };

        let second_player_params = AddPlayerParams {
            game_invitation_code: Uuid::new_v4().to_string(),
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player(&second_player_params) {
            Ok(new_game_engine) => {
                match new_game_engine.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the game piece is different from that of Player One
                        assert_ne!(
                            game_engine.players.first().unwrap().game_piece,
                            player_info.game_piece
                        );
                    }
                }
            }
            Err(error) => {
                assert_eq!(error, GameError::InvitationCodeNotFound)
            }
        }
    }

    #[test]
    fn test_add_second_player_twice() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewGameParams {
            player_one_display_name: display_name.clone(),
        };
        let mut manager = TicTacToeGamesManager::new();
        let game_engine = match manager.create_game_engine(&params) {
            Ok(game_engine) => game_engine,
            Err(_) => {
                panic!()
            }
        };

        let game_info = GameInfo::try_from(game_engine.clone()).unwrap();

        let second_player_params = AddPlayerParams {
            game_invitation_code: game_info.game_invitation_code.clone(),
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player(&second_player_params) {
            Ok(new_game_engine) => {
                match new_game_engine.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the game piece is different from that of Player One
                        assert_ne!(
                            game_engine.players.first().unwrap().game_piece,
                            player_info.game_piece
                        );
                    }
                }
            }
            Err(_) => {
                panic!()
            }
        }

        // This attempt should fail
        let second_player_params = AddPlayerParams {
            game_invitation_code: game_info.game_invitation_code,
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player(&second_player_params) {
            Ok(new_game_engine) => {
                match new_game_engine.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the game piece is different from that of Player One
                        assert_ne!(
                            game_engine.players.first().unwrap().game_piece,
                            player_info.game_piece
                        );
                    }
                }
            }
            Err(error) => {
                assert_eq!(error, GameError::MaximumPlayersAlreadyAdded)
            }
        }
    }

    #[test]
    fn test_add_second_player_using_player_one_name() {
        //

        let player_one_display_name = Uuid::new_v4().to_string();
        let params = NewGameParams {
            player_one_display_name: player_one_display_name.clone(),
        };
        let mut manager = TicTacToeGamesManager::new();
        let game_engine = match manager.create_game_engine(&params) {
            Ok(game_engine) => game_engine,
            Err(_) => {
                panic!()
            }
        };

        let game_info = GameInfo::try_from(game_engine.clone()).unwrap();

        let second_player_params = AddPlayerParams {
            game_invitation_code: game_info.game_invitation_code,
            player_display_name: player_one_display_name,
        };
        match manager.add_player(&second_player_params) {
            Ok(new_game_engine) => {
                match new_game_engine.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the game piece is different from that of Player One
                        assert_ne!(
                            game_engine.players.first().unwrap().game_piece,
                            player_info.game_piece
                        );
                    }
                }
            }
            Err(error) => {
                assert_eq!(error, GameError::DisplayNameAlreadyInUseInGame)
            }
        }
    }
}
