use super::QwenSessionHandle;
use super::lifecycle::{ActiveTurn, join_turn, reap_finished};
use crate::command::{arguments, resumed_arguments};
use crate::driver::write_prompt;
use crate::handle::QwenProcessCancellation;
use crate::pump::{cleanup_failed_start, pump_with_session};
use crate::validation::{failure, unsupported};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, TurnRef};
use swallowtail_runtime::{
    ActivityOperationId, BoxEventStream, BoxFuture, CancellationControl, CleanupOutcome,
    ExecutableRef, HostServices, ProcessHandle, ProcessRequest, RuntimeEvent, RuntimeEventKind,
    RuntimeFailure, RuntimeTurnId, ScopeId, TerminalOutcome, TerminalStatus, TurnHandle,
    TurnRequest, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 4098;

struct QwenTurnHandle {
    turn_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<QwenProcessCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: super::lifecycle::ActiveSlot,
    state: Arc<std::sync::Mutex<super::SessionState>>,
}

impl QwenSessionHandle {
    pub(super) async fn start_turn_inner(
        &mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> Result<Box<dyn TurnHandle>, RuntimeFailure> {
        services.require_execution_host(self.services.execution_host_id())?;
        reap_finished(&self.active, &self.state).await?;
        if self
            .active
            .lock()
            .expect("Qwen active turn lock poisoned")
            .is_some()
        {
            return Err(failure(
                "swallowtail.qwen.headless.turn_active",
                "Qwen interactive session already has an active turn",
            ));
        }
        validate_turn(self, &request)?;
        let expected_session = self
            .state
            .lock()
            .expect("Qwen session lock poisoned")
            .provider_session_id
            .clone();
        let scope = ScopeId::new(format!("qwen-headless:turn:{}", request.turn_id().as_str()))
            .map_err(|_| {
                failure(
                    "swallowtail.qwen.headless.scope_invalid",
                    "Qwen turn scope was invalid",
                )
            })?;
        let process_request =
            ProcessRequest::new(ExecutableRef::from_instance_target(&self.target))
                .with_arguments(match expected_session.as_deref() {
                    Some(session_id) => resumed_arguments(&self.model, session_id),
                    None => arguments(&self.model),
                })
                .with_environment([self.environment.clone()])
                .with_working_resource(self.working_resource.clone());
        let process: Arc<dyn ProcessHandle> = match self
            .services
            .process()
            .expect("validated Qwen process")
            .start(scope.clone(), process_request)
            .await
        {
            Ok(process) => Arc::from(process),
            Err(error) => {
                self.state
                    .lock()
                    .expect("Qwen session lock poisoned")
                    .usable = false;
                return Err(error);
            }
        };
        if let Err(error) = write_prompt(process.as_ref(), request.content()).await {
            self.state
                .lock()
                .expect("Qwen session lock poisoned")
                .usable = false;
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let deadline = request.deadline().expect("validated Qwen turn deadline");
        let deadline = self
            .services
            .time()
            .expect("validated Qwen time")
            .wait_until(deadline);
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            self.state
                .lock()
                .expect("Qwen session lock poisoned")
                .usable = false;
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let cancellation = Arc::new(QwenProcessCancellation::with_scope(
            Arc::clone(&process),
            CancellationScope::ActiveTurn,
        ));
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let terminal_flag = Arc::new(AtomicBool::new(false));
        let task_state = Arc::clone(&self.state);
        let task_cancellation = Arc::clone(&cancellation);
        let task_terminal = Arc::clone(&terminal_flag);
        let task_process = Arc::clone(&process);
        let model = self.model.clone();
        let task_expected = expected_session.clone();
        let task_turn_id = request.turn_id().clone();
        let task = self.services.task().expect("validated Qwen task").spawn(
            scope,
            Box::pin(async move {
                let result = pump_with_session(
                    task_process,
                    event_sender.clone(),
                    task_cancellation,
                    deadline,
                    model,
                    task_expected,
                    ActivityOperationId::Turn(task_turn_id),
                )
                .await;
                {
                    let mut state = task_state.lock().expect("Qwen session lock poisoned");
                    if matches!(result.outcome.status(), TerminalStatus::Completed)
                        && matches!(result.outcome.cleanup(), CleanupOutcome::Clean)
                    {
                        if let Some(session_id) = result.session_id {
                            state.provider_session_id = Some(session_id);
                            state.completed_turns += 1;
                        } else {
                            state.usable = false;
                        }
                    } else {
                        state.usable = false;
                    }
                }
                event_sender.mark_terminal();
                task_terminal.store(true, Ordering::SeqCst);
                let _ = terminal_sender.complete(result.outcome);
            }),
        );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                self.state
                    .lock()
                    .expect("Qwen session lock poisoned")
                    .usable = false;
                cleanup_failed_start(process.as_ref()).await;
                return Err(error);
            }
        };
        let turn_id = request.turn_id().clone();
        *self.active.lock().expect("Qwen active turn lock poisoned") = Some(ActiveTurn {
            turn_id: turn_id.clone(),
            task: Some(task),
            cancellation: Arc::clone(&cancellation),
            terminal: Arc::clone(&terminal_flag),
        });
        Ok(Box::new(QwenTurnHandle {
            turn_id,
            events: Some(Box::pin(event_stream)),
            terminal: Some(Box::pin(terminal_future)),
            cancellation,
            terminal_flag,
            active: Arc::clone(&self.active),
            state: Arc::clone(&self.state),
        }))
    }
}

impl TurnHandle for QwenTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
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
            join_turn(&self.active, &self.turn_id, &self.state).await
        })
    }
}

fn validate_turn(session: &QwenSessionHandle, request: &TurnRequest) -> Result<(), RuntimeFailure> {
    let state = session.state.lock().expect("Qwen session lock poisoned");
    if !state.usable {
        return Err(failure(
            "swallowtail.qwen.headless.session_unusable",
            "Qwen interactive session can no longer accept turns",
        ));
    }
    if state.completed_turns >= 24 {
        return Err(failure(
            "swallowtail.qwen.headless.turn_limit",
            "Qwen interactive session reached its bounded turn limit",
        ));
    }
    drop(state);
    if request.attachments().len() != 0 || request.structured_output().is_some() {
        return Err(unsupported("turn attachments or structured output"));
    }
    let deadline = request
        .deadline()
        .ok_or_else(|| unsupported("a turn without an explicit host deadline"))?;
    if session.services.time().expect("validated Qwen time").now() >= deadline.instant() {
        return Err(failure(
            "swallowtail.qwen.headless.deadline_elapsed",
            "Qwen turn deadline elapsed before provider work",
        ));
    }
    Ok(())
}
