use super::plan::{CodexPreparedEvidence, failure};
use super::{CodexPreparedSessionFuture, CodexPreparedSessionKind, CodexPreparedSessionLoadFuture};
use crate::CodexAppServerDriver;
use swallowtail_core::{PreflightPlan, ProviderSessionBindingOrigin};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest,
    PreparationFailure, SessionResumeBinding,
};

#[path = "session/management_handle.rs"]
mod management_handle;
use management_handle::{
    lifecycle_management_instance, validate_management_context, wrap_management_handle,
};
#[path = "session/preparation.rs"]
mod preparation;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared interactive Codex app-server session.
pub struct CodexPreparedSession {
    kind: CodexPreparedSessionKind,
    evidence: CodexPreparedEvidence,
    request: OpenSessionRequest,
    management_instance: Option<swallowtail_core::ConfiguredInstance>,
}

impl CodexPreparedSession {
    /// Returns the admitted session access posture.
    #[must_use]
    pub const fn kind(&self) -> CodexPreparedSessionKind {
        self.kind
    }

    /// Returns portable evidence for the prepared session.
    #[must_use]
    pub const fn evidence(&self) -> &CodexPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound session-open request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the low-level app-server driver bound to this session.
    #[must_use]
    pub fn low_level_driver(&self) -> CodexAppServerDriver {
        CodexAppServerDriver::new(self.evidence.environment().clone())
    }

    /// Opens a new provider thread with caller-supplied host services.
    pub fn open_session(&self, services: HostServices) -> CodexPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            validate_management_context(management_instance.as_ref(), &access)?;
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

    /// Splits the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(self) -> (CodexPreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }

    /// Builds an exact provider-thread resume request without replay.
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

    /// Builds an exact provider-thread load request with bounded replay.
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

    /// Loads a retained thread and returns replay plus an interactive handle.
    pub fn load_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<CodexPreparedSessionLoadFuture, PreparationFailure> {
        let request = self.load_request(request_id, binding)?;
        Ok(self.clone().load_prepared_session(request, services))
    }

    pub(crate) fn load_prepared_session(
        self,
        request: LoadSessionRequest,
        services: HostServices,
    ) -> CodexPreparedSessionLoadFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            validate_management_context(management_instance.as_ref(), &access)?;
            let loaded = driver.load_session(plan, request.clone(), services).await?;
            let (replay, handle) = loaded.into_parts();
            let handle = wrap_management_handle(
                handle,
                management_instance,
                access,
                Some(
                    request
                        .working_resource()
                        .expect("prepared Codex load binds a working resource")
                        .clone(),
                ),
                ProviderSessionBindingOrigin::Loaded,
            )
            .await?;
            Ok(swallowtail_runtime::LoadedSession::new(replay, handle))
        })
    }

    /// Resumes a retained thread without replaying prior transcript content.
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
            validate_management_context(management_instance.as_ref(), &access)?;
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

impl crate::CodexPreparedIntegration {
    /// Prepares a read-only app-server session.
    pub fn prepare_read_only_session(
        &self,
        input: super::input::CodexSessionProfileInput,
    ) -> Result<CodexPreparedSession, PreparationFailure> {
        preparation::prepare_session(self, CodexPreparedSessionKind::ReadOnly, input)
    }

    /// Prepares an app-server session admitted to one bounded writable workspace.
    pub fn prepare_bounded_workspace_session(
        &self,
        input: super::input::CodexSessionProfileInput,
    ) -> Result<CodexPreparedSession, PreparationFailure> {
        preparation::prepare_session(self, CodexPreparedSessionKind::BoundedWorkspace, input)
    }
}
