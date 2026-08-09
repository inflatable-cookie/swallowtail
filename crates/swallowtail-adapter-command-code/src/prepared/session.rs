use super::{CommandCodeHeadlessModelSelection, CommandCodePreparedIntegration};
use swallowtail_core::{
    AccessRequirement, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, ModelRoute, OperationRequirements, OperationShape, PreflightContext,
    PreflightPlan, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparationStage, PreparedOperationEvidence,
    PreparedWorkingStateRestoration, RequestId, RuntimeFailure, RuntimeTurnId, SessionAccessPolicy,
    WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model, working resource, and deadline for durable interactive continuity.
pub struct CommandCodeSessionProfileInput {
    request_id: RequestId,
    model: CommandCodeHeadlessModelSelection,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
}

impl CommandCodeSessionProfileInput {
    #[must_use]
    /// Creates interactive session input with no open deadline.
    pub const fn new(
        request_id: RequestId,
        model: CommandCodeHeadlessModelSelection,
        working_resource: WorkingResourceRef,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            deadline: None,
        }
    }

    #[must_use]
    /// Adds the session-open deadline.
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Executable durable interactive session with exact plan and environment agreement.
pub struct CommandCodePreparedSession {
    evidence: PreparedOperationEvidence,
    request: OpenSessionRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl CommandCodePreparedIntegration {
    /// Validates and prepares one read-only durable provider session.
    pub fn prepare_session(
        &self,
        input: CommandCodeSessionProfileInput,
    ) -> Result<CommandCodePreparedSession, PreparationFailure> {
        let activity = super::activity::profile(self.observation())?;
        let capabilities = super::activity::with_activity(session_capabilities(), &activity);
        let instance = instance_with_capabilities(self.instance(), capabilities.clone());
        let route = ModelRoute::new(
            input.model.route_id,
            input.model.route_revision,
            instance.id().clone(),
            input.model.model_id,
            capabilities.clone(),
        )
        .with_provider_id(input.model.provider_id);
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
            HostServiceKind::Time,
            HostServiceKind::WorkingResource,
        ])
        .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }))
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        )
        .require_model_route();
        let descriptor = crate::command_code_headless_descriptor();
        let mut context = PreflightContext::new(
            &descriptor,
            &instance,
            self.access_profile(),
            self.access_evidence().status(),
            self.available_host_services(),
        );
        context = context.with_model_route(&route);
        let plan = preflight(&context, &requirements).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let request = OpenSessionRequest::from_plan(
            &plan,
            input.request_id,
            input.working_resource,
            input.deadline,
        )?;
        Ok(CommandCodePreparedSession {
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

impl CommandCodePreparedSession {
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

    /// Opens the prepared durable interactive session.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = crate::CommandCodeHeadlessDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    /// Prepares context-losing replacement after an interrupted turn.
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::fresh_session_replacement(
            interrupted_turn_id,
            crate::CommandCodeHeadlessDriver::new(self.environment.clone()),
            self.plan().clone(),
            self.request.clone(),
        )
    }
}

fn session_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [CapabilityConstraint::MaximumTurns(24)],
        ),
        CapabilityRequirement::new(
            Capability::StreamingEvents,
            [CapabilityConstraint::StreamRecordMaximumCount(4096)],
        ),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
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

fn access_requirement(integration: &CommandCodePreparedIntegration) -> AccessRequirement {
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
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}
