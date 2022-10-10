#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use domain::state::switchbot::SwitchBotState;
use rocket::fairing::AdHoc;
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
        .attach(AdHoc::on_liftoff("Liftoff Printer", |rocket| {
            Box::pin(async {
                println!(
                    "Server is running on {}:{}",
                    rocket.config().address,
                    rocket.config().port
                );
            })
        }))
        .mount("/status", route::status::status::build())
        .mount("/metrics", route::exporter::exporter::build())
        .launch();

    let (_, rocket_result) = futures::future::join(scheduler_handle, rocket_handle).await;
    let _rocket = rocket_result?;

    println!("{}", _rocket.config().port);

    Ok(())
}
