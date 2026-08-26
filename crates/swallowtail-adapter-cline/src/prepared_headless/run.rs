use super::ClineHeadlessPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HarnessMode,
    HostServiceKind, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prompt, read-only resource, and host deadline for one Cline JSON print run.
pub struct ClineHeadlessRunProfileInput {
    request_id: RequestId,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    harness_mode: Option<HarnessMode>,
}

impl ClineHeadlessRunProfileInput {
    #[must_use]
    /// Creates an explicit print-run profile with no hidden model or auto-approve default.
    pub const fn new(
        request_id: RequestId,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            content,
            working_resource,
            deadline,
            harness_mode: None,
        }
    }

    /// Selects portable Plan for the one JSON child.
    ///
    /// Omission keeps the current argv and provider-default mode. Only
    /// `HarnessMode::Plan` is admitted.
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
/// Prepared Cline headless run with immutable plan, request, and evidence agreement.
pub struct ClineHeadlessPreparedRun {
    evidence: PreparedOperationEvidence,
    request: StructuredRunRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl ClineHeadlessPreparedIntegration {
    /// Validates and prepares one bounded JSON print run without starting provider work.
    pub fn prepare_run(
        &self,
        input: ClineHeadlessRunProfileInput,
    ) -> Result<ClineHeadlessPreparedRun, PreparationFailure> {
        let activity = super::activity::profile(self.observation())?;
        let capabilities = super::activity::with_activity(advertised_capabilities(), &activity);
        let instance = instance_with_capabilities(self.instance(), capabilities.clone());
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
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
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let descriptor = crate::cline_headless_descriptor();
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
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::Prohibited)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        match input.harness_mode {
            None => {}
            Some(HarnessMode::Plan) => {
                policy = policy.with_harness_mode(HarnessMode::Plan);
            }
            Some(_) => {
                return Err(super::failure(
                    PreparationStage::Preflight,
                    "swallowtail.cline.headless.preparation.harness_mode_unsupported",
                    "Cline headless admits only portable Plan",
                ));
            }
        }
        let request = StructuredRunRequest::new(input.request_id, input.content, policy)
            .with_working_resource(input.working_resource)
            .with_deadline(input.deadline);
        Ok(ClineHeadlessPreparedRun {
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

impl ClineHeadlessPreparedRun {
    #[must_use]
    /// Returns prepared operation and activity evidence.
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the immutable plan-derived request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Returns the caller-selected portable mode copied onto the request policy.
    pub const fn harness_mode(&self) -> Option<HarnessMode> {
        self.request.policy().harness_mode()
    }

    #[must_use]
    /// Creates the low-level headless driver as an explicit escape hatch.
    pub fn low_level_driver(&self) -> crate::ClineHeadlessDriver {
        crate::ClineHeadlessDriver::new(self.environment.clone())
    }

    /// Starts the single prepared print run.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }
}

pub(super) fn advertised_capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [CapabilityConstraint::HarnessMode(HarnessMode::Plan)],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::StructuredRun,
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

fn access_requirement(integration: &ClineHeadlessPreparedIntegration) -> AccessRequirement {
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
