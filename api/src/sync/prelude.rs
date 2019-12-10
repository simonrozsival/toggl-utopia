use serde::Serialize;

use crate::models::{Project, TimeEntry, User};

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SyncOutcome<T: Serialize> {
    Create {
        entity: T,
    },
    Update {
        id: u64,
        entity: T,
    },
    Delete {
        entity: T,
    },
    Error {
        entity: T,
        code: u64,
        message: String,
    },
}

#[derive(Serialize)]
pub struct SyncResolution {
    pub user: Option<SyncOutcome<User>>,
    pub projects: Option<Vec<SyncOutcome<Project>>>,
    pub time_entries: Option<Vec<SyncOutcome<Project>>>,
}

impl SyncResolution {
    pub fn merge(a: SyncResolution, b: SyncResolution) -> SyncResolution {
        unimplemented!("Not implemented just yet.");
    }
}
