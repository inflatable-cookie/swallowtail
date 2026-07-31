#![allow(dead_code)]

use swallowtail_adapter_antigravity::{
    antigravity_catalogue_descriptor, antigravity_headless_descriptor,
    antigravity_personal_google_access_profile, antigravity_release_binding,
};
use swallowtail_core::{
    AccessProfileId, AccessRequirement, AccessStatus, CancellationScope, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialState, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionHostId, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, ProtocolFacadeId, ProviderId, ReasoningMode, ResourceAccess,
    ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
    StructuredOutputEnforcement, SupportAuthority, preflight,
};

pub fn catalogue_plan(host: ExecutionHostId, target: &str, release: &str) -> PreflightPlan {
    let descriptor = antigravity_catalogue_descriptor();
    let access_id = AccessProfileId::new("access.antigravity.personal").expect("valid access id");
    let access = antigravity_personal_google_access_profile(access_id.clone());
    let capabilities = [CapabilityRequirement::new(Capability::ModelCatalog, [])];
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("antigravity.fixture").expect("valid instance id"),
        InstanceRevision::new("1").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new(target).expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("antigravity-models-v1").expect("valid facade"),
        InstancePolicyId::new("antigravity-ambient-catalogue").expect("valid policy"),
        CapabilityProfile::new(capabilities.clone()),
    )
    .with_interface_versions([
        antigravity_release_binding(release).expect("fixture Antigravity release is valid")
    ])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let services = [HostServiceKind::Process, HostServiceKind::Time];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::ModelCatalog,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capabilities)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_interface_versions([
        antigravity_release_binding(release).expect("fixture Antigravity release is valid")
    ]);
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, services),
        &requirements,
    )
    .expect("Antigravity catalogue fixture preflight succeeds")
}

pub fn continuation_plan(host: ExecutionHostId, target: &str) -> PreflightPlan {
    let descriptor = antigravity_headless_descriptor();
    let access_id = AccessProfileId::new("access.antigravity.personal").expect("valid access id");
    let access_profile = antigravity_personal_google_access_profile(access_id.clone());
    let version = antigravity_release_binding("1.1.9").expect("fixture release is valid");
    let capabilities = vec![
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [CapabilityConstraint::MaximumTurns(24)],
        ),
        CapabilityRequirement::new(
            Capability::StreamingEvents,
            [CapabilityConstraint::StreamRecordMaximumCount(4096)],
        ),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
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
    let profile = CapabilityProfile::new(capabilities.clone());
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("antigravity.continuation.fixture").expect("valid instance id"),
        InstanceRevision::new("1").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new(target).expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("antigravity-stream-json-v1").expect("valid facade"),
        InstancePolicyId::new("antigravity-continuation-policy").expect("valid policy"),
        profile.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let route = ModelRoute::new(
        ModelRouteId::new("antigravity-continuation-model-route").expect("valid route id"),
        ModelRouteRevision::new("1").expect("valid route revision"),
        instance.id().clone(),
        ModelId::new("gemini-3.6-flash-high").expect("valid model id"),
        profile,
    )
    .with_provider_id(ProviderId::new("google").expect("valid provider id"));
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let services = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
        HostServiceKind::WorkingResource,
    ];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capabilities)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .with_session_provider_state_policy(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    .with_interface_versions([version])
    .require_model_route();
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access_profile, &status, services)
            .with_model_route(&route),
        &requirements,
    )
    .expect("Antigravity continuation fixture preflight succeeds")
}

pub fn headless_plan(
    host: ExecutionHostId,
    target: &str,
    access: ResourceAccess,
    isolation: HarnessIsolation,
    effort: Option<&str>,
    structured_output: bool,
) -> PreflightPlan {
    let descriptor = antigravity_headless_descriptor();
    let access_id = AccessProfileId::new("access.antigravity.personal").expect("valid access id");
    let access_profile = antigravity_personal_google_access_profile(access_id.clone());
    let version = antigravity_release_binding("1.1.9").expect("fixture release is valid");
    let capabilities = headless_capabilities(access, effort, structured_output);
    let profile = CapabilityProfile::new(capabilities.clone());
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("antigravity.headless.fixture").expect("valid instance id"),
        InstanceRevision::new("1").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new(target).expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("antigravity-stream-json-v1").expect("valid facade"),
        InstancePolicyId::new("antigravity-headless-policy").expect("valid policy"),
        profile.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let route = ModelRoute::new(
        ModelRouteId::new("antigravity-headless-model-route").expect("valid route id"),
        ModelRouteRevision::new("1").expect("valid route revision"),
        instance.id().clone(),
        ModelId::new("gemini-3.6-flash-high").expect("valid model id"),
        profile,
    );
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let services = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
    ];
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capabilities)
    .with_harness_isolation(isolation)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_interface_versions([version])
    .require_model_route();
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access_profile, &status, services)
            .with_model_route(&route),
        &requirements,
    )
    .expect("Antigravity headless fixture preflight succeeds")
}

fn headless_capabilities(
    access: ResourceAccess,
    effort: Option<&str>,
    structured_output: bool,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(access),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if let Some(effort) = effort {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(
                ReasoningMode::new(effort).expect("valid effort"),
            )],
        ));
    }
    if structured_output {
        capabilities.push(CapabilityRequirement::new(
            Capability::StructuredOutput,
            [
                CapabilityConstraint::SchemaDialect("json-schema-2020-12".to_owned()),
                CapabilityConstraint::StructuredOutputEnforcement(
                    StructuredOutputEnforcement::ProviderNative,
                ),
            ],
        ));
    }
    capabilities
}
