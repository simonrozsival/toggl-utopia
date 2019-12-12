use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::error::Error;
use crate::models::Delta;
use crate::sync::prelude::SyncOutcome;

#[derive(Serialize)]
struct Meta {
    error: bool,
    utc_server_time: DateTime<Utc>,
    processing_request_took_ms: i64,
}

#[derive(Serialize)]
struct Body<T>
where
    T: Serialize,
{
    meta: Meta,
    payload: T,
}

#[derive(Serialize)]
struct ErrorBody {
    code: u16,
    msg: String,
}

/// Creates a meta structure for the standard body template with the current server time.
fn meta(error: bool, start: DateTime<Utc>) -> Meta {
    Meta {
        error,
        utc_server_time: Utc::now(),
        processing_request_took_ms: Utc::now().signed_duration_since(start).num_milliseconds(),
    }
}

/// Wraps the payload in a standard body template with a correct meta values.
fn ok<T>(payload: T, start: DateTime<Utc>) -> Body<T>
where
    T: Serialize,
{
    Body::<T> {
        meta: meta(false, start),
        payload,
    }
}

/// Creates a body with correct meta and payload for the given error.
fn error(err: Error, start: DateTime<Utc>) -> Body<ErrorBody> {
    let msg = match &err {
        Error::ApiError(_, msg) => msg.clone(),
        Error::NetworkError(reqwest_err) => format!("{:?}", reqwest_err),
    };

    Body {
        meta: meta(true, start),
        payload: ErrorBody {
            code: err.code(),
            msg,
        },
    }
}

pub fn snapshot_success(data: Delta, start: DateTime<Utc>) -> HttpResponse {
    let body = ok(data, start);
    HttpResponse::Ok().json(body)
}

pub fn sync_success(data: SyncOutcome, start: DateTime<Utc>) -> HttpResponse {
    let body = ok(data, start);
    HttpResponse::Ok().json(body)
}

pub fn something_went_wrong(err: Error, start: DateTime<Utc>) -> HttpResponse {
    let body = error(err, start);
    HttpResponse::InternalServerError().json(body)
}

pub fn invalid_credentials(start: DateTime<Utc>) -> HttpResponse {
    let body = error(
        Error::ApiError(401, "The credentials you provided are invalid.".to_string()),
        start,
    );
    HttpResponse::Forbidden().json(body)
}
