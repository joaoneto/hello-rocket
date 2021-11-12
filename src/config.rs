use std::env;

lazy_static! {
    /// App environment name
    pub static ref APP_ENV: String = env::var("APP_ENV")
        .unwrap_or("development".to_string());

    /// App Http port
    pub static ref APP_PORT: u16 = env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    /// App DB name
    pub static ref APP_DB: String = env::var("APP_DB")
        .unwrap_or("app".to_string());

    /// User token salt
    pub static ref USER_TOKEN_SALT: String = env::var("USER_TOKEN_SALT")
        .expect("USER_TOKEN_SALT must be present in env!");

    /// User password salt
    pub static ref USER_PASSWORD_SALT: String = env::var("USER_PASSWORD_SALT")
        .expect("USER_PASSWORD_SALT must be present in env!");

    /// MongoDB connection string
    pub static ref MONGODB_URL: String = env::var("MONGODB_URL")
        .expect("MONGODB_URL must be present in env!");
}

pub fn load_env() {
    if &*APP_ENV == "production" {
        dotenv::dotenv().ok();
    }
}
