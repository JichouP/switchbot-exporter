use crate::domain::state::switchbot::{MeterPlusState, PlugMiniState, SwitchBotState};
use prometheus::{Gauge, Opts, Registry, TextEncoder};

pub struct SwitchBotMetrics(pub String);

impl From<&SwitchBotState> for SwitchBotMetrics {
    fn from(state: &SwitchBotState) -> Self {
        let SwitchBotState {
            meter_plus:
                MeterPlusState {
                    humidity,
                    temperature,
                },
            plug_mini_pc:
                PlugMiniState {
                    electric_current: pc_electric_current,
                    electricity_of_day: pc_electricity_of_day,
                    power: pc_power,
                    voltage: pc_voltage,
                    weight: pc_weight,
                },
            plug_mini_desk:
                PlugMiniState {
                    electric_current: desk_electric_current,
                    electricity_of_day: desk_electricity_of_day,
                    power: desk_power,
                    voltage: desk_voltage,
                    weight: desk_weight,
                },
        } = state;

        let registry = Registry::new();

        let gauges = vec![
            ("humidity", "humidity percentage (%)", humidity),
            ("temperature", "temperature in celsius (°C)", temperature),
            (
                "pc_electric_current",
                "the current of the device at the moment, measured in Amp (A)",
                pc_electric_current,
            ),
            (
                "pc_electricity_of_day",
                "the duration that the device has been used during a day, measured in minutes (min)",
                pc_electricity_of_day,
            ),
            (
                "pc_power",
                "ON/OFF state",
                if pc_power == "on" { &1.0 } else { &0.0 },
            ),
            ("pc_voltage", "the voltage of the device, measured in Volt (V)", pc_voltage),
            ("pc_weight", "the power consumed in a day, measured in Watts (W)", pc_weight),
            (
                "desk_electric_current",
                "the current of the device at the moment, measured in Amp (A)",
                desk_electric_current,
            ),
            (
                "desk_electricity_of_day",
                "the duration that the device has been used during a day, measured in minutes (min)",
                desk_electricity_of_day,
            ),
            (
                "desk_power",
                "ON/OFF state",
                if desk_power == "on" { &1.0 } else { &0.0 },
            ),
            ("desk_voltage", "the voltage of the device, measured in Volt (V)", desk_voltage),
            ("desk_weight", "the power consumed in a day, measured in Watts (W)", desk_weight),
        ];

        gauges.into_iter().for_each(|(name, help, value)| {
            let gauge = Gauge::with_opts(Opts::new(name, help)).unwrap();
            gauge.set(value.clone());
            registry.register(Box::new(gauge)).unwrap();
        });

        let encoder = TextEncoder::new();
        let metric_families = registry.gather();
        let metrics = encoder.encode_to_string(&metric_families).unwrap();

        Self(metrics)
    }
}
