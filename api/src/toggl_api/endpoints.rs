use chrono::{DateTime, Utc};

const BASE_URL: &'static str = "https://mobile.toggl.space/api/v9/";

pub fn me() -> String {
    format!("{}/me", BASE_URL)
}

pub fn projects(since: Option<DateTime<Utc>>) -> String {
    match since {
        Some(date) => format!(
            "{}/me/projects?since={}&include_archived=true",
            BASE_URL,
            date.timestamp()
        ),
        None => format!("{}/me/projects?include_archived=true", BASE_URL),
    }
}

pub fn time_entries(since: Option<DateTime<Utc>>) -> String {
    match since {
        Some(date) => format!("{}/me/time_entries?since={}", BASE_URL, date.timestamp()),
        None => format!("{}/me/time_entries", BASE_URL),
    }
}
