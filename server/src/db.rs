use crate::auth::{LoginDataToSend, RegisterUserData};
use crate::error::UserError;
use crate::Excercise;
use actix_web::web::Data;
use sqlx;
use sqlx::{Pool, Postgres};

pub async fn register_user_query(
    user: RegisterUserData,
    pool: Data<Pool<Postgres>>,
) -> Result<(), UserError> {
    let mut conn = pool.acquire().await?;
    if let Some(uuid) = user.id {
        sqlx::query_as!(
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
        .await?;

        return Ok(());
    } else {
        return Err(UserError::UnexpectedError);
    }
}

pub async fn query_user_with_email(
    email: String,
    pool: Data<Pool<Postgres>>,
) -> Result<LoginDataToSend, UserError> {
    let user_query: LoginDataToSend = sqlx::query_as!(
        LoginDataToSend,
        r#"
        SELECT email, password, u2f, totp, id
        FROM users
        where email = $1
            "#,
        email
    )
    .fetch_one(pool.get_ref())
    .await?;

    return Ok(user_query);
}

pub async fn register_excersice(
    ex: Excercise,
    pool: Data<Pool<Postgres>>,
) -> Result<(), UserError> {
    let mut conn = pool.acquire().await?;
    if let Some(uuid) = ex.id {
        sqlx::query_as!(
            Excersice,
            r#"
    INSERT INTO excersices ( name, weight, media_url, sets, rest, reps, weight_unit, used_muscles, workout_type, id)
    VALUES ( $1, $2, $3, $4, $5, $6, $7, $8::muscle[], $9, $10)
            "#,
            ex.name,
            ex.weight,
            ex.media_url,
            ex.sets,
            ex.rest,
            ex.reps,
            ex.weight_unit.to_owned() as _,
            ex.used_muscles.to_owned() as _,
            ex.workout_type.to_owned() as _,
            uuid,
        )
        .execute(&mut conn)
        .await?;

        return Ok(());
    } else {
        return Err(UserError::UnexpectedError);
    }
}
