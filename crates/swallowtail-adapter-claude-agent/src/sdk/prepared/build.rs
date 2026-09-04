//! Builds the validated preflight plan and bound open request for one fresh
//! Claude Agent SDK sidecar session.

use super::super::driver::{ACCESS_NAMESPACE, ENDPOINT_AUDIENCE};
use super::super::selection::{
    claude_agent_sdk_native_binding, claude_agent_sdk_node_binding,
    claude_agent_sdk_package_binding, claude_agent_sdk_sidecar_binding,
    claude_agent_sdk_wire_binding,
};
use super::super::{
    CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_NODE_RUNTIME,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_WIRE,
};
use super::{ClaudeAgentSdkPreparedSession, ClaudeAgentSdkSessionPreparation, preparation_failure};
use std::num::NonZeroU32;
use swallowtail_core::{
    AccessProfile, AccessRequirement, AccessStatus, CancellationScope, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialMechanism, CredentialState, DriverRole, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionLayer, ExtensionNamespace,
    HarnessConfigurationPosture, HarnessIsolation, HarnessRpcPolicy, HarnessSchedulingBounds,
    HostServiceKind, InstanceOwnership, InstancePolicyId, OperationRequirements, OperationShape,
    ProtocolFacadeId, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority,
};
use swallowtail_runtime::{OpenSessionRequest, PreparationFailure, SessionOptions};

pub(super) fn prepare(
    input: ClaudeAgentSdkSessionPreparation,
    options: SessionOptions,
) -> Result<ClaudeAgentSdkPreparedSession, PreparationFailure> {
    if !options.is_empty() {
        return Err(preparation_failure(
            swallowtail_runtime::PreparationStage::Preflight,
            "swallowtail.claude-agent.sdk.preparation.unsupported_options",
            "Claude Agent SDK preparation admits no session options in this layer",
        ));
    }
    // Blocked at the shared guard, not withheld by taste. Contract 013 keys
    // the consumer-tool exclusion on a bounded profile's claimed filesystem
    // boundary, and this route claims none, so its ambient read-write profile
    // with consumer-mediated tool calls is admissible. Preflight still refuses
    // any interactive session pairing `ResourceAccess::ReadWrite` with
    // `Capability::ToolCalls`. Admitting a write tool here would mean dropping
    // the capability this route requires, so it fails with an exact code until
    // that guard is narrowed.
    if input.profile.admits_writes() {
        return Err(preparation_failure(
            swallowtail_runtime::PreparationStage::Preflight,
            "swallowtail.claude-agent.sdk.preparation.write_admission_unavailable",
            "Claude Agent SDK preparation cannot yet bind an ambient read-write interactive \
             session that also declares consumer tool exchange",
        ));
    }
    let resource_access = input.profile.resource_access();
    let capability_requirements = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ToolCalls, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(resource_access),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let versions = [
        claude_agent_sdk_package_binding(CLAUDE_AGENT_SDK_VERSION),
        claude_agent_sdk_native_binding(CLAUDE_AGENT_SDK_NATIVE_VERSION),
        claude_agent_sdk_node_binding(CLAUDE_AGENT_SDK_NODE_RUNTIME),
        claude_agent_sdk_wire_binding(CLAUDE_AGENT_SDK_WIRE),
        claude_agent_sdk_sidecar_binding(CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG),
    ];
    let versions = versions
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .expect("static SDK sidecar version bindings are valid");
    let one = NonZeroU32::new(1).expect("one is non-zero");
    let rpc_policy =
        HarnessRpcPolicy::restrictive(HarnessSchedulingBounds::new(one, one, one, one));
    let descriptor = super::super::claude_agent_sdk_descriptor();
    let instance = ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision,
        descriptor.identity().id().clone(),
        input.execution_host_id.clone(),
        input.target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("claude-agent-sdk-jsonl-v1").expect("static facade is valid"),
        InstancePolicyId::new(instance_policy_id(resource_access)).expect("static policy is valid"),
        capabilities.clone(),
    )
    .with_interface_versions(versions.clone())
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_harness_rpc_policy(rpc_policy.clone());
    let route = swallowtail_core::ModelRoute::new(
        input.route_id,
        input.route_revision,
        input.instance_id,
        input.model,
        capabilities,
    );
    let access = AccessProfile::new(
        input.access_profile_id.clone(),
        CredentialMechanism::ProviderSpecific(
            ExtensionNamespace::new(ACCESS_NAMESPACE).expect("static namespace"),
        ),
        EntitlementMetering::Unknown,
        EndpointAudience::new(ENDPOINT_AUDIENCE).expect("static audience"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(input.credential.clone());
    let status = AccessStatus::new(
        input.access_profile_id,
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let services = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::Time,
    ];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        input.execution_host_id.clone(),
        AccessRequirement::new(access.id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capability_requirements)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_interface_versions(versions)
    .with_harness_rpc_policy(rpc_policy)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(resource_access))
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .require_model_route();
    let plan = swallowtail_runtime::build_plan(
        &descriptor,
        &instance,
        Some(&route),
        &requirements,
        &access,
        &status,
        services,
    )?;
    let request = OpenSessionRequest::from_plan(
        &plan,
        input.request_id,
        input.working_resource,
        Some(input.deadline),
    )?
    .with_options(options);
    Ok(super::build_prepared(
        plan,
        request,
        input.environment,
        input.credential,
        input.profile,
    ))
}

/// The instance policy the admitted tool set implies. A read-only profile
/// keeps the exact `v0.4.0` identifier, so the default plan is unchanged.
const fn instance_policy_id(resource_access: ResourceAccess) -> &'static str {
    match resource_access {
        ResourceAccess::ReadWrite => "claude-agent-sdk-ambient-read-write",
        _ => "claude-agent-sdk-ambient-read",
    }
}
