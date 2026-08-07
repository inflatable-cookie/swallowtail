use super::{read_only_working_resource_capability, require_catalogue_version};
use super::super::input::CodexSessionCatalogueInput;
use super::super::plan::{
    CodexPreparedEvidence, build_plan, descriptor, failure, instance_with_capabilities, require_driver,
    requirements,
};
use crate::{CodexAppServerDriver, CodexPreparedDriver, CodexPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, HarnessConfigurationPosture,
    HostServiceKind, OperationShape,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionCatalogueEvidence,
    ProviderSessionCatalogueDriver, ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope, ProviderSessionOperationFailure,
};

#[derive(Clone, Debug)]
/// Prepared read-only catalogue of retained Codex threads.
pub struct CodexPreparedSessionCatalogue {
    codex: CodexPreparedEvidence,
    evidence: PreparedProviderSessionCatalogueEvidence,
    request: ProviderSessionCatalogueRequest,
}

impl CodexPreparedSessionCatalogue {
    /// Returns portable evidence for the prepared catalogue.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionCatalogueEvidence {
        &self.evidence
    }

    /// Returns the underlying prepared Codex evidence.
    #[must_use]
    pub const fn codex_evidence(&self) -> &CodexPreparedEvidence {
        &self.codex
    }

    /// Returns the exact provider-session catalogue plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionCataloguePlan {
        self.evidence.plan()
    }

    /// Returns the initial catalogue-page request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionCatalogueRequest {
        &self.request
    }

    /// Creates the low-level app-server driver bound to this catalogue.
    #[must_use]
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.codex.environment().clone())
    }

    /// Lists the initial page of retained Codex threads.
    pub fn list_sessions(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        self.list_page(self.request.clone(), services)
    }

    /// Lists one explicitly supplied catalogue page.
    pub fn list_page(
        &self,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        Box::pin(async move { driver.list_provider_sessions(plan, request, services).await })
    }

    /// Builds a continuation request from an opaque provider cursor.
    pub fn next_page_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        cursor: swallowtail_runtime::ProviderSessionCursor,
    ) -> Result<ProviderSessionCatalogueRequest, PreparationFailure> {
        ProviderSessionCatalogueRequest::from_plan(request_id, self.plan(), Some(cursor)).map_err(
            |_| {
                failure(
                    "swallowtail.codex.preparation.thread_catalogue_request_invalid",
                    "Codex thread catalogue continuation request could not be prepared",
                )
            },
        )
    }
}

impl CodexPreparedIntegration {
/// Prepares a bounded working-resource-scoped retained-thread catalogue.
    pub fn prepare_session_catalogue(
        &self,
        input: CodexSessionCatalogueInput,
    ) -> Result<CodexPreparedSessionCatalogue, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        require_catalogue_version(self)?;
        let (request_id, catalogue_id, working_resource, bounds, deadline) = input.into_parts();
        let catalogue = CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []);
        let working_resource_capability = read_only_working_resource_capability();
        let capabilities =
            CapabilityProfile::new([catalogue.clone(), working_resource_capability.clone()]);
        let instance = instance_with_capabilities(self, capabilities);
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
            OperationShape::ProviderSessionCatalogue,
            DriverRole::ProviderSessionCatalogue,
            host_services,
            [catalogue, working_resource_capability],
        )
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let preflight = build_plan(self, &descriptor(self), &instance, None, &requirements)?;
        let plan = ProviderSessionCataloguePlan::new(
            preflight.clone(),
            swallowtail_runtime::ProviderSessionCatalogueAgreement::new(
                catalogue_id,
                ProviderSessionCatalogueScope::working_resource(working_resource),
                bounds,
                deadline,
            ),
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.preparation.thread_catalogue_plan_invalid",
                "Codex thread catalogue plan could not be prepared",
            )
        })?;
        let request =
            ProviderSessionCatalogueRequest::from_plan(request_id, &plan, None).map_err(|_| {
                failure(
                    "swallowtail.codex.preparation.thread_catalogue_request_invalid",
                    "Codex thread catalogue request could not be prepared",
                )
            })?;
        Ok(CodexPreparedSessionCatalogue {
            codex: CodexPreparedEvidence::from_prepared(self, preflight)?,
            evidence: PreparedProviderSessionCatalogueEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
