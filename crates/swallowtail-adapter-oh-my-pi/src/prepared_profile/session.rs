use super::OhMyPiPreparedSessionFuture;
use super::input::OhMyPiSessionProfileInput;
use super::plan::{
    OhMyPiPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use crate::prepared::instance::{reasoning_capability, session_capabilities};
use crate::{OhMyPiPreparedIntegration, OhMyPiRpcDriver};
use swallowtail_core::{CapabilityRequirement, ModelRoute};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, OpenSessionRequest, PreparationFailure,
    PreparedWorkingStateRestoration, RuntimeTurnId, SessionOptions,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OhMyPiPreparedSession {
    evidence: OhMyPiPreparedEvidence,
    request: OpenSessionRequest,
}

impl OhMyPiPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &OhMyPiPreparedEvidence {
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
    pub fn low_level_driver(&self) -> OhMyPiRpcDriver {
        self.evidence.low_level_driver()
    }

    pub fn open_session(&self, services: HostServices) -> OhMyPiPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

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

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OhMyPiPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OhMyPiPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: OhMyPiSessionProfileInput,
    ) -> Result<OhMyPiPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, deadline, options, image_attachments) =
            input.into_parts();
        validate_options(&options)?;
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let mut capabilities = session_capabilities(image_attachments);
        if let Some(mode) = options.reasoning_mode() {
            capabilities = swallowtail_core::CapabilityProfile::new(
                capabilities
                    .iter()
                    .map(|(capability, constraints)| {
                        CapabilityRequirement::new(capability, constraints.iter().cloned())
                    })
                    .chain([reasoning_capability(mode)]),
            );
        }
        let capabilities = super::activity_profile::with_activity(capabilities, &activity_profile);
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
            image_attachments,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?
            .with_options(options);
        Ok(OhMyPiPreparedSession {
            evidence: OhMyPiPreparedEvidence::from_prepared_with_activity(
                self,
                plan,
                activity_profile,
            )?,
            request,
        })
    }
}

fn validate_options(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if options.developer_instructions().is_some()
        || options.harness_mode().is_some()
        || options.tools().len() != 0
    {
        return Err(failure(
            "swallowtail.oh_my_pi.preparation.session_options_unsupported",
            "OhMyPi RPC prepared sessions support only portable reasoning selection",
        ));
    }
    if options
        .reasoning_mode()
        .is_some_and(|mode| !crate::driver::validation::reasoning_mode_supported(mode))
    {
        return Err(failure(
            "swallowtail.oh_my_pi.preparation.reasoning_mode_unsupported",
            "OhMyPi RPC reasoning mode is unsupported",
        ));
    }
    Ok(())
}
