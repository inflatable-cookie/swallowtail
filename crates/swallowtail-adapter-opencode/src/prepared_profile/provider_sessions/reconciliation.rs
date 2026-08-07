use super::{provider_session_requirements, require_reconciliation_qualified};
use super::super::input::OpenCodeSessionReconciliationInput;
use super::super::plan::{build_plan, failure, instance_with_capabilities};
use crate::{OpenCodePreparedIntegration, OpenCodePreparedSession};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, DriverRole,
    ModelRoute, OperationShape, ResourceAccess, SessionAccessPolicy,
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
/// Prepared read-only reconciliation of one exact retained provider session.
pub struct OpenCodePreparedSessionReconciliation {
    pub(super) prepared: OpenCodePreparedIntegration,
    pub(super) evidence: PreparedProviderSessionReconciliationEvidence,
    pub(super) request: ProviderSessionReconciliationRequest,
}


impl OpenCodePreparedSessionReconciliation {
    /// Returns the reconciliation preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionReconciliationEvidence {
        &self.evidence
    }

    /// Returns the immutable reconciliation plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        self.evidence.plan()
    }

    /// Returns the exact reconciliation request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionReconciliationRequest {
        &self.request
    }

    /// Observes the retained session within the prepared replay bounds.
    pub fn reconcile(
        &self,
        services: HostServices,
    ) -> BoxFuture<
        'static,
        Result<ProviderSessionReconciliationOutcome, swallowtail_runtime::RuntimeFailure>,
    > {
        let driver = self.prepared.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .reconcile_provider_session(plan, request, services)
                .await
        })
    }

    /// Composes reconciliation with a separately prepared bounded replay load.
    pub fn prepare_settled_session_restoration(
        self,
        session: OpenCodePreparedSession,
        attachment_request_id: RequestId,
    ) -> Result<PreparedSettledSessionRestoration, PreparationFailure> {
        if !settled_session_plans_share_binding(self.plan().preflight(), session.plan())
            || self.prepared.server() != session.evidence().server()
            || self.prepared.access_evidence() != session.evidence().access()
        {
            return Err(failure(
                "swallowtail.opencode.preparation.settled_session_binding_mismatch",
                "OpenCode reconciliation and attachment do not share one prepared route binding",
            ));
        }
        let request = session.load_request(
            attachment_request_id,
            self.plan().agreement().binding().clone(),
        )?;
        Ok(PreparedSettledSessionRestoration::new(
            self,
            OpenCodeSettledSessionLoad { session, request },
        ))
    }
}

impl SettledSessionReconciliationOperation for OpenCodePreparedSessionReconciliation {
    fn reconcile(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        OpenCodePreparedSessionReconciliation::reconcile(&self, services)
    }
}

struct OpenCodeSettledSessionLoad {
    session: OpenCodePreparedSession,
    request: LoadSessionRequest,
}

impl SettledSessionAttachmentOperation for OpenCodeSettledSessionLoad {
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

impl WorkingStateRestorationOperation for OpenCodePreparedSessionReconciliation {
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

impl OpenCodePreparedIntegration {
    /// Prepares the strongest route-supported post-crash restoration operation.
    pub fn prepare_working_state_restoration(
        &self,
        input: OpenCodeSessionReconciliationInput,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        self.prepare_session_reconciliation(input)
            .map(PreparedWorkingStateRestoration::new)
    }
}

impl OpenCodePreparedIntegration {
    /// Validates and prepares read-only retained-session reconciliation.
    pub fn prepare_session_reconciliation(
        &self,
        input: OpenCodeSessionReconciliationInput,
    ) -> Result<OpenCodePreparedSessionReconciliation, PreparationFailure> {
        require_reconciliation_qualified(self)?;
        let (request_id, model, binding, interrupted_turn_id, provider_turn_ref, bounds, deadline) =
            input.into_parts();
        if provider_turn_ref.is_some() {
            return Err(failure(
                "swallowtail.opencode.preparation.session_reconciliation_turn_ref_unsupported",
                "OpenCode session reconciliation is session-scoped and accepts no provider turn reference",
            ));
        }
        let reconciliation = CapabilityRequirement::new(
            Capability::ProviderSessionReconciliation,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_replay_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_replay_bytes().get()),
            ],
        );
        let resource = crate::prepared::working_resource_capability(ResourceAccess::Read);
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let selected =
            CapabilityProfile::new([reconciliation.clone(), resource.clone(), retention.clone()]);
        let instance = instance_with_capabilities(self, selected.clone());
        let (route_id, route_revision, provider_id, model_id, _) = model.into_parts();
        if Some(&route_id) != binding.model_route_id() || Some(&model_id) != binding.model_id() {
            return Err(failure(
                "swallowtail.opencode.preparation.session_reconciliation_binding_mismatch",
                "OpenCode reconciliation model does not match its durable session binding",
            ));
        }
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            selected,
        )
        .with_provider_id(provider_id);
        let requirements = provider_session_requirements(
            self,
            OperationShape::ProviderSessionReconciliation,
            DriverRole::ProviderSessionReconciliation,
            [reconciliation, resource, retention],
            true,
            deadline.is_some(),
            Some(SessionAccessPolicy::ambient_harness(ResourceAccess::Read)),
        );
        let preflight = build_plan(self, &instance, Some(&route), &requirements)?;
        let plan = ProviderSessionReconciliationPlan::new(
            preflight,
            ProviderSessionReconciliationAgreement::new(
                binding,
                interrupted_turn_id,
                None,
                bounds,
                deadline,
            ),
        )
        .map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.session_reconciliation_plan_invalid",
                "OpenCode session reconciliation plan could not be prepared",
            )
        })?;
        let request =
            ProviderSessionReconciliationRequest::from_plan(request_id, &plan).map_err(|_| {
                failure(
                    "swallowtail.opencode.preparation.session_reconciliation_request_invalid",
                    "OpenCode session reconciliation request could not be prepared",
                )
            })?;
        Ok(OpenCodePreparedSessionReconciliation {
            prepared: self.clone(),
            evidence: PreparedProviderSessionReconciliationEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }

}
