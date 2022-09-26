#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesMeterPlusStatusResponse {
    status_code: usize,
    body: GetDevicesMeterPlusStatusResponseBody,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetDevicesMeterPlusStatusResponseBody {
    device_id: String,
    device_type: String,
    hub_device_id: String,
    /// Unit: %
    humidity: usize,
    /// Unit: C
    temperature: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesPlugMiniStatusResponse {
    status_code: usize,
    body: GetDevicesPlugMiniStatusResponseBody,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetDevicesPlugMiniStatusResponseBody {
    device_id: String,
    device_type: String,
    hub_device_id: String,
    /// ON/OFF state
    power: String,
    /// Unit: V
    voltage: f64,
    /// Unit: W/min
    weight: f64,
    /// Unit: min
    electricity_of_day: usize,
    /// Unit: A
    electric_current: f64,
}
