use super::endpoints;
use super::models::User;
use crate::auth::Credentials;

pub fn get(credentials: &Credentials) -> Result<User, reqwest::Error> {
    let (username, password) = credentials.into_basic();
    let user = reqwest::Client::new()
        .get(&endpoints::me())
        .basic_auth(username, Some(password))
        .send()?
        .json::<User>()?;

    Ok(user)
}
