use crate::{
    CodexModelVerbosity, CodexPreparedDriver, CodexPreparedIntegration,
    codex_app_server_descriptor, codex_exec_descriptor,
};
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, CredentialState, Diagnostic,
    DriverDescriptor, ExecutionLayer, HostServiceKind, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, OperationShape, PreflightPlan, SafeDiagnostic,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Portable evidence shared by prepared Codex operations.
pub struct CodexPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    operation: swallowtail_runtime::PreparedOperationEvidence,
    model_verbosity: Option<CodexModelVerbosity>,
}

impl CodexPreparedEvidence {
    pub(crate) fn from_prepared(
        prepared: &CodexPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            operation: swallowtail_runtime::PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
            model_verbosity: None,
        })
    }

    pub(crate) fn from_prepared_with_activity_profile(
        prepared: &CodexPreparedIntegration,
        plan: PreflightPlan,
        profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            operation:
                swallowtail_runtime::PreparedOperationEvidence::from_plan_with_activity_profile(
                    plan,
                    prepared.access_evidence().clone(),
                    profile,
                )?,
            model_verbosity: None,
        })
    }

    /// Returns the qualified installed-executable observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the approved execution environment.
    #[must_use]
    pub const fn environment(&self) -> &swallowtail_runtime::EnvironmentRef {
        &self.environment
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared-operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &swallowtail_runtime::PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    /// Returns the selected adapter-local verbosity when one was prepared.
    #[must_use]
    pub const fn model_verbosity(&self) -> Option<CodexModelVerbosity> {
        self.model_verbosity
    }

    pub(crate) const fn with_model_verbosity(mut self, verbosity: CodexModelVerbosity) -> Self {
        self.model_verbosity = Some(verbosity);
        self
    }
}

pub(crate) fn require_driver(
    prepared: &CodexPreparedIntegration,
    expected: CodexPreparedDriver,
) -> Result<(), PreparationFailure> {
    if prepared.driver() == expected {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.codex.preparation.driver_mismatch",
            "Prepared Codex installation uses a different driver",
        ))
    }
}

pub(crate) fn descriptor(prepared: &CodexPreparedIntegration) -> DriverDescriptor {
    match prepared.driver() {
        CodexPreparedDriver::StructuredExec => codex_exec_descriptor(),
        CodexPreparedDriver::AppServer => codex_app_server_descriptor(),
    }
}

pub(crate) fn instance_with_capabilities(
    prepared: &CodexPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    let base = prepared.instance();
    let instance = swallowtail_runtime::instance_with_capabilities(base, capabilities);
    if let Some(posture) = base.harness_configuration_posture() {
        instance.with_harness_configuration_posture(posture)
    } else {
        instance
    }
}

pub(crate) fn model_route(
    prepared: &CodexPreparedIntegration,
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: swallowtail_core::ModelId,
    capabilities: CapabilityProfile,
) -> ModelRoute {
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        capabilities,
    )
}

pub(crate) fn requirements(
    prepared: &CodexPreparedIntegration,
    shape: OperationShape,
    role: swallowtail_core::DriverRole,
    host_services: impl IntoIterator<Item = HostServiceKind>,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    swallowtail_runtime::base_requirements(
        ExecutionLayer::HarnessInteraction,
        shape,
        role,
        prepared.instance(),
        prepared.access_profile(),
        [CredentialState::Ready],
        capabilities,
    )
    .with_host_services(host_services)
    .with_interface_versions([prepared.observation().version().clone()])
}

pub(crate) fn build_plan(
    prepared: &CodexPreparedIntegration,
    descriptor: &DriverDescriptor,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        descriptor,
        instance,
        route,
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}

pub(crate) fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}
