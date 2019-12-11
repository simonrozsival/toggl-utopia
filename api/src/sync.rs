mod conflicts;
pub mod prelude;
mod server;
mod utils;

use chrono::{DateTime, Utc};

use crate::models::Delta;
use crate::toggl_api::TogglApi;
use prelude::SyncResolution;

pub fn fetch_snapshot(api: &TogglApi) -> Result<Delta, reqwest::Error> {
    server::fetch_changes_since(None, &api)
}

pub fn update_server_and_calculate_delta_for_client(
    last_sync: DateTime<Utc>,
    client_delta: Delta,
    api: &TogglApi,
) -> Result<SyncResolution, reqwest::Error> {
    // 1. Get the data which have changed on the server since the last update
    let server_delta = server::fetch_changes_since(Some(last_sync), &api)?;

    // 2. Figure out what to change on client and what to change on the server
    let (update_on_client, update_on_server) = conflicts::resolve(client_delta, server_delta);
    assert_eq!(
        utils::share_entities(&update_on_client, &update_on_server),
        false
    );

    // 3. Push the changes to the server
    let server_update_outcome = server::apply_changes(update_on_server, &api);

    // 4. Return the updates to the client
    let resolution = SyncResolution::merge(update_on_client, server_update_outcome);

    Ok(resolution)
}
