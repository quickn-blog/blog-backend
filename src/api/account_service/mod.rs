use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

pub mod errors;

use crate::db;
use crate::CONFIG;
use errors::AccountError;
// use hmac::{Hmac, NewMac};
use jwt_simple::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseBlock<T> {
    status: bool,
    body: Option<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Ping {
    reply: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    pass: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    result: AccountError,
    token: Option<String>, // JWT token
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountToken {
    pk: i32,
}

#[get("/api/account_service/ping")]
pub async fn ping() -> web::Json<ResponseBlock<Ping>> {
    web::Json(ResponseBlock {
        status: true,
        body: Some(Ping {
            reply: String::from("pong!"),
        }),
    })
}

#[post("/api/account_service/login")]
pub async fn login(form: web::Json<LoginForm>) -> HttpResponse {
    let config = CONFIG.clone();
    let (err, pk) =
        db::login(&form.username, &form.pass).unwrap_or((AccountError::DatabaseError, -1));
    let json = if err == AccountError::Nothing {
        LoginResponse {
            result: err,
            token: Some({
                let key = HS256Key::from_bytes(config.secret.secret.as_bytes());
                let auth = AccountToken { pk };
                let claims = Claims::with_custom_claims(auth, Duration::from_days(1));
                key.authenticate(claims).unwrap()
            }),
        }
    } else {
        LoginResponse {
            result: err,
            token: None,
        }
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBlock {
            status: true,
            body: Some(json),
        })
}
