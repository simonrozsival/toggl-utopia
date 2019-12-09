use chrono::{DateTime, Utc};

use super::endpoints;
use super::models::Project;

use crate::auth::Credentials;

pub fn get_all(
    since: Option<DateTime<Utc>>,
    credentials: &Credentials,
) -> Result<Vec<Project>, reqwest::Error> {
    let (username, password) = credentials.into_basic();
    let projects = reqwest::Client::new()
        .get(&endpoints::projects(since))
        .basic_auth(username, Some(password))
        .send()?
        .json::<Vec<Project>>()?;

    Ok(projects)
}
