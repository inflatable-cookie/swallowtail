use crate::connection::grok_diagnostics;
use crate::failure::failure;
use serde_json::Value;
use std::sync::{Arc, Weak};
use swallowtail_acp_client::{
    AcpTurnSink, ActiveTurn as SharedActiveTurn, ActiveTurnConfig, StopReasonPolicy,
    UsageProgressPolicy,
};
use swallowtail_runtime::{
    BoxEventStream, CallbackAbandonment, CallbackExchange, Deadline, ProviderRequestObservation,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcomeFuture, TerminalStatus,
};

pub(crate) struct ActiveTurn {
    inner: Arc<SharedActiveTurn>,
    runtime_id: RuntimeTurnId,
    deadline: Option<Deadline>,
    callbacks: Option<crate::permission::CallbackHub>,
}

impl ActiveTurn {
    pub(crate) fn new(
        runtime_id: RuntimeTurnId,
        session_id: String,
        deadline: Option<Deadline>,
        exchanges_permissions: bool,
        connection: Weak<crate::connection::AcpConnection>,
    ) -> Result<
        (
            Arc<Self>,
            BoxEventStream,
            Option<CallbackExchange>,
            TerminalOutcomeFuture,
        ),
        RuntimeFailure,
    > {
        let config = ActiveTurnConfig::new(grok_diagnostics())
            .with_stop_reason_policy(StopReasonPolicy::ExtendedProviderStops)
            .with_usage_progress_policy(UsageProgressPolicy::EmitProgress);
        let (inner, stream, future) =
            SharedActiveTurn::new(runtime_id.clone(), session_id, config)?;
        let (callbacks, callback_exchange) = if exchanges_permissions {
            let (callbacks, exchange) = crate::permission::CallbackHub::new(connection, true);
            (Some(callbacks), Some(exchange))
        } else {
            (None, None)
        };
        Ok((
            Arc::new(Self {
                inner,
                runtime_id,
                deadline,
                callbacks,
            }),
            stream,
            callback_exchange,
            future,
        ))
    }

    pub(crate) fn session_id(&self) -> &str {
        self.inner.session_id()
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.inner.is_finished()
    }

    pub(crate) fn was_cancelled(&self) -> bool {
        self.inner.was_cancelled()
    }

    pub(crate) fn mark_cancelled(&self) {
        self.inner.mark_cancelled();
        if let Some(callbacks) = &self.callbacks {
            callbacks.abandon(CallbackAbandonment::TurnCancelled);
        }
    }

    pub(crate) fn timeout(&self) {
        if let Some(callbacks) = &self.callbacks {
            callbacks.abandon(CallbackAbandonment::TimedOut);
        }
        self.inner.finish_with_status(TerminalStatus::TimedOut);
    }

    pub(crate) fn observe_permission(
        &self,
        provider_request_id: &Value,
    ) -> Result<ProviderRequestObservation, RuntimeFailure> {
        let observation = self.inner.observe_permission(provider_request_id)?;
        if let Some(callbacks) = &self.callbacks {
            callbacks.abandon(CallbackAbandonment::TurnCancelled);
        }
        Ok(observation)
    }

    pub(crate) fn exchanges_permissions(&self) -> bool {
        self.callbacks
            .as_ref()
            .is_some_and(|callbacks| callbacks.exchanges_permissions())
    }

    pub(crate) fn exchange_permission(
        &self,
        provider_request_id: &Value,
        params: &Value,
    ) -> Result<(), RuntimeFailure> {
        let callbacks = self.callbacks.as_ref().ok_or_else(|| {
            failure(
                "swallowtail.grok.acp.permission_exchange_unavailable",
                "Grok Build permission exchange was not enabled for this turn",
            )
        })?;
        self.inner.emit_event(|sequence| {
            let callback_id = callbacks.enqueue_permission(
                &self.runtime_id,
                sequence,
                self.deadline,
                provider_request_id,
                params,
            )?;
            Ok(RuntimeEventKind::CallbackRequested(callback_id))
        })
    }

    pub(crate) fn handle_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        self.inner.handle_update(params)
    }

    pub(crate) fn finish_prompt(&self, stop_reason: &str) {
        if let Some(callbacks) = &self.callbacks {
            callbacks.abandon(CallbackAbandonment::TurnTerminated);
        }
        self.inner.finish_prompt(stop_reason);
    }

    pub(crate) fn fail(&self, error: &RuntimeFailure) {
        if let Some(callbacks) = &self.callbacks {
            callbacks.abandon(CallbackAbandonment::TurnTerminated);
        }
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
