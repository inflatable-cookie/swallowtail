use super::input::ClaudeAgentSessionProfileInput;
use super::plan::{
    ClaudeAgentPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use super::{ClaudeAgentPreparedSessionFuture, ClaudeAgentPreparedSessionLoadFuture};
use crate::prepared::instance::{REASONING_MODES, session_capabilities};
use crate::{ClaudeAgentAcpDriver, ClaudeAgentPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, ModelRoute, ProviderSessionBindingOrigin,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, DirectContinuationTurnRequest, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, LoadSessionRequest, LoadedSession,
    OpenSessionRequest, PreparationFailure, PreparedAccessEvidence,
    ProviderSessionManagementBinding, ResumeSessionRequest, RuntimeFailure, SessionOptions,
    SessionResumeBinding, TurnHandle, TurnRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentPreparedSession {
    evidence: ClaudeAgentPreparedEvidence,
    request: OpenSessionRequest,
    management_instance: swallowtail_core::ConfiguredInstance,
}

impl ClaudeAgentPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &ClaudeAgentPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> ClaudeAgentAcpDriver {
        self.evidence.low_level_driver()
    }

    pub fn open_session(&self, services: HostServices) -> ClaudeAgentPreparedSessionFuture {
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
                Some(request.working_resource().clone()),
                ProviderSessionBindingOrigin::Loaded,
            )
            .await?;
            Ok(LoadedSession::new(replay, handle))
        }))
    }

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
    pub fn prepare_session(
        &self,
        input: ClaudeAgentSessionProfileInput,
    ) -> Result<ClaudeAgentPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, options) = input.into_parts();
        validate_options(&options)?;
        let supports_reasoning = crate::selection::version_supports_config_options(
            self.observation().version().version(),
        );
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let capabilities =
            with_activity(session_capabilities(supports_reasoning), &activity_profile);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = requirements(self, operation_capabilities(&capabilities, &options));
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

fn with_activity(
    capabilities: CapabilityProfile,
    activity: &swallowtail_core::ObservableActivityProfile,
) -> CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    requirements.push(
        activity
            .capability_requirement()
            .expect("prepared Claude Agent activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn lifecycle_management_instance(
    prepared: &ClaudeAgentPreparedIntegration,
) -> swallowtail_core::ConfiguredInstance {
    instance_with_capabilities(
        prepared,
        CapabilityProfile::new([
            CapabilityRequirement::new(Capability::ProviderNativeSessionClose, []),
            CapabilityRequirement::new(Capability::ProviderSessionDelete, []),
        ]),
    )
}

async fn wrap_management_handle(
    handle: Box<dyn InteractiveSessionHandle>,
    instance: swallowtail_core::ConfiguredInstance,
    access: PreparedAccessEvidence,
    working_resource: Option<WorkingResourceRef>,
    origin: ProviderSessionBindingOrigin,
) -> Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure> {
    let Some(provider_ref) = handle.provider_session_ref().cloned() else {
        return Ok(handle);
    };
    match ProviderSessionManagementBinding::from_bound_session(
        provider_ref,
        &crate::claude_agent_acp_descriptor(),
        &instance,
        access,
        working_resource,
        origin,
    ) {
        Ok(binding) => Ok(Box::new(ManagedClaudeAgentSessionHandle {
            inner: handle,
            binding,
        })),
        Err(error) => {
            let _ = handle.close().await;
            Err(RuntimeFailure::new(error.diagnostic().clone()))
        }
    }
}

struct ManagedClaudeAgentSessionHandle {
    inner: Box<dyn InteractiveSessionHandle>,
    binding: ProviderSessionManagementBinding,
}

impl InteractiveSessionHandle for ManagedClaudeAgentSessionHandle {
    fn request_id(&self) -> &swallowtail_runtime::RequestId {
        self.inner.request_id()
    }

    fn session_id(&self) -> &swallowtail_runtime::RuntimeSessionId {
        self.inner.session_id()
    }

    fn provider_session_ref(&self) -> Option<&swallowtail_core::SessionRef> {
        self.inner.provider_session_ref()
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        self.inner.resume_binding()
    }

    fn management_binding(&self) -> Option<&ProviderSessionManagementBinding> {
        Some(&self.binding)
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        self.inner.start_turn(request, services)
    }

    fn start_direct_continuation_turn<'a>(
        &'a mut self,
        request: DirectContinuationTurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        self.inner.start_direct_continuation_turn(request, services)
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.inner.cancellation()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        self.inner.close()
    }
}

fn validate_options(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if options.developer_instructions().is_some() || options.tools().len() != 0 {
        return Err(failure(
            "swallowtail.claude_agent.preparation.session_options_unsupported",
            "Claude Agent ACP prepared sessions support only the portable reasoning option",
        ));
    }
    if options
        .reasoning_mode()
        .is_some_and(|mode| !REASONING_MODES.contains(&mode.as_str()))
    {
        return Err(failure(
            "swallowtail.claude_agent.preparation.reasoning_mode_unsupported",
            "Claude Agent ACP prepared session reasoning mode is unsupported",
        ));
    }
    Ok(())
}

fn reject_attachment_reasoning(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if options.reasoning_mode().is_some() {
        Err(failure(
            "swallowtail.claude_agent.preparation.attachment_reasoning_unsupported",
            "Claude Agent load and resume cannot redeclare reasoning selection",
        ))
    } else {
        Ok(())
    }
}

fn operation_capabilities(
    available: &CapabilityProfile,
    options: &SessionOptions,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = available
        .iter()
        .filter(|(capability, _)| *capability != Capability::ReasoningSelection)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(mode) = options.reasoning_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [swallowtail_core::CapabilityConstraint::ReasoningMode(
                mode.clone(),
            )],
        ));
    }
    capabilities
}
