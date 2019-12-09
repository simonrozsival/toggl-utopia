use actix_web::{web, HttpRequest, HttpResponse};

use crate::responses::{invalid_credentials, login_succeeded, something_went_wrong};
use crate::sync::login::fetch_server_state;

use crate::auth::Credentials;
use crate::models::Delta;

fn extract_credentials(req: HttpRequest) -> Option<Credentials> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(Credentials::decode)
}

pub fn login(req: HttpRequest) -> HttpResponse {
    let credentials = match extract_credentials(req) {
        Some(credentials) => credentials,
        None => return invalid_credentials(),
    };

    match fetch_server_state(&credentials) {
        Ok(delta) => login_succeeded(delta),
        Err(err) => something_went_wrong(err),
    }
}

pub fn sync((req, delta): (HttpRequest, web::Json<Delta>)) -> HttpResponse {
    let credentials = match extract_credentials(req) {
        Some(credentials) => credentials,
        None => return invalid_credentials(),
    };

    unimplemented!("Syncing isn't implemented just yet.");
}
