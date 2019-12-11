mod conflicts;
pub mod prelude;
mod server;

use chrono::{DateTime, Utc};

use crate::models::Delta;
use crate::toggl_api::TogglApi;
use prelude::SyncOutcome;

pub fn fetch_snapshot(api: &TogglApi) -> Result<Delta, reqwest::Error> {
    server::fetch_changes_since(None, &api)
}

pub fn update_server_and_calculate_delta_for_client(
    last_sync: DateTime<Utc>,
    client_delta: Option<Delta>,
    api: &TogglApi,
) -> Result<SyncOutcome, reqwest::Error> {
    // 1. Get the data which have changed on the server since the last update
    let server_delta = server::fetch_changes_since(Some(last_sync), &api)?;

    // 2. Resolve which time entry should be running after this sync
    let currently_running_time_entry_on_server = server::currently_running_time_entry(&api)?;
    println!("{:#?}", currently_running_time_entry_on_server);
    // @todo

    // 3. Figure out what to change on client and what to change on the server
    let (client_resolution, server_resolution) =
        conflicts::resolve(client_delta.unwrap_or_default(), server_delta);
    // - we assume that these two sets are distinct

    // 4. Push the changes to the server
    let server_update_outcome = server::apply_changes(server_resolution, &api);

    // 5. Determine the duration of the stopped time entry (if any)
    // @todo

    // 6. Return the updates to the client
    let update_on_client = SyncOutcome::convert(client_resolution);
    let resolution = SyncOutcome::merge(update_on_client, server_update_outcome);

    Ok(resolution)
}
