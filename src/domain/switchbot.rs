use self::{get_devices::GetDevicesResponse, get_devices_status::GetDevicesStatusResponse};

pub mod get_devices;
pub mod get_devices_status;

#[async_trait]
pub trait SwitchBotApi {
    async fn get_devices(&self) -> anyhow::Result<GetDevicesResponse>;
    async fn get_devices_status(&self, device_id: &str)
        -> anyhow::Result<GetDevicesStatusResponse>;
}
