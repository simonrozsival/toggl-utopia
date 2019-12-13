use super::models::{Project, TimeEntry};
use serde::{de::DeserializeOwned, Serialize};

const BASE_URL: &str = "https://mobile.toggl.space/api";

type Url = String;

#[derive(Debug)]
pub enum Endpoint<T: Serialize + DeserializeOwned> {
    Get(Url),
    Post(Url, T),
    Put(Url, T),
}

pub trait CreateOrUpdate: Serialize + DeserializeOwned {
    fn create(self) -> Endpoint<Self>;
    fn update(self) -> Endpoint<Self>;
}

impl CreateOrUpdate for Project {
    fn create(self) -> Endpoint<Self> {
        Endpoint::<Project>::Post(
            format!("{}/v9/workspaces/{}/projects", BASE_URL, self.workspace_id),
            self,
        )
    }

    fn update(self) -> Endpoint<Self> {
        Endpoint::<Project>::Put(format!("{}/v9/projects/{}", BASE_URL, self.id), self)
    }
}

impl CreateOrUpdate for TimeEntry {
    fn create(self) -> Endpoint<Self> {
        Endpoint::<TimeEntry>::Post(
            format!(
                "{}/v9/workspaces/{}/time_entries",
                BASE_URL, self.workspace_id
            ),
            TimeEntry {
                created_with: Some("UtoAPI".to_string()),
                ..self
            },
        )
    }

    fn update(self) -> Endpoint<Self> {
        Endpoint::<TimeEntry>::Put(format!("{}/v9/time_entries/{}", BASE_URL, self.id), self)
    }
}

pub mod user {
    use super::super::models::User;
    use super::{Endpoint, BASE_URL};

    pub fn get() -> Endpoint<User> {
        Endpoint::<User>::Get(format!("{}/v9/me", BASE_URL))
    }
}

pub mod projects {
    use super::super::models::Project;
    use super::{Endpoint, BASE_URL};
    use chrono::{DateTime, Utc};

    pub fn get(since: Option<DateTime<Utc>>) -> Endpoint<Vec<Project>> {
        let url = match since {
            Some(date) => format!(
                "{}/v9/me/projects?since={}&include_archived=true",
                BASE_URL,
                date.timestamp()
            ),
            None => format!("{}/v9/me/projects?include_archived=true", BASE_URL),
        };

        Endpoint::<Vec<Project>>::Get(url)
    }
}

pub mod time_entries {
    use super::super::models::{Id, TimeEntry};
    use super::{Endpoint, BASE_URL};
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RunningTimeEntryResponseData {
        data: Option<TimeEntryV8>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TimeEntryV8 {
        id: Id,
        wid: Id,
        pid: Option<Id>,
        start: DateTime<Utc>,
        duration: i64,
        description: String,
        at: DateTime<Utc>,
    }

    impl RunningTimeEntryResponseData {
        pub fn to_time_entry(&self) -> Option<TimeEntry> {
            self.data.as_ref().map(|te| TimeEntry {
                id: te.id,
                workspace_id: te.wid,
                project_id: te.pid,
                start: te.start,
                duration: te.duration,
                description: te.description.clone(),
                at: te.at,
                server_deleted_at: None,
                created_with: None,
            })
        }
    }

    pub fn get(since: Option<DateTime<Utc>>) -> Endpoint<Vec<TimeEntry>> {
        let url = match since {
            Some(date) => format!("{}/v9/me/time_entries?since={}", BASE_URL, date.timestamp()),
            None => format!("{}/v9/me/time_entries", BASE_URL),
        };

        Endpoint::<Vec<TimeEntry>>::Get(url)
    }

    pub fn current_running() -> Endpoint<RunningTimeEntryResponseData> {
        Endpoint::<RunningTimeEntryResponseData>::Get(format!(
            "{}/v8/time_entries/current",
            BASE_URL
        ))
    }
}
