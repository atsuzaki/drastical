use serde::{Deserialize, Serialize};
use std::env;

use crate::discord::DiscordChannel;

///
/// Shape for webhook automatic PushEvent, currently following my setup at Zapier's
///
#[derive(Deserialize, Debug)]
pub struct PushEvent {
    pub tweet_url: String,
    pub content: String,
}

///
/// Shape for webhook manual PushEvent, with channel as option
///
#[derive(Deserialize, Debug)]
pub struct ManualPushEvent {
    pub content: String,

    #[serde(default)]
    pub channel: DiscordChannel,
}

#[derive(Debug)]
pub struct AppState {
    pub env: AppEnv,
}

#[derive(Serialize, Debug)]
pub struct AppEnv {
    pub admin_hook_url: String,
    pub theme_hook_url: String,
}

impl AppEnv {
    pub fn populate() -> Result<AppEnv, std::env::VarError> {
        Ok(AppEnv {
            admin_hook_url: env::var("DISCORD_ADMIN_HOOK")
                .expect("Admin channel webhook url is not set in .env!"),
            theme_hook_url: env::var("DISCORD_THEME_HOOK")
                .expect("Theme channel webhook url is not set in .env!"),
        })
    }
}
