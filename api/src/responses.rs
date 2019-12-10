use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::Delta;
use crate::sync::prelude::SyncResolution;

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
struct Error<T>
where
    T: Serialize,
{
    code: u64,
    error: T,
}

/// Creates a meta structure for the standard body template with the current server time.
fn meta(error: bool, start: DateTime<Utc>) -> Meta {
    Meta {
        error: error,
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
        payload: payload,
    }
}

/// Creates a body with correct meta and payload for the given error.
fn error<T>(code: u64, err: T, start: DateTime<Utc>) -> Body<Error<T>>
where
    T: Serialize,
{
    Body {
        meta: meta(true, start),
        payload: Error::<T> {
            code: code,
            error: err,
        },
    }
}

pub fn snapshot_success(data: Delta, start: DateTime<Utc>) -> HttpResponse {
    let body = ok(data, start);
    HttpResponse::Ok().json(body)
}

pub fn sync_success(data: SyncResolution, start: DateTime<Utc>) -> HttpResponse {
    let body = ok(data, start);
    HttpResponse::Ok().json(body)
}

pub fn something_went_wrong<E>(err: E, start: DateTime<Utc>) -> HttpResponse
where
    E: std::error::Error + std::fmt::Debug,
{
    let body = error(1, format!("{:?}", err), start);
    HttpResponse::InternalServerError().json(body)
}

pub fn invalid_credentials(start: DateTime<Utc>) -> HttpResponse {
    let body = error(2, "The credentials you provided are invalid.", start);
    HttpResponse::Forbidden().json(body)
}
