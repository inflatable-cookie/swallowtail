use super::{
    CODEX_LATEST_QUALIFIED_VERSION, access_requirement, access_state, bind_instance_version,
    bind_required_version, model_route,
};
use swallowtail_adapter_codex::{codex_cli_binding, codex_exec_claim, codex_exec_descriptor};
use swallowtail_core::{
    AccessProfileId, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, DriverRole, ExecutionHostId, ExecutionLayer, HarnessConfigurationPosture,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    InterfaceSupportStatus, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ProtocolFacadeId, SupportAuthority, preflight,
};
use swallowtail_runtime::{OperationPolicy, ProviderRetentionPolicy};

pub fn plan() -> PreflightPlan {
    plan_with([], [])
}

pub fn plan_with(
    optional_capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    optional_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> PreflightPlan {
    plan_with_version(
        CODEX_LATEST_QUALIFIED_VERSION,
        optional_capabilities,
        optional_host_services,
    )
}

pub fn plan_with_version(
    version: &str,
    optional_capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    optional_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> PreflightPlan {
    exec_plan(
        Some(version),
        Some(version),
        optional_capabilities,
        optional_host_services,
    )
}

pub fn unqualified_exec_plan(version: Option<&str>) -> PreflightPlan {
    exec_plan(version, None, [], [])
}

fn exec_plan(
    instance_version: Option<&str>,
    required_version: Option<&str>,
    optional_capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    optional_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> PreflightPlan {
    let posture = required_version.map_or(HarnessConfigurationPosture::Ambient, |_| {
        exec_posture(instance_version)
    });
    let descriptor = codex_exec_descriptor();
    let host_id = ExecutionHostId::new("host.local").expect("host id is valid");
    let access_id = AccessProfileId::new("access.codex").expect("access id is valid");
    let mut capability_requirements =
        vec![CapabilityRequirement::new(Capability::StructuredRun, [])];
    capability_requirements.extend(optional_capabilities);
    let mut host_services = vec![HostServiceKind::Task, HostServiceKind::Process];
    host_services.extend(optional_host_services);
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let instance = bind_instance_version(
        ConfiguredInstance::new(
            ConfiguredInstanceId::new("codex.local").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            host_id.clone(),
            InstanceTargetRef::new("codex-executable").expect("target is valid"),
            InstanceOwnership::HostOwnedEphemeral,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("codex-exec-jsonl").expect("facade is valid"),
            InstancePolicyId::new("read-only").expect("policy is valid"),
            capabilities.clone(),
        )
        .with_harness_configuration_posture(posture),
        instance_version,
    );
    let route = model_route("codex-model-route", &instance, capabilities);
    let (access, status) = access_state(access_id.clone());
    let requirements = bind_required_version(
        OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            host_id,
            access_requirement(access_id),
        )
        .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
        .with_host_services(host_services.clone())
        .with_capabilities(capability_requirements)
        .with_harness_configuration_posture(posture)
        .require_model_route(),
        required_version,
    );
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
            .with_model_route(&route),
        &requirements,
    )
    .expect("Codex fixture preflight succeeds")
}

pub fn current_exec_policy() -> OperationPolicy {
    exec_policy_for_version(CODEX_LATEST_QUALIFIED_VERSION)
}

pub fn bind_current_exec_policy(policy: OperationPolicy) -> OperationPolicy {
    policy.with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
}

pub fn exec_policy_for_version(version: &str) -> OperationPolicy {
    let matched = codex_exec_claim()
        .classify(codex_cli_binding(version).version())
        .expect("fixture version is qualified");
    let retention = if matched.behavior_revision().as_str().contains("retained-") {
        ProviderRetentionPolicy::DurableAllowed
    } else {
        ProviderRetentionPolicy::Prohibited
    };
    OperationPolicy::offline()
        .with_provider_retention(retention)
        .with_harness_configuration_posture(exec_posture(Some(version)))
}

fn exec_posture(version: Option<&str>) -> HarnessConfigurationPosture {
    let version = version.unwrap_or(CODEX_LATEST_QUALIFIED_VERSION);
    if codex_exec_claim()
        .classify(codex_cli_binding(version).version())
        .is_some_and(|matched| matched.support_status() == InterfaceSupportStatus::Deprecated)
    {
        HarnessConfigurationPosture::Ambient
    } else {
        HarnessConfigurationPosture::ProviderSuppressed
    }
}
