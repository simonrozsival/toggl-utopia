use serde::Serialize;

use crate::models::{Entity, Project, TimeEntry, User};
use crate::toggl_api::models::Id;
use std::cmp::PartialEq;

#[derive(Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ConflictResolution<T: Serialize> {
    Create { entity: T },
    Update { id: Id, entity: T },
    Error { id: Id, code: u64, message: String },
}

#[derive(Serialize)]
pub struct SyncResolution {
    pub user: Option<ConflictResolution<User>>,
    pub projects: Vec<ConflictResolution<Project>>,
    pub time_entries: Vec<ConflictResolution<TimeEntry>>,
}

pub fn create<T: Entity>(entity: &T) -> ConflictResolution<T> {
    ConflictResolution::<T>::Create {
        entity: entity.clone(),
    }
}

pub fn update<T: Entity>(original_id: Id, entity: &T) -> ConflictResolution<T> {
    ConflictResolution::<T>::Update {
        id: original_id,
        entity: entity.clone(),
    }
}

pub fn error<T: Entity>(entity: &T, message: String) -> ConflictResolution<T> {
    ConflictResolution::<T>::Error {
        code: 0,
        id: entity.id(),
        message: message,
    }
}

impl SyncResolution {
    pub fn merge(a: SyncResolution, b: SyncResolution) -> SyncResolution {
        unimplemented!("Not implemented just yet.");
    }
}
