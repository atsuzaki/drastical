extern crate actix_web;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use crate::handlers::{index, webhook_manual, webhook_zap};
use crate::models::{AppEnv, AppState};

mod discord;
mod handlers;
mod misc;
mod models;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = std::env::var("PORT").expect("Port is not set in .env!");

    println!("Drastical running on {}", &port);

    HttpServer::new(|| {
        App::new()
            .data(AppState {
                env: AppEnv::populate().unwrap_or_else(|e| {
                    eprintln!("Failure populating environment variables: {:?}", e);
                    std::process::exit(1)
                }),
            })
            .service(web::resource("/").to(index))
            .service(web::resource("/pushZap").route(web::post().to_async(webhook_zap)))
            .service(web::resource("/pushManual").route(web::post().to_async(webhook_manual)))
    })
    .bind(&port)?
    .run()
}
