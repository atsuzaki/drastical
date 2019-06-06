use actix_web::{client, http, Error, HttpResponse};
use futures::Future;
use serde::{Deserialize, Serialize};

use crate::misc::aliases::FutureResponse;

// TODO: place in state or take from webhook data?
static USERNAME: &str = "DigiDailies";
static AVATAR_URL: &str =
    "https://pbs.twimg.com/profile_images/1078696700506791936/QHYnmKxk_400x400.jpg";

// TODO: move to models, or maybe a discord mod.rs
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
    pub fn send(content: &'static str, url: &str) -> impl FutureResponse {
        client::Client::new()
            .post(url)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send_json(&DiscordRequest::new(&content))
            .and_then(|_| Ok(HttpResponse::Ok().body("Message sent!\n")))
            .or_else(|_| Ok(HttpResponse::BadRequest().finish()))
    }

    pub fn new(content: &str) -> DiscordRequest {
        DiscordRequest {
            username: &USERNAME,
            avatar_url: &AVATAR_URL,
            content,
        }
    }
}
