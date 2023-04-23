use actix_web::{get, http::StatusCode, post, web, Error, HttpRequest, HttpResponse};
use chrono::{DateTime, Duration, Utc};
use serde_qs;
use server::{
    auth::{JWTToken, LoginData, RegisterUserData, AUTHENTIFIED_COOKIE, COOKIE_FOR_TOTP_AUTH},
    error::UserError,
    server_messages::ResponseBodyMessage,
    utils::{create_jwt_and_cookie, make_removal_cookie},
};
use sqlx::{postgres::Postgres, Pool};

#[post("/auth/register")]
pub async fn register_user(
    req: HttpRequest,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let config = serde_qs::Config::new(25, false);
    // let mut user = config.deserialize_str::<RegisterUserData>(req.query_string())?;
    let mut user = config.deserialize_str::<RegisterUserData>(req.query_string())?;

    match user.register(pool).await {
        Ok(()) => (),
        Err(err) => match err {
            UserError::DBError(error) => {
                if let Some(error_db) = error.as_database_error() {
                    if let Some(db_err_code) = error_db.code() {
                        if db_err_code == "23505" {
                            let fail_message =
                                ResponseBodyMessage::fail_message("User already exists");
                            return Ok(fail_message.send_response(StatusCode::CONFLICT));
                        }
                    } else {
                        return Ok(HttpResponse::UnprocessableEntity().finish());
                    }
                }
            }
            _ => return Err(UserError::UnexpectedError.into()),
        },
    };

    let success_registering = ResponseBodyMessage::success_message("Registed successfuly");

    return Ok(success_registering.send_response(StatusCode::CREATED));
}

#[post("/auth/login")]
pub async fn login_user(
    req: HttpRequest,
    pool: web::Data<Pool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let config = serde_qs::Config::new(25, false);
    println!("req: {:?}", req.query_string());
    let item = config.deserialize_str::<LoginData>(req.query_string())?;

    println!("item: {:?}", item);

    // used in case that the client calls when already signed in
    match JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE) {
        Ok(_) => {
            let already_logedin_value = ResponseBodyMessage::success_message("Already Logged in");

            return Ok(HttpResponse::Accepted().json(already_logedin_value));
        }
        Err(_) => println!("No jwt token"),
    }

    let login_data = item.login(pool).await?;

    if login_data.totp || login_data.u2f {
        let success_registering = ResponseBodyMessage::success_message("Two factor auth needed");

        let five_min_expiration: DateTime<Utc> = Utc::now() + Duration::minutes(5);

        let jwt_cookie = create_jwt_and_cookie(
            login_data.id.to_string(),
            COOKIE_FOR_TOTP_AUTH,
            five_min_expiration,
        )?;

        Ok(HttpResponse::Ok()
            .cookie(jwt_cookie)
            .json(success_registering))
    } else {
        let expiration_time: DateTime<Utc> = Utc::now() + Duration::days(30);

        let jwt_cookie = match create_jwt_and_cookie(
            login_data.id.to_string(),
            AUTHENTIFIED_COOKIE,
            expiration_time,
        ) {
            Ok(cookie) => cookie,
            Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
        };

        let success_registering = ResponseBodyMessage::success_message(login_data);

        Ok(HttpResponse::Ok()
            .cookie(jwt_cookie)
            .json(success_registering))
    }
}
#[get("/auth/checklogin")]
pub async fn check_login(req: HttpRequest) -> HttpResponse {
    match JWTToken::validate_jwt_token_from_cookie(req, AUTHENTIFIED_COOKIE) {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(_) => {
            let jwt_cookie = match make_removal_cookie(AUTHENTIFIED_COOKIE) {
                Ok(cookie) => cookie,
                Err(_) => return HttpResponse::InternalServerError().finish(),
            };
            return HttpResponse::UnprocessableEntity()
                .cookie(jwt_cookie)
                .finish();
        }
    };
}

#[post("/auth/logout")]
pub async fn logout() -> HttpResponse {
    let jwt_cookie = match make_removal_cookie(AUTHENTIFIED_COOKIE) {
        Ok(cookie) => cookie,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let logout_response = ResponseBodyMessage::success_message("Logged out user");

    HttpResponse::Ok().cookie(jwt_cookie).json(logout_response)
}

#[cfg(test)]
mod tests {
    use crate::register_user;
    use crate::routes::auth::login_user;

    use actix_web::{http::StatusCode, test, web, App};
    // use chrono::{DateTime, Duration, Utc};
    use dotenv;
    use rand::{thread_rng, Rng};
    use serde::Deserialize;
    // use rust_server::auth::SECRET_FOR_TOTP_AUTH;
    // use rust_server::routes::auth::{totp_create_send_secret, totp_validate_creation};
    // use rust_server::utils::{create_jwt_and_cookie, create_secret_from_entropy};
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
        assert_eq!(res.status(), StatusCode::CREATED);
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

        #[derive(Deserialize)]
        struct ErrorFromJson {
            pub status: String,
            pub data: String,
        }

        // Execute application
        let res: ErrorFromJson = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res.data, "User already exists");
    }

    #[actix_web::test]
    async fn login_user_test() {
        dotenv::dotenv().ok();

        let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(login_user),
        )
        .await;

        let form_data = "email=test22s%40test.com&password=%26%238V%2An%25%21WL5%5E544%23Z7xr";
        let uri = format!("/auth/login?{}", form_data);

        // Create request object
        let req = test::TestRequest::post().uri(uri.as_str()).to_request();

        // Execute application
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::OK);
    }
    #[actix_web::test]
    async fn login_user_check_cookie_test() {
        dotenv::dotenv().ok();

        let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(login_user),
        )
        .await;

        let form_data = "email=test22s%40test.com&password=%26%238V%2An%25%21WL5%5E544%23Z7xr";
        let uri = format!("/auth/login?{}", form_data);

        // Create request object
        let req = test::TestRequest::post().uri(uri.as_str()).to_request();

        // Execute application
        let res = test::call_service(&app, req).await;
        let cookie = res.headers().get("set-cookie").unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(cookie.to_str().unwrap().contains("auth"), true);

        // grab auth cookie
    }

    //  #[actix_web::test]
    //  async fn totp_create_secret_test() {
    //      dotenv::dotenv().ok();
    //
    //      let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    //          .await
    //          .unwrap();
    //
    //      let app = test::init_service(
    //          App::new()
    //              .app_data(web::Data::new(pool.clone()))
    //              .service(totp_create_send_secret),
    //      )
    //      .await;
    //
    //      // Create request object
    //      let req = test::TestRequest::post()
    //          .uri("/auth/totp/create")
    //          .to_request();
    //
    //      // Execute application
    //      let res = test::call_service(&app, req).await;
    //      println!("res {:?}", res);
    //      assert_eq!(res.status(), StatusCode::OK);
    //  }

    //  #[actix_web::test]
    //  async fn totp_validate_create_success() {
    //      dotenv::dotenv().ok();
    //
    //      let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    //          .await
    //          .unwrap();
    //
    //      let app = test::init_service(
    //          App::new()
    //              .app_data(web::Data::new(pool.clone()))
    //              .service(totp_validate_creation),
    //      )
    //      .await;
    //
    //      let secret = create_secret_from_entropy();
    //      let expiration_time: DateTime<Utc> = Utc::now() + Duration::minutes(5);
    //
    //      let totp_validation_cookie =
    //          create_jwt_and_cookie(secret.to_owned(), SECRET_FOR_TOTP_AUTH, expiration_time)
    //              .unwrap();
    //
    //      let totp_token = libreauth::oath::TOTPBuilder::new()
    //          .base32_key(&secret)
    //          .finalize()
    //          .unwrap()
    //          .generate();
    //
    //      // Create request object
    //      let req = test::TestRequest::post()
    //          .cookie(totp_validation_cookie)
    //          .set_json(totp_token)
    //          .uri("/auth/totp/validate")
    //          .to_request();
    //
    //      // Execute application
    //      let res = test::call_service(&app, req).await;
    //      assert_eq!(res.status(), StatusCode::OK);
    //  }
    //  #[actix_web::test]
    //  async fn totp_validate_create_error() {
    //      dotenv::dotenv().ok();
    //
    //      let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap())
    //          .await
    //          .unwrap();
    //
    //      let app = test::init_service(
    //          App::new()
    //              .app_data(web::Data::new(pool.clone()))
    //              .service(totp_validate_creation),
    //      )
    //      .await;
    //
    //      let secret = create_secret_from_entropy();
    //      let expiration_time: DateTime<Utc> = Utc::now() + Duration::minutes(5);
    //
    //      let totp_validation_cookie =
    //          create_jwt_and_cookie(secret.to_owned(), SECRET_FOR_TOTP_AUTH, expiration_time)
    //              .unwrap();
    //
    //      // Create request object
    //      let req = test::TestRequest::post()
    //          .cookie(totp_validation_cookie)
    //          .set_json("1231241234")
    //          .uri("/auth/totp/validate")
    //          .to_request();
    //
    //      // Execute application
    //      let res = test::call_service(&app, req).await;
    //      assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    //  }
}
