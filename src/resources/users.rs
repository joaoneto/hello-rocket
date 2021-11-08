use bson::Document;
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::futures::stream::{StreamExt};

use crate::lib::Storage;
use crate::lib::db::get_users_collection;
use crate::lib::security::{Auth, hash_password};
use crate::models::user::{UserRegister, UserRegisterForm, UsersListItem};

/// @TODO: try impl traint for UsersResults::from(user_doc)
fn make_list_user_item(user_doc: Document) -> UsersListItem {
    return UsersListItem {
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
pub async fn list(state: &State<Storage>, _auth: Auth) -> Result<Json<Vec<UsersListItem>>, Status> {
    let collection = get_users_collection(&state.mongo);

    let cursor = collection
        .find(None, None)
        .await
        .expect("Error fetching users");

    let results = cursor
        .map(|user_doc| make_list_user_item(user_doc.unwrap()))
        .collect::<Vec<UsersListItem>>()
        .await;

    Ok(Json(results))
}

#[post("/users/register", format = "json", data = "<user>")]
pub async fn register(user: Json<UserRegisterForm>, state: &State<Storage>) -> Result<Json<UserRegister>, Status> {
    let hashed_password = hash_password(&user.password);
    let mut doc = user.to_document();
    doc.insert("password", bson::Bson::String(hashed_password.to_owned()));

    let insert_result = get_users_collection(&state.mongo)
        .insert_one(doc, None)
        .await
        .expect("Error creating user");

    let result = UserRegister {
        id: insert_result.inserted_id.as_object_id()
            .expect("Error getting inserted_id")
            .to_hex(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
    };

    Ok(Json(result))
}