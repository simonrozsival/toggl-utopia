use super::endpoints;
use super::models::TimeEntry;
use crate::error::Error;

use crate::toggl_api::TogglApi;

impl TogglApi {
    pub fn fetch_current_running_time_entry(&self) -> Result<Option<TimeEntry>, Error> {
        self.make_request(endpoints::time_entries::current_running())
            .map(|res| res.to_time_entry())
    }
}
