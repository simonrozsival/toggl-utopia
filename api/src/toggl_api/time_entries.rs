use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::endpoints;
use super::models::TimeEntry;
use crate::error::Error;

use crate::toggl_api::{models::Id, TogglApi};

impl TogglApi {
    pub fn fetch_time_entries(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<TimeEntry>, Error> {
        let mut response = self.req(endpoints::time_entries(since)).send()?;

        TogglApi::validate(&mut response)?;

        Ok(response.json::<Vec<TimeEntry>>()?)
    }

    pub fn create_time_entry(&self, te: TimeEntry) -> Result<TimeEntry, Error> {
        let modified_te = TimeEntry {
            created_with: Some("UtoAPI".to_string()),
            ..te
        };

        let mut response = self
            .req(endpoints::create_time_entry(&te.workspace_id))
            .json(&modified_te)
            .send()?;

        TogglApi::validate(&mut response)?;

        Ok(response.json::<TimeEntry>()?)
    }

    pub fn update_time_entry(&self, te: TimeEntry) -> Result<TimeEntry, Error> {
        let mut response = self
            .req(endpoints::update_time_entry(&te.id))
            .json(&te)
            .send()?;

        TogglApi::validate(&mut response)?;

        Ok(response.json::<TimeEntry>()?)
    }

    pub fn fetch_current_running_time_entry(&self) -> Result<Option<TimeEntry>, Error> {
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

        let mut response = self.req(endpoints::current_running_time_entry()).send()?;
        TogglApi::validate(&mut response)?;

        let data = response.json::<RunningTimeEntryResponse>()?.data;
        Ok(data.map(|te| TimeEntry {
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
