use super::*;
use swallowtail_runtime::{SessionLifecycleOperation, prepare_negotiated_reasoning_setup};

pub(crate) enum ClaudeAgentReasoningAcknowledgement {
    NotRequested,
    Effective(String),
}

pub(crate) struct ClaudeAgentOpenRejection {
    failure: RuntimeFailure,
    rejected_reasoning: Option<String>,
}

impl ClaudeAgentOpenRejection {
    pub(crate) const fn runtime(failure: RuntimeFailure) -> Self {
        Self {
            failure,
            rejected_reasoning: None,
        }
    }

    pub(crate) const fn rejected(failure: RuntimeFailure, reasoning: String) -> Self {
        Self {
            failure,
            rejected_reasoning: Some(reasoning),
        }
    }

    pub(crate) fn rejected_reasoning(&self) -> Option<&str> {
        self.rejected_reasoning.as_deref()
    }

    pub(crate) fn into_failure(self) -> RuntimeFailure {
        self.failure
    }
}

impl From<RuntimeFailure> for ClaudeAgentOpenRejection {
    fn from(failure: RuntimeFailure) -> Self {
        Self::runtime(failure)
    }
}

impl ClaudeAgentAcpDriver {
    pub(crate) async fn open_session_lifecycle(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> Result<
        (
            Box<dyn InteractiveSessionHandle>,
            ClaudeAgentReasoningAcknowledgement,
        ),
        ClaudeAgentOpenRejection,
    > {
        let selected = validate_plan(&plan, self.credential.as_ref())?;
        let reasoning = prepare_negotiated_reasoning_setup(
            &plan,
            SessionLifecycleOperation::Open,
            request.options(),
        )?
        .map(|setup| setup.requested().clone());
        if reasoning.is_some() && !selected.behavior().supports_config_options() {
            return Err(unsupported("reasoning selection for this adapter version").into());
        }
        validate_open(&plan, &request, &services)?;
        self.start_session_with_acknowledgement(&plan, &request, &services, selected, reasoning)
            .await
            .map(|(session, acknowledgement)| {
                (
                    Box::new(session) as Box<dyn InteractiveSessionHandle>,
                    acknowledgement,
                )
            })
    }
}
