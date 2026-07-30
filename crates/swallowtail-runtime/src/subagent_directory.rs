use crate::{
    ActivityActor, ActivityObservation, ActivityOperationId, RuntimeEvent, RuntimeEventKind,
    SubagentId, SubagentParent, SubagentSnapshot, SubagentStatus,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubagentDirectoryChangeKind {
    Added,
    Replaced,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubagentDirectoryChange {
    id: SubagentId,
    kind: SubagentDirectoryChangeKind,
}

impl SubagentDirectoryChange {
    const fn new(id: SubagentId, kind: SubagentDirectoryChangeKind) -> Self {
        Self { id, kind }
    }

    #[must_use]
    pub const fn id(&self) -> &SubagentId {
        &self.id
    }

    #[must_use]
    pub const fn kind(&self) -> SubagentDirectoryChangeKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubagentDirectoryDelta {
    actor: ActivityActor,
    changes: Vec<SubagentDirectoryChange>,
}

impl SubagentDirectoryDelta {
    const fn new(actor: ActivityActor, changes: Vec<SubagentDirectoryChange>) -> Self {
        Self { actor, changes }
    }

    #[must_use]
    pub const fn actor(&self) -> &ActivityActor {
        &self.actor
    }

    pub fn changes(&self) -> impl ExactSizeIterator<Item = &SubagentDirectoryChange> {
        self.changes.iter()
    }

    #[must_use]
    pub fn is_unchanged(&self) -> bool {
        self.changes.is_empty()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubagentDirectoryFailureKind {
    InvalidCapacity,
    OperationMismatch,
    CapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubagentDirectoryFailure {
    kind: SubagentDirectoryFailureKind,
}

impl SubagentDirectoryFailure {
    const fn new(kind: SubagentDirectoryFailureKind) -> Self {
        Self { kind }
    }

    #[must_use]
    pub const fn kind(self) -> SubagentDirectoryFailureKind {
        self.kind
    }
}

impl fmt::Display for SubagentDirectoryFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self.kind {
            SubagentDirectoryFailureKind::InvalidCapacity => {
                "Subagent directory requires a positive capacity"
            }
            SubagentDirectoryFailureKind::OperationMismatch => {
                "Subagent directory received activity from another operation"
            }
            SubagentDirectoryFailureKind::CapacityExceeded => {
                "Subagent directory exceeded its configured capacity"
            }
        };
        formatter.write_str(message)
    }
}

impl Error for SubagentDirectoryFailure {}

/// Bounded transient child-work projection for one operation.
///
/// The projection stores current topology only. It does not retain activity
/// content, transcripts, consumer selection, or provider control authority.
pub struct SubagentDirectoryProjection {
    operation_id: ActivityOperationId,
    maximum_subagents: usize,
    order: Vec<SubagentId>,
    snapshots: BTreeMap<SubagentId, SubagentSnapshot>,
}

impl SubagentDirectoryProjection {
    pub fn new(
        operation_id: ActivityOperationId,
        maximum_subagents: usize,
    ) -> Result<Self, SubagentDirectoryFailure> {
        if maximum_subagents == 0 {
            return Err(SubagentDirectoryFailure::new(
                SubagentDirectoryFailureKind::InvalidCapacity,
            ));
        }
        Ok(Self {
            operation_id,
            maximum_subagents,
            order: Vec::new(),
            snapshots: BTreeMap::new(),
        })
    }

    #[must_use]
    pub const fn operation_id(&self) -> &ActivityOperationId {
        &self.operation_id
    }

    #[must_use]
    pub const fn maximum_subagents(&self) -> usize {
        self.maximum_subagents
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }

    #[must_use]
    pub fn get(&self, id: &SubagentId) -> Option<&SubagentSnapshot> {
        self.snapshots.get(id)
    }

    pub fn subagents(&self) -> impl ExactSizeIterator<Item = &SubagentSnapshot> {
        self.order.iter().map(|id| {
            self.snapshots
                .get(id)
                .expect("ordered subagent identity must have a snapshot")
        })
    }

    pub fn operation_children(&self) -> impl Iterator<Item = &SubagentSnapshot> {
        self.subagents()
            .filter(|snapshot| snapshot.parent() == &SubagentParent::Operation)
    }

    pub fn children_of<'a>(
        &'a self,
        parent: &'a SubagentId,
    ) -> impl Iterator<Item = &'a SubagentSnapshot> + 'a {
        self.subagents().filter(move |snapshot| {
            matches!(
                snapshot.parent(),
                SubagentParent::Subagent(candidate) if candidate == parent
            )
        })
    }

    pub fn unknown_parent(&self) -> impl Iterator<Item = &SubagentSnapshot> {
        self.subagents()
            .filter(|snapshot| snapshot.parent() == &SubagentParent::Unknown)
    }

    pub fn observe_event(
        &mut self,
        event: &RuntimeEvent,
    ) -> Result<Option<SubagentDirectoryDelta>, SubagentDirectoryFailure> {
        let RuntimeEventKind::Activity(activity) = event.kind() else {
            return Ok(None);
        };
        self.observe_activity(activity).map(Some)
    }

    pub fn observe_activity(
        &mut self,
        activity: &ActivityObservation,
    ) -> Result<SubagentDirectoryDelta, SubagentDirectoryFailure> {
        if activity.operation_id() != &self.operation_id {
            return Err(SubagentDirectoryFailure::new(
                SubagentDirectoryFailureKind::OperationMismatch,
            ));
        }

        let supplied = activity.subagents().cloned().collect::<Vec<_>>();
        let supplied_ids = supplied
            .iter()
            .map(|snapshot| snapshot.id().clone())
            .collect::<BTreeSet<_>>();
        let mut candidate_ids = BTreeSet::new();
        let mut candidates = Vec::new();

        for snapshot in &supplied {
            let SubagentParent::Subagent(parent) = snapshot.parent() else {
                continue;
            };
            if !self.snapshots.contains_key(parent)
                && !supplied_ids.contains(parent)
                && candidate_ids.insert(parent.clone())
            {
                candidates.push(SubagentSnapshot::new(
                    parent.clone(),
                    SubagentParent::Unknown,
                    SubagentStatus::Unknown,
                ));
            }
        }

        if let ActivityActor::Subagent(actor) = activity.actor()
            && !self.snapshots.contains_key(actor)
            && !supplied_ids.contains(actor)
            && candidate_ids.insert(actor.clone())
        {
            candidates.push(SubagentSnapshot::new(
                actor.clone(),
                SubagentParent::Unknown,
                SubagentStatus::Unknown,
            ));
        }

        for snapshot in supplied {
            candidate_ids.insert(snapshot.id().clone());
            candidates.push(snapshot);
        }

        let added = candidates
            .iter()
            .filter(|snapshot| !self.snapshots.contains_key(snapshot.id()))
            .count();
        if self.snapshots.len().saturating_add(added) > self.maximum_subagents {
            return Err(SubagentDirectoryFailure::new(
                SubagentDirectoryFailureKind::CapacityExceeded,
            ));
        }

        let mut changes = Vec::new();
        for snapshot in candidates {
            let id = snapshot.id().clone();
            match self.snapshots.get(&id) {
                None => {
                    self.order.push(id.clone());
                    self.snapshots.insert(id.clone(), snapshot);
                    changes.push(SubagentDirectoryChange::new(
                        id,
                        SubagentDirectoryChangeKind::Added,
                    ));
                }
                Some(current) if current != &snapshot => {
                    self.snapshots.insert(id.clone(), snapshot);
                    changes.push(SubagentDirectoryChange::new(
                        id,
                        SubagentDirectoryChangeKind::Replaced,
                    ));
                }
                Some(_) => {}
            }
        }

        Ok(SubagentDirectoryDelta::new(
            activity.actor().clone(),
            changes,
        ))
    }
}

#[cfg(test)]
mod tests;
