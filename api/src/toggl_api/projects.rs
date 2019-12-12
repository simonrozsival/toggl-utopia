use chrono::{DateTime, Utc};

use super::endpoints;
use super::models::Project;
use crate::error::Error;
use crate::toggl_api::TogglApi;

impl TogglApi {
    pub fn fetch_projects(&self, since: Option<DateTime<Utc>>) -> Result<Vec<Project>, Error> {
        let mut response = self.req(endpoints::projects(since)).send()?;

        TogglApi::validate(&mut response)?;

        Ok(response.json::<Vec<Project>>()?)
    }
}
