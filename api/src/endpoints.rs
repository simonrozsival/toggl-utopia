use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::models::{Delta, Project, TimeEntry};
use crate::responses::*;
use crate::toggl_api;

#[derive(Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn login(credentials: web::Json<Credentials>) -> HttpResponse {
    let user = match toggl_api::user::login(&credentials.email, &credentials.password) {
        Ok(user) => user,
        Err(_) => {
            return invalid_credentials();
        }
    };

    let projects: Vec<Project> = match toggl_api::projects::get_all(None, &user.api_token) {
        Ok(projects) => projects.into_iter().map(Project::from).collect(),
        Err(error) => return something_went_wrong(error),
    };

    let time_entries: Vec<TimeEntry> = match toggl_api::time_entries::get_all(None, &user.api_token)
    {
        Ok(time_entries) => time_entries.into_iter().map(TimeEntry::from).collect(),
        Err(error) => return something_went_wrong(error),
    };

    let delta = Delta {
        projects: Some(projects),
        time_entries: Some(time_entries),
    };

    login_succeeded(user.api_token, delta)
}
