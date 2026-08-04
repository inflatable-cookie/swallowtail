use crate::prepared::failure;
use crate::prepared_profile::{instance_with_capabilities, model_route};
use crate::{
    OpenAiBackgroundDriver, OpenAiBackgroundModelSelection, OpenAiBackgroundPreparedIntegration,
};
use std::num::NonZeroU64;
use swallowtail_core::{
    AccessProfileId, AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, CredentialState, Diagnostic, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, InstanceOwnership, OperationRequirements, OperationShape,
    PreflightContext, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, PersistedProviderRunCheckpoint, PreparationFailure,
    PreparationStage, PreparedProviderRunReconciliationEvidence, PreparedWorkingStateRestoration,
    ProviderRunCheckpoint, ProviderRunReconciliationAgreement, ProviderRunReconciliationDriver,
    ProviderRunReconciliationOutcome, ProviderRunReconciliationPlan,
    ProviderRunReconciliationRequest, RequestId, RuntimeFailure, WorkingStateRestorationMethod,
    WorkingStateRestorationOperation, WorkingStateRestorationOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiBackgroundReconciliationInput {
    request_id: RequestId,
    model: OpenAiBackgroundModelSelection,
    checkpoint: PersistedProviderRunCheckpoint,
    maximum_output_bytes: NonZeroU64,
    deadline: Option<Deadline>,
}

impl OpenAiBackgroundReconciliationInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: OpenAiBackgroundModelSelection,
        checkpoint: PersistedProviderRunCheckpoint,
        maximum_output_bytes: NonZeroU64,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            checkpoint,
            maximum_output_bytes,
            deadline,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OpenAiPreparedBackgroundReconciliation {
    evidence: PreparedProviderRunReconciliationEvidence,
    request: ProviderRunReconciliationRequest,
}

impl OpenAiPreparedBackgroundReconciliation {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderRunReconciliationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderRunReconciliationPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ProviderRunReconciliationRequest {
        &self.request
    }

    pub fn reconcile(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderRunReconciliationOutcome, RuntimeFailure>> {
        let driver = OpenAiBackgroundDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.reconcile_provider_run(plan, request, services).await })
    }
}

impl WorkingStateRestorationOperation for OpenAiPreparedBackgroundReconciliation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderRunReconciliation
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let future = self.reconcile(services);
        Box::pin(async move {
            future
                .await
                .map(WorkingStateRestorationOutcome::RunReconciled)
        })
    }
}

impl OpenAiBackgroundPreparedIntegration {
    pub fn prepare_working_state_restoration(
        &self,
        input: OpenAiBackgroundReconciliationInput,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        self.prepare_run_reconciliation(input)
            .map(PreparedWorkingStateRestoration::new)
    }
}

impl OpenAiBackgroundPreparedIntegration {
    pub fn prepare_run_reconciliation(
        &self,
        input: OpenAiBackgroundReconciliationInput,
    ) -> Result<OpenAiPreparedBackgroundReconciliation, PreparationFailure> {
        let capability = CapabilityRequirement::new(
            Capability::ProviderRunReconciliation,
            [CapabilityConstraint::RecoveredOutputMaximumBytes(
                input.maximum_output_bytes.get(),
            )],
        );
        let capabilities = CapabilityProfile::new([capability.clone()]);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, input.model, capabilities);
        if route.id().as_str() != crate::OPENAI_BACKGROUND_MODEL_ROUTE_ID
            || route.model_id().as_str() != crate::OPENAI_BACKGROUND_MODEL_ID
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.reconciliation_route_rejected",
                "OpenAI background reconciliation requires the exact GPT-5.6 route",
            ));
        }
        let descriptor = crate::openai_background_descriptor();
        let host_services = descriptor
            .required_host_services(DriverRole::ProviderRunReconciliation)
            .collect::<Vec<_>>();
        let requirements = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::ProviderRunReconciliation,
            DriverRole::ProviderRunReconciliation,
            self.instance().execution_host_id().clone(),
            AccessRequirement::new(
                AccessProfileId::new(crate::OPENAI_BACKGROUND_ACCESS_PROFILE_ID)
                    .expect("static access profile id is valid"),
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
        .with_interface_versions([crate::openai_background_facade_binding()])
        .require_model_route();
        let plan = preflight(
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
        let checkpoint = ProviderRunCheckpoint::restore_persisted(&input.checkpoint, &plan)
            .map_err(|_| {
                failure(
                    PreparationStage::Preflight,
                    "swallowtail.openai.preparation.reconciliation_checkpoint_rejected",
                    "OpenAI response checkpoint does not match the prepared route",
                )
            })?;
        crate::checkpoint::decode_cursor(&checkpoint).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let agreement = ProviderRunReconciliationAgreement::new(
            checkpoint,
            input.maximum_output_bytes,
            input.deadline,
        );
        let plan = ProviderRunReconciliationPlan::new(plan, agreement).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let request = ProviderRunReconciliationRequest::from_plan(input.request_id, &plan)
            .map_err(|error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    Diagnostic::new(error.diagnostic().clone()),
                )
            })?;
        let evidence = PreparedProviderRunReconciliationEvidence::from_plan(
            plan,
            self.access_evidence().clone(),
        )?;
        Ok(OpenAiPreparedBackgroundReconciliation { evidence, request })
    }
}
