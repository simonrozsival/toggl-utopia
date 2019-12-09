mod conflicts;
mod server;

use chrono::{DateTime, Utc};

use crate::auth::Credentials;
use crate::models::{Delta, Id};

pub fn fetch_snapshot(credentials: &Credentials) -> Result<Delta, reqwest::Error> {
    server::fetch_changes_since(None, &credentials)
}

pub fn update_server_and_calculate_delta_for_client(
    last_sync: &DateTime<Utc>,
    client_delta: &Delta,
    credentials: &Credentials,
) -> Result<Delta, reqwest::Error> {
    // 1. Get the data which have changed on the server since the last update
    let server_delta = server::fetch_changes_since(Some(last_sync.clone()), &credentials)?;

    // 2. Figure out what to change on client and what to change on the server
    let (update_on_client, update_on_server) = conflicts::resolve(&client_delta, &server_delta);
    assert_eq!(share_entities(&update_on_client, &update_on_server), false);

    // 3. Push the changes to the server
    let update_errors = server::overwrite_with(&update_on_server, &credentials);

    // 4. Return the updates to the client
    Ok(Delta::merge(&update_errors, &update_on_client))
}

fn share_entities(a: &Delta, b: &Delta) -> bool {
    entities_match(&a.user, &b.user)
        || overlap(&a.projects, &b.projects)
        || overlap(&a.time_entries, &b.time_entries)
}

fn entities_match<T>(a: &Option<T>, b: &Option<T>) -> bool
where
    T: Id,
{
    match (a, b) {
        (Some(x), Some(y)) => x.id() == y.id(),
        _ => false,
    }
}

fn overlap<T>(a: &Option<Vec<T>>, b: &Option<Vec<T>>) -> bool
where
    T: Id,
{
    match (a, b) {
        (Some(x), Some(y)) => {
            let ids_x = x.iter().map(|entity| entity.id()).collect::<Vec<_>>();
            y.iter()
                .map(|entity| entity.id())
                .any(|id| ids_x.contains(&id))
        }
        _ => false,
    }
}
