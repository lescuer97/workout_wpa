use actix_web::{http::header, post, web, HttpResponse, Responder};

use server::{self, Excercise};

#[post("/workout")]
pub async fn post_workout(item: web::Json<Excercise>) -> impl Responder {
    println!("Got a request: {:?}", &item);

    // let body = req.

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(item.0)
}
