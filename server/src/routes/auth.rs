use actix_web::web::Data;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use serde_qs;
use server::{auth::RegisterUserData, server_messages::ResponseBodyMessage};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo, Postgres},
    FromRow, Pool, Type,
};

#[post("/auth/register")]
pub async fn register_user(
    req: HttpRequest,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let config = serde_qs::Config::new(25, false);
    let mut user = config.deserialize_str::<RegisterUserData>(req.query_string())?;

    match user.register(pool).await {
        Ok(()) => (),
        Err(_) => {
            let fail_message =
                ResponseBodyMessage::fail_message("There was a problem registering your user");

            return Ok(HttpResponse::UnprocessableEntity().json(fail_message));
        }
    };
    let success_registering = ResponseBodyMessage::success_message("Registed successfuly");

    return Ok(HttpResponse::Ok().json(success_registering));
}

#[cfg(test)]
mod tests {
    use crate::register_user;

    use actix_web::{guard, http::StatusCode, test, web, App};
    // use chrono::{DateTime, Duration, Utc};
    use dotenv;
    use rand::{thread_rng, Rng};
    // use rust_server::auth::SECRET_FOR_TOTP_AUTH;
    // use rust_server::routes::auth::{totp_create_send_secret, totp_validate_creation};
    // use rust_server::utils::{create_jwt_and_cookie, create_secret_from_entropy};
    use serde_json::json;
    use sqlx;
    use sqlx::postgres::PgPool;
    use std::env;

    #[actix_web::test]
    async fn register_new_user_success() {
        dotenv::dotenv().ok();
        let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let mut rng = thread_rng();

        let random_number: u32 = rng.gen_range(0..999999);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register_user),
        )
        .await;
        let email_to_register = format!("test{}@test.com", random_number);
        let form_data = "email=".to_owned() + &email_to_register + "&password=%26%238V%2An%25%21WL5%5E544%23Z7xr&password_repeat=%26%238V%2An%25%21WL5%5E544%23Z7xr";

        let uri = format!("/auth/register?{}", form_data);

        // Create request object
        let req = test::TestRequest::post()
            .uri(&uri)
            // .header(header::CONTENT_TYPE, "application/json")
            .to_request();

        // Execute application
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn register_new_user_already_exists() {
        dotenv::dotenv().ok();
        let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(register_user),
        )
        .await;

        let form_data = "email=test22s%40test.com&password=%26%238V%2An%25%21WL5%5E544%23Z7xr&password_repeat=%26%238V%2An%25%21WL5%5E544%23Z7xr";
        let uri = format!("/auth/register?{}", form_data);

        // Create request object
        let req = test::TestRequest::post()
            .uri(&uri)
            // .header(header::CONTENT_TYPE, "application/json")
            .to_request();

        // Execute application
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    // #[actix_web::test]
    // async fn login_user() {
    //     dotenv::dotenv().ok();
    //
    //     let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    //         .await
    //         .unwrap();
    //
    //     let app = test::init_service(
    //         App::new().app_data(web::Data::new(pool.clone())).service(
    //             web::resource("/auth/login")
    //                 .route(web::route().guard(guard::Options()).to(options_call))
    //                 .route(web::post().to(login_function)),
    //         ),
    //     )
    //     .await;
    //
    //     let data = json!({
    //         "email": "test22s@test.com",
    //         "password": "&#8V*n%!WL5^544#Z7xr",
    //     });
    //
    //     // Create request object
    //     let req = test::TestRequest::post()
    //         .set_json(data)
    //         .uri("/auth/login")
    //         .to_request();
    //
    //     // Execute application
    //     let res = test::call_service(&app, req).await;
    //     assert_eq!(res.status(), StatusCode::OK);
    // }

    // #[actix_web::test]
    // async fn totp_create_secret_test() {
    //     dotenv::dotenv().ok();
    //
    //     let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    //         .await
    //         .unwrap();
    //
    //     let app = test::init_service(
    //         App::new()
    //             .app_data(web::Data::new(pool.clone()))
    //             .service(totp_create_send_secret),
    //     )
    //     .await;
    //
    //     // Create request object
    //     let req = test::TestRequest::post()
    //         .uri("/auth/totp/create")
    //         .to_request();
    //
    //     // Execute application
    //     let res = test::call_service(&app, req).await;
    //     println!("res {:?}", res);
    //     assert_eq!(res.status(), StatusCode::OK);
    // }

    // #[actix_web::test]
    // async fn totp_validate_create_success() {
    //     dotenv::dotenv().ok();
    //
    //     let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    //         .await
    //         .unwrap();
    //
    //     let app = test::init_service(
    //         App::new()
    //             .app_data(web::Data::new(pool.clone()))
    //             .service(totp_validate_creation),
    //     )
    //     .await;
    //
    //     let secret = create_secret_from_entropy();
    //     let expiration_time: DateTime<Utc> = Utc::now() + Duration::minutes(5);
    //
    //     let totp_validation_cookie =
    //         create_jwt_and_cookie(secret.to_owned(), SECRET_FOR_TOTP_AUTH, expiration_time)
    //             .unwrap();
    //
    //     let totp_token = libreauth::oath::TOTPBuilder::new()
    //         .base32_key(&secret)
    //         .finalize()
    //         .unwrap()
    //         .generate();
    //
    //     // Create request object
    //     let req = test::TestRequest::post()
    //         .cookie(totp_validation_cookie)
    //         .set_json(totp_token)
    //         .uri("/auth/totp/validate")
    //         .to_request();
    //
    //     // Execute application
    //     let res = test::call_service(&app, req).await;
    //     assert_eq!(res.status(), StatusCode::OK);
    // }
    // #[actix_web::test]
    // async fn totp_validate_create_error() {
    //     dotenv::dotenv().ok();
    //
    //     let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    //         .await
    //         .unwrap();
    //
    //     let app = test::init_service(
    //         App::new()
    //             .app_data(web::Data::new(pool.clone()))
    //             .service(totp_validate_creation),
    //     )
    //     .await;
    //
    //     let secret = create_secret_from_entropy();
    //     let expiration_time: DateTime<Utc> = Utc::now() + Duration::minutes(5);
    //
    //     let totp_validation_cookie =
    //         create_jwt_and_cookie(secret.to_owned(), SECRET_FOR_TOTP_AUTH, expiration_time)
    //             .unwrap();
    //
    //     // Create request object
    //     let req = test::TestRequest::post()
    //         .cookie(totp_validation_cookie)
    //         .set_json("1231241234")
    //         .uri("/auth/totp/validate")
    //         .to_request();
    //
    //     // Execute application
    //     let res = test::call_service(&app, req).await;
    //     assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    // }
}
