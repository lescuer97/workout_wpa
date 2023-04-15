use actix_web::web::Data;
use anyhow::{bail, Result};
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

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Password Doesn't match")]
    PasswordDontMatch,
    #[error("{0}")]
    DBError(#[from] sqlx::Error),
    #[error("Error wasn't expected")]
    UnexpectedError,
}

impl RegisterUserData {
    fn set_default_user(&mut self) -> Result<()> {
        let same_password = self.check_same_password();

        if !same_password {
            bail!("passwords are not the same");
        }
        let uuid = Uuid::new_v4();

        self.rol = Some(vec![UserRole::EditSelf]);
        self.id = Some(uuid);
        self.password = self.hash_password()?;
        self.u2f = Some(false);
        self.totp = Some(false);

        Ok(())
    }

    pub async fn register(&mut self, pool: Data<Pool<Postgres>>) -> Result<()> {
        self.set_default_user()?;

        let mut conn = pool.acquire().await.unwrap();

        if let Some(uuid) = self.id {
            match sqlx::query_as!(
                RegisterUserData,
                r#"
    INSERT INTO users ( email, password,id, u2f, totp, rol)
    VALUES ( $1, $2, $3, $4, $5, $6::user_role[])
            "#,
                self.email,
                self.password,
                uuid,
                self.u2f,
                self.totp,
                self.rol.to_owned() as _,
            )
            .execute(&mut conn)
            .await
            {
                Ok(user) => user,
                Err(error) => {
                    bail!(UserError::DBError(error))
                }
            };

            Ok(())
        } else {
            bail!(UserError::UnexpectedError)
        }
    }

    fn check_same_password(&self) -> bool {
        self.password == self.password_repeat
    }
    fn hash_password(&self) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let password_bytes = self.password.as_bytes();

        let password_hash = match argon2.hash_password(password_bytes, &salt) {
            Ok(hashed_password) => hashed_password.to_string(),
            Err(_) => bail!("There was a problem hashing the password"),
        };

        let parsed_hash = match PasswordHash::new(&password_hash) {
            Ok(hashed_value) => hashed_value,
            Err(_) => bail!("There was a getting the hashed password"),
        };

        Ok(parsed_hash.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginData {
    pub email: String,
    pub password: Option<String>,
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
