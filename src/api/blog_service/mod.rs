pub mod errors;
use chrono::prelude::*;

use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::api::account_service::*;
use crate::db;
use crate::db::models::{AccountLevel, Post};
use crate::CONFIG;
use errors::*;
// use hmac::{Hmac, NewMac};
use jwt_simple::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct NewPostForm {
    pub title: String,
    pub body: String,
    pub tag: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ViewPostForm {
    pub id: i64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DeletePostForm {
    pub id: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewPostResponse {
    pub error: BlogError,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CountPostsResponse {
    pub error: BlogError,
    pub count: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicPost {
    pub title: String,
    pub body: String,
    pub author: i32,
    pub tags: Vec<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ViewPostResponse {
    pub error: BlogError,
    pub post: Option<PublicPost>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeletePostResponse {
    pub error: BlogError,
}

#[post("/api/blog/new_post")]
pub async fn new_post(parms: web::Json<AsRequest<NewPostForm>>) -> HttpResponse {
    let config = CONFIG.clone();
    let key = HS256Key::from_bytes(config.secret.secret.as_bytes());
    let claims_wrapped = key.verify_token::<AccountToken>(&parms.token, None);
    let json = if let Ok(claims) = claims_wrapped {
        if let Ok(_) = db::create_post(
            &parms.body.title,
            &parms.body.body,
            claims.custom.pk,
            parms.body.tag.iter().map(|s| s.as_str()).collect(),
            0,
        ) {
            Some(NewPostResponse {
                error: BlogError::Nothing,
            })
        } else {
            Some(NewPostResponse {
                error: BlogError::DatabaseError,
            })
        }
    } else {
        Some(NewPostResponse {
            error: BlogError::AuthError,
        })
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBlock {
            status: json.is_some(),
            body: json,
        })
}

#[get("/api/blog/count_posts")]
pub async fn count_posts() -> HttpResponse {
    let json = if let Ok(cnt) = db::count_posts() {
        Some(CountPostsResponse {
            error: BlogError::Nothing,
            count: cnt,
        })
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

#[post("/api/blog/view_post")]
pub async fn view_post(parms: web::Json<AsRequest<ViewPostForm>>) -> HttpResponse {
    let config = CONFIG.clone();
    let key = HS256Key::from_bytes(config.secret.secret.as_bytes());
    let claims_wrapped = key.verify_token::<AccountToken>(&parms.token, None);
    let json = if let Ok(post) = db::by_post_id(parms.body.id as i32) {
        if post.permission == 0 {
            Some(ViewPostResponse {
                error: BlogError::Nothing,
                post: Some(PublicPost {
                    title: post.title,
                    body: post.body,
                    author: post.author,
                    tags: post.tags.split("|").map(|s| s.to_string()).collect(),
                    created_at: post.created_at,
                    modified_at: post.modified_at,
                }),
            })
        } else {
            if let Ok(claims) = claims_wrapped {
                if let Ok(user) = db::find_user(claims.custom.pk) {
                    if post.permission == user.permission {
                        Some(ViewPostResponse {
                            error: BlogError::Nothing,
                            post: Some(PublicPost {
                                title: post.title,
                                body: post.body,
                                author: post.author,
                                tags: post.tags.split("|").map(|s| s.to_string()).collect(),
                                created_at: post.created_at,
                                modified_at: post.modified_at,
                            }),
                        })
                    } else {
                        Some(ViewPostResponse {
                            error: BlogError::DatabaseError,
                            post: None,
                        })
                    }
                } else {
                    Some(ViewPostResponse {
                        error: BlogError::PermissionError,
                        post: None,
                    })
                }
            } else {
                Some(ViewPostResponse {
                    error: BlogError::AuthError,
                    post: None,
                })
            }
        }
    } else {
        Some(ViewPostResponse {
            error: BlogError::DatabaseError,
            post: None,
        })
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBlock {
            status: json.is_some(),
            body: json,
        })
}

#[post("/api/blog/delete_post")]
pub async fn delete_post(parms: web::Json<AsRequest<DeletePostForm>>) -> HttpResponse {
    let config = CONFIG.clone();
    let key = HS256Key::from_bytes(config.secret.secret.as_bytes());
    let claims_wrapped = key.verify_token::<AccountToken>(&parms.token, None);
    let body = if let Ok(claims) = claims_wrapped {
        if let Ok(post) = db::by_post_id(parms.body.id as i32) {
            if post.author == claims.custom.pk {
                if let Ok(_) = db::delete_post(parms.body.id as i32) {
                    Some(DeletePostResponse {
                        error: BlogError::Nothing,
                    })
                } else {
                    Some(DeletePostResponse {
                        error: BlogError::DatabaseError,
                    })
                }
            } else {
                Some(DeletePostResponse {
                    error: BlogError::AuthError,
                })
            }
        } else {
            Some(DeletePostResponse {
                error: BlogError::DatabaseError,
            })
        }
    } else {
        Some(DeletePostResponse {
            error: BlogError::AuthError,
        })
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ResponseBlock {
            status: body.is_some(),
            body,
        })
}
