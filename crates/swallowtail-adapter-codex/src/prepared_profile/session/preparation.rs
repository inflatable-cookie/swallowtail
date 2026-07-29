use super::{CodexPreparedSession, lifecycle_management_instance};
use crate::prepared_profile::CodexPreparedSessionKind;
use crate::prepared_profile::activity_profile::app_server_activity_profile;
use crate::prepared_profile::input::CodexSessionProfileInput;
use crate::prepared_profile::plan::{
    CodexPreparedEvidence, build_plan, descriptor, failure, instance_with_capabilities,
    model_route, require_driver, requirements,
};
use crate::prepared_profile::session_capabilities::{behavior_revision, session_capabilities};
use crate::selection::CODEX_APP_SERVER_WORKSPACE_BEHAVIOR;
use crate::{
    CodexPreparedDriver, CodexPreparedIntegration, codex_bounded_workspace_access_policy,
    codex_bounded_workspace_capability,
};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, HarnessConfigurationPosture,
    HostServiceKind, OperationShape, SessionProviderStatePolicy,
};
use swallowtail_runtime::{OpenSessionRequest, PreparationFailure};

pub(super) fn prepare_session(
    prepared: &CodexPreparedIntegration,
    kind: CodexPreparedSessionKind,
    input: CodexSessionProfileInput,
) -> Result<CodexPreparedSession, PreparationFailure> {
    require_driver(prepared, CodexPreparedDriver::AppServer)?;
    if kind == CodexPreparedSessionKind::BoundedWorkspace
        && behavior_revision(prepared) != Some(CODEX_APP_SERVER_WORKSPACE_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.codex.preparation.workspace_version_unsupported",
            "Prepared Codex version does not support bounded workspace roots",
        ));
    }
    let (request_id, model, working_resource, deadline, options) = input.into_parts();
    if deadline.is_some() {
        return Err(failure(
            "swallowtail.codex.preparation.session_deadline_unsupported",
            "Codex app-server sessions do not support an operation deadline",
        ));
    }
    let (mut capability_requirements, mut extension_namespaces, access_policy) =
        session_capabilities(kind, &options)?;
    let activity_profile = app_server_activity_profile(prepared)?;
    capability_requirements.extend([
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        activity_profile
            .capability_requirement()
            .expect("available Codex activity profile has a capability requirement"),
    ]);
    if kind == CodexPreparedSessionKind::BoundedWorkspace {
        capability_requirements.push(codex_bounded_workspace_capability());
        extension_namespaces.extend(
            codex_bounded_workspace_access_policy()
                .provider_requests()
                .observed_extensions()
                .cloned(),
        );
    }
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let instance = instance_with_capabilities(prepared, capabilities.clone());
    let route = model_route(
        prepared,
        model.route_id().clone(),
        model.route_revision().clone(),
        model.model_id().clone(),
        capabilities,
    );
    let mut host_services = vec![
        HostServiceKind::Task,
        HostServiceKind::Time,
        HostServiceKind::Process,
    ];
    if kind == CodexPreparedSessionKind::BoundedWorkspace {
        host_services.push(HostServiceKind::WorkingResource);
    }
    let requirements = requirements(
        prepared,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host_services,
        capability_requirements,
    )
    .with_extension_namespaces(extension_namespaces)
    .with_session_access_policy(access_policy)
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route();
    let descriptor = descriptor(prepared);
    let plan = build_plan(
        prepared,
        &descriptor,
        &instance,
        Some(&route),
        &requirements,
    )?;
    let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?
        .with_options(options);
    Ok(CodexPreparedSession {
        kind,
        evidence: CodexPreparedEvidence::from_prepared_with_activity_profile(
            prepared,
            plan,
            activity_profile,
        )?,
        request,
        management_instance: lifecycle_management_instance(prepared),
    })
}
