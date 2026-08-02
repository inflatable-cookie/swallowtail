use super::input::KimiSessionProfileInput;
use super::plan::{
    KimiPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use super::{KimiPreparedSessionFuture, KimiPreparedSessionLoadFuture};
use crate::prepared::instance::session_capabilities;
use crate::{KimiAcpDriver, KimiPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ModelRoute,
    ReasoningMode,
};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest,
    PreparationFailure, RequestId, ResumeSessionRequest, SessionOptions, SessionResumeBinding,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiPreparedSession {
    evidence: KimiPreparedEvidence,
    request: OpenSessionRequest,
}

impl KimiPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &KimiPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> KimiAcpDriver {
        self.evidence.low_level_driver()
    }

    pub fn open_session(&self, services: HostServices) -> KimiPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    pub fn load_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<LoadSessionRequest, PreparationFailure> {
        reject_attachment_reasoning(self.request.options())?;
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

    pub fn resume_request(
        &self,
        request_id: RequestId,
        binding: SessionResumeBinding,
    ) -> Result<ResumeSessionRequest, PreparationFailure> {
        reject_attachment_reasoning(self.request.options())?;
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

impl KimiPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: KimiSessionProfileInput,
    ) -> Result<KimiPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, options) = input.into_parts();
        validate_options(&options)?;
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let capabilities = with_activity(session_capabilities(), &activity_profile);
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
        .filter(|(capability, _)| *capability != Capability::ReasoningSelection)
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

pub(super) fn validate_options(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if options.developer_instructions().is_some() || options.tools().len() != 0 {
        return Err(failure(
            "swallowtail.kimi.preparation.session_options_unsupported",
            "Kimi prepared sessions support only the portable reasoning option",
        ));
    }
    if let Some(mode) = options.reasoning_mode()
        && !supported_reasoning_mode(mode)
    {
        return Err(failure(
            "swallowtail.kimi.preparation.reasoning_mode_unsupported",
            "Kimi prepared session reasoning mode is unsupported",
        ));
    }
    Ok(())
}

pub(super) fn reject_attachment_reasoning(
    options: &SessionOptions,
) -> Result<(), PreparationFailure> {
    if options.reasoning_mode().is_some() {
        Err(failure(
            "swallowtail.kimi.preparation.attachment_reasoning_unsupported",
            "Kimi load and resume cannot redeclare reasoning selection",
        ))
    } else {
        Ok(())
    }
}

fn supported_reasoning_mode(mode: &ReasoningMode) -> bool {
    matches!(mode.as_str(), "off" | "on" | "low" | "medium" | "high")
}
