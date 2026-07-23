use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    AccessProfileId, AccessRequirement, CancellationScope, Capability, CapabilityConstraint,
    CapabilityRequirement, CredentialMechanism, CredentialState, DirectAttemptTransport,
    DirectContinuationConfig, DirectContinuationRequirements, DirectToolSelection, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionHostId, ExecutionLayer, HostServiceKind,
    InstanceOwnership, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, ModelId,
    OperationRequirements, OperationShape, ProviderInferenceCachePolicy, RuntimeReadiness,
    SessionAccessPolicy, SupportAuthority,
};
use swallowtail_runtime::{
    OpenDirectContinuationSessionRequest, RuntimeFailure, validate_direct_continuation_plan,
};

pub const DEEPSEEK_FACADE_REVISION: &str = "deepseek-openai-chat-2026-07-22";
pub const DEEPSEEK_ENDPOINT: &str = "https://api.deepseek.com";
pub const DEEPSEEK_MODEL_ID: &str = "deepseek-v4-pro";
pub(crate) const DEEPSEEK_PROVIDER_ID: &str = "deepseek";
pub(crate) const DEEPSEEK_AUDIENCE: &str = "api.deepseek.com";
pub(crate) const DEEPSEEK_FACADE_AXIS: &str = "deepseek.openai-chat-facade";

#[must_use]
pub fn deepseek_v4_config() -> DirectContinuationConfig {
    DirectContinuationConfig::new(
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(3).unwrap(),
        NonZeroU32::new(8).unwrap(),
        NonZeroU32::new(1).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(262_144).unwrap(),
        NonZeroU64::new(1_048_576).unwrap(),
        NonZeroU32::new(4_096).unwrap(),
        NonZeroU64::new(8_192).unwrap(),
        DirectAttemptTransport::Buffered,
        DirectAttemptTransport::ServerSentEvents,
        DirectToolSelection::ProviderAutomatic,
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
    )
}

#[must_use]
pub fn deepseek_facade_binding() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(DEEPSEEK_FACADE_AXIS).unwrap(),
        InterfaceVersion::new(DEEPSEEK_FACADE_REVISION).unwrap(),
    )
}

#[must_use]
pub fn deepseek_facade_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("deepseek.openai-chat-window-1").unwrap(),
        InterfaceVersionAxis::new(DEEPSEEK_FACADE_AXIS).unwrap(),
        InterfaceVersionScheme::Opaque,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(DEEPSEEK_FACADE_REVISION).unwrap(),
            InterfaceBehaviorRevision::new("deepseek.v4-thinking-tools-v1").unwrap(),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static DeepSeek facade claim is valid")
}

#[must_use]
pub fn deepseek_v4_requirements(
    execution_host_id: ExecutionHostId,
    access_profile_id: AccessProfileId,
) -> OperationRequirements {
    let config = deepseek_v4_config();
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ToolCalls, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::OutputTokenLimit, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
    ];
    capabilities.extend(config.capability_requirements());
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        execution_host_id,
        AccessRequirement::new(access_profile_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services([
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ])
    .with_capabilities(capabilities)
    .with_session_access_policy(SessionAccessPolicy::resource_free())
    .with_direct_continuation(DirectContinuationRequirements::new(
        ModelId::new(DEEPSEEK_MODEL_ID).unwrap(),
        config,
    ))
    .with_interface_versions([deepseek_facade_binding()])
    .require_model_route()
}

pub fn validate_deepseek_request_plan(
    plan: &swallowtail_core::PreflightPlan,
    request: &OpenDirectContinuationSessionRequest,
) -> Result<(), RuntimeFailure> {
    validate_direct_continuation_plan(plan, request)?;
    let exact_version = deepseek_facade_binding();
    let reasoning_is_high = request
        .options()
        .reasoning_mode()
        .is_some_and(|mode| mode.as_str() == "high");
    if plan.instance_target_ref().as_host_value() != DEEPSEEK_ENDPOINT
        || plan.protocol_facade_id().as_str() != DEEPSEEK_FACADE_REVISION
        || plan
            .provider_id()
            .is_none_or(|id| id.as_str() != DEEPSEEK_PROVIDER_ID)
        || plan
            .model_id()
            .is_none_or(|id| id.as_str() != DEEPSEEK_MODEL_ID)
        || plan.endpoint_audience().as_str() != DEEPSEEK_AUDIENCE
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || !plan
            .interface_versions()
            .any(|binding| binding == &exact_version)
        || plan.classify_interface_version(&exact_version).is_none()
        || !reasoning_is_high
        || request.options().developer_instructions().is_some()
        || request.options().tools().len() > 8
        || request.config().tool_selection() != DirectToolSelection::ProviderAutomatic
        || request.config().provider_cache_policy()
            != ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority
    {
        return Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.deepseek.request_plan_mismatch",
            "DeepSeek request does not match the exact preflight selection",
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
