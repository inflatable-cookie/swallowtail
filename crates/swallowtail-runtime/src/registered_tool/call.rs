//! Validated bindings, issued calls, ordered progress, and settled outcomes.

use super::admission::ConsumerAdmissionBinding;
use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::{
    RegisteredServerId, RegisteredServerRevision, RegisteredToolCallId,
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolLeaseGeneration,
    RegisteredToolProtocolVersion, RegisteredToolTransport, RegisteredToolTransportGeneration,
};
use super::limits::RegisteredToolBounds;
use super::payload::RegisteredToolPayload;
use super::schema::RegisteredToolSchemaDigest;
use crate::{Deadline, RuntimeTurnId, ScopeId};
use std::num::NonZeroU64;
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};

/// Live binding minted by the kernel for exactly one open lease.
///
/// There is no public constructor and no public minting path. Only
/// [`super::kernel::RegisteredToolOperationKernel`] creates one, and only after
/// a matching topology proof. Provider input, model arguments, tool arguments,
/// provider session ids, and PIDs can never create one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedRegisteredToolBinding {
    execution_host_id: ExecutionHostId,
    configured_instance: ConfiguredInstanceId,
    scope: ScopeId,
    turn: RuntimeTurnId,
    server_id: RegisteredServerId,
    server_revision: RegisteredServerRevision,
    lease_generation: RegisteredToolLeaseGeneration,
    transport: RegisteredToolTransport,
    transport_generation: RegisteredToolTransportGeneration,
    protocol_version: RegisteredToolProtocolVersion,
    effective_bounds: RegisteredToolBounds,
    admission: ConsumerAdmissionBinding,
}

impl ValidatedRegisteredToolBinding {
    #[allow(clippy::too_many_arguments)]
    pub(super) const fn mint(
        execution_host_id: ExecutionHostId,
        configured_instance: ConfiguredInstanceId,
        scope: ScopeId,
        turn: RuntimeTurnId,
        server_id: RegisteredServerId,
        server_revision: RegisteredServerRevision,
        lease_generation: RegisteredToolLeaseGeneration,
        transport: RegisteredToolTransport,
        transport_generation: RegisteredToolTransportGeneration,
        protocol_version: RegisteredToolProtocolVersion,
        effective_bounds: RegisteredToolBounds,
        admission: ConsumerAdmissionBinding,
    ) -> Self {
        Self {
            execution_host_id,
            configured_instance,
            scope,
            turn,
            server_id,
            server_revision,
            lease_generation,
            transport,
            transport_generation,
            protocol_version,
            effective_bounds,
            admission,
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

    /// Returns the owning turn bound at open.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Returns the registered server identity.
    #[must_use]
    pub const fn server_id(&self) -> &RegisteredServerId {
        &self.server_id
    }

    /// Returns the exact registration revision.
    #[must_use]
    pub const fn server_revision(&self) -> &RegisteredServerRevision {
        &self.server_revision
    }

    /// Returns the lease generation.
    #[must_use]
    pub const fn lease_generation(&self) -> RegisteredToolLeaseGeneration {
        self.lease_generation
    }

    /// Returns the selected carrier.
    #[must_use]
    pub const fn transport(&self) -> RegisteredToolTransport {
        self.transport
    }

    /// Returns the transport generation.
    #[must_use]
    pub const fn transport_generation(&self) -> RegisteredToolTransportGeneration {
        self.transport_generation
    }

    /// Returns the negotiated protocol version.
    #[must_use]
    pub const fn protocol_version(&self) -> &RegisteredToolProtocolVersion {
        &self.protocol_version
    }

    /// Returns the effective bounds fixed by the prepared plan.
    #[must_use]
    pub const fn effective_bounds(&self) -> RegisteredToolBounds {
        self.effective_bounds
    }

    /// Returns the trusted consumer admission binding.
    #[must_use]
    pub const fn admission(&self) -> &ConsumerAdmissionBinding {
        &self.admission
    }

    /// Reports whether another binding is the same live binding.
    #[must_use]
    pub fn same_binding(&self, other: &Self) -> bool {
        self == other
    }
}

/// Consumer request that issues one registered-tool call through a lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolCallRequest {
    call_id: RegisteredToolCallId,
    tool: RegisteredToolId,
    arguments: RegisteredToolPayload,
    deadline: Deadline,
}

impl RegisteredToolCallRequest {
    /// Creates one bounded call request for an exact selected tool.
    #[must_use]
    pub const fn new(
        call_id: RegisteredToolCallId,
        tool: RegisteredToolId,
        arguments: RegisteredToolPayload,
        deadline: Deadline,
    ) -> Self {
        Self {
            call_id,
            tool,
            arguments,
            deadline,
        }
    }

    /// Returns the unique call identity.
    #[must_use]
    pub const fn call_id(&self) -> &RegisteredToolCallId {
        &self.call_id
    }

    /// Returns the selected namespaced tool identity.
    #[must_use]
    pub const fn tool(&self) -> &RegisteredToolId {
        &self.tool
    }

    /// Returns the bounded arguments.
    #[must_use]
    pub const fn arguments(&self) -> &RegisteredToolPayload {
        &self.arguments
    }

    /// Returns the caller-selected deadline.
    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }
}

/// One call the kernel has validated and committed to dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolCall {
    binding: ValidatedRegisteredToolBinding,
    call_id: RegisteredToolCallId,
    tool: RegisteredToolId,
    kind: RegisteredToolExecutionKind,
    arguments: RegisteredToolPayload,
    deadline: Deadline,
}

impl RegisteredToolCall {
    pub(super) fn mint(
        binding: ValidatedRegisteredToolBinding,
        call_id: RegisteredToolCallId,
        tool: RegisteredToolId,
        kind: RegisteredToolExecutionKind,
        arguments: RegisteredToolPayload,
        deadline: Deadline,
    ) -> Result<Self, RegisteredToolFailure> {
        if arguments.byte_len() > binding.effective_bounds().max_argument_bytes() {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self {
            binding,
            call_id,
            tool,
            kind,
            arguments,
            deadline,
        })
    }

    /// Returns the live validated binding.
    #[must_use]
    pub const fn binding(&self) -> &ValidatedRegisteredToolBinding {
        &self.binding
    }

    /// Returns the unique call identity.
    #[must_use]
    pub const fn call_id(&self) -> &RegisteredToolCallId {
        &self.call_id
    }

    /// Returns the selected namespaced tool identity.
    #[must_use]
    pub const fn tool(&self) -> &RegisteredToolId {
        &self.tool
    }

    /// Returns the execution kind bound to that identity.
    #[must_use]
    pub const fn kind(&self) -> RegisteredToolExecutionKind {
        self.kind
    }

    /// Returns the bounded arguments.
    #[must_use]
    pub const fn arguments(&self) -> &RegisteredToolPayload {
        &self.arguments
    }

    /// Returns the effective call deadline.
    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }
}

/// One bounded, ordered, non-terminal progress notification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolProgress {
    binding: ValidatedRegisteredToolBinding,
    call_id: RegisteredToolCallId,
    tool: RegisteredToolId,
    kind: RegisteredToolExecutionKind,
    sequence: NonZeroU64,
    payload: RegisteredToolPayload,
}

impl RegisteredToolProgress {
    pub(super) fn mint(
        call: &RegisteredToolCall,
        sequence: NonZeroU64,
        payload: RegisteredToolPayload,
    ) -> Result<Self, RegisteredToolFailure> {
        if payload.byte_len() > call.binding().effective_bounds().max_progress_item_bytes() {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self {
            binding: call.binding().clone(),
            call_id: call.call_id().clone(),
            tool: call.tool().clone(),
            kind: call.kind(),
            sequence,
            payload,
        })
    }

    /// Returns the live validated binding this notification is correlated to.
    #[must_use]
    pub const fn binding(&self) -> &ValidatedRegisteredToolBinding {
        &self.binding
    }

    /// Returns the correlated call identity.
    #[must_use]
    pub const fn call_id(&self) -> &RegisteredToolCallId {
        &self.call_id
    }

    /// Returns the correlated namespaced tool identity.
    #[must_use]
    pub const fn tool(&self) -> &RegisteredToolId {
        &self.tool
    }

    /// Returns the correlated execution kind.
    #[must_use]
    pub const fn kind(&self) -> RegisteredToolExecutionKind {
        self.kind
    }

    /// Returns the per-call monotonic sequence number.
    #[must_use]
    pub const fn sequence(&self) -> NonZeroU64 {
        self.sequence
    }

    /// Returns the bounded progress payload.
    #[must_use]
    pub const fn payload(&self) -> &RegisteredToolPayload {
        &self.payload
    }
}

/// Bounded successful result of one registered-tool call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolResult {
    payload: RegisteredToolPayload,
    schema_digest: RegisteredToolSchemaDigest,
}

impl RegisteredToolResult {
    /// Binds one bounded payload to the declared output schema digest.
    #[must_use]
    pub const fn new(
        payload: RegisteredToolPayload,
        schema_digest: RegisteredToolSchemaDigest,
    ) -> Self {
        Self {
            payload,
            schema_digest,
        }
    }

    /// Returns the bounded result payload.
    #[must_use]
    pub const fn payload(&self) -> &RegisteredToolPayload {
        &self.payload
    }

    /// Returns the declared output schema digest.
    #[must_use]
    pub const fn schema_digest(&self) -> &RegisteredToolSchemaDigest {
        &self.schema_digest
    }
}

/// Whether a failed call's remote execution disposition is known.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolExecutionDisposition {
    /// The call demonstrably did not start remote execution.
    NotExecuted,
    /// The call executed and its outcome is known.
    Executed,
    /// The call may have executed; it must never replay automatically.
    Unknown,
}

impl RegisteredToolExecutionDisposition {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotExecuted => "not-executed",
            Self::Executed => "executed",
            Self::Unknown => "unknown",
        }
    }
}

/// Exactly one correlated bounded result or typed failure for one call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolOutcome {
    call_id: RegisteredToolCallId,
    tool: RegisteredToolId,
    kind: RegisteredToolExecutionKind,
    disposition: RegisteredToolExecutionDisposition,
    settlement: Result<RegisteredToolResult, RegisteredToolFailure>,
}

impl RegisteredToolOutcome {
    /// Records one bounded successful result for an exact call.
    #[must_use]
    pub fn completed(call: &RegisteredToolCall, result: RegisteredToolResult) -> Self {
        Self {
            call_id: call.call_id().clone(),
            tool: call.tool().clone(),
            kind: call.kind(),
            disposition: RegisteredToolExecutionDisposition::Executed,
            settlement: Ok(result),
        }
    }

    /// Records one typed failure and its honest execution disposition.
    #[must_use]
    pub fn failed(
        call: &RegisteredToolCall,
        failure: RegisteredToolFailure,
        disposition: RegisteredToolExecutionDisposition,
    ) -> Self {
        Self {
            call_id: call.call_id().clone(),
            tool: call.tool().clone(),
            kind: call.kind(),
            disposition,
            settlement: Err(failure),
        }
    }

    /// Returns the correlated call identity.
    #[must_use]
    pub const fn call_id(&self) -> &RegisteredToolCallId {
        &self.call_id
    }

    /// Returns the correlated namespaced tool identity.
    #[must_use]
    pub const fn tool(&self) -> &RegisteredToolId {
        &self.tool
    }

    /// Returns the correlated execution kind.
    #[must_use]
    pub const fn kind(&self) -> RegisteredToolExecutionKind {
        self.kind
    }

    /// Returns the honest execution disposition.
    #[must_use]
    pub const fn disposition(&self) -> RegisteredToolExecutionDisposition {
        self.disposition
    }

    /// Returns the bounded result, when the call settled successfully.
    #[must_use]
    pub fn result(&self) -> Option<&RegisteredToolResult> {
        self.settlement.as_ref().ok()
    }

    /// Returns the typed failure, when the call did not settle successfully.
    #[must_use]
    pub fn failure(&self) -> Option<&RegisteredToolFailure> {
        self.settlement.as_ref().err()
    }

    /// Reports whether an explicit consumer retry may dispatch provider work.
    ///
    /// A retry always needs a fresh consumer attempt and a fresh operation
    /// binding; an unknown disposition never replays.
    #[must_use]
    pub const fn permits_explicit_retry(&self) -> bool {
        matches!(
            self.disposition,
            RegisteredToolExecutionDisposition::NotExecuted
        )
    }
}
