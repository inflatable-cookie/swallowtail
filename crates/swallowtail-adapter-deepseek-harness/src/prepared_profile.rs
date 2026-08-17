use crate::prepared::DeepSeekHarnessPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis,
    ActivityKindClass, ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelRoute,
    ObservableActivityProfile, OperationRequirements, OperationShape, PreflightContext,
    PreflightPlan, ProviderId, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SupportAuthority, preflight,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit provider, model, and route selection for one harness run.
pub struct DeepSeekHarnessModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl DeepSeekHarnessModelSelection {
    #[must_use]
    /// Creates one explicit model-route selection without a provider default.
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        provider_id: ProviderId,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            provider_id,
            model_id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prompt, provider, model, read-only resource, and deadline for one run.
pub struct DeepSeekHarnessRunProfileInput {
    request_id: RequestId,
    model: DeepSeekHarnessModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
}

impl DeepSeekHarnessRunProfileInput {
    #[must_use]
    /// Creates an explicit structured-run profile with no hidden defaults.
    pub const fn new(
        request_id: RequestId,
        model: DeepSeekHarnessModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Immutable evidence for one prepared DeepSeek Harness operation.
pub struct DeepSeekHarnessPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    operation: PreparedOperationEvidence,
}

impl DeepSeekHarnessPreparedEvidence {
    fn from_prepared(
        prepared: &DeepSeekHarnessPreparedIntegration,
        plan: PreflightPlan,
        activity: ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity,
            )?,
        })
    }

    #[must_use]
    /// Returns the exact executable observation admitted during preparation.
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    /// Returns the prepared access evidence.
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    /// Returns the complete prepared-operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the admitted observable-activity profile.
    pub const fn observable_activity(&self) -> &ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    /// Returns the validated preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    #[must_use]
    /// Returns the host-approved Cordis configuration reference.
    pub const fn environment(&self) -> &swallowtail_runtime::EnvironmentRef {
        &self.environment
    }

    fn low_level_driver(&self) -> crate::DeepSeekHarnessJsonRpcDriver {
        crate::DeepSeekHarnessJsonRpcDriver::new(self.environment.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot DeepSeek Harness structured run.
pub struct DeepSeekHarnessPreparedRun {
    evidence: DeepSeekHarnessPreparedEvidence,
    request: StructuredRunRequest,
}

impl DeepSeekHarnessPreparedRun {
    #[must_use]
    /// Returns immutable evidence for the prepared operation.
    pub const fn evidence(&self) -> &DeepSeekHarnessPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the validated preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the bound structured-run request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Creates the low-level driver bound to this prepared run.
    pub fn low_level_driver(&self) -> crate::DeepSeekHarnessJsonRpcDriver {
        self.evidence.low_level_driver()
    }

    /// Starts the prepared run with caller-supplied host services.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    /// Splits the prepared run into evidence, plan, and request.
    pub fn into_parts(
        self,
    ) -> (
        DeepSeekHarnessPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl DeepSeekHarnessPreparedIntegration {
    /// Validates and prepares one exact DeepSeek Harness structured run.
    pub fn prepare_run(
        &self,
        input: DeepSeekHarnessRunProfileInput,
    ) -> Result<DeepSeekHarnessPreparedRun, PreparationFailure> {
        let activity = activity_profile(self.observation())?;
        let capabilities = with_activity(capabilities(), &activity);
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
        ])
        .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        }))
        .with_interface_versions([self.observation().version().clone()])
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let descriptor = crate::deepseek_harness_jsonrpc_descriptor();
        let context = PreflightContext::new(
            &descriptor,
            &instance,
            self.access_profile(),
            self.access_evidence().status(),
            self.available_host_services(),
        )
        .with_model_route(&route);
        let plan = preflight(&context, &requirements).map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::Prohibited)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let request = StructuredRunRequest::new(input.request_id, input.content, policy)
            .with_working_resource(input.working_resource)
            .with_deadline(input.deadline);
        Ok(DeepSeekHarnessPreparedRun {
            evidence: DeepSeekHarnessPreparedEvidence::from_prepared(self, plan, activity)?,
            request,
        })
    }
}

pub(crate) fn capabilities() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
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

pub(crate) fn advertised_capabilities() -> CapabilityProfile {
    capabilities()
}

fn activity_profile(
    observation: &swallowtail_core::InstalledExecutableObservation,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior = match observation.compatibility() {
        swallowtail_core::InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().clone()
        }
        _ => {
            return Err(failure(
                "swallowtail.deepseek_harness.preparation.activity_version_incompatible",
                "DeepSeek Harness activity requires the exact permitted runtime-bin",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            observation.version().axis().clone(),
            behavior,
        )],
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [ActivityContentStream::FinalAnswerText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
        ],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| {
        failure(
            "swallowtail.deepseek_harness.preparation.activity_profile_invalid",
            "DeepSeek Harness activity profile could not be derived",
        )
    })
}

fn with_activity(
    capabilities: CapabilityProfile,
    activity: &ObservableActivityProfile,
) -> CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .filter(|(capability, _)| *capability != Capability::ObservableActivity)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    requirements.push(
        activity
            .capability_requirement()
            .expect("prepared DeepSeek Harness activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(class, lifecycle, streams, disclosure, []).map_err(|_| {
        failure(
            "swallowtail.deepseek_harness.preparation.activity_profile_invalid",
            "DeepSeek Harness activity profile could not be derived",
        )
    })
}

fn access_requirement(integration: &DeepSeekHarnessPreparedIntegration) -> AccessRequirement {
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

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
