#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDevicesStatusResponse {
    status_code: usize,
    body: GetDevicesStatusResponseBody,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum GetDevicesStatusResponseBody {
    PlugMini {
        device_id: String,
        device_type: String,
        hub_device_id: String,
        /// ON/OFF state
        power: String,
        /// Unit: V
        voltage: usize,
        /// Unit: W/min
        weight: usize,
        /// Unit: min
        electricity_of_day: usize,
        /// Unit: A
        electric_current: usize,
    },
    MeterPlus {
        device_id: String,
        device_type: String,
        hub_device_id: String,
        /// Unit: %
        humidity: usize,
        /// Unit: C
        temperature: usize,
    },
}
