use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use crate::server_messages::ResponseBodyMessage;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Password Doesn't match")]
    PasswordDontMatch,
    #[error("{0}")]
    DBError(#[from] sqlx::Error),
    #[error("Error wasn't expected")]
    UnexpectedError,
    #[error("There was an error hashing the password")]
    HashingError,

    #[error("{0}")]
    SerdeQsError(#[from] serde_qs::Error),
}

/// Actix Web uses `ResponseError` for conversion of errors to a response
impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::PasswordDontMatch => {
                tracing::error!("Password doesn't match");
                return ResponseBodyMessage::fail_message("Password doesn't match")
                    .send_response(StatusCode::FORBIDDEN);
            }

            UserError::DBError(error) => {
                let error_db = error.as_database_error();
                if let Some(err) = error_db {
                    tracing::error!("Database Error: {}", err);
                }
                return ResponseBodyMessage::fail_message("Error with the database")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }

            UserError::UnexpectedError => {
                tracing::error!("Unexpected error");
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            UserError::HashingError => {
                tracing::error!("something happened while hashing");
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            UserError::SerdeQsError(err) => {
                tracing::error!("Serde had some error: {}", err);
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}
use actix_web::cookie::time;

#[derive(Debug, thiserror::Error, Clone)]
pub enum AuthError {
    #[error("No JWT Error")]
    NoJWTToken,
    #[error("The token is not valid")]
    InvalidToken,
    #[error("Unexpected error has ocurred")]
    UnexpectedError,
    #[error("Unexpected error has ocurred")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    TotpError(#[from] libreauth::oath::Error),
    #[error("Time Error: {0}")]
    ComponentRange(#[from] time::error::ComponentRange),
}
impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::NoJWTToken => {
                tracing::error!("No JWT Token");
                return ResponseBodyMessage::fail_message("you are not logged in")
                    .send_response(StatusCode::FORBIDDEN);
            }
            AuthError::InvalidToken => {
                tracing::error!("Invalid Token");
                return ResponseBodyMessage::fail_message("Please log in again")
                    .send_response(StatusCode::FORBIDDEN);
            }
            AuthError::UnexpectedError => {
                tracing::error!("Something unexpected happened");
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            AuthError::JsonWebTokenError(error) => {
                tracing::error!("something happened while parsing jwt: {}", error);
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::FORBIDDEN);
            }
            AuthError::TotpError(err) => {
                tracing::error!("totp error: {}", err);
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            AuthError::ComponentRange(err) => {
                tracing::error!("{}", err);
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}
