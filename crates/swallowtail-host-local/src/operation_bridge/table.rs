//! Per-turn lease ownership and monotonic generation allocation.

use std::collections::BTreeMap;
use std::sync::Arc;
use swallowtail_runtime::RuntimeTurnId;

/// One registry of live operation-scoped leases keyed by open generation.
///
/// At most one lease per turn attempt is open at a time. Generations are
/// monotonic and never reused, so a retired lease can never be revived.
pub(crate) struct LeaseTable<L> {
    next_generation: u64,
    by_turn: BTreeMap<RuntimeTurnId, u64>,
    live: BTreeMap<u64, Arc<L>>,
}

impl<L> Default for LeaseTable<L> {
    fn default() -> Self {
        Self {
            next_generation: 1,
            by_turn: BTreeMap::new(),
            live: BTreeMap::new(),
        }
    }
}

impl<L> LeaseTable<L> {
    /// Returns the open generation for one turn attempt, when present.
    pub(crate) fn generation_for_turn(&self, turn: &RuntimeTurnId) -> Option<u64> {
        self.by_turn.get(turn).copied()
    }

    /// Reserves the next monotonic generation for one turn attempt.
    pub(crate) fn reserve(&mut self, turn: &RuntimeTurnId) -> Option<u64> {
        if self.by_turn.contains_key(turn) {
            return None;
        }
        let generation = self.next_generation;
        self.next_generation = self.next_generation.saturating_add(1);
        self.by_turn.insert(turn.clone(), generation);
        Some(generation)
    }

    /// Records the live lease for a reserved generation.
    pub(crate) fn insert(&mut self, generation: u64, lease: Arc<L>) {
        self.live.insert(generation, lease);
    }

    /// Returns the live lease for one exact generation.
    pub(crate) fn get(&self, generation: u64) -> Option<Arc<L>> {
        self.live.get(&generation).cloned()
    }

    /// Releases one turn's reservation and its live lease.
    pub(crate) fn forget(&mut self, turn: &RuntimeTurnId, generation: u64) {
        if self.by_turn.get(turn) == Some(&generation) {
            self.by_turn.remove(turn);
        }
        self.live.remove(&generation);
    }

    /// Returns the number of live leases this table still owns.
    pub(crate) fn live_count(&self) -> usize {
        self.live.len()
    }
}
