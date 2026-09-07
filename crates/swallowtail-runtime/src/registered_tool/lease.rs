//! Scoped registered-tool lease, its lifecycle truth, and its call channel.

use super::admission::ConsumerAdmissionBinding;
use super::call::{RegisteredToolCallRequest, RegisteredToolOutcome};
use super::identity::{
    RegisteredToolLeaseGeneration, RegisteredToolTransport, RegisteredToolTransportGeneration,
};
use super::kernel::RegisteredToolOperationKernel;
use super::selection::RegisteredToolSelection;
use crate::{BoxFuture, Deadline, RuntimeFailure, RuntimeTurnId, ScopeId};
use std::fmt;
use std::sync::Arc;
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};

/// Public admission state of one registered-tool lease.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolAdmissionState {
    /// New calls, progress, and results may be admitted.
    Open,
    /// New admission is frozen; issued work settles or is abandoned.
    Frozen,
    /// The lease has released its transport and private material.
    Closed,
}

impl RegisteredToolAdmissionState {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Frozen => "frozen",
            Self::Closed => "closed",
        }
    }

    /// Reports whether new calls may still be admitted.
    #[must_use]
    pub const fn admits_new_work(self) -> bool {
        matches!(self, Self::Open)
    }
}

impl fmt::Display for RegisteredToolAdmissionState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Contract 063 lifecycle position of one registered-tool profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolLifecycleState {
    /// Registered with no resources bound.
    Registered,
    /// Prepared and immutable, with no resources bound.
    Prepared,
    /// Opening; partial resources must join on failure.
    Opening,
    /// Ready before any provider dispatch.
    Ready,
    /// Exactly one call is committed and outstanding.
    CallPending,
    /// Admission is frozen before settling or abandoning calls.
    Frozen,
    /// Joined teardown is running.
    Closing,
    /// Teardown reported its exact cleanup truth.
    Closed,
}

impl RegisteredToolLifecycleState {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Registered => "registered",
            Self::Prepared => "prepared",
            Self::Opening => "opening",
            Self::Ready => "ready",
            Self::CallPending => "call-pending",
            Self::Frozen => "frozen",
            Self::Closing => "closing",
            Self::Closed => "closed",
        }
    }
}

/// Exact cause that started joined registered-tool teardown.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolCleanupCause {
    /// The turn completed normally.
    Completion,
    /// The operation was cancelled.
    Cancellation,
    /// The operation or call reached its deadline.
    Deadline,
    /// The provider failed.
    ProviderFailure,
    /// The transport failed or was lost.
    TransportFailure,
    /// The consumer closed the lease explicitly.
    ExplicitClose,
}

impl RegisteredToolCleanupCause {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completion => "completion",
            Self::Cancellation => "cancellation",
            Self::Deadline => "deadline",
            Self::ProviderFailure => "provider-failure",
            Self::TransportFailure => "transport-failure",
            Self::ExplicitClose => "explicit-close",
        }
    }
}

/// Bounded observation of admission, outstanding calls, and cleanup truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolCompletionState {
    admission: RegisteredToolAdmissionState,
    lifecycle: RegisteredToolLifecycleState,
    outstanding_calls: usize,
    cleanup_failed: bool,
}

impl RegisteredToolCompletionState {
    /// Records one bounded completion observation.
    #[must_use]
    pub const fn new(
        admission: RegisteredToolAdmissionState,
        lifecycle: RegisteredToolLifecycleState,
        outstanding_calls: usize,
        cleanup_failed: bool,
    ) -> Self {
        Self {
            admission,
            lifecycle,
            outstanding_calls,
            cleanup_failed,
        }
    }

    /// Returns the observed admission state.
    #[must_use]
    pub const fn admission(&self) -> RegisteredToolAdmissionState {
        self.admission
    }

    /// Returns the observed lifecycle position.
    #[must_use]
    pub const fn lifecycle(&self) -> RegisteredToolLifecycleState {
        self.lifecycle
    }

    /// Returns the bounded outstanding-call count.
    #[must_use]
    pub const fn outstanding_calls(&self) -> usize {
        self.outstanding_calls
    }

    /// Reports whether a joined cleanup attempt already failed.
    #[must_use]
    pub const fn cleanup_failed(&self) -> bool {
        self.cleanup_failed
    }

    /// Reports whether successful completion may be admitted.
    #[must_use]
    pub const fn allows_successful_completion(&self) -> bool {
        matches!(self.admission, RegisteredToolAdmissionState::Frozen)
            && self.outstanding_calls == 0
            && !self.cleanup_failed
    }
}

/// Request that opens one registered-tool lease inside the consumer host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolOpenRequest {
    execution_host_id: ExecutionHostId,
    configured_instance: ConfiguredInstanceId,
    scope: ScopeId,
    turn: RuntimeTurnId,
    selection: RegisteredToolSelection,
    admission: ConsumerAdmissionBinding,
    deadline: Deadline,
}

impl RegisteredToolOpenRequest {
    /// Binds configured and prepared identity, selection, admission, deadline.
    #[must_use]
    pub const fn new(
        execution_host_id: ExecutionHostId,
        configured_instance: ConfiguredInstanceId,
        scope: ScopeId,
        turn: RuntimeTurnId,
        selection: RegisteredToolSelection,
        admission: ConsumerAdmissionBinding,
        deadline: Deadline,
    ) -> Self {
        Self {
            execution_host_id,
            configured_instance,
            scope,
            turn,
            selection,
            admission,
            deadline,
        }
    }

    /// Returns the execution host that must own every required service.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Returns the configured instance bound to this operation.
    #[must_use]
    pub const fn configured_instance(&self) -> &ConfiguredInstanceId {
        &self.configured_instance
    }

    /// Returns the operation scope that will own the lease.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the owning turn attempt.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Returns the immutable selection bound at prepare.
    #[must_use]
    pub const fn selection(&self) -> &RegisteredToolSelection {
        &self.selection
    }

    /// Returns the trusted consumer admission binding.
    #[must_use]
    pub const fn admission(&self) -> &ConsumerAdmissionBinding {
        &self.admission
    }

    /// Returns the operation deadline.
    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }
}

/// Non-clonable scoped token for one opened registered-tool lease.
///
/// A lease exists only because [`RegisteredToolOperationKernel`] created it. It
/// is not constructible, clonable, or serializable outside that kernel, carries
/// no endpoint or bearer material for the host-mediated callback carrier, and
/// exposes safe binding descriptions only. Drop is defensive cleanup and is
/// never success evidence.
pub struct RegisteredToolBridgeLease {
    execution_host_id: ExecutionHostId,
    configured_instance: ConfiguredInstanceId,
    scope: ScopeId,
    turn: RuntimeTurnId,
    selection: RegisteredToolSelection,
    generation: RegisteredToolLeaseGeneration,
    transport_generation: RegisteredToolTransportGeneration,
    deadline: Deadline,
    kernel: Arc<RegisteredToolOperationKernel>,
    release: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl RegisteredToolBridgeLease {
    pub(super) fn mint(
        request: &RegisteredToolOpenRequest,
        generation: RegisteredToolLeaseGeneration,
        transport_generation: RegisteredToolTransportGeneration,
        kernel: Arc<RegisteredToolOperationKernel>,
    ) -> Self {
        Self {
            execution_host_id: request.execution_host_id().clone(),
            configured_instance: request.configured_instance().clone(),
            scope: request.scope().clone(),
            turn: request.turn().clone(),
            selection: request.selection().clone(),
            generation,
            transport_generation,
            deadline: request.deadline(),
            kernel,
            release: None,
        }
    }

    /// Binds one host cleanup action to this lease.
    ///
    /// A second bind is ignored so callers cannot replace or disarm cleanup.
    #[must_use]
    pub fn with_release(mut self, release: impl FnOnce() + Send + 'static) -> Self {
        if self.release.is_none() {
            self.release = Some(Box::new(release));
        }
        self
    }

    /// Reports whether this lease is the one that exact kernel opened.
    ///
    /// This is an identity comparison, never a way to rebind a lease.
    #[must_use]
    pub fn is_bound_to(&self, kernel: &Arc<RegisteredToolOperationKernel>) -> bool {
        Arc::ptr_eq(&self.kernel, kernel)
    }

    /// Issues one bounded call through the kernel's serialized admission point.
    pub fn call(
        &self,
        request: RegisteredToolCallRequest,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        RegisteredToolOperationKernel::issue(&self.kernel, request)
    }

    /// Returns the execution host bound at open.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Returns the configured instance bound at open.
    #[must_use]
    pub const fn configured_instance(&self) -> &ConfiguredInstanceId {
        &self.configured_instance
    }

    /// Returns the operation scope bound at open.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the owning turn attempt bound at open.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Returns the immutable selection bound at open.
    #[must_use]
    pub const fn selection(&self) -> &RegisteredToolSelection {
        &self.selection
    }

    /// Returns the lease generation bound at open.
    #[must_use]
    pub const fn generation(&self) -> RegisteredToolLeaseGeneration {
        self.generation
    }

    /// Returns the transport generation bound at open.
    #[must_use]
    pub const fn transport_generation(&self) -> RegisteredToolTransportGeneration {
        self.transport_generation
    }

    /// Returns the selected carrier.
    #[must_use]
    pub const fn transport(&self) -> RegisteredToolTransport {
        self.selection.transport()
    }

    /// Returns the operation deadline bound at open.
    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }
}

impl Drop for RegisteredToolBridgeLease {
    fn drop(&mut self) {
        if let Some(release) = self.release.take() {
            release();
        }
    }
}

impl fmt::Debug for RegisteredToolBridgeLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredToolBridgeLease")
            .field("execution_host_id", &self.execution_host_id)
            .field("configured_instance", &self.configured_instance)
            .field("scope", &self.scope)
            .field("turn", &self.turn)
            .field("generation", &self.generation)
            .field("transport", &self.selection.transport())
            .field("transport_generation", &self.transport_generation)
            .field("kernel", &"<private operation kernel>")
            .finish()
    }
}
