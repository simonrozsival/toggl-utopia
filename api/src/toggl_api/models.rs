use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type UserId = u64;
pub type WorkspaceId = u64;
pub type ProjectId = u64;
pub type TimeEntryId = u64;
pub type ApiToken = String;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub default_workspace_id: u64,
    pub api_token: ApiToken,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub color: String,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub workspace_id: WorkspaceId,
    pub description: String,
    pub project_id: Option<ProjectId>,
    pub start: DateTime<Utc>,
    pub duration: i64,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}
