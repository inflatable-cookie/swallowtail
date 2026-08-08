use super::input::AlibabaSessionHistoryInput;
use super::plan::{
    AlibabaModelStudioPreparedEvidence, build_plan, instance_with_capabilities, model_route,
};
use crate::prepared::failure;
use crate::{AlibabaModelStudioDriver, AlibabaModelStudioPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparationStage,
    PreparedProviderSessionHistoryEvidence, ProviderSessionHistoryAgreement,
    ProviderSessionHistoryCursor, ProviderSessionHistoryDriver, ProviderSessionHistoryPage,
    ProviderSessionHistoryPlan, ProviderSessionHistoryRequest, RequestId, RuntimeFailure,
};

#[derive(Clone, Debug)]
/// Prepared read-only newest-first history pages for one retained conversation.
pub struct AlibabaModelStudioPreparedSessionHistory {
    evidence: AlibabaModelStudioPreparedEvidence,
    history: PreparedProviderSessionHistoryEvidence,
    request: ProviderSessionHistoryRequest,
}

impl AlibabaModelStudioPreparedSessionHistory {
    #[must_use]
    /// Returns portable evidence for the prepared history-page operation.
    pub const fn evidence(&self) -> &PreparedProviderSessionHistoryEvidence {
        &self.history
    }

    #[must_use]
    /// Returns route-specific prepared evidence.
    pub const fn alibaba_evidence(&self) -> &AlibabaModelStudioPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the exact history-page plan.
    pub const fn plan(&self) -> &ProviderSessionHistoryPlan {
        self.history.plan()
    }

    #[must_use]
    /// Returns the initial newest-window request.
    pub const fn request(&self) -> &ProviderSessionHistoryRequest {
        &self.request
    }

    #[must_use]
    /// Returns the public low-level workspace driver.
    pub fn low_level_driver(&self) -> AlibabaModelStudioDriver {
        AlibabaModelStudioDriver::new()
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
                    PreparationStage::Preflight,
                    "swallowtail.alibaba_model_studio.preparation.history_request_invalid",
                    "Alibaba Model Studio history continuation request could not be prepared",
                )
            })
    }
}

impl AlibabaModelStudioPreparedIntegration {
    /// Prepares newest-first history pages for one retained conversation.
    pub fn prepare_session_history(
        &self,
        input: AlibabaSessionHistoryInput,
    ) -> Result<AlibabaModelStudioPreparedSessionHistory, PreparationFailure> {
        let (request_id, history_id, route_id, route_revision, model_id, binding, bounds, deadline) =
            input.into_parts();
        if route_id.as_str() != crate::MODEL_ROUTE_ID || model_id.as_str() != crate::EXACT_MODEL_ID
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.route_rejected",
                "Alibaba Model Studio preparation requires the exact Singapore Qwen route",
            ));
        }
        if Some(&route_id) != binding.model_route_id() || Some(&model_id) != binding.model_id() {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.history_binding_mismatch",
                "Alibaba Model Studio history model does not match its durable session binding",
            ));
        }
        let history = CapabilityRequirement::new(
            Capability::ProviderSessionHistory,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_page_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_page_bytes().get()),
            ],
        );
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let capabilities = CapabilityProfile::new([history.clone(), retention.clone()]);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, route_id, route_revision, model_id, capabilities);
        let requirements = crate::alibaba_model_studio_history_requirements(
            self.instance().execution_host_id().clone(),
        )
        .with_capabilities([history, retention]);
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let history_plan = ProviderSessionHistoryPlan::new(
            plan.clone(),
            ProviderSessionHistoryAgreement::new(history_id, binding, bounds, deadline),
        )
        .map_err(|_| {
            failure(
                PreparationStage::Preflight,
                "swallowtail.alibaba_model_studio.preparation.history_plan_invalid",
                "Alibaba Model Studio history plan could not be prepared",
            )
        })?;
        let request = ProviderSessionHistoryRequest::from_plan(request_id, &history_plan, None)
            .map_err(|_| {
                failure(
                    PreparationStage::Preflight,
                    "swallowtail.alibaba_model_studio.preparation.history_request_invalid",
                    "Alibaba Model Studio history request could not be prepared",
                )
            })?;
        Ok(AlibabaModelStudioPreparedSessionHistory {
            evidence: AlibabaModelStudioPreparedEvidence::from_prepared(
                self,
                plan,
                swallowtail_core::ObservableActivityProfile::not_applicable(),
            )?,
            history: PreparedProviderSessionHistoryEvidence::from_plan(
                history_plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
