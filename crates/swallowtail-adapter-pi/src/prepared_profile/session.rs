use super::PiPreparedSessionFuture;
use super::input::PiSessionProfileInput;
use super::plan::{
    PiPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use crate::prepared::instance::session_capabilities;
use crate::{PiPreparedIntegration, PiRpcDriver};
use swallowtail_core::{CapabilityRequirement, ModelRoute};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, OpenSessionRequest, PreparationFailure, SessionOptions,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PiPreparedSession {
    evidence: PiPreparedEvidence,
    request: OpenSessionRequest,
}

impl PiPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &PiPreparedEvidence {
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
    pub fn low_level_driver(&self) -> PiRpcDriver {
        self.evidence.low_level_driver()
    }

    pub fn open_session(&self, services: HostServices) -> PiPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        PiPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl PiPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: PiSessionProfileInput,
    ) -> Result<PiPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, deadline, options, image_attachments) =
            input.into_parts();
        validate_options(&options)?;
        let activity_profile = super::activity_profile::activity_profile(self)?;
        let capabilities = super::activity_profile::with_activity(
            session_capabilities(image_attachments),
            &activity_profile,
        );
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
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?;
        Ok(PiPreparedSession {
            evidence: PiPreparedEvidence::from_prepared_with_activity(
                self,
                plan,
                activity_profile,
            )?,
            request,
        })
    }
}

fn validate_options(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if !options.is_empty() {
        return Err(failure(
            "swallowtail.pi.preparation.session_options_unsupported",
            "Pi RPC prepared sessions do not support portable session options",
        ));
    }
    Ok(())
}
