pub mod models;
pub mod schema;

use crate::api::account_service::errors::AccountError;
use crate::middlewares::postgresql::establish_connection;
use chrono::prelude::*;
use diesel::prelude::*;
use models::*;
use schema::*;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

pub fn register<'a>(
    username: &'a str,
    pass: &'a str,
    email: &'a str,
    nickname: &'a str,
    permission: AccountLevel,
) -> QueryResult<User> {
    let db = establish_connection();
    let mut hasher = Sha3_256::new();
    hasher.update(pass.as_bytes());
    let pass_hashed = hex::encode(hasher.finalize());
    let new_user = Register {
        username,
        pass: &pass_hashed,
        email,
        nickname,
        permission: permission as i32,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&db)
}

pub fn login<'a>(username: &'a str, pass: &'a str) -> QueryResult<(AccountError, i32)> {
    let db = establish_connection();
    let mut hasher = Sha3_256::new();
    hasher.update(pass.as_bytes());
    let pass_hashed = hasher.finalize();
    let mut items = users::table
        .filter(users::dsl::username.eq(username))
        .load::<User>(&db)?;
    if let Some(user) = items.pop() {
        if hex::encode(pass_hashed) == user.pass {
            Ok((AccountError::Nothing, user.id))
        } else {
            Ok((AccountError::PassNotMatched, -1))
        }
    } else {
        Ok((AccountError::UserNotExists, -1))
    }
}

pub fn find_user<'a>(pk: i32) -> QueryResult<User> {
    let db = establish_connection();
    users::table.find(pk).first(&db)
}

pub fn by_username<'a>(username: &'a str) -> QueryResult<Vec<User>> {
    let db = establish_connection();
    users::table
        .filter(users::dsl::username.eq(username))
        .load::<User>(&db)
}

pub fn by_email<'a>(email: &'a str) -> QueryResult<Vec<User>> {
    let db = establish_connection();
    users::table
        .filter(users::dsl::email.eq(email))
        .load::<User>(&db)
}

pub fn create_post<'a>(
    title: &'a str,
    body: &'a str,
    author: i32,
    tags: Vec<&'a str>,
    permission: i32,
) -> QueryResult<Post> {
    let db = establish_connection();
    let new_post = NewPost {
        title,
        body,
        author,
        tags: &tags.join("|"),
        permission,
    };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&db)
}

pub fn posts_by<'a>(start: i64, count: i64) -> QueryResult<Vec<i32>> {
    let db = establish_connection();
    posts::table
        .order(posts::modified_at.desc())
        .select(posts::id)
        .offset(start)
        .limit(count)
        .load::<i32>(&db)
}

pub fn post_header_by<'a>(start: i64, count: i64) -> QueryResult<Vec<PostHeader>> {
    let db = establish_connection();
    posts::table
        .order(posts::modified_at.desc())
        .select((
            posts::id,
            posts::title,
            posts::author,
            posts::created_at,
            posts::modified_at,
        ))
        .offset(start)
        .limit(count)
        .load::<PostHeader>(&db)
}

pub fn count_posts() -> QueryResult<i64> {
    let db = establish_connection();
    posts::table.count().get_result(&db)
}

pub fn by_post_id<'a>(pk: i32) -> QueryResult<Post> {
    let db = establish_connection();
    posts::table.find(pk).first(&db)
}

pub fn delete_post<'a>(pk: i32) -> QueryResult<usize> {
    let db = establish_connection();
    diesel::delete(posts::table.filter(posts::id.eq(pk))).execute(&db)
}

pub fn edit_post<'a>(pk: i32, title: &'a str, body: &'a str, tags: &'a str) -> QueryResult<Post> {
    let db = establish_connection();
    diesel::update(posts::table.filter(posts::id.eq(pk)))
        .set((
            posts::title.eq(title),
            posts::body.eq(body),
            posts::tags.eq(tags),
        ))
        .get_result(&db)
}
