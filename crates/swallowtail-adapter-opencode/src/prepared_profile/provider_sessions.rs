use super::input::{
    OpenCodeSessionCatalogueInput, OpenCodeSessionProfileInput, OpenCodeSessionReconciliationInput,
};
use super::plan::{build_plan, failure, instance_with_capabilities};
use crate::{OpenCodeHttpDriver, OpenCodePreparedIntegration, OpenCodePreparedSession};
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, ProviderSessionImportAvailability, ResourceAccess,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, LoadSessionRequest, PreparationFailure,
    PreparedProviderSessionCatalogueEvidence, PreparedProviderSessionImportEvidence,
    PreparedProviderSessionReconciliationEvidence, PreparedSettledSessionRestoration,
    PreparedWorkingStateRestoration, ProviderSessionCandidate, ProviderSessionCatalogueDriver,
    ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest,
    ProviderSessionCatalogueScope, ProviderSessionImportAgreement, ProviderSessionImportDriver,
    ProviderSessionImportOutcome, ProviderSessionImportPlan, ProviderSessionImportRequest,
    ProviderSessionOperationFailure, ProviderSessionReconciliationAgreement,
    ProviderSessionReconciliationDriver, ProviderSessionReconciliationOutcome,
    ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest, RequestId,
    RuntimeFailure, SessionPlanAgreement, SettledSessionAttachment, SettledSessionAttachmentKind,
    SettledSessionAttachmentOperation, SettledSessionReconciliationOperation,
    WorkingStateRestorationMethod, WorkingStateRestorationOperation,
    WorkingStateRestorationOutcome, settled_session_plans_share_binding,
};

#[derive(Clone, Debug)]
/// Prepared, working-resource-scoped catalogue of retained OpenCode sessions.
pub struct OpenCodePreparedSessionCatalogue {
    prepared: OpenCodePreparedIntegration,
    evidence: PreparedProviderSessionCatalogueEvidence,
    request: ProviderSessionCatalogueRequest,
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

#[derive(Clone, Debug)]
/// Prepared import of one candidate selected from a session catalogue.
pub struct OpenCodePreparedSessionImport {
    prepared: OpenCodePreparedIntegration,
    evidence: PreparedProviderSessionImportEvidence,
    request: ProviderSessionImportRequest,
}

#[derive(Clone, Debug)]
/// Prepared read-only reconciliation of one exact retained provider session.
pub struct OpenCodePreparedSessionReconciliation {
    prepared: OpenCodePreparedIntegration,
    evidence: PreparedProviderSessionReconciliationEvidence,
    request: ProviderSessionReconciliationRequest,
}

impl OpenCodePreparedSessionReconciliation {
    /// Returns the reconciliation preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionReconciliationEvidence {
        &self.evidence
    }

    /// Returns the immutable reconciliation plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        self.evidence.plan()
    }

    /// Returns the exact reconciliation request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionReconciliationRequest {
        &self.request
    }

    /// Observes the retained session within the prepared replay bounds.
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

    /// Composes reconciliation with a separately prepared bounded replay load.
    pub fn prepare_settled_session_restoration(
        self,
        session: OpenCodePreparedSession,
        attachment_request_id: RequestId,
    ) -> Result<PreparedSettledSessionRestoration, PreparationFailure> {
        if !settled_session_plans_share_binding(self.plan().preflight(), session.plan())
            || self.prepared.server() != session.evidence().server()
            || self.prepared.access_evidence() != session.evidence().access()
        {
            return Err(failure(
                "swallowtail.opencode.preparation.settled_session_binding_mismatch",
                "OpenCode reconciliation and attachment do not share one prepared route binding",
            ));
        }
        let request = session.load_request(
            attachment_request_id,
            self.plan().agreement().binding().clone(),
        )?;
        Ok(PreparedSettledSessionRestoration::new(
            self,
            OpenCodeSettledSessionLoad { session, request },
        ))
    }
}

impl SettledSessionReconciliationOperation for OpenCodePreparedSessionReconciliation {
    fn reconcile(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        OpenCodePreparedSessionReconciliation::reconcile(&self, services)
    }
}

struct OpenCodeSettledSessionLoad {
    session: OpenCodePreparedSession,
    request: LoadSessionRequest,
}

impl SettledSessionAttachmentOperation for OpenCodeSettledSessionLoad {
    fn kind(&self) -> SettledSessionAttachmentKind {
        SettledSessionAttachmentKind::Load
    }

    fn attach(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<SettledSessionAttachment, RuntimeFailure>> {
        let future = self.session.load_prepared_session(self.request, services);
        Box::pin(async move { future.await.map(SettledSessionAttachment::Loaded) })
    }
}

impl WorkingStateRestorationOperation for OpenCodePreparedSessionReconciliation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderSessionReconciliation
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let future = self.reconcile(services);
        Box::pin(async move {
            future
                .await
                .map(WorkingStateRestorationOutcome::SessionReconciled)
        })
    }
}

impl OpenCodePreparedIntegration {
    /// Prepares the strongest route-supported post-crash restoration operation.
    pub fn prepare_working_state_restoration(
        &self,
        input: OpenCodeSessionReconciliationInput,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        self.prepare_session_reconciliation(input)
            .map(PreparedWorkingStateRestoration::new)
    }
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
    /// Validates and prepares read-only retained-session reconciliation.
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
        if Some(&route_id) != binding.model_route_id() || Some(&model_id) != binding.model_id() {
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
