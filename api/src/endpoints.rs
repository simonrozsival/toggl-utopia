use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::responses::{invalid_credentials, something_went_wrong, success};
use crate::sync;

use crate::auth::Credentials;
use crate::models::Delta;

#[derive(Deserialize)]
pub struct SyncRequestBody {
    last_sync: DateTime<Utc>,
    delta: Delta,
}

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

    match sync::fetch_snapshot(&credentials) {
        Ok(delta) => success(delta),
        Err(err) => something_went_wrong(err),
    }
}

pub fn sync((req, sync_req): (HttpRequest, web::Json<SyncRequestBody>)) -> HttpResponse {
    let credentials = match extract_credentials(req) {
        Some(credentials) => credentials,
        None => return invalid_credentials(),
    };

    match sync::update_server_and_calculate_delta_for_client(
        &sync_req.last_sync,
        &sync_req.delta,
        &credentials,
    ) {
        Ok(response_delta) => success(response_delta),
        Err(err) => something_went_wrong(err),
    }
}
