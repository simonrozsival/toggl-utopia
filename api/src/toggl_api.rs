mod endpoints;
pub mod models;
pub mod projects;
pub mod time_entries;
pub mod user;

use crate::auth::Credentials;
use crate::error::Error;

use endpoints::Endpoint;
use reqwest::{header, Client};

pub struct TogglApi {
    client: Client,
}

impl TogglApi {
    pub fn new(credentials: Credentials) -> Option<TogglApi> {
        let (username, password) = credentials.into_basic();
        let encoded_basic_auth = format!(
            "Basic {}",
            base64::encode(&format!("{}:{}", username, password))
        );

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&encoded_basic_auth).ok()?,
        );

        let client = Client::builder().default_headers(headers).build().ok()?;

        Some(TogglApi { client })
    }

    pub fn req(&self, endpoint: Endpoint) -> reqwest::RequestBuilder {
        self.client.request(endpoint.method, &endpoint.url)
    }

    pub fn validate(res: &mut reqwest::Response) -> Result<(), Error> {
        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::ApiError(
                res.status().as_u16(),
                res.text().unwrap_or("Unknown error.".to_string()),
            ))
        }
    }
}
