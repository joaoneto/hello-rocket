#[macro_use] extern crate rocket;

mod lib;
mod models;
mod resources;
mod guards;

use std::env;
use lib::{db, Storage};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn load_env() {
    let mode = env::var("MODE")
        .unwrap_or("development".to_string());
    if mode != "production" {
        dotenv::dotenv().ok();
    }
}

#[launch]
async fn rocket() -> _ {
    load_env();
    let port: u16 = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
        .unwrap();
    let mongo = db::get_connection().await;
    let storage = Storage { mongo: mongo.clone() };
    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .manage(storage)
        .mount("/", routes![
            index,
            resources::users::list,
            resources::login::create,
        ])
}