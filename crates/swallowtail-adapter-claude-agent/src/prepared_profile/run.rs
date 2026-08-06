use super::input::{ClaudeAgentRunProfileInput, ClaudeAgentRunRetention};
use super::plan::{
    ClaudeAgentPreparedEvidence, build_plan, instance_with_capabilities, run_requirements,
};
use crate::prepared::instance::{REASONING_MODES, run_capabilities};
use crate::{ClaudeAgentAcpDriver, ClaudeAgentPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    HarnessConfigurationPosture, HarnessIsolation, ModelRoute, OwnedRemoteResourceKind,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, ProviderRetentionPolicy,
    RunHandle, RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot structured run through Claude Agent ACP.
pub struct ClaudeAgentPreparedRun {
    evidence: ClaudeAgentPreparedEvidence,
    request: StructuredRunRequest,
}

impl ClaudeAgentPreparedRun {
    /// Returns portable evidence for the prepared run.
    #[must_use]
    pub const fn evidence(&self) -> &ClaudeAgentPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound structured-run request.
    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    /// Creates the low-level ACP driver bound to this run.
    #[must_use]
    pub fn low_level_driver(&self) -> ClaudeAgentAcpDriver {
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

    /// Splits the prepared run into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ClaudeAgentPreparedEvidence,
        swallowtail_core::PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl ClaudeAgentPreparedIntegration {
    /// Prepares a structured run through the admitted ACP integration.
    pub fn prepare_run(
        &self,
        input: ClaudeAgentRunProfileInput,
    ) -> Result<ClaudeAgentPreparedRun, PreparationFailure> {
        let (
            request_id,
            model,
            content,
            working_resource,
            deadline,
            reasoning,
            permission_handling,
            retention,
        ) = input.into_parts();
        if reasoning
            .as_ref()
            .is_some_and(|mode| !REASONING_MODES.contains(&mode.as_str()))
        {
            return Err(super::plan::failure(
                "swallowtail.claude_agent.preparation.reasoning_mode_unsupported",
                "Claude Agent ACP prepared run reasoning mode is unsupported",
            ));
        }
        let supports_reasoning = crate::selection::version_supports_config_options(
            self.observation().version().version(),
        );
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let available = with_activity(run_capabilities(supports_reasoning), &activity_profile);
        let mut operation_capabilities = operation_capabilities(&available, reasoning.as_ref());
        apply_retention(&mut operation_capabilities, retention);
        let capabilities = CapabilityProfile::new(operation_capabilities.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = run_requirements(self, operation_capabilities, permission_handling);
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        );
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let provider_retention = match retention {
            ClaudeAgentRunRetention::Durable => ProviderRetentionPolicy::DurableAllowed,
            ClaudeAgentRunRetention::TemporaryWithOwnedSessionCleanup => {
                ProviderRetentionPolicy::TemporaryAllowed
            }
        };
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(provider_retention)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(ClaudeAgentPreparedRun {
            evidence: ClaudeAgentPreparedEvidence::from_prepared(self, plan, activity_profile)?,
            request,
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

fn apply_retention(
    capabilities: &mut Vec<CapabilityRequirement>,
    retention: ClaudeAgentRunRetention,
) {
    capabilities
        .retain(|requirement| requirement.capability() != Capability::ProviderDurableRetention);
    match retention {
        ClaudeAgentRunRetention::Durable => capabilities.push(CapabilityRequirement::new(
            Capability::ProviderDurableRetention,
            [],
        )),
        ClaudeAgentRunRetention::TemporaryWithOwnedSessionCleanup => {
            capabilities.push(CapabilityRequirement::new(
                Capability::ProviderTemporaryRetention,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(
                Capability::ProviderNativeSessionClose,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(
                Capability::OwnedRemoteResourceDeletion,
                [CapabilityConstraint::OwnedRemoteResource(
                    OwnedRemoteResourceKind::Session,
                )],
            ));
        }
    }
}

fn operation_capabilities(
    available: &swallowtail_core::CapabilityProfile,
    reasoning: Option<&swallowtail_core::ReasoningMode>,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = available
        .iter()
        .filter(|(capability, _)| *capability != swallowtail_core::Capability::ReasoningSelection)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(mode) = reasoning {
        capabilities.push(CapabilityRequirement::new(
            swallowtail_core::Capability::ReasoningSelection,
            [swallowtail_core::CapabilityConstraint::ReasoningMode(
                mode.clone(),
            )],
        ));
    }
    capabilities
}
