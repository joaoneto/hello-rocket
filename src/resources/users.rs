use mongodb::{
    bson,
    bson::{doc},
};
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::futures::stream::{StreamExt};

use crate::models::user::{User};
use crate::lib::{Storage};

#[get("/users")]
pub async fn all(state: &State<Storage>) -> Result<Json<Vec<User>>, Status> {
    let collection = state.mongo
        .database("hello_rocket")
        .collection("users");

    let mut cursor = collection
        .find(None, None)
        .await
        .expect("Error");

    let mut results: Vec<User> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => results.push(result_model),
                Err(_) => return Err(Status::InternalServerError),
            },
            Err(_) => return Err(Status::InternalServerError),
        }
    }

    Ok(Json(results))
}