use super::endpoints;
use super::models::User;

use crate::toggl_api::TogglApi;

impl TogglApi {
    pub fn fetch_user(&self) -> Result<User, reqwest::Error> {
        let user = self.req(endpoints::me()).send()?.json::<User>()?;
        Ok(user)
    }
}
