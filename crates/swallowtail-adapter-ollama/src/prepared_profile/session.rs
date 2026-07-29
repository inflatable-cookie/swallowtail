use super::input::OllamaSessionProfileInput;
use super::plan::{
    OllamaPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::{OllamaNativeAttachedDriver, OllamaPreparedIntegration};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    DriverRole, OperationShape, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, RuntimeFailure, SessionAccessPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OllamaPreparedSession {
    evidence: OllamaPreparedEvidence,
    request: OpenSessionRequest,
}

impl OllamaPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &OllamaPreparedEvidence {
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
    pub fn low_level_driver(&self) -> OllamaNativeAttachedDriver {
        OllamaNativeAttachedDriver::new()
    }

    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        OllamaPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OllamaPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: OllamaSessionProfileInput,
    ) -> Result<OllamaPreparedSession, PreparationFailure> {
        let (request_id, deadline) = input.into_parts();
        let capability_requirements = session_capabilities();
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, self.model_selection().clone(), capabilities);
        let requirements = requirements(
            self,
            &route,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            capability_requirements,
        )
        .with_session_access_policy(SessionAccessPolicy::resource_free())
        .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request = OpenSessionRequest::resource_free_from_plan(&plan, request_id, deadline)?;
        Ok(OllamaPreparedSession {
            evidence: OllamaPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

fn session_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [
                CapabilityConstraint::MaximumTurns(24),
                CapabilityConstraint::PrivateHistoryMaximumBytes(1_048_576),
            ],
        ),
        CapabilityRequirement::new(
            Capability::StreamingEvents,
            [CapabilityConstraint::StreamRecordMaximumCount(4096)],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::OutputTokenLimit,
            [CapabilityConstraint::OutputTokenMaximum(8)],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
    ]
}
