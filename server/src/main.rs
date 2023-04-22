#![feature(try_trait_v2)]
use actix_cors::Cors;
use actix_web::{error, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use routes::auth::register_user;
use sqlx::postgres::PgPool;
use sqlx::{Pool, Postgres};
use std::env;

mod routes;
use routes::workout::post_workout;

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;
    println!("error: {:?}", err);

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
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_env_var: String = match env::var("DATABASE_URL") {
        Ok(variable) => variable,
        Err(_) => {
            println!("there is no DATABASE_URL variable set");
            String::from("")
        }
    };

    let pool: Pool<Postgres> = PgPool::connect(&database_env_var).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(post_workout)
            .service(register_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn initial_test() {
        assert_eq!(2 + 2, 4);
    }
}
