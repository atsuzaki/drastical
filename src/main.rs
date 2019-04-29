extern crate actix_web;
extern crate dotenv;

mod discord {
    use serde::{Serialize};

    static USERNAME: &str = "DigiDailies";
    static AVATAR_URL: &str = "https://pbs.twimg.com/profile_images/1078696700506791936/QHYnmKxk_400x400.jpg";

    #[derive(Serialize, Debug)]
    pub struct DiscordRequest<'a> {
        pub username: &'a str,
        pub content: &'a str,
        pub avatar_url: &'a str,
    }

    impl<'a> DiscordRequest<'a> {
        pub fn new(content: & str) -> DiscordRequest {
            DiscordRequest {
                username: &USERNAME,
                avatar_url: &AVATAR_URL,
                content,
            }
        }
    }
}


use std::env;
use dotenv::dotenv;

use actix_web::{App, Error, HttpRequest, HttpResponse, http, error, client, server, Responder, AsyncResponder, middleware};
use actix_web::error::{ResponseError, ErrorInternalServerError}; // TODO: temp
use futures::future::Future;

use crate::discord::DiscordRequest;

fn index(req: &HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Request received\n")
}

fn send_to_discord(req: &HttpRequest) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let url = env::var("DISCORD_ADMIN_HOOK").expect("Please set admin channel Webhooks URL!");
    let a = client::ClientRequest::post(url)
        .header(http::header::CONTENT_TYPE, "application/json")
        .json(&DiscordRequest::new("Hello from Rust!"))
        .unwrap();

    println!("{:#?}", a);

    let b = a.send()
        .map_err(|e| {
            println!("{:?}", e.error_response());
            ErrorInternalServerError(e)
        })
        .and_then(|result| {
            println!("{:?}", result);
            Ok(HttpResponse::Ok().body("Request sent!\n"))
        })
        .responder();

    b
}

fn main() {
    println!("Starting http server");
    dotenv().ok();

    let sys = actix::System::new("digidailies-service");

    server::new(
        || App::new()
            .middleware(middleware::Logger::default())
            .resource("/push", |r| r.method(http::Method::POST).f(index))
            .resource("/sendToDiscord",
                      |r| r.method(http::Method::POST).f(send_to_discord))
        )

        .bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("Server running on 127.0.0.1:8088");
    let _ = sys.run();
}
