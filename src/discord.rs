use actix_web::{client, http, HttpResponse};
use futures::Future;
use serde::{Deserialize, Serialize};

use crate::misc::aliases::FutureResponse;

// TODO: place in state or take from webhook data?
static USERNAME: &str = "DigiDailies";

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
    pub fn send(content: String, pfp_url: String, url: &str) -> impl FutureResponse {
        client::Client::new()
            .post(url)
            .header(http::header::CONTENT_TYPE, "application/json")
            .send_json(&DiscordRequest::new(content, pfp_url))
            // TODO: check status from discord before returning Ok
            .and_then(|_| Ok(HttpResponse::Ok().body("Message sent!\n")))
            .or_else(|_| Ok(HttpResponse::BadRequest().finish()))
    }

    pub fn new<S: Into<String>>(content: S, pfp_url: S) -> DiscordRequest<'a> {
        DiscordRequest {
            username: &USERNAME,
            avatar_url: pfp_url.into(),
            content: content.into(),
        }
    }
}
