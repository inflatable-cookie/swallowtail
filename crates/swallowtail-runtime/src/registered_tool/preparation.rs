//! Immutable opt-in preparation and the prepared registered-tool binding.

use super::admission::ConsumerAdmissionBinding;
use super::failure::{RegisteredToolFailureKind, fail};
use super::lease::{RegisteredToolBridgeLease, RegisteredToolOpenRequest};
use super::limits::{RegisteredToolBounds, RegisteredToolLimits};
use super::readiness::RegisteredToolReadiness;
use super::selection::RegisteredToolSelection;
use super::service::RegisteredToolBridgeHostService;
use super::snapshot::RegisteredToolSnapshot;
use crate::{BoxFuture, Deadline, HostServices, RuntimeFailure, RuntimeTurnId, ScopeId};
use std::sync::Arc;
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};

/// Immutable opt-in registered-tool request that opens no resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolPreparation {
    snapshot: Arc<RegisteredToolSnapshot>,
    selection: RegisteredToolSelection,
    admission: ConsumerAdmissionBinding,
    limits: RegisteredToolLimits,
}

impl RegisteredToolPreparation {
    /// Creates an immutable opt-in request without opening any resource.
    ///
    /// Absence of this request preserves every previous route behavior.
    #[must_use]
    pub fn new(
        snapshot: Arc<RegisteredToolSnapshot>,
        selection: RegisteredToolSelection,
        admission: ConsumerAdmissionBinding,
        limits: RegisteredToolLimits,
    ) -> Self {
        Self {
            snapshot,
            selection,
            admission,
            limits,
        }
    }

    /// Returns the immutable registration snapshot.
    #[must_use]
    pub fn snapshot(&self) -> &Arc<RegisteredToolSnapshot> {
        &self.snapshot
    }

    /// Returns the immutable selection.
    #[must_use]
    pub const fn selection(&self) -> &RegisteredToolSelection {
        &self.selection
    }

    /// Returns the trusted consumer admission binding.
    #[must_use]
    pub const fn admission(&self) -> &ConsumerAdmissionBinding {
        &self.admission
    }

    /// Returns the consumer-selected narrowing limits.
    #[must_use]
    pub const fn limits(&self) -> RegisteredToolLimits {
        self.limits
    }

    /// Validates identity, limits, port availability, and selected topology.
    ///
    /// Preparation opens nothing, starts no provider, and chooses no route.
    pub fn prepare(
        &self,
        hosts: &HostServices,
        configured_identity: ConfiguredInstanceId,
        operation_scope: ScopeId,
        turn: RuntimeTurnId,
        deadline: Deadline,
    ) -> Result<PreparedRegisteredToolBinding, RuntimeFailure> {
        if !Arc::ptr_eq(&self.snapshot, self.selection.snapshot())
            && self.snapshot.as_ref() != self.selection.snapshot().as_ref()
        {
            return Err(fail(RegisteredToolFailureKind::UnsupportedRegistration));
        }
        let readiness = RegisteredToolReadiness::evaluate(hosts, &self.selection);
        readiness.require_ready()?;
        let port = hosts
            .registered_tool_bridge()
            .cloned()
            .ok_or_else(|| fail(RegisteredToolFailureKind::MissingHostService))?;
        let effective_bounds = self
            .selection
            .effective_bounds()
            .narrowed(self.limits.bounds());
        Ok(PreparedRegisteredToolBinding {
            execution_host_id: hosts.execution_host_id().clone(),
            configured_identity,
            operation_scope,
            turn,
            deadline,
            selection: self.selection.clone(),
            admission: self.admission.clone(),
            effective_bounds,
            readiness,
            port,
        })
    }
}

/// Prepared, immutable registered-tool binding that still owns no resource.
#[derive(Clone)]
pub struct PreparedRegisteredToolBinding {
    execution_host_id: ExecutionHostId,
    configured_identity: ConfiguredInstanceId,
    operation_scope: ScopeId,
    turn: RuntimeTurnId,
    deadline: Deadline,
    selection: RegisteredToolSelection,
    admission: ConsumerAdmissionBinding,
    effective_bounds: RegisteredToolBounds,
    readiness: RegisteredToolReadiness,
    port: Arc<dyn RegisteredToolBridgeHostService>,
}

impl PreparedRegisteredToolBinding {
    /// Returns the execution host bound at prepare.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Returns the configured instance bound at prepare.
    #[must_use]
    pub const fn configured_identity(&self) -> &ConfiguredInstanceId {
        &self.configured_identity
    }

    /// Returns the operation scope bound at prepare.
    #[must_use]
    pub const fn operation_scope(&self) -> &ScopeId {
        &self.operation_scope
    }

    /// Returns the owning turn attempt bound at prepare.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Returns the operation deadline bound at prepare.
    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }

    /// Returns the immutable selection bound at prepare.
    #[must_use]
    pub const fn selection(&self) -> &RegisteredToolSelection {
        &self.selection
    }

    /// Returns the effective bounds after consumer narrowing.
    #[must_use]
    pub const fn effective_bounds(&self) -> RegisteredToolBounds {
        self.effective_bounds
    }

    /// Returns the typed readiness record proved at prepare.
    #[must_use]
    pub const fn readiness(&self) -> &RegisteredToolReadiness {
        &self.readiness
    }

    /// Opens one scoped lease through the bound registered-tool host port.
    pub fn open(&self) -> BoxFuture<'_, Result<RegisteredToolBridgeLease, RuntimeFailure>> {
        self.port.open(RegisteredToolOpenRequest::new(
            self.execution_host_id.clone(),
            self.configured_identity.clone(),
            self.operation_scope.clone(),
            self.turn.clone(),
            self.selection.clone(),
            self.admission.clone(),
            self.deadline,
        ))
    }
}

impl std::fmt::Debug for PreparedRegisteredToolBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedRegisteredToolBinding")
            .field("execution_host_id", &self.execution_host_id)
            .field("configured_identity", &self.configured_identity)
            .field("operation_scope", &self.operation_scope)
            .field("turn", &self.turn)
            .field("selection", &self.selection)
            .field("effective_bounds", &self.effective_bounds)
            .field("readiness", &self.readiness)
            .field("port", &"<registered tool bridge host port>")
            .finish()
    }
}
