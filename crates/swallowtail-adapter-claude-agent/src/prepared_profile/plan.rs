use crate::ClaudeAgentPreparedIntegration;
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, CredentialMechanism,
    CredentialState, Diagnostic, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, ModelRoute, OperationRequirements, OperationShape, PreflightPlan,
    ResourceAccess, SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Portable evidence shared by prepared Claude Agent operations.
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
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            credential: prepared.access_profile().credential_reference().cloned(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    /// Returns the qualified installed-executable observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared-operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the validated preflight plan.
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
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(super) fn requirements(
    prepared: &ClaudeAgentPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    permission_handling: super::ClaudeAgentPermissionHandling,
) -> OperationRequirements {
    let requirements = swallowtail_runtime::base_requirements(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        swallowtail_core::DriverRole::InteractiveSession,
        prepared.instance(),
        prepared.access_profile(),
        claude_agent_credential_states(prepared),
        capabilities,
    )
    .with_host_services(operation_host_services(prepared))
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .require_model_route();
    match permission_handling {
        super::ClaudeAgentPermissionHandling::RejectAndStop => requirements
            .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read)),
        super::ClaudeAgentPermissionHandling::ConsumerMediated => requirements
            .with_extension_namespaces([crate::claude_agent_permission_namespace()])
            .with_session_access_policy(
                SessionAccessPolicy::ambient_harness_with_consumer_mediated_requests(
                    ResourceAccess::Read,
                    [crate::claude_agent_permission_namespace()],
                ),
            ),
    }
}

pub(super) fn run_requirements(
    prepared: &ClaudeAgentPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    permission_handling: super::ClaudeAgentPermissionHandling,
) -> OperationRequirements {
    let requirements = swallowtail_runtime::base_requirements(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        swallowtail_core::DriverRole::StructuredRun,
        prepared.instance(),
        prepared.access_profile(),
        claude_agent_credential_states(prepared),
        capabilities,
    )
    .with_host_services(operation_host_services(prepared))
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

pub(super) fn claude_agent_credential_states(
    prepared: &ClaudeAgentPreparedIntegration,
) -> Vec<CredentialState> {
    match prepared.access_profile().credential_mechanism() {
        CredentialMechanism::ApiKey => vec![CredentialState::Ready],
        CredentialMechanism::LocalUnauthenticated => vec![CredentialState::NotRequired],
        _ => unreachable!("Claude Agent preparation rejected the credential mechanism"),
    }
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
    swallowtail_runtime::build_plan(
        &crate::claude_agent_acp_descriptor(),
        instance,
        route,
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}

pub(super) fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}
