use self::client::SwitchBotClient;
use crate::domain::switchbot::{
    get_devices::DevicesResponse, get_devices_status::DevicesStatusResponse, SwitchBotApi,
};
mod client;

pub struct SwitchBotApiImpl {}

impl SwitchBotApiImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl SwitchBotApi for SwitchBotApiImpl {
    async fn get_devices(&self) -> anyhow::Result<DevicesResponse> {
        let client = SwitchBotClient::new();
        Ok(client.get::<DevicesResponse>("/devices").await?)
    }

    async fn get_devices_status(&self, device_id: &str) -> anyhow::Result<DevicesStatusResponse> {
        let client = SwitchBotClient::new();
        Ok(client
            .get::<DevicesStatusResponse>(&format!("/devices/{}/status", device_id))
            .await?)
    }
}
