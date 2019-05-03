use actix_web::{client, error, http, AsyncResponder, FutureResponse, HttpResponse};
use futures::future::Future;
use serde::{Deserialize, Serialize};

static USERNAME: &str = "DigiDailies";
static AVATAR_URL: &str =
    "https://pbs.twimg.com/profile_images/1078696700506791936/QHYnmKxk_400x400.jpg";

#[derive(Serialize, Debug)]
pub struct DiscordRequest<'a> {
    pub username: &'a str,
    pub content: &'a str,
    pub avatar_url: &'a str,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DiscordChannel {
    Theme,
    Admin,
}

impl Default for DiscordChannel {
    fn default() -> Self {
        DiscordChannel::Admin
    }
}

impl<'a> DiscordRequest<'a> {
    // TODO: more generic error type?
    pub fn send(content: &str, url: &str) -> FutureResponse<HttpResponse> {
        client::ClientRequest::post(url)
            .header(http::header::CONTENT_TYPE, "application/json")
            .json(&DiscordRequest::new(content))
            .unwrap()
            .send()
            .map_err(|e| error::ErrorBadRequest(e))
            .and_then(|_| Ok(HttpResponse::Ok().body("Message sent!\n")))
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
