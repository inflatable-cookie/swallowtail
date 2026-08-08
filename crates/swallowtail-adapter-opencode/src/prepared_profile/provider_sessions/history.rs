use super::super::input::OpenCodeSessionHistoryInput;
use super::super::plan::{build_plan, failure, instance_with_capabilities};
use super::{provider_session_requirements, require_history_qualified};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, DriverRole,
    ModelRoute, OperationShape, ResourceAccess, SessionAccessPolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionHistoryEvidence,
    ProviderSessionHistoryAgreement, ProviderSessionHistoryCursor, ProviderSessionHistoryDriver,
    ProviderSessionHistoryPage, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
    RequestId, RuntimeFailure,
};

#[derive(Clone, Debug)]
/// Prepared read-only newest-first history pages for one retained OpenCode session.
pub struct OpenCodePreparedSessionHistory {
    prepared: OpenCodePreparedIntegration,
    evidence: PreparedProviderSessionHistoryEvidence,
    request: ProviderSessionHistoryRequest,
}

impl OpenCodePreparedSessionHistory {
    /// Returns portable evidence for the prepared history-page operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionHistoryEvidence {
        &self.evidence
    }

    /// Returns the exact history-page plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionHistoryPlan {
        self.evidence.plan()
    }

    /// Returns the initial newest-window request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionHistoryRequest {
        &self.request
    }

    /// Creates the low-level HTTP driver bound to this history plan.
    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        self.prepared.low_level_driver()
    }

    /// Reads the newest bound history window for the prepared plan.
    pub fn page_history(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionHistoryPage, RuntimeFailure>> {
        self.page(self.request.clone(), services)
    }

    /// Reads one explicitly supplied history page.
    pub fn page(
        &self,
        request: ProviderSessionHistoryRequest,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionHistoryPage, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        Box::pin(async move {
            driver
                .page_provider_session_history(plan, request, services)
                .await
        })
    }

    /// Builds a continuation request from an opaque older-page cursor.
    pub fn older_page_request(
        &self,
        request_id: RequestId,
        older_cursor: ProviderSessionHistoryCursor,
    ) -> Result<ProviderSessionHistoryRequest, PreparationFailure> {
        ProviderSessionHistoryRequest::from_plan(request_id, self.plan(), Some(older_cursor))
            .map_err(|_| {
                failure(
                    "swallowtail.opencode.preparation.session_history_request_invalid",
                    "OpenCode session history continuation request could not be prepared",
                )
            })
    }
}

impl OpenCodePreparedIntegration {
    /// Prepares newest-first history pages for one retained OpenCode session.
    pub fn prepare_session_history(
        &self,
        input: OpenCodeSessionHistoryInput,
    ) -> Result<OpenCodePreparedSessionHistory, PreparationFailure> {
        require_history_qualified(self)?;
        let (request_id, history_id, model, binding, bounds, deadline) = input.into_parts();
        let (route_id, route_revision, provider_id, model_id, _) = model.into_parts();
        if Some(&route_id) != binding.model_route_id() || Some(&model_id) != binding.model_id() {
            return Err(failure(
                "swallowtail.opencode.preparation.session_history_binding_mismatch",
                "OpenCode history model does not match its durable session binding",
            ));
        }
        let history = CapabilityRequirement::new(
            Capability::ProviderSessionHistory,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_page_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_page_bytes().get()),
            ],
        );
        let resource = crate::prepared::working_resource_capability(ResourceAccess::Read);
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let selected =
            CapabilityProfile::new([history.clone(), resource.clone(), retention.clone()]);
        let instance = instance_with_capabilities(self, selected.clone());
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
            OperationShape::ProviderSessionHistory,
            DriverRole::ProviderSessionHistory,
            [history, resource, retention],
            true,
            deadline.is_some(),
            Some(SessionAccessPolicy::ambient_harness(ResourceAccess::Read)),
        );
        let preflight = build_plan(self, &instance, Some(&route), &requirements)?;
        let plan = ProviderSessionHistoryPlan::new(
            preflight,
            ProviderSessionHistoryAgreement::new(history_id, binding, bounds, deadline),
        )
        .map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.session_history_plan_invalid",
                "OpenCode session history plan could not be prepared",
            )
        })?;
        let request =
            ProviderSessionHistoryRequest::from_plan(request_id, &plan, None).map_err(|_| {
                failure(
                    "swallowtail.opencode.preparation.session_history_request_invalid",
                    "OpenCode session history request could not be prepared",
                )
            })?;
        Ok(OpenCodePreparedSessionHistory {
            prepared: self.clone(),
            evidence: PreparedProviderSessionHistoryEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
