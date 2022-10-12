use std::env;

pub struct SwitchBotEnv {
    pub token: String,
    pub secret: String,
    pub meter_plus_device_id: String,
    pub plug_mini_pc_device_id: String,
    pub plug_mini_desk_device_id: String,
}

impl SwitchBotEnv {
    pub fn new() -> Self {
        Self {
            token: env::var("SWITCHBOT_TOKEN").expect("SWITCHBOT_TOKEN is not set"),
            secret: env::var("SWITCHBOT_SECRET").expect("SWITCHBOT_SECRET is not set"),
            meter_plus_device_id: env::var("SWITCHBOT_METER_PLUS_DEVICE_ID")
                .expect("SWITCHBOT_METER_PLUS_DEVICE_ID is not set"),
            plug_mini_pc_device_id: env::var("SWITCHBOT_PLUG_MINI_PC_DEVICE_ID")
                .expect("SWITCHBOT_PLUG_MINI_PC_DEVICE_ID is not set"),
            plug_mini_desk_device_id: env::var("SWITCHBOT_PLUG_MINI_DESK_DEVICE_ID")
                .expect("SWITCHBOT_PLUG_MINI_DESK_DEVICE_ID is not set"),
        }
    }
}
