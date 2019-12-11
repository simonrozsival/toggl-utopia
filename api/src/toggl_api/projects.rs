use chrono::{DateTime, Utc};

use super::endpoints;
use super::models::Project;
use crate::toggl_api::TogglApi;

impl TogglApi {
    pub fn fetch_projects(
        &self,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<Project>, reqwest::Error> {
        let projects = self
            .req(endpoints::projects(since))
            .send()?
            .json::<Vec<Project>>()?;

        Ok(projects)
    }
}
