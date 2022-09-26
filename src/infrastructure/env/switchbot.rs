use std::env;

pub struct SwitchBotEnv {
    pub token: String,
    pub secret: String,
}

impl SwitchBotEnv {
    pub fn new() -> Self {
        Self {
            token: env::var("SWITCH_BOT_TOKEN").unwrap(),
            secret: env::var("SWITCH_BOT_SECRET").unwrap(),
        }
    }
}
