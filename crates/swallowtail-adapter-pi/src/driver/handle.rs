use super::disposition::{CommandDisposition, parse_command_disposition};
use super::input::SharedAttachmentMaterialization;
use super::session::{ActiveSlot, cleanup_failure};
use crate::connection::PiConnection;
use crate::failure::failure;
use crate::turn::ActiveTurn;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, HarnessMessageClass, TurnRef};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, HarnessCommandAcknowledgement, HarnessCommandResponse, HarnessScheduledMessage,
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnHandle,
};

pub(super) struct TurnCancellation {
    connection: Arc<PiConnection>,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
    rejected: AtomicBool,
}

impl TurnCancellation {
    pub(super) fn new(connection: Arc<PiConnection>, turn: Arc<ActiveTurn>) -> Self {
        Self {
            connection,
            turn,
            requested: AtomicBool::new(false),
            rejected: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                if self.rejected.load(Ordering::SeqCst) {
                    return Err(failure(
                        "swallowtail.pi.rpc.abort_rejected",
                        "Pi RPC rejected native abort",
                    ));
                }
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_abort_in_flight();
            let id = format!("abort:{}", self.turn.runtime_id().as_str());
            let error = match self
                .connection
                .abort_command(
                    id.clone(),
                    json!({"id": id, "type": "abort"}),
                    Arc::clone(&self.turn),
                )
                .await
            {
                Ok(response) if response.success => {
                    self.turn.mark_abort_acknowledged();
                    return Ok(CancellationAcknowledgement::Requested);
                }
                Ok(_) => failure(
                    "swallowtail.pi.rpc.abort_rejected",
                    "Pi RPC rejected native abort",
                ),
                Err(error) => {
                    let _ = self.turn.reconcile_abort_response(false);
                    error
                }
            };
            // A turn that could not be aborted while still active may still be
            // running on the provider. Scope connection closure to this exact
            // unfinished turn so a late refusal after settlement cannot
            // terminate a replacement turn.
            if self.connection.close_on_rejected_abort(
                &self.turn,
                swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.pi.rpc.cancel_rejected",
                    "Pi could not abort the active turn",
                ),
            ) {
                self.rejected.store(true, Ordering::SeqCst);
            }
            Err(error)
        })
    }
}

pub(super) struct SessionCancellation {
    connection: Arc<PiConnection>,
    active: ActiveSlot,
    requested: AtomicBool,
}

impl SessionCancellation {
    pub(super) fn new(connection: Arc<PiConnection>, active: ActiveSlot) -> Self {
        Self {
            connection,
            active,
            requested: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            if let Some(active) = self
                .active
                .lock()
                .expect("Pi active-task lock poisoned")
                .as_ref()
            {
                active.turn.mark_cancelled();
            }
            self.connection.force_stop().await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

pub(super) struct PiTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: TurnCancellation,
    connection: Arc<PiConnection>,
    active: ActiveSlot,
    attachment: SharedAttachmentMaterialization,
    command_dispositions: bool,
}

pub(super) struct PiTurnBinding {
    pub(super) connection: Arc<PiConnection>,
    pub(super) turn: Arc<ActiveTurn>,
    pub(super) active: ActiveSlot,
    pub(super) attachment: SharedAttachmentMaterialization,
    pub(super) command_dispositions: bool,
}

impl PiTurnHandle {
    pub(super) fn new(
        runtime_id: RuntimeTurnId,
        events: BoxEventStream,
        callbacks: CallbackExchange,
        terminal: BoxFuture<'static, TerminalOutcome>,
        binding: PiTurnBinding,
    ) -> Self {
        Self {
            runtime_id,
            events: Some(events),
            callbacks: Some(callbacks),
            terminal: Some(terminal),
            cancellation: TurnCancellation::new(Arc::clone(&binding.connection), binding.turn),
            connection: binding.connection,
            active: binding.active,
            attachment: binding.attachment,
            command_dispositions: binding.command_dispositions,
        }
    }
}

impl TurnHandle for PiTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        self.callbacks.take()
    }

    fn schedule_harness_message(
        &mut self,
        message: HarnessScheduledMessage,
    ) -> BoxFuture<'_, Result<HarnessCommandResponse, RuntimeFailure>> {
        Box::pin(async move {
            let class = message.class();
            if class == HarnessMessageClass::Prompt || self.cancellation.turn.is_finished() {
                return Ok(HarnessCommandResponse::new(
                    message.command_id().clone(),
                    HarnessCommandAcknowledgement::Rejected,
                ));
            }
            if !self.cancellation.turn.reserve_scheduling(class) {
                return Ok(HarnessCommandResponse::new(
                    message.command_id().clone(),
                    HarnessCommandAcknowledgement::Rejected,
                ));
            }
            let command = match class {
                HarnessMessageClass::Steering => "steer",
                HarnessMessageClass::FollowUp => "follow_up",
                HarnessMessageClass::Prompt => unreachable!(),
            };
            let id = message.command_id().as_str().to_owned();
            let result = self
                .connection
                .command(
                    id.clone(),
                    command,
                    json!({"id": id, "type": command, "message": message.content().as_str()}),
                )
                .await;
            match result {
                Ok(response) if response.success => match parse_command_disposition(
                    response.data.as_ref(),
                    command,
                    self.command_dispositions,
                ) {
                    Ok(Some(CommandDisposition::Handled)) => {
                        self.cancellation.turn.release_scheduling(class);
                        Ok(HarnessCommandResponse::new(
                            message.command_id().clone(),
                            HarnessCommandAcknowledgement::Rejected,
                        ))
                    }
                    Ok(Some(CommandDisposition::Queued | CommandDisposition::Started))
                    | Ok(None) => Ok(HarnessCommandResponse::new(
                        message.command_id().clone(),
                        HarnessCommandAcknowledgement::Accepted,
                    )),
                    Err(error) => {
                        self.cancellation.turn.release_scheduling(class);
                        Err(error)
                    }
                },
                Ok(_) => {
                    self.cancellation.turn.release_scheduling(class);
                    Ok(HarnessCommandResponse::new(
                        message.command_id().clone(),
                        HarnessCommandAcknowledgement::Rejected,
                    ))
                }
                Err(error) => {
                    self.cancellation.turn.release_scheduling(class);
                    Err(error)
                }
            }
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let cancel_cleanup = if self.cancellation.rejected.load(Ordering::SeqCst) {
                cleanup_failure(
                    "cancel_rejected",
                    "Pi could not abort the active turn during close",
                )
            } else if self.cancellation.turn.is_finished() {
                CleanupOutcome::NotApplicable
            } else {
                match self.cancellation.request().await {
                    Ok(_) => CleanupOutcome::NotApplicable,
                    Err(_) => cleanup_failure(
                        "cancel_rejected",
                        "Pi could not abort the active turn during close",
                    ),
                }
            };
            let active = {
                let mut slot = self.active.lock().expect("Pi active-task lock poisoned");
                if slot
                    .as_ref()
                    .is_some_and(|active| Arc::ptr_eq(&active.turn, &self.cancellation.turn))
                {
                    slot.take()
                } else {
                    None
                }
            };
            let task_cleanup =
                if let Some(task) = active.and_then(|mut active| active.deadline_task.take()) {
                    if task.join().await.is_err() {
                        cleanup_failure(
                            "turn_join_failed",
                            "Pi RPC turn deadline task did not join cleanly",
                        )
                    } else {
                        CleanupOutcome::NotApplicable
                    }
                } else {
                    CleanupOutcome::NotApplicable
                };
            CleanupOutcome::merge(
                CleanupOutcome::merge(cancel_cleanup, task_cleanup),
                self.attachment.release().await,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::session::ActiveTask;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicU32;
    use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
    use swallowtail_runtime::{
        Deadline, DeadlineObservation, HostServices, JoinedTask, MonotonicInstant, ProcessExit,
        ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ScopeId, ScopedTaskService,
        TimeService,
    };

    struct RefusingStdin;

    impl ProcessHandle for RefusingStdin {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.stdin_refused",
                    "stdin refused",
                )))
            })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct DummyTaskService;

    impl ScopedTaskService for DummyTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            _task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "test.unused",
                "unused",
            )))
        }
    }

    struct DummyTimeService;

    impl TimeService for DummyTimeService {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(0)
        }

        fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(std::future::pending())
        }
    }

    struct WatchTurnFinish {
        turn: Arc<ActiveTurn>,
    }

    impl JoinedTask for WatchTurnFinish {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async move {
                let mut finished = Box::pin(self.turn.finished_future());
                let mut polls = 0usize;
                std::future::poll_fn(move |context| {
                    if std::future::Future::poll(finished.as_mut(), context).is_ready() {
                        return std::task::Poll::Ready(Ok(()));
                    }
                    polls += 1;
                    if polls > 16 {
                        return std::task::Poll::Ready(Err(RuntimeFailure::new(
                            SafeDiagnostic::new(
                                "test.deadline_watch_hung",
                                "deadline watcher waited past rejected abort",
                            ),
                        )));
                    }
                    context.waker().wake_by_ref();
                    std::task::Poll::Pending
                })
                .await
            })
        }
    }

    #[test]
    fn rejected_abort_during_close_wakes_deadline_task_and_reports_cleanup_failure() {
        let services =
            HostServices::new(ExecutionHostId::new("pi.rpc.close-test").expect("host id is valid"));
        let connection = PiConnection::new(
            Arc::new(RefusingStdin),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-close").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        let active: ActiveSlot = Arc::new(Mutex::new(Some(ActiveTask {
            turn: Arc::clone(&turn),
            deadline_task: Some(Box::new(WatchTurnFinish {
                turn: Arc::clone(&turn),
            })),
            attachment: SharedAttachmentMaterialization::default(),
        })));
        let connection_ref = Arc::clone(&connection);
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("initial turn attaches");
        let handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-close").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection,
                turn: Arc::clone(&turn),
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        let handle: Box<dyn TurnHandle> = Box::new(handle);
        let cleanup = futures_executor::block_on(handle.close());
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diagnostic) if diagnostic.code() == "swallowtail.pi.rpc.cleanup_failed" && diagnostic.message().contains("cancel_rejected")),
            "expected cancel_rejected cleanup failure, got {cleanup:?}"
        );
        assert!(
            turn.is_finished(),
            "turn must finish so deadline task wakes"
        );
        let (next_turn, _next_events, _next_callbacks, _next_terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-next").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection_ref),
            false,
        )
        .expect("next turn constructs");
        let next_err = connection_ref
            .set_active_turn(next_turn)
            .expect_err("connection must refuse a replacement turn after rejected abort");
        assert_eq!(
            next_err.diagnostic().code(),
            "swallowtail.pi.rpc.connection_closed"
        );
    }

    struct HeldRefusingStdin {
        release: Mutex<Option<futures_channel::oneshot::Receiver<()>>>,
    }

    impl ProcessHandle for HeldRefusingStdin {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            let receiver = self.release.lock().expect("release lock").take();
            Box::pin(async move {
                if let Some(receiver) = receiver {
                    let _ = receiver.await;
                }
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.late_abort_refused",
                    "late abort refused",
                )))
            })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    #[test]
    fn late_abort_refusal_after_original_turn_settles_preserves_replacement_turn() {
        use std::task::{Context, Poll, Waker};

        let (release_tx, release_rx) = futures_channel::oneshot::channel();
        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.late-refusal-test").expect("host id is valid"),
        );
        let connection = PiConnection::new(
            Arc::new(HeldRefusingStdin {
                release: Mutex::new(Some(release_rx)),
            }),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn1, events1, callbacks1, terminal1) = ActiveTurn::new(
            RuntimeTurnId::new("turn-1").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn1 constructs");
        connection
            .set_active_turn(Arc::clone(&turn1))
            .expect("turn1 attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let handle1 = PiTurnHandle::new(
            RuntimeTurnId::new("turn-1").expect("turn id is valid"),
            events1,
            callbacks1,
            Box::pin(terminal1),
            super::PiTurnBinding {
                connection: Arc::clone(&connection),
                turn: Arc::clone(&turn1),
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );

        // 1. Start abort on turn1; it suspends before the refusal arrives.
        let mut cancel_fut = handle1.cancellation().request();
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(
            matches!(cancel_fut.as_mut().poll(&mut cx), Poll::Pending),
            "abort should remain in-flight while turn1 settles"
        );

        // 2. Original turn1 settles and clears the slot before the abort refusal arrives.
        turn1.fail_connection(SafeDiagnostic::new("test.settled", "turn1 settled"));
        connection.clear_active_turn(&turn1);

        // 3. Replacement turn2 is admitted onto the healthy connection.
        let (turn2, _events2, _callbacks2, _terminal2) = ActiveTurn::new(
            RuntimeTurnId::new("turn-2").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn2 constructs");
        connection
            .set_active_turn(Arc::clone(&turn2))
            .expect("replacement turn2 must admit after turn1 settled");

        // 4. Late abort refusal for turn1 resolves.
        release_tx.send(()).expect("release sends");
        let cancel_err = futures_executor::block_on(cancel_fut)
            .expect_err("late abort refusal still returns its error to caller");
        assert_eq!(cancel_err.diagnostic().code(), "test.late_abort_refused");

        // 5. Replacement turn2 and the shared connection must remain intact.
        assert!(
            !turn2.is_finished(),
            "late refusal for turn1 must not terminate replacement turn2"
        );
        drop(cancel_err);
        let close1 = futures_executor::block_on(Box::new(handle1).close());
        assert_eq!(close1, CleanupOutcome::NotApplicable);

        // 6. Once turn2 finishes, a subsequent turn3 can still be admitted.
        turn2.fail_connection(SafeDiagnostic::new("test.done", "turn2 done"));
        connection.clear_active_turn(&turn2);
        let (turn3, _events3, _callbacks3, _terminal3) = ActiveTurn::new(
            RuntimeTurnId::new("turn-3").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn3 constructs");
        connection
            .set_active_turn(turn3)
            .expect("connection must stay open after a late refusal for an already-settled turn");
    }

    #[test]
    fn rejected_abort_resolves_terminal_outcome_with_cancel_rejected_diagnostic() {
        use swallowtail_runtime::TerminalStatus;

        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.cancel-rejected-test").expect("host id is valid"),
        );
        let connection = PiConnection::new(
            Arc::new(RefusingStdin),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-cancel-rejected").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let mut handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-cancel-rejected").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection,
                turn,
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");
        let cancel_err = futures_executor::block_on(handle.cancellation().request())
            .expect_err("failed abort must return error");
        assert_eq!(cancel_err.diagnostic().code(), "test.stdin_refused");

        let outcome = futures_executor::block_on(terminal_fut);
        assert!(
            matches!(
                outcome.status(),
                TerminalStatus::RuntimeFailed(diagnostic)
                    if diagnostic.code() == "swallowtail.pi.rpc.cancel_rejected"
            ),
            "rejected abort must resolve with cancel_rejected diagnostic, got {:?}",
            outcome.status()
        );
    }

    #[test]
    fn abort_write_followed_by_eof_before_ack_resolves_cancel_rejected() {
        use swallowtail_runtime::TerminalStatus;

        struct WriteOkThenEof {
            written: Mutex<Option<futures_channel::oneshot::Sender<()>>>,
            eof_gate: Mutex<Option<futures_channel::oneshot::Receiver<()>>>,
        }

        impl ProcessHandle for WriteOkThenEof {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                if let Some(tx) = self.written.lock().expect("written lock").take() {
                    let _ = tx.send(());
                }
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                let rx = self.eof_gate.lock().expect("eof lock").take();
                Box::pin(async move {
                    if let Some(rx) = rx {
                        let _ = rx.await;
                    }
                    Ok(None)
                })
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
            }
        }

        let (written_tx, written_rx) = futures_channel::oneshot::channel();
        let (eof_tx, eof_rx) = futures_channel::oneshot::channel();
        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.abort-eof-test").expect("host id is valid"),
        );
        let connection = PiConnection::new(
            Arc::new(WriteOkThenEof {
                written: Mutex::new(Some(written_tx)),
                eof_gate: Mutex::new(Some(eof_rx)),
            }),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-abort-eof").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let mut handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-abort-eof").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection: Arc::clone(&connection),
                turn,
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");

        let pump_conn = Arc::clone(&connection);
        let pump_thread = std::thread::spawn(move || {
            futures_executor::block_on(pump_conn.pump());
        });

        let cancel_thread =
            std::thread::spawn(move || futures_executor::block_on(handle.cancellation().request()));

        // Wait until the abort command write has succeeded, then trigger EOF
        // before any abort response arrives.
        futures_executor::block_on(written_rx).expect("abort write completed");
        eof_tx.send(()).expect("trigger EOF");

        let cancel_result = cancel_thread.join().expect("cancel thread joins");
        assert!(
            cancel_result.is_err(),
            "in-flight abort interrupted by EOF must return error"
        );
        pump_thread.join().expect("pump thread joins");

        let outcome = futures_executor::block_on(terminal_fut);
        assert!(
            matches!(
                outcome.status(),
                TerminalStatus::RuntimeFailed(diagnostic)
                    if diagnostic.code() == "swallowtail.pi.rpc.cancel_rejected"
            ),
            "EOF before abort acknowledgement must resolve with cancel_rejected, got {:?}",
            outcome.status()
        );
    }

    #[test]
    fn successful_abort_response_immediately_followed_by_eof_resolves_cancelled() {
        use std::task::{Context, Poll, Waker};
        use swallowtail_runtime::{ProcessOutputStream, TerminalStatus};

        struct AckThenEof {
            response: Mutex<Option<Vec<u8>>>,
        }

        impl ProcessHandle for AckThenEof {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                let next = self.response.lock().expect("response lock").take();
                Box::pin(async move {
                    Ok(next
                        .map(|bytes| ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes)))
                })
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
            }
        }

        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.abort-ack-eof-test").expect("host id is valid"),
        );
        let ack_line = b"{\"type\":\"response\",\"id\":\"abort:turn-ack-eof\",\"command\":\"abort\",\"success\":true}\n".to_vec();
        let connection = PiConnection::new(
            Arc::new(AckThenEof {
                response: Mutex::new(Some(ack_line)),
            }),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-ack-eof").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let mut handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-ack-eof").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection: Arc::clone(&connection),
                turn,
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");

        // Poll cancellation once so the abort command is registered and written,
        // then run the entire pump (ack + EOF) to completion BEFORE polling the
        // cancellation future again.
        let mut cancel_fut = handle.cancellation().request();
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(matches!(cancel_fut.as_mut().poll(&mut cx), Poll::Pending));

        futures_executor::block_on(Arc::clone(&connection).pump());

        let outcome = futures_executor::block_on(terminal_fut);
        assert_eq!(
            outcome.status(),
            &TerminalStatus::Cancelled,
            "successful abort reply immediately followed by EOF must resolve Cancelled"
        );
        assert!(futures_executor::block_on(cancel_fut).is_ok());
    }

    #[test]
    fn normal_rpc_settlement_aborted_false_racing_rejected_abort_resolves_completed() {
        use crate::protocol::PiAgentEvent;
        use std::task::{Context, Poll, Waker};
        use swallowtail_runtime::{TerminalStatus, TokenUsage};

        let (release_tx, release_rx) = futures_channel::oneshot::channel();
        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.normal-settle-race").expect("host id is valid"),
        );
        let connection = PiConnection::new(
            Arc::new(HeldRefusingStdin {
                release: Mutex::new(Some(release_rx)),
            }),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-normal-race").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            true,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let mut handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-normal-race").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection: Arc::clone(&connection),
                turn: Arc::clone(&turn),
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");

        // Start abort (in-flight), then provider authoritatively settles normally
        // with `aborted: Some(false)`.
        let mut cancel_fut = handle.cancellation().request();
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(matches!(cancel_fut.as_mut().poll(&mut cx), Poll::Pending));

        turn.handle_event(PiAgentEvent::MessageEnded(Some(TokenUsage::new(
            Some(10),
            Some(5),
        ))))
        .expect("usage recorded");
        turn.handle_event(PiAgentEvent::Settled {
            aborted: Some(false),
        })
        .expect("normal settlement succeeds");

        release_tx.send(()).expect("release rejected abort");
        assert!(futures_executor::block_on(cancel_fut).is_err());

        let outcome = futures_executor::block_on(terminal_fut);
        assert_eq!(
            outcome.status(),
            &TerminalStatus::Completed,
            "explicit RPC aborted:false settlement must resolve Completed despite in-flight abort"
        );
    }

    #[test]
    fn settlement_before_successful_abort_reply_resolves_cancelled_without_usage_missing() {
        use std::task::{Context, Poll, Waker};
        use swallowtail_runtime::{ProcessOutputStream, TerminalStatus};

        struct SettleThenAck {
            lines: Mutex<std::collections::VecDeque<Vec<u8>>>,
        }
        impl ProcessHandle for SettleThenAck {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                let line = self.lines.lock().expect("lines lock").pop_front();
                Box::pin(async move {
                    Ok(line
                        .map(|bytes| ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes)))
                })
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
            }
        }

        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.settle-before-ack-test").expect("host id is valid"),
        );
        let lines = std::collections::VecDeque::from([
            b"{\"type\":\"agent_settled\",\"aborted\":false}\n".to_vec(),
            b"{\"type\":\"response\",\"id\":\"abort:turn-settle-ack\",\"command\":\"abort\",\"success\":true}\n".to_vec(),
        ]);
        let connection = PiConnection::new(
            Arc::new(SettleThenAck {
                lines: Mutex::new(lines),
            }),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-settle-ack").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let mut handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-settle-ack").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection: Arc::clone(&connection),
                turn,
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");

        let mut cancel_fut = handle.cancellation().request();
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(matches!(cancel_fut.as_mut().poll(&mut cx), Poll::Pending));

        futures_executor::block_on(Arc::clone(&connection).pump());

        let outcome = futures_executor::block_on(terminal_fut);
        assert_eq!(
            outcome.status(),
            &TerminalStatus::Cancelled,
            "settlement before a successful abort reply must resolve Cancelled without usage_missing"
        );
        assert!(futures_executor::block_on(cancel_fut).is_ok());
    }

    #[test]
    fn negative_abort_reply_immediately_followed_by_eof_resolves_cancel_rejected() {
        use std::task::{Context, Poll, Waker};
        use swallowtail_runtime::{ProcessOutputStream, TerminalStatus};

        struct NackThenEof {
            response: Mutex<Option<Vec<u8>>>,
        }
        impl ProcessHandle for NackThenEof {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                let line = self.response.lock().expect("nack lock").take();
                Box::pin(async move {
                    Ok(line
                        .map(|bytes| ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes)))
                })
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
            }
        }

        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.abort-nack-eof-test").expect("host id is valid"),
        );
        let nack_line = b"{\"type\":\"response\",\"id\":\"abort:turn-nack-eof\",\"command\":\"abort\",\"success\":false}\n".to_vec();
        let connection = PiConnection::new(
            Arc::new(NackThenEof {
                response: Mutex::new(Some(nack_line)),
            }),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-nack-eof").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let mut handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-nack-eof").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection: Arc::clone(&connection),
                turn,
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");

        let mut cancel_fut = handle.cancellation().request();
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(matches!(cancel_fut.as_mut().poll(&mut cx), Poll::Pending));

        futures_executor::block_on(Arc::clone(&connection).pump());

        let outcome = futures_executor::block_on(terminal_fut);
        match outcome.status() {
            TerminalStatus::RuntimeFailed(diag) => {
                assert_eq!(diag.code(), "swallowtail.pi.rpc.cancel_rejected");
            }
            other => panic!("expected cancel_rejected RuntimeFailed, got {other:?}"),
        }
        let cancel_err = futures_executor::block_on(cancel_fut)
            .expect_err("rejected abort must return an error");
        assert_eq!(
            cancel_err.diagnostic().code(),
            "swallowtail.pi.rpc.abort_rejected"
        );
    }

    #[test]
    fn reconcile_abort_response_activity_overflow_wakes_abort_waiter() {
        use crate::protocol::PiAgentEvent;
        use std::task::{Context, Poll, Waker};
        use swallowtail_runtime::ProcessOutputStream;

        struct SettleThenAck {
            lines: Mutex<std::collections::VecDeque<Vec<u8>>>,
        }
        impl ProcessHandle for SettleThenAck {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                let line = self.lines.lock().expect("lines lock").pop_front();
                Box::pin(async move {
                    Ok(line
                        .map(|bytes| ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes)))
                })
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
            }
        }

        let services = HostServices::new(
            ExecutionHostId::new("pi.rpc.reconcile-overflow-test").expect("host id is valid"),
        );
        let lines = std::collections::VecDeque::from([
            b"{\"type\":\"agent_settled\",\"aborted\":false}\n".to_vec(),
            b"{\"type\":\"response\",\"id\":\"abort:turn-overflow\",\"command\":\"abort\",\"success\":true}\n".to_vec(),
        ]);
        let connection = PiConnection::new(
            Arc::new(SettleThenAck {
                lines: Mutex::new(lines),
            }),
            Arc::new(DummyTaskService),
            Arc::new(DummyTimeService),
            services,
        );
        let (turn, events, callbacks, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-overflow").expect("turn id is valid"),
            Arc::new(AtomicU32::new(0)),
            Arc::downgrade(&connection),
            false,
        )
        .expect("turn constructs");
        connection
            .set_active_turn(Arc::clone(&turn))
            .expect("turn attaches");
        let active: ActiveSlot = Arc::new(Mutex::new(None));
        let mut handle = PiTurnHandle::new(
            RuntimeTurnId::new("turn-overflow").expect("turn id is valid"),
            events,
            callbacks,
            Box::pin(terminal),
            super::PiTurnBinding {
                connection: Arc::clone(&connection),
                turn: Arc::clone(&turn),
                active,
                attachment: SharedAttachmentMaterialization::default(),
                command_dispositions: false,
            },
        );
        // Keep the event stream receiver alive without draining so the buffer
        // fills completely and deferred activity completion fails on send.
        let _events = handle.take_events().expect("event stream is present");
        let terminal_fut = handle
            .take_terminal_outcome()
            .expect("terminal outcome future is present");

        turn.handle_event(PiAgentEvent::ToolStarted {
            call_id: "call-1".to_owned(),
            name: "read".to_owned(),
        })
        .expect("tool start emits activity");
        for _ in 0..254 {
            turn.handle_event(PiAgentEvent::Progress)
                .expect("progress fills event buffer");
        }

        let mut cancel_fut = handle.cancellation().request();
        let waker = Waker::noop();
        let mut cx = Context::from_waker(waker);
        assert!(matches!(cancel_fut.as_mut().poll(&mut cx), Poll::Pending));

        futures_executor::block_on(Arc::clone(&connection).pump());

        // Must resolve with RuntimeFailed (not Cancelled) and wake the waiter.
        let cancel_err = futures_executor::block_on(cancel_fut)
            .expect_err("reconciliation overflow must wake abort waiter with Err");
        let outcome = futures_executor::block_on(terminal_fut);
        match outcome.status() {
            swallowtail_runtime::TerminalStatus::RuntimeFailed(diag) => {
                assert_eq!(diag.code(), cancel_err.diagnostic().code());
            }
            other => panic!("expected RuntimeFailed on reconciliation overflow, got {other:?}"),
        }
    }
}
