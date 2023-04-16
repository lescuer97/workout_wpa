use actix_web::{web::Data, Error};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo, Postgres},
    FromRow, Pool, Type,
};
use uuid::Uuid;

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
        let password_hash_string = user_query.password.unwrap();

        let parsed_hash = match PasswordHash::new(password_hash_string.as_str()) {
            Ok(hashed_value) => hashed_value,
            Err(_) => return Err(UserError::HashingError),
        };

        let password_match = Argon2::default()
            .verify_password(self.password.clone().unwrap().as_bytes(), &parsed_hash)
            .is_ok();

        if password_match {
            Ok(())
        } else {
            return Err(UserError::PasswordDontMatch);
        }
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
