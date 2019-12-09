use crate::auth::Credentials;
use crate::models::{Delta, Project, TimeEntry, User};
use crate::toggl_api;

pub fn fetch_server_state(credentials: &Credentials) -> Result<Delta, reqwest::Error> {
    let user: User = toggl_api::user::get(&credentials)?.into();

    let projects: Vec<Project> = toggl_api::projects::get_all(None, &credentials)?
        .into_iter()
        .map(|p| p.into())
        .collect();

    let time_entries: Vec<TimeEntry> = toggl_api::time_entries::get_all(None, &credentials)?
        .into_iter()
        .map(|te| te.into())
        .collect();

    Ok(Delta {
        user: Some(user),
        projects: Some(projects),
        time_entries: Some(time_entries),
    })
}
