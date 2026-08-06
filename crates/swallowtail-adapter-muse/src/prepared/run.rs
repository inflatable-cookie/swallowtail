use super::MusePreparedIntegration;
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, CredentialState, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelId,
    ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, ProviderId, ReasoningMode, ResourceAccess,
    ResourceRepresentation, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle,
    RuntimeFailure, StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

/// Exact provider ID emitted and accepted by the qualified Muse Code route.
pub const MUSE_META_PROVIDER_ID: &str = "meta";

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact provider, model, and route selection for one Muse Code run.
pub struct MuseHeadlessModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl MuseHeadlessModelSelection {
    #[must_use]
    /// Creates one explicit model-route selection.
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
/// Prompt, model, effort, read-only resource, and deadline for one run.
pub struct MuseRunProfileInput {
    request_id: RequestId,
    model: MuseHeadlessModelSelection,
    content: OperationContent,
    effort: ReasoningMode,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
}

impl MuseRunProfileInput {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Creates an explicit read-only run profile with no hidden model or effort defaults.
    pub const fn new(
        request_id: RequestId,
        model: MuseHeadlessModelSelection,
        content: OperationContent,
        effort: ReasoningMode,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            effort,
            working_resource,
            deadline,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Executable Muse Code run with immutable plan, request, and evidence agreement.
pub struct MusePreparedRun {
    evidence: PreparedOperationEvidence,
    request: StructuredRunRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl MusePreparedIntegration {
    /// Validates and prepares one exact Muse Code headless run.
    pub fn prepare_run(
        &self,
        input: MuseRunProfileInput,
    ) -> Result<MusePreparedRun, PreparationFailure> {
        validate_selection(&input.model, &input.effort)?;
        let activity = super::activity::profile(self.observation())?;
        let capabilities = super::activity::with_activity(capabilities(&input.effort), &activity);
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
        .with_harness_isolation(HarnessIsolation::ProviderEnforced)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
        .require_model_route();
        let descriptor = crate::muse_headless_descriptor();
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
        let policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::Prohibited)
            .with_harness_isolation(HarnessIsolation::ProviderEnforced)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
            .with_reasoning_mode(input.effort);
        let request = StructuredRunRequest::new(input.request_id, input.content, policy)
            .with_working_resource(input.working_resource)
            .with_deadline(input.deadline);
        Ok(MusePreparedRun {
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

impl MusePreparedRun {
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
    /// Creates the low-level exact driver as an explicit escape hatch.
    pub fn low_level_driver(&self) -> crate::MuseHeadlessDriver {
        crate::MuseHeadlessDriver::new(self.environment.clone())
    }

    /// Starts the single prepared run.
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

pub(super) fn capabilities(effort: &ReasoningMode) -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(effort.clone())],
        ),
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

pub(super) fn advertised_capabilities() -> CapabilityProfile {
    let efforts = ["none", "minimal", "low", "medium", "high", "xhigh", "ultra"]
        .into_iter()
        .map(|effort| {
            CapabilityConstraint::ReasoningMode(
                ReasoningMode::new(effort).expect("static Muse Code effort is valid"),
            )
        });
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ObservableActivity, []),
        CapabilityRequirement::new(Capability::ReasoningSelection, efforts),
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

fn validate_selection(
    model: &MuseHeadlessModelSelection,
    effort: &ReasoningMode,
) -> Result<(), PreparationFailure> {
    if model.provider_id.as_str() != MUSE_META_PROVIDER_ID
        || model.model_id.as_str() != crate::MUSE_SPARK_MODEL_ID
    {
        return Err(super::failure(
            PreparationStage::Preflight,
            "swallowtail.muse_code.preparation.model_selection_rejected",
            "Muse Code requires the exact Meta Muse Spark 1.2 selection",
        ));
    }
    if !matches!(
        effort.as_str(),
        "none" | "minimal" | "low" | "medium" | "high" | "xhigh" | "ultra"
    ) {
        return Err(super::failure(
            PreparationStage::Preflight,
            "swallowtail.muse_code.preparation.effort_rejected",
            "Muse Code effort is outside the qualified seven-level set",
        ));
    }
    Ok(())
}

fn access_requirement(integration: &MusePreparedIntegration) -> AccessRequirement {
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
