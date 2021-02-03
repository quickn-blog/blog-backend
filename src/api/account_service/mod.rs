use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

pub mod errors;

use crate::db;
use crate::CONFIG;
use crate::db::models::AccountLevel;
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

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LoginForm {
    username: String,
    pass: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    result: AccountError,
    token: Option<String>, // JWT token
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterForm {
    username: String,
    pass: String,
    email: String,
    nickname: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    result: AccountError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccountToken {
    pk: i32,
}

#[derive(Clone, Deserialize)]
pub struct AuthRequest {
    token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InfoResponse {
    username: String,
    nickname: String,
    email: String,
    level: AccountLevel,
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

#[get("/api/account_service/info")]
pub async fn info(web::Query(parms): web::Query<AuthRequest>) -> HttpResponse {
    let config = CONFIG.clone();
    let key = HS256Key::from_bytes(config.secret.secret.as_bytes());
    let claims_wrapped = key.verify_token::<AccountToken>(&parms.token, None);
    let json = if let Ok(claims) = claims_wrapped {
        if let Ok(user) = db::find_user(claims.custom.pk) {
            Some(
                InfoResponse {
                    username: user.username,
                    nickname: user.nickname,
                    email: user.email,
                    level: match user.permission {
                        1 => AccountLevel::Admin,
                        _ => AccountLevel::Default,
                    },
                }
            )
        } else {
            None
        }
    } else {
        None
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBlock {
            status: json.is_some(),
            body: json,
        })
}

#[post("/api/account_service/login")]
pub async fn login(form_str: String) -> HttpResponse {
    let form: LoginForm = serde_json::from_str(&form_str).unwrap_or_default();
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

#[post("/api/account_service/register")]
pub async fn register(form: web::Json<RegisterForm>) -> HttpResponse {
    let mut result = AccountError::Nothing;
    let v1 = db::by_username(&form.username).unwrap_or(vec![]);
    let v2 = db::by_email(&form.email).unwrap_or(vec![]);
    if !v1.is_empty() {
        result = AccountError::UsernameAlreadyExists;
    }
    if !v2.is_empty() {
        result = AccountError::EmailAlreadyExists;
    }
    if v1.is_empty() && v2.is_empty() {
        if let Err(_) = db::register(&form.username, &form.pass, &form.email, &form.nickname, AccountLevel::Default) {
            result = AccountError::DatabaseError;
        }
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBlock {
            status: true,
            body: Some(RegisterResponse {
                result,
            }),
        })
}

