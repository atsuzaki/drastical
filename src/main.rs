extern crate actix_web;
extern crate dotenv;

use actix_web::{http, middleware, server, App, Error, HttpRequest, HttpResponse, Responder};
use dotenv::dotenv;
use futures::future::Future;
use serde::Serialize;
use std::env;

mod discord;

use crate::discord::DiscordRequest;

#[derive(Debug)]
struct AppState {
    env: AppEnv,
}

#[derive(Serialize, Debug)]
struct AppEnv {
    theme_hook_url: String,
    admin_hook_url: String,
}

impl AppEnv {
    fn populate() -> Result<AppEnv, std::env::VarError> {
        Ok(AppEnv {
            theme_hook_url: env::var("DISCORD_ADMIN_HOOK")?, // TODO: consider using expect instead, so theres err message
            admin_hook_url: env::var("DISCORD_THEME_HOOK")?,
        })
    }
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Request received\n")
}

fn send_to_discord(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let url = &req.state().env.theme_hook_url;
    DiscordRequest::send("Hello from Rust!", &url)
}

fn main() {
    println!("Starting http server");
    dotenv().ok();

    let sys = actix::System::new("digidailies-service");

    server::new(|| {
        App::with_state(AppState {
            env: AppEnv::populate().unwrap_or_else(|e| {
                eprintln!("Failure populating environment variables: {:?}", e);
                std::process::exit(1)
            }),
        })
        .middleware(middleware::Logger::default())
        .resource("/push", |r| r.method(http::Method::POST).f(index))
        .resource("/sendToDiscord", |r| {
            r.method(http::Method::POST).f(send_to_discord)
        })
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .start();

    println!("Server running on 127.0.0.1:8088");
    let _ = sys.run();
}
