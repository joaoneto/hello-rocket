#[macro_use] extern crate rocket;

mod lib;
mod models;
mod resources;

use lib::{db, Storage};
// use rocket::State;
// use mongodb::Client;



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::error::Error> {
    let mongo = db::get_connection().await;
    let storage = Storage { mongo: mongo.clone() };
    rocket::ignite()
        .manage(storage)
        .mount("/", routes![
            index,
            resources::users::all,
        ])
        .launch()
        .await?;

    Ok(())
}