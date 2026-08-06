use crate::OhMyPiPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, EndpointAuthorization, EntitlementState, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ResourceAccess,
    RuntimeReadiness, SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Immutable executable, access, activity, and preflight evidence.
pub struct OhMyPiPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    operation: PreparedOperationEvidence,
}

impl OhMyPiPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &OhMyPiPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
        })
    }

    pub(super) fn from_prepared_with_activity(
        prepared: &OhMyPiPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    /// Returns the qualified installed-package observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the admitted access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    pub(super) fn low_level_driver(&self) -> crate::OhMyPiRpcDriver {
        crate::OhMyPiRpcDriver::new(self.environment.clone())
    }
}

pub(super) fn instance_with_capabilities(
    prepared: &OhMyPiPreparedIntegration,
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
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_harness_rpc_policy(crate::prepared::instance::rpc_policy())
}

pub(super) fn requirements(
    prepared: &OhMyPiPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    image_attachments: bool,
) -> OperationRequirements {
    let mut host_services = vec![
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::WorkingResource,
        HostServiceKind::Time,
    ];
    if image_attachments {
        host_services.extend([HostServiceKind::Attachment, HostServiceKind::BlockingWork]);
    }
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        swallowtail_core::DriverRole::InteractiveSession,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(host_services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_harness_rpc_policy(crate::prepared::instance::rpc_policy())
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .require_model_route()
}

pub(super) fn run_requirements(
    prepared: &OhMyPiPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    image_attachments: bool,
) -> OperationRequirements {
    let mut host_services = vec![
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::WorkingResource,
        HostServiceKind::Time,
    ];
    if image_attachments {
        host_services.extend([HostServiceKind::Attachment, HostServiceKind::BlockingWork]);
    }
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        swallowtail_core::DriverRole::StructuredRun,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(host_services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .require_model_route()
}

pub(super) fn catalogue_requirements(
    prepared: &OhMyPiPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        swallowtail_core::DriverRole::ModelCatalog,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services([
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
    ])
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_harness_rpc_policy(crate::prepared::instance::rpc_policy())
}

pub(super) fn build_plan(
    prepared: &OhMyPiPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::oh_my_pi_rpc_descriptor();
    let context = PreflightContext::new(
        &descriptor,
        instance,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
    .with_model_route(route);
    preflight(&context, requirements).map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

pub(super) fn build_catalogue_plan(
    prepared: &OhMyPiPreparedIntegration,
    instance: &ConfiguredInstance,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::oh_my_pi_rpc_descriptor();
    let context = PreflightContext::new(
        &descriptor,
        instance,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    );
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
