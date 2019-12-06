use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: u8,
    pub message: String,
}

pub fn invalid_credentials() -> HttpResponse {
    HttpResponse::Forbidden().json(ErrorBody {
        code: 1,
        message: "The credentials you provided are invalid.".into(),
    })
}
