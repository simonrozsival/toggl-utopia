mod conflicts;
pub mod prelude;
mod server;
mod utils;

use prelude::SyncResolution;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::auth::Credentials;
use crate::models::Delta;

pub fn fetch_snapshot(credentials: &Credentials) -> Result<Delta, reqwest::Error> {
    server::fetch_changes_since(None, &credentials)
}

pub fn update_server_and_calculate_delta_for_client(
    last_sync: &DateTime<Utc>,
    client_delta: &Delta,
    credentials: &Credentials,
) -> Result<SyncResolution, reqwest::Error> {
    // 1. Get the data which have changed on the server since the last update
    let server_delta = server::fetch_changes_since(Some(last_sync.clone()), &credentials)?;

    // 2. Figure out what to change on client and what to change on the server
    let (update_on_client, update_on_server) = conflicts::resolve(&client_delta, &server_delta);
    assert_eq!(
        utils::share_entities(&update_on_client, &update_on_server),
        false
    );

    // 3. Push the changes to the server
    let server_update_outcome = server::apply_changes(&update_on_server, &credentials);

    // 4. Return the updates to the client
    Ok(SyncResolution::merge(
        update_on_client,
        server_update_outcome,
    ))
}
