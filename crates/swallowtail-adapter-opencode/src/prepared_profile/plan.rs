use crate::OpenCodePreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ResourceAccess,
    RuntimeReadiness, SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodePreparedEvidence {
    server: crate::OpenCodePreparedServerObservation,
    operation: PreparedOperationEvidence,
}

impl OpenCodePreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &OpenCodePreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            server: prepared.server().clone(),
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
        })
    }

    pub(super) fn from_prepared_with_activity(
        prepared: &OpenCodePreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            server: prepared.server().clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    #[must_use]
    pub const fn server(&self) -> &crate::OpenCodePreparedServerObservation {
        &self.server
    }

    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }
}

pub(super) fn instance_with_capabilities(
    prepared: &OpenCodePreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    let base = prepared.instance();
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(super) fn requirements(
    prepared: &OpenCodePreparedIntegration,
    role: DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    image_attachments: bool,
    provider_callbacks: bool,
) -> OperationRequirements {
    let descriptor = crate::opencode_http_descriptor();
    let mut host_services = descriptor.required_host_services(role).collect::<Vec<_>>();
    if image_attachments {
        host_services.push(swallowtail_core::HostServiceKind::Attachment);
    }
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        role,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(host_services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.server().binding().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);

    if role == DriverRole::InteractiveSession {
        let namespaces = provider_callbacks
            .then(|| {
                [
                    crate::driver::callback::permission_namespace(),
                    crate::driver::callback::question_namespace(),
                ]
            })
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let access_policy = if provider_callbacks {
            SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
                ResourceAccess::ReadWrite,
                namespaces.clone(),
            )
        } else {
            SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        };
        requirements
            .with_extension_namespaces(namespaces)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_session_access_policy(access_policy)
            .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
            .require_model_route()
    } else {
        requirements
    }
}

pub(super) fn management_requirements(
    prepared: &OpenCodePreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    let role = DriverRole::ProviderSessionManagement;
    let descriptor = crate::opencode_http_descriptor();
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionManagement,
        role,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(descriptor.required_host_services(role))
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.server().binding().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(super) fn run_requirements(
    prepared: &OpenCodePreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    image_attachments: bool,
    provider_callbacks: bool,
) -> OperationRequirements {
    let role = DriverRole::StructuredRun;
    let descriptor = crate::opencode_http_descriptor();
    let mut host_services = descriptor.required_host_services(role).collect::<Vec<_>>();
    if image_attachments {
        host_services.push(swallowtail_core::HostServiceKind::Attachment);
    }
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        role,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(host_services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.server().binding().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route();
    if provider_callbacks {
        requirements.with_extension_namespaces([
            crate::driver::callback::permission_namespace(),
            crate::driver::callback::question_namespace(),
        ])
    } else {
        requirements
    }
}

pub(super) fn build_plan(
    prepared: &OpenCodePreparedIntegration,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::opencode_http_descriptor();
    let context = PreflightContext::new(
        &descriptor,
        instance,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    );
    let context = match route {
        Some(route) => context.with_model_route(route),
        None => context,
    };
    preflight(&context, requirements).map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

pub(super) fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}
