use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::Delta;
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
struct Error<T>
where
    T: Serialize,
{
    code: u64,
    error: T,
}

#[derive(Serialize)]
struct LoginResult {
    api_token: ApiToken,
    data: Delta,
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

fn error<T>(code: u64, err: T) -> Body<Error<T>>
where
    T: Serialize,
{
    Body {
        meta: meta(true),
        payload: Error::<T> {
            code: code,
            error: err,
        },
    }
}

pub fn login_succeeded(token: ApiToken, data: Delta) -> HttpResponse {
    let body = ok(LoginResult {
        api_token: token,
        data: data,
    });
    HttpResponse::Ok().json(body)
}

pub fn something_went_wrong<E>(err: E) -> HttpResponse
where
    E: std::error::Error + std::fmt::Debug,
{
    let body = error(1, format!("{:?}", err));
    HttpResponse::InternalServerError().json(body)
}

pub fn invalid_credentials() -> HttpResponse {
    let body = error(2, "The credentials you provided are invalid.");
    HttpResponse::Forbidden().json(body)
}
