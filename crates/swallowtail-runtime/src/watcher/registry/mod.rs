//! Pure turn-scoped watcher registry.

mod failure;
mod lifecycle;
mod records;

pub use failure::{WatcherFailure, WatcherFailureKind};
pub use records::{WatcherSnapshot, WatcherStopAcknowledgement, WatcherWaitRepresentation};

use crate::{ActivityId, RuntimeTurnId};
use records::WatcherRecord;
use std::collections::BTreeMap;
use swallowtail_core::{DEFAULT_MAX_WATCHERS_PER_TURN, WatcherId, WatcherOwningTurn};

/// Pure deterministic registry for one turn-owned watcher set.
///
/// The registry retains lifecycle truth only. It never launches work, selects a
/// provider route, or stores executable paths, commands, PIDs, or raw output.
pub struct WatcherRegistry {
    owning_turn: WatcherOwningTurn,
    runtime_turn: RuntimeTurnId,
    maximum_watchers: usize,
    namespace: u64,
    next_sequence: u64,
    order: Vec<WatcherId>,
    records: BTreeMap<WatcherId, WatcherRecord>,
}

impl WatcherRegistry {
    /// Creates a registry for one runtime turn and positive watcher bound.
    pub fn new(
        runtime_turn: RuntimeTurnId,
        maximum_watchers: usize,
    ) -> Result<Self, WatcherFailure> {
        Self::new_with_namespace(runtime_turn, maximum_watchers, 0)
    }

    /// Creates a registry with a host-owned identity namespace.
    ///
    /// A non-zero namespace prevents a stale watcher id from aliasing a later
    /// registry if a consumer reuses a retired runtime-turn key.
    pub fn new_with_namespace(
        runtime_turn: RuntimeTurnId,
        maximum_watchers: usize,
        namespace: u64,
    ) -> Result<Self, WatcherFailure> {
        if maximum_watchers == 0 {
            return Err(WatcherFailure::new(WatcherFailureKind::InvalidCapacity));
        }
        let owning_turn = WatcherOwningTurn::new(runtime_turn.as_str().to_owned())
            .map_err(|_| WatcherFailure::new(WatcherFailureKind::ForeignIdentity))?;
        // Prove at least one turn-bound id fits the public byte bound.
        let _ = WatcherId::new(format!(
            "{}/{}",
            owning_turn.as_str(),
            watcher_suffix(namespace, 1)
        ))
        .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidCapacity))?;
        let _ = ActivityId::new(format!(
            "{}/{}",
            owning_turn.as_str(),
            activity_suffix(namespace, 1)
        ))
        .map_err(|_| WatcherFailure::new(WatcherFailureKind::InvalidCapacity))?;
        Ok(Self {
            owning_turn,
            runtime_turn,
            maximum_watchers,
            namespace,
            next_sequence: 1,
            order: Vec::new(),
            records: BTreeMap::new(),
        })
    }

    pub(super) fn watcher_id(&self, sequence: u64) -> String {
        watcher_suffix(self.namespace, sequence)
    }

    pub(super) fn activity_id(&self, sequence: u64) -> String {
        activity_suffix(self.namespace, sequence)
    }

    /// Creates a registry using the default turn capacity.
    pub fn with_default_capacity(runtime_turn: RuntimeTurnId) -> Result<Self, WatcherFailure> {
        Self::new(runtime_turn, DEFAULT_MAX_WATCHERS_PER_TURN)
    }

    #[must_use]
    /// Returns the runtime turn that owns this registry.
    pub const fn runtime_turn(&self) -> &RuntimeTurnId {
        &self.runtime_turn
    }

    #[must_use]
    /// Returns the opaque owning-turn key.
    pub const fn owning_turn(&self) -> &WatcherOwningTurn {
        &self.owning_turn
    }

    #[must_use]
    /// Returns the configured positive watcher bound.
    pub const fn maximum_watchers(&self) -> usize {
        self.maximum_watchers
    }

    #[must_use]
    /// Returns the number of retained watchers.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    #[must_use]
    /// Reports whether the registry retains no watchers.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Reports whether every owned watcher is terminal and joined.
    #[must_use]
    pub fn all_joined(&self) -> bool {
        self.records
            .values()
            .all(|record| record.phase == swallowtail_core::WatcherLifecyclePhase::Joined)
    }

    fn require_owning_turn(&self, owning_turn: &WatcherOwningTurn) -> Result<(), WatcherFailure> {
        if owning_turn == &self.owning_turn {
            Ok(())
        } else {
            Err(WatcherFailure::new(WatcherFailureKind::ForeignIdentity))
        }
    }

    fn record(&self, watcher_id: &WatcherId) -> Result<&WatcherRecord, WatcherFailure> {
        self.records
            .get(watcher_id)
            .ok_or_else(|| WatcherFailure::new(WatcherFailureKind::UnknownWatcher))
    }

    fn record_mut(&mut self, watcher_id: &WatcherId) -> Result<&mut WatcherRecord, WatcherFailure> {
        self.records
            .get_mut(watcher_id)
            .ok_or_else(|| WatcherFailure::new(WatcherFailureKind::UnknownWatcher))
    }

    fn snapshot(&self, watcher_id: &WatcherId) -> Result<WatcherSnapshot, WatcherFailure> {
        let record = self.record(watcher_id)?;
        Ok(WatcherSnapshot::from_record(
            watcher_id.clone(),
            self.owning_turn.clone(),
            record,
        ))
    }
}

fn watcher_suffix(namespace: u64, sequence: u64) -> String {
    if namespace == 0 {
        format!("watcher-{sequence}")
    } else {
        format!("watcher-{namespace}-{sequence}")
    }
}

fn activity_suffix(namespace: u64, sequence: u64) -> String {
    if namespace == 0 {
        format!("watcher-activity-{sequence}")
    } else {
        format!("watcher-activity-{namespace}-{sequence}")
    }
}
