use swallowtail_adapter_claude_agent::claude_agent_acp_descriptor;
use swallowtail_core::{
    AccessRequirement, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
    HarnessConfigurationPosture, ModelRoute, OperationRequirements, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution, build_plan,
    instance_with_capabilities,
};

use super::fixtures::{agent_prepared, agent_run_with};
use super::naming::source;

pub(super) fn contribution_and_drifted_access() -> (
    ConsumerRouteProjectionContribution,
    Vec<(AccessStatus, Option<ConsumerRouteApplicability>)>,
) {
    let prepared = agent_prepared("1");
    let run = agent_run_with(prepared.clone(), None, false, false);
    let contribution = run
        .consumer_route_projection_contribution(source("projection.shared.access"))
        .expect("ready run contributes");
    let base = run.plan();
    let binding = contribution
        .applicability()
        .model()
        .expect("the run fixes a model route");
    let capabilities =
        swallowtail_core::CapabilityProfile::new(base.requirements().capabilities().cloned());
    let instance = instance_with_capabilities(prepared.instance(), capabilities.clone())
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let model = ModelRoute::new(
        binding.route_id().clone(),
        binding.route_revision().clone(),
        instance.id().clone(),
        binding.model_id().clone(),
        capabilities,
    );

    let statuses = [
        status(
            CredentialState::Expired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Exhausted,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Denied,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Degraded,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ExperimentalObserved,
        ),
    ];
    let shifted = statuses
        .into_iter()
        .map(|status| {
            let requirements =
                requirements_with_access(base.requirements(), instance.ownership(), &status);
            let result = build_plan(
                &claude_agent_acp_descriptor(),
                &instance,
                Some(&model),
                &requirements,
                prepared.access_profile(),
                &status,
                prepared.available_host_services(),
            );
            let plan = result
                .ok()
                .map(|plan| ConsumerRouteApplicability::from_plan(&plan));
            (status, plan)
        })
        .collect();
    (contribution, shifted)
}

fn requirements_with_access(
    base: &OperationRequirements,
    ownership: swallowtail_core::InstanceOwnership,
    status: &AccessStatus,
) -> OperationRequirements {
    let mut requirements = OperationRequirements::new(
        base.execution_layer(),
        base.operation_shape(),
        base.driver_role(),
        base.execution_host_id().clone(),
        AccessRequirement::new(status.profile_id().clone())
            .with_credential_states([status.credential()])
            .with_entitlement_states([status.entitlement()])
            .with_endpoint_authorizations([status.endpoint_authorization()])
            .with_runtime_readiness([status.runtime_readiness()])
            .with_support_authorities([status.support_authority()]),
    )
    .with_ownership_modes([ownership])
    .with_host_services(base.host_services())
    .with_capabilities(base.capabilities().cloned())
    .with_extension_namespaces(base.extension_namespaces().cloned())
    .with_interface_versions(base.interface_versions().cloned());
    if base.model_route_required() {
        requirements = requirements.require_model_route();
    }
    if let Some(isolation) = base.harness_isolation() {
        requirements = requirements.with_harness_isolation(isolation);
    }
    if let Some(posture) = base.harness_configuration_posture() {
        requirements = requirements.with_harness_configuration_posture(posture);
    }
    requirements
}

fn status(
    credential: CredentialState,
    entitlement: EntitlementState,
    endpoint: EndpointAuthorization,
    readiness: RuntimeReadiness,
    authority: SupportAuthority,
) -> AccessStatus {
    AccessStatus::new(
        swallowtail_core::AccessProfileId::new("projection.agent.access")
            .expect("access id is valid"),
        credential,
        entitlement,
        endpoint,
        readiness,
        authority,
    )
}
