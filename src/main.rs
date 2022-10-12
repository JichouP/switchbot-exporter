#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use crossbeam_channel::{bounded, tick, Receiver};
use domain::state::switchbot::SwitchBotState;
use rocket::fairing::AdHoc;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use use_case::scheduler::setup_scheduler;

mod domain;
mod infrastructure;
mod route;
mod service;
mod use_case;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

#[rocket::main]
async fn main() -> Result<(), anyhow::Error> {
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    let switchbot_state = Arc::new(Mutex::new(SwitchBotState::new()));

    let scheduler_handle = setup_scheduler(Arc::clone(&switchbot_state), ctrl_c_events, ticks);

    let rocket_handle = rocket::build()
        .manage(Arc::clone(&switchbot_state))
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

    let (_, _) = futures::future::join(scheduler_handle, rocket_handle).await;

    Ok(())
}
