//! Builds the validated preflight plan and bound open request for one sidecar session.

use super::super::selection::{
    pi_sdk_sidecar_node_binding, pi_sdk_sidecar_package_binding, pi_sdk_sidecar_sidecar_binding,
    pi_sdk_sidecar_wire_binding,
};
use super::super::{
    PI_SDK_SIDECAR_NODE_RUNTIME, PI_SDK_SIDECAR_SDK_VERSION, PI_SDK_SIDECAR_SOURCE_TAG,
    PI_SDK_SIDECAR_WIRE,
};
use super::{PiSdkSidecarPreparedSession, PiSdkSidecarSessionPreparation};
use std::num::NonZeroU32;
use swallowtail_core::{
    AccessProfile, AccessRequirement, AccessStatus, Capability, CapabilityConstraint,
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionLayer, ExtensionNamespace, HarnessConfigurationPosture,
    HarnessIsolation, HarnessRpcPolicy, HarnessSchedulingBounds, HostServiceKind,
    InstanceOwnership, InstancePolicyId, OperationRequirements, OperationShape, ProtocolFacadeId,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy,
    SessionProviderStatePolicy, SupportAuthority,
};
use swallowtail_runtime::{OpenSessionRequest, PreparationFailure, SessionOptions};

pub(super) fn prepare(
    input: PiSdkSidecarSessionPreparation,
    options: SessionOptions,
) -> Result<PiSdkSidecarPreparedSession, PreparationFailure> {
    super::super::reasoning::validate_options(&options)?;
    let mut capability_requirements = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::LoadSession,
            [
                CapabilityConstraint::ReplayMaximumItems(
                    super::super::replay::MAXIMUM_REPLAY_ITEMS as u32,
                ),
                CapabilityConstraint::ReplayMaximumBytes(
                    super::super::replay::MAXIMUM_REPLAY_BYTES as u64,
                ),
            ],
        ),
        CapabilityRequirement::new(Capability::Resume, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if input.image_attachments {
        capability_requirements.push(CapabilityRequirement::new(
            Capability::Attachments,
            [
                CapabilityConstraint::attachment_media_type("image/png")
                    .expect("static media type is valid"),
                CapabilityConstraint::AttachmentMaximumBytes(1024 * 1024),
                CapabilityConstraint::AttachmentMaximumCount(1),
            ],
        ));
    }
    if let Some(reasoning) = options.reasoning_mode() {
        super::super::reasoning::validate_preparation(&input.provider, &input.model, reasoning)?;
        capability_requirements.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(reasoning.clone())],
        ));
    }
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let versions = [
        pi_sdk_sidecar_package_binding(PI_SDK_SIDECAR_SDK_VERSION),
        pi_sdk_sidecar_node_binding(PI_SDK_SIDECAR_NODE_RUNTIME),
        pi_sdk_sidecar_wire_binding(PI_SDK_SIDECAR_WIRE),
        pi_sdk_sidecar_sidecar_binding(PI_SDK_SIDECAR_SOURCE_TAG),
    ];
    let versions = versions
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .expect("static sidecar version bindings are valid");
    let one = NonZeroU32::new(1).expect("one is non-zero");
    let rpc_policy = HarnessRpcPolicy::restrictive(HarnessSchedulingBounds::new(
        one,
        NonZeroU32::new(2).expect("two is non-zero"),
        one,
        one,
    ));
    let descriptor = super::super::pi_sdk_sidecar_descriptor();
    let instance = ConfiguredInstance::new(
        input.instance_id.clone(),
        input.instance_revision,
        descriptor.identity().id().clone(),
        input.execution_host_id.clone(),
        input.target,
        InstanceOwnership::HostOwnedEphemeral,
        input.access_profile_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("pi-sdk-sidecar-jsonl-v1").expect("static facade is valid"),
        InstancePolicyId::new("pi-sdk-sidecar-ambient-read").expect("static policy is valid"),
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
    )
    .with_provider_id(input.provider);
    let access = AccessProfile::new(
        input.access_profile_id.clone(),
        CredentialMechanism::ProviderSpecific(
            ExtensionNamespace::new("pi/delegated-harness-auth").expect("static namespace"),
        ),
        EntitlementMetering::Unknown,
        EndpointAudience::new("pi-harness").expect("static audience"),
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
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .with_session_provider_state_policy(SessionProviderStatePolicy::DurableProviderSessionPreserved)
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
    let request =
        OpenSessionRequest::from_plan(&plan, input.request_id, input.working_resource, None)?
            .with_options(options);
    Ok(PiSdkSidecarPreparedSession {
        plan,
        request,
        environment: input.environment,
        credential: input.credential,
    })
}
