use actix_cors::Cors;
use actix_web::{
    error, get,
    http::header,
    middleware, post,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use server::{self, Excercise};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/workout")]
async fn post_workout(item: web::Json<Excercise>) -> impl Responder {
    println!("Got a request: {:?}", &item);

    // let body = req.

    HttpResponse::Ok()
        .insert_header(header::ContentType(mime::APPLICATION_JSON))
        .json(item.0)
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(post_workout)
            .service(hello)
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         let cors = Cors::permissive();
//
//         App::new()
//             .wrap(cors)
//             .service(post_workout).service(hello)
//     })
//     .bind(("127.0.0.1", 8080)).unwrap().run().await;
//
// }
#[test]
fn initial_test() {
    assert_eq!(2 + 2, 4);
}
