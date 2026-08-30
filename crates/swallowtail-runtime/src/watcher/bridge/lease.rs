//! Driver-visible lease handle and public lifecycle truth.

use super::identity::{
    WatcherBridgeAdmission, WatcherBridgeBearer, WatcherBridgeEndpoint, WatcherBridgeGeneration,
    WatcherBridgeToken,
};
use crate::{RuntimeTurnId, ScopeId, WatcherSnapshot};
use std::fmt;
use swallowtail_core::ExecutionHostId;

/// Request that opens one operation-scoped bridge lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatcherBridgeOpenRequest {
    scope: ScopeId,
    turn: RuntimeTurnId,
}

impl WatcherBridgeOpenRequest {
    /// Binds the lease to one operation scope and owning turn.
    #[must_use]
    pub const fn new(scope: ScopeId, turn: RuntimeTurnId) -> Self {
        Self { scope, turn }
    }

    /// Returns the operation scope that will own the lease.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the turn whose watcher service the lease must use.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }
}

/// Bounded completion-gate observation for the bound turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatcherBridgeCompletionState {
    admission: WatcherBridgeAdmission,
    active_or_unjoined: Vec<WatcherSnapshot>,
}

impl WatcherBridgeCompletionState {
    /// Creates a completion observation from admission and remaining work.
    #[must_use]
    pub const fn new(
        admission: WatcherBridgeAdmission,
        active_or_unjoined: Vec<WatcherSnapshot>,
    ) -> Self {
        Self {
            admission,
            active_or_unjoined,
        }
    }

    /// Returns the lease admission recorded with this observation.
    #[must_use]
    pub const fn admission(&self) -> WatcherBridgeAdmission {
        self.admission
    }

    /// Returns the bounded active or unjoined watcher snapshots.
    #[must_use]
    pub fn active_or_unjoined(&self) -> &[WatcherSnapshot] {
        &self.active_or_unjoined
    }

    /// Reports whether successful completion may be admitted.
    #[must_use]
    pub fn allows_successful_completion(&self) -> bool {
        self.admission == WatcherBridgeAdmission::Frozen && self.active_or_unjoined.is_empty()
    }
}

/// Handle for one opened watcher-bridge lease.
///
/// Endpoint and bearer stay driver-only. Drop always runs host cleanup when a
/// live binding is present. Drivers cannot disarm that cleanup.
pub struct WatcherBridgeLease {
    execution_host_id: ExecutionHostId,
    scope: ScopeId,
    turn: RuntimeTurnId,
    generation: WatcherBridgeGeneration,
    endpoint: WatcherBridgeEndpoint,
    bearer: WatcherBridgeBearer,
    token: Option<WatcherBridgeToken>,
    release: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl WatcherBridgeLease {
    /// Creates a non-live handle for redaction and fixture use.
    ///
    /// This handle cannot close or freeze a host lease.
    #[must_use]
    pub fn new(
        execution_host_id: ExecutionHostId,
        scope: ScopeId,
        turn: RuntimeTurnId,
        generation: WatcherBridgeGeneration,
        endpoint: WatcherBridgeEndpoint,
        bearer: WatcherBridgeBearer,
    ) -> Self {
        Self {
            execution_host_id,
            scope,
            turn,
            generation,
            endpoint,
            bearer,
            token: None,
            release: None,
        }
    }

    /// Binds host cleanup and unforgeable identity for one live lease.
    ///
    /// A second bind is ignored so callers cannot replace or disarm cleanup.
    #[must_use]
    pub fn bind(
        mut self,
        token: WatcherBridgeToken,
        release: impl FnOnce() + Send + 'static,
    ) -> Self {
        if self.token.is_none() && self.release.is_none() {
            self.token = Some(token);
            self.release = Some(Box::new(release));
        }
        self
    }

    /// Reports whether this handle authenticates one live host token.
    #[must_use]
    pub fn binding_matches(&self, token: &WatcherBridgeToken) -> bool {
        self.token
            .as_ref()
            .is_some_and(|bound| bound.matches(token))
    }

    /// Returns the execution host bound at open.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Returns the operation scope bound at open.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the owning turn bound at open.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Returns the open generation bound at open.
    #[must_use]
    pub const fn generation(&self) -> WatcherBridgeGeneration {
        self.generation
    }

    /// Returns the driver-only endpoint bound at open.
    #[must_use]
    pub const fn endpoint(&self) -> &WatcherBridgeEndpoint {
        &self.endpoint
    }

    /// Returns the driver-only bearer bound at open.
    #[must_use]
    pub const fn bearer(&self) -> &WatcherBridgeBearer {
        &self.bearer
    }
}

impl Drop for WatcherBridgeLease {
    fn drop(&mut self) {
        if let Some(release) = self.release.take() {
            release();
        }
    }
}

impl fmt::Debug for WatcherBridgeLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WatcherBridgeLease")
            .field("execution_host_id", &self.execution_host_id)
            .field("scope", &self.scope)
            .field("turn", &self.turn)
            .field("generation", &self.generation)
            .field("endpoint", &self.endpoint)
            .field("bearer", &self.bearer)
            .field("token", &self.token.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

#[cfg(test)]
mod tests;
