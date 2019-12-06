use super::endpoints;
use super::models::{ApiToken, User};

pub fn get_api_token(email: &String, password: &String) -> Result<ApiToken, reqwest::Error> {
    let user = reqwest::Client::new()
        .get(endpoints::ME)
        .basic_auth(&email, Some(&password))
        .send()?
        .json::<User>()?;

    Ok(user.api_token)
}
