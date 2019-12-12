use crate::models::{Delta, Entity};
use crate::toggl_api::models::Id;

type Pair<T> = (T, T);

fn prefer_newer<T: Entity>(client: T, server: T) -> Pair<Option<T>> {
    if client.last_update() > server.last_update() {
        (None, Some(client))
    } else {
        (Some(server), None)
    }
}

fn resolve_single<T: Entity>(client: Option<T>, server: Option<T>) -> Pair<Option<T>> {
    match (client, server) {
        (None,    None)     => (None, None),
        (Some(c), None)     => (None, Some(c)),
        (None,    Some(s))  => (Some(s), None),
        (Some(c), Some(s)) if !c.is_deleted() && s.is_deleted()
            // we shouldn't update an entity which was already deleted on the server, we can't un-delete it
            => (Some(s), None),
        (Some(c), Some(s))  => prefer_newer(c, s)
    }
}

fn pair<T: Entity>(client: Vec<T>, server: Vec<T>) -> Vec<Pair<Option<T>>> {
    use std::collections::HashMap;

    let mut server_entities: HashMap<Id, T> =
        server.iter().fold(HashMap::new(), |mut acc, entity| {
            acc.insert(entity.id(), entity.clone());
            acc
        });

    let mut pairs: Vec<(Option<T>, Option<T>)> = Vec::new();

    for client_entity in client.iter() {
        let maybe_server_entity = server_entities.get(&client_entity.id()).cloned();

        if maybe_server_entity.is_some() {
            server_entities.remove(&client_entity.id());
        }

        pairs.push((Some(client_entity.clone()), maybe_server_entity));
    }

    for (_, server_entity) in server_entities.iter() {
        pairs.push((None, Some(server_entity.clone())));
    }

    pairs
}

pub fn filter_map<T: Entity>(data: Vec<Option<T>>) -> Vec<T> {
    data.into_iter().filter_map(|x| x).collect()
}

fn resolve_many<T: Entity>(client: Option<Vec<T>>, server: Option<Vec<T>>) -> Pair<Vec<T>> {
    let (for_client, for_server): Pair<Vec<Option<T>>> =
        pair(client.unwrap_or_default(), server.unwrap_or_default())
            .into_iter()
            .map(|(c, s)| resolve_single(c, s))
            .unzip();

    (filter_map(for_client), filter_map(for_server))
}

pub fn resolve(client: Delta, server: Delta) -> Pair<Delta> {
    let (client_user, server_user) = resolve_single(client.user, server.user);
    let (client_projects, server_projects) = resolve_many(client.projects, server.projects);
    let (client_time_entries, server_time_entries) =
        resolve_many(client.time_entries, server.time_entries); // todo: this needs additionally resolving two running TEs!

    (
        Delta {
            user: client_user,
            projects: Some(client_projects),
            time_entries: Some(client_time_entries),
        },
        Delta {
            user: server_user,
            projects: Some(server_projects),
            time_entries: Some(server_time_entries),
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::models::Project;
    use crate::toggl_api::models::Id;
    use chrono::{DateTime, TimeZone, Utc};

    fn create_project(id: Id, at: DateTime<Utc>, deleted_at: Option<DateTime<Utc>>) -> Project {
        Project {
            id,
            workspace_id: 0,
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

        #[test]
        fn prefers_client_if_it_was_updated_later() {
            let client = create_project(1, later(), None);
            let server = create_project(2, sooner(), None);

            let (client_res, server_res) = prefer_newer(client.clone(), server.clone());

            assert_eq!(client_res, None);
            assert_eq!(server_res, Some(client));
        }

        #[test]
        fn prefers_server_if_it_was_updated_later() {
            let client = create_project(1, sooner(), None);
            let server = create_project(2, later(), None);

            let (client_res, server_res) = prefer_newer(client.clone(), server.clone());

            assert_eq!(client_res, Some(server));
            assert_eq!(server_res, None);
        }
    }

    mod resolve_single {
        use super::super::resolve_single;
        use super::{create_project, later, sooner};

        #[test]
        fn create_on_server_when_there_is_not_a_counterpart_on_the_server() {
            let client = create_project(1, sooner(), None);

            let (client_res, server_res) = resolve_single(Some(client.clone()), None);

            assert_eq!(client_res, None);
            assert_eq!(server_res, Some(client));
        }

        #[test]
        fn creates_on_client_when_there_is_not_a_counterpart_on_the_client() {
            let server = create_project(1, sooner(), None);

            let (client_res, server_res) = resolve_single(None, Some(server.clone()));

            assert_eq!(client_res, Some(server));
            assert_eq!(server_res, None);
        }

        #[test]
        fn updates_client_when_it_is_deleted_on_server_but_not_on_client() {
            let client = create_project(1, later(), None);
            let server = create_project(2, sooner(), Some(sooner()));

            let (client_res, server_res) =
                resolve_single(Some(client.clone()), Some(server.clone()));

            assert_eq!(client_res, Some(server));
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
            assert_eq!(server_res, Some(client));
        }

        #[test]
        fn updates_client_when_server_has_more_up_to_date_information() {
            let client = create_project(1, sooner(), None);
            let server = create_project(2, later(), None);

            let (client_res, server_res) =
                resolve_single(Some(client.clone()), Some(server.clone()));

            assert_eq!(client_res, Some(server));
            assert_eq!(server_res, None);
        }

        #[test]
        fn updates_server_when_client_has_more_up_to_date_information() {
            let client = create_project(1, later(), None);
            let server = create_project(2, sooner(), None);

            let (client_res, server_res) =
                resolve_single(Some(client.clone()), Some(server.clone()));

            assert_eq!(client_res, None);
            assert_eq!(server_res, Some(client));
        }
    }

    mod pair {
        use super::super::pair;
        use crate::models::Project;
        use crate::toggl_api::models::Id;
        use chrono::{TimeZone, Utc};

        fn proj(id: Id) -> Project {
            Project {
                id,
                workspace_id: 0,
                name: "ABC".to_string(),
                color: "#ff0000".to_string(),
                active: true,
                at: Utc.ymd(2019, 12, 09).and_hms(12, 00, 00),
                server_deleted_at: None,
            }
        }

        #[test]
        fn forms_pairs_for_between_a_pair_of_entities() {
            let client = vec![proj(1)];
            let server = vec![proj(1)];

            let pairs = pair(client, server);

            assert_eq!(pairs.len(), 1);
            assert!(pairs[0].0.is_some());
            assert!(pairs[0].1.is_some());
        }

        #[test]
        fn forms_pairs_for_between_all_entities() {
            let client = vec![proj(1), proj(2), proj(3)];
            let server = vec![proj(3), proj(2), proj(1)];

            let pairs = pair(client, server);

            assert_eq!(pairs.len(), 3);
            assert_eq!(
                pairs
                    .iter()
                    .all(|pair| pair.0.is_some() && pair.1.is_some()),
                true
            );
            assert_eq!(
                pairs
                    .iter()
                    .all(|pair| pair.0.as_ref().unwrap().id == pair.1.as_ref().unwrap().id),
                true
            );
        }

        #[test]
        fn if_there_is_no_counterpar_then_it_should_be_none() {
            let client = vec![proj(1), proj(2)];
            let server = vec![proj(2), proj(3)];

            let pairs = pair(client, server);

            assert_eq!(pairs.len(), 3);
            // first the client
            assert!(pairs[0].0.is_some());
            assert!(pairs[0].1.is_none());
            // then the client+server pair
            assert!(pairs[1].0.is_some());
            assert!(pairs[1].1.is_some());
            // then the lonely entity on the server
            assert!(pairs[2].0.is_none());
            assert!(pairs[2].1.is_some());
        }
    }
}
