// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

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

        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        /*
        X  -  X
        -  X  O
        O  O  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();

        // Make sure AutomaticPlayer can detect the empty locations.
        let empty_locations = AutomaticPlayer::<TicTacToeGame>::determine_empty_locations(&board_state.new_game_state.game_board).unwrap();
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
            .new_game_state.place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();

        // Make sure AutomaticPlayer can detect that a full board has no empty locations.
        let empty_locations = AutomaticPlayer::<TicTacToeGame>::determine_empty_locations(&board_state.new_game_state.game_board);
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

        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        /*
        O  O  X
        X  O  O
        O  X  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 0), &player_one, &player_two)
            .unwrap();
        // Make sure that the Play Status Game indicates InProgress since the Game has not ended
        assert_eq!(board_state.new_game_state.play_status, PlayStatus::InProgress);
    }

    #[test]
    fn test_invalid_piece_placement() {
        //

        // Invalid column
        let player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let player_two = PlayerInfo::new(Uuid::new_v4(), false);
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

        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        // Place an X at 0:0
        let board_state = GameState::new();
        let new_board_state =
            match board_state.place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            {
                Ok(board_state) => board_state,
                Err(error) => {
                    panic!("{:?}", error);
                }
            };

        // Have Player Two attempt to move to the same space (0:0)
        let result =
            new_board_state.new_game_state.place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one);
        if result.is_ok() {
            panic!()
        }
    }

    #[test]
    fn test_stalemate() {
        //

        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        /*
        O  O  X
        X  O  O
        O  X  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();
        // Make sure the Game ended in a Stalemate
        assert_eq!(board_state.new_game_state.play_status, PlayStatus::EndedInStalemate);
    }

    #[test]
    fn test_valid_piece_placement() {
        //

        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        match GameState::new().place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
        {
            Ok(board_state) => {
                // Double check that the location now contains the piece we specified
                assert_eq!(board_state.new_game_state.game_board[0][0], GamePiece::O)
            }
            Err(error) => {
                panic!("{:?}", error);
            }
        }
    }

    #[test]
    fn test_winning_moves() {
        //

        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);

        player_one.game_piece = GamePiece::X;
        player_two.game_piece = GamePiece::O;

        /*
        X  -  X
        -  X  O
        O  O  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();
        // Make sure Player One won
        assert_eq!(board_state.new_game_state.play_status, PlayStatus::EndedInWin);
        assert_eq!(board_state.winning_player.unwrap().player_id, player_one.player_id);

        /*
        O  O  O
        -  X  X
        X  -  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 2), &player_two, &player_one)
            .unwrap();

        // Make sure Player Two won
        assert_eq!(board_state.new_game_state.play_status, PlayStatus::EndedInWin);
        assert_eq!(board_state.winning_player.unwrap().player_id, player_two.player_id);
    }
}

#[cfg(test)]
mod game_play_tests {
    use uuid::Uuid;

    use crate::game_board::{BoardPosition, GamePiece};
    use crate::game_trait::GameTrait;
    use crate::models::requests::GameTurnInfo;
    use crate::models::{GameMode, PlayerInfo};
    use crate::play_status::PlayStatus;
    use crate::tic_tac_toe_game::TicTacToeGame;

    #[test]
    fn test_get_current_board_state() {
        //

        // Start a new Game
        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        let mut game = TicTacToeGame::new(GameMode::TwoPlayers, &player_one, &player_two, Uuid::new_v4().to_string().as_str()).unwrap();

        game.current_player = Some(player_one.clone());

        // Let Player One take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 0),
            player_id: player_one.player_id.clone(),
            session_id: "".to_string(),
        };
        let _ = game.take_turn(&turn_info);

        // Check the board state
        let game_state = game.get_current_game_state();
        assert_eq!(game_state.get_play_status(), PlayStatus::InProgress);
        assert_eq!(game_state.get_id_of_player_who_made_move(), player_one.player_id);

        // Let Player Two take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 1),
            player_id: player_two.player_id.clone(),
            session_id: "".to_string(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(error) => {
                panic!("{:?}", error);
            }
        }

        // Check the board state
        let game_state = game.get_current_game_state();
        assert_eq!(game_state.get_play_status(), PlayStatus::InProgress);
        assert_eq!(game_state.get_id_of_player_who_made_move(), player_two.player_id);
    }

    #[test]
    fn test_get_play_history() {
        //

        // Start a new Game
        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        let mut game = TicTacToeGame::new(GameMode::TwoPlayers, &player_one, &player_two, Uuid::new_v4().to_string().as_str()).unwrap();

        game.current_player = Some(player_one.clone());

        // There should be no moves in the history at this point
        assert_eq!(game.play_history.len(), 0);

        // Let Player One take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 0),
            player_id: player_one.player_id.clone(),
            session_id: "".to_string(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(error) => {
                panic!("{:?}", error);
            }
        }

        // There should be exactly one move in the history
        assert_eq!(game.play_history.len(), 1);

        // Let Player Two take their turn
        let turn_info = GameTurnInfo {
            destination: BoardPosition::new(0, 1),
            player_id: player_two.player_id.clone(),
            session_id: "".to_string(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(error) => {
                panic!("{:?}", error);
            }
        }

        // There should be exactly two moves in the history
        assert_eq!(game.play_history.len(), 2);
    }

    #[test]
    fn test_take_turn() {
        //

        // Start a new Game
        let mut game = TicTacToeGame::new(GameMode::TwoPlayers,
                                          &PlayerInfo::new(Uuid::new_v4(), false),
                                          &PlayerInfo::new(Uuid::new_v4(), false),
                                          Uuid::new_v4().to_string().as_str()).unwrap();

        let first_destination = BoardPosition::new(0, 0);
        let second_destination = BoardPosition::new(1, 1);

        let first_player_game_piece = game.current_player.clone().unwrap().game_piece;

        // Let Player One take their turn
        let turn_info = GameTurnInfo {
            destination: first_destination.clone(),
            player_id: game.current_player.clone().unwrap().player_id,
            session_id: "".to_string(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(error) => {
                panic!("{:?}", error);
            }
        }

        // Verify that position 0:0 contains the current Player's game piece.
        assert_eq!(
            game.get_current_game_state().get_game_board()[first_destination.row][first_destination.column],
            first_player_game_piece
        );

        let second_player_game_piece = game.current_player.clone().unwrap().game_piece;

        // Let Player Two take their turn.
        let turn_info = GameTurnInfo {
            destination: second_destination.clone(),
            player_id: game.current_player.clone().unwrap().player_id,
            session_id: "".to_string(),
        };
        match game.take_turn(&turn_info) {
            Ok(_) => {}
            Err(error) => {
                panic!("{:?}", error);
            }
        }

        // Verify that position 1:1 contains Player One's game piece.
        assert_eq!(
            game.get_current_game_state().get_game_board()[second_destination.row][second_destination.column],
            second_player_game_piece
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
        //

        let mut player_one = PlayerInfo::new(Uuid::new_v4(), false);
        let mut player_two = PlayerInfo::new(Uuid::new_v4(), false);
        player_one.game_piece = GamePiece::O;
        player_two.game_piece = GamePiece::X;

        /*
        X  -  -
        -  -  -
        -  -  -     */
        let board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        let binary_representation = GameState::binary_representation_for_piece_placement(&board_state.new_game_state.game_board, &player_one.game_piece, &player_two.game_piece);
        assert_eq!(binary_representation.0, 0b_100_000_000);

        /*
        X  -  X
        -  X  O
        O  O  X     */
        let mut board_state = GameState::new()
            .place_game_piece(&BoardPosition::new(0, 0), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(0, 2), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 1), &player_one, &player_two)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(1, 2), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 0), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 1), &player_two, &player_one)
            .unwrap();
        board_state = board_state
            .new_game_state.place_game_piece(&BoardPosition::new(2, 2), &player_one, &player_two)
            .unwrap();

        let binary_representation = GameState::binary_representation_for_piece_placement(&board_state.new_game_state.game_board, &player_one.game_piece, &player_two.game_piece);
        assert_eq!(binary_representation.0, 0b_101_010_001);
        assert_eq!(binary_representation.1, 0b_000_001_110);
    }
}
