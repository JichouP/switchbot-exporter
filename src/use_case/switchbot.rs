use crate::{
    domain::switchbot::{
        get_devices::GetDevicesResponse,
        get_devices_status::{GetDevicesMeterPlusStatusResponse, GetDevicesPlugMiniStatusResponse},
        SwitchBotApi,
    },
    infrastructure::api::switchbot::SwitchBotApiImpl,
};

#[allow(dead_code)]
pub async fn get_devices() -> anyhow::Result<GetDevicesResponse> {
    let api = SwitchBotApiImpl::new();
    api.get_devices().await
}

pub async fn get_meter_plus_devices_status(
    device_id: &str,
) -> anyhow::Result<GetDevicesMeterPlusStatusResponse> {
    let api = SwitchBotApiImpl::new();
    api.get_meter_plus_devices_status(device_id).await
}

pub async fn get_plug_mini_devices_status(
    device_id: &str,
) -> anyhow::Result<GetDevicesPlugMiniStatusResponse> {
    let api = SwitchBotApiImpl::new();
    api.get_plug_mini_devices_status(device_id).await
}
