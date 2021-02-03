use crate::db::schema::*;
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

#[derive(Insertable)]
#[table_name = "users"]
pub struct Register<'a> {
    pub username: &'a str,
    pub pass: &'a str,
    pub email: &'a str,
    pub nickname: &'a str,
    pub permission: i32,
}
