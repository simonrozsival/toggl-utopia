use chrono::{DateTime, Utc};

use super::endpoints;
use super::models::{ApiToken, TimeEntry};

pub fn get_all(
    since: Option<DateTime<Utc>>,
    token: &ApiToken,
) -> Result<Vec<TimeEntry>, reqwest::Error> {
    let time_entries = reqwest::Client::new()
        .get(&endpoints::time_entries(since))
        .basic_auth(&token, Some("api_token"))
        .send()?
        .json::<Vec<TimeEntry>>()?;

    Ok(time_entries)
}
