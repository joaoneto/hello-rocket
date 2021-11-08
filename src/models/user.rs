use bson::Document;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsersListItem {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegister {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl UserRegisterForm {
    pub fn to_document(&self) -> Document {
        bson::doc! {
            "name": Some(self.name.clone()),
            "email": Some(self.email.clone()),
            "password": Some(self.password.clone()),
        }
    }
}