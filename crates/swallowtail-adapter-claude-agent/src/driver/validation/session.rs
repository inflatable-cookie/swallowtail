use super::*;
use crate::selection::{ClaudeAgentBehavior, ClaudeAgentPlanSelection};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, ProviderExecutionPolicy, ProviderRecoveryPolicy,
    ProviderRetentionPolicy, StreamReattachmentPolicy, StructuredRunRequest,
    validate_harness_configuration_policy, validate_harness_isolation_policy,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ClaudeAgentLifecycleCapabilities {
    pub(super) close: bool,
    pub(super) delete: bool,
    pub(super) load: bool,
    pub(super) resume: bool,
}

pub(super) fn permission_handling(
    plan: &PreflightPlan,
) -> Result<crate::ClaudeAgentPermissionHandling, RuntimeFailure> {
    let namespaces = plan
        .requirements()
        .extension_namespaces()
        .collect::<Vec<_>>();
    match namespaces.as_slice() {
        [] => Ok(crate::ClaudeAgentPermissionHandling::RejectAndStop),
        [namespace] if *namespace == &crate::claude_agent_permission_namespace() => {
            Ok(crate::ClaudeAgentPermissionHandling::ConsumerMediated)
        }
        _ => Err(failure(
            "swallowtail.claude_agent.acp.permission_profile_mismatch",
            "Claude Agent permission handling does not match its immutable preflight plan",
        )),
    }
}

fn session_access_policy(plan: &PreflightPlan) -> Result<SessionAccessPolicy, RuntimeFailure> {
    Ok(match permission_handling(plan)? {
        crate::ClaudeAgentPermissionHandling::RejectAndStop => {
            SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        }
        crate::ClaudeAgentPermissionHandling::ConsumerMediated => {
            SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
                ResourceAccess::Read,
                [crate::claude_agent_permission_namespace()],
            )
        }
    })
}

pub(super) fn validate_plan(
    plan: &PreflightPlan,
    credential: Option<&CredentialRef>,
) -> Result<ClaudeAgentPlanSelection, RuntimeFailure> {
    if plan.driver_identity().id().as_str() != DRIVER_ID {
        return Err(failure(
            "swallowtail.claude_agent.acp.plan_driver_mismatch",
            "Preflight plan is bound to a different driver",
        ));
    }
    let access_matches = match plan.credential_mechanism() {
        CredentialMechanism::ApiKey => {
            credential.is_some()
                && plan.credential_reference() == credential
                && plan
                    .requirements()
                    .host_services()
                    .any(|service| service == HostServiceKind::Credential)
        }
        CredentialMechanism::LocalUnauthenticated => {
            credential.is_none() && plan.credential_reference().is_none()
        }
        _ => false,
    };
    if !access_matches || plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE {
        return Err(failure(
            "swallowtail.claude_agent.acp.access_profile_rejected",
            "Claude Agent ACP requires configured API-key access or local Claude authentication",
        ));
    }
    if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
        return Err(failure(
            "swallowtail.claude_agent.acp.configuration_posture_rejected",
            "Claude Agent ACP requires explicit ambient configuration",
        ));
    }
    if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
        return Err(failure(
            "swallowtail.claude_agent.acp.isolation_rejected",
            "Claude Agent ACP requires explicit ambient-host isolation posture",
        ));
    }
    crate::selection::select_claude_agent_plan(plan)
}

pub(super) fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.claude_agent.acp.task_service_missing",
            "Claude Agent ACP requires a scoped task service",
        ),
        (
            services.time().is_some(),
            "swallowtail.claude_agent.acp.time_service_missing",
            "Claude Agent ACP requires a monotonic time service",
        ),
        (
            services.process().is_some(),
            "swallowtail.claude_agent.acp.process_service_missing",
            "Claude Agent ACP requires a process service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.claude_agent.acp.resource_service_missing",
            "Claude Agent ACP requires a working-resource service",
        ),
        (
            services.working_resource_io().is_some(),
            "swallowtail.claude_agent.acp.resource_io_service_missing",
            "Claude Agent ACP requires working-resource read I/O",
        ),
    ] {
        if !present {
            return Err(failure(code, message));
        }
    }
    if plan.credential_mechanism() == &CredentialMechanism::ApiKey
        && services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.credential_service_missing",
            "Claude Agent ACP API-key access requires a credential service",
        ));
    }
    swallowtail_runtime::validate_session_plan_agreement(plan, request.plan_agreement())?;
    if request.access_policy() != &session_access_policy(plan)? {
        return Err(failure(
            "swallowtail.claude_agent.acp.access_policy_rejected",
            "Claude Agent ACP requires its preflight-bound ambient read-only access policy",
        ));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free session"));
    }
    if request.deadline().is_some() {
        return Err(unsupported("session deadline"));
    }
    if request.options().developer_instructions().is_some() || request.options().tools().len() != 0
    {
        return Err(unsupported("session options"));
    }
    let requested_mode = request.options().harness_mode();
    let planned_mode = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::HarnessModeSelection)
        .and_then(|requirement| {
            requirement
                .constraints()
                .find_map(|constraint| match constraint {
                    CapabilityConstraint::HarnessMode(mode) => Some(*mode),
                    _ => None,
                })
        });
    if requested_mode != planned_mode {
        return Err(failure(
            "swallowtail.claude_agent.acp.harness_mode_mismatch",
            "Claude Agent session harness mode does not match its preflight plan",
        ));
    }
    Ok(())
}

pub(super) fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.requirements().execution_layer() != ExecutionLayer::HarnessInteraction
        || plan.requirements().operation_shape() != OperationShape::StructuredRun
        || plan.requirements().driver_role() != DriverRole::StructuredRun
        || plan.ownership() != swallowtail_core::InstanceOwnership::HostOwnedEphemeral
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.run_plan_mismatch",
            "Claude Agent structured run does not match its preflight plan",
        ));
    }
    let _ = permission_handling(plan)?;
    let mut required_services = vec![
        HostServiceKind::Task,
        HostServiceKind::Time,
        HostServiceKind::Process,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
    ];
    if plan.credential_mechanism() == &CredentialMechanism::ApiKey {
        required_services.push(HostServiceKind::Credential);
    }
    for service in required_services {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
            || !services.available_kinds().contains(&service)
        {
            return Err(failure(
                "swallowtail.claude_agent.acp.run_host_service_missing",
                "Claude Agent structured run requires its preflight-bound host services",
            ));
        }
    }
    for capability in [
        swallowtail_core::Capability::StructuredRun,
        swallowtail_core::Capability::StreamingEvents,
        swallowtail_core::Capability::WorkingResource,
    ] {
        if !plan
            .requirements()
            .capabilities()
            .any(|required| required.capability() == capability)
        {
            return Err(failure(
                "swallowtail.claude_agent.acp.run_capability_mismatch",
                "Claude Agent structured-run capabilities do not match the preflight plan",
            ));
        }
    }
    let owned_session_cleanup = run_owns_session_cleanup(plan)?;
    let working_resource = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == swallowtail_core::Capability::WorkingResource);
    if working_resource.is_none_or(|required| {
        !required.constraints().any(|constraint| {
            constraint
                == &swallowtail_core::CapabilityConstraint::ResourceAccess(
                    ResourceAccess::ReadWrite,
                )
        })
    }) {
        return Err(failure(
            "swallowtail.claude_agent.acp.run_capability_mismatch",
            "Claude Agent structured-run write access does not match the preflight plan",
        ));
    }
    let interruption = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == swallowtail_core::Capability::Interruption);
    if interruption.is_none_or(|required| {
        !required.constraints().any(|constraint| {
            constraint
                == &swallowtail_core::CapabilityConstraint::CancellationScope(
                    CancellationScope::StructuredRun,
                )
        })
    }) {
        return Err(failure(
            "swallowtail.claude_agent.acp.run_capability_mismatch",
            "Claude Agent structured-run cancellation scope does not match the preflight plan",
        ));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free structured run"));
    }
    if request.attachments().len() != 0
        || request.tools().len() != 0
        || request.structured_output().is_some()
        || request.maximum_output_tokens().is_some()
    {
        return Err(unsupported(
            "structured-run attachments, consumer tools, schema, or output-token limit",
        ));
    }
    let policy = request.policy();
    validate_harness_isolation_policy(plan, policy).map_err(|_| {
        failure(
            "swallowtail.claude_agent.acp.run_isolation_mismatch",
            "Claude Agent structured-run isolation does not match the preflight plan",
        )
    })?;
    validate_harness_configuration_policy(plan, policy).map_err(|_| {
        failure(
            "swallowtail.claude_agent.acp.run_configuration_mismatch",
            "Claude Agent structured-run configuration does not match the preflight plan",
        )
    })?;
    validate_run_reasoning(plan, policy.reasoning_mode())?;
    if policy.external_network() != ExternalNetworkPolicy::Denied
        || policy.external_search() != ExternalSearchPolicy::Disabled
        || policy.provider_execution() != ProviderExecutionPolicy::Attached
        || policy.provider_retention()
            != if owned_session_cleanup {
                ProviderRetentionPolicy::TemporaryAllowed
            } else {
                ProviderRetentionPolicy::DurableAllowed
            }
        || policy.provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || policy.stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported("structured-run lifecycle or inference policy"));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.deadline_elapsed",
            "Claude Agent run deadline elapsed before provider work",
        ));
    }
    Ok(())
}

pub(super) fn run_owns_session_cleanup(plan: &PreflightPlan) -> Result<bool, RuntimeFailure> {
    let has = |capability| {
        plan.requirements()
            .capabilities()
            .any(|required| required.capability() == capability)
    };
    let durable = has(swallowtail_core::Capability::ProviderDurableRetention);
    let temporary = has(swallowtail_core::Capability::ProviderTemporaryRetention);
    let native_close = has(swallowtail_core::Capability::ProviderNativeSessionClose);
    let deletion = plan.requirements().capabilities().find(|required| {
        required.capability() == swallowtail_core::Capability::OwnedRemoteResourceDeletion
    });
    let exact_deletion = deletion.is_some_and(|required| {
        required.constraints().eq([
            &swallowtail_core::CapabilityConstraint::OwnedRemoteResource(
                swallowtail_core::OwnedRemoteResourceKind::Session,
            ),
        ])
    });
    match (
        durable,
        temporary,
        native_close,
        deletion.is_some(),
        exact_deletion,
    ) {
        (true, false, false, false, false) => Ok(false),
        (false, true, true, true, true) => Ok(true),
        _ => Err(failure(
            "swallowtail.claude_agent.acp.run_retention_mismatch",
            "Claude Agent structured-run retention and cleanup capabilities do not match",
        )),
    }
}

