use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

/**
 * Defines the errors used throughout the service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Joel@RustMadeEasy.com
 */

/// Defines the errors used throughout the service.
#[derive(Debug, Display, Error, PartialEq)]
pub enum GameError {
    BoardLocationAlreadyOccupied,
    DisplayNameAlreadyInUseInGame,
    GameHasAlreadyEnded,
    GameNotFound,
    InvalidBoardPosition,
    InvitationCodeNotFound,
    MaximumPlayersAlreadyAdded,
    PlayerNotFound,
    WrongPlayerTakingTurn,
}

impl ResponseError for GameError {
    //

    /// Converts each error variant to an HTTP status code.
    fn status_code(&self) -> StatusCode {
        match *self {
            GameError::InvalidBoardPosition | GameError::MaximumPlayersAlreadyAdded => {
                StatusCode::BAD_REQUEST
            }

            GameError::GameHasAlreadyEnded => StatusCode::NOT_ACCEPTABLE,

            GameError::WrongPlayerTakingTurn => StatusCode::METHOD_NOT_ALLOWED,

            GameError::BoardLocationAlreadyOccupied | GameError::DisplayNameAlreadyInUseInGame => {
                StatusCode::CONFLICT
            }

            GameError::GameNotFound
            | GameError::InvitationCodeNotFound
            | GameError::PlayerNotFound => StatusCode::NOT_FOUND,
        }
    }

    /// Converts a GameError instance to an HttpResponse instance.
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
