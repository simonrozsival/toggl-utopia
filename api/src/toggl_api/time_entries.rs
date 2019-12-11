use chrono::{DateTime, Utc};

use super::endpoints;
use super::models::TimeEntry;

use crate::toggl_api::TogglApi;

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
}
