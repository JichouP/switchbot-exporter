#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use anyhow;
use domain::state::switchbot::SwitchBotState;
use futures;
use std::sync::{Arc, Mutex};
use use_case::scheduler::setup_scheduler;

mod domain;
mod infrastructure;
mod service;
mod use_case;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[rocket::main]
async fn main() -> Result<(), anyhow::Error> {
    let switch_bot_state = Arc::new(Mutex::new(SwitchBotState::new()));

    let scheduler_handle = setup_scheduler(Arc::clone(&switch_bot_state));

    let _rocket = rocket::build()
        .manage(Arc::clone(&switch_bot_state))
        .mount("/", routes![hello])
        .launch();

    let (_, rocket_result) = futures::future::join(scheduler_handle, _rocket).await;
    let _rocket = rocket_result?;

    Ok(())
}
