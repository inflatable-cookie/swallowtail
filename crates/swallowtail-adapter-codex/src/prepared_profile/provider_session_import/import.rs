use super::super::CodexPreparedSessionKind;
use super::super::input::CodexSessionProfileInput;
use super::super::plan::{
    CodexPreparedEvidence, build_plan, descriptor, failure, instance_with_capabilities,
    model_route, require_driver, requirements,
};
use super::super::session_capabilities::behavior_revision;
use super::super::session_capabilities::{session_capabilities, supports_harness_mode};
use super::catalogue::CodexPreparedSessionCatalogue;
use super::{read_only_working_resource_capability, require_catalogue_version};
use crate::selection::CODEX_APP_SERVER_WORKSPACE_BEHAVIOR;
use crate::{
    CodexAppServerDriver, CodexPreparedDriver, CodexPreparedIntegration,
    codex_bounded_workspace_capability,
};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, HarnessConfigurationPosture,
    HostServiceKind, OperationShape, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, PreparedProviderSessionImportEvidence,
    ProviderSessionCandidate, ProviderSessionImportAgreement, ProviderSessionImportDriver,
    ProviderSessionImportOutcome, ProviderSessionImportPlan, ProviderSessionImportRequest,
    ProviderSessionOperationFailure, SessionPlanAgreement,
};

#[derive(Clone, Debug)]
/// Prepared read-only import of one retained Codex thread.
pub struct CodexPreparedSessionImport {
    codex: CodexPreparedEvidence,
    evidence: PreparedProviderSessionImportEvidence,
    request: ProviderSessionImportRequest,
}

impl CodexPreparedSessionImport {
    /// Returns portable evidence for the prepared import.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionImportEvidence {
        &self.evidence
    }

    /// Returns the underlying prepared Codex evidence.
    #[must_use]
    pub const fn codex_evidence(&self) -> &CodexPreparedEvidence {
        &self.codex
    }

    /// Returns the exact provider-session import plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionImportPlan {
        self.evidence.plan()
    }

    /// Returns the bound import request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionImportRequest {
        &self.request
    }

    /// Creates the low-level app-server driver bound to this import.
    #[must_use]
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.codex.environment().clone())
    }

    /// Revalidates and imports the selected retained thread as resume authority.
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
    /// Prepares read-only import of one candidate from the bound catalogue.
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

    /// Prepares bounded-workspace import of one candidate from the bound catalogue.
    /// Prepares bounded-workspace import of one candidate from the bound catalogue.
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
