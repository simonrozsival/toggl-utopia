use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::toggl_api::models::{Project as TogglProject, TimeEntry as TogglTimeEntry, WorkspaceId};

pub type ProjectId = u64;
pub type TimeEntryId = u64;

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub color: String,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct TimeEntry {
    pub id: TimeEntryId,
    pub description: String,
    pub project_id: Option<ProjectId>,
    pub start: DateTime<Utc>,
    pub duration: Option<u64>,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct Delta {
    pub projects: Option<Vec<Project>>,
    pub time_entries: Option<Vec<TimeEntry>>,
}

impl Project {
    pub fn from(project: TogglProject) -> Project {
        Project {
            id: project.id,
            name: project.name.clone(),
            color: project.color,
            active: project.active,
            at: project.at,
            server_deleted_at: project.server_deleted_at,
        }
    }

    pub fn to_toggl_model(self: &Project, workspace_id: WorkspaceId) -> TogglProject {
        TogglProject {
            id: self.id,
            workspace_id: workspace_id,
            name: self.name.clone(),
            color: self.color.clone(),
            active: self.active,
            at: self.at,
            server_deleted_at: self.server_deleted_at,
        }
    }
}

impl TimeEntry {
    pub fn from(time_entry: TogglTimeEntry) -> TimeEntry {
        TimeEntry {
            id: time_entry.id,
            project_id: time_entry.project_id,
            description: time_entry.description.clone(),
            start: time_entry.start,
            duration: if time_entry.duration >= 0 {
                Some(time_entry.duration as u64)
            } else {
                None
            },
            at: time_entry.at,
            server_deleted_at: time_entry.server_deleted_at,
        }
    }

    pub fn to_toggl_model(self: &TimeEntry, workspace_id: WorkspaceId) -> TogglTimeEntry {
        TogglTimeEntry {
            id: self.id,
            workspace_id: workspace_id,
            project_id: self.project_id.clone(),
            description: self.description.clone(),
            start: self.start.clone(),
            duration: match self.duration {
                Some(duration) => duration as i64,
                None => -self.start.timestamp(),
            },
            at: self.at,
            server_deleted_at: self.server_deleted_at,
        }
    }
}
