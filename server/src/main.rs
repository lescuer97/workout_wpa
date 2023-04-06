use actix_cors::Cors;
use actix_web::{error, middleware, web, App, HttpRequest, HttpResponse, HttpServer};

mod routes;
use routes::workout::post_workout;

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
//

#[cfg(test)]
mod tests {
    #[test]
    fn initial_test() {
        assert_eq!(2 + 2, 4);
    }
}
