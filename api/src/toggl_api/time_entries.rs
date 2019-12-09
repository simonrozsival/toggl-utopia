use chrono::{DateTime, Utc};

use super::endpoints;
use super::models::TimeEntry;
use crate::auth::Credentials;

pub fn get_all(
    since: Option<DateTime<Utc>>,
    credentials: &Credentials,
) -> Result<Vec<TimeEntry>, reqwest::Error> {
    let (username, password) = credentials.into_basic();
    let time_entries = reqwest::Client::new()
        .get(&endpoints::time_entries(since))
        .basic_auth(username, Some(password))
        .send()?
        .json::<Vec<TimeEntry>>()?;

    Ok(time_entries)
}
