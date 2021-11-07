use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::outcome::{Outcome};

use crate::lib::security::{Auth, verify_token};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Auth, Self::Error> {
        if let Some(auth) = extract_auth_from_request(request) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| -> Option<Auth> {
            verify_token(token)
                .map_err(|err| {
                    eprintln!("Auth decode error: {:?}", err);
                })
                .ok()
        })
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with("Bearer ") {
        Some(&header["Bearer ".len()..])
    } else {
        None
    }
}
