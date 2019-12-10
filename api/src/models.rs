use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::toggl_api::models::{ApiToken, Id};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Id,
    pub fullname: String,
    pub api_token: ApiToken,
    pub at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub id: Id,
    pub name: String,
    pub color: String,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeEntry {
    pub id: Id,
    pub description: String,
    pub project_id: Option<Id>,
    pub start: DateTime<Utc>,
    pub duration: Option<u64>,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Delta {
    pub user: Option<User>,
    pub projects: Option<Vec<Project>>,
    pub time_entries: Option<Vec<TimeEntry>>,
}

pub trait Resolve {
    fn id(self: &Self) -> u64;
}

impl Resolve for User {
    fn id(self: &Self) -> u64 {
        self.id
    }
}

impl Resolve for Project {
    fn id(self: &Self) -> u64 {
        self.id
    }
}

impl Resolve for TimeEntry {
    fn id(self: &Self) -> u64 {
        self.id
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
