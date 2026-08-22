use super::DriverFixture;
use swallowtail_adapter_xai::xai_websocket_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionLayer,
    InstanceOwnership, InstancePolicyId, InstanceRevision, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ProtocolFacadeId, ProviderId, RuntimeReadiness, SessionAccessPolicy,
    SessionProviderStatePolicy, SupportAuthority, preflight,
};

impl DriverFixture {
    pub fn plan(&self) -> PreflightPlan {
        self.plan_for(swallowtail_core::DriverRole::InteractiveSession)
    }

    pub fn run_plan(&self) -> PreflightPlan {
        self.plan_for(swallowtail_core::DriverRole::StructuredRun)
    }

    pub fn plan_with_output_token_maximum(
        &self,
        role: swallowtail_core::DriverRole,
        maximum: u64,
    ) -> PreflightPlan {
        self.plan_for_with_extra(
            role,
            Some(CapabilityRequirement::new(
                Capability::OutputTokenLimit,
                [swallowtail_core::CapabilityConstraint::OutputTokenMaximum(
                    maximum,
                )],
            )),
        )
    }

    fn plan_for(&self, role: swallowtail_core::DriverRole) -> PreflightPlan {
        self.plan_for_with_extra(role, None)
    }

    fn plan_for_with_extra(
        &self,
        role: swallowtail_core::DriverRole,
        extra: Option<CapabilityRequirement>,
    ) -> PreflightPlan {
        let descriptor = xai_websocket_descriptor();
        let access_id = AccessProfileId::new("access.xai.public").expect("access id is valid");
        let mut all_requirements = capability_requirements();
        all_requirements.extend(run_capability_requirements());
        if let Some(extra) = extra.as_ref() {
            all_requirements.push(extra.clone());
        }
        let capabilities = CapabilityProfile::new(all_requirements);
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("xai.public.websocket").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("xai-responses-websocket-2026-04-23").expect("facade is valid"),
            InstancePolicyId::new("public-api-key-resource-free").expect("policy is valid"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("xai-grok-fixture").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            instance.id().clone(),
            ModelId::new("grok-fixture-exact").expect("model id is valid"),
            capabilities,
        )
        .with_provider_id(ProviderId::new("xai").expect("provider id is valid"));
        let access = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            self.audience.clone(),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            access_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let host_services: Vec<_> = descriptor.required_host_services(role).collect();
        let access_requirement = AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]);
        let operation = match role {
            swallowtail_core::DriverRole::StructuredRun => {
                let mut requirements = run_capability_requirements();
                if let Some(extra) = extra.clone() {
                    requirements.push(extra);
                }
                OperationRequirements::new(
                    ExecutionLayer::DirectModelInference,
                    OperationShape::StructuredRun,
                    role,
                    self.host_id.clone(),
                    access_requirement,
                )
                .with_ownership_modes([InstanceOwnership::ExternalAttached])
                .with_host_services(host_services.clone())
                .with_capabilities(requirements)
                .require_model_route()
            }
            _ => {
                let mut requirements = capability_requirements();
                if let Some(extra) = extra {
                    requirements.push(extra);
                }
                OperationRequirements::new(
                    ExecutionLayer::DirectModelInference,
                    OperationShape::InteractiveSession,
                    role,
                    self.host_id.clone(),
                    access_requirement,
                )
                .with_ownership_modes([InstanceOwnership::ExternalAttached])
                .with_host_services(host_services.clone())
                .with_capabilities(requirements)
                .with_session_access_policy(SessionAccessPolicy::resource_free())
                .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
                .require_model_route()
            }
        };
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
                .with_model_route(&route),
            &operation,
        )
        .expect("xAI session preflight succeeds")
    }
}

fn capability_requirements() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [swallowtail_core::CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::BilledCostReporting, []),
    ]
}

fn run_capability_requirements() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [swallowtail_core::CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::BilledCostReporting, []),
    ]
}
