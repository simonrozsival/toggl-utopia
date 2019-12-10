use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

use crate::toggl_api::models::{ApiToken, Id};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: Id,
    pub fullname: String,
    pub api_token: ApiToken,
    pub at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Project {
    pub id: Id,
    pub name: String,
    pub color: String,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TimeEntry {
    pub id: Id,
    pub description: String,
    pub project_id: Option<Id>,
    pub start: DateTime<Utc>,
    pub duration: Option<u64>,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Delta {
    pub user: Option<User>,
    pub projects: Option<Vec<Project>>,
    pub time_entries: Option<Vec<TimeEntry>>,
}

pub trait Resolve {
    fn id(&self) -> u64;
    fn is_deleted(&self) -> bool;
    fn last_update(&self) -> DateTime<Utc>;
}

impl Resolve for User {
    fn id(&self) -> u64 {
        self.id
    }

    fn is_deleted(&self) -> bool {
        false
    }

    fn last_update(&self) -> DateTime<Utc> {
        self.at
    }
}

impl Resolve for Project {
    fn id(&self) -> u64 {
        self.id
    }

    fn is_deleted(&self) -> bool {
        self.server_deleted_at.is_some()
    }

    fn last_update(&self) -> DateTime<Utc> {
        self.at
    }
}

impl Resolve for TimeEntry {
    fn id(&self) -> u64 {
        self.id
    }

    fn is_deleted(&self) -> bool {
        self.server_deleted_at.is_some()
    }

    fn last_update(&self) -> DateTime<Utc> {
        self.at
    }
}

impl Delta {
    fn merge_optional_vectors<T>(client: Option<Vec<T>>, server: Option<Vec<T>>) -> Option<Vec<T>>
    where
        T: Clone,
    {
        match (client, server) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => Some([x, y].concat()),
        }
    }
}
