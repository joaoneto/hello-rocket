use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    email: String,
    exp: usize,
}

const EXPIRES_UTC: usize = 10000000000;
const SECRET_SALT: &[u8; 11] = b"SECRET_SALT";

pub fn issue_token(user_id: &String, email: &String) -> Result<String, Error> {
    let claims = Claims { sub: user_id.to_owned(), email: email.to_owned(), exp: EXPIRES_UTC };
    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_SALT)) {
        Ok(t) => t,
        Err(_) => panic!("Error issuing token"),
    };

    Ok(token)
}

pub fn verify_token(token: &String, user_id: &String) -> Result<TokenData<Claims>, Error> {
    let validation = Validation { sub: Some(user_id.to_owned()), ..Validation::default() };
    let token_data = match decode::<Claims>(&token, &DecodingKey::from_secret(SECRET_SALT), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"),
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
            _ => panic!("Some other errors"),
        },
    };

    Ok(token_data)
}