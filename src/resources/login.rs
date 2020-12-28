
#[post("/user", format = "application/json", data="<user>")]
pub fn create(user: Json<meta::user::Post>) -> JsonValue {
    
}