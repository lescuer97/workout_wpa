use actix_web::web::Data;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use actix_web::HttpRequest;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo, Postgres},
    FromRow, Pool, Type,
};
use uuid::Uuid;

use crate::{error::AuthError, utils::get_env_variable};

use crate::{
    db::{query_user_with_email, register_user_query},
    error::UserError,
};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct RegisterUserData {
    pub email: String,
    pub password: String,
    pub password_repeat: String,
    pub rol: Option<Vec<UserRole>>,
    pub id: Option<Uuid>,
    pub u2f: Option<bool>,
    pub totp: Option<bool>,
}

impl RegisterUserData {
    fn set_default_user(&mut self) -> Result<(), UserError> {
        let same_password = self.check_same_password();

        if !same_password {
            return Err(UserError::PasswordDontMatch);
        }
        let uuid = Uuid::new_v4();

        self.rol = Some(vec![UserRole::EditSelf]);
        self.id = Some(uuid);
        self.password = self.hash_password()?;
        self.u2f = Some(false);
        self.totp = Some(false);

        Ok(())
    }

    pub async fn register(&mut self, pool: Data<Pool<Postgres>>) -> Result<(), UserError> {
        self.set_default_user()?;

        register_user_query(self.clone(), pool).await?;

        return Ok(());
    }

    fn check_same_password(&self) -> bool {
        self.password == self.password_repeat
    }
    fn hash_password(&self) -> Result<String, UserError> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let password_bytes = self.password.as_bytes();

        let password_hash = match argon2.hash_password(password_bytes, &salt) {
            Ok(hashed_password) => hashed_password.to_string(),
            Err(_) => return Err(UserError::HashingError),
        };

        let parsed_hash = match PasswordHash::new(&password_hash) {
            Ok(hashed_value) => hashed_value,
            Err(_) => return Err(UserError::HashingError),
        };

        Ok(parsed_hash.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginData {
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginDataToSend {
    pub email: String,
    pub password: Option<String>,
    pub u2f: bool,
    pub totp: bool,
    pub id: Uuid,
}

impl LoginData {
    pub async fn login(&self, pool: Data<Pool<Postgres>>) -> Result<LoginDataToSend, UserError> {
        let user_query: LoginDataToSend =
            query_user_with_email(self.email.to_owned(), pool).await?;

        self.check_password_matches_database(user_query.to_owned())?;

        Ok(LoginDataToSend {
            email: user_query.email,
            password: None,
            u2f: user_query.u2f,
            totp: user_query.totp,
            id: user_query.id,
        })
    }
    fn check_password_matches_database(
        &self,
        user_query: LoginDataToSend,
    ) -> Result<(), UserError> {
        let password_hash_string = user_query.password;

        if let Some(password_hash_string) = password_hash_string {
            let parsed_hash = match PasswordHash::new(password_hash_string.as_str()) {
                Ok(hashed_value) => hashed_value,
                Err(_) => return Err(UserError::HashingError),
            };

            if let Some(password) = self.password.clone() {
                let password_match = Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok();

                if password_match {
                    Ok(())
                } else {
                    return Err(UserError::PasswordDontMatch);
                }
            } else {
                return Err(UserError::UnexpectedError);
            }
        } else {
            return Err(UserError::UnexpectedError);
        }
    }
}

pub const AUTHENTIFIED_COOKIE: &str = "auth";
pub const COOKIE_FOR_TOTP_AUTH: &str = "auth-2fa";
pub const SECRET_FOR_TOTP_AUTH: &str = "totp-secret";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct JWTToken {
    pub sub: String,
    pub exp: usize,
}
impl JWTToken {
    pub fn create_jwt_token(value: String, duration: DateTime<Utc>) -> Result<String, AuthError> {
        let secret_jwt = get_env_variable("SECRET_FOR_JWT");
        let token_setup = JWTToken {
            sub: value.to_owned(),
            exp: duration.timestamp() as usize,
        };

        let token: String = encode(
            &Header::default(),
            &token_setup,
            &EncodingKey::from_secret(secret_jwt.as_ref()),
        )?;
        Ok(token)
    }
    pub fn validate_jwt_token_from_cookie(
        request: HttpRequest,
        name_of_token: &str,
    ) -> Result<TokenData<JWTToken>, AuthError> {
        let auth_token = match request.cookie(name_of_token) {
            Some(token) => token.value().to_string(),
            None => return Err(AuthError::NoJWTToken),
        };

        let secret_jwt = get_env_variable("SECRET_FOR_JWT");

        let contents = match decode::<JWTToken>(
            auth_token.as_str(),
            &DecodingKey::from_secret(secret_jwt.as_ref()),
            &Validation::default(),
        ) {
            Ok(token) => token,
            Err(_) => return Err(AuthError::InvalidToken),
        };

        Ok(contents)
    }
    pub fn logout_jwt_token() -> Result<String, AuthError> {
        let expiration_time = Utc::now() - Duration::seconds(1);

        let token_setup = JWTToken {
            sub: "".to_owned(),
            exp: expiration_time.timestamp() as usize,
        };

        let secret_jwt = get_env_variable("SECRET_FOR_JWT");

        let token: String = encode(
            &Header::default(),
            &token_setup,
            &EncodingKey::from_secret(secret_jwt.as_ref()),
        )?;

        Ok(token)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    EditSelf,
    EditOther,
    RemoveOther,
    WatchOther,
    SuperAdmin,
}

impl PgHasArrayType for UserRole {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_user_role")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        auth::{JWTToken, AUTHENTIFIED_COOKIE},
        utils::create_jwt_and_cookie,
    };
    use actix_web::test::TestRequest;
    use chrono::{DateTime, Duration, Utc};

    use dotenv;
    #[test]
    fn create_jwt_token_success() {
        dotenv::dotenv().ok();
        let email: String = String::from("test@test.com");
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::days(30);

        let token = JWTToken::create_jwt_token(email, expiration_time).unwrap();

        assert_eq!(token.is_empty(), false);
    }

    #[actix_web::test]
    async fn validate_jwt_token_success() {
        dotenv::dotenv().ok();

        let email: String = String::from("test@test.com");
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::days(30);

        let jwt_cookie =
            create_jwt_and_cookie(email.to_owned(), AUTHENTIFIED_COOKIE, expiration_time).unwrap();

        let req = TestRequest::default().cookie(jwt_cookie).to_http_request();

        let token = JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE).unwrap();

        assert_eq!(true, !token.claims.sub.is_empty());
    }
    #[actix_web::test]
    async fn invalid_jwt_token() {
        dotenv::dotenv().ok();

        let email: String = String::from("test@test.com");
        let expiration_time: DateTime<Utc> = Utc::now() - Duration::days(30);

        let jwt_cookie =
            create_jwt_and_cookie(email.to_owned(), AUTHENTIFIED_COOKIE, expiration_time).unwrap();

        let req = TestRequest::default().cookie(jwt_cookie).to_http_request();

        let token = JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE);
        assert_eq!(true, token.is_err());
    }
}
