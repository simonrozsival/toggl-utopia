use super::prelude::{create, update, ConflictResolution, SyncResolution};
use crate::models::{Delta, Entity};
use crate::toggl_api::models::Id;

fn prefer_newer<T: Entity>(
    client: T,
    server: T,
) -> (Option<ConflictResolution<T>>, Option<ConflictResolution<T>>) {
    if client.last_update() > server.last_update() {
        (None, Some(update(server.id(), &client)))
    } else {
        (Some(update(client.id(), &server)), None)
    }
}

fn resolve_single<T: Entity>(
    client: Option<T>,
    server: Option<T>,
) -> (Option<ConflictResolution<T>>, Option<ConflictResolution<T>>) {
    match (client, server) {
        (None,    None)     => (None, None),
        (Some(c), None)     => (None, Some(create(&c))),
        (None,    Some(s))  => (Some(create(&s)), None),
        (Some(c), Some(s)) if !c.is_deleted() && s.is_deleted() // we shouldn't update an entity which was already deleted on the server, we can't un-delete it
            => (Some(update(c.id(), &s)), None),
        (Some(c), Some(s))  => prefer_newer(c, s)
    }
}

fn form_pairs<T: Entity>(client: Vec<T>, server: Vec<T>) -> Vec<(Option<T>, Option<T>)> {
    use std::collections::HashMap;

    let mut server_entities: HashMap<Id, T> =
        server.iter().fold(HashMap::new(), |mut acc, entity| {
            acc.insert(entity.id(), entity.clone());
            acc
        });

    let mut pairs: Vec<(Option<T>, Option<T>)> = Vec::new();

    for client_entity in client.iter() {
        let id = server_entities
            .get(&client_entity.id())
            .map(|server_entity| {
                let c = client_entity.clone();
                let s = server_entity.clone();
                pairs.push((Some(c), Some(s)));
                client_entity.id()
            });

        if let Some(id) = id {
            server_entities.remove(&id);
        }
    }

    for (_, server_entity) in server_entities.iter() {
        pairs.push((None, Some(server_entity.clone())));
    }

    pairs
}

fn resolve_many<T: Entity>(
    client: Option<Vec<T>>,
    server: Option<Vec<T>>,
) -> (
    Option<Vec<ConflictResolution<T>>>,
    Option<Vec<ConflictResolution<T>>>,
) {
    let pairs = match (client, server) {
        (None, None) => return (None, None),
        (Some(c), None) => form_pairs(c, vec![]),
        (None, Some(s)) => form_pairs(vec![], s),
        (Some(c), Some(s)) => form_pairs(c, s),
    };

    unimplemented!("Not implemented yet.");
}

pub fn resolve(client: Delta, server: Delta) -> (SyncResolution, SyncResolution) {
    let (client_user, server_user) = resolve_single(client.user, server.user);
    let (client_projects, server_projects) = resolve_many(client.projects, server.projects);

    let client_resolution = SyncResolution {
        user: client_user,
        projects: client_projects,
        time_entries: None,
    };

    let server_resolution = SyncResolution {
        user: server_user,
        projects: server_projects,
        time_entries: None,
    };

    (client_resolution, server_resolution)
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

    fn sooner() -> DateTime<Utc> {
        Utc.ymd(2019, 12, 09).and_hms(12, 00, 00)
    }

    fn later() -> DateTime<Utc> {
        Utc.ymd(2019, 12, 10).and_hms(12, 00, 00)
    }

    mod prefer_newer {
        use super::super::prefer_newer;
        use super::{create_project, later, sooner};
        use crate::sync::prelude::update;

        #[test]
        fn prefers_client_if_it_was_updated_later() {
            let client = create_project(1, later(), None);
            let server = create_project(2, sooner(), None);

            let (client_res, server_res) = prefer_newer(client.clone(), server.clone());

            assert_eq!(client_res, None);
            assert_eq!(server_res, Some(update(server.id, &client)));
        }

        #[test]
        fn prefers_server_if_it_was_updated_later() {
            let client = create_project(1, sooner(), None);
            let server = create_project(2, later(), None);

            let (client_res, server_res) = prefer_newer(client.clone(), server.clone());

            assert_eq!(client_res, Some(update(client.id, &server)));
            assert_eq!(server_res, None);
        }
    }

    mod resolve_single {
        use super::super::resolve_single;
        use super::{create_project, later, sooner};
        use crate::sync::prelude::{create, update};

        #[test]
        fn create_on_server_when_there_is_not_a_counterpart_on_the_server() {
            let client = create_project(1, sooner(), None);

            let (client_res, server_res) = resolve_single(Some(client.clone()), None);

            assert_eq!(client_res, None);
            assert_eq!(server_res, Some(create(&client)));
        }

        #[test]
        fn creates_on_client_when_there_is_not_a_counterpart_on_the_client() {
            let server = create_project(1, sooner(), None);

            let (client_res, server_res) = resolve_single(None, Some(server.clone()));

            assert_eq!(client_res, Some(create(&server)));
            assert_eq!(server_res, None);
        }

        #[test]
        fn updates_client_when_it_is_deleted_on_server_but_not_on_client() {
            let client = create_project(1, later(), None);
            let server = create_project(2, sooner(), Some(sooner()));

            let (client_res, server_res) =
                resolve_single(Some(client.clone()), Some(server.clone()));

            assert_eq!(client_res, Some(update(client.id, &server)));
            assert_eq!(server_res, None);
        }

        #[test]
        fn updates_server_when_it_is_deleted_on_server_and_on_client_and_client_has_more_up_to_date_information(
        ) {
            let client = create_project(1, later(), Some(sooner()));
            let server = create_project(2, sooner(), Some(sooner()));

            let (client_res, server_res) =
                resolve_single(Some(client.clone()), Some(server.clone()));

            assert_eq!(client_res, None);
            assert_eq!(server_res, Some(update(server.id, &client)));
        }

        #[test]
        fn updates_client_when_server_has_more_up_to_date_information() {
            let client = create_project(1, sooner(), None);
            let server = create_project(2, later(), None);

            let (client_res, server_res) =
                resolve_single(Some(client.clone()), Some(server.clone()));

            assert_eq!(client_res, Some(update(client.id, &server)));
            assert_eq!(server_res, None);
        }

        #[test]
        fn updates_server_when_client_has_more_up_to_date_information() {
            let client = create_project(1, later(), None);
            let server = create_project(2, sooner(), None);

            let (client_res, server_res) =
                resolve_single(Some(client.clone()), Some(server.clone()));

            assert_eq!(client_res, None);
            assert_eq!(server_res, Some(update(server.id, &client)));
        }
    }

    mod form_pairs {
        use super::super::form_pairs;
        use crate::models::Project;
        use crate::toggl_api::models::Id;
        use chrono::{TimeZone, Utc};

        fn proj(id: Id) -> Project {
            Project {
                id,
                name: "ABC".to_string(),
                color: "#ff0000".to_string(),
                active: true,
                at: Utc.ymd(2019, 12, 09).and_hms(12, 00, 00),
                server_deleted_at: None,
            }
        }

        #[test]
        fn forms_pairs_for_between_all_entities() {
            let client = vec![proj(1)];
            let server = vec![proj(1)];

            let pairs = form_pairs(client, server);

            assert_eq!(pairs.len(), 1);
            assert_eq!(pairs[0].0.is_some(), true);
            assert_eq!(pairs[0].1.is_some(), true);
        }
    }
}
