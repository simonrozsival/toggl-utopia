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

    pub fn create_project(&self, project: Project) -> Result<Project, Error> {
        let mut response = self
            .req(endpoints::create_project(&project.workspace_id))
            .json(&project)
            .send()?;

        TogglApi::validate(&mut response)?;

        Ok(response.json::<Project>()?)
    }

    pub fn update_project(&self, project: Project) -> Result<Project, Error> {
        let mut response = self
            .req(endpoints::update_project(&project.id))
            .json(&project)
            .send()?;

        TogglApi::validate(&mut response)?;

        Ok(response.json::<Project>()?)
    }
}
