use super::input::ClaudeAgentSessionProfileInput;
use super::plan::{
    ClaudeAgentPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use super::{ClaudeAgentPreparedSessionFuture, ClaudeAgentPreparedSessionLoadFuture};
use crate::driver::{ClaudeAgentOpenObservation, ClaudeAgentOpenRejection};
use crate::prepared::instance::{REASONING_MODES, session_capabilities};
use crate::{ClaudeAgentAcpDriver, ClaudeAgentPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ModelRoute, ProviderSessionBindingOrigin,
};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, LoadSessionRequest, LoadedSession, OpenSessionRequest,
    PreparationFailure, PreparedWorkingStateRestoration, ResumeSessionRequest, RuntimeTurnId,
    SessionOptions, SessionResumeBinding,
};

mod handle;
mod helpers;
mod restoration;

use handle::wrap_management_handle;
pub(super) use helpers::{
    ClaudeAgentPreparedOpenLifecycleFuture, lifecycle_management_instance, operation_capabilities,
    reject_attachment_reasoning, validate_options, with_activity,
};
use restoration::ClaudeAgentContinuationRecovery;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared interactive Claude Agent ACP session.
pub struct ClaudeAgentPreparedSession {
    evidence: ClaudeAgentPreparedEvidence,
    request: OpenSessionRequest,
    management_instance: swallowtail_core::ConfiguredInstance,
}

impl ClaudeAgentPreparedSession {
    /// Returns portable evidence for the prepared session.
    #[must_use]
    pub const fn evidence(&self) -> &ClaudeAgentPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound session-open request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the low-level ACP driver bound to this session.
    #[must_use]
    pub fn low_level_driver(&self) -> ClaudeAgentAcpDriver {
        self.evidence.low_level_driver()
    }

    /// Opens a new provider-owned session with caller-supplied host services.
    pub fn open_session(&self, services: HostServices) -> ClaudeAgentPreparedSessionFuture {
        let lifecycle = self.open_lifecycle(services);
        Box::pin(async move {
            lifecycle
                .await
                .map(|(session, _)| session)
                .map_err(ClaudeAgentOpenRejection::into_failure)
        })
    }

    pub(crate) fn open_lifecycle(
        &self,
        services: HostServices,
    ) -> ClaudeAgentPreparedOpenLifecycleFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let management_instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Box::pin(async move {
            let (handle, observation) = driver
                .open_session_lifecycle(plan, request.clone(), services.clone())
                .await?;
            let handle = wrap_management_handle(
                handle,
                management_instance,
                access,
                request.working_resource().cloned(),
                ProviderSessionBindingOrigin::Created,
            )
            .await?;
            Ok((handle, observation))
        })
    }

    pub(crate) const fn management_instance(&self) -> &swallowtail_core::ConfiguredInstance {
        &self.management_instance
    }

    /// Builds an exact provider-session load request with bounded replay.
    pub fn load_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
    ) -> Result<LoadSessionRequest, PreparationFailure> {
        reject_attachment_reasoning(self.request.options())?;
        LoadSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Claude Agent session binds a working resource")
                .clone(),
            None,
        )
    }

    /// Loads a retained session and returns replay plus an interactive handle.
    pub fn load_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<ClaudeAgentPreparedSessionLoadFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.load_request(request_id, binding)?;
        let instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Ok(Box::pin(async move {
            let loaded = driver.load_session(plan, request.clone(), services).await?;
            let (replay, handle) = loaded.into_parts();
            let handle = wrap_management_handle(
                handle,
                instance,
                access,
                Some(
                    request
                        .working_resource()
                        .expect("prepared Claude Agent load binds a working resource")
                        .clone(),
                ),
                ProviderSessionBindingOrigin::Loaded,
            )
            .await?;
            Ok(LoadedSession::new(replay, handle))
        }))
    }

    /// Prepares exact continuation recovery for an interrupted consumer turn.
    pub fn prepare_working_state_restoration(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        interrupted_turn_id: RuntimeTurnId,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        let request = self.load_request(request_id, binding)?;
        Ok(PreparedWorkingStateRestoration::new(
            ClaudeAgentContinuationRecovery {
                driver: self.low_level_driver(),
                plan: self.plan().clone(),
                request,
                management_instance: self.management_instance.clone(),
                access: self.evidence.access().clone(),
                interrupted_turn_id,
            },
        ))
    }

    /// Builds an exact provider-session resume request without replay.
    pub fn resume_request(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        reject_attachment_reasoning(self.request.options())?;
        ResumeSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Claude Agent session binds a working resource")
                .clone(),
            None,
        )
    }

    /// Resumes a retained provider session without replaying prior content.
    pub fn resume_session(
        &self,
        request_id: swallowtail_runtime::RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<ClaudeAgentPreparedSessionFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.resume_request(request_id, binding)?;
        let instance = self.management_instance.clone();
        let access = self.evidence.access().clone();
        Ok(Box::pin(async move {
            let handle = driver
                .resume_session(plan, request.clone(), services)
                .await?;
            wrap_management_handle(
                handle,
                instance,
                access,
                Some(request.working_resource().clone()),
                ProviderSessionBindingOrigin::Resumed,
            )
            .await
        }))
    }

    /// Splits the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ClaudeAgentPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl ClaudeAgentPreparedIntegration {
    /// Prepares an interactive session through the admitted ACP integration.
    pub fn prepare_session(
        &self,
        input: ClaudeAgentSessionProfileInput,
    ) -> Result<ClaudeAgentPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, options, permission_handling) =
            input.into_parts();
        let supports_reasoning = crate::selection::version_supports_config_options(
            self.observation().version().version(),
        );
        validate_options(&options, supports_reasoning)?;
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let capabilities =
            with_activity(session_capabilities(supports_reasoning), &activity_profile);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = requirements(
            self,
            operation_capabilities(&capabilities, &options),
            permission_handling,
        );
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, None)?
            .with_options(options);
        Ok(ClaudeAgentPreparedSession {
            evidence: ClaudeAgentPreparedEvidence::from_prepared(self, plan, activity_profile)?,
            request,
            management_instance: lifecycle_management_instance(self),
        })
    }
}
