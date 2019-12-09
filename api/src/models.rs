use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::toggl_api::models::{ApiToken, ProjectId, TimeEntryId, UserId};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: UserId,
    pub fullname: String,
    pub api_token: ApiToken,
    pub at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub color: String,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub description: String,
    pub project_id: Option<ProjectId>,
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
