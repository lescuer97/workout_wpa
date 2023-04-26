use actix_web::cookie::time::OffsetDateTime;
use actix_web::cookie::Cookie;
use actix_web::cookie::{self, Expiration};
use actix_web::web;
use chrono::{DateTime, Utc};
use data_encoding::BASE32;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::env;

pub const ENVIROMENT: &str = "ENVIROMENT";

pub fn get_env_variable(name: &str) -> String {
    match env::var(name) {
        Ok(variable) => variable,
        Err(_) => {
            println!(
                "there is no enviroment variable set with the name {} variable set",
                name
            );
            panic!(
                "there is no enviroment variable set with the name {} variable set",
                name
            )
        }
    }
}

fn generate_cookie(
    name: &str,
    value: String,
    expiration_time: DateTime<Utc>,
) -> Result<Cookie, AuthError> {
    let enviroment = get_env_variable(ENVIROMENT);
    let formated_offset = OffsetDateTime::from_unix_timestamp(expiration_time.timestamp())?;
    let formated_expiration = Expiration::from(formated_offset);
    let cookie: Cookie = if enviroment == "development" {
        cookie::Cookie::build(name, value)
            .path("/")
            .secure(true)
            .http_only(true)
            .expires(formated_expiration)
            .same_site(cookie::SameSite::Strict)
            .finish()
    } else {
        cookie::Cookie::build(name, value)
            .path("/")
            .domain(".leito.dev")
            .secure(true)
            .http_only(true)
            .expires(formated_expiration)
            .same_site(cookie::SameSite::Strict)
            .finish()
    };

    Ok(cookie)
}

pub fn create_secret_from_entropy() -> String {
    let mut buf = [0; 64];
    // Get entropy from the browser
    let mut rng = ChaCha20Rng::from_entropy();
    // use it to fill a buffer and create a key for the app
    rng.fill_bytes(&mut buf);

    let secret = BASE32.encode(&buf);
    secret
}

pub fn validate_totp_token(secret: String, token: web::Json<String>) -> Result<bool, AuthError> {
    match libreauth::oath::TOTPBuilder::new()
        .base32_key(&secret)
        .finalize()
    {
        Ok(totp) => return Ok(totp.is_valid(token.as_str())),
        Err(err) => return Err(err.into()),
    };
}

use crate::auth::JWTToken;
use crate::error::AuthError;

pub fn create_jwt_and_cookie(
    value: String,
    cookie_name: &str,
    expiration_time: DateTime<Utc>,
) -> Result<Cookie, AuthError> {
    let jwt: String = JWTToken::create_jwt_token(value, expiration_time)?;

    let cookie: Cookie = generate_cookie(cookie_name, jwt, expiration_time)?;

    Ok(cookie)
}

pub fn make_removal_cookie(cookie_name: &str) -> Result<Cookie, AuthError> {
    let expiration_time: DateTime<Utc> = Utc::now();

    let mut jwt_cookie = generate_cookie(cookie_name, "".to_string(), expiration_time)?;

    jwt_cookie.make_removal();
    Ok(jwt_cookie)
}
