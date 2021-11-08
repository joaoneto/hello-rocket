use serde::{Serialize, Deserialize};
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::lib::Storage;
use crate::lib::db::get_users_collection;
use crate::lib::security::{hash_password, issue_token};
use crate::models::user::{UserLogin};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginToken {
    token: String,
}

#[post("/login", format = "json", data = "<user_login>")]
pub async fn create(user_login: Json<UserLogin>, state: &State<Storage>) -> Result<Json<LoginToken>, Status> {
    let email = user_login.email.as_str();
    let hashed_password = hash_password(user_login.password.as_str());

    let user_doc = get_users_collection(&state.mongo)
        .find_one(bson::doc! { "email": email, "password": hashed_password }, None)
        .await
        .expect("Error fetching users");

    if user_doc.is_some() {
        let token = issue_token(email.clone()).unwrap();
        Ok(Json(LoginToken { token }))
    } else {
        Err(Status::Unauthorized)
    }
}