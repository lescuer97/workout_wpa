use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{auth::AUTHENTIFIED_COOKIE, utils::make_removal_cookie};

const SUCCESS: &str = "success";
const FAILURE: &str = "failure";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResponseBodyMessage<T> {
    pub status: String,
    pub data: T,
}
impl<T: Serialize> ResponseBodyMessage<T> {
    pub fn success_message(data: T) -> Self {
        Self {
            status: SUCCESS.to_string(),
            data,
        }
    }
    pub fn fail_message(data: T) -> Self {
        Self {
            status: FAILURE.to_string(),
            data,
        }
    }
    pub fn send_response(&self, code: StatusCode) -> HttpResponse {
        let removal_cookie = match make_removal_cookie(AUTHENTIFIED_COOKIE) {
            Ok(cookie) => cookie,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };
        match code {
            StatusCode::OK => HttpResponse::Ok().json(self),
            StatusCode::CREATED => HttpResponse::Created().json(self),
            StatusCode::BAD_REQUEST => HttpResponse::BadRequest().json(self),
            StatusCode::FORBIDDEN => HttpResponse::Forbidden().cookie(removal_cookie).json(self),
            StatusCode::NOT_FOUND => HttpResponse::NotFound().json(self),
            StatusCode::INTERNAL_SERVER_ERROR => HttpResponse::InternalServerError().json(self),
            _ => HttpResponse::Ok().json(self),
        }
    }
}
