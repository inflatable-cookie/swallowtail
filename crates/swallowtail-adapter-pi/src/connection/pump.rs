use super::{CommandResult, PiConnection};
use crate::failure::{failure, protocol_failure};
use crate::protocol::{PiRpcDecoder, PiRpcRecord};
use crate::turn::CallbackTimer;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, ProcessOutputStream, RuntimeFailure, ScopeId,
};

impl PiConnection {
    pub(crate) async fn pump(self: std::sync::Arc<Self>) {
        let mut decoder = PiRpcDecoder::new();
        let mut transport_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    match decoder.push(chunk.bytes()) {
                        Ok(records) => {
                            for record in records {
                                if let Err(error) = self.dispatch(record) {
                                    self.emit_protocol_debug(&error, "rpc.pump.dispatch");
                                    transport_failure = Some(error);
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            let error = protocol_failure();
                            self.emit_protocol_debug(&error, "rpc.pump.decode");
                            transport_failure = Some(error);
                        }
                    }
                    if transport_failure.is_some() {
                        break;
                    }
                }
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(error) => {
                    self.emit_protocol_debug(&error, "rpc.pump.read");
                    transport_failure = Some(error);
                    break;
                }
            }
        }
        if transport_failure.is_none() && decoder.finish().is_err() {
            let error = protocol_failure();
            self.emit_protocol_debug(&error, "rpc.pump.finish");
            transport_failure = Some(error);
        }
        let needs_stop = transport_failure.is_some();
        let error = transport_failure.unwrap_or_else(|| {
            failure(
                "swallowtail.pi.rpc.connection_ended",
                "Pi RPC connection ended",
            )
        });
        // No response can arrive after this pump has ended. Close command
        // admission and resolve existing waiters before force-stop or process
        // wait can suspend, so a later command cannot register against a dead
        // reader and hang the shutdown.
        self.closed.store(true, std::sync::atomic::Ordering::SeqCst);
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("Pi active lock poisoned")
            .take()
            && !turn.is_finished()
        {
            turn.fail_connection(error.diagnostic().clone());
        }
        self.fail_pending(error);
        if needs_stop {
            let _ = self.process.force_stop().await;
        }
        let waited = self.process.wait().await;
        // A non-zero exit is a failed cleanup, as the SDK sidecar reports it.
        let cleanup = match waited {
            Ok(exit) if exit.success() => CleanupOutcome::Clean,
            Ok(_) | Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.pi.rpc.process_cleanup_failed",
                "Pi RPC process cleanup failed",
            )),
        };
        *self.cleanup.lock().expect("Pi cleanup lock poisoned") = Some(cleanup);
    }

    fn dispatch(self: &Arc<Self>, record: PiRpcRecord) -> Result<(), RuntimeFailure> {
        match record {
            PiRpcRecord::Response(response) => {
                let mut pending_commands = self.pending.lock().expect("Pi pending lock poisoned");
                let pending = pending_commands.get(&response.id).ok_or_else(|| {
                    failure(
                        "swallowtail.pi.rpc.response_unknown",
                        "Pi RPC returned an unknown command response",
                    )
                })?;
                if pending.command != response.command {
                    return Err(failure(
                        "swallowtail.pi.rpc.response_command_mismatch",
                        "Pi RPC response command did not match its request",
                    ));
                }
                let pending = pending_commands
                    .remove(&response.id)
                    .expect("validated Pi pending command exists");
                drop(pending_commands);
                if let Some(turn) = pending.abort_turn {
                    turn.reconcile_abort_response(response.success)?;
                }
                pending.sender.complete(Ok(CommandResult {
                    success: response.success,
                    data: response.data,
                }));
                Ok(())
            }
            PiRpcRecord::AgentEvent(event) => self.with_active(|turn| turn.handle_event(event)),
            PiRpcRecord::UiDialog(dialog) => {
                let deadline = dialog.timeout_millis.map(|millis| {
                    Deadline::at(MonotonicInstant::from_ticks(
                        self.time.now().ticks().saturating_add(millis),
                    ))
                });
                let turn = self.active_turn()?;
                if let Some(timer) = turn.handle_dialog(dialog, deadline)? {
                    self.spawn_callback_timer(turn, timer)?;
                }
                Ok(())
            }
            PiRpcRecord::UiDisplay(display) => {
                self.with_active(|turn| turn.handle_display(display))
            }
        }
    }

    fn with_active<T>(
        &self,
        operation: impl FnOnce(&crate::turn::ActiveTurn) -> Result<T, RuntimeFailure>,
    ) -> Result<T, RuntimeFailure> {
        operation(self.active_turn()?.as_ref())
    }

    fn active_turn(&self) -> Result<Arc<crate::turn::ActiveTurn>, RuntimeFailure> {
        self.active_turn
            .lock()
            .expect("Pi active lock poisoned")
            .clone()
            .ok_or_else(|| {
                failure(
                    "swallowtail.pi.rpc.event_without_turn",
                    "Pi RPC emitted a model event without an active turn",
                )
            })
    }

    fn spawn_callback_timer(
        self: &Arc<Self>,
        turn: Arc<crate::turn::ActiveTurn>,
        timer: CallbackTimer,
    ) -> Result<(), RuntimeFailure> {
        let scope = ScopeId::new(format!("pi-rpc:callback:{}", timer.callback_id.as_str()))
            .map_err(|_| {
                failure(
                    "swallowtail.pi.rpc.callback_scope_invalid",
                    "Pi RPC callback deadline scope was invalid",
                )
            })?;
        let mut wait = self.time.wait_until(timer.deadline);
        let mut finished = Box::pin(turn.callback_finished(timer.callback_id.clone()));
        let connection = Arc::clone(self);
        let callback_id = timer.callback_id;
        let task = self.task.spawn(
            scope,
            Box::pin(async move {
                let timed_out = poll_fn(|context| {
                    if finished.as_mut().poll(context).is_ready() {
                        Poll::Ready(false)
                    } else if wait.as_mut().poll(context).is_ready() {
                        Poll::Ready(true)
                    } else {
                        Poll::Pending
                    }
                })
                .await;
                if timed_out
                    && let Some(value) = turn.expire_callback(&callback_id)
                    && connection.write_value(value).await.is_err()
                {
                    let _ = connection.force_stop().await;
                }
            }),
        )?;
        self.callback_tasks
            .lock()
            .expect("Pi callback-task lock poisoned")
            .push(task);
        Ok(())
    }

    pub(super) fn fail_pending(&self, error: RuntimeFailure) {
        let pending = std::mem::take(&mut *self.pending.lock().expect("Pi pending lock poisoned"));
        for (_, pending) in pending {
            pending.sender.complete(Err(error.clone()));
        }
    }
}
