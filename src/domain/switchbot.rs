pub mod get_devices;
pub mod get_devices_status;

#[async_trait]
pub trait SwitchBotApi {
    async fn get_devices(&self);
    async fn get_devices_status(&self, device_id: String);
}
