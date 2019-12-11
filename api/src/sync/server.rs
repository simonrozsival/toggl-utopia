use chrono::{DateTime, Utc};

use crate::models::{Delta, Project, TimeEntry, User};
use crate::sync::prelude::{
    error, update, ConflictResolution,
    ConflictResolution::{Create, Update},
    SyncResolution,
};
use crate::toggl_api::models::TimeEntry as TogglTimeEntry;
use crate::toggl_api::TogglApi;

pub fn fetch_changes_since(
    since: Option<DateTime<Utc>>,
    api: &TogglApi,
) -> Result<Delta, reqwest::Error> {
    let user: User = api.fetch_user()?.into();

    let projects: Vec<Project> = api
        .fetch_projects(since)?
        .into_iter()
        .map(|p| p.into())
        .collect();

    let time_entries: Vec<TimeEntry> = api
        .fetch_time_entries(since)?
        .into_iter()
        .map(|te| te.into())
        .collect();

    Ok(Delta {
        user: Some(user),
        projects: Some(projects),
        time_entries: Some(time_entries),
    })
}

fn create_time_entry(te: &TimeEntry, api: &TogglApi) -> Option<ConflictResolution<TimeEntry>> {
    match api.create_time_entry(&TogglTimeEntry::from(&te)) {
        Ok(entity) => Some(update(te.id, &entity.into())),
        Err(err) => Some(error(&te, format!("{:?}", err))),
    }
}

fn update_time_entry(te: &TimeEntry, api: &TogglApi) -> Option<ConflictResolution<TimeEntry>> {
    match api.update_time_entry(&TogglTimeEntry::from(&te)) {
        Ok(_) => None,
        Err(err) => Some(error(&te, format!("{:?}", err))),
    }
}

fn push_time_entry(
    change: &ConflictResolution<TimeEntry>,
    api: &TogglApi,
) -> Option<ConflictResolution<TimeEntry>> {
    match change {
        Create { entity } => create_time_entry(&entity.clone().into(), &api),
        Update { entity, .. } => update_time_entry(&entity.clone().into(), &api),
        _ => {
            panic!("This should not happen, only Create and Update are supported for time entries!")
        }
    }
}

pub fn apply_changes(resolution: SyncResolution, api: &TogglApi) -> SyncResolution {
    if resolution.user.is_some() {
        unimplemented!("We don't support updating projects at this moment.");
    }
    if !resolution.projects.is_empty() {
        // Important note: we don't support creating projects at the moment.
        // If we did, we would have to update the old IDs in the `time_entries` (assuming
        // the entities would be linked by some client-assigned IDs, such as negative numbers).
        unimplemented!("We don't support updating projects at this moment.");
    }

    let mut time_entries = vec![];
    for change in &resolution.time_entries {
        if let Some(response) = push_time_entry(&change, &api) {
            time_entries.push(response);
        }
    }

    SyncResolution {
        user: None,
        projects: vec![],
        time_entries,
    }
}
