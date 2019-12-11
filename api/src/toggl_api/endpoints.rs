use super::models::Id;
use chrono::{DateTime, Utc};

const BASE_URL: &'static str = "https://mobile.toggl.space/api";

#[derive(Debug)]
pub struct Endpoint {
    pub method: reqwest::Method,
    pub url: String,
}

pub fn me() -> Endpoint {
    Endpoint {
        url: format!("{}/v9/me", BASE_URL),
        method: reqwest::Method::GET,
    }
}

pub fn projects(since: Option<DateTime<Utc>>) -> Endpoint {
    let url = match since {
        Some(date) => format!(
            "{}/v9/me/projects?since={}&include_archived=true",
            BASE_URL,
            date.timestamp()
        ),
        None => format!("{}/v9/me/projects?include_archived=true", BASE_URL),
    };

    Endpoint {
        url,
        method: reqwest::Method::GET,
    }
}

pub fn time_entries(since: Option<DateTime<Utc>>) -> Endpoint {
    let url = match since {
        Some(date) => format!("{}/v9/me/time_entries?since={}", BASE_URL, date.timestamp()),
        None => format!("{}/v9/me/time_entries", BASE_URL),
    };

    Endpoint {
        url,
        method: reqwest::Method::GET,
    }
}

pub fn create_time_entry(workspace_id: &Id) -> Endpoint {
    Endpoint {
        url: format!("{}/v9/workspaces/{}/time_entries", BASE_URL, workspace_id),
        method: reqwest::Method::POST,
    }
}

pub fn update_time_entry(id: &Id) -> Endpoint {
    Endpoint {
        url: format!("{}/v9/time_entries/{}", BASE_URL, id),
        method: reqwest::Method::PUT,
    }
}

pub fn current_running_time_entry() -> Endpoint {
    Endpoint {
        url: format!("{}/v8/time_entries/current", BASE_URL),
        method: reqwest::Method::GET,
    }
}
