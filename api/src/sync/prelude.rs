use serde::Serialize;

use crate::models::{Project, Resolve, TimeEntry, User};
use crate::toggl_api::models::Id;
use std::cmp::PartialEq;

#[derive(Serialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum ConflictResolution<T: Serialize> {
    Keep { id: Id },
    Create { entity: T },
    Update { id: Id, entity: T },
    Error { id: Id, code: u64, message: String },
}

#[derive(Serialize)]
pub struct SyncResolution {
    pub user: Option<ConflictResolution<User>>,
    pub projects: Option<Vec<ConflictResolution<Project>>>,
    pub time_entries: Option<Vec<ConflictResolution<Project>>>,
}

pub fn keep<T: Resolve + Serialize>(entity: &T) -> ConflictResolution<T> {
    ConflictResolution::<T>::Keep { id: entity.id() }
}

pub fn create<T: Clone + Resolve + Serialize>(entity: &T) -> ConflictResolution<T> {
    ConflictResolution::<T>::Create {
        entity: entity.clone(),
    }
}

pub fn update<T: Clone + Resolve + Serialize>(
    original_id: Id,
    entity: &T,
) -> ConflictResolution<T> {
    ConflictResolution::<T>::Update {
        id: original_id,
        entity: entity.clone(),
    }
}

pub fn error<T: Resolve + Serialize>(entity: &T, message: String) -> ConflictResolution<T> {
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