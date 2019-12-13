use serde::Serialize;

use crate::error::Error;
use crate::models::{Delta, Entity, Project, TimeEntry, User};
use crate::toggl_api::models::Id;
use std::cmp::PartialEq;

#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum SyncResult<T: Entity> {
    Changed(T),
    Created {
        client_assigned_id: Id,
        entity: T,
    },
    Failed {
        client_assigned_id: Id,
        code: u16,
        message: String,
    },
}

pub fn changed<T: Entity>(entity: T) -> SyncResult<T> {
    SyncResult::<T>::Changed(entity)
}

pub fn created<T: Entity>(client_assigned_id: Id, entity: T) -> SyncResult<T> {
    SyncResult::<T>::Created {
        client_assigned_id,
        entity,
    }
}

pub fn failed<T: Entity>(client_assigned_id: Id, err: Error) -> SyncResult<T> {
    let (code, message) = match err {
        Error::ApiError(code, message) => (code, message),
        Error::NetworkError(inner) => (1, format!("{:#?}", inner)),
    };

    SyncResult::<T>::Failed {
        client_assigned_id,
        code,
        message,
    }
}

impl<T: Entity> SyncResult<T> {
    pub fn from(entity: T) -> SyncResult<T> {
        SyncResult::<T>::Changed(entity)
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct SyncOutcome {
    pub user: Option<SyncResult<User>>,
    pub projects: Vec<SyncResult<Project>>,
    pub time_entries: Vec<SyncResult<TimeEntry>>,
}

impl SyncOutcome {
    pub fn convert(delta: Delta) -> SyncOutcome {
        SyncOutcome {
            user: delta.user.map(SyncResult::<User>::from),
            projects: delta
                .projects
                .unwrap_or_default()
                .into_iter()
                .map(SyncResult::<Project>::from)
                .collect(),
            time_entries: delta
                .time_entries
                .unwrap_or_default()
                .into_iter()
                .map(SyncResult::<TimeEntry>::from)
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

    pub fn without_unchanged(&self, known_changes: Delta) -> SyncOutcome {
        SyncOutcome {
            user: self.user.clone(),
            projects: known_changes
                .projects
                .map(|known_projects| {
                    SyncOutcome::remove_unchanged_in_list(&self.projects, known_projects)
                })
                .unwrap_or_else(|| self.projects.clone()),
            time_entries: known_changes
                .time_entries
                .map(|known_time_entries| {
                    SyncOutcome::remove_unchanged_in_list(&self.time_entries, known_time_entries)
                })
                .unwrap_or_else(|| self.time_entries.clone()),
        }
    }

    fn remove_unchanged_in_list<T: Entity>(
        changes: &[SyncResult<T>],
        known_changes: Vec<T>,
    ) -> Vec<SyncResult<T>> {
        changes
            .iter()
            .filter_map(|change| match change {
                SyncResult::<T>::Changed(entity) if known_changes.contains(&entity) => None, // nothing new, get rid of it
                _ => Some(change.clone()), // keep it
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    mod sync_outcome {
        use super::super::{SyncOutcome, SyncResult};
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
                user: Some(SyncResult::<User>::Created {
                    client_assigned_id: 123,
                    entity: User {
                        id: 1,
                        default_workspace_id: 0,
                        fullname: "user".to_string(),
                        api_token: "token".to_string(),
                        at: Utc::now(),
                    },
                }),
                projects: vec![SyncResult::<Project>::Changed(Project {
                    id: 2,
                    workspace_id: 0,
                    name: "project".to_string(),
                    color: "#ff0000".to_string(),
                    active: true,
                    at: Utc::now(),
                    server_deleted_at: None,
                })],
                time_entries: vec![SyncResult::<TimeEntry>::Failed {
                    client_assigned_id: 3,
                    code: 1,
                    message: "msg".to_string(),
                }],
            };

            let merged = SyncOutcome::merge(a, b.clone());

            assert_eq!(merged, b);
        }

        #[test]
        fn merges_two_non_overlapping_non_empty() {
            let a = SyncOutcome {
                user: Some(SyncResult::<User>::Created {
                    client_assigned_id: 123,
                    entity: User {
                        id: 1,
                        default_workspace_id: 0,
                        fullname: "user".to_string(),
                        api_token: "token".to_string(),
                        at: Utc::now(),
                    },
                }),
                projects: vec![SyncResult::<Project>::Changed(Project {
                    id: 2,
                    workspace_id: 0,
                    name: "project A".to_string(),
                    color: "#ff0000".to_string(),
                    active: true,
                    at: Utc::now(),
                    server_deleted_at: None,
                })],
                time_entries: vec![SyncResult::<TimeEntry>::Failed {
                    client_assigned_id: 3,
                    code: 1,
                    message: "error".to_string(),
                }],
            };
            let b = SyncOutcome {
                user: None,
                projects: vec![SyncResult::<Project>::Changed(Project {
                    id: 4,
                    workspace_id: 0,
                    name: "project B".to_string(),
                    color: "#ff0000".to_string(),
                    active: true,
                    at: Utc::now(),
                    server_deleted_at: None,
                })],
                time_entries: vec![SyncResult::<TimeEntry>::Failed {
                    client_assigned_id: 5,
                    code: 3,
                    message: "error".to_string(),
                }],
            };

            let merged = SyncOutcome::merge(a, b);

            assert!(merged.user.is_some());
            assert_eq!(merged.projects.len(), 2);
            assert_eq!(merged.time_entries.len(), 2);
        }
    }
}
