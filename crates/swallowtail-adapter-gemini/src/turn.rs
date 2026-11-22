use crate::connection::gemini_acp_diagnostics;
use crate::failure::{failure, malformed};
use serde_json::Value;
use std::sync::Arc;
use swallowtail_acp_client::{
    AcpTurnSink, ActiveTurn as SharedActiveTurn, ActiveTurnConfig, StopReasonPolicy,
    UsageProgressPolicy,
};
use swallowtail_protocol_acp::AcpSessionUpdate;
use swallowtail_runtime::{
    BoxEventStream, ProviderRequestObservation, RuntimeFailure, RuntimeTurnId,
    TerminalOutcomeFuture,
};

pub(crate) struct ActiveTurn {
    inner: Arc<SharedActiveTurn>,
    expected_mode: &'static str,
}

impl ActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        session_id: String,
        expected_mode: &'static str,
    ) -> Result<(Arc<Self>, BoxEventStream, TerminalOutcomeFuture), RuntimeFailure> {
        let config = ActiveTurnConfig::new(gemini_acp_diagnostics())
            .with_stop_reason_policy(StopReasonPolicy::ExtendedProviderStops)
            .with_usage_progress_policy(UsageProgressPolicy::EmitProgress)
            .with_content_unsupported(|| {
                failure(
                    "swallowtail.gemini.acp.content_unsupported",
                    "Gemini CLI returned unsupported ACP content",
                )
            });
        let (inner, stream, future) = SharedActiveTurn::new(runtime_id, session_id, config)?;
        Ok((
            Arc::new(Self {
                inner,
                expected_mode,
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
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        self.inner.observe_permission(provider_request_id)
    }

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let decoded =
            swallowtail_protocol_acp::decode_session_update(params).map_err(|_| malformed())?;
        if decoded.session_id.as_str() == self.inner.session_id()
            && let AcpSessionUpdate::CurrentMode(mode) = &decoded.update
            && mode.as_str() != self.expected_mode
        {
            return Err(failure(
                "swallowtail.gemini.acp.mode_widened",
                "Gemini CLI changed the preflight-bound session mode",
            ));
        }
        self.inner.handle_update(params)
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
        self.observe_permission(provider_request_id)
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
