use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::lib::security::{issue_token};
use crate::models::user::{User};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginToken {
    token: String,
}

#[post("/login", format = "application/json", data="<user>")]
pub fn create(user: Json<User>) -> Result<Json<LoginToken>, Status> {
    let user_id = user.name.as_ref().unwrap().to_string();
    let email = user.email.as_ref().unwrap();

    let token = match issue_token(&user_id, &email) {
        Ok(t) => t,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(LoginToken { token: token }))
}