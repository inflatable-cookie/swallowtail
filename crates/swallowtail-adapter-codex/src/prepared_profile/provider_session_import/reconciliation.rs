use super::super::CodexPreparedSession;
use super::super::input::CodexSessionReconciliationInput;
use super::super::plan::{
    CodexPreparedEvidence, build_plan, descriptor, failure, instance_with_capabilities,
    model_route, require_driver, requirements,
};
use super::{read_only_working_resource_capability, require_catalogue_version};
use crate::{CodexAppServerDriver, CodexPreparedDriver, CodexPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, DriverRole,
    HarnessConfigurationPosture, HostServiceKind, OperationShape, ResourceAccess,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, LoadSessionRequest, PreparationFailure,
    PreparedProviderSessionReconciliationEvidence, PreparedSettledSessionRestoration,
    PreparedWorkingStateRestoration, ProviderSessionReconciliationAgreement,
    ProviderSessionReconciliationDriver, ProviderSessionReconciliationOutcome,
    ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest, RequestId,
    RuntimeFailure, SettledSessionAttachment, SettledSessionAttachmentKind,
    SettledSessionAttachmentOperation, SettledSessionReconciliationOperation,
    WorkingStateRestorationMethod, WorkingStateRestorationOperation,
    WorkingStateRestorationOutcome, settled_session_plans_share_binding,
};

#[derive(Clone, Debug)]
/// Prepared read-only reconciliation of one interrupted Codex thread turn.
pub struct CodexPreparedSessionReconciliation {
    codex: CodexPreparedEvidence,
    evidence: PreparedProviderSessionReconciliationEvidence,
    request: ProviderSessionReconciliationRequest,
}

impl CodexPreparedSessionReconciliation {
    /// Returns portable evidence for the prepared reconciliation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionReconciliationEvidence {
        &self.evidence
    }

    /// Returns the underlying prepared Codex evidence.
    #[must_use]
    pub const fn codex_evidence(&self) -> &CodexPreparedEvidence {
        &self.codex
    }

    /// Returns the exact reconciliation plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        self.evidence.plan()
    }

    /// Returns the bound reconciliation request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionReconciliationRequest {
        &self.request
    }

    /// Observes retained provider truth for the interrupted turn.
    pub fn reconcile(
        &self,
        services: HostServices,
    ) -> BoxFuture<
        'static,
        Result<ProviderSessionReconciliationOutcome, swallowtail_runtime::RuntimeFailure>,
    > {
        let driver = CodexAppServerDriver::new(self.codex.environment().clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .reconcile_provider_session(plan, request, services)
                .await
        })
    }

    /// Composes reconciliation with a separately prepared, binding-equal load.
    pub fn prepare_settled_session_restoration(
        self,
        session: CodexPreparedSession,
        attachment_request_id: RequestId,
    ) -> Result<PreparedSettledSessionRestoration, PreparationFailure> {
        if !settled_session_plans_share_binding(self.plan().preflight(), session.plan())
            || self.codex.observation() != session.evidence().observation()
            || self.codex.environment() != session.evidence().environment()
            || self.codex.access() != session.evidence().access()
        {
            return Err(failure(
                "swallowtail.codex.preparation.settled_session_binding_mismatch",
                "Codex reconciliation and attachment do not share one prepared route binding",
            ));
        }
        let request = session.load_request(
            attachment_request_id,
            self.plan().agreement().binding().clone(),
        )?;
        Ok(PreparedSettledSessionRestoration::new(
            self,
            CodexSettledSessionLoad { session, request },
        ))
    }
}

impl SettledSessionReconciliationOperation for CodexPreparedSessionReconciliation {
    fn reconcile(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        CodexPreparedSessionReconciliation::reconcile(&self, services)
    }
}

struct CodexSettledSessionLoad {
    session: CodexPreparedSession,
    request: LoadSessionRequest,
}

impl SettledSessionAttachmentOperation for CodexSettledSessionLoad {
    fn kind(&self) -> SettledSessionAttachmentKind {
        SettledSessionAttachmentKind::Load
    }

    fn attach(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<SettledSessionAttachment, RuntimeFailure>> {
        let future = self.session.load_prepared_session(self.request, services);
        Box::pin(async move { future.await.map(SettledSessionAttachment::Loaded) })
    }
}

impl WorkingStateRestorationOperation for CodexPreparedSessionReconciliation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderSessionReconciliation
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let future = self.reconcile(services);
        Box::pin(async move {
            future
                .await
                .map(WorkingStateRestorationOutcome::SessionReconciled)
        })
    }
}

impl CodexPreparedIntegration {
    /// Prepares the strongest admitted working-state restoration for Codex.
    pub fn prepare_working_state_restoration(
        &self,
        input: CodexSessionReconciliationInput,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        self.prepare_session_reconciliation(input)
            .map(PreparedWorkingStateRestoration::new)
    }
}

impl CodexPreparedIntegration {
    /// Prepares read-only reconciliation for one interrupted retained thread.
    pub fn prepare_session_reconciliation(
        &self,
        input: CodexSessionReconciliationInput,
    ) -> Result<CodexPreparedSessionReconciliation, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        require_catalogue_version(self)?;
        let (request_id, model, binding, interrupted_turn_id, provider_turn_ref, bounds, deadline) =
            input.into_parts();
        if Some(model.route_id()) != binding.model_route_id()
            || Some(model.model_id()) != binding.model_id()
        {
            return Err(failure(
                "swallowtail.codex.preparation.thread_reconciliation_binding_mismatch",
                "Codex reconciliation model does not match its durable session binding",
            ));
        }
        let reconciliation = CapabilityRequirement::new(
            Capability::ProviderSessionReconciliation,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_replay_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_replay_bytes().get()),
            ],
        );
        let resource = read_only_working_resource_capability();
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let capability_requirements =
            vec![reconciliation.clone(), resource.clone(), retention.clone()];
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(
            self,
            model.route_id().clone(),
            model.route_revision().clone(),
            model.model_id().clone(),
            capabilities,
        );
        let mut host_services = vec![
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ];
        if deadline.is_some() {
            host_services.push(HostServiceKind::Time);
        }
        let requirements = requirements(
            self,
            OperationShape::ProviderSessionReconciliation,
            DriverRole::ProviderSessionReconciliation,
            host_services,
            capability_requirements,
        )
        .with_session_access_policy(swallowtail_core::SessionAccessPolicy::ambient_harness(
            ResourceAccess::Read,
        ))
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        )
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let preflight = build_plan(
            self,
            &descriptor(self),
            &instance,
            Some(&route),
            &requirements,
        )?;
        let plan = ProviderSessionReconciliationPlan::new(
            preflight.clone(),
            ProviderSessionReconciliationAgreement::new(
                binding,
                interrupted_turn_id,
                provider_turn_ref,
                bounds,
                deadline,
            ),
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.preparation.thread_reconciliation_plan_invalid",
                "Codex thread reconciliation plan could not be prepared",
            )
        })?;
        let request =
            ProviderSessionReconciliationRequest::from_plan(request_id, &plan).map_err(|_| {
                failure(
                    "swallowtail.codex.preparation.thread_reconciliation_request_invalid",
                    "Codex thread reconciliation request could not be prepared",
                )
            })?;
        Ok(CodexPreparedSessionReconciliation {
            codex: CodexPreparedEvidence::from_prepared(self, preflight)?,
            evidence: PreparedProviderSessionReconciliationEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
