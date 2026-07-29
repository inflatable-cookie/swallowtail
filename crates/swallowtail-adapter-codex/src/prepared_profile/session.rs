use super::input::CodexSessionProfileInput;
use super::plan::{
    CodexPreparedEvidence, build_plan, descriptor, failure, instance_with_capabilities,
    model_route, require_driver, requirements,
};
use super::session_capabilities::{behavior_revision, session_capabilities};
use super::{CodexPreparedSessionFuture, CodexPreparedSessionKind, CodexPreparedSessionLoadFuture};
use crate::selection::CODEX_APP_SERVER_WORKSPACE_BEHAVIOR;
use crate::{
    CodexAppServerDriver, CodexPreparedDriver, CodexPreparedIntegration,
    codex_bounded_workspace_access_policy, codex_bounded_workspace_capability,
};
use swallowtail_core::{
    CapabilityProfile, DriverRole, HarnessConfigurationPosture, HostServiceKind, OperationShape,
    PreflightPlan, ProviderSessionBindingOrigin, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest,
    PreparationFailure, SessionResumeBinding,
};

#[path = "session/management_handle.rs"]
mod management_handle;
use management_handle::{lifecycle_management_instance, wrap_management_handle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodexPreparedSession {
    kind: CodexPreparedSessionKind,
    evidence: CodexPreparedEvidence,
    request: OpenSessionRequest,
    management_instance: Option<swallowtail_core::ConfiguredInstance>,
}

impl CodexPreparedSession {
    #[must_use]
    pub const fn kind(&self) -> CodexPreparedSessionKind {
        self.kind
    }

    #[must_use]
    pub const fn evidence(&self) -> &CodexPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.evidence.environment().clone())
    }

    pub fn open_session(&self, services: HostServices) -> CodexPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            let handle = driver.open_session(plan, request.clone(), services).await?;
            wrap_management_handle(
                handle,
                management_instance,
                access,
                request.working_resource().cloned(),
                ProviderSessionBindingOrigin::Created,
            )
            .await
        })
    }

    #[must_use]
    pub fn into_parts(self) -> (CodexPreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }

    pub fn resume_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
    ) -> Result<swallowtail_runtime::ResumeSessionRequest, PreparationFailure> {
        if self.request.options().tools().len() != 0 {
            return Err(failure(
                "swallowtail.codex.preparation.resume_tools_unsupported",
                "Codex resumed sessions cannot redeclare dynamic tools",
            ));
        }
        Ok(swallowtail_runtime::ResumeSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Codex session binds a working resource")
                .clone(),
            self.request.deadline(),
        )?
        .with_options(self.request.options().clone()))
    }

    pub fn load_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
    ) -> Result<LoadSessionRequest, PreparationFailure> {
        if self.request.options().tools().len() != 0 {
            return Err(failure(
                "swallowtail.codex.preparation.load_tools_unsupported",
                "Codex loaded sessions cannot redeclare dynamic tools",
            ));
        }
        Ok(LoadSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Codex session binds a working resource")
                .clone(),
            self.request.deadline(),
        )?
        .with_options(self.request.options().clone()))
    }

    pub fn load_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<CodexPreparedSessionLoadFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.load_request(request_id, binding)?;
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Ok(Box::pin(async move {
            let loaded = driver.load_session(plan, request.clone(), services).await?;
            let (replay, handle) = loaded.into_parts();
            let handle = wrap_management_handle(
                handle,
                management_instance,
                access,
                Some(request.working_resource().clone()),
                ProviderSessionBindingOrigin::Loaded,
            )
            .await?;
            Ok(swallowtail_runtime::LoadedSession::new(replay, handle))
        }))
    }

    pub fn resume_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<CodexPreparedSessionFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.resume_request(request_id, binding)?;
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Ok(Box::pin(async move {
            let handle = driver
                .resume_session(plan, request.clone(), services)
                .await?;
            wrap_management_handle(
                handle,
                management_instance,
                access,
                Some(request.working_resource().clone()),
                ProviderSessionBindingOrigin::Resumed,
            )
            .await
        }))
    }
}

impl CodexPreparedIntegration {
    pub fn prepare_read_only_session(
        &self,
        input: CodexSessionProfileInput,
    ) -> Result<CodexPreparedSession, PreparationFailure> {
        self.prepare_session(CodexPreparedSessionKind::ReadOnly, input)
    }

    pub fn prepare_bounded_workspace_session(
        &self,
        input: CodexSessionProfileInput,
    ) -> Result<CodexPreparedSession, PreparationFailure> {
        self.prepare_session(CodexPreparedSessionKind::BoundedWorkspace, input)
    }

    fn prepare_session(
        &self,
        kind: CodexPreparedSessionKind,
        input: CodexSessionProfileInput,
    ) -> Result<CodexPreparedSession, PreparationFailure> {
        require_driver(self, CodexPreparedDriver::AppServer)?;
        if kind == CodexPreparedSessionKind::BoundedWorkspace
            && behavior_revision(self) != Some(CODEX_APP_SERVER_WORKSPACE_BEHAVIOR)
        {
            return Err(failure(
                "swallowtail.codex.preparation.workspace_version_unsupported",
                "Prepared Codex version does not support bounded workspace roots",
            ));
        }
        let (request_id, model, working_resource, deadline, options) = input.into_parts();
        if deadline.is_some() {
            return Err(failure(
                "swallowtail.codex.preparation.session_deadline_unsupported",
                "Codex app-server sessions do not support an operation deadline",
            ));
        }
        let (mut capability_requirements, mut extension_namespaces, access_policy) =
            session_capabilities(kind, &options)?;
        if kind == CodexPreparedSessionKind::BoundedWorkspace {
            capability_requirements.push(codex_bounded_workspace_capability());
            extension_namespaces.extend(
                codex_bounded_workspace_access_policy()
                    .provider_requests()
                    .observed_extensions()
                    .cloned(),
            );
        }
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
            HostServiceKind::Time,
            HostServiceKind::Process,
        ];
        if kind == CodexPreparedSessionKind::BoundedWorkspace {
            host_services.push(HostServiceKind::WorkingResource);
        }
        let requirements = requirements(
            self,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            host_services,
            capability_requirements,
        )
        .with_extension_namespaces(extension_namespaces)
        .with_session_access_policy(access_policy)
        .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let descriptor = descriptor(self);
        let plan = build_plan(self, &descriptor, &instance, Some(&route), &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?
            .with_options(options);
        Ok(CodexPreparedSession {
            kind,
            evidence: CodexPreparedEvidence::from_prepared(self, plan)?,
            request,
            management_instance: lifecycle_management_instance(self),
        })
    }
}
