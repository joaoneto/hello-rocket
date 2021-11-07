use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::lib::security::{issue_token};
use crate::models::user::{UserLogin};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginToken {
    token: String,
}

#[post("/login", format = "json", data="<user>")]
pub fn create(user: Json<UserLogin>) -> Result<Json<LoginToken>, Status> {
    let email = user.email.to_string();

    let token = match issue_token(&email) {
        Ok(t) => t,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(LoginToken { token: token }))
}