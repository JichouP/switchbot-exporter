#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use anyhow;
use use_case::switchbot::getDevices;

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
    let res = getDevices().await?;
    println!("{:?}", res);

    let _rocket = rocket::build().mount("/", routes![hello]).launch().await?;

    Ok(())
}
