use std::sync::{Arc, Mutex};

use tokio::task::JoinHandle;

use crate::{
    domain::state::switchbot::SwitchBotState,
    infrastructure::env::switchbot::SwitchBotEnv,
    service::scheduler::setup,
    use_case::switchbot::{get_meter_plus_devices_status, get_plug_mini_devices_status},
};

pub async fn setup_scheduler(switch_bot_state: Arc<Mutex<SwitchBotState>>) -> JoinHandle<()> {
    let switch_bot_state = Arc::clone(&switch_bot_state);

    let f = move || {
        let switch_bot_state = Arc::clone(&switch_bot_state);
        async move {
            let SwitchBotEnv {
                meter_plus_device_id,
                plug_mini_pc_device_id,
                plug_mini_desk_device_id,
                ..
            } = SwitchBotEnv::new();

            // Meter Plus
            {
                let response = get_meter_plus_devices_status(&meter_plus_device_id).await;

                if let Ok(meter_plus) = response {
                    switch_bot_state.lock().unwrap().meter_plus = meter_plus.into();
                }
            }
            // Plug Mini PC
            {
                let response = get_plug_mini_devices_status(&plug_mini_pc_device_id).await;

                if let Ok(plug_mini) = response {
                    switch_bot_state.lock().unwrap().plug_mini_pc = plug_mini.into();
                }
            }
            // Plug Mini Desk
            {
                let response = get_plug_mini_devices_status(&plug_mini_desk_device_id).await;

                if let Ok(plug_mini) = response {
                    switch_bot_state.lock().unwrap().plug_mini_desk = plug_mini.into();
                }
            }
        }
    };

    f().await;

    setup(f).await
}
