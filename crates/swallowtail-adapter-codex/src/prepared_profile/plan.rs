use crate::{
    CodexPreparedDriver, CodexPreparedIntegration, codex_app_server_descriptor,
    codex_exec_descriptor,
};
use swallowtail_core::{
    AccessRequirement, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, DriverDescriptor, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HostServiceKind, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, RuntimeReadiness,
    SafeDiagnostic, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    operation: swallowtail_runtime::PreparedOperationEvidence,
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
        })
    }

    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    pub const fn environment(&self) -> &swallowtail_runtime::EnvironmentRef {
        &self.environment
    }

    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    pub const fn operation(&self) -> &swallowtail_runtime::PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
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
    let mut instance = ConfiguredInstance::new(
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
    .with_interface_versions(base.interface_versions().cloned());
    if let Some(posture) = base.harness_configuration_posture() {
        instance = instance.with_harness_configuration_posture(posture);
    }
    instance
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
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        shape,
        role,
        prepared.instance().execution_host_id().clone(),
        access_requirement(prepared),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(host_services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
}

pub(crate) fn build_plan(
    prepared: &CodexPreparedIntegration,
    descriptor: &DriverDescriptor,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let context = PreflightContext::new(
        descriptor,
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

fn access_requirement(prepared: &CodexPreparedIntegration) -> AccessRequirement {
    AccessRequirement::new(prepared.access_profile().id().clone())
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([prepared.access_profile().support_authority()])
}

pub(crate) fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}
