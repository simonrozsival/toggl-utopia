use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::Into;

use crate::models::{Project as UtopiaProject, TimeEntry as UtopiaTimeEntry, User as UtopiaUser};

pub type Id = u64;
pub type ApiToken = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Id,
    pub default_workspace_id: Id,
    pub fullname: String,
    pub api_token: ApiToken,
    pub at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: Id,
    pub workspace_id: Id,
    pub name: String,
    pub color: String,
    pub active: bool,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeEntry {
    pub id: Id,
    pub workspace_id: Id,
    pub description: String,
    pub project_id: Option<Id>,
    pub start: DateTime<Utc>,
    pub duration: i64,
    pub at: DateTime<Utc>,
    pub server_deleted_at: Option<DateTime<Utc>>,
}

impl Into<UtopiaUser> for User {
    fn into(self) -> UtopiaUser {
        UtopiaUser {
            id: self.id,
            fullname: self.fullname.clone(),
            api_token: self.api_token.clone(),
            at: self.at,
        }
    }
}

impl Into<UtopiaProject> for Project {
    fn into(self) -> UtopiaProject {
        UtopiaProject {
            id: self.id,
            name: self.name.clone(),
            color: self.color,
            active: self.active,
            at: self.at,
            server_deleted_at: self.server_deleted_at,
        }
    }
}

impl Into<UtopiaTimeEntry> for TimeEntry {
    fn into(self) -> UtopiaTimeEntry {
        UtopiaTimeEntry {
            id: self.id,
            project_id: self.project_id,
            description: self.description.clone(),
            start: self.start,
            duration: if self.duration >= 0 {
                Some(self.duration as u64)
            } else {
                None
            },
            at: self.at,
            server_deleted_at: self.server_deleted_at,
        }
    }
}
