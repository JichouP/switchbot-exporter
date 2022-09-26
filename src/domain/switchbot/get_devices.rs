#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesResponse {
    status_code: usize,
    body: GetDevicesResponseBody,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetDevicesResponseBody {
    device_list: Vec<GetDevicesResponseBodyDeviceListItem>,
    infrared_remote_list: Vec<GetDevicesResponseBodyInfraredRemoteListItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesResponseBodyDeviceListItem {
    device_id: String,
    device_name: String,
    device_type: String,
    enable_cloud_service: Option<bool>,
    hub_device_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetDevicesResponseBodyInfraredRemoteListItem {
    device_id: String,
    device_name: String,
    remote_type: String,
    hub_device_id: String,
}
