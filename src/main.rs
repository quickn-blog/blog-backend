extern crate actix_files;
extern crate actix_web;
extern crate env_logger;
extern crate hex;
extern crate hmac;
extern crate jwt_simple;
extern crate serde;
extern crate serde_json;
extern crate sha3;
extern crate toml;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod api;
mod config;
mod db;
mod middlewares;

use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

use config::*;

lazy_static! {
    pub static ref CONFIG: Config = load_config("Blog.toml").unwrap_or_default();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = CONFIG.clone();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(api::account_service::ping)
            .service(api::account_service::login)
            .service(api::account_service::register)
            .service(api::account_service::info)
            .service(api::account_service::get_user)
            .service(api::blog_service::count_posts)
            .service(api::blog_service::new_post)
            .service(api::blog_service::view_post)
            .service(api::blog_service::delete_post)
    })
    .bind(&format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
