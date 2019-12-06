use serde::{Deserialize, Serialize};

pub type ApiToken = String;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub api_token: ApiToken,
}
