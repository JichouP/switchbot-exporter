use crate::{
    domain::switchbot::{
        get_devices::GetDevicesResponse, get_devices_status::GetDevicesStatusResponse, SwitchBotApi,
    },
    infrastructure::api::switchbot::SwitchBotApiImpl,
};

pub async fn getDevices() -> anyhow::Result<GetDevicesResponse> {
    let api = SwitchBotApiImpl::new();
    api.get_devices().await
}

pub async fn getDevicesStatus(device_id: &str) -> anyhow::Result<GetDevicesStatusResponse> {
    let api = SwitchBotApiImpl::new();
    api.get_devices_status(device_id).await
}
