//! Scoped registered-tool lease, its lifecycle truth, and its call channel.

use super::admission::ConsumerAdmissionBinding;
use super::call::{
    RegisteredToolCallRequest, RegisteredToolOutcome, ValidatedRegisteredToolBinding,
};
use super::failure::{RegisteredToolFailureKind, fail};
use super::identity::{
    RegisteredToolLeaseGeneration, RegisteredToolTransport, RegisteredToolTransportGeneration,
};
use super::secrets::{RegisteredToolBearer, RegisteredToolBridgeToken, RegisteredToolEndpoint};
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

/// Kernel-owned channel that issues one call through the serialized point.
pub trait RegisteredToolCallChannel: Send + Sync {
    /// Issues one bounded call and settles exactly one correlated outcome.
    fn issue(
        &self,
        request: RegisteredToolCallRequest,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>>;
}

/// Non-clonable scoped token for one opened registered-tool lease.
///
/// Endpoint and bearer stay driver-only. The lease is not serializable. Drop is
/// defensive cleanup and is never success evidence.
pub struct RegisteredToolBridgeLease {
    execution_host_id: ExecutionHostId,
    configured_instance: ConfiguredInstanceId,
    scope: ScopeId,
    turn: RuntimeTurnId,
    selection: RegisteredToolSelection,
    admission: ConsumerAdmissionBinding,
    generation: RegisteredToolLeaseGeneration,
    transport_generation: RegisteredToolTransportGeneration,
    deadline: Deadline,
    endpoint: Option<RegisteredToolEndpoint>,
    bearer: Option<RegisteredToolBearer>,
    token: Option<RegisteredToolBridgeToken>,
    channel: Option<Arc<dyn RegisteredToolCallChannel>>,
    release: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl RegisteredToolBridgeLease {
    /// Creates a non-live handle for redaction and fixture use.
    ///
    /// This handle cannot issue a call, freeze, or close a host lease.
    #[must_use]
    pub fn new(
        request: RegisteredToolOpenRequest,
        generation: RegisteredToolLeaseGeneration,
        transport_generation: RegisteredToolTransportGeneration,
    ) -> Self {
        let RegisteredToolOpenRequest {
            execution_host_id,
            configured_instance,
            scope,
            turn,
            selection,
            admission,
            deadline,
        } = request;
        Self {
            execution_host_id,
            configured_instance,
            scope,
            turn,
            selection,
            admission,
            generation,
            transport_generation,
            deadline,
            endpoint: None,
            bearer: None,
            token: None,
            channel: None,
            release: None,
        }
    }

    /// Attaches driver-private endpoint and bearer material for a carrier.
    ///
    /// A second attachment is ignored so drivers cannot replace private
    /// material on a live lease.
    #[must_use]
    pub fn with_private_material(
        mut self,
        endpoint: RegisteredToolEndpoint,
        bearer: RegisteredToolBearer,
    ) -> Self {
        if self.endpoint.is_none() && self.bearer.is_none() {
            self.endpoint = Some(endpoint);
            self.bearer = Some(bearer);
        }
        self
    }

    /// Binds host cleanup, the kernel call channel, and unforgeable identity.
    ///
    /// A second bind is ignored so callers cannot replace or disarm cleanup.
    #[must_use]
    pub fn bind(
        mut self,
        token: RegisteredToolBridgeToken,
        channel: Arc<dyn RegisteredToolCallChannel>,
        release: impl FnOnce() + Send + 'static,
    ) -> Self {
        if self.token.is_none() && self.release.is_none() {
            self.token = Some(token);
            self.channel = Some(channel);
            self.release = Some(Box::new(release));
        }
        self
    }

    /// Reports whether this handle authenticates one live host token.
    #[must_use]
    pub fn binding_matches(&self, token: &RegisteredToolBridgeToken) -> bool {
        self.token
            .as_ref()
            .is_some_and(|bound| bound.token_matches(token))
    }

    /// Reports whether this handle is bound to a live host lease.
    #[must_use]
    pub const fn is_live(&self) -> bool {
        self.token.is_some()
    }

    /// Returns the live validated binding, only for a host-bound lease.
    #[must_use]
    pub fn validated_binding(&self) -> Option<ValidatedRegisteredToolBinding> {
        if !self.is_live() {
            return None;
        }
        Some(ValidatedRegisteredToolBinding::mint(
            self.execution_host_id.clone(),
            self.configured_instance.clone(),
            self.scope.clone(),
            self.turn.clone(),
            self.selection.snapshot().server_id().clone(),
            self.selection.snapshot().revision().clone(),
            self.generation,
            self.selection.transport(),
            self.transport_generation,
            self.selection.protocol_version().clone(),
            self.selection.effective_bounds(),
            self.admission.clone(),
        ))
    }

    /// Issues one bounded call through the kernel's serialized admission point.
    pub fn call(
        &self,
        request: RegisteredToolCallRequest,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        match self.channel.as_ref() {
            Some(channel) => channel.issue(request),
            None => Box::pin(async { Err(fail(RegisteredToolFailureKind::NotReady)) }),
        }
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

    /// Returns the trusted consumer admission binding.
    #[must_use]
    pub const fn admission(&self) -> &ConsumerAdmissionBinding {
        &self.admission
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

    /// Returns the driver-only endpoint when a carrier bound one.
    #[must_use]
    pub fn endpoint(&self) -> Option<&RegisteredToolEndpoint> {
        self.endpoint.as_ref()
    }

    /// Returns the driver-only bearer when a carrier bound one.
    #[must_use]
    pub fn bearer(&self) -> Option<&RegisteredToolBearer> {
        self.bearer.as_ref()
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
            .field("endpoint", &self.endpoint)
            .field("bearer", &self.bearer)
            .field("token", &self.token.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}
