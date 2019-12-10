use super::prelude::{create, keep, update, ConflictResolution, SyncResolution};
use crate::models::{Delta, Project, Resolve, TimeEntry, User};
use serde::Serialize;

fn prefer_newer<T: Clone + Resolve + Serialize>(
    client: T,
    server: T,
) -> (ConflictResolution<T>, ConflictResolution<T>) {
    if client.last_update() > server.last_update() {
        (keep(&client), update(server.id(), &client))
    } else {
        (update(client.id(), &server), keep(&server))
    }
}

pub fn resolve_single<T: Clone + Serialize + Resolve>(
    client: Option<T>,
    server: Option<T>,
) -> (ConflictResolution<T>, ConflictResolution<T>) {
    match (client, server) {
        (None, None) => panic!("This doesn't make any sense - at least one entity is needed to perform conflict resolution."),
        (Some(client_entity), None) => (keep(&client_entity), create(&client_entity)),
        (None, Some(server_entity)) => (create(&server_entity), keep(&server_entity)),
        (Some(client_entity), Some(server_entity)) if server_entity.is_deleted() => (update(client_entity.id(), &server_entity), keep(&server_entity)), // cannot update an entity which was already deleted on the server
        (Some(client_entity), Some(server_entity)) => prefer_newer(client_entity, server_entity)
    }
}

pub fn resolve(client: Delta, server: Delta) -> (SyncResolution, SyncResolution) {
    unimplemented!("This needs to be implemented");
}

#[cfg(test)]
mod tests {
    use crate::models::Project;
    use crate::toggl_api::models::Id;
    use chrono::{DateTime, TimeZone, Utc};

    fn create_project(id: Id, at: DateTime<Utc>, deleted_at: Option<DateTime<Utc>>) -> Project {
        Project {
            id,
            name: "ABC".to_string(),
            color: "#ff0000".to_string(),
            active: true,
            at,
            server_deleted_at: deleted_at,
        }
    }

    mod prefer_newer {
        use super::super::prefer_newer;
        use super::create_project;
        use crate::models::Project;
        use crate::sync::prelude::{create, keep, update};
        use chrono::{TimeZone, Utc};

        #[test]
        fn prefers_client_if_it_was_updated_later() {
            let client = create_project(1, Utc.ymd(2019, 12, 10).and_hms(12, 00, 00), None);
            let server = create_project(2, Utc.ymd(2019, 12, 09).and_hms(12, 00, 00), None);

            let (client_res, server_res) = prefer_newer(client.clone(), server.clone());

            assert_eq!(client_res, keep(&client));
            assert_eq!(server_res, update(server.id, &client));
        }

        #[test]
        fn prefers_server_if_it_was_updated_later() {
            let client = create_project(1, Utc.ymd(2019, 12, 09).and_hms(12, 00, 00), None);
            let server = create_project(2, Utc.ymd(2019, 12, 10).and_hms(12, 00, 00), None);

            let (client_res, server_res) = prefer_newer(client.clone(), server.clone());

            assert_eq!(client_res, update(client.id, &server));
            assert_eq!(server_res, keep(&server));
        }
    }

    mod resolve_single {
        use super::super::resolve_single;
        use super::create_project;
        use crate::models::Project;
        use crate::sync::prelude::{create, keep, update};
        use chrono::{TimeZone, Utc};

        #[test]
        fn creates_client_when_there_is_not_a_counterpart_on_the_server() {
            let client = create_project(1, Utc.ymd(2019, 12, 09).and_hms(12, 00, 00), None);

            let (client_res, server_res) = resolve_single(Some(client.clone()), None);

            assert_eq!(client_res, keep(&client));
            assert_eq!(server_res, create(&client));
        }
    }
}
