use serde::Serialize;

use super::utils::share_entities;
use crate::models::{Entity, Project, TimeEntry, User};
use crate::toggl_api::models::Id;
use std::cmp::PartialEq;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub enum ConflictResolution<T: Serialize> {
    Create { entity: T },
    Update { id: Id, entity: T },
    Error { id: Id, code: u64, message: String },
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

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct SyncResolution {
    pub user: Option<ConflictResolution<User>>,
    pub projects: Vec<ConflictResolution<Project>>,
    pub time_entries: Vec<ConflictResolution<TimeEntry>>,
}

impl SyncResolution {
    pub fn merge(a: SyncResolution, b: SyncResolution) -> SyncResolution {
        assert!(!share_entities(&a, &b));

        SyncResolution {
            user: a.user.or(b.user),
            projects: [&a.projects[..], &b.projects[..]].concat(),
            time_entries: [&a.time_entries[..], &b.time_entries[..]].concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod sync_resolution {
        use super::super::{create, error, update, SyncResolution};
        use crate::models::{Project, TimeEntry, User};
        use chrono::Utc;

        fn empty() -> SyncResolution {
            SyncResolution {
                user: None,
                projects: vec![],
                time_entries: vec![],
            }
        }

        #[test]
        fn merges_two_empty_resoutions_into_a_single_empty_resolution() {
            let a = empty();
            let b = empty();

            let merged = SyncResolution::merge(a, b);

            assert!(merged.user.is_none());
            assert!(merged.projects.is_empty());
            assert!(merged.time_entries.is_empty());
        }

        #[test]
        fn merges_a_non_empty_with_an_empty() {
            let a = empty();
            let b = SyncResolution {
                user: Some(update(
                    123,
                    &User {
                        id: 1,
                        default_workspace_id: 0,
                        fullname: "user".to_string(),
                        api_token: "token".to_string(),
                        at: Utc::now(),
                    },
                )),
                projects: vec![create(&Project {
                    id: 2,
                    workspace_id: 0,
                    name: "project".to_string(),
                    color: "#ff0000".to_string(),
                    active: true,
                    at: Utc::now(),
                    server_deleted_at: None,
                })],
                time_entries: vec![error(
                    &TimeEntry {
                        id: 3,
                        workspace_id: 0,
                        description: "description".to_string(),
                        project_id: None,
                        start: Utc::now(),
                        duration: None,
                        at: Utc::now(),
                        server_deleted_at: None,
                    },
                    "error".to_string(),
                )],
            };

            let merged = SyncResolution::merge(a, b.clone());

            assert_eq!(merged, b);
        }

        #[test]
        fn merges_two_non_overlapping_non_empty() {
            let a = SyncResolution {
                user: Some(update(
                    123,
                    &User {
                        id: 1,
                        default_workspace_id: 0,
                        fullname: "user".to_string(),
                        api_token: "token".to_string(),
                        at: Utc::now(),
                    },
                )),
                projects: vec![create(&Project {
                    id: 2,
                    workspace_id: 0,
                    name: "project A".to_string(),
                    color: "#ff0000".to_string(),
                    active: true,
                    at: Utc::now(),
                    server_deleted_at: None,
                })],
                time_entries: vec![error(
                    &TimeEntry {
                        id: 3,
                        workspace_id: 0,
                        description: "description A".to_string(),
                        project_id: None,
                        start: Utc::now(),
                        duration: None,
                        at: Utc::now(),
                        server_deleted_at: None,
                    },
                    "error".to_string(),
                )],
            };
            let b = SyncResolution {
                user: None,
                projects: vec![create(&Project {
                    id: 4,
                    workspace_id: 0,
                    name: "project B".to_string(),
                    color: "#ff0000".to_string(),
                    active: true,
                    at: Utc::now(),
                    server_deleted_at: None,
                })],
                time_entries: vec![error(
                    &TimeEntry {
                        id: 5,
                        workspace_id: 0,
                        description: "description B".to_string(),
                        project_id: None,
                        start: Utc::now(),
                        duration: Some(5),
                        at: Utc::now(),
                        server_deleted_at: None,
                    },
                    "error".to_string(),
                )],
            };

            let merged = SyncResolution::merge(a, b);

            assert!(merged.user.is_some());
            assert_eq!(merged.projects.len(), 2);
            assert_eq!(merged.time_entries.len(), 2);
        }
    }
}
