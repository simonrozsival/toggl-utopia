use super::prelude::SyncResolution;
use crate::models::{Delta, Project, TimeEntry, User};

#[derive(Debug, PartialEq)]
enum Resolution<T> {
    Client(T),
    Server(T),
}

trait Resolvable<T> {
    fn resolve_conflict(client_entity: &T, server_entity: &T) -> Resolution<T>;
}

pub fn resolve(client: &Delta, server: &Delta) -> (SyncResolution, SyncResolution) {
    unimplemented!("This needs to be implemented");
}
