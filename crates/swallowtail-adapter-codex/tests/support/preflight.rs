use swallowtail_adapter_codex::{
    CODEX_LATEST_QUALIFIED_VERSION, codex_app_server_descriptor,
    codex_bounded_workspace_access_policy, codex_bounded_workspace_capability, codex_cli_binding,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, HarnessConfigurationPosture,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, ProtocolFacadeId, RuntimeReadiness, SessionAccessPolicy,
    SupportAuthority, preflight,
};

mod exec;
mod unqualified;
pub use exec::{
    bind_current_exec_policy, current_exec_policy, exec_policy_for_version, plan, plan_with,
    plan_with_version, unqualified_exec_plan,
};
pub use unqualified::unqualified_app_server_plan;

pub fn app_server_plan(role: DriverRole) -> PreflightPlan {
    app_server_plan_with(role, [], [])
}

pub fn app_server_plan_with(
    role: DriverRole,
    optional_capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    optional_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> PreflightPlan {
    app_server_plan_for_version(
        role,
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ConfiguredInstanceId::new("codex.app-server.local").expect("instance id is valid"),
        InstanceTargetRef::new("codex-app-server-executable").expect("target is valid"),
        CODEX_LATEST_QUALIFIED_VERSION,
        optional_capabilities,
        optional_host_services,
    )
}

pub fn app_server_plan_for(
    role: DriverRole,
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
    optional_capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    optional_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> PreflightPlan {
    app_server_plan_for_version(
        role,
        host_id,
        instance_id,
        target,
        CODEX_LATEST_QUALIFIED_VERSION,
        optional_capabilities,
        optional_host_services,
    )
}

pub fn app_server_plan_for_version(
    role: DriverRole,
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
    version: &str,
    optional_capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    optional_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> PreflightPlan {
    app_server_plan_for_policy(
        role,
        host_id,
        instance_id,
        target,
        Some(version),
        Some(version),
        optional_capabilities,
        optional_host_services,
        SessionAccessPolicy::read_only(),
        [],
    )
}

#[allow(clippy::too_many_arguments)]
fn app_server_plan_for_policy(
    role: DriverRole,
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
    instance_version: Option<&str>,
    required_version: Option<&str>,
    optional_capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    optional_host_services: impl IntoIterator<Item = HostServiceKind>,
    access_policy: SessionAccessPolicy,
    extensions: impl IntoIterator<Item = swallowtail_core::ExtensionNamespace>,
) -> PreflightPlan {
    let descriptor = codex_app_server_descriptor();
    let access_id = AccessProfileId::new("access.codex").expect("access id is valid");
    let capability = if role == DriverRole::ModelCatalog {
        Capability::ModelCatalog
    } else {
        Capability::InteractiveSession
    };
    let mut capability_requirements = vec![CapabilityRequirement::new(capability, [])];
    capability_requirements.extend(optional_capabilities);
    let mut host_services = vec![HostServiceKind::Task, HostServiceKind::Process];
    host_services.extend(optional_host_services);
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let instance = bind_instance_version(
        ConfiguredInstance::new(
            instance_id,
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            host_id.clone(),
            target,
            InstanceOwnership::HostOwnedPersistent,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("codex-app-server-v2").expect("facade is valid"),
            InstancePolicyId::new("read-only-no-approval").expect("policy is valid"),
            capabilities.clone(),
        )
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient),
        instance_version,
    );
    let route = model_route("codex-app-server-model-route", &instance, capabilities);
    let (access, status) = access_state(access_id.clone());
    let requirements = bind_required_version(
        OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            role,
            host_id,
            access_requirement(access_id),
        )
        .with_ownership_modes([InstanceOwnership::HostOwnedPersistent])
        .with_host_services(host_services.clone())
        .with_capabilities(capability_requirements)
        .with_extension_namespaces(extensions)
        .with_session_access_policy(access_policy)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient),
        required_version,
    );
    let context = PreflightContext::new(&descriptor, &instance, &access, &status, host_services);
    if role == DriverRole::ModelCatalog {
        preflight(&context, &requirements).expect("app-server catalog preflight succeeds")
    } else {
        preflight(
            &context.with_model_route(&route),
            &requirements.require_model_route(),
        )
        .expect("app-server session preflight succeeds")
    }
}

pub fn bounded_workspace_plan() -> PreflightPlan {
    bounded_workspace_plan_for(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ConfiguredInstanceId::new("codex.app-server.local").expect("instance id is valid"),
        InstanceTargetRef::new("codex-app-server-executable").expect("target is valid"),
    )
}

pub fn bounded_workspace_plan_for(
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
) -> PreflightPlan {
    bounded_workspace_plan_for_version(host_id, instance_id, target, CODEX_LATEST_QUALIFIED_VERSION)
}

pub fn bounded_workspace_plan_for_version(
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
    version: &str,
) -> PreflightPlan {
    let policy = codex_bounded_workspace_access_policy();
    let extensions = policy
        .provider_requests()
        .observed_extensions()
        .cloned()
        .collect::<Vec<_>>();
    app_server_plan_for_policy(
        DriverRole::InteractiveSession,
        host_id,
        instance_id,
        target,
        Some(version),
        Some(version),
        [codex_bounded_workspace_capability()],
        [HostServiceKind::WorkingResource],
        policy,
        extensions,
    )
}

fn bind_instance_version(
    instance: ConfiguredInstance,
    version: Option<&str>,
) -> ConfiguredInstance {
    version.map_or(instance.clone(), |version| {
        instance.with_interface_versions([codex_cli_binding(version)])
    })
}

fn bind_required_version(
    requirements: OperationRequirements,
    version: Option<&str>,
) -> OperationRequirements {
    version.map_or(requirements.clone(), |version| {
        requirements.with_interface_versions([codex_cli_binding(version)])
    })
}

fn model_route(
    id: &str,
    instance: &ConfiguredInstance,
    capabilities: CapabilityProfile,
) -> ModelRoute {
    ModelRoute::new(
        ModelRouteId::new(id).expect("route id is valid"),
        ModelRouteRevision::new("1").expect("route revision is valid"),
        instance.id().clone(),
        ModelId::new("gpt-5.4-mini").expect("model id is valid"),
        capabilities,
    )
}

fn access_state(access_id: AccessProfileId) -> (AccessProfile, AccessStatus) {
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("codex").expect("audience is valid"),
        SupportAuthority::ProviderSupported,
    );
    let status = AccessStatus::new(
        access_id,
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    (access, status)
}

fn access_requirement(access_id: AccessProfileId) -> AccessRequirement {
    AccessRequirement::new(access_id)
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported])
}
