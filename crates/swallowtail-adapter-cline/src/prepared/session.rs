use super::ClinePreparedIntegration;
use crate::driver::{ClineOpenObservation, ClineOpenRejection};
use swallowtail_core::{
    AccessRequirement, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HarnessMode,
    HostServiceKind, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionProviderStatePolicy,
    SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionHandle, OpenSessionRequest, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, PreparedWorkingStateRestoration, RequestId,
    RuntimeFailure, RuntimeTurnId, SessionAccessPolicy, SessionOptions, WorkingResourceRef,
};

pub(crate) type ClinePreparedOpenLifecycleFuture = BoxFuture<
    'static,
    Result<(Box<dyn InteractiveSessionHandle>, ClineOpenObservation), ClineOpenRejection>,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Working resource for one bounded Cline ACP session.
pub struct ClineSessionProfileInput {
    request_id: RequestId,
    working_resource: WorkingResourceRef,
    harness_mode: Option<HarnessMode>,
}

impl ClineSessionProfileInput {
    #[must_use]
    /// Creates read-only session input with no open deadline.
    pub const fn new(request_id: RequestId, working_resource: WorkingResourceRef) -> Self {
        Self {
            request_id,
            working_resource,
            harness_mode: None,
        }
    }

    /// Selects portable Plan for the ACP session.
    ///
    /// Omission keeps the current initialize/`session/new` path and sends no
    /// mode request. Only `HarnessMode::Plan` is admitted.
    #[must_use]
    pub const fn with_harness_mode(mut self, harness_mode: HarnessMode) -> Self {
        self.harness_mode = Some(harness_mode);
        self
    }

    /// Returns the caller-selected portable mode, if any.
    #[must_use]
    pub const fn harness_mode(&self) -> Option<HarnessMode> {
        self.harness_mode
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Cline ACP session ready for initialize plus one bounded prompt.
pub struct ClinePreparedSession {
    evidence: PreparedOperationEvidence,
    request: OpenSessionRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl ClinePreparedIntegration {
    /// Validates and prepares one read-only ACP session without starting provider work.
    pub fn prepare_session(
        &self,
        input: ClineSessionProfileInput,
    ) -> Result<ClinePreparedSession, PreparationFailure> {
        let activity = super::activity::profile(self.observation())?;
        let instance_capabilities =
            super::activity::with_activity(advertised_capabilities(), &activity);
        let instance = instance_with_capabilities(self.instance(), instance_capabilities.clone());
        let (operation_capabilities, options) = match input.harness_mode {
            None => (
                without_harness_mode_selection(&instance_capabilities),
                SessionOptions::default(),
            ),
            Some(HarnessMode::Plan) => (
                instance_capabilities.clone(),
                SessionOptions::default().with_harness_mode(HarnessMode::Plan),
            ),
            Some(_) => {
                return Err(super::failure(
                    PreparationStage::Preflight,
                    "swallowtail.cline.acp.preparation.harness_mode_unsupported",
                    "Cline ACP admits only portable Plan",
                ));
            }
        };
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
        .with_capabilities(
            operation_capabilities
                .iter()
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                }),
        )
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
        let descriptor = crate::cline_acp_descriptor();
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
            OpenSessionRequest::from_plan(&plan, input.request_id, input.working_resource, None)?
                .with_options(options);
        Ok(ClinePreparedSession {
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

impl ClinePreparedSession {
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

    #[must_use]
    /// Returns the caller-selected portable mode copied onto session options.
    pub const fn harness_mode(&self) -> Option<HarnessMode> {
        self.request.options().harness_mode()
    }

    /// Opens the prepared ACP session: initialize, `session/new`, and optional
    /// Plan confirmation before the first prompt.
    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let lifecycle = self.open_lifecycle(services);
        Box::pin(async move {
            lifecycle
                .await
                .map(|(session, _)| session)
                .map_err(ClineOpenRejection::into_failure)
        })
    }

    pub(crate) fn open_lifecycle(
        &self,
        services: HostServices,
    ) -> ClinePreparedOpenLifecycleFuture {
        let driver = crate::ClineAcpDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session_lifecycle(plan, request, services).await })
    }

    #[must_use]
    /// Prepares a context-losing replacement after an interrupted turn.
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::fresh_session_replacement(
            interrupted_turn_id,
            crate::ClineAcpDriver::new(self.environment.clone()),
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
            Capability::HarnessModeSelection,
            [CapabilityConstraint::HarnessMode(HarnessMode::Plan)],
        ),
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

fn without_harness_mode_selection(capabilities: &CapabilityProfile) -> CapabilityProfile {
    CapabilityProfile::new(
        capabilities
            .iter()
            .filter(|(capability, _)| *capability != Capability::HarnessModeSelection)
            .map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
    )
}

fn access_requirement(integration: &ClinePreparedIntegration) -> AccessRequirement {
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
