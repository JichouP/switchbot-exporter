#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesResponse {
    status_code: usize,
    body: DevicesResponseBody,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesResponseBody {
    device_list: Vec<DevicesResponseBodyDeviceListItem>,
    infrared_remote_list: Vec<DevicesResponseBodyInfraredRemoteListItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesResponseBodyDeviceListItem {
    device_id: String,
    device_name: String,
    device_type: String,
    enable_cloud_service: Option<bool>,
    hub_device_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesResponseBodyInfraredRemoteListItem {
    device_id: String,
    device_name: String,
    remote_type: String,
    hub_device_id: String,
}
