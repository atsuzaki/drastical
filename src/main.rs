extern crate actix_web;
extern crate dotenv;

use actix_web::{http, middleware, server, App, Error, HttpRequest, HttpResponse, Json, Responder};
use dotenv::dotenv;
use futures::future::Future;
use serde::{Deserialize, Serialize};
use std::env;

mod discord;

use crate::discord::DiscordRequest;

// Webhook PushEvent, only deserializing content
#[derive(Deserialize, Debug)]
struct PushEvent {
    content: String,
}

#[derive(Debug)]
struct AppState {
    env: AppEnv,
}

#[derive(Serialize, Debug)]
struct AppEnv {
    admin_hook_url: String,
    theme_hook_url: String,
}

impl AppEnv {
    fn populate() -> Result<AppEnv, std::env::VarError> {
        Ok(AppEnv {
            admin_hook_url: env::var("DISCORD_ADMIN_HOOK")?, // TODO: consider using expect instead, so theres err message
            theme_hook_url: env::var("DISCORD_THEME_HOOK")?,
        })
    }
}

fn webhook_zap(p: Json<PushEvent>) -> impl Responder {
    println!("{:?}", p);
    HttpResponse::Ok().body("Request received\n")
}

fn webhook_twitter(p: Json<PushEvent>) -> impl Responder {
    println!("{:?}", p);
    HttpResponse::Ok().body("Request received\n")
}

#[rustfmt::skip]
fn webhook_manual((p, req): (Json<PushEvent>, HttpRequest<AppState>)) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let url = &req.state().env.admin_hook_url;
    DiscordRequest::send(&p.content, &url)
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
        .resource("/pushZap", |r| {
            r.method(http::Method::POST).with(webhook_zap)
        })
        .resource("/pushTwitter", |r| {
            r.method(http::Method::POST).with(webhook_twitter)
        })
        .resource("/pushManual", |r| {
            r.method(http::Method::POST).with(webhook_manual)
        })
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .start();

    println!("Server running on 127.0.0.1:8088");
    let _ = sys.run();
}
