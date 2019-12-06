use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::toggl_api::models::ApiToken;

#[derive(Serialize)]
struct Meta {
    error: bool,
    utc_server_time: DateTime<Utc>,
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

#[derive(Serialize)]
struct LoginResult {
    api_token: ApiToken,
    data: Vec<String>,
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

pub fn login_succeeded(token: ApiToken, data: Vec<String>) -> HttpResponse {
    let body = ok(LoginResult {
        api_token: token,
        data: data,
    });
    HttpResponse::Ok().json(body)
}

pub fn invalid_credentials() -> HttpResponse {
    let body = error(1, "The credentials you provided are invalid.");
    HttpResponse::Forbidden().json(body)
}
