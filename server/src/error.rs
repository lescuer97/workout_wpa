use actix_web::{HttpResponse, ResponseError};

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
}
/// Actix Web uses `ResponseError` for conversion of errors to a response
impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::PasswordDontMatch => {
                // println!("do some stuff related to CustomOne error");
                HttpResponse::Forbidden().finish()
            }

            UserError::DBError(_) => {
                println!("do some stuff related to CustomTwo error");
                HttpResponse::UnprocessableEntity().finish()
            }

            UserError::UnexpectedError => {
                println!("do some stuff related to CustomThree error");
                HttpResponse::InternalServerError().finish()
            }
            UserError::HashingError => {
                println!("do some stuff related to CustomThree error");
                HttpResponse::InternalServerError().finish()
            }

            _ => {
                println!("do some stuff related to CustomFour error");
                HttpResponse::BadRequest().finish()
            }
        }
    }
}
