use chrono::{DateTime, Utc};

use crate::auth::Credentials;
use crate::models::{Delta, Project, TimeEntry, User};
use crate::sync::prelude::SyncResolution;
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

pub fn apply_changes(resolution: SyncResolution, credentials: &Credentials) -> SyncResolution {
    // Important note: we don't support creating projects at the moment.
    // If we did, we would have to update the old IDs in the `time_entries` (assuming
    // the entities would be linked by some client-assigned IDs, such as negative numbers).

    if resolution.user.is_some() {
        unimplemented!("We don't support updating projects at this moment.");
    }

    if !resolution.projects.is_empty() {
        unimplemented!("We don't support updating projects at this moment.");
    }

    if !resolution.time_entries.is_empty() {
        // todo: update the time entries
        // don't forget to correctly mark how to swap the IDs correctly
    }

    SyncResolution {
        user: None,
        projects: vec![],
        time_entries: vec![],
    }
}
