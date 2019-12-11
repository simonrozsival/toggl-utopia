use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::endpoints;
use super::models::TimeEntry;

use crate::toggl_api::{models::Id, TogglApi};

impl TogglApi {
    pub fn fetch_time_entries(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<TimeEntry>, reqwest::Error> {
        let time_entries = self
            .req(endpoints::time_entries(since))
            .send()?
            .json::<Vec<TimeEntry>>()?;

        Ok(time_entries)
    }

    pub fn create_time_entry(&self, te: TimeEntry) -> Result<TimeEntry, reqwest::Error> {
        let modified_te = TimeEntry {
            created_with: Some("UtoAPI".to_string()),
            ..te
        };

        let time_entry = self
            .req(endpoints::create_time_entry(&te.workspace_id))
            .json(&modified_te)
            .send()?
            .json::<TimeEntry>()?;

        Ok(time_entry)
    }

    pub fn update_time_entry(&self, te: TimeEntry) -> Result<TimeEntry, reqwest::Error> {
        let time_entry = self
            .req(endpoints::update_time_entry(&te.id))
            .json(&te)
            .send()?
            .json::<TimeEntry>()?;

        Ok(time_entry)
    }

    pub fn fetch_current_running_time_entry(&self) -> Result<Option<TimeEntry>, reqwest::Error> {
        #[derive(Deserialize, Debug)]
        struct RunningTimeEntryResponse {
            data: Option<RunningTimeEntryV8>,
        }

        #[derive(Deserialize, Debug)]
        struct RunningTimeEntryV8 {
            id: Id,
            wid: Id,
            pid: Option<Id>,
            start: DateTime<Utc>,
            duration: i64,
            description: String,
            at: DateTime<Utc>,
        }

        let res = self
            .req(endpoints::current_running_time_entry())
            .send()?
            .json::<RunningTimeEntryResponse>()?;

        Ok(res.data.map(|te| TimeEntry {
            id: te.id,
            workspace_id: te.wid,
            project_id: te.pid,
            start: te.start,
            duration: te.duration,
            description: te.description,
            at: te.at,
            server_deleted_at: None,
            created_with: None,
        }))
    }
}
