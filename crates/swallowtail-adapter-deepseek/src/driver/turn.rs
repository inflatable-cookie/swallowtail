mod exchange;
mod pump;

use super::lifecycle::{ActiveTurn, TurnCancellation, join_turn, reap_finished};
use super::session::DeepSeekSessionHandle;
use crate::failure::{failure, protocol};
use crate::protocol::{HttpRequest, encode_initial, encode_later_user};
use exchange::ResultSubmitter;
use futures_channel::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::TurnRef;
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationControl, CleanupOutcome, DirectContinuationTurnRequest,
    DirectInferenceAttempt, DirectToolExchange, HostServices, RuntimeEvent, RuntimeEventKind,
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnHandle, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;

struct DeepSeekTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    exchange: Option<DirectToolExchange>,
    cancellation: Arc<TurnCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: super::lifecycle::ActiveSlot,
}

pub(super) struct TurnWork {
    pub(super) attempt: DirectInferenceAttempt,
    pub(super) request: DirectContinuationTurnRequest,
    pub(super) initial_request: HttpRequest,
    pub(super) endpoint: String,
    pub(super) credential: SecretBytes,
    pub(super) call_sender:
        Option<mpsc::Sender<Result<swallowtail_runtime::DirectToolCall, RuntimeFailure>>>,
    pub(super) submitter: Option<Arc<ResultSubmitter>>,
    pub(super) result_receiver:
        Option<futures_channel::oneshot::Receiver<Vec<swallowtail_runtime::DirectToolResult>>>,
}

impl DeepSeekSessionHandle {
    pub(super) async fn start_direct_turn(
        &mut self,
        request: DirectContinuationTurnRequest,
        services: HostServices,
    ) -> Result<Box<dyn TurnHandle>, RuntimeFailure> {
        services.require_execution_host(self.services.execution_host_id())?;
        reap_finished(&self.active).await?;
        if self
            .active
            .lock()
            .expect("active turn lock poisoned")
            .is_some()
        {
            return Err(failure(
                "swallowtail.deepseek.turn_active",
                "DeepSeek session already has an active turn",
            ));
        }
        validate_turn(self, &request)?;
        let attempt = self
            .state
            .lock()
            .expect("continuation state lock poisoned")
            .authorize_user_turn(&request)?;
        let initial_request = match build_request(self, &request, &attempt) {
            Ok(request) => request,
            Err(error) => {
                self.invalidate();
                return Err(error);
            }
        };
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let cancelled = Arc::new(AtomicBool::new(false));
        let (cancellation, cancel_receiver) =
            TurnCancellation::new(Arc::clone(&cancelled), Arc::clone(&self.usable));
        let cancellation = Arc::new(cancellation);
        let (call_sender, exchange, submitter, result_receiver) = if attempt.ordinal().get() == 1 {
            let (sender, receiver) = mpsc::channel(1);
            let (submitter, results) = ResultSubmitter::new();
            let submitter = Arc::new(submitter);
            let exchange = DirectToolExchange::new(
                Box::pin(receiver),
                Arc::clone(&submitter) as Arc<dyn swallowtail_runtime::DirectToolResultSubmitter>,
            );
            (Some(sender), Some(exchange), Some(submitter), Some(results))
        } else {
            (None, None, None, None)
        };
        let credential = self
            .access
            .as_ref()
            .expect("session access exists")
            .secret()?;
        let work = TurnWork {
            attempt,
            request: request.clone(),
            initial_request,
            endpoint: self.endpoint.clone(),
            credential: SecretBytes(credential),
            call_sender,
            submitter,
            result_receiver,
        };
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let terminal_flag = Arc::new(AtomicBool::new(false));
        let task_terminal = Arc::clone(&terminal_flag);
        let task_cancellation = Arc::clone(&cancellation);
        let task = self.services.task().expect("validated task").spawn(
            self.scope.clone(),
            Box::pin({
                let context = pump::TurnContext::from_session(self, cancelled, cancel_receiver);
                async move {
                    let outcome =
                        pump::run_turn(work, context, events.clone(), task_cancellation).await;
                    events.mark_terminal();
                    task_terminal.store(true, Ordering::SeqCst);
                    let _ = terminal_sender.complete(outcome);
                }
            }),
        );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                self.invalidate();
                return Err(error);
            }
        };
        *self.active.lock().expect("active turn lock poisoned") = Some(ActiveTurn {
            turn_id: request.turn_id().clone(),
            task: Some(task),
            cancellation: Arc::clone(&cancellation),
            terminal: Arc::clone(&terminal_flag),
        });
        Ok(Box::new(DeepSeekTurnHandle {
            runtime_id: request.turn_id().clone(),
            events: Some(Box::pin(stream)),
            terminal: Some(Box::pin(terminal_future)),
            exchange,
            cancellation,
            terminal_flag,
            active: Arc::clone(&self.active),
        }) as Box<dyn TurnHandle>)
    }

    pub(super) fn invalidate(&self) {
        self.usable.store(false, Ordering::SeqCst);
        self.state
            .lock()
            .expect("continuation state lock poisoned")
            .invalidate();
    }
}

impl TurnHandle for DeepSeekTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }
    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        None
    }
    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }
    fn take_direct_tool_exchange(&mut self) -> Option<DirectToolExchange> {
        self.exchange.take()
    }
    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.terminal_flag.load(Ordering::SeqCst) {
                let _ = self.cancellation.request().await;
            }
            join_turn(&self.active, &self.runtime_id).await
        })
    }
}

fn validate_turn(
    session: &DeepSeekSessionHandle,
    request: &DirectContinuationTurnRequest,
) -> Result<(), RuntimeFailure> {
    if session.cancellation.is_requested() || !session.usable.load(Ordering::SeqCst) {
        return Err(failure(
            "swallowtail.deepseek.session_closed",
            "DeepSeek session can no longer accept turns",
        ));
    }
    if session.services.time().expect("validated time").now() >= request.deadline().instant() {
        return Err(failure(
            "swallowtail.deepseek.deadline_elapsed",
            "DeepSeek turn deadline elapsed before provider work",
        ));
    }
    Ok(())
}

fn build_request(
    session: &DeepSeekSessionHandle,
    request: &DirectContinuationTurnRequest,
    attempt: &DirectInferenceAttempt,
) -> Result<HttpRequest, RuntimeFailure> {
    let body = match attempt.ordinal().get() {
        1 => encode_initial(request.content().as_str(), &session.tools),
        3 => {
            let history = session.history.lock().expect("history lock poisoned");
            if !history.is_complete() {
                return Err(failure(
                    "swallowtail.deepseek.history_incomplete",
                    "DeepSeek later turn requires complete private continuation history",
                ));
            }
            let first = history.first()?;
            encode_later_user(
                first.user(),
                first.reasoning(),
                first.call_id(),
                first.tool_name(),
                first.arguments(),
                first.result()?,
                first.final_reasoning()?,
                first.answer()?,
                request.content().as_str(),
                &session.tools,
            )
        }
        _ => {
            return Err(failure(
                "swallowtail.deepseek.attempt_sequence_invalid",
                "DeepSeek user turn authorized an invalid attempt ordinal",
            ));
        }
    }
    .map_err(protocol)?;
    Ok(HttpRequest::completion(body, attempt.ordinal().get() != 1))
}

pub(super) struct SecretBytes(Vec<u8>);

impl SecretBytes {
    pub(super) fn copy(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
