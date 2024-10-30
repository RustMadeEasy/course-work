// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

/// Defines the errors used throughout the service.
#[derive(Debug, Display, Error, PartialEq)]
pub(crate) enum GameError {
    /// The specified board location is already occupied by another Game Piece
    BoardLocationAlreadyOccupied,
    /// The Game already has the maximum number of Players
    GameHasMaximumNumberOfPlayers,
    /// The operation cannot be applied because the Game is an ended state
    GameHasAlreadyEnded,
    /// The operation cannot be applied because the Game has not begun
    GameNotStarted,
    /// The specified Game does not exist
    GameNotFound,
    /// The specified Gaming Session does not exist
    GamingSessionNotFound,
    /// The specified board position is not a valid position on a Tic-Tac-Toe game board
    InvalidBoardPosition,
    /// The specified Invitation Code does not exist 
    InvitationCodeNotFound,
    /// The Player's Game Piece has not been selected as yet
    PlayerGamePieceNotSelected,
    /// The specified Player does not exist in the Gaming Session
    PlayerNotFound,
    /// The wrong Player is being specified to take a turn 
    WrongPlayerTakingTurn,
}

impl ResponseError for GameError {
    //

    /// Converts each error variant into an HTTP status code.
    fn status_code(&self) -> StatusCode {
        match *self {
            GameError::GameNotStarted
            | GameError::InvalidBoardPosition
            | GameError::PlayerGamePieceNotSelected => StatusCode::BAD_REQUEST,

            GameError::GameHasAlreadyEnded => StatusCode::NOT_ACCEPTABLE,

            GameError::GameHasMaximumNumberOfPlayers
            | GameError::WrongPlayerTakingTurn => StatusCode::METHOD_NOT_ALLOWED,

            GameError::BoardLocationAlreadyOccupied => StatusCode::CONFLICT,

            GameError::GameNotFound
            | GameError::InvitationCodeNotFound
            | GameError::PlayerNotFound
            | GameError::GamingSessionNotFound => StatusCode::NOT_FOUND,
        }
    }

    /// Converts a GameError instance to an HttpResponse instance.
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
