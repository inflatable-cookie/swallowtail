use super::OllamaSessionHandle;
use super::lifecycle::{ActiveTurn, TurnCompletion, join_turn, reap_finished};
use crate::driver::*;
use crate::failure::unsupported;
use crate::protocol::{ChatMessage, Request};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::TurnRef;
use swallowtail_runtime::{
    BoxEventStream, CancellationControl, RuntimeTurnId, TurnHandle, TurnRequest,
};

struct OllamaTurnHandle {
    turn_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<RunCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: super::lifecycle::ActiveSlot,
    state: Arc<Mutex<super::SessionState>>,
}

impl OllamaSessionHandle {
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
            .expect("Ollama active turn lock poisoned")
            .is_some()
        {
            return Err(failure(
                "swallowtail.ollama.turn_active",
                "Ollama interactive session already has an active turn",
            ));
        }
        validate_turn(self, &request)?;
        let user = ChatMessage::user(request.content().as_str());
        let mut messages = {
            self.state
                .lock()
                .expect("Ollama session lock poisoned")
                .history
                .clone()
        };
        messages.push(user.clone());
        if messages.len() > 47 {
            return Err(failure(
                "swallowtail.ollama.history_limit",
                "Ollama interactive transcript reached its bounded message limit",
            ));
        }
        let chat = Request::chat_history(
            &self.model,
            &messages,
            8,
            self.context_window.map(crate::OllamaContextWindow::as_u32),
        )?;
        let scope = operation_scope("turn", request.turn_id().as_str())?;
        let cancelled = Arc::new(AtomicBool::new(false));
        if let Err(error) = OllamaNativeAttachedDriver::from_transport(self.transport.clone())
            .observe_catalogue(
                scope.clone(),
                &self.endpoint,
                &self.plan,
                request.deadline(),
                &self.services,
                Arc::clone(&cancelled),
            )
            .await
        {
            self.state
                .lock()
                .expect("Ollama session lock poisoned")
                .usable = false;
            return Err(error);
        }
        let subscription = self.transport.subscribe(
            scope.clone(),
            self.endpoint.clone(),
            chat,
            self.model.clone(),
            &self.services,
            Arc::clone(&cancelled),
        )?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let cancellation = Arc::new(RunCancellation::active_turn(Arc::clone(&cancelled)));
        let deadline = request.deadline().map(|deadline| {
            self.services
                .time()
                .expect("validated time")
                .wait_until(deadline)
        });
        let pending = Arc::new(Mutex::new(Some(subscription)));
        let completion = Arc::new(Mutex::new(TurnCompletion::Pending));
        let task_completion = Arc::clone(&completion);
        let history_model = self.model.clone();
        let history_context_window = self.context_window.map(crate::OllamaContextWindow::as_u32);
        let terminal_flag = Arc::new(AtomicBool::new(false));
        let task_terminal = Arc::clone(&terminal_flag);
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let activity_turn_id = request.turn_id().clone();
        let task = self
            .services
            .task()
            .expect("validated Ollama task")
            .spawn(
                scope,
                Box::pin({
                    let pending = Arc::clone(&pending);
                    let cancelled = Arc::clone(&cancelled);
                    async move {
                        let subscription = pending
                            .lock()
                            .expect("Ollama pending work lock poisoned")
                            .take()
                            .expect("Ollama pending work is available");
                        let mut outcome = pump_run(
                            subscription,
                            services,
                            event_sender.clone(),
                            cancelled,
                            deadline,
                            swallowtail_runtime::ActivityOperationId::Turn(activity_turn_id),
                        )
                        .await;
                        let completion = if matches!(
                            (outcome.status(), outcome.cleanup()),
                            (TerminalStatus::Completed, CleanupOutcome::Clean)
                        ) {
                            if let Some(output) = outcome.output() {
                                let assistant = ChatMessage::assistant(output.as_str());
                                let mut committed = messages;
                                committed.push(assistant.clone());
                                match Request::chat_history(
                                    &history_model,
                                    &committed,
                                    8,
                                    history_context_window,
                                ) {
                                    Ok(_) if committed.len() <= 48 => {
                                        TurnCompletion::Commit(user, assistant)
                                    }
                                    _ => {
                                        outcome = TerminalOutcome::new(
                                            TerminalStatus::RuntimeFailed(
                                                swallowtail_core::SafeDiagnostic::new(
                                                    "swallowtail.ollama.history_limit",
                                                    "Ollama interactive transcript exceeded its bounded history limit",
                                                ),
                                            ),
                                            outcome.cleanup().clone(),
                                        );
                                        TurnCompletion::Reusable
                                    }
                                }
                            } else {
                                TurnCompletion::Unusable
                            }
                        } else if matches!(outcome.cleanup(), CleanupOutcome::Clean) {
                            TurnCompletion::Reusable
                        } else {
                            TurnCompletion::Unusable
                        };
                        *task_completion
                            .lock()
                            .expect("Ollama turn completion lock poisoned") = completion;
                        event_sender.mark_terminal();
                        task_terminal.store(true, Ordering::SeqCst);
                        let _ = terminal_sender.complete(outcome);
                    }
                }),
            );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                cancelled.store(true, Ordering::SeqCst);
                let subscription = pending
                    .lock()
                    .expect("Ollama pending work lock poisoned")
                    .take();
                if let Some(subscription) = subscription {
                    let _ = subscription.close().await;
                }
                self.state
                    .lock()
                    .expect("Ollama session lock poisoned")
                    .usable = false;
                return Err(error);
            }
        };
        let turn_id = request.turn_id().clone();
        *self
            .active
            .lock()
            .expect("Ollama active turn lock poisoned") = Some(ActiveTurn {
            turn_id: turn_id.clone(),
            task: Some(task),
            cancellation: Arc::clone(&cancellation),
            terminal: Arc::clone(&terminal_flag),
            completion,
        });
        Ok(Box::new(OllamaTurnHandle {
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

impl TurnHandle for OllamaTurnHandle {
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

fn validate_turn(
    session: &OllamaSessionHandle,
    request: &TurnRequest,
) -> Result<(), RuntimeFailure> {
    let state = session.state.lock().expect("Ollama session lock poisoned");
    if !state.usable {
        return Err(failure(
            "swallowtail.ollama.session_unusable",
            "Ollama interactive session can no longer accept turns",
        ));
    }
    if state.completed_turns >= 24 {
        return Err(failure(
            "swallowtail.ollama.turn_limit",
            "Ollama interactive session reached its bounded turn limit",
        ));
    }
    drop(state);
    if request.attachments().len() != 0 || request.structured_output().is_some() {
        return Err(unsupported("turn attachments or structured output"));
    }
    if let Some(deadline) = request.deadline()
        && session.services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.ollama.deadline_elapsed",
            "Ollama turn deadline elapsed before provider work",
        ));
    }
    Ok(())
}
