use std::env;

pub struct SwitchBotEnv {
    pub token: String,
    pub secret: String,
}

impl SwitchBotEnv {
    pub fn new() -> Self {
        Self {
            token: env::var("SWITCH_BOT_TOKEN").expect("SWITCH_BOT_TOKEN is not set"),
            secret: env::var("SWITCH_BOT_SECRET").expect("SWITCH_BOT_SECRET is not set"),
        }
    }
}
