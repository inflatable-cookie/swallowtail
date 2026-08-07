use super::super::input::OpenCodeSessionCatalogueInput;
use super::super::plan::{build_plan, failure, instance_with_capabilities};
use super::{provider_session_requirements, require_qualified};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, OperationShape,
    ResourceAccess,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionCatalogueEvidence,
    ProviderSessionCatalogueDriver, ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope,
    ProviderSessionOperationFailure,
};

#[derive(Clone, Debug)]
/// Prepared, working-resource-scoped catalogue of retained OpenCode sessions.
pub struct OpenCodePreparedSessionCatalogue {
    pub(super) prepared: OpenCodePreparedIntegration,
    pub(super) evidence: PreparedProviderSessionCatalogueEvidence,
    pub(super) request: ProviderSessionCatalogueRequest,
}

impl OpenCodePreparedSessionCatalogue {
    /// Returns the session-catalogue preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionCatalogueEvidence {
        &self.evidence
    }
    /// Returns the immutable session-catalogue plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionCataloguePlan {
        self.evidence.plan()
    }
    /// Returns the first-page catalogue request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionCatalogueRequest {
        &self.request
    }
    /// Creates the stateless low-level HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        self.prepared.low_level_driver()
    }

    /// Lists the first prepared page of retained sessions.
    pub fn list_sessions(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        self.list_page(self.request.clone(), services)
    }

    /// Lists one explicitly supplied page under the prepared bounds.
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

    /// Builds a continuation request for an opaque provider cursor.
    pub fn next_page_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        cursor: swallowtail_runtime::ProviderSessionCursor,
    ) -> Result<ProviderSessionCatalogueRequest, PreparationFailure> {
        ProviderSessionCatalogueRequest::from_plan(request_id, self.plan(), Some(cursor)).map_err(
            |_| {
                failure(
                    "swallowtail.opencode.preparation.session_catalogue_request_invalid",
                    "OpenCode session catalogue continuation request could not be prepared",
                )
            },
        )
    }
}

impl OpenCodePreparedIntegration {
    /// Validates and prepares a bounded retained-session catalogue.
    pub fn prepare_session_catalogue(
        &self,
        input: OpenCodeSessionCatalogueInput,
    ) -> Result<OpenCodePreparedSessionCatalogue, PreparationFailure> {
        require_qualified(self)?;
        let (request_id, catalogue_id, working_resource, bounds, deadline) = input.into_parts();
        let catalogue = CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []);
        let resource = crate::prepared::working_resource_capability(ResourceAccess::Read);
        let capabilities = CapabilityProfile::new([catalogue.clone(), resource.clone()]);
        let instance = instance_with_capabilities(self, capabilities);
        let requirements = provider_session_requirements(
            self,
            OperationShape::ProviderSessionCatalogue,
            DriverRole::ProviderSessionCatalogue,
            [catalogue, resource],
            false,
            deadline.is_some(),
            None,
        );
        let preflight = build_plan(self, &instance, None, &requirements)?;
        let plan = ProviderSessionCataloguePlan::new(
            preflight,
            swallowtail_runtime::ProviderSessionCatalogueAgreement::new(
                catalogue_id,
                ProviderSessionCatalogueScope::working_resource(working_resource),
                bounds,
                deadline,
            ),
        )
        .map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.session_catalogue_plan_invalid",
                "OpenCode session catalogue plan could not be prepared",
            )
        })?;
        let request =
            ProviderSessionCatalogueRequest::from_plan(request_id, &plan, None).map_err(|_| {
                failure(
                    "swallowtail.opencode.preparation.session_catalogue_request_invalid",
                    "OpenCode session catalogue request could not be prepared",
                )
            })?;
        Ok(OpenCodePreparedSessionCatalogue {
            prepared: self.clone(),
            evidence: PreparedProviderSessionCatalogueEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
