use super::runtime_failure;
use crate::{DirectInferenceAttemptId, RuntimeFailure, RuntimeSessionId};
use std::fmt;
use std::num::NonZeroU64;
use swallowtail_core::{
    AccessProfileId, ConfiguredInstanceId, ExecutionHostId, InstanceRevision, ModelId,
    ModelRouteId, PreflightPlan, ProtocolFacadeId,
};

#[derive(Clone, Eq, PartialEq)]
/// Immutable route and session identity bound to private continuation state.
pub struct DirectContinuationBinding {
    instance_id: ConfiguredInstanceId,
    instance_revision: InstanceRevision,
    facade_id: ProtocolFacadeId,
    access_profile_id: AccessProfileId,
    model_route_id: ModelRouteId,
    model_id: ModelId,
    execution_host_id: ExecutionHostId,
    session_id: RuntimeSessionId,
}

impl DirectContinuationBinding {
    /// Builds a binding from an exact preflight plan and runtime session.
    ///
    /// Fails when the plan does not select an exact model route.
    pub fn from_plan(
        plan: &PreflightPlan,
        session_id: RuntimeSessionId,
    ) -> Result<Self, RuntimeFailure> {
        Ok(Self {
            instance_id: plan.instance_id().clone(),
            instance_revision: plan.instance_revision().clone(),
            facade_id: plan.protocol_facade_id().clone(),
            access_profile_id: plan.access_profile_id().clone(),
            model_route_id: plan.model_route_id().cloned().ok_or_else(missing_route)?,
            model_id: plan.model_id().cloned().ok_or_else(missing_route)?,
            execution_host_id: plan.execution_host_id().clone(),
            session_id,
        })
    }

    #[must_use]
    /// Returns whether the plan and session exactly match this binding.
    pub fn matches_plan(&self, plan: &PreflightPlan, session_id: &RuntimeSessionId) -> bool {
        self.instance_id == *plan.instance_id()
            && self.instance_revision == *plan.instance_revision()
            && self.facade_id == *plan.protocol_facade_id()
            && self.access_profile_id == *plan.access_profile_id()
            && plan.model_route_id() == Some(&self.model_route_id)
            && plan.model_id() == Some(&self.model_id)
            && self.execution_host_id == *plan.execution_host_id()
            && self.session_id == *session_id
    }
}

impl fmt::Debug for DirectContinuationBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DirectContinuationBinding(<redacted>)")
    }
}

#[derive(Clone, Eq, PartialEq)]
/// Redacted metadata describing bounded provider-private continuation state.
///
/// This record exposes only binding, source-attempt, and size evidence. It does
/// not make the provider continuation bytes portable or serializable.
pub struct ProviderPrivateContinuationRecord {
    binding: DirectContinuationBinding,
    source_attempt_id: DirectInferenceAttemptId,
    byte_len: NonZeroU64,
}

impl ProviderPrivateContinuationRecord {
    /// Creates a record when the private continuation fits its configured bound.
    pub fn new(
        binding: DirectContinuationBinding,
        source_attempt_id: DirectInferenceAttemptId,
        byte_len: NonZeroU64,
        maximum_bytes: NonZeroU64,
    ) -> Result<Self, RuntimeFailure> {
        if byte_len > maximum_bytes {
            return Err(runtime_failure(
                "swallowtail.direct_continuation.private_overflow",
                "Provider-private continuation exceeds its bound",
            ));
        }
        Ok(Self {
            binding,
            source_attempt_id,
            byte_len,
        })
    }

    #[must_use]
    /// Returns the retained private continuation size.
    pub const fn byte_len(&self) -> NonZeroU64 {
        self.byte_len
    }

    #[must_use]
    /// Returns the inference attempt that produced the continuation.
    pub const fn source_attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.source_attempt_id
    }

    #[must_use]
    /// Returns whether the private continuation belongs to the plan and session.
    pub fn matches_plan(&self, plan: &PreflightPlan, session_id: &RuntimeSessionId) -> bool {
        self.binding.matches_plan(plan, session_id)
    }
}

impl fmt::Debug for ProviderPrivateContinuationRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderPrivateContinuationRecord")
            .field("binding", &"<redacted>")
            .field("source_attempt_id", &"<redacted>")
            .field("bytes", &format_args!("<private:{} bytes>", self.byte_len))
            .finish()
    }
}

fn missing_route() -> RuntimeFailure {
    runtime_failure(
        "swallowtail.direct_continuation.route_missing",
        "Direct continuation requires an exact model route",
    )
}
