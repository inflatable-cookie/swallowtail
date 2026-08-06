use crate::{GrokAcpDriver, GrokPreparedIntegration};
use swallowtail_core::{
    AccessRequirement, ActivityContentStream, ActivityDisclosure, ActivityInterfaceBasis,
    ActivityKindClass, ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, CredentialState, Diagnostic,
    EndpointAuthorization, EntitlementState, ExecutionLayer, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    ObservableActivityProfile, OperationRequirements, OperationShape, PreflightContext,
    PreflightPlan, ResourceAccess, RuntimeReadiness, SafeDiagnostic, SessionAccessPolicy,
    SessionProviderStatePolicy, preflight,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, PreparationStage, PreparedOperationEvidence,
    PreparedWorkingStateRestoration, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeTurnId, SessionOptions, SessionResumeBinding, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact model route for a prepared Grok operation.
pub struct GrokModelSelection {
    route_id: ModelRouteId,
    route_revision: ModelRouteRevision,
    model_id: ModelId,
}

impl GrokModelSelection {
    /// Creates an exact Grok model selection.
    #[must_use]
    pub const fn new(
        route_id: ModelRouteId,
        route_revision: ModelRouteRevision,
        model_id: ModelId,
    ) -> Self {
        Self {
            route_id,
            route_revision,
            model_id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Grok interactive session.
pub struct GrokSessionProfileInput {
    request_id: RequestId,
    model: GrokModelSelection,
    working_resource: WorkingResourceRef,
    options: SessionOptions,
}

impl GrokSessionProfileInput {
    /// Creates a Grok session profile with explicit model and options.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: GrokModelSelection,
        working_resource: WorkingResourceRef,
        options: SessionOptions,
    ) -> Self {
        Self {
            request_id,
            model,
            working_resource,
            options,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Portable evidence shared by prepared Grok sessions and runs.
pub struct GrokPreparedEvidence {
    observation: swallowtail_core::InstalledExecutableObservation,
    environment: swallowtail_runtime::EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
    operation: PreparedOperationEvidence,
}

impl GrokPreparedEvidence {
    fn from_prepared(
        prepared: &GrokPreparedIntegration,
        plan: PreflightPlan,
        activity_profile: ObservableActivityProfile,
    ) -> Result<Self, PreparationFailure> {
        Ok(Self {
            observation: prepared.observation().clone(),
            environment: prepared.environment().clone(),
            credential: prepared
                .access_profile()
                .credential_reference()
                .expect("prepared Grok access has one credential reference")
                .clone(),
            operation: PreparedOperationEvidence::from_plan_with_activity_profile(
                plan,
                prepared.access_evidence().clone(),
                activity_profile,
            )?,
        })
    }

    /// Returns the qualified installed-executable observation.
    #[must_use]
    pub const fn observation(&self) -> &swallowtail_core::InstalledExecutableObservation {
        &self.observation
    }

    /// Returns the prepared access evidence.
    #[must_use]
    pub const fn access(&self) -> &swallowtail_runtime::PreparedAccessEvidence {
        self.operation.access()
    }

    /// Returns the complete prepared-operation evidence.
    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.operation.plan()
    }

    fn low_level_driver(&self) -> GrokAcpDriver {
        GrokAcpDriver::new(self.environment.clone(), self.credential.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared interactive Grok ACP session.
pub struct GrokPreparedSession {
    evidence: GrokPreparedEvidence,
    request: OpenSessionRequest,
}

/// Future returned when a prepared Grok session is opened.
pub type GrokPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;

impl GrokPreparedSession {
    /// Returns portable evidence for the prepared session.
    #[must_use]
    pub const fn evidence(&self) -> &GrokPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound session-open request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the low-level driver bound to this session.
    #[must_use]
    pub fn low_level_driver(&self) -> GrokAcpDriver {
        self.evidence.low_level_driver()
    }

    /// Opens the prepared session with caller-supplied host services.
    pub fn open_session(&self, services: HostServices) -> GrokPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    /// Builds an exact provider-session attachment recovery request.
    pub fn attachment_recovery_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        ResumeSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Grok session binds a working resource")
                .clone(),
            None,
        )
    }

    /// Prepares bounded restoration of an interrupted turn by session attachment.
    pub fn prepare_working_state_restoration(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        interrupted_turn_id: RuntimeTurnId,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        let request = self.attachment_recovery_request(request_id, binding)?;
        Ok(
            PreparedWorkingStateRestoration::provider_session_attachment_recovery(
                interrupted_turn_id,
                self.low_level_driver(),
                self.plan().clone(),
                request,
            ),
        )
    }

    /// Splits the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(self) -> (GrokPreparedEvidence, PreflightPlan, OpenSessionRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl GrokPreparedIntegration {
    /// Prepares an interactive session from the admitted integration.
    pub fn prepare_session(
        &self,
        input: GrokSessionProfileInput,
    ) -> Result<GrokPreparedSession, PreparationFailure> {
        validate_options(&input.options)?;
        if input.model.model_id.as_str() != "grok-4.5" {
            return Err(preparation_failure(
                "swallowtail.grok.preparation.model_unsupported",
                "Grok prepared sessions require the qualified grok-4.5 model",
            ));
        }
        let activity_profile = activity_profile(self)?;
        let capabilities =
            with_activity(crate::prepared::session_capabilities(), &activity_profile);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = ModelRoute::new(
            input.model.route_id,
            input.model.route_revision,
            self.instance().id().clone(),
            input.model.model_id,
            capabilities.clone(),
        );
        let requirements = session_requirements(self, profile_requirements(&capabilities));
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request =
            OpenSessionRequest::from_plan(&plan, input.request_id, input.working_resource, None)?
                .with_options(input.options);
        Ok(GrokPreparedSession {
            evidence: GrokPreparedEvidence::from_prepared(self, plan, activity_profile)?,
            request,
        })
    }
}

include!("prepared_profile/plan.rs");
include!("prepared_profile/run.rs");
