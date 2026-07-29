use super::input::QwenSessionProfileInput;
use super::plan::{QwenPreparedEvidence, build_plan, instance_with_capabilities, requirements};
use crate::{QwenHeadlessDriver, QwenPreparedIntegration};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    DriverRole, ModelRoute, OperationShape, ResourceAccess, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenSessionRequest, PreparationFailure, RuntimeFailure, SessionAccessPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QwenPreparedSession {
    evidence: QwenPreparedEvidence,
    request: OpenSessionRequest,
}

impl QwenPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &QwenPreparedEvidence {
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
    pub fn low_level_driver(&self) -> QwenHeadlessDriver {
        self.evidence.low_level_driver()
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
        QwenPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl QwenPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: QwenSessionProfileInput,
    ) -> Result<QwenPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, deadline) = input.into_parts();
        let capability_requirements = session_capabilities();
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, provider_id, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        )
        .with_provider_id(provider_id);
        let requirements = requirements(
            self,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            capability_requirements,
        )
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
        .with_session_provider_state_policy(
            SessionProviderStatePolicy::DurableProviderSessionPreserved,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, deadline)?;
        Ok(QwenPreparedSession {
            evidence: QwenPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

fn session_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(
            Capability::InteractiveSession,
            [CapabilityConstraint::MaximumTurns(24)],
        ),
        CapabilityRequirement::new(
            Capability::StreamingEvents,
            [CapabilityConstraint::StreamRecordMaximumCount(4096)],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
    ]
}
