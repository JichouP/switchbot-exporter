#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesMeterPlusStatusResponse {
    pub status_code: usize,
    pub body: GetDevicesMeterPlusStatusResponseBody,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesMeterPlusStatusResponseBody {
    pub device_id: String,
    pub device_type: String,
    pub hub_device_id: String,
    /// Unit: %
    pub humidity: usize,
    /// Unit: C
    pub temperature: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesPlugMiniStatusResponse {
    pub status_code: usize,
    pub body: GetDevicesPlugMiniStatusResponseBody,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesPlugMiniStatusResponseBody {
    pub device_id: String,
    pub device_type: String,
    pub hub_device_id: String,
    /// ON/OFF state
    pub power: String,
    /// Unit: V
    pub voltage: f64,
    /// Unit: W/min
    pub weight: f64,
    /// Unit: min
    pub electricity_of_day: usize,
    /// Unit: A
    pub electric_current: f64,
}
