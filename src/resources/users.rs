use bson::Document;
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::futures::stream::{StreamExt};

use crate::models::user::{ListUsersItem};
use crate::lib::{Storage};
use crate::lib::security::Auth;

/// @TODO: try impl traint for UsersResults::from(user_doc)
fn make_list_user_item(user_doc: Document) -> ListUsersItem {
    return ListUsersItem {
        id: user_doc
            .get_object_id("_id")
            .expect("Error getting _id")
            .to_hex(),
        name: user_doc
            .get_str("name")
            .ok()
            .and_then(|m| Some(m.to_owned())),
        email: user_doc
            .get_str("email")
            .ok()
            .and_then(|m| Some(m.to_string())),
    };
}

#[get("/users")]
pub async fn list(state: &State<Storage>, _auth: Auth) -> Result<Json<Vec<ListUsersItem>>, Status> {
    let collection = state.mongo
        .database("hello_rocket")
        .collection::<Document>("users");

    let cursor = collection
        .find(None, None)
        .await
        .expect("Error fetching users");

    let results = cursor
        .map(|user_doc| make_list_user_item(user_doc.unwrap()))
        .collect::<Vec<ListUsersItem>>()
        .await;

    Ok(Json(results))
}