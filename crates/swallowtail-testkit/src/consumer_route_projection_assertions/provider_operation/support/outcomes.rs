use swallowtail_core::{
    ProviderSessionCatalogueBounds, ResourceAccess, SessionAccessPolicy, SessionRef,
};
use swallowtail_runtime::{
    CleanupOutcome, ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord,
    PreparedOperationEvidence, ProviderSessionCatalogueAgreement, ProviderSessionCatalogueId,
    ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest,
    ProviderSessionCatalogueScope, ProviderSessionHistoryAgreement, ProviderSessionHistoryId,
    ProviderSessionHistoryPage, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
    ProviderSessionHistoryTotal, SessionResumeBinding, page_provider_session_history_window,
};

use super::ProviderOperationFixture;
use super::plan::{history_bounds, nonzero_u32, resource};

impl ProviderOperationFixture {
    pub(crate) fn prepared(&self) -> PreparedOperationEvidence {
        PreparedOperationEvidence::from_plan(self.plan(), self.access_evidence.clone())
            .expect("prepared operation is valid")
    }

    pub(crate) fn record(
        &self,
        evidence: &PreparedOperationEvidence,
    ) -> ConfiguredProviderInstanceRecord {
        ConfiguredProviderInstanceRecord::admit(
            ConfiguredProviderInstanceAdmission::new(
                self.driver.clone(),
                self.instance.clone(),
                self.access_profile.clone(),
                self.access_evidence.clone(),
            )
            .with_prepared_routes([evidence.clone()]),
        )
        .expect("configured record is valid")
    }

    pub(crate) fn catalogue_plan(&self) -> ProviderSessionCataloguePlan {
        ProviderSessionCataloguePlan::new(
            self.plan(),
            ProviderSessionCatalogueAgreement::new(
                ProviderSessionCatalogueId::new("fixture-catalogue").expect("id is valid"),
                ProviderSessionCatalogueScope::working_resource(resource()),
                ProviderSessionCatalogueBounds::new(
                    nonzero_u32(4),
                    nonzero_u32(8),
                    nonzero_u32(64),
                    nonzero_u32(256),
                    nonzero_u32(128),
                )
                .expect("bounds are valid"),
                None,
            ),
        )
        .expect("catalogue plan is valid")
    }

    pub(crate) fn catalogue_outcome(
        &self,
        plan: &ProviderSessionCataloguePlan,
    ) -> ProviderSessionCatalogueOutcome {
        let request = ProviderSessionCatalogueRequest::from_plan(
            swallowtail_runtime::RequestId::new("fixture-catalogue-request")
                .expect("request id is valid"),
            plan,
            None,
        )
        .expect("catalogue request is valid");
        ProviderSessionCatalogueOutcome::new(
            plan,
            &request,
            Vec::new(),
            None,
            CleanupOutcome::Clean,
        )
        .expect("catalogue outcome is valid")
    }

    pub(crate) fn history_plan(&self) -> ProviderSessionHistoryPlan {
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
        ProviderSessionHistoryPlan::new(
            self.plan(),
            ProviderSessionHistoryAgreement::new(
                ProviderSessionHistoryId::new("fixture-history").expect("id is valid"),
                SessionResumeBinding::new(
                    SessionRef::new("provider/private/session").expect("session ref is valid"),
                    self.instance.id().clone(),
                    self.instance.execution_host_id().clone(),
                    self.route.id().clone(),
                    self.route.model_id().clone(),
                    resource(),
                    policy,
                ),
                history_bounds(),
                None,
            ),
        )
        .expect("history plan is valid")
    }

    pub(crate) fn history_page(
        &self,
        plan: &ProviderSessionHistoryPlan,
    ) -> ProviderSessionHistoryPage {
        let request = ProviderSessionHistoryRequest::from_plan(
            swallowtail_runtime::RequestId::new("fixture-history-request")
                .expect("request id is valid"),
            plan,
            None,
        )
        .expect("history request is valid");
        let window = page_provider_session_history_window(
            plan,
            &request,
            Vec::new(),
            ProviderSessionHistoryTotal::Exact(0),
        )
        .expect("history window is valid");
        ProviderSessionHistoryPage::new(plan, &request, window, CleanupOutcome::Clean)
            .expect("history page is valid")
    }
}
