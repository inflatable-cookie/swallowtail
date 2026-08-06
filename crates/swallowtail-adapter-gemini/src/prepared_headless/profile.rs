#[path = "profile/plan.rs"]
mod plan;

use super::GeminiHeadlessPreparedIntegration;
use super::instance::run_capabilities;
use crate::headless_activity::profile::{activity_profile, with_activity};
use plan::{build_plan, instance_with_capabilities, requirements};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, HarnessConfigurationPosture,
    HarnessIsolation, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, PreflightPlan,
    ProviderId,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, OperationContent, OperationPolicy, PreparationFailure,
    PreparedOperationEvidence, ProviderRetentionPolicy, RequestId, RunHandle, RuntimeFailure,
    StructuredRunDriver, StructuredRunRequest, WorkingResourceRef,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
/// Provider transcript-retention posture for one headless run.
pub enum GeminiHeadlessRunRetention {
    /// Allows the provider-owned transcript to remain after completion.
    #[default]
    Durable,
    /// Requests a temporary transcript and cleanup by this adapter.
    TemporaryWithOwnedTranscriptCleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact provider and model-route identity for a headless run.
pub struct GeminiHeadlessModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl GeminiHeadlessModelSelection {
    /// Creates one explicit Gemini model selection without choosing defaults.
    #[must_use]
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

    fn into_parts(self) -> (ModelRouteId, ModelRouteRevision, ProviderId, ModelId) {
        (
            self.route_id,
            self.route_revision,
            self.provider_id,
            self.model_id,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for preparing one bounded Gemini CLI headless run.
pub struct GeminiHeadlessRunProfileInput {
    request_id: RequestId,
    model: GeminiHeadlessModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    retention: GeminiHeadlessRunRetention,
}

impl GeminiHeadlessRunProfileInput {
    /// Creates a durable-transcript run profile.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: GeminiHeadlessModelSelection,
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
            retention: GeminiHeadlessRunRetention::Durable,
        }
    }

    /// Changes the profile to adapter-owned temporary transcript cleanup.
    #[must_use]
    pub const fn with_owned_transcript_cleanup(mut self) -> Self {
        self.retention = GeminiHeadlessRunRetention::TemporaryWithOwnedTranscriptCleanup;
        self
    }

    fn into_parts(
        self,
    ) -> (
        RequestId,
        GeminiHeadlessModelSelection,
        OperationContent,
        WorkingResourceRef,
        Deadline,
        GeminiHeadlessRunRetention,
    ) {
        (
            self.request_id,
            self.model,
            self.content,
            self.working_resource,
            self.deadline,
            self.retention,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Immutable executable, access, activity, and preflight evidence for a run.
pub struct GeminiHeadlessPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
    operation: PreparedOperationEvidence,
}

impl GeminiHeadlessPreparedEvidence {
    fn from_prepared(
        prepared: &GeminiHeadlessPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: swallowtail_core::ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            credential: prepared
                .access_profile()
                .credential_reference()
                .expect("prepared Gemini headless access has one credential reference")
                .clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    /// Returns the qualified Gemini CLI observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the admitted access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the observable activity contract selected for the run.
    #[must_use]
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    fn low_level_driver(&self) -> crate::GeminiHeadlessDriver {
        crate::GeminiHeadlessDriver::new(self.environment.clone(), self.credential.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Gemini headless run ready for explicit dispatch.
pub struct GeminiHeadlessPreparedRun {
    evidence: GeminiHeadlessPreparedEvidence,
    request: StructuredRunRequest,
}

impl GeminiHeadlessPreparedRun {
    /// Returns the run's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &GeminiHeadlessPreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the structured run request.
    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    /// Reconstructs the low-level driver from prepared evidence.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::GeminiHeadlessDriver {
        self.evidence.low_level_driver()
    }

    /// Starts the prepared run using the supplied host services.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    /// Consumes the prepared run into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        GeminiHeadlessPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl GeminiHeadlessPreparedIntegration {
    /// Validates and prepares a run without starting provider work.
    pub fn prepare_run(
        &self,
        input: GeminiHeadlessRunProfileInput,
    ) -> Result<GeminiHeadlessPreparedRun, PreparationFailure> {
        let (request_id, model, content, working_resource, deadline, retention) =
            input.into_parts();
        let activity = activity_profile(self)?;
        let capabilities = with_activity(run_capabilities_for(retention), &activity);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
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
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let provider_retention = match retention {
            GeminiHeadlessRunRetention::Durable => ProviderRetentionPolicy::DurableAllowed,
            GeminiHeadlessRunRetention::TemporaryWithOwnedTranscriptCleanup => {
                ProviderRetentionPolicy::TemporaryAllowed
            }
        };
        let policy = OperationPolicy::offline()
            .with_provider_retention(provider_retention)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_working_resource(working_resource)
            .with_deadline(deadline);
        Ok(GeminiHeadlessPreparedRun {
            evidence: GeminiHeadlessPreparedEvidence::from_prepared(self, plan, activity)?,
            request,
        })
    }
}

fn run_capabilities_for(retention: GeminiHeadlessRunRetention) -> CapabilityProfile {
    let mut capabilities = run_capabilities()
        .iter()
        .filter(|(capability, _)| *capability != Capability::ProviderDurableRetention)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    match retention {
        GeminiHeadlessRunRetention::Durable => capabilities.push(CapabilityRequirement::new(
            Capability::ProviderDurableRetention,
            [],
        )),
        GeminiHeadlessRunRetention::TemporaryWithOwnedTranscriptCleanup => {
            capabilities.push(CapabilityRequirement::new(
                Capability::ProviderTemporaryRetention,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(
                Capability::OwnedRemoteResourceDeletion,
                [swallowtail_core::CapabilityConstraint::OwnedRemoteResource(
                    swallowtail_core::OwnedRemoteResourceKind::Session,
                )],
            ));
        }
    }
    CapabilityProfile::new(capabilities)
}
