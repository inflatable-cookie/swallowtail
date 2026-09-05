use crate::QwenPreparedIntegration;
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, DriverRole, EndpointAuthorization, EntitlementState,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, RuntimeReadiness,
    preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, ModelCatalogDriver, ModelCatalogRequest, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, RequestId, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Qwen model-catalogue request.
pub struct QwenCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<Deadline>,
}

impl QwenCatalogueProfileInput {
    /// Creates a catalogue request without a deadline.
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    /// Adds a catalogue discovery deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Qwen model-catalogue operation.
pub struct QwenPreparedCatalogue {
    evidence: PreparedOperationEvidence,
    request: ModelCatalogRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl QwenPreparedCatalogue {
    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound catalogue request.
    #[must_use]
    pub const fn request(&self) -> &ModelCatalogRequest {
        &self.request
    }

    /// Emits the exact Contract 061 contribution for this prepared catalogue.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: swallowtail_runtime::ConsumerRouteProjectionSourceId,
    ) -> Result<
        swallowtail_runtime::ConsumerRouteProjectionContribution,
        swallowtail_runtime::ConsumerRouteProjectionFailure,
    > {
        crate::consumer_route_projection::catalogue(self, source_id)
    }

    /// Returns portable evidence for the prepared operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    /// Starts model discovery with caller-supplied host services.
    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<swallowtail_core::ModelCatalogEntry>, RuntimeFailure>> {
        let driver = crate::QwenHeadlessDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models(plan, request, services).await })
    }
}

/// Prepares a model-catalogue request from an admitted Qwen integration.
pub fn prepare_qwen_catalogue(
    prepared: &QwenPreparedIntegration,
    input: QwenCatalogueProfileInput,
) -> Result<QwenPreparedCatalogue, PreparationFailure> {
    let capability = CapabilityRequirement::new(Capability::ModelCatalog, []);
    let instance =
        instance_with_capabilities(prepared, CapabilityProfile::new([capability.clone()]));
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::ModelCatalog,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services([HostServiceKind::Process, HostServiceKind::Time])
    .with_capabilities([capability])
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let plan = preflight(
        &PreflightContext::new(
            &crate::qwen_headless_descriptor(),
            &instance,
            prepared.access_profile(),
            prepared.access_evidence().status(),
            prepared.available_host_services(),
        ),
        &requirements,
    )
    .map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })?;
    let request = match input.deadline {
        Some(deadline) => ModelCatalogRequest::new(input.request_id).with_deadline(deadline),
        None => ModelCatalogRequest::new(input.request_id),
    };
    Ok(QwenPreparedCatalogue {
        evidence: PreparedOperationEvidence::from_plan(plan, prepared.access_evidence().clone())?,
        request,
        environment: prepared.environment().clone(),
    })
}

fn instance_with_capabilities(
    prepared: &QwenPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    let base = prepared.instance();
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
