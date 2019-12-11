use serde::Serialize;

use crate::models::{Entity, Project, TimeEntry, User};
use crate::toggl_api::models::Id;
use std::cmp::PartialEq;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub enum ConflictResolution<T: Entity> {
    Ignore,
    Create(T),
    Update(T),
}

impl<T: Entity> ConflictResolution<T> {
    pub fn is_ignore(&self) -> bool {
        match self {
            ConflictResolution::<T>::Ignore => true,
            _ => false,
        }
    }

    pub fn to_option(self) -> Option<ConflictResolution<T>> {
        match self {
            ConflictResolution::<T>::Ignore => None,
            _ => Some(self),
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct ConflictResolutionResults {
    pub user: Option<ConflictResolution<User>>,
    pub projects: Vec<ConflictResolution<Project>>,
    pub time_entries: Vec<ConflictResolution<TimeEntry>>,
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub enum SyncResult<T: Entity> {
    Changed {
        entity: T,
    },
    Created {
        client_assigned_id: Id,
        entity: T,
    },
    Failed {
        client_assigned_id: Id,
        code: u64,
        message: String,
    },
}

pub fn changed<T: Entity>(entity: T) -> SyncResult<T> {
    SyncResult::<T>::Changed { entity }
}

pub fn created<T: Entity>(client_assigned_id: Id, entity: T) -> SyncResult<T> {
    SyncResult::<T>::Created {
        client_assigned_id,
        entity,
    }
}

pub fn failed<T: Entity>(client_assigned_id: Id, message: String) -> SyncResult<T> {
    SyncResult::<T>::Failed {
        client_assigned_id,
        code: 0,
        message,
    }
}

impl<T: Entity> SyncResult<T> {
    pub fn convert(conflict_resolution: ConflictResolution<T>) -> SyncResult<T> {
        match conflict_resolution {
            ConflictResolution::<T>::Create(entity) => SyncResult::<T>::Changed { entity },
            ConflictResolution::<T>::Update(entity) => SyncResult::<T>::Changed { entity },
            ConflictResolution::<T>::Ignore => panic!("'Ignore' cannot be a sync result."),
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct SyncOutcome {
    pub user: Option<SyncResult<User>>,
    pub projects: Vec<SyncResult<Project>>,
    pub time_entries: Vec<SyncResult<TimeEntry>>,
}

impl SyncOutcome {
    pub fn convert(conflict_resolution: ConflictResolutionResults) -> SyncOutcome {
        SyncOutcome {
            user: conflict_resolution.user.map(SyncResult::<User>::convert),
            projects: conflict_resolution
                .projects
                .into_iter()
                .map(SyncResult::<Project>::convert)
                .collect(),
            time_entries: conflict_resolution
                .time_entries
                .into_iter()
                .map(SyncResult::<TimeEntry>::convert)
                .collect(),
        }
    }

    pub fn merge(a: SyncOutcome, b: SyncOutcome) -> SyncOutcome {
        SyncOutcome {
            user: a.user.or(b.user),
            projects: [&a.projects[..], &b.projects[..]].concat(),
            time_entries: [&a.time_entries[..], &b.time_entries[..]].concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    mod sync_outcome {
        use super::super::{create, error, update, SyncOutcome};
        use crate::models::{Project, TimeEntry, User};
        use chrono::Utc;

        fn empty() -> SyncOutcome {
            SyncOutcome {
                user: None,
                projects: vec![],
                time_entries: vec![],
            }
        }

        #[test]
        fn merges_two_empty_resoutions_into_a_single_empty_resolution() {
            let a = empty();
            let b = empty();

            let merged = SyncOutcome::merge(a, b);

            assert!(merged.user.is_none());
            assert!(merged.projects.is_empty());
            assert!(merged.time_entries.is_empty());
        }

        #[test]
        fn merges_a_non_empty_with_an_empty() {
            let a = empty();
            let b = SyncOutcome {
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

            let merged = SyncOutcome::merge(a, b.clone());

            assert_eq!(merged, b);
        }

        #[test]
        fn merges_two_non_overlapping_non_empty() {
            let a = SyncOutcome {
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
            let b = SyncOutcome {
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

            let merged = SyncOutcome::merge(a, b);

            assert!(merged.user.is_some());
            assert_eq!(merged.projects.len(), 2);
            assert_eq!(merged.time_entries.len(), 2);
        }
    }
}
