use actix_web::error::ErrorInternalServerError;
use actix_web::{client, http, AsyncResponder, Error, HttpResponse};
use futures::future::Future;
use serde::Serialize;

static USERNAME: &str = "DigiDailies";
static AVATAR_URL: &str =
    "https://pbs.twimg.com/profile_images/1078696700506791936/QHYnmKxk_400x400.jpg";

#[derive(Serialize, Debug)]
pub struct DiscordRequest<'a> {
    pub username: &'a str,
    pub content: &'a str,
    pub avatar_url: &'a str,
}

impl<'a> DiscordRequest<'a> {
    pub fn send(content: &str, url: &str) -> Box<Future<Item = HttpResponse, Error = Error>> {
        client::ClientRequest::post(url)
            .header(http::header::CONTENT_TYPE, "application/json")
            .json(&DiscordRequest::new(content))
            .unwrap()
            .send()
            .map_err(|e| ErrorInternalServerError(e)) // TODO: better error message
            .and_then(|_| Ok(HttpResponse::Ok().body("Request sent!\n")))
            .responder()
    }

    pub fn new(content: &str) -> DiscordRequest {
        DiscordRequest {
            username: &USERNAME,
            avatar_url: &AVATAR_URL,
            content,
        }
    }
}
