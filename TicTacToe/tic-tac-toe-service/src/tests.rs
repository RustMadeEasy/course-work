#[cfg(test)]
mod auto_player_tests {
    use crate::auto_player::AutomaticPlayer;
    use crate::game_board::{BoardPosition, GamePiece};
    use crate::game_state::GameState;
    use crate::models::PlayerInfo;
    use crate::tic_tac_toe_game::TicTacToeGame;
    use uuid::Uuid;

    #[test]
    fn test_get_empty_locations() {
        //

        //

        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);

        /*
        X  -  X
        -  X  O
        O  O  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();

        // Make sure AutomaticPlayer can detect the empty locations.
        let empty_locations = AutomaticPlayer::<TicTacToeGame>::determine_empty_locations(&board_state.game_board).unwrap();
        assert_eq!(empty_locations[0], BoardPosition::new(0, 1));
        assert_eq!(empty_locations[1], BoardPosition::new(1, 0));

        /*
        O  O  X
        X  O  O
        O  X  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();

        // Make sure AutomaticPlayer can detect that a full board has no empty locations.
        let empty_locations = AutomaticPlayer::<TicTacToeGame>::determine_empty_locations(&board_state.game_board);
        assert!(empty_locations.is_none());
    }
}


#[cfg(test)]
mod game_board_tests {
    use uuid::Uuid;

    use crate::game_board::{BoardPosition, GamePiece};
    use crate::game_state::GameState;
    use crate::models::PlayerInfo;
    use crate::play_status::PlayStatus;

    #[test]
    fn test_in_progress_status() {
        //

        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);

        /*
        O  O  X
        X  O  O
        O  X  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 0), &player_one, &player_two)
            .unwrap();
        // Make sure that the Play Status Game indicates InProgress since the Game has not ended
        assert_eq!(board_state.play_status, PlayStatus::InProgress);
    }

    #[test]
    fn test_invalid_piece_placement() {
        //

        // Invalid column
        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);
        if GameState::new()
            .place_game_piece(&BoardPosition::new(10, 0), &player_one, &player_two)
            .is_ok()
        {
            panic!()
        }

        // Invalid row
        if GameState::new()
            .place_game_piece(&BoardPosition::new(0, 10), &player_one, &player_two)
            .is_ok()
        {
            panic!()
        }

        // Invalid column and invalid row
        if GameState::new()
            .place_game_piece(&BoardPosition::new(30, 30), &player_one, &player_two)
            .is_ok()
        {
            panic!()
        }
    }

    #[test]
    fn test_occupied_piece_placement() {
        //

        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);

        // Place an X at 0:0
        let board_state = GameState::new();
        let new_board_state =
            match board_state.place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            {
                Ok(board_state) => board_state,
                Err(_) => {
                    panic!()
                }
            };

        // Have Player Two attempt to move to the same space (0:0)
        let result =
            new_board_state.place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one);
        if result.is_ok() {
            panic!()
        }
    }

    #[test]
    fn test_stalemate() {
        //

        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);

        /*
        O  O  X
        X  O  O
        O  X  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();
        // Make sure the Game ended in a Stalemate
        assert_eq!(board_state.play_status, PlayStatus::EndedInStalemate);
    }

    #[test]
    fn test_valid_piece_placement() {
        //

        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        match GameState::new().place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
        {
            Ok(board_state) => {
                // Double check that the location now contains the piece we specified
                assert_eq!(board_state.game_board[0][0], GamePiece::O)
            }
            Err(_) => {
                panic!()
            }
        }
    }

    #[test]
    fn test_winning_moves() {
        //

        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);

        /*
        X  -  X
        -  X  O
        O  O  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();
        // Make sure Player One won
        assert_eq!(board_state.play_status, PlayStatus::EndedInWin);
        assert_eq!(board_state.winning_player_id.unwrap(), player_one.player_id);

        /*
        O  O  O
        -  X  X
        X  -  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();

        // Make sure Player Two won
        assert_eq!(board_state.play_status, PlayStatus::EndedInWin);
        assert_eq!(board_state.winning_player_id.unwrap(), player_two.player_id);
    }
}

#[cfg(test)]
mod game_manager_tests {
    use uuid::Uuid;

    use crate::errors::GameError;
    use crate::games_manager::TicTacToeGamesManager;
    use crate::models::requests::{GameMode, JoinSessionParams, NewSinglePlayerGameParams};

    #[tokio::test]
    async fn test_add_second_player() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewSinglePlayerGameParams {
            game_mode: GameMode::TwoPlayers,
            player_one_display_name: display_name.clone(),
            computer_skill_level: None,
        };
        let mut manager = TicTacToeGamesManager::new();
        let game = match manager.create_new_session_and_game(&params).await {
            Ok(game) => game,
            Err(_) => {
                panic!()
            }
        };

        let second_player_params = JoinSessionParams {
            game_invitation_code: game.game_invitation_code,
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player_to_session(&second_player_params).await {
            Ok(new_game_instance) => {
                match new_game_instance.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the Game piece is different from that of Player One
                        assert_ne!(
                            game.players.first().unwrap().game_piece,
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

    #[tokio::test]
    async fn test_add_second_player_twice() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewSinglePlayerGameParams {
            game_mode: GameMode::TwoPlayers,
            player_one_display_name: display_name.clone(),
            computer_skill_level: None,
        };
        let mut manager = TicTacToeGamesManager::new();
        let game = match manager.create_new_session_and_game(&params).await {
            Ok(game) => game,
            Err(_) => {
                panic!()
            }
        };

        let second_player_params = JoinSessionParams {
            game_invitation_code: game.game_invitation_code.clone(),
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player_to_session(&second_player_params).await {
            Ok(new_game_instance) => {
                match new_game_instance.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the Game piece is different from that of Player One
                        assert_ne!(
                            game.players.first().unwrap().game_piece,
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
        let second_player_params = JoinSessionParams {
            game_invitation_code: game.game_invitation_code,
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player_to_session(&second_player_params).await {
            Ok(new_game_instance) => {
                match new_game_instance.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the Game piece is different from that of Player One
                        assert_ne!(
                            game.players.first().unwrap().game_piece,
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

    #[tokio::test]
    async fn test_add_second_player_using_player_one_name() {
        //

        let player_one_display_name = Uuid::new_v4().to_string();
        let params = NewSinglePlayerGameParams {
            game_mode: GameMode::TwoPlayers,
            player_one_display_name: player_one_display_name.clone(),
            computer_skill_level: None,
        };
        let mut manager = TicTacToeGamesManager::new();
        let game = match manager.create_new_session_and_game(&params).await {
            Ok(game) => game,
            Err(_) => {
                panic!()
            }
        };

        let second_player_params = JoinSessionParams {
            game_invitation_code: game.game_invitation_code,
            player_display_name: player_one_display_name,
        };
        match manager.add_player_to_session(&second_player_params).await {
            Ok(new_game_instance) => {
                match new_game_instance.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the Game piece is different from that of Player One
                        assert_ne!(
                            game.players.first().unwrap().game_piece,
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

    #[tokio::test]
    async fn test_add_second_player_with_invalid_invitation_code() {
        //

        let display_name = Uuid::new_v4().to_string();
        let params = NewSinglePlayerGameParams {
            game_mode: GameMode::TwoPlayers,
            player_one_display_name: display_name.clone(),
            computer_skill_level: None,
        };
        let mut manager = TicTacToeGamesManager::new();
        let game = match manager.create_new_session_and_game(&params).await {
            Ok(game) => game,
            Err(_) => {
                panic!()
            }
        };

        let second_player_params = JoinSessionParams {
            game_invitation_code: Uuid::new_v4().to_string(),
            player_display_name: Uuid::new_v4().to_string(),
        };
        match manager.add_player_to_session(&second_player_params).await {
            Ok(new_game_instance) => {
                match new_game_instance.players.last() {
                    None => {
                        panic!()
                    }
                    Some(player_info) => {
                        // Make sure the Game piece is different from that of Player One
                        assert_ne!(
                            game.players.first().unwrap().game_piece,
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
}

#[cfg(test)]
mod game_play_tests {
    use uuid::Uuid;

    use crate::game_board::{BoardPosition, GamePiece};
    use crate::game_trait::GameTrait;
    use crate::models::requests::{GameMode, GameTurnInfo, NewSinglePlayerGameParams};
    use crate::play_status::PlayStatus;
    use crate::tic_tac_toe_game::TicTacToeGame;

    #[test]
    fn test_get_current_board_state() {
        //

        // Start a new Game
        let params = NewSinglePlayerGameParams {
            game_mode: GameMode::TwoPlayers,
            player_one_display_name: "Player One".to_string(),
            computer_skill_level: None,
        };
        let mut game = TicTacToeGame::new(&params, Uuid::new_v4(), "".to_string(), 0).unwrap();
        let player_one_id = game.players.first().unwrap().player_id.clone();

        // Add the Second Player
        match game.add_player("Player Two", false) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        };
        let player_two_id = game.players.last().unwrap().player_id.clone();

        // Let Player One take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 0),
            player_id: player_one_id.clone(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        }

        // Check the board state
        let game_state = game.get_current_game_state();
        assert_eq!(game_state.get_play_status(), PlayStatus::InProgress);
        assert_eq!(game_state.get_id_of_player_who_made_move(), player_one_id);

        // Let Player Two take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 1),
            player_id: player_two_id.clone(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        }

        // Check the board state
        let game_state = game.get_current_game_state();
        assert_eq!(game_state.get_play_status(), PlayStatus::InProgress);
        assert_eq!(game_state.get_id_of_player_who_made_move(), player_two_id);
    }

    #[test]
    fn test_get_play_history() {
        //

        // Start a new Game
        let params = NewSinglePlayerGameParams {
            game_mode: GameMode::TwoPlayers,
            player_one_display_name: "Player One".to_string(),
            computer_skill_level: None,
        };
        let mut game = TicTacToeGame::new(&params, Uuid::new_v4(), "".to_string(), 0).unwrap();
        let player_one_id = game.players.first().unwrap().player_id.clone();

        // Add the Second Player
        match game.add_player("Player Two", false) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        };
        let player_two_id = game.players.last().unwrap().player_id.clone();

        // There should be no moves in the history at this point
        assert_eq!(game.play_history.len(), 0);

        // Let Player One take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 0),
            player_id: player_one_id.clone(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        }

        // There should be exactly one move in the history
        assert_eq!(game.play_history.len(), 1);

        // Let Player Two take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 1),
            player_id: player_two_id.clone(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        }

        // There should be exactly two moves in the history
        assert_eq!(game.play_history.len(), 2);
    }

    #[test]
    fn test_take_turn() {
        //

        // Start a new Game
        let params = NewSinglePlayerGameParams {
            game_mode: GameMode::TwoPlayers,
            player_one_display_name: "Player One".to_string(),
            computer_skill_level: None,
        };
        let mut game = TicTacToeGame::new(&params, Uuid::new_v4(), "".to_string(), 0).unwrap();
        let player_one_id = game.players.first().unwrap().player_id.clone();

        // Add the Second Player
        match game.add_player("Player Two", false) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        };
        let player_two_id = game.players.last().unwrap().player_id.clone();

        let player_one_destination = BoardPosition::new(0, 0);
        let player_two_destination = BoardPosition::new(1, 1);

        // Let Player One take their turn, placing an X at 0:0
        let turn_info = GameTurnInfo {
            destination: player_one_destination.clone(),
            player_id: player_one_id.clone(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        }

        // There should be an X at 0:0
        assert_eq!(
            game.get_current_game_state().get_game_board()[player_one_destination.row]
                [player_one_destination.column],
            GamePiece::X
        );

        // Let Player Two take their turn, placing an O at 1:1
        let turn_info = GameTurnInfo {
            destination: player_two_destination.clone(),
            player_id: player_two_id.clone(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(_) => {
                panic!();
            }
        }

        // There should be an O at 1:1
        assert_eq!(
            game.get_current_game_state().get_game_board()[player_two_destination.row]
                [player_two_destination.column],
            GamePiece::O
        );
    }
}

#[cfg(test)]
mod game_state_tests {
    use crate::game_board::{BoardPosition, GamePiece};
    use crate::game_state::GameState;
    use crate::models::PlayerInfo;
    use uuid::Uuid;

    #[test]
    fn test_binary_representation_for_piece_placement() {
        let player_one = PlayerInfo::new(Uuid::new_v4(), &GamePiece::X, false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), &GamePiece::O, false);

        /*
        X  -  -
        -  -  -
        -  -  -     */
        let board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        let binary_representation = GameState::binary_representation_for_piece_placement(&board_state.game_board, &player_one.game_piece, &player_two.game_piece);
        assert_eq!(binary_representation.0, 0b_100_000_000);

        /*
        X  -  X
        -  X  O
        O  O  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();

        let binary_representation = GameState::binary_representation_for_piece_placement(&board_state.game_board, &player_one.game_piece, &player_two.game_piece);
        assert_eq!(binary_representation.0, 0b_101_010_001);
        assert_eq!(binary_representation.1, 0b_000_001_110);
    }
}
