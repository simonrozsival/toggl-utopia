use super::endpoints;
use super::models::{ApiToken, User};

pub fn login(email: &String, password: &String) -> Result<User, reqwest::Error> {
    let user = reqwest::Client::new()
        .get(&endpoints::me())
        .basic_auth(&email, Some(&password))
        .send()?
        .json::<User>()?;

    Ok(user)
}

pub fn get_user(token: &ApiToken) -> Result<User, reqwest::Error> {
    let user = reqwest::Client::new()
        .get(&endpoints::me())
        .basic_auth(&token, Some("api_token"))
        .send()?
        .json::<User>()?;

    Ok(user)
}
