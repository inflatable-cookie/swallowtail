use super::{KimiInteractiveSession, protocol_failure, session_closed, turn_scope};
use crate::failure::failure;
use crate::local_server::interactive::callbacks::{
    CallbackHub, approval_namespace, question_namespace,
};
use crate::local_server::transport::{Request, session_path};
use futures_channel::oneshot;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::ProviderRequestHandling;
use swallowtail_runtime::{
    CallbackExchange, HostServices, JoinedTask, RuntimeFailure, RuntimeTurnId, TurnRequest,
};

pub(super) async fn run_pump_when_ready(receiver: oneshot::Receiver<super::pump::PumpInput>) {
    if let Ok(input) = receiver.await {
        super::pump::run(input).await;
    }
}

pub(super) async fn fail_setup<T>(
    sender: oneshot::Sender<super::pump::PumpInput>,
    task: Box<dyn JoinedTask>,
    error: RuntimeFailure,
) -> Result<T, RuntimeFailure> {
    drop(sender);
    let _ = task.join().await;
    Err(error)
}

impl KimiInteractiveSession {
    pub(super) fn prompt_request(&self, request: &TurnRequest) -> Result<Request, RuntimeFailure> {
        let base = session_path(&self.provider_session_id)?;
        let mut body = serde_json::json!({
            "content": super::super::content_json(request.content()),
            "model": self.model_id.as_str(),
            "permission_mode": self.configuration.permission_mode().as_wire_value(),
        });
        if let Some(reasoning) = super::super::reasoning_wire(&self.options) {
            body["thinking"] = serde_json::Value::String(reasoning.to_owned());
        }
        if let Some(profile) = self.configuration.profile() {
            body["profile"] = serde_json::Value::String(profile.to_owned());
        }
        if self.configuration.disabled_tools().len() != 0 {
            body["disabled_tools"] = serde_json::Value::Array(
                self.configuration
                    .disabled_tools()
                    .map(|tool| serde_json::Value::String(tool.to_owned()))
                    .collect(),
            );
        }
        let body = serde_json::to_vec(&body).map_err(|_| protocol_failure())?;
        Ok(Request::post_json(format!("{base}/prompts"), body))
    }

    pub(super) fn callback_exchange(
        &self,
        turn_id: &RuntimeTurnId,
        services: &HostServices,
    ) -> Result<(Option<CallbackHub>, Option<CallbackExchange>), RuntimeFailure> {
        let policy = self.resume.access_policy().provider_requests();
        if policy.handling_for(&approval_namespace()) != ProviderRequestHandling::Exchange
            || policy.handling_for(&question_namespace()) != ProviderRequestHandling::Exchange
        {
            return Ok((None, None));
        }
        let access = self.access.as_ref().ok_or_else(session_closed)?;
        let (hub, exchange) = CallbackHub::new(
            turn_scope(turn_id)?,
            self.provider_session_id.clone(),
            access.endpoint.clone(),
            Arc::downgrade(&access.secret),
            services.clone(),
            self.transport.clone(),
        );
        Ok((Some(hub), Some(exchange)))
    }

    pub(super) fn validate_turn(
        &self,
        request: &TurnRequest,
        services: &HostServices,
    ) -> Result<(), RuntimeFailure> {
        services.require_execution_host(self.resume.execution_host_id())?;
        if self.cancellation.requested.load(Ordering::SeqCst) {
            return Err(session_closed());
        }
        if request.attachments().len() != 0 || request.structured_output().is_some() {
            return Err(crate::failure::unsupported(
                "Kimi local-server attachments or structured output",
            ));
        }
        if request
            .deadline()
            .is_some_and(|deadline| services.time().expect("validated").now() >= deadline.instant())
        {
            return Err(failure(
                "swallowtail.kimi.local_server.turn_timed_out",
                "Kimi local-server turn deadline elapsed before provider work",
            ));
        }
        Ok(())
    }
}

pub(super) async fn before_turn_deadline<T, F>(
    work: F,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<T, RuntimeFailure>
where
    F: std::future::Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let mut work = Box::pin(work);
    let mut timer = services
        .time()
        .expect("validated time")
        .wait_until(deadline);
    std::future::poll_fn(|context| {
        if let std::task::Poll::Ready(result) = work.as_mut().poll(context) {
            return std::task::Poll::Ready(result);
        }
        if timer.as_mut().poll(context).is_ready() {
            cancelled.store(true, Ordering::SeqCst);
            return std::task::Poll::Ready(Err(failure(
                "swallowtail.kimi.local_server.turn_timed_out",
                "Kimi local-server turn timed out",
            )));
        }
        std::task::Poll::Pending
    })
    .await
}
