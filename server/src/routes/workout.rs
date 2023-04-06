use actix_web::{
    error, get,
    http::header,
    middleware, post,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use server::{self, Excercise};

#[post("/workout")]
pub async fn post_workout(item: web::Json<Excercise>) -> impl Responder {
    println!("Got a request: {:?}", &item);

    // let body = req.

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(item.0)
}
