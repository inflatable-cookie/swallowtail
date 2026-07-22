mod pump;

use super::lifecycle::{ActiveTurn, TurnCancellation, join_turn, reap_finished};
use super::session::AlibabaSessionHandle;
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::{TurnOptions, WireRequest};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::TurnRef;
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationControl, CleanupOutcome, HostServices, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnHandle, TurnRequest,
    runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;

struct AlibabaTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<TurnCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: super::lifecycle::ActiveSlot,
}

impl AlibabaSessionHandle {
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
                "swallowtail.alibaba_model_studio.turn_active",
                "Alibaba Model Studio session already has an active turn",
            ));
        }
        validate_turn(self, &request)?;
        let wire = WireRequest::response(
            &self.conversation,
            request.content(),
            &TurnOptions::frozen(),
        )
        .map_err(protocol)?;
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let cancelled = Arc::new(AtomicBool::new(false));
        let subscription = self.transport.subscribe(
            self.scope.clone(),
            self.access
                .as_ref()
                .expect("session access exists")
                .endpoint
                .clone(),
            self.access
                .as_ref()
                .expect("session access exists")
                .secret()?,
            wire,
            &self.services,
            Arc::clone(&cancelled),
        )?;
        let cancellation = Arc::new(TurnCancellation::new(
            cancelled,
            Arc::clone(&self.usable),
            Arc::clone(&self.remote_uncertain),
        ));
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let terminal_flag = Arc::new(AtomicBool::new(false));
        let pending = Arc::new(Mutex::new(Some(subscription)));
        let task_pending = Arc::clone(&pending);
        let task_cancellation = Arc::clone(&cancellation);
        let task_terminal = Arc::clone(&terminal_flag);
        let completed_turns = Arc::clone(&self.completed_turns);
        let deadline = request.deadline().map(|deadline| {
            self.services
                .time()
                .expect("validated time")
                .wait_until(deadline)
        });
        let task = self.services.task().expect("validated task").spawn(
            self.scope.clone(),
            Box::pin(async move {
                let subscription = task_pending
                    .lock()
                    .expect("pending turn lock poisoned")
                    .take()
                    .expect("pending turn exists");
                let outcome = pump::pump_turn(
                    subscription,
                    events.clone(),
                    Arc::clone(&task_cancellation),
                    deadline,
                )
                .await;
                if matches!(
                    outcome.status(),
                    swallowtail_runtime::TerminalStatus::Completed
                ) {
                    completed_turns.fetch_add(1, Ordering::SeqCst);
                }
                events.mark_terminal();
                task_terminal.store(true, Ordering::SeqCst);
                let _ = terminal_sender.complete(outcome);
            }),
        );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                let _ = cancellation.request().await;
                let subscription = pending.lock().expect("pending turn lock poisoned").take();
                if let Some(subscription) = subscription {
                    let _ = subscription.close().await;
                }
                return Err(error);
            }
        };
        *self.active.lock().expect("active turn lock poisoned") = Some(ActiveTurn {
            turn_id: request.turn_id().clone(),
            task: Some(task),
            cancellation: Arc::clone(&cancellation),
            terminal: Arc::clone(&terminal_flag),
        });
        Ok(Box::new(AlibabaTurnHandle {
            runtime_id: request.turn_id().clone(),
            events: Some(Box::pin(stream)),
            terminal: Some(Box::pin(terminal_future)),
            cancellation,
            terminal_flag,
            active: Arc::clone(&self.active),
        }) as Box<dyn TurnHandle>)
    }
}

impl TurnHandle for AlibabaTurnHandle {
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

fn validate_turn(
    session: &AlibabaSessionHandle,
    request: &TurnRequest,
) -> Result<(), RuntimeFailure> {
    if session.cancellation.is_requested() || !session.usable.load(Ordering::SeqCst) {
        return Err(failure(
            "swallowtail.alibaba_model_studio.session_closed",
            "Alibaba Model Studio session can no longer accept turns",
        ));
    }
    if session.completed_turns.load(Ordering::SeqCst) >= 2 {
        return Err(failure(
            "swallowtail.alibaba_model_studio.turn_limit_reached",
            "Alibaba Model Studio session reached its two-turn limit",
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
            "swallowtail.alibaba_model_studio.deadline_elapsed",
            "Alibaba Model Studio turn deadline elapsed before provider work",
        ));
    }
    Ok(())
}
