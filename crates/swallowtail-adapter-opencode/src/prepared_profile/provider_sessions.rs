use super::input::{
    OpenCodeSessionCatalogueInput, OpenCodeSessionProfileInput, OpenCodeSessionReconciliationInput,
};
use super::plan::{build_plan, failure, instance_with_capabilities};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, ProviderSessionImportAvailability, ResourceAccess,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionCatalogueEvidence,
    PreparedProviderSessionImportEvidence, PreparedProviderSessionReconciliationEvidence,
    ProviderSessionCandidate, ProviderSessionCatalogueDriver, ProviderSessionCatalogueOutcome,
    ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope,
    ProviderSessionImportAgreement, ProviderSessionImportDriver, ProviderSessionImportOutcome,
    ProviderSessionImportPlan, ProviderSessionImportRequest, ProviderSessionOperationFailure,
    ProviderSessionReconciliationAgreement, ProviderSessionReconciliationDriver,
    ProviderSessionReconciliationOutcome, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest, SessionPlanAgreement,
};

#[derive(Clone, Debug)]
pub struct OpenCodePreparedSessionCatalogue {
    prepared: OpenCodePreparedIntegration,
    evidence: PreparedProviderSessionCatalogueEvidence,
    request: ProviderSessionCatalogueRequest,
}

impl OpenCodePreparedSessionCatalogue {
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
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        self.prepared.low_level_driver()
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
                    "swallowtail.opencode.preparation.session_catalogue_request_invalid",
                    "OpenCode session catalogue continuation request could not be prepared",
                )
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct OpenCodePreparedSessionImport {
    prepared: OpenCodePreparedIntegration,
    evidence: PreparedProviderSessionImportEvidence,
    request: ProviderSessionImportRequest,
}

#[derive(Clone, Debug)]
pub struct OpenCodePreparedSessionReconciliation {
    prepared: OpenCodePreparedIntegration,
    evidence: PreparedProviderSessionReconciliationEvidence,
    request: ProviderSessionReconciliationRequest,
}

impl OpenCodePreparedSessionReconciliation {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionReconciliationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ProviderSessionReconciliationRequest {
        &self.request
    }

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
}

impl OpenCodePreparedSessionImport {
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
    pub fn low_level_driver(&self) -> OpenCodeHttpDriver {
        self.prepared.low_level_driver()
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

impl OpenCodePreparedIntegration {
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
        if &route_id != binding.model_route_id() || &model_id != binding.model_id() {
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
        let (request_id, model, working_resource, deadline, image_attachments, provider_callbacks) =
            input.into_parts();
        if image_attachments || provider_callbacks {
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
                        Capability::ModelCatalog | Capability::ProviderSessionDelete
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

fn require_qualified(prepared: &OpenCodePreparedIntegration) -> Result<(), PreparationFailure> {
    if prepared.server().is_qualified() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.opencode.preparation.session_catalogue_version_unsupported",
            "OpenCode session catalogue and import require a qualified server version",
        ))
    }
}

fn require_reconciliation_qualified(
    prepared: &OpenCodePreparedIntegration,
) -> Result<(), PreparationFailure> {
    if prepared.server().is_qualified() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.opencode.preparation.session_reconciliation_version_unsupported",
            "OpenCode session reconciliation requires a qualified server version",
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn provider_session_requirements(
    prepared: &OpenCodePreparedIntegration,
    shape: OperationShape,
    role: DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    require_model: bool,
    include_time: bool,
    access: Option<SessionAccessPolicy>,
) -> OperationRequirements {
    let mut services = vec![
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Network,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
    ];
    if include_time {
        services.push(HostServiceKind::Time);
    }
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
    .with_interface_versions([prepared.server().binding().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let requirements = access.map_or(requirements.clone(), |policy| {
        requirements
            .with_session_access_policy(policy)
            .with_session_provider_state_policy(
                SessionProviderStatePolicy::DurableProviderSessionPreserved,
            )
    });
    if require_model {
        requirements.require_model_route()
    } else {
        requirements
    }
}
