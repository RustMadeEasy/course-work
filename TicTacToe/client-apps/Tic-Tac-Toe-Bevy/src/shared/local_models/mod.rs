pub(crate) mod local_game_piece;
pub(crate) mod local_game_state;
pub(crate) mod local_grid_position;
pub(crate) mod local_player_info;
pub(crate) mod local_player_status;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author Joel@RustMadeEasy.com

// This module contains the local, Bevy-client-specific renditions of the TicTacToe client SDK's
// local_models. It seems to be redundant. But, we don't want the major body of the app tied
// directly to the SDK's models. These local models and the ServiceClient provide us with the
// requisite abstraction layer.
