use super::input::{KimiSessionCatalogueInput, KimiSessionProfileInput};
use super::plan::{build_plan, build_plan_without_route, failure, instance_with_capabilities};
use super::session::{reject_attachment_reasoning, validate_options};
use crate::prepared::instance::session_capabilities;
use crate::{KimiAcpDriver, KimiPreparedIntegration};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, ProviderSessionImportAvailability, ResourceAccess,
    ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionCatalogueEvidence,
    PreparedProviderSessionImportEvidence, ProviderSessionCandidate,
    ProviderSessionCatalogueDriver, ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope, ProviderSessionImportAgreement,
    ProviderSessionImportDriver, ProviderSessionImportOutcome, ProviderSessionImportPlan,
    ProviderSessionImportRequest, ProviderSessionOperationFailure, SessionPlanAgreement,
};

#[derive(Clone, Debug)]
pub struct KimiPreparedSessionCatalogue {
    kimi: KimiPreparedIntegration,
    evidence: PreparedProviderSessionCatalogueEvidence,
    request: ProviderSessionCatalogueRequest,
}

impl KimiPreparedSessionCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionCatalogueEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionCataloguePlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ProviderSessionCatalogueRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> KimiAcpDriver {
        self.kimi.low_level_driver()
    }

    pub fn list_sessions(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        self.list_page(self.request.clone(), services)
    }

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

    pub fn next_page_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        cursor: swallowtail_runtime::ProviderSessionCursor,
    ) -> Result<ProviderSessionCatalogueRequest, PreparationFailure> {
        ProviderSessionCatalogueRequest::from_plan(request_id, self.plan(), Some(cursor)).map_err(
            |_| {
                failure(
                    "swallowtail.kimi.preparation.session_catalogue_request_invalid",
                    "Kimi session catalogue continuation request could not be prepared",
                )
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct KimiPreparedSessionImport {
    kimi: KimiPreparedIntegration,
    evidence: PreparedProviderSessionImportEvidence,
    request: ProviderSessionImportRequest,
}

impl KimiPreparedSessionImport {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionImportEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionImportPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ProviderSessionImportRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> KimiAcpDriver {
        self.kimi.low_level_driver()
    }

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

impl KimiPreparedIntegration {
    pub fn prepare_session_catalogue(
        &self,
        input: KimiSessionCatalogueInput,
    ) -> Result<KimiPreparedSessionCatalogue, PreparationFailure> {
        require_catalogue_version(self)?;
        require_state_root(self)?;
        let (request_id, catalogue_id, working_resource, bounds, deadline) = input.into_parts();
        let catalogue = CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []);
        let resource = working_resource_capability(ResourceAccess::Read);
        let capabilities = CapabilityProfile::new([catalogue.clone(), resource.clone()]);
        let instance = instance_with_capabilities(self, capabilities);
        let requirements = operation_requirements(
            self,
            OperationShape::ProviderSessionCatalogue,
            DriverRole::ProviderSessionCatalogue,
            catalogue_services(deadline.is_some()),
            [catalogue, resource],
            None,
            None,
            false,
        );
        let preflight = build_plan_without_route(self, &instance, &requirements)?;
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
                "swallowtail.kimi.preparation.session_catalogue_plan_invalid",
                "Kimi session catalogue plan could not be prepared",
            )
        })?;
        let request =
            ProviderSessionCatalogueRequest::from_plan(request_id, &plan, None).map_err(|_| {
                failure(
                    "swallowtail.kimi.preparation.session_catalogue_request_invalid",
                    "Kimi session catalogue request could not be prepared",
                )
            })?;
        Ok(KimiPreparedSessionCatalogue {
            kimi: self.clone(),
            evidence: PreparedProviderSessionCatalogueEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }

    pub fn prepare_session_import(
        &self,
        catalogue: &KimiPreparedSessionCatalogue,
        candidate: ProviderSessionCandidate,
        input: KimiSessionProfileInput,
    ) -> Result<KimiPreparedSessionImport, PreparationFailure> {
        require_catalogue_version(self)?;
        let state_root = require_state_root(self)?;
        if catalogue.kimi.state_root() != Some(state_root)
            || candidate.import_availability() != ProviderSessionImportAvailability::Available
        {
            return Err(failure(
                "swallowtail.kimi.preparation.session_import_source_mismatch",
                "Kimi session import does not match its state root or candidate authority",
            ));
        }
        let (request_id, model, working_resource, options) = input.into_parts();
        validate_options(&options)?;
        reject_attachment_reasoning(&options)?;
        let mut capability_requirements = session_capabilities()
            .iter()
            .map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            })
            .collect::<Vec<_>>();
        capability_requirements.extend([
            CapabilityRequirement::new(Capability::ProviderSessionImport, []),
            CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        ]);
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        );
        let requirements = operation_requirements(
            self,
            OperationShape::ProviderSessionImport,
            DriverRole::ProviderSessionImport,
            catalogue_services(false),
            capability_requirements,
            Some(SessionAccessPolicy::ambient_harness(
                ResourceAccess::ReadWrite,
            )),
            Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
            true,
        );
        let preflight = build_plan(self, &instance, &route, &requirements)?;
        let session = SessionPlanAgreement::from_plan(&preflight).map_err(|_| {
            failure(
                "swallowtail.kimi.preparation.session_import_agreement_invalid",
                "Kimi session import agreement could not be prepared",
            )
        })?;
        let plan = ProviderSessionImportPlan::new(
            preflight,
            catalogue.plan().clone(),
            ProviderSessionImportAgreement::new(candidate, working_resource, session, None),
        )
        .map_err(|_| {
            failure(
                "swallowtail.kimi.preparation.session_import_plan_invalid",
                "Kimi session import does not match its source catalogue",
            )
        })?;
        let request = ProviderSessionImportRequest::from_plan(request_id, &plan).map_err(|_| {
            failure(
                "swallowtail.kimi.preparation.session_import_request_invalid",
                "Kimi session import request could not be prepared",
            )
        })?;
        Ok(KimiPreparedSessionImport {
            kimi: self.clone(),
            evidence: PreparedProviderSessionImportEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}

fn require_catalogue_version(prepared: &KimiPreparedIntegration) -> Result<(), PreparationFailure> {
    if prepared.observation().is_qualified() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.preparation.session_catalogue_version_unsupported",
            "Kimi session catalogue requires a qualified executable version",
        ))
    }
}

fn require_state_root(
    prepared: &KimiPreparedIntegration,
) -> Result<&swallowtail_runtime::WorkingResourceRef, PreparationFailure> {
    prepared.state_root().ok_or_else(|| {
        failure(
            "swallowtail.kimi.preparation.session_catalogue_state_root_missing",
            "Kimi session catalogue requires an explicit opaque state-root identity",
        )
    })
}

fn working_resource_capability(access: ResourceAccess) -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(access),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}

fn catalogue_services(include_time: bool) -> Vec<HostServiceKind> {
    let mut services = vec![
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
    ];
    if include_time {
        services.push(HostServiceKind::Time);
    }
    services
}

#[allow(clippy::too_many_arguments)]
fn operation_requirements(
    prepared: &KimiPreparedIntegration,
    shape: OperationShape,
    role: DriverRole,
    services: impl IntoIterator<Item = HostServiceKind>,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    access_policy: Option<SessionAccessPolicy>,
    provider_state: Option<SessionProviderStatePolicy>,
    require_model: bool,
) -> OperationRequirements {
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        shape,
        role,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let requirements = if let Some(access_policy) = access_policy {
        requirements.with_session_access_policy(access_policy)
    } else {
        requirements
    };
    let requirements = if let Some(provider_state) = provider_state {
        requirements.with_session_provider_state_policy(provider_state)
    } else {
        requirements
    };
    if require_model {
        requirements.require_model_route()
    } else {
        requirements
    }
}
