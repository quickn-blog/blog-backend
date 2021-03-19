use crate::db::schema::*;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountLevel {
    Default = 0,
    Admin = 1,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass: String,
    pub email: String,
    pub nickname: String,
    pub permission: i32,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: i32,
    pub tags: String,
    pub permission: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct Register<'a> {
    pub username: &'a str,
    pub pass: &'a str,
    pub email: &'a str,
    pub nickname: &'a str,
    pub permission: i32,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub author: i32,
    pub tags: &'a str,
    pub permission: i32,
}

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct PostHeader {
    pub id: i32,
    pub title: String,
    pub author: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}