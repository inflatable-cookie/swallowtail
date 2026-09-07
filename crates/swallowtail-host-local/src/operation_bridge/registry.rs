//! The one lease, generation, and lifecycle owner shared by both profiles.

use crate::registered_tool::LiveRegisteredLease;
use crate::watcher_bridge::LiveWatcherLease;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};
use swallowtail_runtime::{CleanupOutcome, RuntimeFailure, RuntimeTurnId};

/// Closed profile one operation-bridge lease belongs to.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BridgeProfile {
    /// The Contract 060 watcher profile.
    Watcher,
    /// The Contract 063 registered-tool profile.
    RegisteredTool,
}

impl BridgeProfile {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Watcher => "watcher",
            Self::RegisteredTool => "registered-tool",
        }
    }
}

/// Cause that starts one joined teardown across every selected profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationBridgeCleanupCause {
    /// The turn completed normally.
    Completion,
    /// The operation was cancelled.
    Cancellation,
    /// The operation or a call reached its deadline.
    Deadline,
    /// The provider failed.
    ProviderFailure,
    /// A transport failed or was lost.
    TransportFailure,
    /// The consumer closed the operation explicitly.
    ExplicitClose,
}

/// Profile-specific teardown for one lease the shared registry owns.
pub(crate) trait BridgeLeaseOwner: Send + Sync {
    /// Freezes new admission for this lease.
    fn freeze(&self);

    /// Joins this lease's work and releases it from the shared registry.
    fn join_and_release(
        &self,
        cause: OperationBridgeCleanupCause,
    ) -> Result<CleanupOutcome, RuntimeFailure>;
}

/// Typed handle to one live lease, resolved by its shared generation.
#[derive(Clone)]
pub(crate) enum BridgeLease {
    Watcher(Arc<LiveWatcherLease>),
    RegisteredTool(Arc<LiveRegisteredLease>),
}

struct OwnedLease {
    profile: BridgeProfile,
    turn: RuntimeTurnId,
    lease: BridgeLease,
    owner: Arc<dyn BridgeLeaseOwner>,
}

#[derive(Default)]
struct RegistryState {
    next_generation: u64,
    by_turn: BTreeMap<(RuntimeTurnId, BridgeProfile), u64>,
    owned: BTreeMap<u64, OwnedLease>,
}

/// One private-operation bridge registry for every closed profile.
///
/// Both the watcher and the registered-tool profile draw generations from this
/// one monotonic counter, register their per-turn ownership here, and release
/// here. There is no second lease manager, generation space, or lifecycle
/// owner, so an operation that selects both profiles has one joined teardown.
pub(crate) struct OperationBridgeRegistry {
    state: Mutex<RegistryState>,
}

impl Default for OperationBridgeRegistry {
    fn default() -> Self {
        Self {
            state: Mutex::new(RegistryState {
                next_generation: 1,
                by_turn: BTreeMap::new(),
                owned: BTreeMap::new(),
            }),
        }
    }
}

impl OperationBridgeRegistry {
    fn locked(&self) -> MutexGuard<'_, RegistryState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Reserves the next shared generation for one profile on one turn.
    ///
    /// Returns `None` when that profile already owns an open lease for the turn.
    pub(crate) fn reserve(&self, profile: BridgeProfile, turn: &RuntimeTurnId) -> Option<u64> {
        let mut state = self.locked();
        if state.by_turn.contains_key(&(turn.clone(), profile)) {
            return None;
        }
        let generation = state.next_generation;
        state.next_generation = state.next_generation.saturating_add(1);
        state.by_turn.insert((turn.clone(), profile), generation);
        Some(generation)
    }

    /// Records the live lease and its profile teardown for a reservation.
    pub(crate) fn attach(
        &self,
        generation: u64,
        profile: BridgeProfile,
        turn: RuntimeTurnId,
        lease: BridgeLease,
        owner: Arc<dyn BridgeLeaseOwner>,
    ) {
        self.locked().owned.insert(
            generation,
            OwnedLease {
                profile,
                turn,
                lease,
                owner,
            },
        );
    }

    /// Returns the live watcher lease for one exact shared generation.
    pub(crate) fn watcher_lease(&self, generation: u64) -> Option<Arc<LiveWatcherLease>> {
        match self
            .locked()
            .owned
            .get(&generation)
            .map(|owned| &owned.lease)
        {
            Some(BridgeLease::Watcher(lease)) => Some(Arc::clone(lease)),
            _ => None,
        }
    }

    /// Returns the live registered-tool lease for one exact shared generation.
    pub(crate) fn registered_lease(&self, generation: u64) -> Option<Arc<LiveRegisteredLease>> {
        match self
            .locked()
            .owned
            .get(&generation)
            .map(|owned| &owned.lease)
        {
            Some(BridgeLease::RegisteredTool(lease)) => Some(Arc::clone(lease)),
            _ => None,
        }
    }

    /// Releases one profile's reservation and its live lease.
    pub(crate) fn forget(&self, profile: BridgeProfile, turn: &RuntimeTurnId, generation: u64) {
        let mut state = self.locked();
        if state.by_turn.get(&(turn.clone(), profile)) == Some(&generation) {
            state.by_turn.remove(&(turn.clone(), profile));
        }
        state.owned.remove(&generation);
    }

    /// Returns how many live leases this registry owns across both profiles.
    pub(crate) fn lease_count(&self) -> usize {
        self.locked().owned.len()
    }

    /// Returns how many live leases one profile owns.
    pub(crate) fn lease_count_for(&self, profile: BridgeProfile) -> usize {
        self.locked()
            .owned
            .values()
            .filter(|owned| owned.profile == profile)
            .count()
    }

    /// Returns every profile and shared generation owned for one turn.
    pub(crate) fn generations_for_turn(&self, turn: &RuntimeTurnId) -> Vec<(BridgeProfile, u64)> {
        let state = self.locked();
        let mut owned: Vec<_> = state
            .owned
            .iter()
            .filter(|(_, lease)| &lease.turn == turn)
            .map(|(generation, lease)| (lease.profile, *generation))
            .collect();
        owned.sort_unstable();
        owned
    }

    /// Freezes and joins every profile lease owned for one turn.
    ///
    /// Admission freezes across both profiles first, then each lease joins under
    /// the one sequence. A failed join dominates the reported outcome and its
    /// lease stays owned here.
    pub(crate) fn close_turn(
        &self,
        turn: &RuntimeTurnId,
        cause: OperationBridgeCleanupCause,
    ) -> Result<CleanupOutcome, RuntimeFailure> {
        let owners: Vec<Arc<dyn BridgeLeaseOwner>> = {
            let state = self.locked();
            state
                .owned
                .values()
                .filter(|owned| &owned.turn == turn)
                .map(|owned| Arc::clone(&owned.owner))
                .collect()
        };
        for owner in &owners {
            owner.freeze();
        }
        let mut outcome = CleanupOutcome::NotApplicable;
        for owner in &owners {
            outcome = combine(outcome, owner.join_and_release(cause)?);
        }
        Ok(outcome)
    }
}

fn combine(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (&left, &right) {
        (CleanupOutcome::Failed(_), _) => left,
        (_, CleanupOutcome::Failed(_)) => right,
        (CleanupOutcome::Degraded(_), _) => left,
        (_, CleanupOutcome::Degraded(_)) => right,
        (CleanupOutcome::Clean, _) | (_, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        _ => CleanupOutcome::NotApplicable,
    }
}
