#![allow(proc_macro_derive_resolution_fallback)]

use serde::{Serialize, Deserialize};
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
