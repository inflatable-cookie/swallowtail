use super::input::KimiSessionProfileInput;
use super::plan::{
    KimiPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use super::{KimiPreparedSessionFuture, KimiPreparedSessionLoadFuture};
use crate::prepared::instance::{acp_behavior, session_capabilities};
use crate::selection::KimiAcpBehavior;
use crate::{KimiAcpDriver, KimiPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, HarnessMode,
    ModelRoute, ReasoningMode,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest,
    PreparationFailure, PreparedWorkingStateRestoration,
    ProviderSessionContinuationRecoveryOutcome, RequestId, ResumeSessionRequest, RuntimeFailure,
    RuntimeTurnId, SessionOptions, SessionResumeBinding, WorkingStateRestorationMethod,
    WorkingStateRestorationOperation, WorkingStateRestorationOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared interactive Kimi Code ACP session.
pub struct KimiPreparedSession {
    evidence: KimiPreparedEvidence,
    request: OpenSessionRequest,
}

impl KimiPreparedSession {
    /// Returns portable evidence for the prepared session.
    #[must_use]
    pub const fn evidence(&self) -> &KimiPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound session-open request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Creates the low-level ACP driver bound to this session.
    #[must_use]
    pub fn low_level_driver(&self) -> KimiAcpDriver {
        self.evidence.low_level_driver()
    }

    /// Opens a new Kimi ACP session with caller-supplied host services.
    pub fn open_session(&self, services: HostServices) -> KimiPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    /// Builds an exact provider-session load request with bounded replay.
    pub fn load_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<LoadSessionRequest, PreparationFailure> {
        reject_attachment_options(self.request.options())?;
        LoadSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Kimi session binds a working resource")
                .clone(),
            None,
        )
    }

    /// Loads a retained session and returns replay plus an interactive handle.
    pub fn load_session(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<KimiPreparedSessionLoadFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.load_request(request_id, binding)?;
        Ok(Box::pin(async move {
            driver.load_session(plan, request, services).await
        }))
    }

    /// Prepares exact continuation recovery for an interrupted consumer turn.
    pub fn prepare_working_state_restoration(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        interrupted_turn_id: RuntimeTurnId,
    ) -> Result<PreparedWorkingStateRestoration, PreparationFailure> {
        let request = self.load_request(request_id, binding)?;
        Ok(PreparedWorkingStateRestoration::new(
            KimiAcpContinuationRecovery {
                driver: self.low_level_driver(),
                plan: self.plan().clone(),
                request,
                interrupted_turn_id,
            },
        ))
    }

    /// Builds an exact provider-session resume request without replay.
    pub fn resume_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        reject_attachment_options(self.request.options())?;
        ResumeSessionRequest::from_plan(
            self.plan(),
            request_id,
            binding,
            self.request
                .working_resource()
                .expect("prepared Kimi session binds a working resource")
                .clone(),
            None,
        )
    }

    /// Resumes a retained Kimi session without replaying prior content.
    pub fn resume_session(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
        services: HostServices,
    ) -> Result<KimiPreparedSessionFuture, PreparationFailure> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.resume_request(request_id, binding)?;
        Ok(Box::pin(async move {
            driver.resume_session(plan, request, services).await
        }))
    }

    /// Splits the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        KimiPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

struct KimiAcpContinuationRecovery {
    driver: KimiAcpDriver,
    plan: swallowtail_core::PreflightPlan,
    request: LoadSessionRequest,
    interrupted_turn_id: RuntimeTurnId,
}

impl WorkingStateRestorationOperation for KimiAcpContinuationRecovery {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderSessionContinuationRecovery
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let Self {
            driver,
            plan,
            request,
            interrupted_turn_id,
        } = *self;
        Box::pin(async move {
            let loaded = driver.load_session(plan, request, services).await?;
            Ok(WorkingStateRestorationOutcome::SessionRecovered(
                ProviderSessionContinuationRecoveryOutcome::new(interrupted_turn_id, loaded),
            ))
        })
    }
}

impl KimiPreparedIntegration {
    /// Prepares an interactive session through the admitted ACP integration.
    pub fn prepare_session(
        &self,
        input: KimiSessionProfileInput,
    ) -> Result<KimiPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, options) = input.into_parts();
        let behavior = acp_behavior(self.observation())?;
        validate_options(&options, behavior)?;
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let capabilities = with_activity(session_capabilities(behavior), &activity_profile);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities.clone(),
        );
        let requirements = requirements(self, operation_capabilities(&capabilities, &options));
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, None)?
            .with_options(options);
        Ok(KimiPreparedSession {
            evidence: KimiPreparedEvidence::from_prepared(self, plan, activity_profile)?,
            request,
        })
    }
}

fn operation_capabilities(
    available: &CapabilityProfile,
    options: &SessionOptions,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = available
        .iter()
        .filter(|(capability, _)| {
            !matches!(
                capability,
                Capability::ReasoningSelection | Capability::HarnessModeSelection
            )
        })
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(mode) = options.reasoning_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(mode.clone())],
        ));
    }
    if let Some(mode) = options.harness_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [CapabilityConstraint::HarnessMode(mode)],
        ));
    }
    capabilities
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
            .expect("prepared Kimi activity is available"),
    );
    CapabilityProfile::new(requirements)
}

pub(super) fn validate_options(
    options: &SessionOptions,
    behavior: KimiAcpBehavior,
) -> Result<(), PreparationFailure> {
    if options.developer_instructions().is_some() || options.tools().len() != 0 {
        return Err(failure(
            "swallowtail.kimi.preparation.session_options_unsupported",
            "Kimi prepared sessions support only portable reasoning and plan-mode options",
        ));
    }
    if let Some(mode) = options.reasoning_mode()
        && !supported_reasoning_mode(mode, behavior)
    {
        return Err(failure(
            "swallowtail.kimi.preparation.reasoning_mode_unsupported",
            "Kimi prepared session reasoning mode is unsupported",
        ));
    }
    if options
        .harness_mode()
        .is_some_and(|mode| mode != HarnessMode::Plan)
    {
        return Err(failure(
            "swallowtail.kimi.preparation.harness_mode_unsupported",
            "Kimi prepared session harness mode is unsupported",
        ));
    }
    Ok(())
}

pub(super) fn reject_attachment_options(
    options: &SessionOptions,
) -> Result<(), PreparationFailure> {
    if options.reasoning_mode().is_some() {
        return Err(failure(
            "swallowtail.kimi.preparation.attachment_reasoning_unsupported",
            "Kimi load and resume cannot redeclare reasoning selection",
        ));
    }
    if options.harness_mode().is_some() {
        return Err(failure(
            "swallowtail.kimi.preparation.attachment_harness_mode_unsupported",
            "Kimi load and resume cannot redeclare harness-mode selection",
        ));
    }
    Ok(())
}

fn supported_reasoning_mode(mode: &ReasoningMode, behavior: KimiAcpBehavior) -> bool {
    behavior
        .admitted_reasoning_modes()
        .iter()
        .any(|admitted| *admitted == mode.as_str())
}
