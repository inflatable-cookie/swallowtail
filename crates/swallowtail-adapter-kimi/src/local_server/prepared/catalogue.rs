use super::{
    KimiLocalServerCatalogueInput, KimiLocalServerPreparedIntegration, preparation_failure,
};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityProfile, CapabilityRequirement, CredentialState,
    Diagnostic, DriverRole, EndpointAuthorization, EntitlementState, ExecutionLayer,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, RuntimeReadiness,
    preflight,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerPreparedCatalogue {
    evidence: PreparedOperationEvidence,
    request: ModelCatalogRequest,
}

impl KimiLocalServerPreparedCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> crate::KimiLocalServerDriver {
        crate::KimiLocalServerDriver::new()
    }

    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<swallowtail_core::ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }
}

impl KimiLocalServerPreparedIntegration {
    pub fn prepare_catalogue(
        &self,
        input: KimiLocalServerCatalogueInput,
    ) -> Result<KimiLocalServerPreparedCatalogue, PreparationFailure> {
        let (request_id, deadline, allow_unverified_newer) = input.into_parts();
        if !self.server().is_qualified() && !allow_unverified_newer {
            return Err(preparation_failure(
                PreparationStage::CompatibilityClassification,
                "swallowtail.kimi.local_server.preparation.catalogue_unverified_newer",
                "Newer unverified Kimi local-server catalogue requires explicit acceptance",
            ));
        }
        let capability = CapabilityRequirement::new(Capability::ModelCatalog, []);
        let instance = super::operation::instance_with_capabilities(
            self,
            CapabilityProfile::new([capability.clone()]),
        );
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::ModelCatalog,
            self.instance().execution_host_id().clone(),
            AccessRequirement::new(self.access_profile().id().clone())
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([self.access_profile().support_authority()]),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services(
            crate::kimi_local_server_descriptor().required_host_services(DriverRole::ModelCatalog),
        )
        .with_capabilities([capability])
        .with_interface_versions([self.server().binding().clone()]);
        let plan = preflight(
            &PreflightContext::new(
                &crate::kimi_local_server_descriptor(),
                &instance,
                self.access_profile(),
                self.access_evidence().status(),
                self.available_host_services(),
            ),
            &requirements,
        )
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let request = match deadline {
            Some(deadline) => ModelCatalogRequest::new(request_id).with_deadline(deadline),
            None => ModelCatalogRequest::new(request_id),
        };
        Ok(KimiLocalServerPreparedCatalogue {
            evidence: PreparedOperationEvidence::from_plan(plan, self.access_evidence().clone())?,
            request,
        })
    }
}
