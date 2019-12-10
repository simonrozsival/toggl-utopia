use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::responses::{invalid_credentials, snapshot_success, something_went_wrong, sync_success};
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
    let start = Utc::now();

    let credentials = match extract_credentials(req) {
        Some(credentials) => credentials,
        None => return invalid_credentials(start),
    };

    match sync::fetch_snapshot(&credentials) {
        Ok(delta) => snapshot_success(delta, start),
        Err(err) => something_went_wrong(err, start),
    }
}

pub fn sync((req, sync_req): (HttpRequest, web::Json<SyncRequestBody>)) -> HttpResponse {
    let start = Utc::now();
    let SyncRequestBody { last_sync, delta } = sync_req.into_inner();

    let credentials = match extract_credentials(req) {
        Some(credentials) => credentials,
        None => return invalid_credentials(start),
    };

    match sync::update_server_and_calculate_delta_for_client(last_sync, delta, &credentials) {
        Ok(sync_resolution) => sync_success(sync_resolution, start),
        Err(err) => something_went_wrong(err, start),
    }
}
