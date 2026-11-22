use crate::connection::copilot_diagnostics;
use crate::failure::failure;
#[cfg(test)]
use crate::failure::malformed;
use serde_json::Value;
use std::sync::Arc;
#[cfg(test)]
use std::sync::Mutex;
use swallowtail_acp_client::{
    AcpTurnSink, ActiveTurn as SharedActiveTurn, ActiveTurnConfig, StopReasonPolicy,
    UsageProgressPolicy,
};
use swallowtail_runtime::{
    BoxEventStream, ProviderRequestObservation, RuntimeFailure, RuntimeTurnId,
    TerminalOutcomeFuture,
};

pub(crate) struct ActiveTurn {
    inner: Arc<SharedActiveTurn>,
    #[cfg(test)]
    assessment: Option<AssessmentTracker>,
}

#[cfg(test)]
struct AssessmentTracker {
    action: crate::assessment::CopilotCliAssessmentAction,
    activity: Mutex<crate::acp_activity::AcpActivityProjection>,
}

fn turn_config() -> ActiveTurnConfig {
    ActiveTurnConfig::new(copilot_diagnostics())
        .with_stop_reason_policy(StopReasonPolicy::Standard)
        .with_usage_progress_policy(UsageProgressPolicy::Silent)
        .with_content_unsupported(|| {
            failure(
                "swallowtail.copilot-cli.acp.content_unsupported",
                "Copilot CLI returned unsupported ACP content",
            )
        })
}

impl ActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        session_id: String,
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let (inner, stream, future) = SharedActiveTurn::new(runtime_id, session_id, turn_config())?;
        Ok((
            Arc::new(Self {
                inner,
                #[cfg(test)]
                assessment: None,
            }),
            stream,
            future,
        ))
    }

    #[cfg(test)]
    pub(crate) fn new_for_assessment(
        runtime_id: RuntimeTurnId,
        session_id: String,
        action: crate::assessment::CopilotCliAssessmentAction,
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let (inner, stream, future) =
            SharedActiveTurn::new(runtime_id.clone(), session_id, turn_config())?;
        Ok((
            Arc::new(Self {
                inner,
                assessment: Some(AssessmentTracker {
                    action,
                    activity: Mutex::new(crate::acp_activity::AcpActivityProjection::new(
                        runtime_id,
                    )),
                }),
            }),
            stream,
            future,
        ))
    }

    pub(crate) fn session_id(&self) -> &str {
        self.inner.session_id()
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.inner.is_finished()
    }

    pub(crate) fn mark_cancelled(&self) {
        self.inner.mark_cancelled();
    }

    pub(crate) fn observe_permission(
        &self,
        provider_request_id: &Value,
        _tool_call_id: &str,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        #[cfg(test)]
        if let Some(assessment) = &self.assessment {
            let activity = assessment
                .activity
                .lock()
                .expect("turn activity lock poisoned");
            let action = assessment.action;
            let action_is_pending = _tool_call_id == action.tool_call_id
                && activity.is_pending_execute_for_assessment(
                    _tool_call_id,
                    action.path,
                    action.old_text,
                    action.new_text,
                );
            if !action_is_pending {
                return Err(failure(
                    "swallowtail.copilot-cli.acp.permission_action_mismatch",
                    "Copilot CLI permission request does not match the assessment sentinel action",
                ));
            }
        }
        self.inner.observe_permission(provider_request_id)
    }

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        self.inner.handle_update(params)?;
        #[cfg(test)]
        if let Some(assessment) = &self.assessment {
            let decoded =
                swallowtail_protocol_acp::decode_session_update(params).map_err(|_| malformed())?;
            let _ = assessment
                .activity
                .lock()
                .expect("turn activity lock poisoned")
                .project(&decoded.update)?;
        }
        Ok(())
    }

    pub(crate) fn finish_prompt(&self, stop_reason: &str) {
        self.inner.finish_prompt(stop_reason);
    }

    pub(crate) fn fail(&self, error: &RuntimeFailure) {
        self.inner.fail(error);
    }
}

impl AcpTurnSink for ActiveTurn {
    fn session_id(&self) -> &str {
        self.session_id()
    }

    fn is_finished(&self) -> bool {
        self.is_finished()
    }

    fn mark_cancelled(&self) {
        self.mark_cancelled();
    }

    fn observe_permission(
        &self,
        provider_request_id: &Value,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        self.observe_permission(provider_request_id, "")
    }

    fn observe_permission_with_tool_call(
        &self,
        provider_request_id: &Value,
        tool_call_id: &str,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        self.observe_permission(provider_request_id, tool_call_id)
    }

    fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        self.handle_update(params)
    }

    fn finish_prompt(&self, stop_reason: &str) {
        self.finish_prompt(stop_reason);
    }

    fn fail(&self, error: &RuntimeFailure) {
        self.fail(error);
    }
}

#[cfg(test)]
include!("turn_tests.rs");
