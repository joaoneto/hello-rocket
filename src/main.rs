#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod lib;
mod models;
mod resources;
mod guards;
mod config;

use lib::{db, Storage};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    config::load_env();

    let mongo = db::get_connection().await;
    let storage = Storage { mongo: mongo.clone() };
    let figment = rocket::Config::figment()
        .merge(("port", *config::APP_PORT))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .manage(storage)
        .mount("/", routes![
            index,
            resources::users::list,
            resources::users::register,
            resources::login::create,
        ])
}