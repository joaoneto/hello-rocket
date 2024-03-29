use jsonwebtoken::errors::{Error};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use sha2::Sha256;
use hmac::{Hmac, Mac};

use crate::config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub sub: String,
    pub exp: i64,
}

pub fn issue_token(id: &str) -> Result<String, Error> {
    let exp = Utc::now() + Duration::days(60);
    let auth = Auth { sub: id.to_owned(), exp: exp.timestamp() };
    let token = encode(&Header::default(), &auth, &EncodingKey::from_secret(&*config::USER_TOKEN_SALT.as_bytes()))?;

    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Auth, Error> {
    let token_data = decode::<Auth>(&token, &DecodingKey::from_secret(&*config::USER_TOKEN_SALT.as_bytes()), &Validation::new(Algorithm::HS256))?;

    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> String {
    let mut mac: Hmac<Sha256> = Hmac::new_from_slice(password.as_bytes()).expect("HMAC can take key of any size");
    mac.update(&*config::USER_PASSWORD_SALT.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    let mut hashed_password = String::new();
    
    for byte in code_bytes.iter() {
        hashed_password.push_str(&format!("{:02x}", byte));
    }
    
    hashed_password
}
