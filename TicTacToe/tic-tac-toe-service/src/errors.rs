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

/**
 * Defines the errors used throughout the service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

/// Defines the errors used throughout the service.
#[derive(Debug, Display, Error, PartialEq)]
pub enum GameError {
    BoardLocationAlreadyOccupied,
    GameHasAlreadyEnded,
    GameNotStarted,
    GameNotFound,
    InvalidBoardPosition,
    InvitationCodeNotFound,
    InvalidSession,
    PlayerNotFound,
    SessionHasTooFewPlayers,
    SessionNotFound,
    WrongPlayerTakingTurn,
}

impl ResponseError for GameError {
    //

    /// Converts each error variant to an HTTP status code.
    fn status_code(&self) -> StatusCode {
        match *self {
            GameError::SessionHasTooFewPlayers |
            GameError::GameNotStarted |
            GameError::InvalidBoardPosition |
            GameError::InvalidSession => {
                StatusCode::BAD_REQUEST
            }

            GameError::GameHasAlreadyEnded => StatusCode::NOT_ACCEPTABLE,

            GameError::WrongPlayerTakingTurn => StatusCode::METHOD_NOT_ALLOWED,

            GameError::BoardLocationAlreadyOccupied => {
                StatusCode::CONFLICT
            }

            GameError::GameNotFound
            | GameError::InvitationCodeNotFound
            | GameError::PlayerNotFound => StatusCode::NOT_FOUND,
            GameError::SessionNotFound => StatusCode::NOT_FOUND,
        }
    }

    /// Converts a GameError instance to an HttpResponse instance.
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
