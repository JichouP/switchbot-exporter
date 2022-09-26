use self::client::SwitchBotClient;
use crate::domain::switchbot::{
    get_devices::GetDevicesResponse, get_devices_status::GetDevicesStatusResponse, SwitchBotApi,
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
    async fn get_devices(&self) -> anyhow::Result<GetDevicesResponse> {
        let client = SwitchBotClient::new();
        client.get::<GetDevicesResponse>("/devices").await
    }

    async fn get_devices_status(
        &self,
        device_id: &str,
    ) -> anyhow::Result<GetDevicesStatusResponse> {
        let client = SwitchBotClient::new();
        client
            .get::<GetDevicesStatusResponse>(&format!("/devices/{}/status", device_id))
            .await
    }
}
