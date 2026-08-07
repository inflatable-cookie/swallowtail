use super::failure::preparation_failure;
use super::{KimiLocalServerPreparedReconciliation, KimiLocalServerReconciliationInput};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, InstanceOwnership, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, ResourceAccess,
    ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
    preflight,
};
use swallowtail_runtime::{
    PreparationFailure, PreparationStage, PreparedProviderSessionReconciliationEvidence,
    PreparedWorkingStateRestoration, ProviderOperationCheckpoint,
    ProviderSessionReconciliationAgreement, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest,
};

impl crate::KimiLocalServerPreparedIntegration {
    /// Prepares the strongest admitted working-state restoration for this server.
    pub fn prepare_working_state_restoration(
        &self,
        input: KimiLocalServerReconciliationInput,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        self.prepare_session_reconciliation(input)
            .map(PreparedWorkingStateRestoration::new)
    }
}

impl crate::KimiLocalServerPreparedIntegration {
    /// Prepares read-only reconciliation for one interrupted retained session.
    pub fn prepare_session_reconciliation(
        &self,
        input: KimiLocalServerReconciliationInput,
    ) -> Result<KimiLocalServerPreparedReconciliation, PreparationFailure> {
        if self.instance().ownership() != InstanceOwnership::ExternalAttached
            || !self.server().is_qualified()
        {
            return Err(preparation_failure(
                "swallowtail.kimi.local_server.preparation.reconciliation_topology_unsupported",
                "Kimi local-server reconciliation requires a qualified attached server",
            ));
        }
        let KimiLocalServerReconciliationInput {
            request_id,
            model,
            binding,
            checkpoint,
            bounds,
            deadline,
        } = input;
        let reconciliation = CapabilityRequirement::new(
            Capability::ProviderSessionReconciliation,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_replay_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_replay_bytes().get()),
            ],
        );
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let resource = CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        );
        let capabilities =
            CapabilityProfile::new([reconciliation.clone(), retention.clone(), resource.clone()]);
        let instance =
            super::super::prepared::instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = model.into_parts();
        if Some(&route_id) != binding.model_route_id() || Some(&model_id) != binding.model_id() {
            return Err(preparation_failure(
                "swallowtail.kimi.local_server.preparation.reconciliation_binding_mismatch",
                "Kimi local-server reconciliation model does not match its durable binding",
            ));
        }
        let route = ModelRoute::new(
            route_id,
            route_revision,
            instance.id().clone(),
            model_id,
            capabilities,
        );
        let access_policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionReconciliation,
            DriverRole::ProviderSessionReconciliation,
            self.instance().execution_host_id().clone(),
            AccessRequirement::new(self.access_profile().id().clone())
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([self.access_profile().support_authority()]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(
            crate::kimi_local_server_descriptor()
                .required_host_services(DriverRole::ProviderSessionReconciliation),
        )
        .with_capabilities([reconciliation, retention, resource])
        .with_interface_versions([self.server().binding().clone()])
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .with_session_access_policy(access_policy)
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        )
        .require_model_route();
        let descriptor = crate::kimi_local_server_descriptor();
        let preflight = preflight(
            &PreflightContext::new(
                &descriptor,
                &instance,
                self.access_profile(),
                self.access_evidence().status(),
                self.available_host_services(),
            )
            .with_model_route(&route),
            &requirements,
        )
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let checkpoint =
            ProviderOperationCheckpoint::restore_persisted(&checkpoint, &preflight, &binding)
                .map_err(|error| {
                    PreparationFailure::new(
                        PreparationStage::Preflight,
                        Diagnostic::new(error.diagnostic().clone()),
                    )
                })?;
        let agreement = ProviderSessionReconciliationAgreement::new(
            binding,
            checkpoint.runtime_turn_id().clone(),
            Some(checkpoint.provider_turn_ref().clone()),
            bounds,
            deadline,
        )
        .with_checkpoint(checkpoint)
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let plan =
            ProviderSessionReconciliationPlan::new(preflight, agreement).map_err(|error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(error.diagnostic().clone()),
                )
            })?;
        let request = ProviderSessionReconciliationRequest::from_plan(request_id, &plan).map_err(
            |error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(error.diagnostic().clone()),
                )
            },
        )?;
        Ok(KimiLocalServerPreparedReconciliation {
            evidence: PreparedProviderSessionReconciliationEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
