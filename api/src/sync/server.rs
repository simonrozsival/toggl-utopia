use chrono::{DateTime, Utc};

use crate::error::Error;
use crate::models::{Delta, Entity, Project, TimeEntry, User};
use crate::sync::prelude::{created, failed, SyncOutcome, SyncResult};
use crate::toggl_api::models::{Project as TogglProject, TimeEntry as TogglTimeEntry};
use crate::toggl_api::TogglApi;

pub fn fetch_changes_since(since: Option<DateTime<Utc>>, api: &TogglApi) -> Result<Delta, Error> {
    let user: User = api.fetch_user()?.into();

    let projects: Vec<Project> = api
        .fetch_projects(since)?
        .into_iter()
        .filter(|project| since.unwrap_or(project.at) <= project.at) // remove false positives
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

fn create_time_entry(entity: &TimeEntry, api: &TogglApi) -> Option<SyncResult<TimeEntry>> {
    match api.create_time_entry(TogglTimeEntry::from(&entity)) {
        Ok(created_time_entry) => Some(created(entity.id, created_time_entry.into())),
        Err(err) => Some(failed(entity.id, err)),
    }
}

fn update_time_entry(entity: &TimeEntry, api: &TogglApi) -> Option<SyncResult<TimeEntry>> {
    match api.update_time_entry(TogglTimeEntry::from(&entity)) {
        Ok(_) => None,
        Err(err) => Some(failed(entity.id, err)),
    }
}

fn push_time_entry(entity: &TimeEntry, api: &TogglApi) -> Option<SyncResult<TimeEntry>> {
    if !entity.exists_on_server() {
        create_time_entry(entity, &api)
    } else {
        update_time_entry(entity, &api)
    }
}

fn create_project(entity: &Project, api: &TogglApi) -> Option<SyncResult<Project>> {
    match api.create_project(TogglProject::from(&entity)) {
        Ok(created_project) => Some(created(entity.id, created_project.into())),
        Err(err) => Some(failed(entity.id, err)),
    }
}

fn update_project(entity: &Project, api: &TogglApi) -> Option<SyncResult<Project>> {
    match api.update_project(TogglProject::from(&entity)) {
        Ok(_) => None,
        Err(err) => Some(failed(entity.id, err)),
    }
}

fn push_project(entity: &Project, api: &TogglApi) -> Option<SyncResult<Project>> {
    if !entity.exists_on_server() {
        create_project(entity, &api)
    } else {
        update_project(entity, &api)
    }
}

pub fn apply_changes(delta: Delta, api: &TogglApi) -> SyncOutcome {
    use std::collections::HashMap;

    if delta.user.is_some() {
        unimplemented!("We don't support updating projects at this moment.");
    }

    let mut project_id_map = HashMap::new();
    let projects: Vec<_> = delta
        .projects
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entity| push_project(&entity, &api))
        .collect();

    for result in projects.iter() {
        if let SyncResult::<Project>::Created {
            client_assigned_id,
            entity,
        } = result
        {
            project_id_map.insert(client_assigned_id, entity.id);
        }
    }

    let time_entries = delta
        .time_entries
        .unwrap_or_default()
        .into_iter()
        .map(|te| {
            if let Some(id) = te.project_id {
                if let Some(updated_id) = project_id_map.get(&id) {
                    return TimeEntry {
                        project_id: Some(*updated_id),
                        ..te
                    };
                }
            }

            te
        })
        .filter_map(|entity| push_time_entry(&entity, &api))
        .collect();

    SyncOutcome {
        user: None,
        projects,
        time_entries,
    }
}
