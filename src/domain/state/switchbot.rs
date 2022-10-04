use crate::domain::switchbot::get_devices_status::{
    GetDevicesMeterPlusStatusResponse, GetDevicesMeterPlusStatusResponseBody,
    GetDevicesPlugMiniStatusResponse, GetDevicesPlugMiniStatusResponseBody,
};

#[derive(Debug, Clone, Serialize)]
pub struct SwitchBotState {
    pub meter_plus: MeterPlusState,
    pub plug_mini_pc: PlugMiniState,
    pub plug_mini_desk: PlugMiniState,
}

impl SwitchBotState {
    pub fn new() -> Self {
        Self {
            meter_plus: MeterPlusState::default(),
            plug_mini_pc: PlugMiniState::default(),
            plug_mini_desk: PlugMiniState::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MeterPlusState {
    pub humidity: usize,
    pub temperature: f64,
}

impl From<GetDevicesMeterPlusStatusResponse> for MeterPlusState {
    fn from(v: GetDevicesMeterPlusStatusResponse) -> Self {
        let GetDevicesMeterPlusStatusResponseBody {
            humidity,
            temperature,
            ..
        } = v.body;

        Self {
            humidity,
            temperature,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PlugMiniState {
    pub power: String,
    pub voltage: f64,
    pub weight: f64,
    pub electricity_of_day: usize,
    pub electric_current: f64,
}

impl From<GetDevicesPlugMiniStatusResponse> for PlugMiniState {
    fn from(v: GetDevicesPlugMiniStatusResponse) -> Self {
        let GetDevicesPlugMiniStatusResponseBody {
            power,
            voltage,
            weight,
            electricity_of_day,
            electric_current,
            ..
        } = v.body;

        Self {
            power,
            voltage,
            weight,
            electricity_of_day,
            electric_current,
        }
    }
}
