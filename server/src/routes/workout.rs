use actix_web::{http::header, post, Error, HttpRequest, HttpResponse};
use serde_qs as qs;
use server::{self, Excercise};

#[post("/workout")]
pub async fn post_workout(req: HttpRequest) -> Result<HttpResponse, Error> {
    let config = qs::Config::new(25, false);
    let ex = config.deserialize_str::<Excercise>(req.query_string())?;

    return Ok(HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(ex));
}
