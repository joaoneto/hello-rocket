use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub sub: String,
    pub exp: i64,
}

/// @TODO  move to config
const SECRET_SALT: &[u8; 11] = b"SECRET_SALT";

pub fn issue_token(id: &str) -> Result<String, Error> {
    let exp = Utc::now() + Duration::days(60);
    let auth = Auth { sub: id.to_owned(), exp: exp.timestamp() };
    let token = match encode(&Header::default(), &auth, &EncodingKey::from_secret(SECRET_SALT)) {
        Ok(t) => t,
        Err(_) => panic!("Error issuing token"),
    };

    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Auth, Error> {
    let token_data = match decode::<Auth>(&token, &DecodingKey::from_secret(SECRET_SALT), &Validation::new(Algorithm::HS256)) {
        Ok(data) => data.claims,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"),
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
            _ => panic!("Some other errors"),
        },
    };

    Ok(token_data)
}