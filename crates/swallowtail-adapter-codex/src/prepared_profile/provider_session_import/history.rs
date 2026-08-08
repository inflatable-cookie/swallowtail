use super::super::input::CodexSessionHistoryInput;
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
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionHistoryEvidence,
    ProviderSessionHistoryAgreement, ProviderSessionHistoryCursor, ProviderSessionHistoryDriver,
    ProviderSessionHistoryPage, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
    RequestId, RuntimeFailure,
};

#[derive(Clone, Debug)]
/// Prepared read-only newest-first history pages for one retained Codex thread.
pub struct CodexPreparedSessionHistory {
    codex: CodexPreparedEvidence,
    evidence: PreparedProviderSessionHistoryEvidence,
    request: ProviderSessionHistoryRequest,
}

impl CodexPreparedSessionHistory {
    /// Returns portable evidence for the prepared history-page operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionHistoryEvidence {
        &self.evidence
    }

    /// Returns the underlying prepared Codex evidence.
    #[must_use]
    pub const fn codex_evidence(&self) -> &CodexPreparedEvidence {
        &self.codex
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

    /// Creates the low-level app-server driver bound to this history plan.
    #[must_use]
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.codex.environment().clone())
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
                    "swallowtail.codex.preparation.thread_history_request_invalid",
                    "Codex thread history continuation request could not be prepared",
                )
            })
    }
}

impl CodexPreparedIntegration {
    /// Prepares newest-first history pages for one retained Codex thread.
    pub fn prepare_session_history(
        &self,
        input: CodexSessionHistoryInput,
    ) -> Result<CodexPreparedSessionHistory, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        require_catalogue_version(self)?;
        let (request_id, history_id, model, binding, bounds, deadline) = input.into_parts();
        if Some(model.route_id()) != binding.model_route_id()
            || Some(model.model_id()) != binding.model_id()
        {
            return Err(failure(
                "swallowtail.codex.preparation.thread_history_binding_mismatch",
                "Codex history model does not match its durable session binding",
            ));
        }
        let history = CapabilityRequirement::new(
            Capability::ProviderSessionHistory,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_page_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_page_bytes().get()),
            ],
        );
        let resource = read_only_working_resource_capability();
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let capability_requirements = vec![history.clone(), resource.clone(), retention.clone()];
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
            OperationShape::ProviderSessionHistory,
            DriverRole::ProviderSessionHistory,
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
        let plan = ProviderSessionHistoryPlan::new(
            preflight.clone(),
            ProviderSessionHistoryAgreement::new(history_id, binding, bounds, deadline),
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.preparation.thread_history_plan_invalid",
                "Codex thread history plan could not be prepared",
            )
        })?;
        let request = ProviderSessionHistoryRequest::from_plan(request_id, &plan, None).map_err(
            |_| {
                failure(
                    "swallowtail.codex.preparation.thread_history_request_invalid",
                    "Codex thread history request could not be prepared",
                )
            },
        )?;
        Ok(CodexPreparedSessionHistory {
            codex: CodexPreparedEvidence::from_prepared(self, preflight)?,
            evidence: PreparedProviderSessionHistoryEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
