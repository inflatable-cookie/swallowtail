use super::CursorPreparedAcpIntegration;
use swallowtail_core::{
    CapabilityRequirement, DriverRole, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, OperationRequirements, OperationShape, PreflightPlan,
    ResourceAccess, SessionAccessPolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparedOperationEvidence,
    PreparedWorkingStateRestoration, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeTurnId, SessionResumeBinding, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorAcpSessionProfileInput {
    request_id: RequestId,
    working_resource: WorkingResourceRef,
}

impl CursorAcpSessionProfileInput {
    #[must_use]
    pub const fn new(request_id: RequestId, working_resource: WorkingResourceRef) -> Self {
        Self {
            request_id,
            working_resource,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorPreparedAcpSession {
    evidence: PreparedOperationEvidence,
    request: OpenSessionRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl CursorPreparedAcpIntegration {
    pub fn prepare_session(
        &self,
        input: CursorAcpSessionProfileInput,
    ) -> Result<CursorPreparedAcpSession, PreparationFailure> {
        let activity = super::activity::acp(self.observation())?;
        let capabilities =
            super::activity::with_activity(super::plan::acp_capabilities(), &activity);
        let instance =
            super::plan::instance_with_capabilities(self.instance(), capabilities.clone());
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            self.instance().execution_host_id().clone(),
            super::plan::access_requirement(self.access_profile()),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ])
        .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }))
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(
            ResourceAccess::ReadWrite,
        ))
        .with_session_provider_state_policy(super::plan::ACP_PROVIDER_STATE);
        let plan = super::plan::build_plan(
            &crate::cursor_acp_descriptor(),
            &instance,
            self.access_profile(),
            self.access_evidence(),
            self.available_host_services(),
            &requirements,
            None,
        )?;
        let request =
            OpenSessionRequest::from_plan(&plan, input.request_id, input.working_resource, None)?;
        Ok(CursorPreparedAcpSession {
            evidence: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                self.access_evidence().clone(),
                activity,
            )?,
            request,
            environment: self.environment().clone(),
        })
    }
}

impl CursorPreparedAcpSession {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
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
    pub fn low_level_driver(&self) -> crate::CursorAcpDriver {
        crate::CursorAcpDriver::new(self.environment.clone())
    }

    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    pub fn attachment_recovery_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        ResumeSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Cursor session binds a working resource")
                .clone(),
            None,
        )
    }

    pub fn prepare_working_state_restoration(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        interrupted_turn_id: RuntimeTurnId,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        let request = self.attachment_recovery_request(request_id, binding)?;
        Ok(
            PreparedWorkingStateRestoration::provider_session_attachment_recovery(
                interrupted_turn_id,
                self.low_level_driver(),
                self.plan().clone(),
                request,
            ),
        )
    }
}
