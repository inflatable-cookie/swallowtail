use super::super::mcp::OpenCodeAcpStdioMcpServer;
use super::OpenCodeAcpPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionProviderStatePolicy,
    SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparationStage, PreparedOperationEvidence,
    PreparedWorkingStateRestoration, RequestId, RuntimeFailure, RuntimeTurnId, SessionAccessPolicy,
    WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Working resource for one bounded OpenCode ACP session.
pub struct OpenCodeAcpSessionProfileInput {
    request_id: RequestId,
    working_resource: WorkingResourceRef,
    stdio_mcp: Option<OpenCodeAcpStdioMcpServer>,
}

impl OpenCodeAcpSessionProfileInput {
    #[must_use]
    /// Creates read-only session input with no open deadline.
    pub const fn new(request_id: RequestId, working_resource: WorkingResourceRef) -> Self {
        Self {
            request_id,
            working_resource,
            stdio_mcp: None,
        }
    }

    /// Binds one admitted stdio MCP declaration to this session.
    #[must_use]
    pub fn with_stdio_mcp_server(mut self, server: OpenCodeAcpStdioMcpServer) -> Self {
        self.stdio_mcp = Some(server);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared OpenCode ACP session ready for initialize plus one bounded prompt.
pub struct OpenCodeAcpPreparedSession {
    evidence: PreparedOperationEvidence,
    request: OpenSessionRequest,
    environment: swallowtail_runtime::EnvironmentRef,
    stdio_mcp: Option<OpenCodeAcpStdioMcpServer>,
}

impl OpenCodeAcpPreparedIntegration {
    /// Validates and prepares one read-only ACP session without starting provider work.
    pub fn prepare_session(
        &self,
        input: OpenCodeAcpSessionProfileInput,
    ) -> Result<OpenCodeAcpPreparedSession, PreparationFailure> {
        let activity = super::activity::profile(self.observation())?;
        let capabilities = super::activity::with_activity(advertised_capabilities(), &activity);
        let instance = instance_with_capabilities(self.instance(), capabilities.clone());
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            self.instance().execution_host_id().clone(),
            access_requirement(self),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services([
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ])
        .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }))
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
        let descriptor = crate::opencode_acp_descriptor();
        let context = PreflightContext::new(
            &descriptor,
            &instance,
            self.access_profile(),
            self.access_evidence().status(),
            self.available_host_services(),
        );
        let plan = preflight(&context, &requirements).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let request =
            OpenSessionRequest::from_plan(&plan, input.request_id, input.working_resource, None)?;
        Ok(OpenCodeAcpPreparedSession {
            evidence: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                self.access_evidence().clone(),
                activity,
            )?,
            request,
            environment: self.environment().clone(),
            stdio_mcp: input.stdio_mcp,
        })
    }
}

impl OpenCodeAcpPreparedSession {
    #[must_use]
    /// Returns prepared operation and activity evidence.
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable interactive preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived session-open request.
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Opens the prepared ACP session: initialize plus `session/new`.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let mut driver = crate::OpenCodeAcpDriver::new(self.environment.clone());
        if let Some(server) = self.stdio_mcp.clone() {
            driver = match driver.with_stdio_mcp_server(server) {
                Ok(driver) => driver,
                Err(error) => return Box::pin(async move { Err(error) }),
            };
        }
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    /// Prepares a context-losing replacement after an interrupted turn.
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::fresh_session_replacement(
            interrupted_turn_id,
            crate::OpenCodeAcpDriver::new(self.environment.clone())
                .with_prepared_stdio_mcp(self.stdio_mcp.clone()),
            self.plan().clone(),
            self.request.clone(),
        )
    }
}

pub(super) fn advertised_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ])
}

fn access_requirement(integration: &OpenCodeAcpPreparedIntegration) -> AccessRequirement {
    AccessRequirement::new(integration.access_profile().id().clone())
        .with_credential_states([CredentialState::NotRequired])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported])
}

fn instance_with_capabilities(
    base: &ConfiguredInstance,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(base, capabilities)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}
