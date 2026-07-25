use super::ClaudeAgentPreparedSessionFuture;
use super::input::ClaudeAgentSessionProfileInput;
use super::plan::{
    ClaudeAgentPreparedEvidence, build_plan, failure, instance_with_capabilities, requirements,
};
use crate::prepared::instance::session_capabilities;
use crate::{ClaudeAgentAcpDriver, ClaudeAgentPreparedIntegration};
use swallowtail_core::{CapabilityRequirement, ModelRoute};
use swallowtail_runtime::{
    HostServices, InteractiveSessionDriver, OpenSessionRequest, PreparationFailure, SessionOptions,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaudeAgentPreparedSession {
    evidence: ClaudeAgentPreparedEvidence,
    request: OpenSessionRequest,
}

impl ClaudeAgentPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &ClaudeAgentPreparedEvidence {
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
    pub fn low_level_driver(&self) -> ClaudeAgentAcpDriver {
        self.evidence.low_level_driver()
    }

    pub fn open_session(&self, services: HostServices) -> ClaudeAgentPreparedSessionFuture {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.open_session(plan, request, services).await })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ClaudeAgentPreparedEvidence,
        swallowtail_core::PreflightPlan,
        OpenSessionRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl ClaudeAgentPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: ClaudeAgentSessionProfileInput,
    ) -> Result<ClaudeAgentPreparedSession, PreparationFailure> {
        let (request_id, model, working_resource, options) = input.into_parts();
        validate_options(&options)?;
        let capabilities = session_capabilities();
        let instance = instance_with_capabilities(self, capabilities.clone());
        let (route_id, route_revision, model_id) = model.into_parts();
        let route = ModelRoute::new(
            route_id,
            route_revision,
            self.instance().id().clone(),
            model_id,
            capabilities,
        );
        let requirements = requirements(
            self,
            session_capabilities()
                .iter()
                .map(|(capability, constraints)| {
                    CapabilityRequirement::new(capability, constraints.iter().cloned())
                }),
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let request = OpenSessionRequest::from_plan(&plan, request_id, working_resource, None)?;
        Ok(ClaudeAgentPreparedSession {
            evidence: ClaudeAgentPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}

fn validate_options(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if !options.is_empty() {
        return Err(failure(
            "swallowtail.claude_agent.preparation.session_options_unsupported",
            "Claude Agent ACP prepared sessions do not support portable session options",
        ));
    }
    Ok(())
}
