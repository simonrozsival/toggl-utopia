use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::responses::*;
use crate::toggl_api;

#[derive(Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn login(credentials: web::Json<Credentials>) -> HttpResponse {
    match toggl_api::user::get_api_token(&credentials.email, &credentials.password) {
        Ok(token) => login_succeeded(
            token,
            vec!["there", "will", "be", "real", "data", "I", "swear"]
                .into_iter()
                .map(String::from)
                .collect(),
        ),
        Err(_) => invalid_credentials(),
    }
}
