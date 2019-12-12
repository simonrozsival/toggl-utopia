use chrono::{DateTime, Utc};

use crate::error::Error;
use crate::models::{Delta, Entity, Project, TimeEntry, User};
use crate::sync::prelude::{created, failed, SyncOutcome, SyncResult};
use crate::toggl_api::models::TimeEntry as TogglTimeEntry;
use crate::toggl_api::TogglApi;

pub fn fetch_changes_since(since: Option<DateTime<Utc>>, api: &TogglApi) -> Result<Delta, Error> {
    let user: User = api.fetch_user()?.into();

    let projects: Vec<Project> = api
        .fetch_projects(since)?
        .into_iter()
        .map(|p| p.into())
        .collect();

    let time_entries: Vec<TimeEntry> = api
        .fetch_time_entries(since)?
        .into_iter()
        .filter(|te| since.unwrap_or(te.at) <= te.at) // remove false positives
        .map(|te| te.into())
        .collect();

    Ok(Delta {
        user: Some(user),
        projects: Some(projects),
        time_entries: Some(time_entries),
    })
}

pub fn currently_running_time_entry(api: &TogglApi) -> Result<Option<TimeEntry>, Error> {
    let maybe_te = api.fetch_current_running_time_entry()?;
    Ok(maybe_te.map(|te| te.into()))
}

fn create_time_entry(te: &TimeEntry, api: &TogglApi) -> Option<SyncResult<TimeEntry>> {
    match api.create_time_entry(TogglTimeEntry::from(&te)) {
        Ok(entity) => Some(created(te.id, entity.into())),
        Err(err) => Some(failed(te.id, err)),
    }
}

fn update_time_entry(te: &TimeEntry, api: &TogglApi) -> Option<SyncResult<TimeEntry>> {
    match api.update_time_entry(TogglTimeEntry::from(&te)) {
        Ok(_) => None,
        Err(err) => Some(failed(te.id, err)),
    }
}

fn push_time_entry(entity: &TimeEntry, api: &TogglApi) -> Option<SyncResult<TimeEntry>> {
    if !entity.exists_on_server() {
        return create_time_entry(entity.into(), &api);
    } else {
        return update_time_entry(entity.into(), &api);
    }
}

pub fn apply_changes(delta: Delta, api: &TogglApi) -> SyncOutcome {
    if delta.user.is_some() {
        unimplemented!("We don't support updating projects at this moment.");
    }

    if let Some(projects) = delta.projects {
        if !projects.is_empty() {
            // Important note: we don't support creating projects at the moment.
            // If we did, we would have to update the old IDs in the `time_entries` (assuming
            // the entities would be linked by some client-assigned IDs, such as negative numbers).
            unimplemented!("We don't support updating projects at this moment.");
        }
    }

    let time_entries = delta
        .time_entries
        .unwrap_or_default()
        .into_iter()
        .filter_map(|te| push_time_entry(&te, &api))
        .collect();

    SyncOutcome {
        user: None,
        projects: vec![],
        time_entries,
    }
}
