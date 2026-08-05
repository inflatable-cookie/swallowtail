use super::CodexPreparedSessionKind;
use super::input::{
    CodexSessionCatalogueInput, CodexSessionProfileInput, CodexSessionReconciliationInput,
};
use super::plan::{
    CodexPreparedEvidence, build_plan, descriptor, failure, instance_with_capabilities,
    model_route, require_driver, requirements,
};
use super::session_capabilities::{behavior_revision, session_capabilities, supports_harness_mode};
use crate::selection::{CODEX_APP_SERVER_WORKSPACE_BEHAVIOR, supports_thread_catalogue_version};
use crate::{
    CodexAppServerDriver, CodexPreparedDriver, CodexPreparedIntegration,
    codex_bounded_workspace_capability,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, DriverRole,
    HarnessConfigurationPosture, HostServiceKind, OperationShape, ResourceAccess,
    ResourceRepresentation, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionCatalogueEvidence,
    PreparedProviderSessionImportEvidence, PreparedProviderSessionReconciliationEvidence,
    PreparedWorkingStateRestoration, ProviderSessionCandidate, ProviderSessionCatalogueDriver,
    ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest,
    ProviderSessionCatalogueScope, ProviderSessionImportAgreement, ProviderSessionImportDriver,
    ProviderSessionImportOutcome, ProviderSessionImportPlan, ProviderSessionImportRequest,
    ProviderSessionOperationFailure, ProviderSessionReconciliationAgreement,
    ProviderSessionReconciliationDriver, ProviderSessionReconciliationOutcome,
    ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest, RuntimeFailure,
    SessionPlanAgreement, WorkingStateRestorationMethod, WorkingStateRestorationOperation,
    WorkingStateRestorationOutcome,
};

#[derive(Clone, Debug)]
pub struct CodexPreparedSessionCatalogue {
    codex: CodexPreparedEvidence,
    evidence: PreparedProviderSessionCatalogueEvidence,
    request: ProviderSessionCatalogueRequest,
}

impl CodexPreparedSessionCatalogue {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionCatalogueEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn codex_evidence(&self) -> &CodexPreparedEvidence {
        &self.codex
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
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.codex.environment().clone())
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
                    "swallowtail.codex.preparation.thread_catalogue_request_invalid",
                    "Codex thread catalogue continuation request could not be prepared",
                )
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct CodexPreparedSessionImport {
    codex: CodexPreparedEvidence,
    evidence: PreparedProviderSessionImportEvidence,
    request: ProviderSessionImportRequest,
}

#[derive(Clone, Debug)]
pub struct CodexPreparedSessionReconciliation {
    codex: CodexPreparedEvidence,
    evidence: PreparedProviderSessionReconciliationEvidence,
    request: ProviderSessionReconciliationRequest,
}

impl CodexPreparedSessionReconciliation {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionReconciliationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn codex_evidence(&self) -> &CodexPreparedEvidence {
        &self.codex
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
        let driver = CodexAppServerDriver::new(self.codex.environment().clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .reconcile_provider_session(plan, request, services)
                .await
        })
    }
}

impl WorkingStateRestorationOperation for CodexPreparedSessionReconciliation {
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

impl CodexPreparedIntegration {
    pub fn prepare_working_state_restoration(
        &self,
        input: CodexSessionReconciliationInput,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        self.prepare_session_reconciliation(input)
            .map(PreparedWorkingStateRestoration::new)
    }
}

impl CodexPreparedSessionImport {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionImportEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn codex_evidence(&self) -> &CodexPreparedEvidence {
        &self.codex
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
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.codex.environment().clone())
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

impl CodexPreparedIntegration {
    pub fn prepare_session_reconciliation(
        &self,
        input: CodexSessionReconciliationInput,
    ) -> Result<CodexPreparedSessionReconciliation, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        require_catalogue_version(self)?;
        let (request_id, model, binding, interrupted_turn_id, provider_turn_ref, bounds, deadline) =
            input.into_parts();
        if Some(model.route_id()) != binding.model_route_id()
            || Some(model.model_id()) != binding.model_id()
        {
            return Err(failure(
                "swallowtail.codex.preparation.thread_reconciliation_binding_mismatch",
                "Codex reconciliation model does not match its durable session binding",
            ));
        }
        let reconciliation = CapabilityRequirement::new(
            Capability::ProviderSessionReconciliation,
            [
                CapabilityConstraint::ReplayMaximumItems(bounds.maximum_replay_items().get()),
                CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_replay_bytes().get()),
            ],
        );
        let resource = read_only_working_resource_capability();
        let retention = CapabilityRequirement::new(Capability::ProviderDurableRetention, []);
        let capability_requirements =
            vec![reconciliation.clone(), resource.clone(), retention.clone()];
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
            OperationShape::ProviderSessionReconciliation,
            DriverRole::ProviderSessionReconciliation,
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
        let plan = ProviderSessionReconciliationPlan::new(
            preflight.clone(),
            ProviderSessionReconciliationAgreement::new(
                binding,
                interrupted_turn_id,
                provider_turn_ref,
                bounds,
                deadline,
            ),
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.preparation.thread_reconciliation_plan_invalid",
                "Codex thread reconciliation plan could not be prepared",
            )
        })?;
        let request =
            ProviderSessionReconciliationRequest::from_plan(request_id, &plan).map_err(|_| {
                failure(
                    "swallowtail.codex.preparation.thread_reconciliation_request_invalid",
                    "Codex thread reconciliation request could not be prepared",
                )
            })?;
        Ok(CodexPreparedSessionReconciliation {
            codex: CodexPreparedEvidence::from_prepared(self, preflight)?,
            evidence: PreparedProviderSessionReconciliationEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }

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

    pub fn prepare_read_only_session_import(
        &self,
        catalogue: &CodexPreparedSessionCatalogue,
        candidate: ProviderSessionCandidate,
        input: CodexSessionProfileInput,
    ) -> Result<CodexPreparedSessionImport, PreparationFailure> {
        self.prepare_session_import(
            catalogue,
            candidate,
            CodexPreparedSessionKind::ReadOnly,
            input,
        )
    }

    pub fn prepare_bounded_workspace_session_import(
        &self,
        catalogue: &CodexPreparedSessionCatalogue,
        candidate: ProviderSessionCandidate,
        input: CodexSessionProfileInput,
    ) -> Result<CodexPreparedSessionImport, PreparationFailure> {
        self.prepare_session_import(
            catalogue,
            candidate,
            CodexPreparedSessionKind::BoundedWorkspace,
            input,
        )
    }

    fn prepare_session_import(
        &self,
        catalogue: &CodexPreparedSessionCatalogue,
        candidate: ProviderSessionCandidate,
        kind: CodexPreparedSessionKind,
        input: CodexSessionProfileInput,
    ) -> Result<CodexPreparedSessionImport, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        require_catalogue_version(self)?;
        if kind == CodexPreparedSessionKind::BoundedWorkspace
            && behavior_revision(self) != Some(CODEX_APP_SERVER_WORKSPACE_BEHAVIOR)
        {
            return Err(failure(
                "swallowtail.codex.preparation.workspace_version_unsupported",
                "Prepared Codex version does not support bounded workspace roots",
            ));
        }
        let (request_id, model, working_resource, deadline, options, user_input_exchange) =
            input.into_parts();
        if options.tools().len() != 0 {
            return Err(failure(
                "swallowtail.codex.preparation.import_tools_unsupported",
                "Codex imported sessions cannot redeclare dynamic tools during load",
            ));
        }
        if options.harness_mode().is_some() && !supports_harness_mode(self) {
            return Err(failure(
                "swallowtail.codex.preparation.harness_mode_unsupported",
                "Prepared Codex version does not support harness mode selection",
            ));
        }
        let (mut capability_requirements, extension_namespaces, access_policy) =
            session_capabilities(kind, &options, user_input_exchange)?;
        capability_requirements.extend([
            CapabilityRequirement::new(Capability::ProviderSessionImport, []),
            CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
            match kind {
                CodexPreparedSessionKind::ReadOnly => read_only_working_resource_capability(),
                CodexPreparedSessionKind::BoundedWorkspace => codex_bounded_workspace_capability(),
            },
        ]);
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
            OperationShape::ProviderSessionImport,
            DriverRole::ProviderSessionImport,
            host_services,
            capability_requirements,
        )
        .with_extension_namespaces(extension_namespaces)
        .with_session_access_policy(access_policy)
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
        let session = SessionPlanAgreement::from_plan(&preflight).map_err(|_| {
            failure(
                "swallowtail.codex.preparation.thread_import_session_invalid",
                "Codex thread import session agreement could not be prepared",
            )
        })?;
        let plan = ProviderSessionImportPlan::new(
            preflight.clone(),
            catalogue.plan().clone(),
            ProviderSessionImportAgreement::new(candidate, working_resource, session, deadline),
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.preparation.thread_import_plan_invalid",
                "Codex thread import plan does not match its source catalogue",
            )
        })?;
        let request = ProviderSessionImportRequest::from_plan(request_id, &plan).map_err(|_| {
            failure(
                "swallowtail.codex.preparation.thread_import_request_invalid",
                "Codex thread import request could not be prepared",
            )
        })?;
        Ok(CodexPreparedSessionImport {
            codex: CodexPreparedEvidence::from_prepared(self, preflight)?,
            evidence: PreparedProviderSessionImportEvidence::from_plan(
                plan,
                self.access_evidence().clone(),
            )?,
            request,
        })
    }
}

fn require_catalogue_version(
    prepared: &CodexPreparedIntegration,
) -> Result<(), PreparationFailure> {
    if supports_thread_catalogue_version(prepared.observation().version().version()) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.codex.preparation.thread_catalogue_version_unsupported",
            "Prepared Codex version does not support the qualified thread catalogue",
        ))
    }
}

fn read_only_working_resource_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}
