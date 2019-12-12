use super::endpoints;
use super::models::User;

use crate::error::Error;
use crate::toggl_api::TogglApi;

impl TogglApi {
    pub fn fetch_user(&self) -> Result<User, Error> {
        let mut response = self.req(endpoints::me()).send()?;

        TogglApi::validate(&mut response)?;

        Ok(response.json::<User>()?)
    }
}
