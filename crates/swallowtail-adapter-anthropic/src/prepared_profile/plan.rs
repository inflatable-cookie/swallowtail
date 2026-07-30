use crate::AnthropicPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HostServiceKind, ModelRoute, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, ProviderId, RuntimeReadiness, SessionAccessPolicy,
    SessionProviderStatePolicy, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage, PreparedOperationEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicPreparedEvidence {
    operation: PreparedOperationEvidence,
}

impl AnthropicPreparedEvidence {
    pub(super) fn from_prepared(
        prepared: &AnthropicPreparedIntegration,
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
        prepared: &AnthropicPreparedIntegration,
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
    prepared: &AnthropicPreparedIntegration,
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
    prepared: &AnthropicPreparedIntegration,
    model: super::AnthropicModelSelection,
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
    .with_provider_id(ProviderId::new("anthropic").expect("static provider id is valid"))
}

pub(super) fn requirements(
    prepared: &AnthropicPreparedIntegration,
    role: DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    extra_host_services: impl IntoIterator<Item = HostServiceKind>,
) -> OperationRequirements {
    let descriptor = crate::anthropic_direct_descriptor();
    let mut host_services: Vec<_> = descriptor.required_host_services(role).collect();
    host_services.extend(extra_host_services);
    let mut requirements = OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        if role == DriverRole::InteractiveSession {
            OperationShape::InteractiveSession
        } else {
            OperationShape::StructuredRun
        },
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
    .with_capabilities(capabilities);
    requirements =
        requirements.with_interface_versions([crate::anthropic_messages_facade_binding()]);
    if role == DriverRole::InteractiveSession {
        requirements = requirements
            .with_session_access_policy(SessionAccessPolicy::resource_free())
            .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
    }
    requirements
}

pub(super) fn build_plan(
    prepared: &AnthropicPreparedIntegration,
    instance: &ConfiguredInstance,
    route: Option<&ModelRoute>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::anthropic_direct_descriptor();
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
