use super::GeminiPreparedSessionFuture;
use super::input::GeminiSessionProfileInput;
use super::plan::{
    GeminiPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use crate::prepared::instance::session_capabilities;
use crate::{GeminiAcpDriver, GeminiPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ResourceAccess,
    ResourceRepresentation,
};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, OpenSessionRequest, PreparationFailure,
    PreparedWorkingStateRestoration, RuntimeTurnId, SessionOptions,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Gemini CLI ACP session ready for explicit opening.
pub struct GeminiPreparedSession {
    evidence: GeminiPreparedEvidence,
    request: OpenSessionRequest,
}

impl GeminiPreparedSession {
    /// Returns the session's preparation evidence.
    #[must_use]
    pub const fn evidence(&self) -> &GeminiPreparedEvidence {
        &self.evidence
    }

    /// Returns the immutable preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the open-session request.
    #[must_use]
    pub const fn request(&self) -> &OpenSessionRequest {
        &self.request
    }

    /// Reconstructs the low-level ACP driver from prepared evidence.
    #[must_use]
    pub fn low_level_driver(&self) -> GeminiAcpDriver {
        self.evidence.low_level_driver()
    }

    /// Opens the prepared session using the supplied host services.
    pub fn open_session(&self, services: HostServices) -> GeminiPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    /// Prepares a fresh replacement after interruption, with prior context lost.
    #[must_use]
    pub fn prepare_working_state_restoration(
        &self,
        interrupted_turn_id: RuntimeTurnId,
    ) -> PreparedWorkingStateRestoration {
        PreparedWorkingStateRestoration::fresh_session_replacement(
            interrupted_turn_id,
            self.low_level_driver(),
            self.plan().clone(),
            self.request.clone(),
        )
    }

    /// Consumes the prepared session into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        GeminiPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl GeminiPreparedIntegration {
    /// Validates and prepares an ACP session without starting provider work.
    pub fn prepare_session(
        &self,
        input: GeminiSessionProfileInput,
    ) -> Result<GeminiPreparedSession, PreparationFailure> {
        let (request_id, working_resource, options, resource_access) = input.into_parts();
        validate_options(&options, resource_access)?;
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let capabilities = session_capabilities_for(resource_access, &options, &activity_profile);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let requirements = requirements(
            self,
            capabilities.iter().map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            }),
            resource_access,
        );
        let plan = build_plan(self, &instance, &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, None)?
            .with_options(options);
        Ok(GeminiPreparedSession {
            evidence: GeminiPreparedEvidence::from_prepared(self, plan, activity_profile)?,
            request,
        })
    }
}

fn session_capabilities_for(
    resource_access: ResourceAccess,
    options: &SessionOptions,
    activity: &swallowtail_core::ObservableActivityProfile,
) -> CapabilityProfile {
    let mut capabilities = session_capabilities()
        .iter()
        .filter(|(capability, _)| {
            !matches!(
                capability,
                Capability::WorkingResource | Capability::HarnessModeSelection
            )
        })
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    capabilities.push(CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(resource_access),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    ));
    if let Some(mode) = options.harness_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [CapabilityConstraint::HarnessMode(mode)],
        ));
    }
    capabilities.push(
        activity
            .capability_requirement()
            .expect("prepared Gemini activity is available"),
    );
    CapabilityProfile::new(capabilities)
}

fn validate_options(
    options: &SessionOptions,
    resource_access: ResourceAccess,
) -> Result<(), PreparationFailure> {
    if options.developer_instructions().is_some()
        || options.reasoning_mode().is_some()
        || options.tools().len() != 0
    {
        return Err(failure(
            "swallowtail.gemini.preparation.session_options_unsupported",
            "Gemini ACP prepared sessions support only the portable plan mode option",
        ));
    }
    if options.harness_mode().is_some()
        && (options.harness_mode() != Some(swallowtail_core::HarnessMode::Plan)
            || resource_access != ResourceAccess::Read)
    {
        return Err(failure(
            "swallowtail.gemini.preparation.harness_mode_unsupported",
            "Gemini ACP plan mode requires the read-only session posture",
        ));
    }
    Ok(())
}
