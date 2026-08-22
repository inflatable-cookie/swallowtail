use super::input::QwenRunProfileInput;
use super::plan::{QwenPreparedEvidence, build_plan, instance_with_capabilities, requirements};
use crate::activity::profile::{activity_profile, with_activity};
use crate::prepared::instance::run_capabilities;
use crate::{QwenHeadlessDriver, QwenPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    HarnessConfigurationPosture, HarnessIsolation, ModelRoute,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, ProviderRetentionPolicy,
    RunHandle, RuntimeFailure, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot Qwen structured run.
pub struct QwenPreparedRun {
    evidence: QwenPreparedEvidence,
    request: StructuredRunRequest,
}

impl QwenPreparedRun {
    /// Returns portable evidence for the prepared run.
    #[must_use]
    pub const fn evidence(&self) -> &QwenPreparedEvidence {
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

    /// Creates the low-level driver bound to this run.
    #[must_use]
    pub fn low_level_driver(&self) -> QwenHeadlessDriver {
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
        QwenPreparedEvidence,
        swallowtail_core::PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl QwenPreparedIntegration {
    /// Prepares a structured run from the admitted integration.
    pub fn prepare_run(
        &self,
        input: QwenRunProfileInput,
    ) -> Result<QwenPreparedRun, PreparationFailure> {
        let (request_id, model, content, working_resource, deadline, reasoning) =
            input.into_parts();
        let activity = activity_profile(self)?;
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
        if let Some(reasoning) = reasoning.as_ref() {
            crate::reasoning::validate_preparation(
                self.observation().version().version(),
                &provider_id,
                &model_id,
                reasoning,
            )?;
        }
        let mut capability_requirements = run_capabilities()
            .iter()
            .map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            })
            .collect::<Vec<_>>();
        if let Some(reasoning) = reasoning.as_ref() {
            capability_requirements.push(CapabilityRequirement::new(
                Capability::ReasoningSelection,
                [CapabilityConstraint::ReasoningMode(reasoning.clone())],
            ));
        }
        let capabilities =
            with_activity(CapabilityProfile::new(capability_requirements), &activity);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        )
        .with_provider_id(provider_id);
        let requirements = requirements(
            self,
            swallowtail_core::OperationShape::StructuredRun,
            swallowtail_core::DriverRole::StructuredRun,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        if let Some(reasoning) = reasoning.clone() {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline);
        Ok(QwenPreparedRun {
            evidence: QwenPreparedEvidence::from_prepared(self, plan, activity, reasoning)?,
            request,
        })
    }
}
