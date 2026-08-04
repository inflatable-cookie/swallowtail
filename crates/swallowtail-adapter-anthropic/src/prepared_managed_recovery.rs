use crate::prepared_managed::failure;
use crate::{
    AnthropicManagedAgentDriver, AnthropicManagedModelSelection,
    AnthropicManagedPreparedIntegration,
};
use std::num::NonZeroU64;
use swallowtail_core::{
    AccessProfileId, AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, Diagnostic, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionLayer, InstanceOwnership, ModelRoute,
    OperationRequirements, OperationShape, OwnedRemoteResourceKind, PreflightContext, ProviderId,
    RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, PersistedProviderRecoveredResourceCleanupBinding,
    PersistedProviderRunCheckpoint, PreparationFailure, PreparationStage,
    PreparedProviderRecoveredResourceCleanupEvidence, PreparedProviderRunReconciliationEvidence,
    ProviderRecoveredResourceCleanupAgreement, ProviderRecoveredResourceCleanupBinding,
    ProviderRecoveredResourceCleanupDriver, ProviderRecoveredResourceCleanupOutcome,
    ProviderRecoveredResourceCleanupPlan, ProviderRecoveredResourceCleanupRequest,
    ProviderRunCheckpoint, ProviderRunReconciliationAgreement, ProviderRunReconciliationDriver,
    ProviderRunReconciliationOutcome, ProviderRunReconciliationPlan,
    ProviderRunReconciliationRequest, RequestId, RuntimeFailure,
};

include!("prepared_managed_recovery/types.rs");

impl AnthropicManagedPreparedIntegration {
    pub fn prepare_run_reconciliation(
        &self,
        input: AnthropicManagedRunReconciliationInput,
    ) -> Result<AnthropicPreparedManagedRunReconciliation, PreparationFailure> {
        let capability = CapabilityRequirement::new(
            Capability::ProviderRunReconciliation,
            [CapabilityConstraint::RecoveredOutputMaximumBytes(
                input.maximum_output_bytes.get(),
            )],
        );
        let preflight = recovery_plan(
            self,
            input.model,
            capability.clone(),
            DriverRole::ProviderRunReconciliation,
            OperationShape::ProviderRunReconciliation,
        )?;
        let checkpoint = ProviderRunCheckpoint::restore_persisted(&input.checkpoint, &preflight)
            .map_err(|_| rejected("reconciliation checkpoint"))?;
        crate::managed_recovery::from_checkpoint(&checkpoint)
            .map_err(|_| rejected("reconciliation checkpoint"))?;
        let plan = ProviderRunReconciliationPlan::new(
            preflight,
            ProviderRunReconciliationAgreement::new(
                checkpoint,
                input.maximum_output_bytes,
                input.deadline,
            ),
        )
        .map_err(runtime_preparation)?;
        let request = ProviderRunReconciliationRequest::from_plan(input.request_id, &plan)
            .map_err(runtime_preparation)?;
        let evidence = PreparedProviderRunReconciliationEvidence::from_plan(
            plan,
            self.access_evidence().clone(),
        )?;
        Ok(AnthropicPreparedManagedRunReconciliation { evidence, request })
    }

    pub fn prepare_recovered_cleanup(
        &self,
        input: AnthropicManagedRecoveredCleanupInput,
    ) -> Result<AnthropicPreparedManagedRecoveredCleanup, PreparationFailure> {
        let capability = CapabilityRequirement::new(
            Capability::ProviderRecoveredResourceCleanup,
            [
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Environment),
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
            ],
        );
        let preflight = recovery_plan(
            self,
            input.model,
            capability.clone(),
            DriverRole::ProviderRecoveredResourceCleanup,
            OperationShape::ProviderRecoveredResourceCleanup,
        )?;
        let binding =
            ProviderRecoveredResourceCleanupBinding::restore_persisted(&input.binding, &preflight)
                .map_err(|_| rejected("cleanup binding"))?;
        crate::managed_recovery::from_cleanup_binding(&binding)
            .map_err(|_| rejected("cleanup binding"))?;
        let plan = ProviderRecoveredResourceCleanupPlan::new(
            preflight,
            ProviderRecoveredResourceCleanupAgreement::new(binding, input.deadline),
        )
        .map_err(runtime_preparation)?;
        let request = ProviderRecoveredResourceCleanupRequest::from_plan(input.request_id, &plan)
            .map_err(runtime_preparation)?;
        let evidence = PreparedProviderRecoveredResourceCleanupEvidence::from_plan(
            plan,
            self.access_evidence().clone(),
        )?;
        Ok(AnthropicPreparedManagedRecoveredCleanup { evidence, request })
    }
}

fn recovery_plan(
    prepared: &AnthropicManagedPreparedIntegration,
    model: AnthropicManagedModelSelection,
    capability: CapabilityRequirement,
    role: DriverRole,
    shape: OperationShape,
) -> Result<swallowtail_core::PreflightPlan, PreparationFailure> {
    let capabilities = CapabilityProfile::new([capability.clone()]);
    let instance = instance_with_capabilities(prepared, capabilities.clone());
    let (route_id, route_revision, model_id) = model.into_parts();
    let route = ModelRoute::new(
        route_id,
        route_revision,
        instance.id().clone(),
        model_id,
        capabilities,
    )
    .with_provider_id(ProviderId::new("anthropic").expect("provider id is valid"));
    let descriptor = crate::anthropic_managed_agent_descriptor();
    let host_services = descriptor.required_host_services(role).collect::<Vec<_>>();
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        shape,
        role,
        instance.execution_host_id().clone(),
        AccessRequirement::new(
            AccessProfileId::new(crate::ANTHROPIC_MANAGED_ACCESS_PROFILE_ID)
                .expect("access profile id is valid"),
        )
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(host_services)
    .with_capabilities([capability])
    .with_interface_versions([crate::anthropic_managed_facade_binding()])
    .require_model_route();
    preflight(
        &PreflightContext::new(
            &descriptor,
            &instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        )
        .with_model_route(&route),
        &requirements,
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

fn instance_with_capabilities(
    prepared: &AnthropicManagedPreparedIntegration,
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
    if let Some(agent) = base.provider_agent() {
        instance = instance.with_provider_agent(agent.clone());
    }
    instance
}

fn rejected(subject: &'static str) -> PreparationFailure {
    failure(
        PreparationStage::Preflight,
        "swallowtail.anthropic.managed.preparation.recovery_binding_rejected",
        match subject {
            "reconciliation checkpoint" => {
                "Anthropic Managed Agents reconciliation checkpoint was rejected"
            }
            _ => "Anthropic Managed Agents recovered cleanup binding was rejected",
        },
    )
}

fn runtime_preparation(error: RuntimeFailure) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(error.diagnostic().clone()),
    )
}
