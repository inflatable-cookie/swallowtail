use crate::KimiPlatformPreparedIntegration;
use swallowtail_core::{
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, CredentialState, DriverRole,
    ExecutionLayer, ModelRoute, OperationRequirements, OperationShape, PreflightPlan, ProviderId,
};
use swallowtail_runtime::{PreparationFailure, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inspectable prepared evidence for one Kimi Platform operation.
pub struct KimiPlatformPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl KimiPlatformPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &KimiPlatformPreparedIntegration,
        plan: PreflightPlan,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan(
                plan,
                prepared.access_evidence().clone(),
            )?,
        })
    }

    pub(super) fn from_prepared_with_activity(
        prepared: &KimiPlatformPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    #[must_use]
    /// Returns the access evidence and provenance bound to the operation.
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    /// Returns the shared prepared-operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the route's observable-activity contract.
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    /// Returns the immutable preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }
}

pub(super) fn instance_with_capabilities(
    prepared: &KimiPlatformPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
}

pub(super) fn model_route(
    prepared: &KimiPlatformPreparedIntegration,
    model: super::KimiPlatformModelSelection,
    capabilities: CapabilityProfile,
) -> ModelRoute {
    let (route_id, route_revision, model_id) = model.into_parts();
    ModelRoute::new(
        route_id,
        route_revision,
        prepared.instance().id().clone(),
        model_id,
        capabilities,
    )
    .with_provider_id(
        ProviderId::new(crate::KIMI_PLATFORM_PROVIDER_ID)
            .expect("static Kimi provider id is valid"),
    )
}

pub(super) fn requirements(
    prepared: &KimiPlatformPreparedIntegration,
    role: DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    let descriptor = crate::kimi_platform_direct_descriptor();
    swallowtail_runtime::base_requirements(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        role,
        prepared.instance(),
        prepared.access_profile(),
        [CredentialState::Ready],
        capabilities,
    )
    .with_host_services(descriptor.required_host_services(role))
}

pub(super) fn build_plan(
    prepared: &KimiPlatformPreparedIntegration,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        &crate::kimi_platform_direct_descriptor(),
        instance,
        route,
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}
