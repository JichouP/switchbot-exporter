#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use domain::state::switchbot::SwitchBotState;
use std::sync::{Arc, Mutex};
use use_case::scheduler::setup_scheduler;

mod domain;
mod infrastructure;
mod route;
mod service;
mod use_case;

#[rocket::main]
async fn main() -> Result<(), anyhow::Error> {
    let switch_bot_state = Arc::new(Mutex::new(SwitchBotState::new()));

    let scheduler_handle = setup_scheduler(Arc::clone(&switch_bot_state));

    let rocket_handle = rocket::build()
        .manage(Arc::clone(&switch_bot_state))
        .mount("/", route::status::status::build())
        .launch();

    let (_, rocket_result) = futures::future::join(scheduler_handle, rocket_handle).await;
    let _rocket = rocket_result?;

    Ok(())
}
