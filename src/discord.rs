use actix_web::{client, http, HttpResponse};
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
    pub avatar_url: &'a str,
    pub content: String,
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
    pub fn send(content: String, url: &str) -> impl FutureResponse {
        client::Client::new()
            .post(url)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send_json(&DiscordRequest::new(content))
            // TODO: check status from discord before returning Ok
            .and_then(|_| Ok(HttpResponse::Ok().body("Message sent!\n")))
            .or_else(|_| Ok(HttpResponse::BadRequest().finish()))
    }

    pub fn new<S: Into<String>>(content: S) -> DiscordRequest<'a> {
        DiscordRequest {
            username: &USERNAME,
            avatar_url: &AVATAR_URL,
            content: content.into(),
        }
    }
}
