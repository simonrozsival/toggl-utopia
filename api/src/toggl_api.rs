pub mod endpoints;
pub mod models;
pub mod time_entries;

use crate::auth::Credentials;
use crate::error::Error;

use endpoints::{CreateOrUpdate, Endpoint};

use reqwest::{header, Client};
use serde::{de::DeserializeOwned, Serialize};

pub struct TogglApi {
    pub client: Client,
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

    pub fn fetch<T>(&self, endpoint: Endpoint<T>) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned,
    {
        match endpoint {
            Endpoint::<T>::Get(_) => self.make_request(endpoint),
            _ => panic!("Fetch requires a GET endpoint."),
        }
    }

    pub fn create<T>(&self, entity: T) -> Result<T, Error>
    where
        T: CreateOrUpdate + Serialize + DeserializeOwned,
    {
        let endpoint = CreateOrUpdate::create(entity);
        self.make_request(endpoint)
    }

    pub fn update<T>(&self, entity: T) -> Result<T, Error>
    where
        T: CreateOrUpdate + Serialize + DeserializeOwned,
    {
        let endpoint = CreateOrUpdate::update(entity);
        self.make_request(endpoint)
    }

    fn make_request<T>(&self, endpoint: Endpoint<T>) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let req = match endpoint {
            Endpoint::<T>::Get(url) => self.client.get(&url),
            Endpoint::<T>::Post(url, entity) => self.client.post(&url).json(&entity),
            Endpoint::<T>::Put(url, entity) => self.client.put(&url).json(&entity),
        };
        let mut res = req.send()?;

        TogglApi::validate(&mut res)?;

        Ok(res.json::<T>()?)
    }

    fn validate(res: &mut reqwest::Response) -> Result<(), Error> {
        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::ApiError(
                res.status().as_u16(),
                res.text().unwrap_or_else(|_| "Unknown error.".to_string()),
            ))
        }
    }
}
