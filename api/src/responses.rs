use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::toggl_api::models::ApiToken;

#[derive(Serialize)]
struct Meta {
    error: bool,
    server_time: DateTime<Utc>,
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
struct Error {
    code: u64,
    message: String,
}

fn meta(error: bool) -> Meta {
    Meta {
        error: error,
        utc_server_time: Utc::now(),
    }
}

fn ok<T>(payload: T) -> Body<T>
where
    T: Serialize,
{
    Body::<T> {
        meta: meta(false),
        payload: payload,
    }
}

fn error(code: u64, message: &str) -> Body<Error> {
    Body {
        meta: meta(true),
        payload: Error {
            code: code,
            message: message.into(),
        },
    }
}

pub fn api_token(token: ApiToken) -> HttpResponse {
    let body = ok(token);
    HttpResponse::Ok().json(body)
}

pub fn invalid_credentials() -> HttpResponse {
    let body = error(1, "The credentials you provided are invalid.");
    HttpResponse::Forbidden().json(body)
}
