use super::{
    WatcherFailure, WatcherRegistry, WatcherSnapshot, WatcherStopAcknowledgement,
    WatcherWaitRepresentation,
};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    WatcherCleanupCause, WatcherId, WatcherOperationData, WatcherOwningTurn, WatcherRequester,
};

/// Shared pure registry state used by distinct model and operator control roles.
pub type SharedWatcherRegistry = Arc<Mutex<WatcherRegistry>>;

/// Object-safe model control path against one shared watcher registry.
pub trait ModelWatcherControl: Send + Sync {
    /// Accepts a model-requested start without launching work.
    fn accept_start(
        &self,
        operation_data: WatcherOperationData,
    ) -> Result<WatcherSnapshot, WatcherFailure>;

    /// Inspects one watcher owned by the active turn.
    fn inspect(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherSnapshot, WatcherFailure>;

    /// Lists watchers owned by the active turn.
    fn list(&self, owning_turn: &WatcherOwningTurn)
    -> Result<Vec<WatcherSnapshot>, WatcherFailure>;

    /// Returns wait gating truth for one owned watcher.
    fn wait(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherWaitRepresentation, WatcherFailure>;

    /// Requests stop through the model path.
    fn stop(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<(WatcherStopAcknowledgement, WatcherSnapshot), WatcherFailure>;
}

/// Object-safe operator control path against the same shared watcher registry.
pub trait OperatorWatcherControl: Send + Sync {
    /// Accepts an operator-requested start without launching work.
    fn accept_start(
        &self,
        operation_data: WatcherOperationData,
    ) -> Result<WatcherSnapshot, WatcherFailure>;

    /// Inspects one watcher owned by the active turn.
    fn inspect(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherSnapshot, WatcherFailure>;

    /// Lists watchers owned by the active turn.
    fn list(&self, owning_turn: &WatcherOwningTurn)
    -> Result<Vec<WatcherSnapshot>, WatcherFailure>;

    /// Returns wait gating truth for one owned watcher.
    fn wait(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherWaitRepresentation, WatcherFailure>;

    /// Requests stop through the operator path.
    fn stop(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<(WatcherStopAcknowledgement, WatcherSnapshot), WatcherFailure>;

    /// Stops and joins every owned watcher for turn cancellation or deadline.
    fn stop_and_join_all(
        &self,
        cause: WatcherCleanupCause,
    ) -> Result<Vec<WatcherSnapshot>, WatcherFailure>;
}

/// Dual control surface retaining model and operator requester identity.
///
/// Both roles mutate one registry. Requester identity is preserved on accept
/// and in call attribution; state is never duplicated per role.
#[derive(Clone)]
pub struct WatcherControlSurface {
    registry: SharedWatcherRegistry,
}

impl WatcherControlSurface {
    /// Wraps one shared registry for distinct model and operator roles.
    #[must_use]
    pub fn new(registry: SharedWatcherRegistry) -> Self {
        Self { registry }
    }

    #[must_use]
    /// Returns the shared registry handle.
    pub fn registry(&self) -> &SharedWatcherRegistry {
        &self.registry
    }

    #[must_use]
    /// Returns the model control role.
    pub fn model(&self) -> ModelWatcherRole {
        ModelWatcherRole {
            registry: Arc::clone(&self.registry),
        }
    }

    #[must_use]
    /// Returns the operator control role.
    pub fn operator(&self) -> OperatorWatcherRole {
        OperatorWatcherRole {
            registry: Arc::clone(&self.registry),
        }
    }
}

/// Model-facing control role over one shared registry.
#[derive(Clone)]
pub struct ModelWatcherRole {
    registry: SharedWatcherRegistry,
}

impl ModelWatcherControl for ModelWatcherRole {
    fn accept_start(
        &self,
        operation_data: WatcherOperationData,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        self.with_registry(|registry| {
            registry.accept_start(WatcherRequester::Model, operation_data)
        })
    }

    fn inspect(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        self.with_registry(|registry| registry.inspect(owning_turn, watcher_id))
    }

    fn list(
        &self,
        owning_turn: &WatcherOwningTurn,
    ) -> Result<Vec<WatcherSnapshot>, WatcherFailure> {
        self.with_registry(|registry| registry.list(owning_turn))
    }

    fn wait(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherWaitRepresentation, WatcherFailure> {
        self.with_registry(|registry| registry.wait_representation(owning_turn, watcher_id))
    }

    fn stop(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<(WatcherStopAcknowledgement, WatcherSnapshot), WatcherFailure> {
        self.with_registry(|registry| registry.request_stop(owning_turn, watcher_id))
    }
}

impl ModelWatcherRole {
    fn with_registry<T>(
        &self,
        operation: impl FnOnce(&mut WatcherRegistry) -> Result<T, WatcherFailure>,
    ) -> Result<T, WatcherFailure> {
        let mut registry = self
            .registry
            .lock()
            .expect("watcher registry lock poisoned");
        operation(&mut registry)
    }
}

/// Operator-facing control role over the same shared registry.
#[derive(Clone)]
pub struct OperatorWatcherRole {
    registry: SharedWatcherRegistry,
}

impl OperatorWatcherControl for OperatorWatcherRole {
    fn accept_start(
        &self,
        operation_data: WatcherOperationData,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        self.with_registry(|registry| {
            registry.accept_start(WatcherRequester::Operator, operation_data)
        })
    }

    fn inspect(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherSnapshot, WatcherFailure> {
        self.with_registry(|registry| registry.inspect(owning_turn, watcher_id))
    }

    fn list(
        &self,
        owning_turn: &WatcherOwningTurn,
    ) -> Result<Vec<WatcherSnapshot>, WatcherFailure> {
        self.with_registry(|registry| registry.list(owning_turn))
    }

    fn wait(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<WatcherWaitRepresentation, WatcherFailure> {
        self.with_registry(|registry| registry.wait_representation(owning_turn, watcher_id))
    }

    fn stop(
        &self,
        owning_turn: &WatcherOwningTurn,
        watcher_id: &WatcherId,
    ) -> Result<(WatcherStopAcknowledgement, WatcherSnapshot), WatcherFailure> {
        self.with_registry(|registry| registry.request_stop(owning_turn, watcher_id))
    }

    fn stop_and_join_all(
        &self,
        cause: WatcherCleanupCause,
    ) -> Result<Vec<WatcherSnapshot>, WatcherFailure> {
        self.with_registry(|registry| registry.stop_and_join_all(cause))
    }
}

impl OperatorWatcherRole {
    fn with_registry<T>(
        &self,
        operation: impl FnOnce(&mut WatcherRegistry) -> Result<T, WatcherFailure>,
    ) -> Result<T, WatcherFailure> {
        let mut registry = self
            .registry
            .lock()
            .expect("watcher registry lock poisoned");
        operation(&mut registry)
    }
}
