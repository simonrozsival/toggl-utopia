use super::models::Id;
use chrono::{DateTime, Utc};

const BASE_URL: &'static str = "https://mobile.toggl.space/api/v9";

pub struct Endpoint {
    pub method: reqwest::Method,
    pub url: String,
}

pub fn me() -> Endpoint {
    Endpoint {
        url: format!("{}/me", BASE_URL),
        method: reqwest::Method::GET,
    }
}

pub fn projects(since: Option<DateTime<Utc>>) -> Endpoint {
    let url = match since {
        Some(date) => format!(
            "{}/me/projects?since={}&include_archived=true",
            BASE_URL,
            date.timestamp()
        ),
        None => format!("{}/me/projects?include_archived=true", BASE_URL),
    };

    Endpoint {
        url,
        method: reqwest::Method::GET,
    }
}

pub fn time_entries(since: Option<DateTime<Utc>>) -> Endpoint {
    let url = match since {
        Some(date) => format!("{}/me/time_entries?since={}", BASE_URL, date.timestamp()),
        None => format!("{}/me/time_entries", BASE_URL),
    };

    Endpoint {
        url,
        method: reqwest::Method::GET,
    }
}

pub fn create_time_entry(workspace_id: &Id) -> Endpoint {
    Endpoint {
        url: format!("{}/workspaces/{}/time_entries", BASE_URL, workspace_id),
        method: reqwest::Method::POST,
    }
}

pub fn update_time_entry(id: &Id) -> Endpoint {
    Endpoint {
        url: format!("{}/time_entries/{}", BASE_URL, id),
        method: reqwest::Method::PUT,
    }
}
