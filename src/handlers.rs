use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use futures::future::{ok, Either};

use crate::misc::aliases::FutureResponse;
use crate::models::*;

use crate::discord::{DiscordChannel, DiscordRequest};

pub fn index(_state: Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Drastical service\n")
}



///
/// Endpoint for receiving Zapier webhook data
/// Will be deprecated and replaced as soon as DigiDailies twitter dev account is approved
///
#[rustfmt::skip]
pub fn webhook_zap(p: Json<PushEvent>, data: Data<AppState>) -> impl FutureResponse {
    let url = &data.env.theme_hook_url;

    if p.content.len() > 2 && &p.content[0..2] == "RT" { // TODO: better way
       Either::A(ok(HttpResponse::Accepted().body("Is a retweet")))
    }
    else {

       Either::B(DiscordRequest::send(p.tweet_url.clone(), &url)
 )    }
}

///
/// Endpoint for manual content pushes
///
pub fn webhook_manual(p: Json<ManualPushEvent>, data: Data<AppState>) -> impl FutureResponse {
    let url = if p.channel == DiscordChannel::Theme {
        &data.env.theme_hook_url
    } else {
        &data.env.admin_hook_url
    };
    DiscordRequest::send(p.content.clone(), &url)
}
