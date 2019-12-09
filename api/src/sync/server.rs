use chrono::{DateTime, Utc};

use crate::auth::Credentials;
use crate::models::{Delta, Project, TimeEntry, User};
use crate::toggl_api;

pub fn fetch_changes_since(
    since: Option<DateTime<Utc>>,
    credentials: &Credentials,
) -> Result<Delta, reqwest::Error> {
    let user: User = toggl_api::user::get(&credentials)?.into();

    let projects: Vec<Project> = toggl_api::projects::get_all(since, &credentials)?
        .into_iter()
        .map(|p| p.into())
        .collect();

    let time_entries: Vec<TimeEntry> = toggl_api::time_entries::get_all(since, &credentials)?
        .into_iter()
        .map(|te| te.into())
        .collect();

    Ok(Delta {
        user: Some(user),
        projects: Some(projects),
        time_entries: Some(time_entries),
    })
}

pub fn overwrite_with(delta: &Delta, credentials: &Credentials) -> Delta {
    unimplemented!()
}
