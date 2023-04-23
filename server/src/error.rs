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
                // println!("do some stuff related to CustomOne error");
                return ResponseBodyMessage::fail_message("Password doesn't match")
                    .send_response(StatusCode::FORBIDDEN);
            }

            UserError::DBError(error) => {
                let error_db = error.as_database_error();
                if let Some(err) = error_db {
                    println!("Error message: {:?}", err.message());
                    println!("Error Code: {:?}", err.code());
                }
                // println!(" Error {:?}", error.as_database_error());
                // match error {
                //     sqlx::Error::RowNotFound => {
                //         return HttpResponse::NotFound().finish();
                //     }
                //     _ => {
                //         return HttpResponse::InternalServerError().finish();
                //     }
                // }
                return ResponseBodyMessage::fail_message("Error with the database")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }

            UserError::UnexpectedError => {
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            UserError::HashingError => {
                println!("do some stuff related to CustomThree error");
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            UserError::SerdeQsError(_) => {
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}

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
    #[error("Error from totp authentification")]
    TotpError,
}
impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::NoJWTToken => {
                return ResponseBodyMessage::fail_message("you are not logged in")
                    .send_response(StatusCode::FORBIDDEN);
            }
            AuthError::InvalidToken => {
                return ResponseBodyMessage::fail_message("Please log in again")
                    .send_response(StatusCode::FORBIDDEN);
            }
            AuthError::UnexpectedError => {
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            AuthError::JsonWebTokenError(_) => {
                // println!("do some stuff related to CustomOne error");
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
            AuthError::TotpError => {
                // println!("do some stuff related to CustomOne error");
                return ResponseBodyMessage::fail_message("Unexpected error")
                    .send_response(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
}
