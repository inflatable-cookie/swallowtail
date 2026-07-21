mod pump;

use self::pump::{PendingTurn, pump_turn};
use super::lifecycle::{ActiveTurn, TurnCancellation, join_turn, reap_finished};
use super::session::XaiSessionHandle;
use crate::failure::{failure, unsupported};
use futures_channel::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::TurnRef;
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationControl, CleanupOutcome, HostServices, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnHandle, TurnRequest,
    runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;
const INGRESS_CAPACITY: usize = 32;

struct XaiTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<TurnCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: super::lifecycle::ActiveSlot,
}

impl XaiSessionHandle {
    pub(super) async fn start_turn_inner(
        &mut self,
        request: TurnRequest,
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
                "swallowtail.xai.turn_active",
                "xAI WebSocket session already has an active turn",
            ));
        }
        validate_turn(self, &request)?;

        let turn_scope = swallowtail_runtime::ScopeId::new(format!(
            "xai-websocket:turn:{}",
            request.turn_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.xai.scope_invalid",
                "xAI turn scope was invalid",
            )
        })?;
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let (updates, receiver) = mpsc::channel(INGRESS_CAPACITY);
        let cancellation = Arc::new(TurnCancellation::new(
            self.connection.closer(),
            Arc::clone(&self.chain_valid),
        ));
        let connection = self.connection.clone();
        let model = self.model.clone();
        let input = request.content().as_str().to_owned();
        let continuation = Arc::clone(&self.continuation);
        let chain_valid = Arc::clone(&self.chain_valid);
        let blocking = self
            .services
            .blocking_work()
            .cloned()
            .expect("validated blocking work");
        let work = blocking.run(
            turn_scope.clone(),
            Box::new(move || {
                connection.run_turn(&model, &input, &continuation, &chain_valid, updates)
            }),
        );
        let pending = Arc::new(Mutex::new(Some(PendingTurn {
            updates: receiver,
            work,
        })));
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let terminal_flag = Arc::new(AtomicBool::new(false));
        let task_service = self.services.task().cloned().expect("validated task");
        let task_pending = Arc::clone(&pending);
        let task_cancellation = Arc::clone(&cancellation);
        let task_terminal = Arc::clone(&terminal_flag);
        let turn_id = request.turn_id().clone();
        let active_turn_id = turn_id.clone();
        let model_route_id = self.model_route_id.clone();
        let access_profile_id = self.access_profile_id.clone();
        let deadline = request.deadline();
        let time = self.services.time().cloned().expect("validated time");
        let task = task_service.spawn(
            turn_scope,
            Box::pin(async move {
                let pending = task_pending
                    .lock()
                    .expect("pending turn lock poisoned")
                    .take()
                    .expect("pending turn is available");
                let outcome = pump_turn(
                    pending,
                    events.clone(),
                    Arc::clone(&task_cancellation),
                    deadline.map(|deadline| time.wait_until(deadline)),
                    turn_id,
                    model_route_id,
                    access_profile_id,
                )
                .await;
                events.mark_terminal();
                task_terminal.store(true, Ordering::SeqCst);
                let _ = terminal_sender.complete(outcome);
            }),
        );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                let _ = cancellation.request().await;
                let pending = { pending.lock().expect("pending turn lock poisoned").take() };
                if let Some(pending) = pending {
                    let _ = pending.work.await;
                }
                return Err(error);
            }
        };
        *self.active.lock().expect("active turn lock poisoned") = Some(ActiveTurn {
            turn_id: active_turn_id,
            task: Some(task),
            cancellation: Arc::clone(&cancellation),
            terminal: Arc::clone(&terminal_flag),
        });
        Ok(Box::new(XaiTurnHandle {
            runtime_id: request.turn_id().clone(),
            events: Some(Box::pin(stream)),
            terminal: Some(Box::pin(terminal_future)),
            cancellation,
            terminal_flag,
            active: Arc::clone(&self.active),
        }) as Box<dyn TurnHandle>)
    }
}

impl TurnHandle for XaiTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
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

fn validate_turn(session: &XaiSessionHandle, request: &TurnRequest) -> Result<(), RuntimeFailure> {
    if session.cancellation.is_requested()
        || !session.chain_valid.load(Ordering::SeqCst)
        || !session.connection.is_open()
    {
        return Err(failure(
            "swallowtail.xai.session_closed",
            "xAI WebSocket session can no longer accept turns",
        ));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("turn attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured turn output"));
    }
    if let Some(deadline) = request.deadline()
        && session.services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.xai.deadline_elapsed",
            "xAI turn deadline elapsed before provider work",
        ));
    }
    Ok(())
}
