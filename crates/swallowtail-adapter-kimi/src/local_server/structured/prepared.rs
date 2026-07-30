use super::super::KimiLocalServerPreparedIntegration;
use std::num::NonZeroU32;
use swallowtail_core::{
    AccessRequirement, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, CredentialState, Diagnostic, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ReasoningMode,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, preflight,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparationStage, PreparedOperationEvidence, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    RequestId, RunHandle, RuntimeFailure, StreamReattachmentPolicy, StructuredRunDriver,
    StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerRunInput {
    request_id: RequestId,
    model: crate::KimiModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    reasoning: Option<ReasoningMode>,
    configuration: super::super::KimiLocalServerSessionConfiguration,
    allow_unverified_newer: bool,
    managed_recovery_accepted: bool,
    stream_reattachment: StreamReattachmentPolicy,
}

impl KimiLocalServerRunInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: crate::KimiModelSelection,
        content: OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Deadline,
        configuration: super::super::KimiLocalServerSessionConfiguration,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
            reasoning: None,
            configuration,
            allow_unverified_newer: false,
            managed_recovery_accepted: false,
            stream_reattachment: StreamReattachmentPolicy::Disabled,
        }
    }

    #[must_use]
    pub fn with_reasoning(mut self, reasoning: ReasoningMode) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    pub const fn allow_unverified_newer(mut self) -> Self {
        self.allow_unverified_newer = true;
        self
    }

    #[must_use]
    pub const fn accept_managed_recovery(mut self) -> Self {
        self.managed_recovery_accepted = true;
        self
    }

    #[must_use]
    pub const fn with_one_stream_reattachment(mut self) -> Self {
        self.stream_reattachment =
            StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"));
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiLocalServerPreparedRun {
    evidence: PreparedOperationEvidence,
    request: StructuredRunRequest,
    configuration: super::super::KimiLocalServerSessionConfiguration,
}

impl KimiLocalServerPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub const fn configuration(&self) -> &super::super::KimiLocalServerSessionConfiguration {
        &self.configuration
    }

    #[must_use]
    pub fn low_level_driver(&self) -> super::super::KimiLocalServerDriver {
        super::super::KimiLocalServerDriver::with_session_configuration(self.configuration.clone())
    }

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

impl KimiLocalServerPreparedIntegration {
    pub fn prepare_run(
        &self,
        input: KimiLocalServerRunInput,
    ) -> Result<KimiLocalServerPreparedRun, PreparationFailure> {
        if !self.server().is_qualified() && !input.allow_unverified_newer {
            return Err(failure(
                "swallowtail.kimi.local_server.preparation.run_unverified_newer",
                "Newer unverified Kimi local-server runs require explicit acceptance",
            ));
        }
        if !input.managed_recovery_accepted {
            return Err(failure(
                "swallowtail.kimi.local_server.preparation.recovery_agreement_required",
                "Kimi local-server runs require explicit managed-recovery acceptance",
            ));
        }
        super::super::interactive::validate_revision_options(self, &input.configuration)?;
        let activity_profile = super::super::activity::profile::activity_profile(self)?;
        let capabilities = super::super::activity::profile::with_activity(
            run_capabilities(input.reasoning.as_ref(), input.stream_reattachment),
            &activity_profile,
        );
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = input.model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        );
        let requirements = requirements(self, capabilities);
        let descriptor = crate::kimi_local_server_descriptor();
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
                Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_provider_recovery(ProviderRecoveryPolicy::ManagedAllowed)
            .with_stream_reattachment(input.stream_reattachment)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        if let Some(reasoning) = input.reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let request = StructuredRunRequest::new(input.request_id, input.content, policy)
            .with_working_resource(input.working_resource)
            .with_deadline(input.deadline);
        Ok(KimiLocalServerPreparedRun {
            evidence: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                self.access_evidence().clone(),
                activity_profile,
            )?,
            request,
            configuration: input.configuration.with_structured_lifecycle(
                true,
                match input.stream_reattachment {
                    StreamReattachmentPolicy::Bounded(value) => value.get(),
                    StreamReattachmentPolicy::Disabled => 0,
                },
            ),
        })
    }
}

fn run_capabilities(
    reasoning: Option<&ReasoningMode>,
    stream_reattachment: StreamReattachmentPolicy,
) -> CapabilityProfile {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(Capability::ProviderManagedRecovery, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if let Some(reasoning) = reasoning {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(reasoning.clone())],
        ));
    }
    if let StreamReattachmentPolicy::Bounded(maximum) = stream_reattachment {
        capabilities.push(CapabilityRequirement::new(
            Capability::StreamReattachment,
            [CapabilityConstraint::ReattachmentMaximumCount(
                maximum.get(),
            )],
        ));
    }
    CapabilityProfile::new(capabilities)
}

fn instance_with_capabilities(
    prepared: &KimiLocalServerPreparedIntegration,
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

fn requirements(
    prepared: &KimiLocalServerPreparedIntegration,
    capabilities: CapabilityProfile,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(
        crate::kimi_local_server_descriptor().required_host_services(DriverRole::StructuredRun),
    )
    .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
        CapabilityRequirement::new(capability, constraints.iter().cloned())
    }))
    .with_interface_versions([prepared.server().binding().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route()
}

fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
