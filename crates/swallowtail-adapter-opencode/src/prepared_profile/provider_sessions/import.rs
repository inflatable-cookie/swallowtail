use super::catalogue::OpenCodePreparedSessionCatalogue;
use super::{provider_session_requirements, require_qualified};
use super::super::input::OpenCodeSessionProfileInput;
use super::super::plan::{build_plan, failure, instance_with_capabilities};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, ModelRoute, OperationShape,
    ProviderSessionImportAvailability, ResourceAccess, SessionAccessPolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionImportEvidence,
    ProviderSessionCandidate, ProviderSessionImportAgreement, ProviderSessionImportDriver,
    ProviderSessionImportOutcome, ProviderSessionImportPlan, ProviderSessionImportRequest,
    ProviderSessionOperationFailure, SessionPlanAgreement,
};

#[derive(Clone, Debug)]
/// Prepared import of one candidate selected from a session catalogue.
pub struct OpenCodePreparedSessionImport {
    pub(super) prepared: OpenCodePreparedIntegration,
    pub(super) evidence: PreparedProviderSessionImportEvidence,
    pub(super) request: ProviderSessionImportRequest,
}


impl OpenCodePreparedSessionImport {
    /// Returns the import preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionImportEvidence {
        &self.evidence
    }
    /// Returns the immutable import plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionImportPlan {
        self.evidence.plan()
    }
    /// Returns the exact import request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionImportRequest {
        &self.request
    }
    /// Creates the stateless low-level HTTP driver.
    #[must_use]
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        self.prepared.low_level_driver()
    }

    /// Revalidates and imports the selected provider-session candidate.
    pub fn import_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure>>
    {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .import_provider_session(plan, request, services)
                .await
        })
    }
}

impl OpenCodePreparedIntegration {
    /// Validates and prepares import authority for one catalogue candidate.
    pub fn prepare_session_import(
        &self,
        catalogue: &OpenCodePreparedSessionCatalogue,
        candidate: ProviderSessionCandidate,
        input: OpenCodeSessionProfileInput,
    ) -> Result<OpenCodePreparedSessionImport, PreparationFailure> {
        require_qualified(self)?;
        if catalogue.prepared.instance().id() != self.instance().id()
            || candidate.import_availability() != ProviderSessionImportAvailability::Available
        {
            return Err(failure(
                "swallowtail.opencode.preparation.session_import_source_mismatch",
                "OpenCode session import does not match its catalogue authority",
            ));
        }
        let (
            request_id,
            model,
            working_resource,
            deadline,
            image_attachments,
            provider_callbacks,
            active_turn_detachment,
        ) = input.into_parts();
        if image_attachments || provider_callbacks || active_turn_detachment {
            return Err(failure(
                "swallowtail.opencode.preparation.session_import_options_unsupported",
                "OpenCode session import currently requires the default read-only session profile",
            ));
        }
        let selected = CapabilityProfile::new(
            crate::prepared::all_capabilities()
                .iter()
                .filter(|(capability, _)| {
                    !matches!(
                        *capability,
                        Capability::ModelCatalog
                            | Capability::ProviderSessionDelete
                            | Capability::ActiveOperationDetachment
                            | Capability::ProviderDurableRetention
                    )
                })
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                })
                .chain([
                    CapabilityRequirement::new(Capability::ProviderSessionImport, []),
                    CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
                ]),
        );
        let instance = instance_with_capabilities(self, selected.clone());
        let (route_id, route_revision, provider_id, model_id, _) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            selected.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = provider_session_requirements(
            self,
            OperationShape::ProviderSessionImport,
            DriverRole::ProviderSessionImport,
            selected.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
            true,
            deadline.is_some(),
            Some(SessionAccessPolicy::ambient_harness(ResourceAccess::Read)),
        );
        let preflight = build_plan(self, &instance, Some(&route), &requirements)?;
        let session = SessionPlanAgreement::from_plan(&preflight).map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.session_import_agreement_invalid",
                "OpenCode session import agreement could not be prepared",
            )
        })?;
        let plan = ProviderSessionImportPlan::new(
            preflight,
            catalogue.plan().clone(),
            ProviderSessionImportAgreement::new(candidate, working_resource, session, deadline),
        )
        .map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.session_import_plan_invalid",
                "OpenCode session import does not match its source catalogue",
            )
        })?;
        let request = ProviderSessionImportRequest::from_plan(request_id, &plan).map_err(|_| {
            failure(
                "swallowtail.opencode.preparation.session_import_request_invalid",
                "OpenCode session import request could not be prepared",
            )
        })?;
        Ok(OpenCodePreparedSessionImport {
            prepared: self.clone(),
            evidence: PreparedProviderSessionImportEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}
