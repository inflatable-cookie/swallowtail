use super::{CommandResult, OhMyPiConnection};
use crate::failure::{failure, protocol_failure};
use crate::protocol::{OhMyPiRpcDecoder, OhMyPiRpcRecord};
use crate::turn::CallbackTimer;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, ProcessOutputStream, RuntimeFailure, ScopeId,
};

impl OhMyPiConnection {
    pub(crate) async fn pump(self: std::sync::Arc<Self>) {
        let mut decoder = OhMyPiRpcDecoder::new();
        let mut transport_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    match decoder.push(chunk.bytes()) {
                        Ok(records) => {
                            for record in records {
                                if let Err(error) = self.dispatch(record) {
                                    transport_failure = Some(error);
                                    break;
                                }
                            }
                        }
                        Err(_) => transport_failure = Some(protocol_failure()),
                    }
                    if transport_failure.is_some() {
                        break;
                    }
                }
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(error) => {
                    transport_failure = Some(error);
                    break;
                }
            }
        }
        if transport_failure.is_none() && decoder.finish().is_err() {
            transport_failure = Some(protocol_failure());
        }
        if transport_failure.is_some() {
            let _ = self.process.force_stop().await;
        }
        let waited = self.process.wait().await;
        self.closed.store(true, std::sync::atomic::Ordering::SeqCst);
        let cleanup = if waited.is_ok() {
            CleanupOutcome::Clean
        } else {
            CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.oh_my_pi.rpc.process_cleanup_failed",
                "OhMyPi RPC process cleanup failed",
            ))
        };
        *self.cleanup.lock().expect("OhMyPi cleanup lock poisoned") = Some(cleanup);
        let error = transport_failure.unwrap_or_else(|| {
            failure(
                "swallowtail.oh_my_pi.rpc.connection_ended",
                "OhMyPi RPC connection ended",
            )
        });
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("OhMyPi active lock poisoned")
            .take()
            && !turn.is_finished()
        {
            turn.fail_connection(error.diagnostic().clone());
        }
        self.fail_pending(error);
    }

    fn dispatch(self: &Arc<Self>, record: OhMyPiRpcRecord) -> Result<(), RuntimeFailure> {
        match record {
            OhMyPiRpcRecord::Response(response) => {
                let mut pending_commands =
                    self.pending.lock().expect("OhMyPi pending lock poisoned");
                let pending = pending_commands.get(&response.id).ok_or_else(|| {
                    failure(
                        "swallowtail.oh_my_pi.rpc.response_unknown",
                        "OhMyPi RPC returned an unknown command response",
                    )
                })?;
                if pending.command != response.command {
                    return Err(failure(
                        "swallowtail.oh_my_pi.rpc.response_command_mismatch",
                        "OhMyPi RPC response command did not match its request",
                    ));
                }
                let pending = pending_commands
                    .remove(&response.id)
                    .expect("validated OhMyPi pending command exists");
                drop(pending_commands);
                pending.sender.complete(Ok(CommandResult {
                    success: response.success,
                    data: response.data,
                }));
                Ok(())
            }
            OhMyPiRpcRecord::AgentEvent(event) => self.with_active(|turn| turn.handle_event(event)),
            OhMyPiRpcRecord::UiDialog(dialog) => {
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
            OhMyPiRpcRecord::UiDisplay(display) => {
                match self
                    .active_turn
                    .lock()
                    .expect("OhMyPi active lock poisoned")
                    .clone()
                {
                    Some(turn) => turn.handle_display(display),
                    None => Ok(()),
                }
            }
            OhMyPiRpcRecord::Lifecycle => Ok(()),
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
            .expect("OhMyPi active lock poisoned")
            .clone()
            .ok_or_else(|| {
                failure(
                    "swallowtail.oh_my_pi.rpc.event_without_turn",
                    "OhMyPi RPC emitted a model event without an active turn",
                )
            })
    }

    fn spawn_callback_timer(
        self: &Arc<Self>,
        turn: Arc<crate::turn::ActiveTurn>,
        timer: CallbackTimer,
    ) -> Result<(), RuntimeFailure> {
        let scope = ScopeId::new(format!(
            "oh-my-pi-rpc:callback:{}",
            timer.callback_id.as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.oh_my_pi.rpc.callback_scope_invalid",
                "OhMyPi RPC callback deadline scope was invalid",
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
            .expect("OhMyPi callback-task lock poisoned")
            .push(task);
        Ok(())
    }

    fn fail_pending(&self, error: RuntimeFailure) {
        let pending =
            std::mem::take(&mut *self.pending.lock().expect("OhMyPi pending lock poisoned"));
        for (_, pending) in pending {
            pending.sender.complete(Err(error.clone()));
        }
    }
}
