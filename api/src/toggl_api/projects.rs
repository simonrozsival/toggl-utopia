use chrono::{DateTime, Utc};

use super::endpoints;
use super::models::{ApiToken, Project};

pub fn get_all(
    since: Option<DateTime<Utc>>,
    token: &ApiToken,
) -> Result<Vec<Project>, reqwest::Error> {
    let projects = reqwest::Client::new()
        .get(&endpoints::projects(since))
        .basic_auth(&token, Some("api_token"))
        .send()?
        .json::<Vec<Project>>()?;

    Ok(projects)
}
