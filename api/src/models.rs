use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

use crate::toggl_api::models::{ApiToken, Id};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: Id,
    pub default_workspace_id: Id,
    pub fullname: String,
    pub api_token: ApiToken,
    pub at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Project {
    pub id: Id,
    pub workspace_id: Id,
    pub name: String,
    pub color: String,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TimeEntry {
    pub id: Id,
    pub workspace_id: Id,
    pub description: String,
    pub project_id: Option<Id>,
    pub start: DateTime<Utc>,
    pub duration: Option<u64>,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct Delta {
    pub user: Option<User>,
    pub projects: Option<Vec<Project>>,
    pub time_entries: Option<Vec<TimeEntry>>,
}

pub trait Entity: Clone + Serialize {
    fn id(&self) -> Id;
    fn is_deleted(&self) -> bool;
    fn last_update(&self) -> DateTime<Utc>;
    fn exists_on_server(&self) -> bool {
        self.id() > 0
    }
}

impl Entity for User {
    fn id(&self) -> Id {
        self.id
    }

    fn is_deleted(&self) -> bool {
        false
    }

    fn last_update(&self) -> DateTime<Utc> {
        self.at
    }
}

impl Entity for Project {
    fn id(&self) -> Id {
        self.id
    }

    fn is_deleted(&self) -> bool {
        self.server_deleted_at.is_some()
    }

    fn last_update(&self) -> DateTime<Utc> {
        self.at
    }
}

impl Entity for TimeEntry {
    fn id(&self) -> Id {
        self.id
    }

    fn is_deleted(&self) -> bool {
        self.server_deleted_at.is_some()
    }

    fn last_update(&self) -> DateTime<Utc> {
        self.at
    }
}
