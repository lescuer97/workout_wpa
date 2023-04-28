use actix_web::{http::header, post, web, Error, HttpRequest, HttpResponse};
use serde_qs as qs;
use server::{self, db::register_excersice, Excercise};
use sqlx::{Pool, Postgres};

#[post("/workout")]
pub async fn create_workout(
    req: HttpRequest,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let config = qs::Config::new(25, false);
    let mut ex = config.deserialize_str::<Excercise>(req.query_string())?;

    ex.id = Some(uuid::Uuid::new_v4());

    register_excersice(ex.clone(), pool).await?;

    return Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(ex));
}
