use chrono::{DateTime, Utc};

use crate::error::Error;
use crate::models::{Delta, Entity, Project, TimeEntry, User};
use crate::sync::prelude::{created, failed, SyncOutcome, SyncResult};
use crate::toggl_api::{
    endpoints,
    endpoints::CreateOrUpdate,
    models::{Project as TogglProject, TimeEntry as TogglTimeEntry, User as TogglUser},
    TogglApi,
};

pub fn fetch_changes_since(since: Option<DateTime<Utc>>, api: &TogglApi) -> Result<Delta, Error> {
    let user: User = api.fetch(endpoints::user::get())?.into();

    let projects: Vec<Project> = api
        .fetch(endpoints::projects::get(since))?
        .into_iter()
        .filter(|project| since.unwrap_or(project.at) <= project.at) // remove false positives
        .map(|p| p.into())
        .collect();

    let time_entries: Vec<TimeEntry> = api
        .fetch(endpoints::time_entries::get(since))?
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
        .filter_map(|entity| push::<Project, TogglProject>(&api, &entity))
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
        .filter_map(|entity| push::<TimeEntry, TogglTimeEntry>(&api, &entity))
        .collect();

    SyncOutcome {
        user: None,
        projects,
        time_entries,
    }
}

fn push<T, TToggl>(api: &TogglApi, entity: &T) -> Option<SyncResult<T>>
where
    T: Entity + Into<TToggl>,
    TToggl: CreateOrUpdate + Into<T>,
{
    if entity.exists_on_server() {
        update(api, entity)
    } else {
        create(api, entity)
    }
}

fn create<T, TToggl>(api: &TogglApi, entity: &T) -> Option<SyncResult<T>>
where
    T: Entity + Into<TToggl>,
    TToggl: Into<T> + CreateOrUpdate,
{
    match api.create(entity.clone().into()) {
        Ok(res) => Some(created(entity.id(), res.into())),
        Err(err) => Some(failed(entity.id(), err)),
    }
}

fn update<T, TToggl>(api: &TogglApi, entity: &T) -> Option<SyncResult<T>>
where
    T: Entity + Into<TToggl>,
    TToggl: Into<T> + CreateOrUpdate,
{
    match api.update(entity.clone().into()) {
        Ok(_) => None,
        Err(err) => Some(failed(entity.id(), err)),
    }
}
