#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate actix_cors;
extern crate actix_rt;
extern crate actix_web;
extern crate dotenv;
extern crate env_logger;
extern crate serde;
extern crate serde_json;

use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};

mod api;
mod conf;
mod models;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "actix_web=debug,debug");

    env_logger::init();

    let app_name = std::env::var("APP_NAME").expect("APP_NAME not found.");
    let app_host = std::env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = std::env::var("APP_PORT").expect("APP_PORT not found.");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let api_version = std::env::var("API_VERSION").expect("API_VERSION not found.");

    std::env::set_var("API_VERSION", format!("v{}", api_version));
    let app_url = format!("{}:{}", &app_host, &app_port);

    let client = utils::mongo::get_client(db_url.as_str(), app_name.as_str());
    info!("Connected to database.");

    HttpServer::new(move || {
        App::new()
            .data(client.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["HEAD", "OPTIONS", "GET", "POST"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600)
                    .finish(),
            )
            .configure(conf::config_services)
    })
    .bind(&app_url)
    .unwrap()
    .run()
    .await
}
