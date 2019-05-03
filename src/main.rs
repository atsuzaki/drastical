extern crate actix_web;
extern crate dotenv;

use actix_web::{
    http, middleware, server, App, FutureResponse, HttpRequest, HttpResponse, Json, Responder,
};
use dotenv::dotenv;
use futures::future::{ok, Future};
use serde::{Deserialize, Serialize};
use std::env;

mod discord;

use crate::discord::{DiscordChannel, DiscordRequest};

///
/// Shape for webhook automatic PushEvent, currently following my setup at Zapier's
///
#[derive(Deserialize, Debug)]
struct PushEvent {
    tweet_url: String,
    content: String,
}

///
/// Shape for webhook manual PushEvent, with channel as option
///
#[derive(Deserialize, Debug)]
struct ManualPushEvent {
    content: String,

    #[serde(default)]
    channel: DiscordChannel,
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
            admin_hook_url: env::var("DISCORD_ADMIN_HOOK")
                .expect("Admin channel webhook url is not set in .env!"),
            theme_hook_url: env::var("DISCORD_THEME_HOOK")
                .expect("Theme channel webhook url is not set in .env!"),
        })
    }
}

fn index(req: &HttpRequest<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Welcome to Drastical service\n")
}

///
/// Endpoint for receiving Zapier webhook data
/// Will be deprecated and replaced as soon as DigiDailies twitter dev account is approved
///
#[rustfmt::skip]
fn webhook_zap((p, req): (Json<PushEvent>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let url = &req.state().env.theme_hook_url;

    if p.content.len() > 2 && &p.content[0..2] == "rt" { // TODO: better way
        Box::new(ok(HttpResponse::Accepted().body("Is a retweet"))) // Wrap into a FutureResponse // TODO: make helper 
    }
    else {
        DiscordRequest::send(&p.tweet_url, &url)
    }
}

///
/// Endpoint for manual content pushes
///
#[rustfmt::skip]
fn webhook_manual((p, req): (Json<ManualPushEvent>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse>  {
    let url = if p.channel == DiscordChannel::Theme { &req.state().env.theme_hook_url } else { &req.state().env.admin_hook_url };
    DiscordRequest::send(&p.content, &url)
}

fn main() {
    println!("Starting http server");
    dotenv().ok();

    let port = env::var("PORT").expect("Port is not set in .env!");
    let sys = actix::System::new("digidailies-service");

    server::new(|| {
        App::with_state(AppState {
            env: AppEnv::populate().unwrap_or_else(|e| {
                eprintln!("Failure populating environment variables: {:?}", e);
                std::process::exit(1)
            }),
        })
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .resource("/pushZap", |r| {
            r.method(http::Method::POST).with(webhook_zap)
        })
        .resource("/pushManual", |r| {
            r.method(http::Method::POST).with(webhook_manual)
        })
    })
    .bind(&port)
    .unwrap()
    .start();

    println!("Drastical running on {}", &port);
    let _ = sys.run();
}
