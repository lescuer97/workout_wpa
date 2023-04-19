use crate::auth::{LoginDataToSend, RegisterUserData};
use crate::error::UserError;
use actix_web::web::Data;
use sqlx;
use sqlx::{Pool, Postgres};

pub async fn register_user_query(
    user: RegisterUserData,
    pool: Data<Pool<Postgres>>,
) -> Result<(), UserError> {
    let mut conn = pool.acquire().await?;
    if let Some(uuid) = user.id {
        match sqlx::query_as!(
            RegisterUserData,
            r#"
    INSERT INTO users ( email, password,id, u2f, totp, rol)
    VALUES ( $1, $2, $3, $4, $5, $6::user_role[])
            "#,
            user.email,
            user.password,
            uuid,
            user.u2f,
            user.totp,
            user.rol.to_owned() as _,
        )
        .execute(&mut conn)
        .await
        {
            Ok(user) => user,
            Err(error) => {
                return Err(UserError::DBError(error));
            }
        };

        return Ok(());
    } else {
        return Err(UserError::UnexpectedError);
    }
}

pub async fn query_user_with_email(
    email: String,
    pool: Data<Pool<Postgres>>,
) -> Result<LoginDataToSend, UserError> {
    let user_query: LoginDataToSend = match sqlx::query_as!(
        LoginDataToSend,
        r#"
        SELECT email, password, u2f, totp, id
        FROM users
        where email = $1
            "#,
        email
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(user) => user,
        Err(error) => return Err(UserError::DBError(error)),
    };

    return Ok(user_query);
}
