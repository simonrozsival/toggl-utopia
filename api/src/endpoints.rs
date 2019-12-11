use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::responses::{invalid_credentials, snapshot_success, something_went_wrong, sync_success};
use crate::sync;

use crate::auth::Credentials;
use crate::models::Delta;
use crate::toggl_api::TogglApi;

#[derive(Deserialize)]
pub struct SyncRequestBody {
    last_sync: DateTime<Utc>,
    delta: Option<Delta>,
}

fn create_api(req: HttpRequest) -> Option<TogglApi> {
    let credentials = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(Credentials::decode)?;

    TogglApi::new(credentials)
}

pub fn login(req: HttpRequest) -> HttpResponse {
    let start = Utc::now();

    let api = match create_api(req) {
        Some(api) => api,
        None => return invalid_credentials(start),
    };

    match sync::fetch_snapshot(&api) {
        Ok(delta) => snapshot_success(delta, start),
        Err(err) => something_went_wrong(err, start),
    }
}

pub fn sync((req, sync_req): (HttpRequest, web::Json<SyncRequestBody>)) -> HttpResponse {
    let start = Utc::now();
    let SyncRequestBody { last_sync, delta } = sync_req.into_inner();

    let api = match create_api(req) {
        Some(api) => api,
        None => return invalid_credentials(start),
    };

    match sync::update_server_and_calculate_delta_for_client(last_sync, delta, &api) {
        Ok(result) => sync_success(result, start),
        Err(err) => something_went_wrong(err, start),
    }
}
