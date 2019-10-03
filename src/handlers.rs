use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use futures::future::{ok, Either};

use crate::misc::aliases::FutureResponse;
use crate::misc::helpers::is_retweet;
use crate::models::*;

use crate::discord::{DiscordChannel, DiscordRequest};

pub fn index(state: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Drastical service\n")
}

///
/// Endpoint for receiving Zapier webhook data
/// Will be deprecated and replaced as soon as DigiDailies twitter dev account is approved
///
pub fn webhook_zap(p: Json<PushEvent>, state: Data<AppState>) -> impl FutureResponse {
    let url = &state.env.theme_hook_url;

    if is_retweet(&p.content) {
        Either::A(ok(HttpResponse::Accepted().body("Is a retweet")))
    } else {
        Either::B(DiscordRequest::send(
            p.tweet_url.clone(),
            p.pfp_url.clone(),
            &url,
        ))
    }
}

///
/// Endpoint for manual content pushes
///
pub fn webhook_manual(p: Json<ManualPushEvent>, state: Data<AppState>) -> impl FutureResponse {
    let url = if p.channel == DiscordChannel::Theme {
        &state.env.theme_hook_url
    } else {
        &state.env.admin_hook_url
    };
    DiscordRequest::send(p.content.clone(), "".to_string(), &url) // TODO: provide default pfp url
}
