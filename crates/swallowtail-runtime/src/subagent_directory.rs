#![deny(missing_docs)]

use crate::{
    ActivityActor, ActivityObservation, ActivityOperationId, RuntimeEvent, RuntimeEventKind,
    SubagentId, SubagentParent, SubagentSnapshot, SubagentStatus,
};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

/// Kind of change applied to one transient child-work snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubagentDirectoryChangeKind {
    /// The child identity was observed for the first time.
    Added,
    /// Later provider truth replaced the child's complete prior snapshot.
    Replaced,
}

/// One child identity changed by a directory observation.
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
    /// Returns the operation-local child identity.
    pub const fn id(&self) -> &SubagentId {
        &self.id
    }

    #[must_use]
    /// Returns whether the snapshot was added or replaced.
    pub const fn kind(&self) -> SubagentDirectoryChangeKind {
        self.kind
    }
}

/// Ordered directory changes plus the exact actor of the source activity.
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
    /// Returns the primary or child actor attributed by the adapter.
    pub const fn actor(&self) -> &ActivityActor {
        &self.actor
    }

    /// Iterates directory changes in observation order.
    pub fn changes(&self) -> impl ExactSizeIterator<Item = &SubagentDirectoryChange> {
        self.changes.iter()
    }

    #[must_use]
    /// Reports whether the activity changed no retained child snapshot.
    pub fn is_unchanged(&self) -> bool {
        self.changes.is_empty()
    }
}

/// Stable reason a transient child-work projection rejected an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubagentDirectoryFailureKind {
    /// The requested directory capacity was zero.
    InvalidCapacity,
    /// Activity belongs to a different operation.
    OperationMismatch,
    /// New identities would exceed the explicit positive child bound.
    CapacityExceeded,
}

/// Safe failure returned by the transient subagent directory projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubagentDirectoryFailure {
    kind: SubagentDirectoryFailureKind,
}

impl SubagentDirectoryFailure {
    const fn new(kind: SubagentDirectoryFailureKind) -> Self {
        Self { kind }
    }

    #[must_use]
    /// Returns the stable projection-failure classification.
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
    /// Creates a transient directory for one operation and positive child bound.
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
    /// Returns the exact operation whose activity may update this directory.
    pub const fn operation_id(&self) -> &ActivityOperationId {
        &self.operation_id
    }

    #[must_use]
    /// Returns the maximum number of retained child identities.
    pub const fn maximum_subagents(&self) -> usize {
        self.maximum_subagents
    }

    #[must_use]
    /// Returns the number of currently retained child snapshots.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    #[must_use]
    /// Reports whether no child identity has been observed.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }

    #[must_use]
    /// Returns the current snapshot for one operation-local child identity.
    pub fn get(&self, id: &SubagentId) -> Option<&SubagentSnapshot> {
        self.snapshots.get(id)
    }

    /// Iterates current snapshots in first-observed identity order.
    pub fn subagents(&self) -> impl ExactSizeIterator<Item = &SubagentSnapshot> {
        self.order.iter().map(|id| {
            self.snapshots
                .get(id)
                .expect("ordered subagent identity must have a snapshot")
        })
    }

    /// Iterates children whose parent is the primary operation.
    pub fn operation_children(&self) -> impl Iterator<Item = &SubagentSnapshot> {
        self.subagents()
            .filter(|snapshot| snapshot.parent() == &SubagentParent::Operation)
    }

    /// Iterates children with one exact known child parent.
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

    /// Iterates children whose provider parentage remains unknown.
    pub fn unknown_parent(&self) -> impl Iterator<Item = &SubagentSnapshot> {
        self.subagents()
            .filter(|snapshot| snapshot.parent() == &SubagentParent::Unknown)
    }

    /// Observes an activity event, ignoring other common runtime event kinds.
    pub fn observe_event(
        &mut self,
        event: &RuntimeEvent,
    ) -> Result<Option<SubagentDirectoryDelta>, SubagentDirectoryFailure> {
        let RuntimeEventKind::Activity(activity) = event.kind() else {
            return Ok(None);
        };
        self.observe_activity(activity).map(Some)
    }

    /// Applies one exact-operation activity observation transactionally.
    ///
    /// Capacity or operation mismatch leaves the projection unchanged.
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
