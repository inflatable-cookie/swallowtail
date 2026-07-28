use crate::ClaudeAgentPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialMechanism, CredentialState, Diagnostic, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ResourceAccess,
    RuntimeReadiness, SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    credential: Option<swallowtail_core::CredentialRef>,
    operation: PreparedOperationEvidence,
}

impl ClaudeAgentPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &ClaudeAgentPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            credential: prepared.access_profile().credential_reference().cloned(),
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
        })
    }

    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
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

    pub(super) fn low_level_driver(&self) -> crate::ClaudeAgentAcpDriver {
        match self.credential.as_ref() {
            Some(credential) => {
                crate::ClaudeAgentAcpDriver::new(self.environment.clone(), credential.clone())
            }
            None => crate::ClaudeAgentAcpDriver::with_local_auth(self.environment.clone()),
        }
    }
}

pub(super) fn instance_with_capabilities(
    prepared: &ClaudeAgentPreparedIntegration,
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
    prepared: &ClaudeAgentPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        swallowtail_core::DriverRole::InteractiveSession,
        prepared.instance().execution_host_id().clone(),
        access_requirement(prepared),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(operation_host_services(prepared))
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .require_model_route()
}

pub(super) fn run_requirements(
    prepared: &ClaudeAgentPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    permission_handling: super::ClaudeAgentPermissionHandling,
) -> OperationRequirements {
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        swallowtail_core::DriverRole::StructuredRun,
        prepared.instance().execution_host_id().clone(),
        access_requirement(prepared),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(operation_host_services(prepared))
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route();
    match permission_handling {
        super::ClaudeAgentPermissionHandling::RejectAndStop => requirements,
        super::ClaudeAgentPermissionHandling::ConsumerMediated => {
            requirements.with_extension_namespaces([crate::claude_agent_permission_namespace()])
        }
    }
}

pub(super) fn access_requirement(prepared: &ClaudeAgentPreparedIntegration) -> AccessRequirement {
    let credential_state = match prepared.access_profile().credential_mechanism() {
        CredentialMechanism::ApiKey => CredentialState::Ready,
        CredentialMechanism::LocalUnauthenticated => CredentialState::NotRequired,
        _ => unreachable!("Claude Agent preparation rejected the credential mechanism"),
    };
    AccessRequirement::new(prepared.access_profile().id().clone())
        .with_credential_states([credential_state])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([prepared.access_profile().support_authority()])
}

pub(super) fn operation_host_services(
    prepared: &ClaudeAgentPreparedIntegration,
) -> Vec<HostServiceKind> {
    let mut services = vec![
        HostServiceKind::Task,
        HostServiceKind::Time,
        HostServiceKind::Process,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
    ];
    if prepared.access_profile().credential_mechanism() == &CredentialMechanism::ApiKey {
        services.push(HostServiceKind::Credential);
    }
    services
}

pub(super) fn build_plan(
    prepared: &ClaudeAgentPreparedIntegration,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::claude_agent_acp_descriptor();
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
