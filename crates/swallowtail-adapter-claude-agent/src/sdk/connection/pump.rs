use super::{CommandResult, SdkConnection};
use crate::sdk::failure::{failure, protocol_failure};
use crate::sdk::wire::{ClaudeAgentSdkDecoder, ClaudeAgentSdkRecord};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use swallowtail_runtime::{ProcessOutputStream, RuntimeFailure};

impl SdkConnection {
    /// Drains the sidecar stdout wire until it ends or fails, then joins the
    /// sidecar process and records whether its exit was actually observed.
    pub(crate) async fn pump(self: Arc<Self>) {
        let mut decoder = ClaudeAgentSdkDecoder::new();
        let mut transport_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    match decoder.push(chunk.bytes()) {
                        Ok(records) => {
                            for record in records {
                                if let Err(error) = self.dispatch(record) {
                                    self.emit_protocol_debug(&error, "sdk.pump.dispatch");
                                    transport_failure = Some(error);
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            let error = protocol_failure();
                            self.emit_protocol_debug(&error, "sdk.pump.decode");
                            transport_failure = Some(error);
                        }
                    }
                    if transport_failure.is_some() {
                        break;
                    }
                }
                // Sidecar stderr carries only bounded host-owned diagnostics;
                // chunks are dropped without inspection.
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(error) => {
                    self.emit_protocol_debug(&error, "sdk.pump.read");
                    transport_failure = Some(error);
                    break;
                }
            }
        }
        if transport_failure.is_none() && decoder.finish().is_err() {
            let error = protocol_failure();
            self.emit_protocol_debug(&error, "sdk.pump.finish");
            transport_failure = Some(error);
        }
        if transport_failure.is_some() {
            let _ = self.escalate().await;
        }
        // An observed exit is the only evidence of exit. A wait failure is
        // recorded as "not observed", never as a clean stop.
        let observed = self.process.wait().await.is_ok();
        *self
            .exit_observed
            .lock()
            .expect("SDK sidecar exit lock poisoned") = Some(observed);
        let error = transport_failure.unwrap_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.connection_ended",
                "Claude Agent SDK sidecar connection ended",
            )
        });
        self.record_terminal_error(&error);
        self.closed.store(true, Ordering::SeqCst);
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("SDK sidecar active lock poisoned")
            .take()
            && !turn.is_finished()
        {
            turn.fail_connection(error.diagnostic().clone());
        }
        self.fail_pending(error);
    }

    fn dispatch(self: &Arc<Self>, record: ClaudeAgentSdkRecord) -> Result<(), RuntimeFailure> {
        match record {
            ClaudeAgentSdkRecord::Response(response) => {
                let mut pending_commands = self
                    .pending
                    .lock()
                    .expect("SDK sidecar pending lock poisoned");
                let pending = pending_commands.get(&response.id).ok_or_else(|| {
                    failure(
                        "swallowtail.claude-agent.sdk.response_unknown",
                        "Claude Agent SDK sidecar returned an unknown command response",
                    )
                })?;
                if pending.command.as_str() != response.command {
                    return Err(failure(
                        "swallowtail.claude-agent.sdk.response_command_mismatch",
                        "Claude Agent SDK sidecar response command did not match its request",
                    ));
                }
                let pending = pending_commands
                    .remove(&response.id)
                    .expect("validated SDK sidecar pending command exists");
                drop(pending_commands);
                pending.sender.complete(Ok(CommandResult {
                    success: response.success,
                    data: response.data,
                }));
                Ok(())
            }
            ClaudeAgentSdkRecord::Event(event) => {
                let active = self
                    .active_turn
                    .lock()
                    .expect("SDK sidecar active lock poisoned")
                    .clone();
                match active {
                    Some(turn) => turn.handle_event(event),
                    None => Err(failure(
                        "swallowtail.claude-agent.sdk.event_without_turn",
                        "Claude Agent SDK sidecar emitted a turn event outside an active turn",
                    )),
                }
            }
            ClaudeAgentSdkRecord::Callback(callback) => {
                let active = self
                    .active_turn
                    .lock()
                    .expect("SDK sidecar active lock poisoned")
                    .clone();
                match active {
                    Some(turn) => turn.handle_admission(&callback.id, &callback.tool_name),
                    None => Err(failure(
                        "swallowtail.claude-agent.sdk.admission_without_turn",
                        "Claude Agent SDK sidecar requested tool admission outside an active turn",
                    )),
                }
            }
            ClaudeAgentSdkRecord::Terminal(_) => Err(failure(
                "swallowtail.claude-agent.sdk.sidecar_terminated",
                "Claude Agent SDK sidecar reported a terminal failure",
            )),
            // Diagnostics are bounded, redacted, and observation-only.
            ClaudeAgentSdkRecord::Diagnostic(_) => Ok(()),
        }
    }

    fn fail_pending(&self, error: RuntimeFailure) {
        let pending = std::mem::take(
            &mut *self
                .pending
                .lock()
                .expect("SDK sidecar pending lock poisoned"),
        );
        for (_, command) in pending {
            command
                .sender
                .complete(Err(RuntimeFailure::new(error.diagnostic().clone())));
        }
    }
}
