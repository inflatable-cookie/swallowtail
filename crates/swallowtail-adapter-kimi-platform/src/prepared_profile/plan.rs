use crate::KimiPlatformPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, ModelRoute, OperationRequirements, OperationShape, PreflightContext,
    PreflightPlan, ProviderId, RuntimeReadiness, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }
}

pub(super) fn instance_with_capabilities(
    prepared: &KimiPlatformPreparedIntegration,
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
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
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
    .with_host_services(descriptor.required_host_services(role))
    .with_capabilities(capabilities)
}

pub(super) fn build_plan(
    prepared: &KimiPlatformPreparedIntegration,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::kimi_platform_direct_descriptor();
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
