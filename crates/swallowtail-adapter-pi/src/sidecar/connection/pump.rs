use super::{CommandResult, SidecarConnection};
use crate::sidecar::failure::{failure, protocol_failure};
use crate::sidecar::wire::{PiSdkSidecarDecoder, PiSdkSidecarEvent, PiSdkSidecarRecord};
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{CleanupOutcome, ProcessOutputStream, RuntimeFailure};

impl SidecarConnection {
    pub(crate) async fn pump(self: std::sync::Arc<Self>) {
        let mut decoder = PiSdkSidecarDecoder::new();
        let mut transport_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    match decoder.push(chunk.bytes()) {
                        Ok(records) => {
                            for record in records {
                                if let Err(error) = self.dispatch(record) {
                                    self.emit_protocol_debug(&error, "sdk-sidecar.pump.dispatch");
                                    transport_failure = Some(error);
                                    break;
                                }
                            }
                        }
                        Err(_) => {
                            let error = protocol_failure();
                            self.emit_protocol_debug(&error, "sdk-sidecar.pump.decode");
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
                    self.emit_protocol_debug(&error, "sdk-sidecar.pump.read");
                    transport_failure = Some(error);
                    break;
                }
            }
        }
        if transport_failure.is_none() && decoder.finish().is_err() {
            let error = protocol_failure();
            self.emit_protocol_debug(&error, "sdk-sidecar.pump.finish");
            transport_failure = Some(error);
        }
        if transport_failure.is_some() {
            let _ = self.process.force_stop().await;
        }
        let waited = self.process.wait().await;
        let cleanup = if waited.is_ok() {
            CleanupOutcome::Clean
        } else {
            CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.pi.sdk-sidecar.process_cleanup_failed",
                "Pi SDK sidecar process cleanup failed",
            ))
        };
        *self.cleanup.lock().expect("sidecar cleanup lock poisoned") = Some(cleanup);
        let error = transport_failure.unwrap_or_else(|| {
            failure(
                "swallowtail.pi.sdk-sidecar.connection_ended",
                "Pi SDK sidecar connection ended",
            )
        });
        self.record_terminal_error(&error);
        self.closed.store(true, std::sync::atomic::Ordering::SeqCst);
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("sidecar active lock poisoned")
            .take()
            && !turn.is_finished()
        {
            turn.fail_connection(error.diagnostic().clone());
        }
        self.fail_pending(error);
    }

    fn dispatch(self: &Arc<Self>, record: PiSdkSidecarRecord) -> Result<(), RuntimeFailure> {
        match record {
            PiSdkSidecarRecord::Response(response) => {
                let mut pending_commands =
                    self.pending.lock().expect("sidecar pending lock poisoned");
                let pending = pending_commands.get(&response.id).ok_or_else(|| {
                    failure(
                        "swallowtail.pi.sdk-sidecar.response_unknown",
                        "Pi SDK sidecar returned an unknown command response",
                    )
                })?;
                if pending.command.as_str() != response.command {
                    return Err(failure(
                        "swallowtail.pi.sdk-sidecar.response_command_mismatch",
                        "Pi SDK sidecar response command did not match its request",
                    ));
                }
                let pending = pending_commands
                    .remove(&response.id)
                    .expect("validated sidecar pending command exists");
                drop(pending_commands);
                pending.sender.complete(Ok(CommandResult {
                    success: response.success,
                    data: response.data,
                }));
                Ok(())
            }
            PiSdkSidecarRecord::Event(PiSdkSidecarEvent::ReplayItem { sequence, item }) => {
                // Replay items belong to an armed load replay phase; anywhere
                // else they fail the transport closed.
                let mut replay = self.replay.lock().expect("sidecar replay lock poisoned");
                match replay.as_mut() {
                    Some(collector) => collector.push(sequence, item),
                    None => Err(failure(
                        "swallowtail.pi.sdk-sidecar.replay_unexpected",
                        "Pi SDK sidecar emitted replay evidence outside a load replay phase",
                    )),
                }
            }
            PiSdkSidecarRecord::Event(event) => {
                let turn = self
                    .active_turn
                    .lock()
                    .expect("sidecar active lock poisoned")
                    .clone()
                    .ok_or_else(|| {
                        failure(
                            "swallowtail.pi.sdk-sidecar.event_without_turn",
                            "Pi SDK sidecar emitted an event without an active turn",
                        )
                    })?;
                turn.handle_event(event)
            }
            PiSdkSidecarRecord::Terminal(_) => Err(failure(
                "swallowtail.pi.sdk-sidecar.terminal_record",
                "Pi SDK sidecar reported an unrecoverable terminal failure",
            )),
            // Diagnostics are safe redacted observations; they never change
            // driver state.
            PiSdkSidecarRecord::Diagnostic(_) => Ok(()),
        }
    }

    fn fail_pending(&self, error: RuntimeFailure) {
        let pending =
            std::mem::take(&mut *self.pending.lock().expect("sidecar pending lock poisoned"));
        for (_, pending) in pending {
            pending.sender.complete(Err(error.clone()));
        }
    }
}
