mod conflicts;
pub mod prelude;
mod server;

use chrono::{DateTime, Utc};

use crate::models::{Delta, TimeEntry};
use crate::toggl_api::TogglApi;
use prelude::{SyncOutcome, SyncResult};

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

    let running_on_server = server::currently_running_time_entry(&api)?;
    let running_on_client = client_delta
        .as_ref()
        .and_then(|delta| delta.time_entries.as_ref())
        .and_then(|time_entries| {
            time_entries
                .into_iter()
                .find(|te| te.is_running())
                .map(|te| te.clone())
        });

    // Lemma:
    // There might be a running TE "A" on client even if it isn't in the delta
    // because it has been running since the previous sync and it hasn't been touched
    // since. Either the running TE on the server is the same, or the TE running on the
    // client will be stopped on client in the sync outcome which we'll send back.
    //
    // Proof:
    // If there is a different running TE "B" on the server now, then "A" must
    // have been stopped on the server since the last sync and therefore there will be
    // in server delta and it will be sent to the client as a "change" outcome and it
    // will be stopped. This cannot be overwritten by the client, because if the user
    // touched "A", it would be in the client delta and `running_on_client` wouldn't
    // be None. Qed.

    // 2. Figure out what to change on client and what to change on the server
    let (mut client_resolution, mut server_resolution) =
        conflicts::resolve(client_delta.unwrap_or_default(), server_delta);
    // - we assume that the two resulting sets are distinct

    // 3. Check how the running TEs were affected by the conflict resolution
    let running_on_client = running_on_client
        .map(|time_entry| effect_of_conflict_resolution(time_entry, &client_resolution));
    let running_on_server = running_on_server
        .map(|time_entry| effect_of_conflict_resolution(time_entry, &server_resolution));
    let maybe_stopped = should_stop(running_on_client, running_on_server).map(|te| te.stop());
    if let Some(stopped) = &maybe_stopped {
        // We must now propagate this change both to the server and to the client.
        // We will add this change to the list of server resolutions and the TE will be
        // updated. Unfortunatelly this won't produce any feedback for the client
        // (the server thinks that the client already has the data), so we must add it
        // to the list of client resolutions as well after we make sure, that this update
        // actually worked.

        let time_entries = push_and_maybe_replace(stopped.clone(), server_resolution.time_entries);
        server_resolution.time_entries = Some(time_entries);
    }

    // 4. Push the changes to the server
    let server_update_outcome = server::apply_changes(server_resolution, &api);

    // 5. Check if we tried stopping a TE and if it hasn't failed, push the change to the user
    if let Some(stopped) = maybe_stopped {
        // has stopping failed?
        let potential_stopping_failure =
            server_update_outcome.time_entries.iter().find(|outcome| {
                if let SyncResult::<TimeEntry>::Failed {
                    client_assigned_id, ..
                } = outcome
                {
                    client_assigned_id == &stopped.id
                } else {
                    false
                }
            });

        if let None = potential_stopping_failure {
            let time_entries = push_and_maybe_replace(stopped, client_resolution.time_entries);
            client_resolution.time_entries = Some(time_entries);
        }
    }

    // 5. Return the updates to the client
    let update_on_client = SyncOutcome::convert(client_resolution);
    let resolution = SyncOutcome::merge(update_on_client, server_update_outcome);

    Ok(resolution)
}

fn push_and_maybe_replace(
    stopped: TimeEntry,
    time_entries: Option<Vec<TimeEntry>>,
) -> Vec<TimeEntry> {
    let mut time_entries: Vec<_> = time_entries
        .unwrap_or(vec![])
        .into_iter()
        .filter(|existing| existing.id != stopped.id)
        .collect();

    time_entries.push(stopped);

    time_entries
}

fn effect_of_conflict_resolution(te: TimeEntry, resolved: &Delta) -> TimeEntry {
    resolved
        .time_entries
        .as_ref()
        .and_then(|updated_tes| {
            updated_tes
                .into_iter()
                .find(|updated_te| updated_te.id == te.id)
                .map(|te| te.clone())
        })
        .unwrap_or(te) // if the TE was updated
}

fn should_stop(client_te: Option<TimeEntry>, server_te: Option<TimeEntry>) -> Option<TimeEntry> {
    if let (Some(client), Some(server)) = (client_te, server_te) {
        if client.id == server.id {
            None
        } else if client.at > server.at {
            Some(server)
        } else {
            Some(client)
        }
    } else {
        None
    }
}
