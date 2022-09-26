use self::{get_devices::DevicesResponse, get_devices_status::DevicesStatusResponse};

pub mod get_devices;
pub mod get_devices_status;

#[async_trait]
pub trait SwitchBotApi {
    async fn get_devices(&self) -> anyhow::Result<DevicesResponse>;
    async fn get_devices_status(&self, device_id: &str) -> anyhow::Result<DevicesStatusResponse>;
}
