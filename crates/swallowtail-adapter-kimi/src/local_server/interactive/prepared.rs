use super::{
    KimiLocalServerPermissionMode, KimiLocalServerPreparedSession,
    KimiLocalServerPreparedSessionFuture, KimiLocalServerSessionInput,
};
use crate::local_server::KimiLocalServerPreparedIntegration;
use crate::local_server::prepared::lifecycle_capabilities;
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, ResourceAccess,
    ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
    preflight,
};
use swallowtail_runtime::{
    HostServices, OpenSessionRequest, PreparationFailure, PreparationStage,
    PreparedOperationEvidence, RequestId, ResumeSessionRequest, SessionResumeBinding,
};

pub(super) fn open(
    prepared: &KimiLocalServerPreparedSession,
    services: HostServices,
) -> KimiLocalServerPreparedSessionFuture {
    let driver = prepared.low_level_driver();
    let plan = prepared.plan().clone();
    let request = prepared.request().clone();
    let management = prepared.management_instance.clone();
    let access = prepared.evidence.access().clone();
    Box::pin(async move {
        driver
            .open_bound_session(plan, request, services, management, access)
            .await
    })
}

pub(super) fn resume(
    prepared: &KimiLocalServerPreparedSession,
    request_id: RequestId,
    binding: SessionResumeBinding,
    services: HostServices,
) -> Result<KimiLocalServerPreparedSessionFuture, PreparationFailure> {
    let request = ResumeSessionRequest::from_plan(
        prepared.plan(),
        request_id,
        binding,
        prepared
            .request()
            .working_resource()
            .expect("prepared local-server session binds a resource")
            .clone(),
        prepared.request().deadline(),
    )?
    .with_options(prepared.request().options().clone());
    let driver = prepared.low_level_driver();
    let plan = prepared.plan().clone();
    let management = prepared.management_instance.clone();
    let access = prepared.evidence.access().clone();
    Ok(Box::pin(async move {
        driver
            .resume_bound_session(plan, request, services, management, access)
            .await
    }))
}

impl KimiLocalServerPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: KimiLocalServerSessionInput,
    ) -> Result<KimiLocalServerPreparedSession, PreparationFailure> {
        let KimiLocalServerSessionInput {
            request_id,
            model,
            working_resource,
            deadline,
            options,
            configuration,
            allow_unverified_newer,
        } = input;
        if !self.server().is_qualified() && !allow_unverified_newer {
            return Err(failure(
                "swallowtail.kimi.local_server.preparation.session_unverified_newer",
                "Newer unverified Kimi local-server sessions require explicit acceptance",
            ));
        }
        validate_options(&options)?;
        validate_revision_options(self, &configuration)?;

        let capabilities = session_capabilities(&options);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        );
        let access_policy = access_policy(configuration.permission_mode());
        let requirements = requirements(self, capabilities.clone(), access_policy.clone());
        let descriptor = crate::kimi_local_server_descriptor();
        let context = PreflightContext::new(
            &descriptor,
            &instance,
            self.access_profile(),
            self.access_evidence().status(),
            self.available_host_services(),
        )
        .with_model_route(&route);
        let plan = preflight(&context, &requirements).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?
            .with_options(options);
        Ok(KimiLocalServerPreparedSession {
            evidence: PreparedOperationEvidence::from_plan(plan, self.access_evidence().clone())?,
            request,
            configuration,
            management_instance: instance_with_capabilities(self, lifecycle_capabilities()),
        })
    }
}

fn session_capabilities(options: &swallowtail_runtime::SessionOptions) -> CapabilityProfile {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
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
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if let Some(mode) = options.reasoning_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(mode.clone())],
        ));
    }
    CapabilityProfile::new(capabilities)
}

fn requirements(
    prepared: &KimiLocalServerPreparedIntegration,
    capabilities: CapabilityProfile,
    access_policy: SessionAccessPolicy,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(
        crate::kimi_local_server_descriptor()
            .required_host_services(DriverRole::InteractiveSession),
    )
    .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
        CapabilityRequirement::new(capability, constraints.iter().cloned())
    }))
    .with_interface_versions([prepared.server().binding().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_access_policy(access_policy)
    .with_session_provider_state_policy(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    .require_model_route()
}

pub(in crate::local_server) fn access_policy(
    permission: KimiLocalServerPermissionMode,
) -> SessionAccessPolicy {
    if permission == KimiLocalServerPermissionMode::Manual {
        SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
            ResourceAccess::ReadWrite,
            [
                super::callbacks::approval_namespace(),
                super::callbacks::question_namespace(),
            ],
        )
    } else {
        SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
    }
}

fn validate_options(
    options: &swallowtail_runtime::SessionOptions,
) -> Result<(), PreparationFailure> {
    if options.developer_instructions().is_some() || options.tools().len() != 0 {
        return Err(failure(
            "swallowtail.kimi.local_server.preparation.session_options_unsupported",
            "Kimi local-server sessions do not support developer instructions or consumer tools",
        ));
    }
    Ok(())
}

pub(in crate::local_server) fn validate_revision_options(
    prepared: &KimiLocalServerPreparedIntegration,
    configuration: &super::KimiLocalServerSessionConfiguration,
) -> Result<(), PreparationFailure> {
    if configuration.profile().is_none() && configuration.disabled_tools().len() == 0 {
        return Ok(());
    }
    if super::super::selection::supports_profile_tools(prepared.server().compatibility()) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.local_server.preparation.revision_option_unsupported",
            "Selected Kimi local-server options require the profile-and-tools revision",
        ))
    }
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
#[path = "prepared/instance.rs"]
mod instance;

use self::instance::instance_with_capabilities;
