#[path = "profile/plan.rs"]
mod plan;

use super::GeminiHeadlessPreparedIntegration;
use super::instance::run_capabilities;
use crate::headless_activity::profile::{activity_profile, with_activity};
use plan::{build_plan, instance_with_capabilities, requirements};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, HarnessConfigurationPosture,
    HarnessIsolation, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, PreflightPlan,
    ProviderId, ProviderSessionBindingOrigin, SessionRef,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationControl, CleanupOutcome, Deadline,
    HostServices, OperationContent, OperationPolicy, PreparationFailure, PreparedOperationEvidence,
    ProviderRetentionPolicy, ProviderSessionManagementBinding, RequestId, RunHandle,
    RuntimeFailure, RuntimeRunId, StructuredRunDriver, StructuredRunRequest, TerminalOutcome,
    TerminalStatus, WorkingResourceRef,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GeminiHeadlessRunRetention {
    #[default]
    Durable,
    TemporaryWithOwnedTranscriptCleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiHeadlessModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    provider_id: ProviderId,
    model_id: ModelId,
}

impl GeminiHeadlessModelSelection {
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
pub struct GeminiHeadlessRunProfileInput {
    request_id: RequestId,
    model: GeminiHeadlessModelSelection,
    content: OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Deadline,
    retention: GeminiHeadlessRunRetention,
}

impl GeminiHeadlessRunProfileInput {
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

    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn observable_activity(&self) -> &swallowtail_core::ObservableActivityProfile {
        self.operation.observable_activity()
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    fn low_level_driver(&self) -> crate::GeminiHeadlessDriver {
        crate::GeminiHeadlessDriver::new(self.environment.clone(), self.credential.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeminiHeadlessPreparedRun {
    evidence: GeminiHeadlessPreparedEvidence,
    request: StructuredRunRequest,
    management_binding: Option<ProviderSessionManagementBinding>,
}

impl GeminiHeadlessPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &GeminiHeadlessPreparedEvidence {
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
    pub fn low_level_driver(&self) -> crate::GeminiHeadlessDriver {
        self.evidence.low_level_driver()
    }

    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let binding = self.management_binding.clone();
        Box::pin(async move {
            let inner = driver.start_run(plan, request, services).await?;
            Ok(Box::new(GeminiManagedHeadlessRunHandle::new(inner, binding)) as Box<dyn RunHandle>)
        })
    }

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
        let management_binding = if retention == GeminiHeadlessRunRetention::Durable {
            Some(management_binding(self, &request_id, &working_resource)?)
        } else {
            None
        };
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
            management_binding,
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

fn management_binding(
    prepared: &GeminiHeadlessPreparedIntegration,
    request_id: &RequestId,
    working_resource: &WorkingResourceRef,
) -> Result<ProviderSessionManagementBinding, PreparationFailure> {
    let instance = instance_with_capabilities(
        prepared,
        CapabilityProfile::new([CapabilityRequirement::new(
            Capability::ProviderSessionDelete,
            [],
        )]),
    );
    let provider_ref = SessionRef::new(crate::headless::provider_session_id(request_id))
        .expect("driver-selected Gemini session id is valid");
    ProviderSessionManagementBinding::from_bound_session(
        provider_ref,
        &crate::gemini_headless_descriptor(),
        &instance,
        prepared.access_evidence().clone(),
        Some(working_resource.clone()),
        ProviderSessionBindingOrigin::Created,
    )
    .map_err(|error| {
        PreparationFailure::new(
            swallowtail_runtime::PreparationStage::Preflight,
            swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

struct ManagementBindingState {
    pending: Option<ProviderSessionManagementBinding>,
    ready: Option<ProviderSessionManagementBinding>,
}

struct GeminiManagedHeadlessRunHandle {
    inner: Box<dyn RunHandle>,
    binding: Arc<Mutex<ManagementBindingState>>,
}

impl GeminiManagedHeadlessRunHandle {
    fn new(inner: Box<dyn RunHandle>, binding: Option<ProviderSessionManagementBinding>) -> Self {
        Self {
            inner,
            binding: Arc::new(Mutex::new(ManagementBindingState {
                pending: binding,
                ready: None,
            })),
        }
    }
}

impl RunHandle for GeminiManagedHeadlessRunHandle {
    fn request_id(&self) -> &RequestId {
        self.inner.request_id()
    }

    fn run_id(&self) -> &RuntimeRunId {
        self.inner.run_id()
    }

    fn provider_run_ref(&self) -> Option<&swallowtail_core::RunRef> {
        self.inner.provider_run_ref()
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.inner.take_events()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        self.inner.take_callbacks()
    }

    fn take_management_binding(&mut self) -> Option<ProviderSessionManagementBinding> {
        self.binding
            .lock()
            .expect("Gemini management binding lock poisoned")
            .ready
            .take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.inner.cancellation()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        let terminal = self.inner.take_terminal_outcome()?;
        let binding = Arc::clone(&self.binding);
        Some(Box::pin(async move {
            let outcome = terminal.await;
            if outcome.status() == &TerminalStatus::Completed {
                let mut state = binding
                    .lock()
                    .expect("Gemini management binding lock poisoned");
                state.ready = state.pending.take();
            }
            outcome
        }))
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        self.inner.close()
    }
}
