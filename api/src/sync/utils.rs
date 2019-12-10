use super::prelude::{ConflictResolution, SyncResolution};
use crate::models::Entity;
use crate::toggl_api::models::Id;
use serde::Serialize;
use ConflictResolution::{Create, Error, Update};

pub fn share_entities(a: &SyncResolution, b: &SyncResolution) -> bool {
    entities_match(&a.user, &b.user)
        || overlap(&a.projects, &b.projects)
        || overlap(&a.time_entries, &b.time_entries)
}

fn entities_match<T>(a: &Option<ConflictResolution<T>>, b: &Option<ConflictResolution<T>>) -> bool
where
    T: Entity + Serialize,
{
    match (a, b) {
        (Some(x), Some(y)) => outcome_id(x) == outcome_id(y),
        _ => false,
    }
}

fn overlap<T>(x: &Vec<ConflictResolution<T>>, y: &Vec<ConflictResolution<T>>) -> bool
where
    T: Entity + Serialize,
{
    let ids_x = x.iter().map(outcome_id).collect::<Vec<_>>();
    y.iter().map(outcome_id).any(|id| ids_x.contains(&id))
}

fn outcome_id<T>(outcome: &ConflictResolution<T>) -> Id
where
    T: Entity + Serialize,
{
    match outcome {
        Create { entity } => entity.id(),
        Update { id: _, entity } => entity.id(),
        Error {
            id,
            code: _,
            message: _,
        } => *id,
    }
}
